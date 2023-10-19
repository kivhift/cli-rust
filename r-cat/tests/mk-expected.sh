#!/bin/bash

set -eu -o pipefail

# Make things easier by being able to work relative to containing directory.
cd ${0%/*}

declare -r IN=inputs OUT=expected

[ -d $OUT ] || mkdir $OUT

declare all= file= outf=
for file in $IN/*; do
    all+=" $file"
    outf=$OUT/${file##*/}
    cat $file > $outf.out
    cat -n $file > $outf.n.out
    cat -b $file > $outf.b.out
    cat < $file > $outf.stdin.out
    cat -n < $file > $outf.n.stdin.out
    cat -b < $file > $outf.b.stdin.out
done

cat $all > $OUT/all.out
cat -n $all > $OUT/all.n.out
cat -b $all > $OUT/all.b.out
