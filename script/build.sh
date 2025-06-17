#!/usr/bin/env bash

ROOT=$1

(cd $ROOT/rust && cargo build --release)

if [ $? -ne 0 ]; then
    exit 1
fi

LUA=$ROOT/lua

rm -f $LUA/*.so

cp $ROOT/rust/target/release/libwildcat.so $LUA
mv $LUA/libwildcat.so $LUA/wildcat.so

(cd $ROOT/rust && cargo clean)
