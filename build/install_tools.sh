#!/bin/bash

set -euxo pipefail

#wllvm and gllvm
pip install --upgrade pip==9.0.3
pip install wllvm
mkdir ${HOME}/go
go install github.com/SRI-CSL/gllvm/cmd/...@v1.3.1

