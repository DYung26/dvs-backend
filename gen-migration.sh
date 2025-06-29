#!/bin/bash

timestamp=$(date +%Y%m%d%H%M%S)
name=${1:-migration}
diesel migration generate "${timestamp}_${name}"
# diesel migration run
