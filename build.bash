#!/bin/bash

if [[ -n $1 ]];
then
    rm -rf artifacts/
fi

cmake -S . -B artifacts/
cmake --build artifacts --parallel
./artifacts/log-example
