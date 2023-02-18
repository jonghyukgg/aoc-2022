from ortools.linear_solver import pywraplp
import re

# PART1_FILE_NAME = "sample.txt"
PART1_FILE_NAME = "input.txt"
# PART2_FILE_NAME = "sample.txt"
PART2_FILE_NAME = "input.txt"


def f(blueprint, T):
    solver = pywraplp.Solver.CreateSolver("SCIP")
    if not solver:
        raise Exception("System Error")
    infinity = solver.infinity()

    # robots built each min
    o_build = {t: solver.IntVar(0.0, infinity, f"o_{t}") for t in range(2, T)}
    c_build = {t: solver.IntVar(0.0, infinity, f"c_{t}") for t in range(2, T)}
    b_build = {t: solver.IntVar(0.0, infinity, f"b_{t}") for t in range(2, T)}
    g_build = {t: solver.IntVar(0.0, infinity, f"g_{t}") for t in range(2, T)}

    # min 1
    o, c, b, g = 1, 0, 0, 0
    o_r, c_r, b_r, g_r = 1, 0, 0, 0
    for t in range(2, T):
        # build one at a time
        solver.Add(o_build[t] + c_build[t] + b_build[t] + g_build[t] <= 1)

        # assets used
        o -= (
            o_build[t] * blueprint["o_o"]
            + c_build[t] * blueprint["c_o"]
            + b_build[t] * blueprint["b_o"]
            + g_build[t] * blueprint["g_o"]
        )
        solver.Add(o >= 0)
        c -= b_build[t] * blueprint["b_c"]
        solver.Add(c >= 0)
        b -= g_build[t] * blueprint["g_b"]
        solver.Add(b >= 0)

        # proceed
        o, c, b, g = o + o_r, c + c_r, b + b_r, g + g_r
        o_r += o_build[t]
        c_r += c_build[t]
        b_r += b_build[t]
        g_r += g_build[t]
    g += g_r  # last minute

    solver.Maximize(g)
    status = solver.Solve()
    if status == pywraplp.Solver.OPTIMAL:
        return int(solver.Objective().Value() + 1e-6)
    raise Exception("No solution found")


def parse_line(line):
    x = re.search(
        r"Blueprint \d+: "
        + r"Each ore robot costs (\d+) ore. "
        + r"Each clay robot costs (\d+) ore. "
        + r"Each obsidian robot costs (\d+) ore and (\d+) clay. "
        + r"Each geode robot costs (\d+) ore and (\d+) obsidian.",
        line,
    )
    return {
        "o_o": int(x.group(1)),
        "c_o": int(x.group(2)),
        "b_o": int(x.group(3)),
        "b_c": int(x.group(4)),
        "g_o": int(x.group(5)),
        "g_b": int(x.group(6)),
    }


def main():
    ans = 0
    for idx, blueprint_str in enumerate(open(PART1_FILE_NAME, "r")):
        blueprint = parse_line(blueprint_str)
        res = f(blueprint, 24)
        # print(idx + 1, res)
        ans += (idx + 1) * res
    print("Part 1:", ans)

    ans = 1
    for idx, blueprint_str in enumerate(open(PART2_FILE_NAME, "r")):
        if idx >= 3:
            break
        blueprint = parse_line(blueprint_str)
        res = f(blueprint, 32)
        # print(idx + 1, res)
        ans *= res
    print("Part 2:", ans)


if __name__ == "__main__":
    main()
