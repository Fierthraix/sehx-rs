#!/usr/bin/env python3
import argparse
from pathlib import Path
import sys


def read(file):
    for line in file:
        for chunk in line.split():
            yield chunk


def run(file):
    byte_string = ''.join(chunk for chunk in read(file.open('r')))
    my_hex = bytearray.fromhex(byte_string)
    sys.stdout.buffer.write(my_hex)


if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument('file', type=Path)
    run(parser.parse_args().file)
