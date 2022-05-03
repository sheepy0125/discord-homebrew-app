#!/bin/sh
# I don't know how to use Makefiles and am too lazy to learn. So, here is my make script.

DEVICE_NAME="sdc1"

cleanup() {
    echo "Cleaning up..."
    echo "Waiting for SD card to finish..."
    echo "Sleeping for 1 seconds to let the SD card finish writing..."
    sleep 1
    sudo umount mounted_sd
    rmdir mounted_sd > /dev/null 2>&1
}
error() {
    cleanup
    figlet "FAILED!!!!"
    exit 1
}

cleanup
mkdir -p mounted_sd || error
sudo mount /dev/$DEVICE_NAME mounted_sd || error
make && sudo cp target/3ds/release/rust3ds-discord.3dsx mounted_sd/3ds || error
cleanup
figlet "SUCCESS!"