# INSA-login-rememberer

This web extension intends to automatically log-in students to their INSA account.
It used to be tedious as cookies expired too often.
Passwords are saved locally in the browser's storage.
Both manifest v2 and v3 are supported.

## Installation

Select your browser:

<a href="https://github.com/Mubelotix/insa-login-rememberer/releases/download/v3/164d1a1b8f5d4b7184ce-3.0.xpi" target="_blank"><img src="https://imgur.com/ihXsdDO.png" width="64" height="64" alt="Firefox"></a>
<a href="https://chrome.google.com/webstore/detail/insa-login-rememberer/alnfaipldfkadlijdfjbkclngnmfokph?hl=fr&authuser=0" target="_blank"><img src="https://imgur.com/z8yjLZ2.png" width="64" height="64" alt="Brave"></a>
<a href="https://chrome.google.com/webstore/detail/insa-login-rememberer/alnfaipldfkadlijdfjbkclngnmfokph?hl=fr&authuser=0" target="_blank"><img src="https://imgur.com/3C4iKO0.png" width="64" height="64" alt="Chrome"></a>
<a href="https://chrome.google.com/webstore/detail/insa-login-rememberer/alnfaipldfkadlijdfjbkclngnmfokph?hl=fr&authuser=0" target="_blank"><img src="https://imgur.com/vMcaXaw.png" width="64" height="64" alt="Edge"></a>
<a href="https://chrome.google.com/webstore/detail/insa-login-rememberer/alnfaipldfkadlijdfjbkclngnmfokph?hl=fr&authuser=0" target="_blank"><img src="https://imgur.com/EuDp4vP.png" width="64" height="64" alt="Vivaldi"></a>
<a href="https://chrome.google.com/webstore/detail/insa-login-rememberer/alnfaipldfkadlijdfjbkclngnmfokph?hl=fr&authuser=0" target="_blank"><img src="https://imgur.com/nSJ9htU.png" width="64" height="64" alt="Opera"></a>
<a href="https://github.com/Mubelotix/insa-login-rememberer/releases/download/v3/164d1a1b8f5d4b7184ce-3.0.xpi" target="_blank"><img src="https://imgur.com/MQYBSrD.png" width="64" height="64" alt="Tor"></a>
<!--
<a href="https://bitwarden.com/download/" target="_blank"><img src="https://imgur.com/ENbaWUu.png" width="64" height="64"></a>
-->

## Build

Use the `build.sh` script to build the extension.
[Rust](https://rust-lang.org/) and [`wasm-pack`](https://rustwasm.github.io/wasm-pack/) are required but the script should be able to install them automatically.
The script is intended to run on Linux.
