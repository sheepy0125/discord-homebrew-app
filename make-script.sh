#!/bin/sh
# I don't know how to use Makefiles and am too lazy to learn. So, here is my make script.
mkdir mounted_sd
sudo mount /dev/sdc1 mounted_sd
make && sudo cp target/3ds/release/rust3ds-discord.3dsx mounted_sd/3ds
sudo umount mounted_sd
rmdir mounted_sd