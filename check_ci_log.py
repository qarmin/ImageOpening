#!/usr/bin/env python
# -*- coding: utf-8 -*-

import sys

if len(sys.argv) < 2:
    print("ERROR: You must run program with file name as argument.")
    sys.exit(100)

fname = sys.argv[1]

fileread = open(fname.strip(), "r")
file_contents = fileread.read()


if file_contents.find("panic") != -1:
    print("ERROR: Panic")
    sys.exit(1)

sys.exit(0)
