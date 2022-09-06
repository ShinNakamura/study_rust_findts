#! /bin/bash
unalias -a

files="$(cargo run -- "**/*.txt")"
for name in a s d
do
    echo "is test tests/data/$name.txt in"
    (echo "$files" | grep -F "tests/data/$name.txt") || exit 1
    echo "OK"
    echo ""
done
