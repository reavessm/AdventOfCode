#!/bin/bash

for f in Day*
do
  (cd ${f} && cargo clean)
done
