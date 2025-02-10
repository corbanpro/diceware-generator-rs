#!/bin/bash

cargo build --release
cp ./target/release/dicew ~/.local/bin/
