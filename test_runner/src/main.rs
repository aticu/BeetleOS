//! The test runner is used to run the integration tests for the kernel.

use clap::{crate_authors, crate_version, value_t, App, Arg};
use std::{
    env::current_dir,
    fmt,
    fs::{copy, create_dir_all, read_dir, File},
    io::{Read, Write},
    path::PathBuf,
    process::{exit, Command, Stdio},
    str::from_utf8,
    time::Duration,
};
use tempfile::Builder;
use wait_timeout::ChildExt;

/// Reset the colors in the shell.
macro_rules! reset {
    () => {
        "\x1b[0m"
    };
}

/// Set the color to the test fail color in the shell.
macro_rules! test_fail_color {
    () => {
        "\x1b[34m"
    };
}

/// Set the background color to the test fail background color in the shell.
macro_rules! test_fail_background {
    () => {
        "\x1b[49m"
    };
}

/// Prepares the shell to print the failure of a test.
macro_rules! test_fail {
    () => {
        concat!(
            bold!(),
            underline!(),
            test_fail_color!(),
            test_fail_background!()
        )
    };
}

/// Set the font to bold in the shell.
macro_rules! bold {
    () => {
        "\x1b[1m"
    };
}

/// Set the font to underlined in the shell.
macro_rules! underline {
    () => {
        "\x1b[4m"
    };
}

/// The main entry point for the test runner.
fn main() -> Result<(), String> {
    let config = get_config()?;

    // Find existing tests.
    let test_names = get_test_names(&config)?;

    // Prepare the tests for running on the target platform.
    for test in &test_names {
        match prepare_test(&config, test) {
            Err(error) => {
                eprint!(
                    "{}{}{}: {}",
                    test_fail!(),
                    get_test_short_name(&config, test),
                    reset!(),
                    error
                );
                exit(1);
            }
            _ => (),
        }
    }

    let mut fails = Vec::new();

    // Run the tests.
    for test in &test_names {
        match run_test(&config, test) {
            Err(error) => {
                eprintln!("\t{}Fail{}: {}", test_fail!(), reset!(), error);
                fails.push((get_test_short_name(&config, test), error));
            }
            _ => eprintln!("\tSuccess"),
        }
    }

    eprintln!();

    if fails.len() > 0 {
        eprintln!(
            "The following tests failed ({} out of {}):",
            fails.len(),
            test_names.len()
        );

        for fail in fails {
            eprintln!("\t{}{}{}: {}", test_fail!(), fail.0, reset!(), fail.1);
        }

        exit(1);
    } else {
        eprintln!("All {} tests were successful!", test_names.len())
    }

    Ok(())
}

/// All the reasons a test can fail.
#[derive(Debug)]
enum TestFailReason {
    /// The test failed to compile.
    FailedToCompile(Vec<u8>),
    /// The test failed to be prepared for running.
    FailedToPrepare(String),
    /// The test was interrupted before it could finish.
    TestInterrupted,
    /// The test failed explicitly with the given reason.
    TestExplicitFail(String),
    /// The test timed out, before it could finish.
    TestTimeout,
}

impl fmt::Display for TestFailReason {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TestFailReason::FailedToCompile(compiler_output) => write!(
                f,
                "The test failed to compile:\n{}",
                from_utf8(compiler_output).unwrap_or("Failed to get compiler output.")
            ),
            TestFailReason::FailedToPrepare(reason) => {
                write!(f, "The test could not be prepared for running: {}", reason)
            }
            TestFailReason::TestInterrupted => {
                write!(f, "The test was interrupted, before it could finish")
            }
            TestFailReason::TestExplicitFail(reason) => write!(f, "{}", reason.trim()),
            TestFailReason::TestTimeout => write!(f, "Timed out"),
        }
    }
}

/// Gets the short name of the test.
fn get_test_short_name<'a>(config: &Config, long_name: &'a str) -> &'a str {
    let mut iter = long_name.chars();

    for _ in 0.."test-".len() + config.arch.len() + "-".len() {
        iter.next();
    }

    iter.as_str()
}

