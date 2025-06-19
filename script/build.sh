#!/usr/bin/env bash

ROOT=$1

(cd $ROOT && cargo build --release)

if [ $? -ne 0 ]; then
    exit 1
fi

LUA=$ROOT/lua

rm -f $LUA/*.so

cp $ROOT/target/release/libwildcat.so $LUA
mv $LUA/libwildcat.so $LUA/wildcat.so

(cd $ROOT && cargo clean)
