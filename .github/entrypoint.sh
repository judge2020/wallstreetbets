#!/bin/bash

# https://doc.rust-lang.org/cargo/reference/config.html#hierarchical-structure
mkdir /.cargo
touch /.cargo/config
cat >>/.cargo/config <<EOF
[target.x86_64-pc-windows-gnu]
linker = "/usr/bin/x86_64-w64-mingw32-gcc"
EOF

cargo build --target x86_64-pc-windows-gnu $@