/// Runs the test.
fn run_test(config: &Config, name: &str) -> Result<(), TestFailReason> {
    eprintln!("Running test '{}'", get_test_short_name(config, name));

    match config.run_on.as_str() {
        "qemu" => {
            let mut command = Command::new("qemu-system-x86_64");

            command
                .arg("--no-reboot")
                .arg("-smp")
                .arg("cores=4")
                .arg("-s")
                .arg("-serial")
                .arg("stdio")
                .arg("-device")
                .arg("isa-debug-exit,iobase=0xf4,iosize=0x04")
                .arg("-net")
                .arg("none")
                .arg("-display")
                .arg("none")
                .arg("-bios")
                .arg(&config.bios)
                .arg("-cdrom")
                .arg(config.result_dir.join(format!("{}.iso", name)))
                .stdin(Stdio::null())
                .stdout(Stdio::piped())
                .stderr(Stdio::null());

            let mut child = command.spawn().map_err(|err| {
                TestFailReason::FailedToPrepare(format!("Could not run qemu: {}", err))
            })?;

            let status_code = match child
                .wait_timeout(Duration::from_secs(config.timeout))
                .map_err(|err| {
                    TestFailReason::FailedToPrepare(format!("Could not wait for qemu: {}", err))
                })? {
                Some(status) => status.code(),
                None => {
                    child.kill().map_err(|err| {
                        TestFailReason::FailedToPrepare(format!("Could not kill qemu: {}", err))
                    })?;
                    Err(TestFailReason::TestTimeout)?
                }
            };

            const FORMATTING_OUTPUT_LENGTH: usize = 144;

            let mut child_stdout = Vec::new();

            child
                .stdout
                .ok_or_else(|| {
                    TestFailReason::FailedToPrepare(format!("Could not gather qemus output"))
                })?.read_to_end(&mut child_stdout)
                .map_err(|err| {
                    TestFailReason::FailedToPrepare(format!(
                        "Could not gather qemus output: {}",
                        err
                    ))
                })?;

            let output_str = from_utf8(&child_stdout[FORMATTING_OUTPUT_LENGTH..])
                .unwrap_or("No reason given.")
                .to_string();

            match status_code {
                None => Err(TestFailReason::TestInterrupted)?,
                Some(1) => (),
                Some(_) => Err(TestFailReason::TestExplicitFail(output_str))?,
            }
        }
        _ => unimplemented!("Currently only qemu is supported."),
    }

    Ok(())
}

/// Prepares the test to be run on the target platform.
fn prepare_test(config: &Config, name: &str) -> Result<(), TestFailReason> {
    compile_test(config, name)?;

    // TODO: Check if tests are still up to date.
    match config.arch.as_str() {
        "x86_64" => {
            // Create a temporary directory to work in.
            let tmp_dir = Builder::new()
                .prefix(&format!("test_runner.{}", name))
                .tempdir()
                .map_err(|err| TestFailReason::FailedToPrepare(format!("tempdir: {}", err)))?;

            // Create a subdirectory.
            let boot_path = tmp_dir.path().join("EFI").join("BOOT");
            create_dir_all(&boot_path)
                .map_err(|err| TestFailReason::FailedToPrepare(format!("tempdir: {}", err)))?;

            // Copy the binary to the correct path.
            copy(config.result_dir.join(name), boot_path.join("BOOTX64.EFI"))
                .map_err(|err| TestFailReason::FailedToPrepare(format!("cp: {}", err)))?;

            // Create the esp image.
            let esp_path = tmp_dir.path().join("esp.img");
            {
                let mut esp = File::create(&esp_path).map_err(|err| {
                    TestFailReason::FailedToPrepare(format!("{}: {}", esp_path.display(), err))
                })?;

                let zeroes = [0; 1024];

                for _ in 0..64 * 1024 {
                    esp.write_all(&zeroes).map_err(|err| {
                        TestFailReason::FailedToPrepare(format!("{}: {}", esp_path.display(), err))
                    })?;
                }
            }

            // Create the FAT32 file system on the esp image.
            let mut command = Command::new("mkfs.vfat");
            command
                .arg("-F")
                .arg("32")
                .arg(&esp_path)
                .arg("-n")
                .arg("EFISYS")
                .stdin(Stdio::null())
                .stdout(Stdio::null())
                .stderr(Stdio::null());
            let status = command.status().map_err(|err| {
                TestFailReason::FailedToPrepare(format!("{}: {}", esp_path.display(), err))
            })?;
            if !status.success() {
                Err(TestFailReason::FailedToPrepare(
                    "Could not create the FAT image on the ESP.".to_string(),
                ))?;
            }

            // Copy the test onto the esp image.
            let mut command = Command::new("mcopy");
            command
                .arg("-i")
                .arg(&esp_path)
                .arg("-s")
                .arg(tmp_dir.path().join("EFI"))
                .arg("::")
                .stdin(Stdio::null())
                .stdout(Stdio::null())
                .stderr(Stdio::null());
            let status = command.status().map_err(|err| {
                TestFailReason::FailedToPrepare(format!("{}: {}", esp_path.display(), err))
            })?;
            if !status.success() {
                Err(TestFailReason::FailedToPrepare(
                    "Could not copy the test to the ESP.".to_string(),
                ))?;
            }

            // Create the directory for the iso file.
            let iso_dir = tmp_dir.path().join("iso");
            create_dir_all(&iso_dir).map_err(|err| {
                TestFailReason::FailedToPrepare(format!("{}: {}", esp_path.display(), err))
            })?;

            // Copy the esp image to the iso path.
            copy(&esp_path, iso_dir.join("esp.img"))
                .map_err(|err| TestFailReason::FailedToPrepare(format!("cp: {}", err)))?;

            // Finally create the iso.
            let mut command = Command::new("xorriso");
            command
                .arg("-as")
                .arg("mkisofs")
                .arg("-o")
                .arg(config.result_dir.join(format!("{}.iso", name)))
                .arg("-e")
                .arg("esp.img")
                .arg("-no-emul-boot")
                .arg(&iso_dir)
                .stdin(Stdio::null())
                .stdout(Stdio::null())
                .stderr(Stdio::null());
            let status = command
                .status()
                .map_err(|err| TestFailReason::FailedToPrepare(format!("xorriso: {}", err)))?;
            if !status.success() {
                Err(TestFailReason::FailedToPrepare(
                    "Could not create the iso image.".to_string(),
                ))?;
            }
        }
        _ => {
            eprintln!(
                "test_runner doesn't currently support the architecture {}.",
                config.arch
            );
            exit(1);
        }
    }

    Ok(())
}

