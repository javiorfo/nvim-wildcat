#!/usr/bin/env bash

# ROOT=$1
ROOT=/home/javier/.local/share/nvim/lazy/nvim-wildcat

(cd $ROOT/rust && cargo build --release)

if [ $? -ne 0 ]; then
    exit 1
fi

LUA=$ROOT/lua

rm -f $LUA/wildcatr.so

cp $ROOT/rust/target/release/libwildcatr.so $LUA
mv $LUA/libwildcatr.so $LUA/wildcatr.so

# (cd $ROOT/rust && cargo clean)
