#!/bin/sh
riscv64-elf-as test.asm
riscv64-elf-objcopy -O binary a.out test.bin
