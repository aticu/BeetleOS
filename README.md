# BeetleOS
An experimental operating system written in [Rust][5]. A rewrite of [VeOS][8].

## Trying BeetleOS for yourself
To run BeetleOS on a fresh Ubuntu install (tested on 18.04 LTS) take the following steps (adjust the steps for your operating system):

Install build dependencies and qemu to run BeetleOS:
```
$ sudo apt install curl gcc git make lld xorriso ovmf qemu
```

Install Rust (you can use the default settings):
```
$ curl https://sh.rustup.rs -sSf | sh
```
Then relog to have Rust added to the PATH variable.

Install xargo to cross compile the [core][9]-library:
```
$ cargo install xargo
```

Download BeetleOS and change to its folder:
```
$ git clone https://github.com/aticu/BeetleOS.git
$ cd BeetleOS
```

Install the correct version of the Rust compiler needed for BeetleOS and the Rust source code for cross compiling:
```
$ rustup update $(cat rust-toolchain)
$ rustup component add rust-src
```

And finally run it:
```
$ make run
```

Or you can run integration tests with:
```
$ make test
```

## Acknowledgements
A lot of this work is based on work from the following people/organizations or at least highly influenced by it:
- Philipp Oppermann and his "[Writing an OS in Rust][1]" blog.
- The contributors of the [spin crate][2].
- Eric Kidd's [blog][3].
- The [OSDev wiki][4].
- The [Redox][6] project.
- Mike Rieker's excellent [APIC tutorial][7].

[1]: http://os.phil-opp.com/ "Writing an OS in Rust"
[2]: https://crates.io/crates/spin "The spin crate on crates.io"
[3]: http://www.randomhacks.net/bare-metal-rust/ "Bare Metal Rust: Building kernels in Rust"
[4]: http://wiki.osdev.org/Main_Page "OSDev wiki Main Page"
[5]: https://www.rust-lang.org/
[6]: https://www.redox-os.org
[7]: https://web.archive.org/web/20140308064246/http://www.osdever.net/tutorials/pdf/apic.pdf
[8]: https://github.com/aticu/VeOS
[9]: https://doc.rust-lang.org/core/