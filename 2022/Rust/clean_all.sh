#!/bin/bash

for f in *
do
  [[ -d ${f} ]] && (cd ${f} && cargo clean)
done
