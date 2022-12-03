#!/bin/bash

echo "Building ..."
for f in Day*
do
  (cd ${f} && cargo build 2>/dev/null)
  echo -n "."
done
echo ""
echo "Done!"

for f in Day*
do
  (cd ${f} && cargo run -- -p1 2>/dev/null && cargo run -- -p2 2>/dev/null)
done
