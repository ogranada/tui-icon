#!/bin/bash 

current="$(pwd)"

echo "Checking $current"

mkdir -p target
for file in $current/*; do
  if [[ $file == *.png ]]; then
    tgt="${file/\.png/.pbm}"
    echo "Processing $file to $tgt"
    convert $file -background white -flatten $tgt
    mv $tgt target
  fi
done


