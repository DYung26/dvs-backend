#!/bin/bash

# echo "$ diesel setup"
# diesel setup
# timestamp=$(date +%Y%m%d%H%M%S)
# name=${1:-migration}
# echo "diesel migration generate <name>"
# diesel migration generate "${timestamp}_${name}"
echo "diesel migration run"
diesel migration run
echo "diesel print-schema > src/schema.rs"
diesel print-schema > src/schema.rs
