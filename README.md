> 没啥好看的，快跑吧。我是废物这件事已经石锤了。

### Description

This project is based on the book [Writing an OS in Rust](os.phil-opp.com) by Philipp Oppermann.

Expect this to be a duplicate of [blog_os](https://github.com/phil-opp/blog_os) :-D

### Build

To build the project:

```
$ rustup update nightly --force
$ cargo build
```

To create a bootable disk image:

```
$ cargo install bootimage
$ cargo bootimage
```

### Run

To run the disk image in QEMU:

```
$ cargo run
```

### Resources

[zh_CN translation for *Writing an OS in Rust*](https://github.com/rustcc/writing-an-os-in-rust).
