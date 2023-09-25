# INSA-login-rememberer

This web extension intends to automatically log-in students to their INSA account.
It used to be tedious as cookies expired too often.
Passwords are saved locally in the browser's storage.
Both manifest v2 and v3 are supported.

Use the `build.sh` script to build the extension.
[Rust](https://rust-lang.org/) and [`wasm-pack`](https://rustwasm.github.io/wasm-pack/) are required but the script should be able to install them automatically.
The script is intended to run on Linux.
