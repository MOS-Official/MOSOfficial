#!/bin/bash

rustup toolchain install nightly

cargo install bootimage 

sudo apt update 
sudo apt install qemu-system-x86 -y
