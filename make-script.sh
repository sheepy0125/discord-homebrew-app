#!/bin/sh
# I don't know how to use Makefiles and am too lazy to learn. So, here is my make script.

DEVICE_NAME="sdc1"

cleanup() {
    echo "Cleaning up..."
    echo "Sleeping for 3 seconds to let the SD card finish writing..."
    sleep 3
    sudo umount mounted_sd
    rmdir mounted_sd > /dev/null 2>&1
}
error() {
    cleanup
    figlet "FAILED!!!!"
    exit 1
}

cleanup
make || error
mkdir -p mounted_sd || error
sudo mount /dev/$DEVICE_NAME mounted_sd || error
sudo cp target/3ds/release/rust3ds-discord.3dsx mounted_sd/3ds || error
cleanup
figlet "SUCCESS!"