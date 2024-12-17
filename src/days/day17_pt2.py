from z3 import Solver, sat, BitVec

with open("input/day17.txt") as file:
    A_in, B_in, C_in, _, P_in = file.readlines()
    A = int(A_in.split(": ")[1])
    B = int(B_in.split(": ")[1])
    C = int(C_in.split(": ")[1])
    P = [int(x) for x in P_in.split(": ")[1].split(",")]

instructions = list(zip(P[::2], P[1::2]))
a_counter, b_counter, c_counter = 0, 0, 0
z3_vars = {}

def z(name):
    return z3_vars.setdefault(name, BitVec(name, 64))

def calc_combo(value, A, B, C):
    match value:
        case n if 0 <= n <= 3:
            return n
        case 4:
            return z(f"a{A}")
        case 5:
            return z(f"b{B}")
        case 6:
            return z(f"c{C}")

solver = Solver()

for i, target in enumerate(P):
    for opcode, operand in instructions:
        combo_operand = calc_combo(operand, a_counter, b_counter, c_counter)

        match opcode:
            case 0:
                a_counter += 1
                solver.add(z(f"a{a_counter}") == z(f"a{a_counter - 1}") >> combo_operand)
            case 1:
                b_counter += 1
                solver.add(z(f"b{b_counter - 1}") ^ operand == z(f"b{b_counter}"))
            case 2:
                b_counter += 1
                solver.add(combo_operand % 8 == z(f"b{b_counter}"))
            case 3:
                if i == len(P) - 1:
                    solver.add(z(f"a{a_counter}") == 0)
                else:
                    solver.add(z(f"a{a_counter}") > 0)
            case 4:
                b_counter += 1
                solver.add(z(f"b{b_counter - 1}") ^ z(f"c{c_counter}") == z(f"b{b_counter}"))
            case 5:
                solver.add(z(f"b{b_counter}") % 8 == target)
            case 7:
                c_counter += 1
                solver.add(z(f"c{c_counter}") == z(f"a{a_counter}") >> combo_operand)

a0 = z('a0')
min_a = None

while solver.check() == sat:
    model = solver.model()
    x_value = model[a0].as_long()
    if min_a is None or x_value < min_a:
        min_a = x_value
    solver.add(a0 < min_a)

print(min_a)
