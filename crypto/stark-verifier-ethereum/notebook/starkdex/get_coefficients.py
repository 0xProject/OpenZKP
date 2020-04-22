#!/usr/bin/env python3

import json


def rearrange_coefficients(coefficients):
    block_size = 7
    new_coefficients = []
    for i in range(0, len(coefficients), block_size):
        new_coefficients.extend(coefficients[i:i+block_size][::-1])
    return new_coefficients


def get_coefficients(path):
    with open(path) as solidity_file:
        lines = [line for line in solidity_file if "add(0x" in line]

    lines = rearrange_coefficients(lines)[::-1]

    return [line.split(",")[0][14:] for line in lines]


def format(s):
    return s[2:].rjust(64, "0")


if __name__ == "__main__":
    file_names = [
        "DexHashPointsXColumn.sol",
        "DexHashPointsYColumn.sol",
        "DexEcdsaPointsXColumn.sol",
        "DexEcdsaPointsYColumn.sol",
    ]

    for file_name in file_names:
        coefficients = map(format, get_coefficients(file_name))

        print "pub(crate) const %s: [FieldElement; %i] = [" % (
            file_name.split(".")[0], len(coefficients)
        )
        for coefficient in coefficients:
            print 'field_element!("%s"),' % coefficient
        print "];"
