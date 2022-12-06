#!/bin/bash
LD_PRELOAD=${PWD}/lib/azul/target/release/libazul.so "$@"
