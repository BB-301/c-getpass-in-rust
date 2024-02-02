#!/bin/bash

set -e

mkdir -p ./build

gcc \
    -std=c17 -Wall -Wextra -Werror -O0 -pedantic \
    -c ./c/my_lib.c -o ./build/my_lib.o

ar rcs ./build/libmy_lib.a ./build/my_lib.o
