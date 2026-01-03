# chkprm
 A simple command line utility to make permissions more understandable for Mac/Linux.

![chkprm_demo](https://github.com/user-attachments/assets/1690708e-18bb-4736-94f0-b6ab0dc3e01b)

chkprm allows you to check the current permissions on any file.

Use `chkprm <file>` to see the three main permissions for a file (user, group, other).

## Installation
You can install chkprm via the binary in the latest releases page.


Alternatively, you can build it from source using Rust, Cargo, and Git:
```bash
git clone https://github.com/solarcosmic/chkprm.git
cd chkprm
cargo build --release
```
The binary "`chkprm`" should then be located in the `target/debug` directory.