/// Compiles the test with the given name and returns the path to the resulting binary.
fn compile_test(config: &Config, name: &str) -> Result<(), TestFailReason> {
    let mut command = Command::new(&config.rust_compiler);

    command
        .current_dir(&config.compile_dir)
        .env("RUST_TARGET_PATH", &config.rust_target_path)
        .arg("build")
        .arg("--color")
        .arg("always")
        .arg("--features")
        .arg(format!("{} integration_test", config.run_on))
        .arg(format!("--target={}", config.target_triple))
        .arg(format!("--bin={}", name));

    if config.release {
        command.arg("--release");
    }

    let output = command
        .output()
        .expect(&format!("Could not run command: '{:?}'", command));

    if !output.status.success() {
        Err(TestFailReason::FailedToCompile(output.stderr))?;
    }

    Ok(())
}

/// Returns the names of all tests to be run.
fn get_test_names(config: &Config) -> Result<Vec<String>, String> {
    let test_prefix = format!("test-x86_64-");
    let mut test_names = Vec::new();

    for entry in
        read_dir(&config.test_dir).map_err(|err| format!("{:?}: {}", config.test_dir, err))?
    {
        if let Ok(entry) = entry {
            if let Some(name) = entry.file_name().to_str() {
                if entry.path().is_file() && name.starts_with(&test_prefix) && name.ends_with(".rs")
                {
                    test_names.push((&name[..name.len() - 3]).to_string())
                }
            }
        }
    }

    Ok(test_names)
}

/// Represents a configuration for a run of the test runner.
#[derive(Debug)]
struct Config {
    /// The architecture to test.
    arch: String,
    /// The target triple to compile the tests for.
    target_triple: String,
    /// Whether or not to run tests in release mode.
    release: bool,
    /// The directory where the test source files are located.
    test_dir: PathBuf,
    /// The directory in which to invoke the compiler.
    compile_dir: PathBuf,
    /// The directory in which the resulting binaries are located.
    result_dir: PathBuf,
    /// The directory where the target specifications are located.
    rust_target_path: String,
    /// The rust compiler to use.
    rust_compiler: String,
    /// The virtual machine the tests are run on.
    run_on: String,
    /// The path to the bios image to use.
    bios: String,
    /// The timeout in seconds.
    timeout: u64,
}

