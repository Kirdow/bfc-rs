#!/bin/sh

echo \>\> Compiling BFC-RS \<\<
cargo build

cd vendor/bf-rs
echo \>\> Compiling BF-RS \<\<
cargo build
cd ../..

echo \>\> Compiling demo.bfc to demo.bf \<\<
target/debug/bfc demo/demo.bfc demo/out/demo.bf

cd vendor/bf-rs
echo \>\> Running compiled demo.bf \<\<
target/debug/bf ../../demo/out/demo.bf
cd ../..