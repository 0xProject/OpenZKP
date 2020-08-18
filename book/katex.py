#!/usr/bin/env python3
import json
import sys

if len(sys.argv) > 1:
    if sys.argv[1] == 'supports':
        # sys.argv[2] is the renderer name
        sys.exit(0)

katex = '\n' + open('../.cargo/katex-header.html', 'r').read()
context, book = json.load(sys.stdin)


def fix(items):
    for section in items:
        if 'Chapter' in section:
            section['Chapter']['content'] += katex
            fix(section['Chapter']['sub_items'])


fix(book['sections'])
json.dump(book, sys.stdout)
