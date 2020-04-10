#!/usr/bin/env python3
import json
import sys

if len(sys.argv) > 1:
    if sys.argv[1] == 'supports':
        # sys.argv[2] is the renderer name
        sys.exit(0)

katex = '\n' + open('../.cargo/katex-header.html', 'r').read()
context, book = json.load(sys.stdin)
for section in book['sections']:
    if 'Chapter' in section:
        section['Chapter']['content'] += katex
json.dump(book, sys.stdout)