/// Returns the configuration for this run of test_runner.
fn get_config() -> Result<Config, String> {
    let matches = App::new("test_runner")
        .version(&crate_version!()[..])
        .author(crate_authors!())
        .about("test_runner runs the integration tests for BeetleOS. Runs all tests in the test-dir of the form 'test-<arch>-<test-name>.rs'.")
        .arg(Arg::with_name("arch")
            .required(true)
            .takes_value(true)
            .help("The architecture to test for")
            .long_help("The name of the architecture that the tests should be run for."))
        .arg(Arg::with_name("target-triple")
            .required(false)
            .takes_value(true)
            .long("target-triple")
            .help("The target triple to compile for")
            .long_help("The name of the target triple that the tests should be compiled for. Default is '<arch>-unknown-none'."))
        .arg(Arg::with_name("release")
            .required(false)
            .takes_value(false)
            .long("release")
            .help("If present, run in release mode")
            .long_help("If this flag is present, the tests are run in release mode."))
        .arg(Arg::with_name("test-dir")
            .required(false)
            .takes_value(true)
            .requires("compile-dir")
            .requires("result-dir")
            .long("test-dir")
            .help("The directory where the test source files are located")
            .long_help("Specifies the directory of the source files of the tests. The default is '<current_dir>/kernel/src/bin'."))
        .arg(Arg::with_name("compile-dir")
            .required(false)
            .takes_value(true)
            .requires("test-dir")
            .requires("result-dir")
            .long("compile-dir")
            .help("The directory in which to compile the tests")
            .long_help("Specifies the directory in which the Rust compiler should be run. The default is '<current_dir>/kernel'."))
        .arg(Arg::with_name("result-dir")
            .required(false)
            .takes_value(true)
            .requires("compile-dir")
            .requires("test-dir")
            .long("result-dir")
            .help("The directory in which the resulting binaries are found")
            .long_help("Specifies the directory in which the resulting binaries will be found. The default is '<current_dir>/target/<target-triple>/<release-type>'."))
        .arg(Arg::with_name("rust-target-path")
            .required(false)
            .takes_value(true)
            .long("rust-target-path")
            .help("The path where the target specifications are located")
            .long_help("Specifies the directory where the target specifictaions are located. The default is '<current_dir>/targets'."))
        .arg(Arg::with_name("rust-compiler")
            .required(false)
            .takes_value(true)
            .long("rust-compiler")
            .help("The rust compiler to use")
            .long_help("Specifies the rust compiler binary to invoke. The default is 'xargo'."))
        .arg(Arg::with_name("run-on")
            .required(false)
            .takes_value(true)
            .long("run-on")
            .help("The VM to test on")
            .long_help("Specifies the virtual machine to use for testing. The default is 'qemu'."))
        .arg(Arg::with_name("bios")
            .required(false)
            .takes_value(true)
            .long("bios")
            .help("The bios to use")
            .long_help("Specifies the path to the bios to use. The default is '/usr/share/ovmf/OVMF.fd'"))
        .arg(Arg::with_name("timeout")
            .required(false)
            .takes_value(true)
            .long("timeout")
            .help("The timeout, in seconds, used for tests")
            .long_help("Specifies the timeout that is used for tests in seconds. The default is 30."))
        .get_matches();

    let arch = matches.value_of("arch").unwrap().to_string();
    let target_triple = matches
        .value_of("target-triple")
        .map(|triple| triple.to_string())
        .unwrap_or(format!("{}-unknown-none", arch));
    let release = matches.is_present("release");
    let test_dir = matches
        .value_of("test-dir")
        .map(|path| Ok(PathBuf::from(path)))
        .unwrap_or_else(|| {
            current_dir()
                .map(|path| path.join("kernel").join("src").join("bin"))
                .map_err(|dir_err| format!("{}", dir_err))
        })?;
    let compile_dir = matches
        .value_of("compile-dir")
        .map(|path| Ok(PathBuf::from(path)))
        .unwrap_or_else(|| {
            current_dir()
                .map(|path| path.join("kernel"))
                .map_err(|dir_err| format!("{}", dir_err))
        })?;
    let result_dir = matches
        .value_of("result-dir")
        .map(|path| Ok(PathBuf::from(path)))
        .unwrap_or_else(|| {
            current_dir()
                .map(|path| {
                    path.join("target").join(&target_triple).join(if release {
                        "release"
                    } else {
                        "debug"
                    })
                }).map_err(|dir_err| format!("{}", dir_err))
        })?;
    let rust_target_path = matches
        .value_of("rust-target-path")
        .map(|path| Ok(path.to_string()))
        .unwrap_or_else(|| {
            current_dir()
                .map(|path| format!("{}", path.join("targets").display()))
                .map_err(|dir_err| format!("{}", dir_err))
        })?;

    Ok(Config {
        arch,
        target_triple,
        release,
        test_dir,
        compile_dir,
        result_dir,
        rust_target_path,
        rust_compiler: matches
            .value_of("rust-compiler")
            .unwrap_or("xargo")
            .to_string(),
        run_on: matches.value_of("run-on").unwrap_or("qemu").to_string(),
        bios: matches
            .value_of("bios")
            .unwrap_or("/usr/share/ovmf/OVMF.fd")
            .to_string(),
        timeout: value_t!(matches, "timeout", u64).unwrap_or(30),
    })
}
