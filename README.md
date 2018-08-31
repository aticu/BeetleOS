# BeetleOS
An experimental operating system written in [Rust][5]. A rewrite of [VeOS][8].

## Trying BeetleOS
To run BeetleOS on a fresh Ubuntu install (tested on 18.04 LTS) take the following steps:

Install build dependencies and qemu to run BeetleOS:
```
$ sudo apt install curl gcc git make lld xorriso qemu
```

Install Rust (you can use the default settings):
```
$ curl https://sh.rustup.rs -sSf | sh
```
Then relog to have Rust added to the PATH variable.

Install xargo and the Rust source code to cross compile the [core][9]-library:
```
$ cargo install xargo
$ rustup component add rust-src
```

Download BeetleOS:
```
$ git clone https://github.com/aticu/BeetleOS.git
```

And finally run it:
```
$ cd BeetleOS
$ make run
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