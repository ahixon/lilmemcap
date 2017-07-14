#!/bin/sh

TARGET=thumbv6m-none-eabi
STORAGE_BLOCK=0x280000

arm-none-eabi-objcopy -O binary target/$TARGET/debug/lilmemcap lilmemcap-flat.bin
echo "Wrting to board..."
sudo /mnt/vodka/private/alex/memento/src/rockchip/rkbin/tools/rkdeveloptool wl ${STORAGE_BLOCK} lilmemcap-flat.bin

# load_emmc 0x282000 0x250000 32
