#!/bin/bash

set -eu -o pipefail

# Make things easier by being able to work relative to containing directory.
cd ${0%/*}

declare -r IN=inputs OUT=expected

[ -d $OUT ] || mkdir $OUT

declare -r files=(
    empty.txt
    one.txt
    two.txt
    three.txt
    ten.txt
    UTF-8-demo.txt
)

declare all= file= outf= i=
for file in ${files[*]}; do
    outf=$OUT/$file.out
    file=$IN/$file
    all+=" $file"

    head $file > $outf
    head -c 1 $file > $outf-c1
    for i in 2 4; do
        head -c $i $file > $outf-c$i
        head -n $i $file > $outf-n$i
    done
    head -c 162 $file > $outf-c162
done

declare outa=$OUT/all.out
head $all > $outa
head -c 1 $all > $outa-c1
for i in 2 4; do
    head -c $i $all > $outa-c$i
    head -n $i $all > $outa-n$i
done
