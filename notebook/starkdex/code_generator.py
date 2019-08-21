import json

with open("denominators.json") as f:
    denominators = json.load(f)
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
