#! /bin/sh
# HACK: Workaround for <https://github.com/rust-lang/cargo/issues/5034>
# Make sure all lib.rs and main.rs files start with the same prefix
correct=$(cat ./lints.rs)
lines=$(wc -l < ./lints.rs)
files=$(find . \( -name target -prune \) -o \( -name lib.rs -o -name main.rs \) -print)
failed=false
for file in $files; do
    start=$(head -n $lines $file)
    if [ "$start" != "$correct" ]; then
        echo "Incorrect lints in $file:"
        echo "$start" | diff -wb -U3 --minimal --color - ./lints.rs || true
        echo ""
        failed=true
    fi
done
if [ "$failed" = true ]; then
    false
fi
