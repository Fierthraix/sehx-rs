#!/usr/bin/env python3
"""
from sys import argv
from struct import pack

hexes = open(argv[1], 'r').read()
bin = open(argv[2], 'wb')
for hex in hexes.split():
    bin.write(pack('I', int(hex, base=16)))
"""
import argparse
from pathlib import Path
import struct
import sys


def read(file):
    for line in file:
        for chunk in line.split():
            yield struct.pack('I', int(chunk, 16))


def run(file):
    byte_string = [chunk for chunk in read(file.open('r'))]
    my_hex = list(byte for byte in byte_string)
    sys.stdout.buffer.write(my_hex)


if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument('file', type=Path)
    run(parser.parse_args().file)
