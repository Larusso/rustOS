RustOS
======

[![Build Status](https://travis-ci.org/Larusso/rustOS.svg?branch=master)](https://travis-ci.org/Larusso/rustOS)

This project follows the blogpost series [Writing an OS in Rust (Second Edition)](https://os.phil-opp.com) by Philipp Oppermann.

Build
=====

* install [rustup](https://www.rustup.rs/)
* install [QEMO](https://www.qemu.org/download/)
* install rust nightly
  `rustup update nightly`
* install xbuild
  `cargo install cargo-xbuild`
* install bootimage
  `cargo install bootimage --version "^0.5.0"`
* build
  `bootimage build`
* run
  `bootimage run`

License
=======

[Apache License 2.0](LICENSE)
