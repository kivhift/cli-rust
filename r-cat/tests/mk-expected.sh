#!/bin/bash

set -eu -o pipefail

# Make things easier by being able to work relative to containing directory.
cd ${0%/*}

declare -r IN=inputs OUT=expected

[ -d $OUT ] || mkdir $OUT

declare -r files=(
    UTF-8-demo.txt
    empty.txt
    fox.txt
    spiders.txt
    the-bustle.txt
)

declare all= file= outf= a=
for file in ${files[*]}; do
    outf=$OUT/$file.out
    file=$IN/$file
    all+=" $file"

    for a in '' -b -n; do
        cat $a $file > $outf$a
        cat $a < $file > $outf.stdin$a
    done
done

for a in '' -b -n; do
    cat $a $all > $OUT/all.out$a
done
