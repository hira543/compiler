#!/bin/bash
nasm -f elf64 -o output.o output.asm
gcc -g -no-pie -nostartfiles -o a output.o
