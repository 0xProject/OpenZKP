import re

with open("denominators.txt") as f:
    lines = f.readlines()

for line in lines:
    assert re.match(r'^.*\-.*$', line), line
    if re.match(r'point\^\(trace_length / [0-9]+\)', line):
        if re.match(r'trace_generator^(251 * trace_length / 256)', line):
    else:
        continue
        print 'special line:', line
    #
    # if re.match(r'', line):
    #     print line
    #
    # first, second = line.split("-", 1)
    # try:
    #     base, power = first.split("^", 1)
    # except ValueError:
    #     base = first
    #     power = "(trace_length / trace_length)"
    #
    # numerator, denominator = power.split("/", 1)
    # denominator = denominator[:-1]
    #
    # print line[:-1],
    # print '->'
    # print '\t%s * i %% trace_length == 0 ' % (denominator)
    # print first, second
