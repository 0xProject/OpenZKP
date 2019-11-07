import re

base_pattern = re.compile(
    r"^\s*// Constraint expression for (.*): (.*)\.\n$")

trace_table_pattern = re.compile(
    r"column([0-9]+)_row([0-9]+)")

numerator_pattern = re.compile(
    r"^\s*// Numerator: (.*)\n$")

denominator_pattern = re.compile(
    r"^\s*// Denominator: (.*)\n$")


def translate(line):
    return (
        re.sub(trace_table_pattern, r"Trace(\1, \2)", line)
        .replace("point^", "X^")
        .replace("point -", "X -")
        .replace("^", ".pow")
        .replace("powtrace_length", "pow(trace_length)")
        .replace(") - 1", ") - 1.into()")
        .replace("1 - ", "Constant(1.into()) - ")
        .replace("X - 1", "X - 1.into()")
        .replace("t - 1", "t - 1.into()")
        .replace("column4_row_exprConstant(1.into())", "column4_row_expr1")
    )


def get_intermediate_values(line):
    matches = (
        re.findall(r"(state_transition__.*?)[ |\)]", line) +
        re.findall(r"(hash_pool__.*?)[ |\)]", line) +
        re.findall(r"(sig_verify__.*?)[ |\)]", line) +
        re.findall(r"(amounts_range_check__.*?)[ |\)]", line)
    )
    return set(matches)


labels = []
bases = []
numerators = []
denominators = []

with open("DexConstraintPoly.sol") as f:
    lines = f.readlines()

for line in lines:
    line = translate(line)

    base_and_label, count = re.subn(base_pattern, r"\2~\1", line)
    if count:
        base, label = base_and_label.split("~")
        bases.append(base)
        labels.append(label)

    numerator, count = re.subn(numerator_pattern, r"\1", line)
    if count:
        numerators.append(numerator)

    denominator, count = re.subn(denominator_pattern, r"\1", line)
    if count:
        denominators.append(denominator)

intermediate_values = set.union(*map(get_intermediate_values, bases))
# This occurs at the end of the string, so the existing patterns don't
# pick it up.
intermediate_values.add(
    "state_transition__merkle_update__new_authentication__sibling_0")
intermediate_values_pattern = re.compile(
    r"^\s*// (%s) = (.*)\n$" % "|".join(map(lambda s: s.replace("__", "/"),
                                            intermediate_values)))
for line in lines:
    match = re.match(intermediate_values_pattern, line)
    if match:
        variable, value = match.groups()
        print "let %s = %s;" % (variable.replace("/", "__"), translate(value))

print "vec!["
for base, numerator, denominator, label in zip(bases, numerators,
                                               denominators, labels):
    if numerator != "1":
        print "(%s) * (%s) / (%s), // %s" % (base, numerator, denominator,
                                             label)
    else:
        print "(%s) / (%s), // %s" % (base, denominator, label)
print "]"
