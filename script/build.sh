#!/usr/bin/env bash

# ROOT=$1
ROOT=/home/javier/.local/share/nvim/lazy/nvim-wildcat

(cd $ROOT/rust && cargo build --release)

if [ $? -ne 0 ]; then
    exit 1
fi

LUA=$ROOT/lua

rm -f $LUA/wildcat_rust.so

cp $ROOT/rust/target/release/libwildcat_rust.so $LUA
mv $LUA/libwildcat_rust.so $LUA/wildcat_rust.so

# (cd $ROOT/rust && cargo clean)
