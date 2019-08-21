import json

with open("denominators.json") as f:
    denominator_mapping = json.load(f)

with open("numerators.json") as f:
    numerator_mapping = json.load(f)

with open("everything.txt") as f:
    lines = f.readlines()

numerators = [numerator_mapping[line[11:-1]] for line in lines[1::3]]
denominators = [denominator_mapping[line[13:-1]] for line in lines[2::3]]

with open("bases.txt") as f:
    bases = [line[:-1] for line in f.readlines()]

for base, numerator, denominator in zip(bases, numerators, denominators):
    print "if (%s) && !(%s) {assert_eq!(%s, FieldElement::ZERO);}" % (denominator, numerator, base)
