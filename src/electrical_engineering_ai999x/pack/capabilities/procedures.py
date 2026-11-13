"""Electrical engineering procedures: deterministic algorithms with typed inputs and several outputs (domain code,
loaded by the pack; enabled only once tools/certify_procedures.py has certified them).

Each procedure states its inputs, outputs, conditions, method, LaTeX, the documented question templates it parses,
verified worked examples, an INDEPENDENT reference computation (used only by certification; the checking libraries
are imported inside the reference functions) and a deliberately wrong variant that the checks must reject.

The shipped code (run, verify, parse) is plain Python: math, cmath, re, itertools.

Conventions of the core-course section (magnetic circuits, induction, capacitors, plane waves): mu0 = 4 pi x 1e-7 H/m
and eps0 = 1/(mu0 c^2) with c = 299 792 458 m/s (so 1/sqrt(mu0 eps0) = c exactly); the references use the same values.
Temperatures are taken in kelvin (a question's degrees Celsius are converted by its reader); a temperature
coefficient is per kelvin (= per degree Celsius). Reactive power is reported in var and apparent power in VA as plain
numbers. Machine speeds are in revolutions per second (questions may write rpm). A phase angle phi is in radians.
"""
from __future__ import annotations

import cmath
import itertools
import math
import re

# ================================================================================================ shared helpers
_NUMRE = r"(?<![\w.])[-+]?(?:\d+(?:\.\d+)?|\.\d+)(?:[eE][-+]?\d+)?"
_PREFIX = {"": 1.0, "p": 1e-12, "n": 1e-9, "u": 1e-6, "µ": 1e-6, "μ": 1e-6, "m": 1e-3, "k": 1e3, "M": 1e6, "G": 1e9}


def _unit_table(bases, words=None):
    out = {}
    for p, f in _PREFIX.items():
        for b in bases:
            out[p + b] = f
    out.update(words or {})
    return out


_UNITS = {
    "resistance": _unit_table(["ohms", "ohm", "Ω"], {"kilohms": 1e3, "kilohm": 1e3, "megohms": 1e6, "megohm": 1e6}),
    "capacitance": _unit_table(["F"], {"farads": 1.0, "farad": 1.0, "microfarads": 1e-6, "microfarad": 1e-6,
                                       "nanofarads": 1e-9, "nanofarad": 1e-9, "picofarads": 1e-12, "picofarad": 1e-12,
                                       "millifarads": 1e-3, "millifarad": 1e-3}),
    "inductance": _unit_table(["H"], {"henries": 1.0, "henrys": 1.0, "henry": 1.0, "millihenries": 1e-3,
                                      "millihenrys": 1e-3, "millihenry": 1e-3, "microhenries": 1e-6, "microhenrys": 1e-6,
                                      "microhenry": 1e-6}),
    "voltage": _unit_table(["V", "volts", "volt"]),
    "current": _unit_table(["A", "amps", "amp", "amperes", "ampere"]),
    "frequency": _unit_table(["Hz"]),
    "power": _unit_table(["W", "watts", "watt"]),
}
_SI_UNIT = {"resistance": "ohm", "capacitance": "F", "inductance": "H", "voltage": "V", "current": "A",
            "frequency": "Hz", "power": "W"}


def _unit_alt(kind):
    return "|".join(re.escape(u) for u in sorted(_UNITS[kind], key=len, reverse=True))


def _value_run(question: str, kind: str, keep_unit: bool = False):
    """'10 ohm, 20 ohm and 30 ohm' or '10, 20 and 30 ohm' -> SI values. Every number of the question must belong to
    the one list (a number outside it, such as 'three 10 ohm resistors' written '3', makes the reading refuse)."""
    ualt = _unit_alt(kind)
    toks = []
    for m in re.finditer(_NUMRE, question):
        um = re.match(rf"\s*({ualt})(?![A-Za-zΩ])", question[m.end():])
        toks.append({"s": m.start(), "e": m.end() + (um.end() if um else 0), "v": float(m.group(0)),
                     "u": um.group(1) if um else None})
    if len(toks) < 2:
        return None
    for a, b in zip(toks, toks[1:]):
        between = question[a["e"]:b["s"]]
        if not re.fullmatch(r"\s*(?:,|and|,\s*and)?\s*(?:[A-Za-z]\w*\s*=\s*)?", between):
            return None
    if toks[-1]["u"] is None:
        return None
    out, unit = [], None
    for t in reversed(toks):
        unit = t["u"] or unit
        out.append(t["v"] * _UNITS[kind][unit])
    out.reverse()
    if keep_unit:                                    # the values in the unit of the last one, and that unit
        last = toks[-1]["u"]
        return [v / _UNITS[kind][last] for v in out], last
    return out


def _named_values(question: str, names, kind: str) -> dict:
    """'Ra = 10 ohm, Rb = 20 ohm' -> {'Ra': '10 ohm', ...} (a value with its unit, read by the runtime)."""
    ualt = _unit_alt(kind)
    num = _NUMRE.replace('(?<![\\w.])', '')            # outside the f-string: Python 3.10/3.11 forbid "\" inside one
    found = {}
    for n in names:
        m = re.search(rf"(?<![\w]){re.escape(n)}\s*=\s*({num})\s*({ualt})(?![A-Za-zΩ])", question)
        if m:
            u = m.group(2)
            found[n] = float(m.group(1)) * _UNITS[kind][u]
    return found


def _rel(a, b):
    return abs(a - b) / max(1e-300, abs(a), abs(b))


def _gauss(A, b):
    """Solve A x = b by Gaussian elimination with partial pivoting (plain Python). A singular system is a
    ValueError: the circuit or the equations do not determine the unknowns."""
    n = len(A)
    if n == 0 or any(len(r) != n for r in A) or len(b) != n:
        raise ValueError("the matrix must be square and the vector must have one entry per row")
    M = [[complex(x) if isinstance(x, complex) else float(x) for x in row] + [b[i]] for i, row in enumerate(A)]
    scale = max(abs(x) for row in A for x in row) or 1.0
    for c in range(n):
        p = max(range(c, n), key=lambda r: abs(M[r][c]))
        if abs(M[p][c]) <= 1e-12 * scale:
            raise ValueError("the system is singular: the equations do not determine every unknown")
        M[c], M[p] = M[p], M[c]
        for r in range(c + 1, n):
            f = M[r][c] / M[c][c]
            if f != 0:
                for k in range(c, n + 1):
                    M[r][k] -= f * M[c][k]
    x = [0.0] * n
    for r in reversed(range(n)):
        x[r] = (M[r][n] - sum(M[r][k] * x[k] for k in range(r + 1, n))) / M[r][r]
    return x


def _positive_values(vals, what):
    if len(vals) < 2:
        raise ValueError(f"give at least two {what}")
    for v in vals:
        if not (math.isfinite(v) and v > 0):
            raise ValueError(f"every {what[:-1] if what.endswith('s') else what} must be a positive finite number")


_E12 = [1.0, 1.2, 1.5, 1.8, 2.2, 2.7, 3.3, 3.9, 4.7, 5.6, 6.8, 8.2]


def _e12(rng, lo_exp, hi_exp):
    return round(rng.choice(_E12) * 10 ** rng.randint(lo_exp, hi_exp), 12)


# ================================================================================ series and parallel combinations
_COMBO = {  # kind: (symbol, unit word, plural noun, e12 exponent range)
    "resistance": ("R", "ohm", "resistors", (0, 5)),
    "capacitance": ("C", "F", "capacitors", (-12, -4)),
    "inductance": ("L", "H", "inductors", (-6, -1)),
}


def _combo_run(kind, mode):
    sym, _u, noun, _r = _COMBO[kind]
    adds = (kind != "capacitance") == (mode == "series")         # R and L add in series; C adds in parallel

    def run(**kw):
        vals = kw[sym]
        _positive_values(vals, noun)
        total = sum(vals) if adds else 1.0 / sum(1.0 / v for v in vals)
        out = {"equivalent": total, "count": float(len(vals))}
        if kind != "resistance":                     # values in any one unit: the result is in that unit
            out["unit"] = kw.get("unit") or "the unit of the inputs"
        return out
    return run


def _combo_verify(kind, mode):
    sym = _COMBO[kind][0]
    adds = (kind != "capacitance") == (mode == "series")

    def verify(inputs, outputs):
        vals, eq = inputs[sym], outputs["equivalent"]
        if adds:                                    # the sum, compensated (math.fsum), relative to its size
            return abs(eq - math.fsum(vals)) / math.fsum(vals) + abs(outputs.get("count", len(vals)) - len(vals))
        # reciprocal form: sum of eq/v_i must be exactly 1
        return abs(math.fsum(eq / v for v in vals) - 1.0) + abs(outputs.get("count", len(vals)) - len(vals))
    return verify


def _combo_wrong(kind, mode):
    sym = _COMBO[kind][0]
    adds = (kind != "capacitance") == (mode == "series")

    def wrong(**kw):                                # the series and parallel rules exchanged
        vals = kw[sym]
        return {"equivalent": (1.0 / sum(1.0 / v for v in vals)) if adds else sum(vals), "count": float(len(vals))}
    return wrong


def _combo_reference(kind, mode):
    """INDEPENDENT reference: the network is built as a graph (series: a chain of nodes; parallel: every element
    between the same two nodes) and its two-terminal impedance is the effective impedance from the pseudo-inverse
    of the complex admittance Laplacian (numpy.linalg.pinv) at an angular frequency of 1 rad/s."""
    sym = _COMBO[kind][0]

    def reference(**kw):
        import numpy as np
        vals = kw[sym]
        n = len(vals)
        if kind == "resistance":
            y = [1.0 / v for v in vals]
        elif kind == "capacitance":
            y = [1j * v for v in vals]               # Y = j w C, w = 1
        else:
            y = [1.0 / (1j * v) for v in vals]       # Y = 1/(j w L)
        nodes = n + 1 if mode == "series" else 2
        Lap = np.zeros((nodes, nodes), dtype=complex)
        for k, yk in enumerate(y):
            a, b = (k, k + 1) if mode == "series" else (0, 1)
            Lap[a, a] += yk
            Lap[b, b] += yk
            Lap[a, b] -= yk
            Lap[b, a] -= yk
        e = np.zeros(nodes)
        e[0], e[-1] = 1.0, -1.0
        z = complex(e @ np.linalg.pinv(Lap) @ e)
        if kind == "resistance":
            eq = z.real
        elif kind == "capacitance":
            eq = (1.0 / (1j * z)).real                # Z = 1/(j C)
        else:
            eq = (z / 1j).real                        # Z = j L
        return {"equivalent": eq}
    return reference


def _combo_parse(kind, mode):
    other = "parallel" if mode == "series" else "series"
    noun = _COMBO[kind][2]
    word = {"resistance": "resistance", "capacitance": "capacitance", "inductance": "inductance"}[kind]

    def parse(question):
        q = question.lower()
        if not re.search(rf"\b(equivalent|total|combined|net)\s+{word}\b", q) or not re.search(rf"\bin {mode}\b", q):
            return None
        if re.search(rf"\b{other}\b", q) or "||" in q or "+" in question:
            return None                              # a mixed network: resistor_network reads it
        vals = _value_run(question, kind, keep_unit=kind != "resistance")
        if not vals:
            return None
        if kind == "resistance":
            return {"R": vals}
        return {_COMBO[kind][0]: vals[0], "unit": vals[1]}
    return parse


def _combo_spec(kind, mode, examples, questions, templates):
    sym, unit, noun, (lo, hi) = _COMBO[kind]
    adds = (kind != "capacitance") == (mode == "series")
    formula = {True: rf"{sym}_{{eq}} = \sum_i {sym}_i", False: rf"\frac{{1}}{{{sym}_{{eq}}}} = \sum_i \frac{{1}}{{{sym}_i}}"}[adds]
    if kind == "resistance":
        inputs = [{"symbol": sym, "type": "vector", "min_length": 2, "max_length": 50, "required": True,
                   "meaning": "the resistance values in ohms (a list, e.g. '10, 20, 30')"}]
        outputs = [{"symbol": "equivalent", "kind": kind, "meaning": f"equivalent {kind}"},
                   {"symbol": "count", "kind": "number", "meaning": f"number of {noun}"}]
    else:                                            # unit-free: the combination rule is homogeneous of degree one
        inputs = [{"symbol": sym, "type": "vector", "min_length": 2, "max_length": 50, "required": True,
                   "meaning": f"the {kind} values, all in one unit ({unit}, m{unit}, u{unit}, n{unit} ...; a list, e.g. '10, 22, 47')"},
                  {"symbol": "unit", "type": "text", "min_length": 1, "max_length": 12, "required": False,
                   "meaning": "the unit the values are written in (the result is in the same unit)"}]
        outputs = [{"symbol": "equivalent", "kind": "number", "meaning": f"equivalent {kind}, in the unit of the inputs"},
                   {"symbol": "count", "kind": "number", "meaning": f"number of {noun}"},
                   {"symbol": "unit", "kind": "text", "meaning": "the unit of the result"}]
    return {
        "name": f"Equivalent {kind} of {noun} in {mode}", "topic": "DC circuits / series and parallel combinations",
        "purpose": f"The single {kind} equivalent to two or more {noun} connected in {mode}.",
        "inputs": inputs, "outputs": outputs,
        "primary": "equivalent", "conditions": [f"every value positive", "at least two values"],
        "method": ("the sum of the values" if adds else "the reciprocal of the sum of reciprocals"),
        "latex": formula,
        "run": _combo_run(kind, mode), "verify": _combo_verify(kind, mode), "parse": _combo_parse(kind, mode),
        "reference": _combo_reference(kind, mode), "wrong": _combo_wrong(kind, mode),
        "random_inputs": (lambda rng: {sym: [_e12(rng, lo, hi) for _ in range(rng.randint(2, 6))]}) if kind == "resistance"
        else (lambda rng: {sym: [_e12(rng, 0, 3) for _ in range(rng.randint(2, 6))]}),
        "templates": templates, "examples": examples, "questions": questions}


PROCEDURES = {}
PROCEDURES["series_resistance"] = _combo_spec(
    "resistance", "series",
    [{"inputs": {"R": "10, 20, 30"}, "outputs": {"equivalent": 60.0}},
     {"inputs": {"R": [1000, 2200]}, "outputs": {"equivalent": 3200.0}}],
    [{"q": "Find the equivalent resistance of 10 ohm, 20 ohm and 30 ohm resistors in series.", "outputs": {"equivalent": 60.0}},
     {"q": "What is the total resistance of 1 kohm and 2.2 kohm in series?", "outputs": {"equivalent": 3200.0}},
     {"q": "Find the equivalent resistance of 10, 0 and 5 ohm resistors in series.", "outcome": "INVALID_INPUT"},
     {"q": "Find the equivalent resistance of 3 resistors of 10 ohm in series.", "outcome": "UNRECOGNIZED_FORMAT"}],
    ["Find the equivalent resistance of 10 ohm, 20 ohm and 30 ohm resistors in series.",
     "What is the total resistance of 1 kohm and 2.2 kohm in series?"])
PROCEDURES["parallel_resistance"] = _combo_spec(
    "resistance", "parallel",
    [{"inputs": {"R": "10, 40"}, "outputs": {"equivalent": 8.0}},
     {"inputs": {"R": [30, 60, 20]}, "outputs": {"equivalent": 10.0}}],
    [{"q": "Find the equivalent resistance of 10 ohm and 40 ohm resistors in parallel.", "outputs": {"equivalent": 8.0}},
     {"q": "What is the total resistance of 30, 60 and 20 ohm in parallel?", "outputs": {"equivalent": 10.0}},
     {"q": "What is the total resistance of 1 kohm and 1 kohm in parallel?", "outputs": {"equivalent": 500.0}},
     {"q": "Find the equivalent resistance of 10 ohm and -40 ohm resistors in parallel.", "outcome": "INVALID_INPUT"}],
    ["Find the equivalent resistance of 10 ohm and 40 ohm resistors in parallel.",
     "What is the total resistance of 30, 60 and 20 ohm in parallel?"])
PROCEDURES["series_capacitance"] = _combo_spec(
    "capacitance", "series",
    [{"inputs": {"C": "3, 6", "unit": "uF"}, "outputs": {"equivalent": 2.0, "unit": "uF"}},
     {"inputs": {"C": [10, 10, 10, 10]}, "outputs": {"equivalent": 2.5}}],
    [{"q": "Find the equivalent capacitance of 3 uF and 6 uF capacitors in series.", "outputs": {"equivalent": 2.0, "unit": "uF"}},
     {"q": "What is the total capacitance of 10 nF, 10 nF and 5 nF in series?", "outputs": {"equivalent": 2.5, "unit": "nF"}},
     {"q": "Find the equivalent capacitance of 100 nF and 0.1 uF capacitors in series.", "outputs": {"equivalent": 0.05, "unit": "uF"}},
     {"q": "Find the equivalent capacitance of 3 uF and 0 uF capacitors in series.", "outcome": "INVALID_INPUT"}],
    ["Find the equivalent capacitance of 3 uF and 6 uF capacitors in series.",
     "What is the total capacitance of 10 nF, 10 nF and 5 nF in series?"])
PROCEDURES["parallel_capacitance"] = _combo_spec(
    "capacitance", "parallel",
    [{"inputs": {"C": "3, 6", "unit": "uF"}, "outputs": {"equivalent": 9.0}},
     {"inputs": {"C": [1, 2.2]}, "outputs": {"equivalent": 3.2}}],
    [{"q": "Find the equivalent capacitance of 3 uF and 6 uF capacitors in parallel.", "outputs": {"equivalent": 9.0, "unit": "uF"}},
     {"q": "What is the total capacitance of 100 pF, 220 pF and 470 pF in parallel?", "outputs": {"equivalent": 790.0, "unit": "pF"}}],
    ["Find the equivalent capacitance of 3 uF and 6 uF capacitors in parallel.",
     "What is the total capacitance of 100 pF, 220 pF and 470 pF in parallel?"])
PROCEDURES["series_inductance"] = _combo_spec(
    "inductance", "series",
    [{"inputs": {"L": "10, 22", "unit": "mH"}, "outputs": {"equivalent": 32.0}},
     {"inputs": {"L": [1, 2, 3]}, "outputs": {"equivalent": 6.0}}],
    [{"q": "Find the equivalent inductance of 10 mH and 22 mH inductors in series.", "outputs": {"equivalent": 32.0, "unit": "mH"}},
     {"q": "What is the total inductance of 1 mH, 2 mH and 3 mH in series?", "outputs": {"equivalent": 6.0, "unit": "mH"}}],
    ["Find the equivalent inductance of 10 mH and 22 mH inductors in series.",
     "What is the total inductance of 1 mH, 2 mH and 3 mH in series?"])
PROCEDURES["parallel_inductance"] = _combo_spec(
    "inductance", "parallel",
    [{"inputs": {"L": "10, 40", "unit": "mH"}, "outputs": {"equivalent": 8.0}},
     {"inputs": {"L": [6, 3]}, "outputs": {"equivalent": 2.0}}],
    [{"q": "Find the equivalent inductance of 10 mH and 40 mH inductors in parallel.", "outputs": {"equivalent": 8.0, "unit": "mH"}},
     {"q": "What is the total inductance of 6 uH and 3 uH in parallel?", "outputs": {"equivalent": 2.0, "unit": "uH"}}],
    ["Find the equivalent inductance of 10 mH and 40 mH inductors in parallel.",
     "What is the total inductance of 6 mH and 3 mH in parallel?"])


# ======================================================================== series-parallel resistor network expression
def _net_tokens(text):
    toks = re.findall(r"\|\||\+|\(|\)|(?:\d+(?:\.\d+)?|\.\d+)(?:[eE][-+]?\d+)?|\S", text)
    if "".join(toks) != re.sub(r"\s+", "", text):
        raise ValueError("cannot read the network expression")
    return toks


def _net_tree(text):
    """'10 + (20 || 30)' -> ('s', [10, ('p', [20, 30])]). '||' binds tighter than '+'."""
    toks = _net_tokens(text)
    pos = [0]

    def peek():
        return toks[pos[0]] if pos[0] < len(toks) else None

    def take():
        if pos[0] >= len(toks):
            raise ValueError("the network expression ends too early (unbalanced parentheses?)")
        pos[0] += 1
        return toks[pos[0] - 1]

    def factor():
        t = peek()
        if t == "(":
            take()
            node = expr()
            if take() != ")":
                raise ValueError("unbalanced parentheses in the network expression")
            return node
        if t is None or not re.fullmatch(r"(?:\d+(?:\.\d+)?|\.\d+)(?:[eE][-+]?\d+)?", t):
            raise ValueError(f"expected a resistance value, found {t!r}")
        take()
        v = float(t)
        if not (v > 0 and math.isfinite(v)):
            raise ValueError("every resistance must be positive")
        return v

    def term():
        items = [factor()]
        while peek() == "||":
            take()
            items.append(factor())
        return items[0] if len(items) == 1 else ("p", items)

    def expr():
        items = [term()]
        while peek() == "+":
            take()
            items.append(term())
        return items[0] if len(items) == 1 else ("s", items)
    tree = expr()
    if pos[0] != len(toks):
        raise ValueError(f"unexpected {toks[pos[0]]!r} in the network expression")
    return tree


def _net_value(t):
    if isinstance(t, float):
        return t
    vals = [_net_value(x) for x in t[1]]
    return math.fsum(vals) if t[0] == "s" else 1.0 / math.fsum(1.0 / v for v in vals)


def _net_count(t):
    return 1 if isinstance(t, float) else sum(_net_count(x) for x in t[1])


def resistor_network(network) -> dict:
    tree = _net_tree(network)
    if isinstance(tree, float):
        raise ValueError("the expression has a single resistor: combine at least two with + or ||")
    return {"equivalent": _net_value(tree), "resistors": float(_net_count(tree))}


def _verify_network(inputs, outputs):
    """Every parallel group re-checked by its reciprocal identity (sum of R_eq/R_i = 1) and every series group by its
    sum, through the whole tree."""
    tree = _net_tree(inputs["network"])
    worst = [0.0]

    def walk(t):
        if isinstance(t, float):
            return t
        vals = [walk(x) for x in t[1]]
        if t[0] == "s":
            return math.fsum(vals)
        eq = 1.0 / math.fsum(1.0 / v for v in vals)
        worst[0] = max(worst[0], abs(math.fsum(eq / v for v in vals) - 1.0))
        return eq
    want = walk(tree)
    return max(worst[0], _rel(outputs["equivalent"], want), abs(outputs.get("resistors", _net_count(tree)) - _net_count(tree)))


def _reference_network(network):
    """INDEPENDENT reference: the expression becomes a graph of nodes and resistor edges (series: new intermediate
    nodes; parallel: shared end nodes) and the two-terminal resistance is the effective resistance from the
    pseudo-inverse of the conductance Laplacian (numpy.linalg.pinv)."""
    import numpy as np
    tree = _net_tree(network)
    edges, count = [], [2]

    def build(t, a, b):
        if isinstance(t, float):
            edges.append((a, b, t))
            return
        if t[0] == "p":
            for x in t[1]:
                build(x, a, b)
            return
        cur = a
        for i, x in enumerate(t[1]):
            nxt = b if i == len(t[1]) - 1 else count[0]
            if nxt != b:
                count[0] += 1
            build(x, cur, nxt)
            cur = nxt
    build(tree, 0, 1)
    n = count[0]
    Lap = np.zeros((n, n))
    for a, b, r in edges:
        g = 1.0 / r
        Lap[a, a] += g
        Lap[b, b] += g
        Lap[a, b] -= g
        Lap[b, a] -= g
    e = np.zeros(n)
    e[0], e[1] = 1.0, -1.0
    return {"equivalent": float(e @ np.linalg.pinv(Lap) @ e)}


def _wrong_network(network):
    """A deliberately wrong variant: a parallel group taken as the sum of the reciprocals (the final inversion
    forgotten)."""
    def walk(t):
        if isinstance(t, float):
            return t
        vals = [walk(x) for x in t[1]]
        return math.fsum(vals) if t[0] == "s" else math.fsum(1.0 / v for v in vals)
    tree = _net_tree(network)
    return {"equivalent": walk(tree), "resistors": float(_net_count(tree))}


def _random_network_expr(rng, depth=0):
    if depth >= 2 or (depth and rng.random() < 0.3):
        return f"{_e12(rng, 0, 4):g}"
    op = rng.choice([" + ", " || "])
    parts = [_random_network_expr(rng, depth + 1) for _ in range(rng.randint(2, 3))]
    s = op.join(parts)
    return f"({s})" if depth else s


def parse_network(question):
    m = re.search(r"\b(?:equivalent|total|combined)\s+resistance\s+of\s+(?:the\s+network\s+)?([0-9.\s+|()eE-]+?)\s*(ohms?|Ω)?\s*[.?]?\s*$", question, re.I)
    if not m or not re.search(r"\+|\|\|", m.group(1)):
        return None
    return {"network": m.group(1).strip()}


PROCEDURES["resistor_network"] = {
    "name": "Equivalent resistance of a series-parallel resistor network", "topic": "DC circuits / series and parallel combinations",
    "purpose": "The equivalent resistance of a network written with + for series and || for parallel, e.g. 10 + (20 || 30).",
    "inputs": [{"symbol": "network", "type": "text", "min_length": 3, "max_length": 400, "required": True,
                "meaning": "the network in ohms: + joins in series, || joins in parallel ('||' binds tighter than '+'), parentheses group"}],
    "outputs": [{"symbol": "equivalent", "kind": "resistance", "meaning": "equivalent resistance"},
                {"symbol": "resistors", "kind": "number", "meaning": "number of resistors in the network"}],
    "primary": "equivalent", "conditions": ["every resistance positive", "at least two resistors", "'||' binds tighter than '+'"],
    "method": "recursive reduction of the expression tree: series groups add, parallel groups add reciprocally",
    "latex": r"R_s = R_1 + R_2,\quad R_p = \left(\frac{1}{R_1} + \frac{1}{R_2}\right)^{-1}",
    "run": resistor_network, "verify": _verify_network, "parse": parse_network, "reference": _reference_network,
    "wrong": _wrong_network, "random_inputs": lambda rng: {"network": _random_network_expr(rng)},
    "templates": ["Find the equivalent resistance of 10 + (20 || 30) ohm.",
                  "What is the total resistance of (100 || 100) + 50 ohms?"],
    "examples": [{"inputs": {"network": "10 + (20 || 30)"}, "outputs": {"equivalent": 22.0, "resistors": 3.0}},
                 {"inputs": {"network": "(100 || 100) + 50"}, "outputs": {"equivalent": 100.0}},
                 {"inputs": {"network": "6 || 3 + 4"}, "outputs": {"equivalent": 6.0}}],
    "questions": [{"q": "Find the equivalent resistance of 10 + (20 || 30) ohm.", "outputs": {"equivalent": 22.0}},
                  {"q": "What is the total resistance of (100 || 100) + 50 ohms?", "outputs": {"equivalent": 100.0}},
                  {"q": "Find the equivalent resistance of 4 || (6 + 6) ohm.", "outputs": {"equivalent": 3.0}},
                  {"q": "Find the equivalent resistance of 10 + (20 || 0) ohm.", "outcome": "INVALID_INPUT"},
                  {"q": "Find the equivalent resistance of 10 + (20 || 30 ohm.", "outcome": "INVALID_INPUT"}]}


# ============================================================================== linear circuit equations (mesh, nodal)
# wording of an AC (phasor) question: the DC readers below refuse it, so it reaches only the AC solvers (2026-11-12)
_AC_HINT = r"\b(?:ac|a\.c\.|phasors?|reactance|impedances?|sinusoidal)\b|\d\s*(?:[kM]?Hz|hertz)\b"


def _bracket_after(question, words):
    """The written matrix or vector that follows one of `words`: '[[10, -4], [-4, 8]]', '[12, 0]' or '10 -4; -4 8'."""
    for w in words:
        m = re.search(rf"\b{w}\s*(?:of\s+|=\s*|:\s*)?(\[\s*\[.*?\]\s*\]|\[[^\[\]]*\])", question, re.I)
        if m:
            return m.group(1)
    return None


def _linear_parse(matrix_words, vector_words, keyword):
    def parse(question):
        if not re.search(keyword, question, re.I) or re.search(_AC_HINT, question, re.I):
            return None
        A = _bracket_after(question, matrix_words)
        b = _bracket_after(question, vector_words)
        if A is None and b is None:
            return None
        out = {}
        if A is not None:
            out["A"] = A
        if b is not None:
            out["b"] = b
        return out
    return parse


def _linear_run(unknown):
    def run(A, b):
        n = len(A)
        if any(len(r) != n for r in A):
            raise ValueError("the matrix must be square")
        if len(b) != n:
            raise ValueError(f"the vector needs {n} entries, one per row of the matrix")
        x = _gauss(A, b)
        return {unknown: x, "first": x[0]}
    return run


def _linear_verify(unknown):
    def verify(inputs, outputs):
        A, b, x = inputs["A"], inputs["b"], outputs[unknown]
        if len(x) != len(b):
            return 1.0
        worst = 0.0
        for row, bi in zip(A, b):
            terms = [a * xi for a, xi in zip(row, x)]
            scale = math.fsum(abs(t) for t in terms) + abs(bi) or 1.0
            worst = max(worst, abs(math.fsum(terms) - bi) / scale)
        return worst + _rel(outputs["first"], x[0]) if x[0] != outputs["first"] else worst
    return verify


def _linear_reference(unknown):
    def reference(A, b):
        """INDEPENDENT reference: scipy.linalg.solve (LAPACK LU factorization)."""
        import numpy as np
        from scipy import linalg
        x = linalg.solve(np.array(A, float), np.array(b, float))
        return {unknown: [float(v) for v in x], "first": float(x[0])}
    return reference


def _linear_wrong(unknown):
    def wrong(A, b):                                 # the coupling (off-diagonal) terms ignored
        x = [bi / row[i] for i, (row, bi) in enumerate(zip(A, b))]
        return {unknown: x, "first": x[0]}
    return wrong


def _random_mesh(rng):
    n = rng.randint(2, 5)
    R = [[0.0] * n for _ in range(n)]
    for i in range(n):
        own = _e12(rng, 0, 3)
        R[i][i] += own
        for j in range(i + 1, n):
            if rng.random() < 0.6:
                shared = _e12(rng, 0, 3)
                R[i][i] += shared
                R[j][j] += shared
                R[i][j] -= shared
                R[j][i] -= shared
    V = [round(rng.uniform(-24, 24), 1) for _ in range(n)]
    if all(v == 0 for v in V):
        V[0] = 12.0
    return {"A": R, "b": V}


def _random_nodal(rng):
    m = _random_mesh(rng)
    G = [[x / 1000.0 for x in r] for r in m["A"]]    # conductances in siemens
    I = [round(v / 1000.0, 4) for v in m["b"]]
    if all(v == 0 for v in I):
        I[0] = 0.01
    return {"A": G, "b": I}


PROCEDURES["mesh_currents"] = {
    "name": "Mesh currents from the mesh equations (Kirchhoff's voltage law)", "topic": "DC circuits / mesh and nodal analysis",
    "purpose": "Solve the mesh equations R i = v of a circuit: the resistance matrix and the source-voltage vector give every mesh current.",
    "inputs": [{"symbol": "A", "type": "matrix", "required": True,
                "meaning": "mesh resistance matrix in ohms (self-resistances on the diagonal, minus the shared resistances off it)"},
               {"symbol": "b", "type": "vector", "min_length": 1, "max_length": 30, "required": True,
                "meaning": "net source voltage around each mesh in volts, in the mesh-current direction"}],
    "outputs": [{"symbol": "currents", "kind": "number", "meaning": "the mesh currents in amperes, in the order of the rows"},
                {"symbol": "first", "kind": "current", "meaning": "the first mesh current"}],
    "primary": "first", "conditions": ["square matrix", "one source voltage per mesh", "a non-singular matrix"],
    "method": "Gaussian elimination with partial pivoting", "latex": r"\mathbf{R}\,\mathbf{i} = \mathbf{v}",
    "run": _linear_run("currents"), "verify": _linear_verify("currents"),
    "parse": _linear_parse(["resistance matrix", "matrix"], ["source vector", "voltage vector", "vector"], r"\bmesh\b"),
    "reference": _linear_reference("currents"), "wrong": _linear_wrong("currents"), "random_inputs": _random_mesh,
    "templates": ["Solve the mesh equations with resistance matrix [[10, -4], [-4, 8]] and source vector [12, 0].",
                  "Find the mesh currents for resistance matrix [[6, -2, 0], [-2, 8, -3], [0, -3, 7]] and voltage vector [10, 0, -5]."],
    "examples": [{"inputs": {"A": "[[10, -4], [-4, 8]]", "b": "[12, 0]"}, "outputs": {"currents": [1.5, 0.75], "first": 1.5}},
                 {"inputs": {"A": "4 -2; -2 6", "b": "10 0"}, "outputs": {"currents": [3.0, 1.0]}}],
    "questions": [{"q": "Solve the mesh equations with resistance matrix [[10, -4], [-4, 8]] and source vector [12, 0].",
                   "outputs": {"currents": [1.5, 0.75]}},
                  {"q": "Find the mesh currents for resistance matrix [[4, -2], [-2, 6]] and voltage vector [10, 0].",
                   "outputs": {"currents": [3.0, 1.0]}},
                  {"q": "Solve the mesh equations with resistance matrix [[1, 2], [2, 4]] and source vector [1, 1].", "outcome": "INVALID_INPUT"},
                  {"q": "Solve the mesh equations with resistance matrix [[10, -4], [-4, 8]].", "outcome": "NEEDS_INPUT"}]}

PROCEDURES["nodal_voltages"] = {
    "name": "Node voltages from the nodal equations (Kirchhoff's current law)", "topic": "DC circuits / mesh and nodal analysis",
    "purpose": "Solve the nodal equations G v = i of a circuit: the conductance matrix and the injected-current vector give every node voltage.",
    "inputs": [{"symbol": "A", "type": "matrix", "required": True,
                "meaning": "nodal conductance matrix in siemens (sums of conductances on the diagonal, minus the shared conductances off it)"},
               {"symbol": "b", "type": "vector", "min_length": 1, "max_length": 30, "required": True,
                "meaning": "current injected into each node by sources, in amperes"}],
    "outputs": [{"symbol": "voltages", "kind": "number", "meaning": "the node voltages in volts, in the order of the rows"},
                {"symbol": "first", "kind": "voltage", "meaning": "the first node voltage"}],
    "primary": "first", "conditions": ["square matrix", "one injected current per node", "a non-singular matrix"],
    "method": "Gaussian elimination with partial pivoting", "latex": r"\mathbf{G}\,\mathbf{v} = \mathbf{i}",
    "run": _linear_run("voltages"), "verify": _linear_verify("voltages"),
    "parse": _linear_parse(["conductance matrix", "matrix"], ["current vector", "source vector", "vector"], r"\bnodal\b|\bnode voltages\b"),
    "reference": _linear_reference("voltages"), "wrong": _linear_wrong("voltages"), "random_inputs": _random_nodal,
    "templates": ["Solve the nodal equations with conductance matrix [[0.5, -0.25], [-0.25, 0.75]] and current vector [2, 0].",
                  "Find the node voltages for conductance matrix [[0.3, -0.1], [-0.1, 0.2]] and current vector [1, 0.5]."],
    "examples": [{"inputs": {"A": "[[0.5, -0.25], [-0.25, 0.75]]", "b": "[2, 0]"}, "outputs": {"voltages": [4.8, 1.6], "first": 4.8}},
                 {"inputs": {"A": [[0.3, -0.1], [-0.1, 0.2]], "b": [1, 0.5]}, "outputs": {"voltages": [5.0, 5.0]}}],
    "questions": [{"q": "Solve the nodal equations with conductance matrix [[0.5, -0.25], [-0.25, 0.75]] and current vector [2, 0].",
                   "outputs": {"voltages": [4.8, 1.6]}},
                  {"q": "Find the node voltages for conductance matrix [[0.3, -0.1], [-0.1, 0.2]] and current vector [1, 0.5].",
                   "outputs": {"voltages": [5.0, 5.0]}},
                  {"q": "Solve the nodal equations with conductance matrix [[0.5, -0.25], [-0.25, 0.75]] and current vector [2, 0, 1].",
                   "outcome": "INVALID_INPUT"}]}


# ================================================================================== netlists: node voltages, Thevenin
def _netlist(text):
    """'V1 1 0 12; R1 1 2 1000; R2 2 0 2000' -> elements (type, name, node+, node-, value). Node 0 is ground.
    R: ohms; V: volts (node+ is the positive terminal); I: amperes flowing from node+ through the source to node-."""
    els, names = [], set()
    for part in re.split(r"[;\n]+", text):
        p = part.strip().rstrip(".").strip()
        if not p:
            continue
        toks = p.split()
        if len(toks) != 4:
            raise ValueError(f"cannot read the element '{p}': write <name> <node> <node> <value>")
        name, a, b, v = toks
        t = name[0].upper()
        if t not in "RVI":
            raise ValueError(f"element '{name}': only resistors (R), voltage sources (V) and current sources (I)")
        if name.upper() in names:
            raise ValueError(f"element name '{name}' is used twice")
        names.add(name.upper())
        try:
            val = float(v)
        except ValueError:
            raise ValueError(f"element '{name}': '{v}' is not a number") from None
        if not math.isfinite(val):
            raise ValueError(f"element '{name}': the value must be finite")
        if t == "R" and val <= 0:
            raise ValueError(f"resistor '{name}' must have a positive resistance")
        if a == b:
            raise ValueError(f"element '{name}' connects node {a} to itself")
        els.append((t, name, a, b, val))
    if not any(e[0] == "R" for e in els):
        raise ValueError("the circuit needs at least one resistor")
    if not any("0" in (e[2], e[3]) for e in els):
        raise ValueError("the circuit needs a ground node named 0")
    return els


def _node_key(n):
    return (0, int(n), "") if n.isdigit() else (1, 0, n)


def _mna(els):
    """Modified nodal analysis (plain Python): node voltages and voltage-source currents (positive from node+
    through the source to node-)."""
    nodes = sorted({n for e in els for n in (e[2], e[3]) if n != "0"}, key=_node_key)
    idx = {n: i for i, n in enumerate(nodes)}
    vs = [e for e in els if e[0] == "V"]
    N = len(nodes) + len(vs)
    A = [[0.0] * N for _ in range(N)]
    z = [0.0] * N
    for t, name, a, b, val in els:
        ia, ib = idx.get(a), idx.get(b)
        if t == "R":
            g = 1.0 / val
            for i, j, s in ((ia, ia, g), (ib, ib, g), (ia, ib, -g), (ib, ia, -g)):
                if i is not None and j is not None:
                    A[i][j] += s
        elif t == "I":
            if ia is not None:
                z[ia] -= val
            if ib is not None:
                z[ib] += val
    for k, (t, name, a, b, val) in enumerate(vs):
        r = len(nodes) + k
        ia, ib = idx.get(a), idx.get(b)
        if ia is not None:
            A[ia][r] += 1.0
            A[r][ia] += 1.0
        if ib is not None:
            A[ib][r] -= 1.0
            A[r][ib] -= 1.0
        z[r] = val
    x = _gauss(A, z)
    vmax = max((abs(x[idx[n]]) for n in nodes), default=0.0)
    imax = max((abs(x[len(nodes) + k]) for k in range(len(vs))), default=0.0)
    snap = lambda v, s: 0.0 if abs(v) <= 1e-12 * s else v      # noqa: E731  rounding residue of an exact zero
    volts = {"0": 0.0, **{n: snap(x[idx[n]], vmax) for n in nodes}}
    cur = {e[1]: snap(x[len(nodes) + k], imax) for k, e in enumerate(vs)}
    return volts, cur


def _power_delivered(els, volts, cur):
    p = 0.0
    for t, name, a, b, val in els:
        if t == "V":
            p += -val * cur[name]
        elif t == "I":
            p += val * (volts[b] - volts[a])
    return p


def dc_node_voltages(netlist) -> dict:
    els = _netlist(netlist)
    volts, cur = _mna(els)
    return {"node_voltages": {n: v for n, v in volts.items() if n != "0"}, "source_currents": cur,
            "power_delivered": _power_delivered(els, volts, cur)}


def _verify_nodes(inputs, outputs):
    """Kirchhoff's current law at every node, every voltage source's constraint, and the power balance (power
    delivered by the sources = power dissipated in the resistors)."""
    els = _netlist(inputs["netlist"])
    V = {"0": 0.0, **outputs["node_voltages"]}
    cur = outputs["source_currents"]
    nodes = {n for e in els for n in (e[2], e[3])}
    if set(V) != nodes or set(cur) != {e[1] for e in els if e[0] == "V"}:
        return 1.0
    kcl = {n: [] for n in nodes}
    worst, diss, terms = 0.0, 0.0, []
    for t, name, a, b, val in els:
        if t == "R":
            i = (V[a] - V[b]) / val
            diss += i * i * val
        elif t == "V":
            i = cur[name]
            worst = max(worst, abs((V[a] - V[b]) - val) / max(abs(val), abs(V[a]), abs(V[b]), 1e-12))
        else:
            i = val
        kcl[a].append(i)
        kcl[b].append(-i)
    big = max((abs(x) for ts in kcl.values() for x in ts), default=0.0) or 1.0   # a dead-end node carries ~0 A
    for n, terms in kcl.items():
        if n == "0":
            continue
        scale = max(math.fsum(abs(x) for x in terms), big)
        worst = max(worst, abs(math.fsum(terms)) / scale)
    p = outputs["power_delivered"]
    worst = max(worst, abs(p - diss) / max(abs(p), abs(diss), 1e-300) if (p or diss) else 0.0)
    return worst


def _exact(x):
    """A float's exact rational value (sympy.Rational from fractions.Fraction: no guessing of a 'nicer' rational)."""
    import sympy as sp
    from fractions import Fraction
    f = Fraction(x)
    return sp.Rational(f.numerator, f.denominator)


def _sympy_solve_circuit(els, extra=()):
    """INDEPENDENT reference: Kirchhoff's current law written as one equation per node with every element's law
    substituted, the voltage sources as constraint equations, solved exactly in rational arithmetic by sympy.solve."""
    import sympy as sp
    els = list(els) + list(extra)
    nodes = sorted({n for e in els for n in (e[2], e[3]) if n != "0"}, key=_node_key)
    v = {n: sp.Symbol(f"v_{n}") for n in nodes}
    v["0"] = sp.Integer(0)
    isrc = {e[1]: sp.Symbol(f"i_{e[1]}") for e in els if e[0] == "V"}
    eqs = []
    for n in nodes:
        leaving = 0
        for t, name, a, b, val in els:
            q = _exact(val)
            if t == "R":
                i = (v[a] - v[b]) / q
            elif t == "V":
                i = isrc[name]
            else:
                i = q
            if a == n:
                leaving += i
            elif b == n:
                leaving -= i
        eqs.append(sp.Eq(leaving, 0))
    for t, name, a, b, val in els:
        if t == "V":
            eqs.append(sp.Eq(v[a] - v[b], _exact(val)))
    unknowns = [v[n] for n in nodes] + list(isrc.values())
    M, rhs = sp.linear_eq_to_matrix(eqs, unknowns)
    try:
        s = dict(zip(unknowns, M.LUsolve(rhs)))
    except (ValueError, ZeroDivisionError, sp.matrices.common.NonInvertibleMatrixError) as exc:
        raise ValueError("the circuit equations have no unique solution") from exc
    return {n: s[v[n]] for n in nodes} | {"0": sp.Integer(0)}, {k: s[x] for k, x in isrc.items()}


def _reference_nodes(netlist):
    els = _netlist(netlist)
    volts, cur = _sympy_solve_circuit(els)
    p = 0
    for t, name, a, b, val in els:
        if t == "V":
            p += -val * float(cur[name])
        elif t == "I":
            p += val * float(volts[b] - volts[a])
    return {"node_voltages": {n: float(x) for n, x in volts.items() if n != "0"},
            "source_currents": {k: float(x) for k, x in cur.items()}, "power_delivered": float(p)}


def _wrong_nodes(netlist):
    """A deliberately wrong variant: the current sources' direction reversed (SPICE convention misread) and every
    voltage source's polarity flipped."""
    els = [(t, n, b, a, v) if t in "VI" else (t, n, a, b, v) for t, n, a, b, v in _netlist(netlist)]
    volts, cur = _mna(els)
    return {"node_voltages": {n: v for n, v in volts.items() if n != "0"}, "source_currents": cur,
            "power_delivered": _power_delivered(els, volts, cur)}


def _random_netlist(rng, current_source=True, port=False):
    for _ in range(100):
        n = rng.randint(2, 5)
        nodes = [str(i) for i in range(n + 1)]
        els, k = [], 1
        for i in range(1, n + 1):
            els.append(("R", f"R{k}", nodes[i], nodes[rng.randrange(0, i)], _e12(rng, 1, 4)))
            k += 1
        for _ in range(rng.randint(0, n)):
            a, b = rng.sample(nodes, 2)
            els.append(("R", f"R{k}", a, b, _e12(rng, 1, 4)))
            k += 1
        a, b = rng.sample(nodes, 2)
        els.append(("V", "V1", a, b, rng.choice([1, -1]) * round(rng.uniform(1, 24), 1)))
        if current_source and rng.random() < 0.5:
            c, d = rng.sample(nodes, 2)
            els.append(("I", "I1", c, d, rng.choice([1, -1]) * round(rng.uniform(0.001, 0.05), 4)))
        text = "; ".join(f"{name} {a} {b} {val:g}" for _t, name, a, b, val in els)
        if not port:
            return {"netlist": text}
        pa, pb = rng.sample(nodes, 2)
        if {pa, pb} == {a, b}:                       # the source directly across the terminals: no resistance
            continue
        return {"netlist": text, "node_a": pa, "node_b": pb}
    raise ValueError("no admissible circuit drawn")


def parse_netlist(question):
    if re.search(r"\b(thevenin|th[eé]venin|norton)\b", question, re.I) or re.search(_AC_HINT, question, re.I):
        return None
    m = re.search(r"\b(?:node voltages|voltage at every node|nodal analysis)\b.*?\bcircuit\s*:\s*(.+?)\s*$", question, re.I | re.S)
    return {"netlist": m.group(1)} if m else None


PROCEDURES["dc_node_voltages"] = {
    "name": "Node voltages of a DC resistive circuit from its netlist", "topic": "DC circuits / mesh and nodal analysis",
    "purpose": "Every node voltage, every voltage-source current and the power delivered, for a circuit of resistors and independent sources written as a netlist.",
    "inputs": [{"symbol": "netlist", "type": "text", "min_length": 5, "max_length": 2000, "required": True,
                "meaning": "elements separated by ';': R<name> <node> <node> <ohms>, V<name> <node+> <node-> <volts>, "
                           "I<name> <node> <node> <amperes> (flowing from the first node through the source to the second); node 0 is ground"}],
    "outputs": [{"symbol": "node_voltages", "kind": "number", "meaning": "voltage of every node relative to node 0, in volts"},
                {"symbol": "source_currents", "kind": "number", "meaning": "current through each voltage source from its + node to its - node, in amperes"},
                {"symbol": "power_delivered", "kind": "power", "meaning": "total power delivered by the sources (= power dissipated in the resistors)"}],
    "primary": "power_delivered", "conditions": ["a ground node 0", "every resistance positive",
                                                 "a unique solution (no floating node, no loop of voltage sources)"],
    "method": "modified nodal analysis solved by Gaussian elimination", "latex": r"\begin{bmatrix} G & B \\ B^T & 0 \end{bmatrix}\begin{bmatrix} v \\ i \end{bmatrix} = \begin{bmatrix} i_s \\ e \end{bmatrix}",
    "run": dc_node_voltages, "verify": _verify_nodes, "parse": parse_netlist, "reference": _reference_nodes,
    "wrong": _wrong_nodes, "random_inputs": lambda rng: _random_netlist(rng),
    "templates": ["Find the node voltages of the circuit: V1 1 0 12; R1 1 2 1000; R2 2 0 2000.",
                  "Use nodal analysis on the circuit: I1 0 1 0.01; R1 1 0 100; R2 1 2 200; R3 2 0 300."],
    "examples": [{"inputs": {"netlist": "V1 1 0 12; R1 1 2 1000; R2 2 0 2000"},
                  "outputs": {"node_voltages": {"1": 12.0, "2": 8.0}, "source_currents": {"V1": -0.004}, "power_delivered": 0.048}},
                 {"inputs": {"netlist": "I1 0 1 0.01; R1 1 0 100; R2 1 2 200; R3 2 0 300"},
                  "outputs": {"node_voltages": {"1": 5 / 6, "2": 0.5}}}],
    "questions": [{"q": "Find the node voltages of the circuit: V1 1 0 12; R1 1 2 1000; R2 2 0 2000.",
                   "outputs": {"node_voltages": {"1": 12.0, "2": 8.0}}},
                  {"q": "Use nodal analysis on the circuit: I1 0 1 0.01; R1 1 0 100; R2 1 2 200; R3 2 0 300.",
                   "outputs": {"node_voltages": {"1": 5 / 6, "2": 0.5}}},
                  {"q": "Find the node voltages of the circuit: V1 1 0 12; R1 1 2 1000; R2 3 4 2000.", "outcome": "INVALID_INPUT"},
                  {"q": "Find the node voltages of the circuit: V1 1 0 12; R1 1 2 -5; R2 2 0 2000.", "outcome": "INVALID_INPUT"}]}


def _thevenin(netlist, node_a, node_b):
    els = _netlist(netlist)
    nodes = {n for e in els for n in (e[2], e[3])}
    for n in (node_a, node_b):
        if n not in nodes:
            raise ValueError(f"node {n} is not in the circuit")
    if node_a == node_b:
        raise ValueError("the two terminals must be different nodes")
    volts, _ = _mna(els)
    vth = volts[node_a] - volts[node_b]
    if abs(vth) <= 1e-12 * max(abs(v) for v in volts.values()):
        vth = 0.0                                    # two nodes at one potential: exactly zero
    dead = [(t, n, a, b, 0.0) for t, n, a, b, v in els if t == "V"] + [e for e in els if e[0] == "R"]
    dead.append(("I", "Itest", node_b, node_a, 1.0))  # 1 A into node a, out of node b
    v2, _ = _mna(dead)
    rth = v2[node_a] - v2[node_b]
    scale = max(e[4] for e in els if e[0] == "R")
    if abs(rth) <= 1e-12 * scale:
        raise ValueError("an ideal voltage source is across the terminals: the Thevenin resistance is 0")
    return vth, rth


def thevenin_equivalent(netlist, node_a, node_b) -> dict:
    vth, rth = _thevenin(netlist, node_a, node_b)
    return {"Vth": vth, "Rth": rth}


def norton_equivalent(netlist, node_a, node_b) -> dict:
    vth, rth = _thevenin(netlist, node_a, node_b)
    return {"In": vth / rth, "Rn": rth}


def _verify_thevenin(inputs, outputs):
    """The full circuit (sources on) solved twice: open, the terminal voltage must be Vth; with a 1 A test current
    driven into node a and out of node b, it must be Vth + Rth (superposition) — the two pin both Vth and Rth."""
    vth, rth = outputs["Vth"], outputs["Rth"]
    a, b = inputs["node_a"], inputs["node_b"]
    els = _netlist(inputs["netlist"])
    v1, _ = _mna(els)
    v2, _ = _mna(els + [("I", "Itest_check", b, a, 1.0)])
    o1, o2 = v1[a] - v1[b], v2[a] - v2[b]
    s = max(abs(o1), abs(o2), abs(vth) + abs(rth), 1e-300)
    return abs(o1 - vth) / s + abs(o2 - (vth + rth)) / s


def _verify_norton(inputs, outputs):
    return _verify_thevenin(inputs, {"Vth": outputs["In"] * outputs["Rn"], "Rth": outputs["Rn"]})


def _reference_thevenin_pair(netlist, node_a, node_b):
    """INDEPENDENT reference (sympy, exact): the open-circuit voltage, and the short-circuit current with a 0 V source
    placed across the terminals; Rth = Voc/Isc (if Isc = 0, a 1 V test source with the independent sources removed)."""
    import sympy as sp
    els = _netlist(netlist)
    volts, _ = _sympy_solve_circuit(els)
    voc = volts[node_a] - volts[node_b]
    _, cur = _sympy_solve_circuit(els, [("V", "Vshort_ref", node_a, node_b, 0.0)])
    isc = cur["Vshort_ref"]                         # current from a through the short to b
    if isc != 0:
        rth = voc / isc
    else:
        dead = [(t, n, a, b, 0.0) for t, n, a, b, v in els if t == "V"] + [e for e in els if e[0] == "R"]
        _, c2 = _sympy_solve_circuit(dead, [("V", "Vtest_ref", node_a, node_b, 1.0)])
        rth = sp.Integer(1) / (-c2["Vtest_ref"])
    return float(voc), float(rth)


def _wrong_thevenin(netlist, node_a, node_b):
    """A deliberately wrong variant: Rth found with the current sources shorted instead of opened."""
    els = _netlist(netlist)
    vth = _thevenin(netlist, node_a, node_b)[0]           # the same (correct) open-circuit voltage
    dead = [(t, n, a, b, 0.0) for t, n, a, b, v in els if t == "V"] + [e for e in els if e[0] == "R"] + \
        [("V", "short_" + n, a, b, 0.0) for t, n, a, b, v in els if t == "I"]
    dead.append(("I", "Itest", node_b, node_a, 1.0))
    try:
        v2, _ = _mna(dead)
        rth = v2[node_a] - v2[node_b]
    except ValueError:
        rth = 0.5
    if not any(e[0] == "I" for e in els):            # no current source: misread the voltage sources as opened
        dead = [e for e in els if e[0] == "R"] + [("I", "Itest", node_b, node_a, 1.0)]
        try:
            v2, _ = _mna(dead)
            rth = v2[node_a] - v2[node_b]
        except ValueError:
            rth = 2 * rth + 1
    return vth, rth


def parse_thevenin(kind):
    def parse(question):
        if not re.search(rf"\b{kind}\b", question, re.I) or re.search(_AC_HINT, question, re.I):
            return None
        m = re.search(r"\bbetween\s+nodes?\s+(\w+)\s+and\s+(\w+)\b.*?\bcircuit\s*:\s*(.+?)\s*$", question, re.I | re.S)
        if not m:
            m2 = re.search(r"\bcircuit\s*:\s*(.+?)\s*$", question, re.I | re.S)
            return {"netlist": m2.group(1)} if m2 else None
        return {"node_a": m.group(1), "node_b": m.group(2), "netlist": m.group(3)}
    return parse


_PORT_INPUTS = [{"symbol": "netlist", "type": "text", "min_length": 5, "max_length": 2000, "required": True,
                 "meaning": "elements separated by ';': R<name> <node> <node> <ohms>, V<name> <node+> <node-> <volts>, "
                            "I<name> <node> <node> <amperes>; node 0 is ground"},
                {"symbol": "node_a", "type": "text", "min_length": 1, "max_length": 20, "required": True,
                 "meaning": "the terminal node (the + side of the equivalent)"},
                {"symbol": "node_b", "type": "text", "min_length": 1, "max_length": 20, "required": True,
                 "meaning": "the reference terminal node"}]

PROCEDURES["thevenin_equivalent"] = {
    "name": "Thevenin equivalent of a DC circuit between two nodes", "topic": "DC circuits / Thevenin and Norton equivalents",
    "purpose": "The Thevenin voltage and resistance seen between two nodes of a resistive circuit with independent sources.",
    "inputs": _PORT_INPUTS,
    "outputs": [{"symbol": "Vth", "kind": "voltage", "meaning": "Thevenin (open-circuit) voltage, node a relative to node b"},
                {"symbol": "Rth", "kind": "resistance", "meaning": "Thevenin resistance"}],
    "primary": "Vth", "conditions": ["both nodes in the circuit", "no ideal voltage source directly across the terminals",
                                     "a unique solution (no floating node)"],
    "method": "open-circuit voltage by modified nodal analysis; resistance by a 1 A test current with the independent sources set to zero",
    "latex": r"V_{th} = v_a - v_b\big|_{open},\quad R_{th} = \frac{v_a - v_b}{1\,\mathrm{A}}\big|_{sources\ off}",
    "run": thevenin_equivalent, "verify": _verify_thevenin, "parse": parse_thevenin("thevenin"),
    "reference": lambda netlist, node_a, node_b: dict(zip(("Vth", "Rth"), _reference_thevenin_pair(netlist, node_a, node_b))),
    "wrong": lambda netlist, node_a, node_b: dict(zip(("Vth", "Rth"), _wrong_thevenin(netlist, node_a, node_b))),
    "random_inputs": lambda rng: _random_netlist(rng, port=True),
    "templates": ["Find the Thevenin equivalent between nodes 2 and 0 of the circuit: V1 1 0 12; R1 1 2 1000; R2 2 0 2000.",
                  "What is the Thevenin equivalent between nodes 3 and 0 of the circuit: V1 1 0 10; R1 1 2 100; R2 2 0 100; R3 2 3 50; R4 3 0 200."],
    "examples": [{"inputs": {"netlist": "V1 1 0 12; R1 1 2 1000; R2 2 0 2000", "node_a": "2", "node_b": "0"},
                  "outputs": {"Vth": 8.0, "Rth": 2000 / 3}},
                 {"inputs": {"netlist": "I1 0 1 0.01; R1 1 0 100; R2 1 2 200", "node_a": "2", "node_b": "0"},
                  "outputs": {"Vth": 1.0, "Rth": 300.0}}],
    "questions": [{"q": "Find the Thevenin equivalent between nodes 2 and 0 of the circuit: V1 1 0 12; R1 1 2 1000; R2 2 0 2000.",
                   "outputs": {"Vth": 8.0, "Rth": 2000 / 3}},
                  {"q": "What is the Thevenin equivalent between nodes 3 and 0 of the circuit: V1 1 0 10; R1 1 2 100; R2 2 0 100; R3 2 3 50; R4 3 0 200.",
                   "outputs": {"Vth": 10 / 3, "Rth": 200 / 3}},
                  {"q": "Find the Thevenin equivalent between nodes 1 and 0 of the circuit: V1 1 0 12; R1 1 2 1000; R2 2 0 2000.", "outcome": "INVALID_INPUT"},
                  {"q": "Find the Thevenin equivalent of the circuit: V1 1 0 12; R1 1 2 1000; R2 2 0 2000.", "outcome": "NEEDS_INPUT"}]}

PROCEDURES["norton_equivalent"] = {
    "name": "Norton equivalent of a DC circuit between two nodes", "topic": "DC circuits / Thevenin and Norton equivalents",
    "purpose": "The Norton current and resistance seen between two nodes of a resistive circuit with independent sources.",
    "inputs": _PORT_INPUTS,
    "outputs": [{"symbol": "In", "kind": "current", "meaning": "Norton (short-circuit) current from node a to node b"},
                {"symbol": "Rn", "kind": "resistance", "meaning": "Norton resistance (equal to the Thevenin resistance)"}],
    "primary": "In", "conditions": ["both nodes in the circuit", "no ideal voltage source directly across the terminals",
                                    "a unique solution (no floating node)"],
    "method": "Thevenin voltage and resistance by modified nodal analysis, then I_N = V_th / R_th",
    "latex": r"I_N = \frac{V_{th}}{R_{th}},\quad R_N = R_{th}",
    "run": norton_equivalent, "verify": _verify_norton, "parse": parse_thevenin("norton"),
    "reference": lambda netlist, node_a, node_b: (lambda v, r: {"In": v / r, "Rn": r})(*_reference_thevenin_pair(netlist, node_a, node_b)),
    "wrong": lambda netlist, node_a, node_b: (lambda v, r: {"In": v / r if r else 1.0, "Rn": r})(*_wrong_thevenin(netlist, node_a, node_b)),
    "random_inputs": lambda rng: _random_netlist(rng, port=True),
    "templates": ["Find the Norton equivalent between nodes 2 and 0 of the circuit: V1 1 0 12; R1 1 2 1000; R2 2 0 2000."],
    "examples": [{"inputs": {"netlist": "V1 1 0 12; R1 1 2 1000; R2 2 0 2000", "node_a": "2", "node_b": "0"},
                  "outputs": {"In": 0.012, "Rn": 2000 / 3}}],
    "questions": [{"q": "Find the Norton equivalent between nodes 2 and 0 of the circuit: V1 1 0 12; R1 1 2 1000; R2 2 0 2000.",
                   "outputs": {"In": 0.012, "Rn": 2000 / 3}},
                  {"q": "Find the Norton equivalent between nodes 1 and 0 of the circuit: V1 1 0 12; R1 1 0 1000.", "outcome": "INVALID_INPUT"}]}


# ============================================================================================ delta-wye conversion
def delta_to_wye(Rab, Rbc, Rca) -> dict:
    for n, v in (("Rab", Rab), ("Rbc", Rbc), ("Rca", Rca)):
        if not (v > 0 and math.isfinite(v)):
            raise ValueError(f"{n} must be a positive resistance")
    s = Rab + Rbc + Rca
    return {"Ra": Rab * Rca / s, "Rb": Rab * Rbc / s, "Rc": Rbc * Rca / s}


def wye_to_delta(Ra, Rb, Rc) -> dict:
    for n, v in (("Ra", Ra), ("Rb", Rb), ("Rc", Rc)):
        if not (v > 0 and math.isfinite(v)):
            raise ValueError(f"{n} must be a positive resistance")
    p = Ra * Rb + Rb * Rc + Rc * Ra
    return {"Rab": p / Rc, "Rbc": p / Ra, "Rca": p / Rb}


def _pair_resistances_delta(Rab, Rbc, Rca):
    par = lambda x, y: x * y / (x + y)             # noqa: E731
    return {"ab": par(Rab, Rbc + Rca), "bc": par(Rbc, Rab + Rca), "ca": par(Rca, Rab + Rbc)}


def _verify_dw(inputs, outputs, forward):
    """The two networks must look the same from every pair of terminals (third terminal open)."""
    if forward:
        d, y = (inputs["Rab"], inputs["Rbc"], inputs["Rca"]), (outputs["Ra"], outputs["Rb"], outputs["Rc"])
    else:
        y, d = (inputs["Ra"], inputs["Rb"], inputs["Rc"]), (outputs["Rab"], outputs["Rbc"], outputs["Rca"])
    pd = _pair_resistances_delta(*d)
    py = {"ab": y[0] + y[1], "bc": y[1] + y[2], "ca": y[2] + y[0]}
    return max(_rel(pd[k], py[k]) for k in pd)


def _reference_delta_to_wye(Rab, Rbc, Rca):
    """INDEPENDENT reference: the terminal-pair effective resistances of the delta from the pseudo-inverse of its
    conductance Laplacian (numpy), then the wye arms from Ra+Rb, Rb+Rc, Rc+Ra solved as a linear system (numpy)."""
    import numpy as np
    L = np.zeros((3, 3))
    for (i, j), r in (((0, 1), Rab), ((1, 2), Rbc), ((2, 0), Rca)):
        L[i, i] += 1 / r
        L[j, j] += 1 / r
        L[i, j] -= 1 / r
        L[j, i] -= 1 / r
    P = np.linalg.pinv(L)
    eff = lambda i, j: P[i, i] + P[j, j] - 2 * P[i, j]   # noqa: E731
    arms = np.linalg.solve(np.array([[1, 1, 0], [0, 1, 1], [1, 0, 1]], float), np.array([eff(0, 1), eff(1, 2), eff(2, 0)]))
    return {"Ra": float(arms[0]), "Rb": float(arms[1]), "Rc": float(arms[2])}


def _reference_wye_to_delta(Ra, Rb, Rc):
    """INDEPENDENT reference: Kron reduction (Schur complement, numpy) of the wye's 4-node conductance matrix onto
    its three terminals; the delta's branch conductances are minus the off-diagonal entries."""
    import numpy as np
    G = np.zeros((4, 4))                              # nodes a, b, c, centre
    for i, r in enumerate((Ra, Rb, Rc)):
        G[i, i] += 1 / r
        G[3, 3] += 1 / r
        G[i, 3] -= 1 / r
        G[3, i] -= 1 / r
    K = G[:3, :3] - np.outer(G[:3, 3], G[3, :3]) / G[3, 3]
    return {"Rab": float(-1 / K[0, 1]), "Rbc": float(-1 / K[1, 2]), "Rca": float(-1 / K[2, 0])}


def _parse_dw(names, word):
    def parse(question):
        q = question.lower()
        if not re.search(rf"\b(?:{word[0]})\b.*\bto\s+(?:an?\s+)?(?:equivalent\s+)?(?:{word[1]})\b", q):
            return None
        vals = _named_values(question, names, "resistance")
        return vals or None
    return parse


def _rand3(rng):
    return [_e12(rng, 0, 4) for _ in range(3)]


PROCEDURES["delta_to_wye"] = {
    "name": "Delta-to-wye (Pi-to-T) resistor transformation", "topic": "DC circuits / network transformations",
    "purpose": "The three wye (star) resistances equivalent to a delta (triangle) of resistances between terminals a, b, c.",
    "inputs": [{"symbol": "Rab", "kind": "resistance", "meaning": "delta resistance between terminals a and b", "required": True},
               {"symbol": "Rbc", "kind": "resistance", "meaning": "delta resistance between terminals b and c", "required": True},
               {"symbol": "Rca", "kind": "resistance", "meaning": "delta resistance between terminals c and a", "required": True}],
    "outputs": [{"symbol": "Ra", "kind": "resistance", "meaning": "wye arm at terminal a"},
                {"symbol": "Rb", "kind": "resistance", "meaning": "wye arm at terminal b"},
                {"symbol": "Rc", "kind": "resistance", "meaning": "wye arm at terminal c"}],
    "primary": "Ra", "conditions": ["every resistance positive"],
    "method": "each arm is the product of the two adjacent delta resistances over their sum",
    "latex": r"R_a = \frac{R_{ab}R_{ca}}{R_{ab}+R_{bc}+R_{ca}}",
    "run": delta_to_wye, "verify": lambda i, o: _verify_dw(i, o, True), "parse": _parse_dw(["Rab", "Rbc", "Rca"], ("delta", "wye|star")),
    "reference": _reference_delta_to_wye,
    "wrong": lambda Rab, Rbc, Rca: {"Ra": Rab * Rbc / (Rab + Rbc + Rca), "Rb": Rbc * Rca / (Rab + Rbc + Rca), "Rc": Rca * Rab / (Rab + Rbc + Rca)},
    "random_inputs": lambda rng: dict(zip(("Rab", "Rbc", "Rca"), _rand3(rng))),
    "templates": ["Convert a delta with Rab = 10 ohm, Rbc = 20 ohm, Rca = 30 ohm to a wye."],
    "examples": [{"inputs": {"Rab": 10, "Rbc": 20, "Rca": 30}, "outputs": {"Ra": 5.0, "Rb": 10 / 3, "Rc": 10.0}},
                 {"inputs": {"Rab": 30, "Rbc": 30, "Rca": 30}, "outputs": {"Ra": 10.0, "Rb": 10.0, "Rc": 10.0}}],
    "questions": [{"q": "Convert a delta with Rab = 10 ohm, Rbc = 20 ohm, Rca = 30 ohm to a wye.",
                   "outputs": {"Ra": 5.0, "Rb": 10 / 3, "Rc": 10.0}},
                  {"q": "Convert a delta network with Rab = 3 kohm, Rbc = 3 kohm, Rca = 3 kohm to an equivalent star.",
                   "outputs": {"Ra": 1000.0, "Rb": 1000.0, "Rc": 1000.0}},
                  {"q": "Convert a delta with Rab = 10 ohm, Rbc = 20 ohm to a wye.", "outcome": "NEEDS_INPUT"}]}

PROCEDURES["wye_to_delta"] = {
    "name": "Wye-to-delta (T-to-Pi) resistor transformation", "topic": "DC circuits / network transformations",
    "purpose": "The three delta (triangle) resistances equivalent to a wye (star) of resistances at terminals a, b, c.",
    "inputs": [{"symbol": "Ra", "kind": "resistance", "meaning": "wye arm at terminal a", "required": True},
               {"symbol": "Rb", "kind": "resistance", "meaning": "wye arm at terminal b", "required": True},
               {"symbol": "Rc", "kind": "resistance", "meaning": "wye arm at terminal c", "required": True}],
    "outputs": [{"symbol": "Rab", "kind": "resistance", "meaning": "delta resistance between a and b"},
                {"symbol": "Rbc", "kind": "resistance", "meaning": "delta resistance between b and c"},
                {"symbol": "Rca", "kind": "resistance", "meaning": "delta resistance between c and a"}],
    "primary": "Rab", "conditions": ["every resistance positive"],
    "method": "each delta resistance is the sum of pairwise products of the arms over the opposite arm",
    "latex": r"R_{ab} = \frac{R_aR_b + R_bR_c + R_cR_a}{R_c}",
    "run": wye_to_delta, "verify": lambda i, o: _verify_dw(i, o, False), "parse": _parse_dw(["Ra", "Rb", "Rc"], ("wye|star", "delta")),
    "reference": _reference_wye_to_delta,
    "wrong": lambda Ra, Rb, Rc: (lambda p: {"Rab": p / Ra, "Rbc": p / Rb, "Rca": p / Rc})(Ra * Rb + Rb * Rc + Rc * Ra),
    "random_inputs": lambda rng: dict(zip(("Ra", "Rb", "Rc"), _rand3(rng))),
    "templates": ["Convert a wye with Ra = 5 ohm, Rb = 10 ohm, Rc = 20 ohm to a delta."],
    "examples": [{"inputs": {"Ra": 5, "Rb": 10, "Rc": 20}, "outputs": {"Rab": 17.5, "Rbc": 70.0, "Rca": 35.0}},
                 {"inputs": {"Ra": 10, "Rb": 10, "Rc": 10}, "outputs": {"Rab": 30.0, "Rbc": 30.0, "Rca": 30.0}}],
    "questions": [{"q": "Convert a wye with Ra = 5 ohm, Rb = 10 ohm, Rc = 20 ohm to a delta.",
                   "outputs": {"Rab": 17.5, "Rbc": 70.0, "Rca": 35.0}},
                  {"q": "Convert a star network with Ra = 1 kohm, Rb = 1 kohm, Rc = 1 kohm to a delta.",
                   "outputs": {"Rab": 3000.0, "Rbc": 3000.0, "Rca": 3000.0}},
                  {"q": "Convert a wye with Ra = 5 ohm, Rb = 0 ohm, Rc = 20 ohm to a delta.", "outcome": "INVALID_INPUT"}]}


# ================================================================================ polynomials in s (control systems)
def _padd(a, b):
    n = max(len(a), len(b))
    a, b = [0.0] * (n - len(a)) + list(a), [0.0] * (n - len(b)) + list(b)
    return [x + y for x, y in zip(a, b)]


def _pmul(a, b):
    out = [0.0] * (len(a) + len(b) - 1)
    for i, x in enumerate(a):
        for j, y in enumerate(b):
            out[i + j] += x * y
    return out


def _pval(c, z):
    acc = 0j if isinstance(z, complex) else 0.0
    for x in c:
        acc = acc * z + x
    return acc


def _pder(c):
    n = len(c) - 1
    return [x * (n - i) for i, x in enumerate(c[:-1])] or [0.0]


def _ptrim(c):
    c = [float(x) for x in c]
    while len(c) > 1 and c[0] == 0:
        c = c[1:]
    return c


def _roots(coeffs):
    """Every root of c0 s^n + ... + cn (highest power first), by the Aberth-Ehrlich simultaneous iteration with
    Newton polishing (plain Python, complex arithmetic). Roots at 0 are split off exactly."""
    c = _ptrim(coeffs)
    if len(c) < 2:
        return []
    zeros = 0
    while len(c) > 1 and c[-1] == 0:
        c.pop()
        zeros += 1
    n = len(c) - 1
    if n == 0:
        return [0.0] * zeros
    a = [x / c[0] for x in c]
    d = _pder(a)
    r = abs(a[-1]) ** (1.0 / n) or 1.0
    z = [r * cmath.exp(1j * (2 * math.pi * k / n + 0.4)) for k in range(n)]
    for _ in range(800):
        biggest = 0.0
        for i in range(n):
            p = _pval(a, z[i])
            if p == 0:
                continue
            dp = _pval(d, z[i])
            ratio = p / dp if dp != 0 else p * 1e-3
            s = sum(1.0 / (z[i] - z[j]) for j in range(n) if j != i and z[i] != z[j])
            w = ratio / (1 - ratio * s)
            z[i] -= w
            biggest = max(biggest, abs(w) / max(1.0, abs(z[i])))
        if biggest < 1e-16:
            break
    out = []
    for x in z:                                      # two Newton steps on the polynomial itself
        for _ in range(2):
            dp = _pval(d, x)
            if dp != 0:
                x = x - _pval(a, x) / dp
        out.append(x)
    return out + [0.0] * zeros


def _canonical_roots(roots, tol=1e-9):
    """Real roots as floats; complex roots paired with their conjugates (equal real parts); sorted by real part,
    then imaginary part."""
    real, cplx = [], []
    for r in roots:
        r = complex(r)
        (real if abs(r.imag) <= tol * max(1.0, abs(r)) else cplx).append(r)
    out = [complex(x.real, 0.0) for x in real]
    ups = sorted((x for x in cplx if x.imag > 0), key=lambda x: (x.real, x.imag))
    downs = [x for x in cplx if x.imag < 0]
    for u in ups:
        if not downs:
            break
        j = min(range(len(downs)), key=lambda k: abs(downs[k] - u.conjugate()))
        dn = downs.pop(j)
        re_, im = (u.real + dn.real) / 2, (u.imag - dn.imag) / 2
        out += [complex(re_, -im), complex(re_, im)]
    out += downs
    out.sort(key=lambda x: (x.real, x.imag))
    return [x.real if x.imag == 0 else x for x in out]


def _spoly(text):
    """A polynomial in s written as a person would: 's^3 + 6s^2 + 11s + 6', 's(s+1)(s+5)', '(s+2)^2', '10' ->
    coefficients, highest power first. Grammar: sums of products of numbers, s, s^k and parenthesised polynomials."""
    t = text.replace(" ", "").replace("**", "^").replace("·", "*").replace("−", "-")
    toks = re.findall(r"\d+(?:\.\d+)?|\.\d+|s|\^|\(|\)|\+|-|\*|.", t)
    if not toks or any(x not in ("s", "^", "(", ")", "+", "-", "*") and not re.fullmatch(r"\d+(?:\.\d+)?|\.\d+", x) for x in toks):
        raise ValueError(f"cannot read the polynomial '{text}' (use s, numbers, + - * ^ and parentheses)")
    pos = [0]

    def peek():
        return toks[pos[0]] if pos[0] < len(toks) else None

    def take():
        if pos[0] >= len(toks):
            raise ValueError(f"the polynomial '{text}' ends too early")
        pos[0] += 1
        return toks[pos[0] - 1]

    def expr():
        sign = 1.0
        if peek() in ("+", "-"):
            sign = -1.0 if take() == "-" else 1.0
        p = [sign * x for x in term()]
        while peek() in ("+", "-"):
            sg = -1.0 if take() == "-" else 1.0
            p = _padd(p, [sg * x for x in term()])
        return p

    def term():
        p = factor()
        while peek() is not None and peek() not in ("+", "-", ")"):
            if peek() == "*":
                take()
            p = _pmul(p, factor())
        return p

    def factor():
        x = take()
        if x == "s":
            p = [1.0, 0.0]
        elif x == "(":
            p = expr()
            if take() != ")":
                raise ValueError(f"unbalanced parentheses in '{text}'")
        elif re.fullmatch(r"\d+(?:\.\d+)?|\.\d+", x):
            p = [float(x)]
        else:
            raise ValueError(f"unexpected '{x}' in '{text}'")
        if peek() == "^":
            take()
            k = take()
            if not k.isdigit() or int(k) > 20:
                raise ValueError("powers must be whole numbers up to 20")
            q = [1.0]
            for _ in range(int(k)):
                q = _pmul(q, p)
            p = q
        return p
    p = expr()
    if pos[0] != len(toks):
        raise ValueError(f"unexpected '{toks[pos[0]]}' in '{text}'")
    return _ptrim(p)


def _split_tf(text):
    depth, cut = 0, None
    for i, ch in enumerate(text):
        depth += {"(": 1, ")": -1}.get(ch, 0)
        if ch == "/" and depth == 0:
            if cut is not None:
                raise ValueError("write the transfer function as one numerator over one denominator")
            cut = i
    if cut is None:
        return _spoly(text), [1.0]
    return _spoly(text[:cut]), _spoly(text[cut + 1:])


def _fmt_poly(c):
    return "[" + ", ".join(f"{x:g}" for x in c) + "]"


def _tf_from_question(question):
    """'G(s) = 10/(s(s+1)(s+5))' (or L(s) =) -> (num, den) coefficient lists; a 'numerator [..] and denominator [..]'
    form is also read. None when no transfer function is written."""
    m = re.search(r"\b[GL]\(s\)\s*=\s*([0-9s.+\-*/^()\s]+?)\s*(?:,|\?|\.(?:\s|$)|;|\bat\b|\bwith\b|\bfor\b|$)", question)
    if m:
        try:
            num, den = _split_tf(m.group(1))
        except ValueError:
            return {"_bad": True}
        return {"num": _fmt_poly(num), "den": _fmt_poly(den)}
    n = re.search(r"\bnumerator\s*(\[[^\]]*\])", question, re.I)
    d = re.search(r"\bdenominator\s*(\[[^\]]*\])", question, re.I)
    if n or d:
        out = {}
        if n:
            out["num"] = n.group(1)
        if d:
            out["den"] = d.group(1)
        return out
    return None


def _check_tf(num, den):
    num, den = _ptrim(num), _ptrim(den)
    if all(x == 0 for x in den):
        raise ValueError("the denominator must not be zero")
    if all(x == 0 for x in num):
        raise ValueError("the numerator must not be zero")
    if len(num) > len(den):
        raise ValueError("the transfer function must be proper (numerator degree <= denominator degree)")
    for x in num + den:
        if not math.isfinite(x):
            raise ValueError("coefficients must be finite")
    return num, den


_TF_IN = [{"symbol": "num", "type": "vector", "min_length": 1, "max_length": 12, "required": True,
           "meaning": "numerator coefficients of the open-loop G(s), highest power of s first"},
          {"symbol": "den", "type": "vector", "min_length": 1, "max_length": 13, "required": True,
           "meaning": "denominator coefficients of G(s), highest power of s first"}]


def _random_loop(rng, kind="any"):
    """A realistic loop transfer function K*prod(s+z)/(s^r prod(s+p))."""
    r = rng.choice([0, 1, 1, 2]) if kind == "any" else kind
    poles = [round(rng.uniform(0.5, 20), 2) for _ in range(rng.randint(1, 3))]
    zeros = [round(rng.uniform(0.2, 10), 2) for _ in range(rng.randint(0, 1 if r < 2 else 1))]
    if r == 2 and not zeros:
        zeros = [round(rng.uniform(0.1, 0.5), 2)]
    K = round(rng.uniform(0.5, 50), 2)
    num = [K]
    for zz in zeros:
        num = _pmul(num, [1.0, zz])
    den = [1.0] + [0.0] * r
    for pp in poles:
        den = _pmul(den, [1.0, pp])
    return {"num": [round(x, 10) for x in num], "den": [round(x, 10) for x in den]}


# --------------------------------------------------------------------------------------------- Routh-Hurwitz
def routh_hurwitz(coefficients) -> dict:
    c = _ptrim(coefficients)
    if len(c) < 2:
        raise ValueError("give a polynomial of degree 1 or more")
    if any(not math.isfinite(x) for x in c):
        raise ValueError("coefficients must be finite")
    n = len(c) - 1
    width = n // 2 + 1
    r0 = c[0::2] + [0.0] * (width - len(c[0::2]))
    r1 = c[1::2] + [0.0] * (width - len(c[1::2]))
    rows = [r0, r1]
    scale = max(abs(x) for x in c)
    for _ in range(n - 1):
        a, b = rows[-2], rows[-1]
        if abs(b[0]) <= 1e-12 * scale:
            raise ValueError("a zero appears in the first column of the Routh array (a special case: roots on or "
                             "symmetric about the imaginary axis); this capability does not handle it")
        new = [(b[0] * a[i + 1] - a[0] * b[i + 1]) / b[0] for i in range(width - 1)] + [0.0]
        rows.append(new)
        scale = max(scale, max(abs(x) for x in new))
    first = [r[0] for r in rows]
    if any(abs(x) <= 1e-12 * scale for x in first):
        raise ValueError("a zero appears in the first column of the Routh array (a special case); this capability "
                         "does not handle it")
    changes = sum(1 for x, y in zip(first, first[1:]) if (x > 0) != (y > 0))
    return {"rhp_roots": float(changes), "stable": changes == 0, "first_column": first}


def _verify_routh(inputs, outputs):
    """The count of right-half-plane roots from the roots themselves (Aberth iteration), the stability flag, and the
    first column's sign pattern."""
    roots = _roots(inputs["coefficients"])
    rhp = sum(1 for r in roots if complex(r).real > 0)
    fc = outputs.get("first_column", [])
    ch = sum(1 for x, y in zip(fc, fc[1:]) if (x > 0) != (y > 0)) if fc else rhp
    col = 0.0
    if fc:                                           # the first column itself, entry by entry
        want = routh_hurwitz(inputs["coefficients"])["first_column"]
        col = 1.0 if len(want) != len(fc) else max(_rel(a, b) for a, b in zip(fc, want))
    return abs(outputs["rhp_roots"] - rhp) + abs(ch - rhp) + (outputs["stable"] != (rhp == 0)) + col


def _routh_wrong(coefficients):
    """A deliberately wrong variant: the 2x2 determinant of each new Routh row taken with the opposite sign."""
    c = _ptrim(coefficients)
    n = len(c) - 1
    width = n // 2 + 1
    rows = [c[0::2] + [0.0] * (width - len(c[0::2])), c[1::2] + [0.0] * (width - len(c[1::2]))]
    for _ in range(n - 1):
        a, b = rows[-2], rows[-1]
        if b[0] == 0:
            break
        rows.append([(a[0] * b[i + 1] - b[0] * a[i + 1]) / b[0] for i in range(width - 1)] + [0.0])
    first = [r[0] for r in rows]
    ch = sum(1 for x, y in zip(first, first[1:]) if (x > 0) != (y > 0))
    return {"rhp_roots": float(ch), "stable": ch == 0, "first_column": first}


def _reference_routh(coefficients):
    """INDEPENDENT reference: the eigenvalues of the companion matrix (numpy.roots) counted in the right half plane."""
    import numpy as np
    r = np.roots(np.array(_ptrim(coefficients), float))
    rhp = int(np.sum(r.real > 0))
    return {"rhp_roots": float(rhp), "stable": rhp == 0}


def _random_charpoly(rng):
    n = rng.randint(2, 7)
    for _ in range(100):
        c = [1.0] + [round(rng.uniform(-3, 12), 1) for _ in range(n)]
        if all(x != 0 for x in c):
            return {"coefficients": c}
    return {"coefficients": [1.0, 2.0, 3.0]}


def parse_routh(question):
    if not re.search(r"\brouth\b", question, re.I):
        return None
    m = re.search(r"\b(?:to|on|for|of)\s+(?:the\s+)?(?:characteristic\s+)?(?:polynomial\s+|equation\s+)?([0-9s.+\-*^()\s]*s[0-9s.+\-*^()\s]*?)(?:\s*=\s*0)?\s*[.?]?\s*$", question, re.I)
    if m:
        try:
            return {"coefficients": _fmt_poly(_spoly(m.group(1)))}
        except ValueError:
            return None
    b = re.search(r"\bcoefficients\s*(\[[^\]]*\])", question, re.I)
    return {"coefficients": b.group(1)} if b else None


PROCEDURES["routh_hurwitz"] = {
    "name": "Routh-Hurwitz stability test", "topic": "control systems / stability",
    "purpose": "The Routh array's first column, the number of roots in the right half plane and whether a characteristic polynomial is stable.",
    "inputs": [{"symbol": "coefficients", "type": "vector", "min_length": 2, "max_length": 16, "required": True,
                "meaning": "characteristic polynomial coefficients, highest power of s first"}],
    "outputs": [{"symbol": "rhp_roots", "kind": "number", "meaning": "number of roots with positive real part (sign changes in the first column)"},
                {"symbol": "stable", "kind": "boolean", "meaning": "true when every root has a negative real part"},
                {"symbol": "first_column", "kind": "number", "meaning": "the first column of the Routh array"}],
    "primary": "rhp_roots", "conditions": ["no zero in the first column of the Routh array (the special cases are not handled)"],
    "method": "Routh array built row by row; sign changes of its first column counted",
    "latex": r"b_i = \frac{a_{1}\,a_{0,i+1} - a_{0}\,a_{1,i+1}}{a_{1}}",
    "run": routh_hurwitz, "verify": _verify_routh, "parse": parse_routh, "reference": _reference_routh,
    "wrong": lambda coefficients: _routh_wrong(coefficients),
    "random_inputs": _random_charpoly,
    "templates": ["Use the Routh-Hurwitz criterion on s^4 + 2s^3 + 3s^2 + 4s + 5.",
                  "Apply the Routh test to the characteristic polynomial s^3 + 6s^2 + 11s + 6 = 0."],
    "examples": [{"inputs": {"coefficients": "1, 2, 3, 4, 5"}, "outputs": {"rhp_roots": 2.0, "stable": False}},
                 {"inputs": {"coefficients": [1, 6, 11, 6]}, "outputs": {"rhp_roots": 0.0, "stable": True, "first_column": [1.0, 6.0, 10.0, 6.0]}},
                 {"inputs": {"coefficients": [1, 1, 2, 24]}, "outputs": {"rhp_roots": 2.0}}],
    "questions": [{"q": "Use the Routh-Hurwitz criterion on s^4 + 2s^3 + 3s^2 + 4s + 5.", "outputs": {"rhp_roots": 2.0, "stable": False}},
                  {"q": "Apply the Routh test to the characteristic polynomial s^3 + 6s^2 + 11s + 6 = 0.", "outputs": {"rhp_roots": 0.0, "stable": True}},
                  {"q": "Apply the Routh test to (s+1)(s-2)(s+3).", "outputs": {"rhp_roots": 1.0}},
                  {"q": "Use the Routh-Hurwitz criterion on s^3 + 2s^2 + s + 2.", "outcome": "INVALID_INPUT"}]}


# --------------------------------------------------------------------------------------------- closed-loop poles
def closed_loop_poles(num, den, K=1.0) -> dict:
    num, den = _check_tf(num, den)
    if not math.isfinite(K):
        raise ValueError("the gain K must be finite")
    char = _padd(den, [K * x for x in num])
    char = _ptrim(char)
    if len(char) < 2:
        raise ValueError("the closed-loop characteristic polynomial has no roots")
    poles = _canonical_roots(_roots(char))
    mr = max(complex(p).real for p in poles)
    return {"poles": poles, "max_real_part": mr, "stable": mr < 0, "characteristic": char}


def _verify_poles(inputs, outputs):
    num, den = _ptrim(inputs["num"]), _ptrim(inputs["den"])
    char = _ptrim(_padd(den, [inputs.get("K", 1.0) * x for x in num]))
    poles = outputs["poles"]
    if len(poles) != len(char) - 1:
        return 1.0
    worst = 0.0
    for p in poles:
        p = complex(p)
        terms = [c * p ** (len(char) - 1 - i) for i, c in enumerate(char)]
        worst = max(worst, abs(sum(terms)) / (sum(abs(t) for t in terms) or 1.0))
    mr = max(complex(p).real for p in poles)
    return worst + abs(outputs["max_real_part"] - mr) + (outputs["stable"] != (mr < 0))


def _reference_poles(num, den, K=1.0):
    """INDEPENDENT reference: python-control closes the loop (control.feedback of K*G with unity feedback) and
    returns the closed-loop poles."""
    import control
    sys = control.feedback(control.tf([K * x for x in _ptrim(num)], _ptrim(den)), 1)
    p = _canonical_roots([complex(x) for x in sys.poles()])
    return {"poles": p, "max_real_part": max(complex(x).real for x in p)}


def parse_poles(question):
    if not re.search(r"\bclosed[- ]loop poles\b", question, re.I):
        return None
    tf = _tf_from_question(question)
    if tf is None or "_bad" in tf:
        return None
    k = re.search(r"\b(?:gain\s+)?K\s*=\s*(" + _NUMRE.replace("(?<![\\w.])", "") + r")", question)
    if k:
        tf["K"] = float(k.group(1))
    return tf


PROCEDURES["closed_loop_poles"] = {
    "name": "Closed-loop poles with unity negative feedback", "topic": "control systems / stability",
    "purpose": "The poles of K*G(s)/(1 + K*G(s)): the roots of den(s) + K*num(s), and whether the closed loop is stable.",
    "inputs": _TF_IN + [{"symbol": "K", "kind": "number", "required": False, "meaning": "loop gain multiplying G(s) (default 1)"}],
    "outputs": [{"symbol": "poles", "kind": "number", "meaning": "closed-loop poles (real first by real part, conjugate pairs together)"},
                {"symbol": "max_real_part", "kind": "number", "meaning": "largest real part of the poles (negative when stable)"},
                {"symbol": "stable", "kind": "boolean", "meaning": "true when every pole has a negative real part"},
                {"symbol": "characteristic", "kind": "number", "meaning": "closed-loop characteristic polynomial den + K num"}],
    "primary": "max_real_part", "conditions": ["a proper G(s)", "unity negative feedback"],
    "method": "roots of den(s) + K num(s) by the Aberth-Ehrlich iteration",
    "latex": r"1 + K\,G(s) = 0 \iff \mathrm{den}(s) + K\,\mathrm{num}(s) = 0",
    "run": closed_loop_poles, "verify": _verify_poles, "parse": parse_poles, "reference": _reference_poles,
    "wrong": lambda num, den, K=1.0: (lambda p: {"poles": p, "max_real_part": max(complex(x).real for x in p),
                                                 "stable": max(complex(x).real for x in p) < 0})(
        _canonical_roots(_roots(_ptrim(_padd(_ptrim(den), [-K * x for x in _ptrim(num)]))))),
    "random_inputs": lambda rng: dict(_random_loop(rng), K=round(rng.uniform(0.2, 5), 2)),
    "templates": ["Find the closed-loop poles of G(s) = 10/(s(s+1)(s+5)).",
                  "What are the closed-loop poles of G(s) = (s+2)/(s^2 + 3s) with K = 4?"],
    "examples": [{"inputs": {"num": "1", "den": "1, 3, 0"}, "outputs": {"poles": [-2.618033988749895, -0.3819660112501051], "stable": True}},
                 {"inputs": {"num": [10], "den": [1, 6, 5, 0]}, "outputs": {"stable": True}}],
    "questions": [{"q": "Find the closed-loop poles of G(s) = 2/(s^2 + 3s).", "outputs": {"poles": [-2.0, -1.0]}},
                  {"q": "What are the closed-loop poles of G(s) = 1/(s(s+2)) with K = 2?", "outputs": {"poles": [complex(-1, -1), complex(-1, 1)]}},
                  {"q": "Find the closed-loop poles of G(s) = 10/(s(s+1)(s+5)).", "outputs": {"stable": True}},
                  {"q": "Find the closed-loop poles of G(s) = (s^3+1)/(s+1).", "outcome": "INVALID_INPUT"}]}


# --------------------------------------------------------------------------------------------- frequency response
def _G(num, den, w):
    s = 1j * w
    d = _pval(den, s)
    if d == 0:
        raise ValueError("G(s) has a pole on the imaginary axis at this frequency")
    return _pval(num, s) / d


def _iw(c):
    """Coefficients of p(jw) as a polynomial in w (complex), highest first."""
    n = len(c) - 1
    return [x * (1j) ** (n - i) for i, x in enumerate(c)]


def _positive_real_roots(c, scale_w=1.0):
    out = []
    for r in _roots(c):
        r = complex(r)
        if r.real > 1e-9 * scale_w and abs(r.imag) <= 1e-7 * max(abs(r), 1e-300):
            out.append(r.real)
    out.sort()
    merged = []
    for x in out:
        if not merged or abs(x - merged[-1]) > 1e-7 * x:
            merged.append(x)
    return merged


def _polish(f, w):
    """Newton (secant) polishing of a simple real root."""
    for _ in range(30):
        h = 1e-7 * w
        fw = f(w)
        d = (f(w + h) - f(w - h)) / (2 * h)
        if d == 0:
            break
        step = fw / d
        w2 = w - step
        if w2 <= 0:
            break
        if abs(step) <= 1e-16 * w:
            w = w2
            break
        w = w2
    return w


def _gain_crossover(num, den):
    n_iw, d_iw = _iw(num), _iw(den)
    nn = [x.real for x in _pmul(n_iw, [y.conjugate() for y in n_iw])]
    dd = [x.real for x in _pmul(d_iw, [y.conjugate() for y in d_iw])]
    P = _ptrim(_padd(nn, [-x for x in dd]))
    ws = _positive_real_roots(P)
    return [_polish(lambda w: abs(_G(num, den, w)) - 1.0, w) for w in ws]


def _phase_crossover(num, den):
    n_iw, d_iw = _iw(num), _iw(den)
    nd = _pmul(n_iw, [y.conjugate() for y in d_iw])         # G = N conj(D) / |D|^2: Im(N conj D) = 0
    im = _ptrim([x.imag for x in nd])
    if len(im) < 2:
        return []
    ws = [w for w in _positive_real_roots(im) if _G(num, den, w).real < 0]
    return [_polish(lambda w: _G(num, den, w).imag / abs(_G(num, den, w)), w) for w in ws]


def phase_margin(num, den) -> dict:
    num, den = _check_tf(num, den)
    wc = _gain_crossover(num, den)
    if len(wc) != 1:
        raise ValueError("|G(jw)| crosses 1 " + ("nowhere" if not wc else f"{len(wc)} times") +
                         ": this capability needs exactly one gain crossover")
    g = _G(num, den, wc[0])
    pm = math.degrees(cmath.phase(g)) % 360.0 - 180.0
    return {"phase_margin": pm, "gain_crossover": wc[0]}


def gain_margin(num, den) -> dict:
    num, den = _check_tf(num, den)
    wp = _phase_crossover(num, den)
    if len(wp) != 1:
        raise ValueError(("the phase never reaches -180 degrees, so the gain margin is infinite" if not wp else
                          f"the phase crosses -180 degrees {len(wp)} times") + ": this capability needs exactly one phase crossover")
    gm = 1.0 / abs(_G(num, den, wp[0]))
    return {"gain_margin": gm, "gain_margin_db": 20 * math.log10(gm), "phase_crossover": wp[0]}


def _verify_pm(inputs, outputs):
    num, den = _ptrim(inputs["num"]), _ptrim(inputs["den"])
    w = outputs["gain_crossover"]
    g = _G(num, den, w)
    pm = math.degrees(cmath.phase(g)) % 360.0 - 180.0
    return abs(abs(g) - 1.0) + abs(outputs["phase_margin"] - pm) / 180.0


def _verify_gm(inputs, outputs):
    num, den = _ptrim(inputs["num"]), _ptrim(inputs["den"])
    w = outputs["phase_crossover"]
    g = _G(num, den, w)
    return (abs(g.imag) / abs(g) + (g.real >= 0) + _rel(outputs["gain_margin"], 1.0 / abs(g))
            + abs(outputs["gain_margin_db"] - 20 * math.log10(outputs["gain_margin"])) / max(1.0, abs(outputs["gain_margin_db"])))


def _scan_crossings(f, num, den):
    """Sign changes of f(w) on a dense logarithmic grid (scipy.signal.freqs gives G(jw)), each refined by
    scipy.optimize.brentq."""
    import numpy as np
    from scipy import optimize, signal
    poles = np.abs(np.roots(den)) if len(den) > 1 else np.array([1.0])
    zeros = np.abs(np.roots(num)) if len(num) > 1 else np.array([1.0])
    pts = np.concatenate([poles[poles > 0], zeros[zeros > 0], [1.0]])
    lo, hi = pts.min() * 1e-4, pts.max() * 1e4 * max(1.0, abs(num[0] / den[0])) ** 0.5 * 10
    grid = np.logspace(np.log10(lo), np.log10(hi), 40001)
    _, h = signal.freqs(num, den, worN=grid)
    vals = f(h)
    out = []
    for i in np.nonzero(np.sign(vals[:-1]) * np.sign(vals[1:]) < 0)[0]:
        def g(w):
            _, hw = signal.freqs(num, den, worN=[w])
            return float(f(hw)[0])
        out.append(optimize.brentq(g, grid[i], grid[i + 1], xtol=1e-300, rtol=4 * np.finfo(float).eps, maxiter=500))
    return out, signal


def _reference_pm(num, den):
    """INDEPENDENT reference: |G(jw)| - 1 evaluated by scipy.signal.freqs on a dense grid, the crossing refined by
    scipy.optimize.brentq; the phase from numpy.angle."""
    import numpy as np
    num, den = _ptrim(num), _ptrim(den)
    ws, signal = _scan_crossings(lambda h: np.abs(h) - 1.0, num, den)
    w = ws[0]
    _, h = signal.freqs(num, den, worN=[w])
    return {"phase_margin": float(np.remainder(np.angle(h[0], deg=True), 360.0) - 180.0), "gain_crossover": float(w)}


def _reference_gm(num, den):
    """INDEPENDENT reference: the angle of -G(jw) (scipy.signal.freqs) crossing zero on a dense grid, refined by
    scipy.optimize.brentq; the gain margin 1/|G| there."""
    import numpy as np
    num, den = _ptrim(num), _ptrim(den)
    ws, signal = _scan_crossings(lambda h: np.where(h.real < 0, h.imag, np.nan), num, den)
    ws = [w for w in ws if signal.freqs(num, den, worN=[w])[1][0].real < 0]
    w = ws[0]
    gm = 1.0 / abs(signal.freqs(num, den, worN=[w])[1][0])
    return {"gain_margin": float(gm), "gain_margin_db": float(20 * np.log10(gm)), "phase_crossover": float(w)}


def _parse_margin(word):
    def parse(question):
        if not re.search(rf"\b{word}\s+margin\b", question, re.I):
            return None
        other = "phase" if word == "gain" else "gain"
        if re.search(rf"\b{other}\s+margin\b", question, re.I):
            return None                               # both asked: two capabilities, one question each
        tf = _tf_from_question(question)
        return None if tf is None or "_bad" in tf else tf
    return parse


def _random_margin_loop(rng):
    poles = sorted(round(rng.uniform(0.5, 10), 2) for _ in range(rng.choice([2, 2, 3])))
    r = rng.choice([0, 1, 1])
    K = round(rng.uniform(1, 40), 2) * (poles[0] if r else poles[0] * poles[-1])
    den = [1.0] + [0.0] * r
    for p in poles:
        den = _pmul(den, [1.0, p])
    return {"num": [round(K, 6)], "den": [round(x, 8) for x in den]}


PROCEDURES["phase_margin"] = {
    "name": "Phase margin and gain-crossover frequency", "topic": "control systems / frequency response",
    "purpose": "The phase margin of a unity-feedback loop with open-loop G(s): 180 degrees plus the phase of G at the frequency where |G(jw)| = 1.",
    "inputs": _TF_IN,
    "outputs": [{"symbol": "phase_margin", "kind": "number", "meaning": "phase margin in degrees"},
                {"symbol": "gain_crossover", "kind": "angular_frequency", "meaning": "gain-crossover frequency (|G| = 1)"}],
    "primary": "phase_margin", "conditions": ["a proper G(s)", "exactly one gain crossover"],
    "method": "the gain crossover as the positive real root of |N(jw)|^2 - |D(jw)|^2 (Aberth iteration, Newton-polished); PM = (arg G(jw_c) mod 360) - 180",
    "latex": r"|G(j\omega_c)| = 1,\quad \mathrm{PM} = 180^\circ + \angle G(j\omega_c)",
    "run": phase_margin, "verify": _verify_pm, "parse": _parse_margin("phase"), "reference": _reference_pm,
    "wrong": lambda num, den: (lambda r: {"phase_margin": -r["phase_margin"], "gain_crossover": r["gain_crossover"]})(phase_margin(num, den)),
    "random_inputs": _random_margin_loop,
    "templates": ["Find the phase margin of G(s) = 10/(s(s+1)(s+5)).",
                  "What is the phase margin of G(s) = 4/(s(s+2))?"],
    "examples": [{"inputs": {"num": "4", "den": "1, 2, 0"}, "outputs": {"gain_crossover": math.sqrt(math.sqrt(20) - 2)}},
                 {"inputs": {"num": [10], "den": [1, 6, 5, 0]}, "outputs": {}}],
    "questions": [{"q": "What is the phase margin of G(s) = 4/(s(s+2))?",
                   "outputs": {"gain_crossover": math.sqrt(math.sqrt(20) - 2),
                               "phase_margin": 180 - 90 - math.degrees(math.atan(math.sqrt(math.sqrt(20) - 2) / 2))}},
                  {"q": "Find the phase margin of G(s) = 10/(s(s+1)(s+5)).", "outputs": {}},
                  {"q": "Find the phase margin of G(s) = 0.5/(s+1).", "outcome": "INVALID_INPUT"}]}

PROCEDURES["gain_margin"] = {
    "name": "Gain margin and phase-crossover frequency", "topic": "control systems / frequency response",
    "purpose": "The gain margin of a unity-feedback loop with open-loop G(s): 1/|G(jw)| at the frequency where the phase is -180 degrees.",
    "inputs": _TF_IN,
    "outputs": [{"symbol": "gain_margin", "kind": "number", "meaning": "gain margin as a ratio"},
                {"symbol": "gain_margin_db", "kind": "number", "meaning": "gain margin in decibels"},
                {"symbol": "phase_crossover", "kind": "angular_frequency", "meaning": "phase-crossover frequency (phase -180 degrees)"}],
    "primary": "gain_margin", "conditions": ["a proper G(s)", "exactly one phase crossover with G negative real"],
    "method": "the phase crossover as the positive real root of Im(N(jw) conj D(jw)) with Re G < 0 (Aberth iteration, Newton-polished)",
    "latex": r"\angle G(j\omega_{pc}) = -180^\circ,\quad \mathrm{GM} = \frac{1}{|G(j\omega_{pc})|}",
    "run": gain_margin, "verify": _verify_gm, "parse": _parse_margin("gain"), "reference": _reference_gm,
    "wrong": lambda num, den: (lambda r: {"gain_margin": 1.0 / r["gain_margin"], "gain_margin_db": -r["gain_margin_db"],
                                          "phase_crossover": r["phase_crossover"]})(gain_margin(num, den)),
    "random_inputs": lambda rng: (lambda x: x if len(x["den"]) >= 4 else _random_margin_loop(rng))(_random_margin_loop(rng)),
    "templates": ["Find the gain margin of G(s) = 10/(s(s+1)(s+5)).", "What is the gain margin of G(s) = 6/(s+1)^3?"],
    "examples": [{"inputs": {"num": "10", "den": "1, 6, 5, 0"}, "outputs": {"gain_margin": 3.0, "phase_crossover": math.sqrt(5)}},
                 {"inputs": {"num": [6], "den": [1, 3, 3, 1]}, "outputs": {"gain_margin": 8 / 6, "phase_crossover": math.sqrt(3)}}],
    "questions": [{"q": "Find the gain margin of G(s) = 10/(s(s+1)(s+5)).", "outputs": {"gain_margin": 3.0, "phase_crossover": math.sqrt(5)}},
                  {"q": "What is the gain margin of G(s) = 6/(s+1)^3?", "outputs": {"gain_margin": 8 / 6}},
                  {"q": "Find the gain margin of G(s) = 4/(s(s+2)).", "outcome": "INVALID_INPUT"}]}


def bode_point(num, den, w) -> dict:
    num, den = _ptrim(num), _ptrim(den)
    if all(x == 0 for x in den):
        raise ValueError("the denominator must not be zero")
    if not (w > 0 and math.isfinite(w)):
        raise ValueError("the frequency must be positive")
    g = _G(num, den, w)
    if g == 0:
        raise ValueError("G(jw) = 0 at this frequency: the magnitude in decibels is minus infinity")
    return {"magnitude": abs(g), "magnitude_db": 20 * math.log10(abs(g)), "phase_deg": math.degrees(cmath.phase(g)),
            "real": g.real, "imag": g.imag}


def _verify_bode(inputs, outputs):
    num, den = _ptrim(inputs["num"]), _ptrim(inputs["den"])
    g = _G(num, den, inputs["w"])
    d = math.degrees(cmath.phase(g)) - outputs["phase_deg"]
    d = (d + 180) % 360 - 180
    return (_rel(outputs["magnitude"], abs(g)) + abs(outputs["magnitude_db"] - 20 * math.log10(abs(g))) / max(1.0, abs(outputs["magnitude_db"]))
            + abs(d) / 180 + abs(complex(outputs.get("real", g.real), outputs.get("imag", g.imag)) - g) / abs(g))


def _reference_bode(num, den, w):
    """INDEPENDENT reference: scipy.signal.freqs (the analog frequency response) and numpy.angle."""
    import numpy as np
    from scipy import signal
    _, h = signal.freqs(_ptrim(num), _ptrim(den), worN=[w])
    h = complex(h[0])
    return {"magnitude": abs(h), "magnitude_db": float(20 * np.log10(abs(h))), "phase_deg": float(np.angle(h, deg=True))}


def parse_bode(question):
    if not re.search(r"\b(?:bode|magnitude|frequency response)\b", question, re.I) or re.search(r"\bmargin\b", question, re.I):
        return None
    tf = _tf_from_question(question)
    if tf is None or "_bad" in tf:
        return None
    m = re.search(r"\b(?:at|w\s*=|ω\s*=|omega\s*=)\s*(" + _NUMRE.replace("(?<![\\w.])", "") + r")\s*(rad/s|Hz|kHz)\b", question)
    if m:
        v = float(m.group(1))
        tf["w"] = v if m.group(2) == "rad/s" else 2 * math.pi * v * (1e3 if m.group(2) == "kHz" else 1.0)
    return tf


PROCEDURES["bode_point"] = {
    "name": "Frequency response (Bode magnitude and phase) at one frequency", "topic": "control systems / frequency response",
    "purpose": "|G(jw)| as a ratio and in decibels, and the phase of G(jw), for a transfer function at an angular frequency w.",
    "inputs": [_TF_IN[0], _TF_IN[1],
               {"symbol": "w", "kind": "angular_frequency", "required": True, "meaning": "angular frequency in rad/s"}],
    "outputs": [{"symbol": "magnitude", "kind": "number", "meaning": "|G(jw)|"},
                {"symbol": "magnitude_db", "kind": "number", "meaning": "20 log10 |G(jw)| in decibels"},
                {"symbol": "phase_deg", "kind": "number", "meaning": "phase of G(jw) in degrees, in (-180, 180]"},
                {"symbol": "real", "kind": "number", "meaning": "real part of G(jw)"},
                {"symbol": "imag", "kind": "number", "meaning": "imaginary part of G(jw)"}],
    "primary": "magnitude_db", "conditions": ["no pole of G on the imaginary axis at w", "w > 0"],
    "method": "Horner evaluation of numerator and denominator at s = jw", "latex": r"G(j\omega) = \frac{N(j\omega)}{D(j\omega)},\quad 20\log_{10}|G(j\omega)|",
    "run": bode_point, "verify": _verify_bode, "parse": parse_bode, "reference": _reference_bode,
    "wrong": lambda num, den, w: (lambda g: {"magnitude": abs(g), "magnitude_db": 20 * math.log10(abs(g)) if g else 0.0,
                                             "phase_deg": math.degrees(cmath.phase(g))})(
        _pval(_ptrim(num), complex(w)) / _pval(_ptrim(den), complex(w)) if _pval(_ptrim(den), complex(w)) else 1.0),
    "random_inputs": lambda rng: dict(_random_loop(rng), w=round(10 ** rng.uniform(-1, 2), 3)),
    "templates": ["Find the Bode magnitude of G(s) = 10/(s+1) at 1 rad/s.",
                  "What is the frequency response of G(s) = 100/(s^2 + 10s + 100) at 10 rad/s?"],
    "examples": [{"inputs": {"num": "10", "den": "1, 1", "w": 1}, "outputs": {"magnitude": 10 / math.sqrt(2), "phase_deg": -45.0}},
                 {"inputs": {"num": [100], "den": [1, 10, 100], "w": 10}, "outputs": {"magnitude": 1.0, "phase_deg": -90.0, "magnitude_db": 0.0}}],
    "questions": [{"q": "Find the Bode magnitude of G(s) = 10/(s+1) at 1 rad/s.", "outputs": {"magnitude_db": 20 * math.log10(10 / math.sqrt(2)), "phase_deg": -45.0}},
                  {"q": "What is the frequency response of G(s) = 100/(s^2 + 10s + 100) at 10 rad/s?", "outputs": {"magnitude": 1.0, "phase_deg": -90.0}},
                  {"q": "Find the Bode magnitude of G(s) = 1/(s^2 + 4) at 2 rad/s.", "outcome": "INVALID_INPUT"},
                  {"q": "Find the Bode magnitude of G(s) = 10/(s+1).", "outcome": "NEEDS_INPUT"}]}


# --------------------------------------------------------------------------------------------- steady-state error
_INPUT_ORDER = {"step": 0, "ramp": 1, "parabola": 2}


def steady_state_error(num, den, input) -> dict:                               # noqa: A002 (the input signal)
    num, den = _check_tf(num, den)
    char = _ptrim(_padd(den, num))
    if max(complex(r).real for r in _roots(char)) >= -1e-9 * max(1.0, max(abs(complex(r)) for r in _roots(char))):
        raise ValueError("the closed loop is not stable, so there is no steady-state error")
    k = _INPUT_ORDER[input]
    zd = 0
    while zd < len(den) and den[len(den) - 1 - zd] == 0:
        zd += 1
    zn = 0
    while zn < len(num) and num[len(num) - 1 - zn] == 0:
        zn += 1
    r = zd - zn                                        # the system type (poles at the origin minus zeros there)
    nprime, dprime = num[:len(num) - zn], den[:len(den) - zd]
    if k > r:
        K = 0.0
    elif k == r:
        K = nprime[-1] / dprime[-1]
    else:
        K = math.inf
    if k == 0:
        ess = 0.0 if K == math.inf else 1.0 / (1.0 + K)
    else:
        ess = 0.0 if K == math.inf else ("infinite" if K == 0 else 1.0 / K)
    return {"steady_state_error": ess, "error_constant": "infinite" if K == math.inf else K, "system_type": float(r)}


def _verify_sse(inputs, outputs):
    """lim s->0 of s R(s)/(1 + G(s)) = D(s) / (s^k (D(s) + N(s))), decided by the lowest-order nonzero coefficients
    of that numerator and denominator (exact rational arithmetic, fractions)."""
    from fractions import Fraction
    num = [Fraction(x) for x in _ptrim(inputs["num"])]
    den = [Fraction(x) for x in _ptrim(inputs["den"])]
    k = _INPUT_ORDER[inputs["input"]]
    top = den[::-1]                                     # lowest power first
    n_, d_ = num[::-1], den[::-1]
    bot = [Fraction(0)] * k + [(d_[i] if i < len(d_) else 0) + (n_[i] if i < len(n_) else 0) for i in range(max(len(d_), len(n_)))]
    a = next(i for i, x in enumerate(top) if x != 0)
    b = next(i for i, x in enumerate(bot) if x != 0)
    want = Fraction(0) if a > b else (top[a] / bot[b] if a == b else None)
    ess = outputs["steady_state_error"]
    if want is None:
        return 0.0 if ess == "infinite" else 1.0
    if isinstance(ess, str):
        return 1.0
    return abs(ess - float(want)) / max(1.0, abs(float(want)))


def _reference_sse(num, den, input):                                            # noqa: A002
    """INDEPENDENT reference: sympy.limit of s * R(s) / (1 + G(s)) as s -> 0 with R = 1/s, 1/s^2, 1/s^3."""
    import sympy as sp
    s = sp.Symbol("s")
    N = sum(sp.nsimplify(c, rational=True) * s ** (len(num) - 1 - i) for i, c in enumerate(_ptrim(num)))
    D = sum(sp.nsimplify(c, rational=True) * s ** (len(den) - 1 - i) for i, c in enumerate(_ptrim(den)))
    k = _INPUT_ORDER[input]
    lim = sp.limit(sp.cancel(s * (1 / s ** (k + 1)) / (1 + N / D)), s, 0, "+")
    ess = "infinite" if lim in (sp.oo, -sp.oo, sp.zoo) else float(lim)
    Kc = sp.limit(sp.cancel(s ** k * N / D), s, 0, "+")
    return {"steady_state_error": ess, "error_constant": "infinite" if Kc in (sp.oo, -sp.oo, sp.zoo) else float(Kc)}


def _random_sse(rng):
    r = rng.choice([0, 1, 2])
    x = _random_loop(rng, r)
    k = rng.choice([r, r, r, 0, 1, 2])
    return {**x, "input": ["step", "ramp", "parabola"][k]}


def parse_sse(question):
    if not re.search(r"\bsteady[- ]state error\b", question, re.I):
        return None
    tf = _tf_from_question(question)
    if tf is None or "_bad" in tf:
        return None
    m = re.search(r"\b(step|ramp|parabol(?:a|ic))\b", question, re.I)
    if m:
        tf["input"] = "parabola" if m.group(1).lower().startswith("parabol") else m.group(1).lower()
    return tf


PROCEDURES["steady_state_error"] = {
    "name": "Steady-state error of a unity-feedback system", "topic": "control systems / steady-state accuracy",
    "purpose": "The system type, the error constant (Kp, Kv or Ka) and the steady-state error for a unit step, ramp or parabola input.",
    "inputs": _TF_IN + [{"symbol": "input", "type": "choice", "choices": ["step", "ramp", "parabola"], "required": True,
                         "meaning": "the test input: unit step 1/s, unit ramp 1/s^2 or unit parabola t^2/2 (1/s^3)"}],
    "outputs": [{"symbol": "steady_state_error", "kind": "number", "meaning": "steady-state error (or 'infinite')"},
                {"symbol": "error_constant", "kind": "number", "meaning": "position (Kp), velocity (Kv) or acceleration (Ka) error constant"},
                {"symbol": "system_type", "kind": "number", "meaning": "number of open-loop poles at the origin"}],
    "primary": "steady_state_error", "conditions": ["a stable closed loop", "unity negative feedback", "unit-amplitude test input"],
    "method": "system type from the poles at the origin; error constant lim s^k G(s); e_ss = 1/(1+Kp) for a step, 1/Kv for a ramp, 1/Ka for a parabola",
    "latex": r"e_{ss} = \lim_{s\to 0} \frac{s\,R(s)}{1 + G(s)}",
    "run": steady_state_error, "verify": _verify_sse, "parse": parse_sse, "reference": _reference_sse,
    "wrong": lambda num, den, input: (lambda r: dict(r, steady_state_error=(1.0 / (1.0 + r["error_constant"]))
                                                     if isinstance(r["error_constant"], float) and input != "step" else
                                                     ((1.0 / r["error_constant"]) if isinstance(r["error_constant"], float) and r["error_constant"] else r["steady_state_error"])))(
        steady_state_error(num, den, input)),
    "random_inputs": _random_sse,
    "templates": ["Find the steady-state error of G(s) = 10/((s+1)(s+2)) for a step input.",
                  "What is the steady-state error of G(s) = 20/(s(s+5)) for a ramp input?"],
    "examples": [{"inputs": {"num": "10", "den": "1, 3, 2", "input": "step"}, "outputs": {"steady_state_error": 1 / 6, "error_constant": 5.0}},
                 {"inputs": {"num": [20], "den": [1, 5, 0], "input": "ramp"}, "outputs": {"steady_state_error": 0.25, "error_constant": 4.0}},
                 {"inputs": {"num": [20], "den": [1, 5, 0], "input": "step"}, "outputs": {"steady_state_error": 0.0}}],
    "questions": [{"q": "Find the steady-state error of G(s) = 10/((s+1)(s+2)) for a step input.", "outputs": {"steady_state_error": 1 / 6}},
                  {"q": "What is the steady-state error of G(s) = 20/(s(s+5)) for a ramp input?", "outputs": {"steady_state_error": 0.25}},
                  {"q": "What is the steady-state error of G(s) = 10/((s+1)(s+2)) for a ramp input?", "outputs": {"steady_state_error": "infinite"}},
                  {"q": "Find the steady-state error of G(s) = 1/((s-1)(s+2)) for a step input.", "outcome": "INVALID_INPUT"},
                  {"q": "Find the steady-state error of G(s) = 10/((s+1)(s+2)).", "outcome": "NEEDS_INPUT"}]}


# --------------------------------------------------------------------------------------------- second-order step response
def _bisect(f, a, b):
    fa = f(a)
    for _ in range(300):
        m = 0.5 * (a + b)
        if m in (a, b):
            break
        fm = f(m)
        if (fm > 0) == (fa > 0):
            a, fa = m, fm
        else:
            b = m
    return 0.5 * (a + b)


def second_order_step(zeta, wn) -> dict:
    if not (0 < zeta < 1):
        raise ValueError("the damping ratio must be between 0 and 1 (an underdamped system)")
    if not (wn > 0 and math.isfinite(wn)):
        raise ValueError("the natural frequency must be positive")
    q = math.sqrt(1 - zeta * zeta)
    wd, sg = wn * q, zeta * wn

    def y(t):
        return 1 - math.exp(-sg * t) * (math.cos(wd * t) + zeta / q * math.sin(wd * t))
    tp = math.pi / wd
    mp = math.exp(-zeta * math.pi / q)
    t10 = _bisect(lambda t: y(t) - 0.1, 0.0, tp)
    t90 = _bisect(lambda t: y(t) - 0.9, 0.0, tp)
    k = int(math.floor(math.log(1 / 0.02) * wd / (sg * math.pi)))
    while k > 0 and math.exp(-sg * k * math.pi / wd) <= 0.02:
        k -= 1
    tk = k * math.pi / wd
    tz = (math.pi / 2 + math.asin(zeta) + k * math.pi) / wd
    ts = _bisect(lambda t: abs(y(t) - 1) - 0.02, tk, tz)
    return {"percent_overshoot": 100 * mp, "overshoot": mp, "peak_time": tp, "rise_time": t90 - t10, "t10": t10, "t90": t90,
            "settling_time": ts, "damped_frequency": wd}


def _verify_step2(inputs, outputs):
    z, wn = inputs["zeta"], inputs["wn"]
    q = math.sqrt(1 - z * z)
    wd, sg = wn * q, z * wn
    y = lambda t: 1 - math.exp(-sg * t) * (math.cos(wd * t) + z / q * math.sin(wd * t))        # noqa: E731
    dy = lambda t: wn / q * math.exp(-sg * t) * math.sin(wd * t)                              # noqa: E731
    tp, ts = outputs["peak_time"], outputs["settling_time"]
    res = [abs(wd * tp - math.pi) / math.pi,
           abs(y(tp) - 1 - outputs["overshoot"]), abs(outputs["percent_overshoot"] - 100 * outputs["overshoot"]) / 100,
           abs(y(outputs["t10"]) - 0.1), abs(y(outputs["t90"]) - 0.9),
           abs(outputs["rise_time"] - (outputs["t90"] - outputs["t10"])) * wn,
           abs(abs(y(ts) - 1) - 0.02),
           # nothing later leaves the 2% band: every later extremum is inside it
           max(0.0, math.exp(-sg * (math.floor(ts * wd / math.pi) + 1) * math.pi / wd) - 0.02)]
    return max(res)


def _reference_step2(zeta, wn):
    """INDEPENDENT reference: scipy.integrate.solve_ivp (DOP853, rtol 1e-13) of y'' + 2 zeta wn y' + wn^2 y = wn^2
    from rest, with event location for 10% and 90% of the final value, the first peak (y' = 0) and every crossing
    of the 2% band (the last one is the settling time)."""
    import numpy as np
    from scipy import integrate

    def rhs(t, s):
        return [s[1], wn * wn * (1 - s[0]) - 2 * zeta * wn * s[1]]
    e10 = lambda t, s: s[0] - 0.1                       # noqa: E731
    e90 = lambda t, s: s[0] - 0.9                       # noqa: E731
    epk = lambda t, s: s[1]                             # noqa: E731
    epk.direction = -1
    eup = lambda t, s: s[0] - 1.02                      # noqa: E731
    elo = lambda t, s: s[0] - 0.98                      # noqa: E731
    A = 1 / np.sqrt(1 - zeta * zeta)
    t_end = max(np.log(A / 0.001) / (zeta * wn), 1.5 * np.pi / (wn * np.sqrt(1 - zeta ** 2)))   # past the first peak
    sol = integrate.solve_ivp(rhs, (0, t_end), [0.0, 0.0], method="DOP853", rtol=1e-13, atol=1e-15,
                              events=[e10, e90, epk, eup, elo], max_step=(np.pi / (wn * np.sqrt(1 - zeta ** 2))) / 20)
    t10, t90, tp = sol.t_events[0][0], sol.t_events[1][0], sol.t_events[2][0]
    yp = sol.y_events[2][0][0]
    band = np.concatenate([sol.t_events[3], sol.t_events[4]])
    return {"overshoot": float(yp - 1), "peak_time": float(tp), "t10": float(t10), "t90": float(t90),
            "rise_time": float(t90 - t10), "settling_time": float(band.max())}


def parse_step2(question):
    q = question.lower()
    if not re.search(r"\b(second[- ]order|2nd[- ]order)\b", q) or not re.search(r"\bstep\b", q):
        return None
    out = {}
    z = re.search(r"\b(?:damping ratio|zeta|ζ)\s*(?:of|=|is)?\s*(" + _NUMRE.replace("(?<![\\w.])", "") + r")", q)
    w = re.search(r"\b(?:natural frequency|undamped natural frequency|wn|ωn|omega_n)\s*(?:of|=|is)?\s*(" + _NUMRE.replace("(?<![\\w.])", "") + r")\s*(rad/s)", q)
    if z:
        out["zeta"] = float(z.group(1))
    if w:
        out["wn"] = float(w.group(1))
    return out or None


PROCEDURES["second_order_step"] = {
    "name": "Step-response specifications of a standard second-order system", "topic": "control systems / time response",
    "purpose": "Percent overshoot, peak time, 10-90% rise time and exact 2% settling time of wn^2/(s^2 + 2 zeta wn s + wn^2) for a unit step.",
    "inputs": [{"symbol": "zeta", "kind": "number", "required": True, "meaning": "damping ratio (0 < zeta < 1)"},
               {"symbol": "wn", "kind": "angular_frequency", "required": True, "meaning": "undamped natural frequency"}],
    "outputs": [{"symbol": "percent_overshoot", "kind": "number", "meaning": "maximum overshoot in percent"},
                {"symbol": "overshoot", "kind": "number", "meaning": "maximum overshoot as a fraction of the final value"},
                {"symbol": "peak_time", "kind": "time", "meaning": "time of the first peak"},
                {"symbol": "rise_time", "kind": "time", "meaning": "10% to 90% rise time"},
                {"symbol": "t10", "kind": "time", "meaning": "time to reach 10% of the final value"},
                {"symbol": "t90", "kind": "time", "meaning": "time to reach 90% of the final value"},
                {"symbol": "settling_time", "kind": "time", "meaning": "exact 2% settling time (last time the response leaves the band)"},
                {"symbol": "damped_frequency", "kind": "angular_frequency", "meaning": "damped natural frequency wn sqrt(1 - zeta^2)"}],
    "primary": "settling_time", "conditions": ["0 < zeta < 1 (underdamped)", "unit step from rest", "unity DC gain"],
    "method": "closed-form overshoot and peak time; rise and settling times by bisection on the exact response "
              "(the last 2% crossing lies between the last peak above the band and the following zero of the error)",
    "latex": r"M_p = e^{-\zeta\pi/\sqrt{1-\zeta^2}},\quad t_p = \frac{\pi}{\omega_n\sqrt{1-\zeta^2}}",
    "run": second_order_step, "verify": _verify_step2, "parse": parse_step2, "reference": _reference_step2,
    "wrong": lambda zeta, wn: dict(second_order_step(zeta, wn), settling_time=4 / (zeta * wn), peak_time=math.pi / wn),
    "random_inputs": lambda rng: {"zeta": round(rng.uniform(0.05, 0.95), 3), "wn": round(10 ** rng.uniform(-1, 2), 3)},
    "templates": ["Find the step response specifications of a second-order system with damping ratio 0.5 and natural frequency 4 rad/s."],
    "examples": [{"inputs": {"zeta": 0.5, "wn": 4}, "outputs": {"percent_overshoot": 100 * math.exp(-0.5 * math.pi / math.sqrt(0.75)),
                                                               "peak_time": math.pi / (4 * math.sqrt(0.75))}},
                 {"inputs": {"zeta": 0.7, "wn": 10}, "outputs": {"peak_time": math.pi / (10 * math.sqrt(0.51))}}],
    "questions": [{"q": "Find the step response specifications of a second-order system with damping ratio 0.5 and natural frequency 4 rad/s.",
                   "outputs": {"percent_overshoot": 100 * math.exp(-0.5 * math.pi / math.sqrt(0.75))}},
                  {"q": "A second-order system has zeta = 1.2 and wn = 5 rad/s. Find its step response overshoot.", "outcome": "INVALID_INPUT"},
                  {"q": "A second-order system has damping ratio 0.3. Find its step response settling time.", "outcome": "NEEDS_INPUT"}]}


# ================================================================================================ AC circuits
_UNITS["time"] = _unit_table(["s"], {"seconds": 1.0, "second": 1.0, "milliseconds": 1e-3, "millisecond": 1e-3,
                                     "microseconds": 1e-6, "microsecond": 1e-6})
_UNITS["frequency"] = _unit_table(["Hz"], {"hertz": 1.0, "kilohertz": 1e3, "megahertz": 1e6})
_NUM_NB = _NUMRE.replace("(?<![\\w.])", "")


def _phrase_value(question, phrases, kind):
    """The value with its unit stated right after one of `phrases` ('a series resistor of 1 kohm'), in SI; None if
    absent."""
    ualt = _unit_alt(kind)
    for ph in phrases:
        m = re.search(rf"\b{ph}\s*(?:of|=|is|:)?\s*({_NUM_NB})\s*({ualt})(?![A-Za-zΩ])", question, re.I if kind not in ("capacitance", "inductance", "resistance", "voltage", "current", "power", "frequency", "time") else 0)
        if m:
            return float(m.group(1)) * _UNITS[kind][m.group(2)]
    return None


def _frequency_in(question):
    m = re.search(rf"\b(?:at|f\s*=|frequency\s*(?:of|=|is)?)\s*({_NUM_NB})\s*({_unit_alt('frequency')})(?![A-Za-z])", question)
    return float(m.group(1)) * _UNITS["frequency"][m.group(2)] if m else None


def _rlc_parts(question):
    out = {}
    for name, kind in (("R", "resistance"), ("L", "inductance"), ("C", "capacitance")):
        v = _named_values(question, [name], kind)
        if v:
            out[name] = v[name]
    f = _frequency_in(question)
    if f is not None:
        out["f"] = f
    return out


def _check_positive(**kw):
    for k, v in kw.items():
        if not (math.isfinite(v) and v > 0):
            raise ValueError(f"{k} must be positive")


def _impedance_out(Z, w, L, C):
    # "+ 0.0" turns a negative zero into 0.0: Python 3.14 divides complex numbers differently and 1/(0.02+0j) gives
    # 50-0j there, which would print "-0" for the imaginary part and the phase
    return {"impedance": abs(Z), "phase_deg": math.degrees(cmath.phase(complex(Z.real + 0.0, Z.imag + 0.0))) + 0.0,
            "real": Z.real + 0.0, "imag": Z.imag + 0.0, "XL": w * L, "XC": 1.0 / (w * C)}


def series_rlc_impedance(R, L, C, f) -> dict:
    if not (math.isfinite(R) and R >= 0):
        raise ValueError("R must be zero or positive")
    _check_positive(L=L, C=C, f=f)
    w = 2 * math.pi * f
    X = w * L - 1.0 / (w * C)
    if abs(X) <= 1e-12 * max(w * L, 1.0 / (w * C)):  # at resonance the reactance is exactly 0, not rounding residue
        X = 0.0
    return _impedance_out(complex(R, X), w, L, C)


def parallel_rlc_impedance(R, L, C, f) -> dict:
    _check_positive(R=R, L=L, C=C, f=f)
    w = 2 * math.pi * f
    B = w * C - 1.0 / (w * L)
    if abs(B) <= 1e-12 * max(w * C, 1.0 / (w * L)):  # at resonance the susceptance is exactly 0, not rounding residue
        B = 0.0                                        # (the residue differed between Python 3.13 and 3.14)
    Y = complex(1.0 / R, B)
    return _impedance_out(1.0 / Y, w, L, C)


def _verify_impedance(series):
    def verify(inputs, outputs):
        R, L, C, f = inputs["R"], inputs["L"], inputs["C"], inputs["f"]
        w = 2 * math.pi * f
        Z = complex(outputs["real"], outputs["imag"])
        # the defining law: series V = (R + jwL + 1/(jwC)) I; parallel I = (1/R + 1/(jwL) + jwC) V, at I or V = 1
        res = abs(Z - (R + 1j * w * L + 1 / (1j * w * C))) / abs(Z) if series else abs(Z * (1 / R + 1 / (1j * w * L) + 1j * w * C) - 1)
        return (res + _rel(outputs["impedance"], abs(Z)) + abs(outputs["phase_deg"] - math.degrees(cmath.phase(Z))) / 180
                + _rel(outputs["XL"], w * L) + _rel(outputs["XC"], 1 / (w * C)))
    return verify


def _reference_impedance(series):
    def reference(R, L, C, f):
        """INDEPENDENT reference: the impedance as a rational function of s evaluated on the jw axis by
        scipy.signal.freqs — series Z(s) = (LCs^2 + RCs + 1)/(Cs); parallel Z(s) = RLs/(RLCs^2 + Ls + R)."""
        import numpy as np
        from scipy import signal
        w = 2 * np.pi * f
        if series:
            _, h = signal.freqs([L * C, R * C, 1.0], [C, 0.0], worN=[w])
        else:
            _, h = signal.freqs([R * L, 0.0], [R * L * C, L, R], worN=[w])
        Z = complex(h[0])
        return {"impedance": abs(Z), "phase_deg": float(np.angle(Z, deg=True)), "real": Z.real, "imag": Z.imag}
    return reference


def _wrong_impedance(series):
    def wrong(R, L, C, f):                               # the frequency used as if it were the angular frequency
        w = f
        Z = complex(R, w * L - 1.0 / (w * C)) if series else 1.0 / complex(1.0 / R, w * C - 1.0 / (w * L))
        return _impedance_out(Z, w, L, C)
    return wrong


def _parse_rlc(word):
    def parse(question):
        q = question.lower()
        if not re.search(rf"\b{word}\s+rlc\b", q) or "impedance" not in q or "step" in q:
            return None
        out = _rlc_parts(question)
        return out or None
    return parse


def _random_rlc(rng):
    return {"R": _e12(rng, 0, 3), "L": _e12(rng, -4, -1), "C": _e12(rng, -8, -5), "f": _e12(rng, 1, 4)}


_RLC_IN = [{"symbol": "R", "kind": "resistance", "required": True, "meaning": "resistance"},
           {"symbol": "L", "kind": "inductance", "required": True, "meaning": "inductance"},
           {"symbol": "C", "kind": "capacitance", "required": True, "meaning": "capacitance"},
           {"symbol": "f", "kind": "frequency", "required": True, "meaning": "frequency of the source in Hz"}]
_Z_OUT = [{"symbol": "impedance", "kind": "resistance", "meaning": "magnitude of the impedance |Z|"},
          {"symbol": "phase_deg", "kind": "number", "meaning": "phase angle of Z in degrees (positive: inductive)"},
          {"symbol": "real", "kind": "resistance", "meaning": "real part of Z"},
          {"symbol": "imag", "kind": "resistance", "meaning": "imaginary part of Z (net reactance)"},
          {"symbol": "XL", "kind": "resistance", "meaning": "inductive reactance wL"},
          {"symbol": "XC", "kind": "resistance", "meaning": "capacitive reactance 1/(wC)"}]

PROCEDURES["series_rlc_impedance"] = {
    "name": "Impedance of a series RLC circuit", "topic": "AC circuits / impedance",
    "purpose": "The complex impedance of R, L and C in series at a frequency: magnitude, phase angle and the two reactances.",
    "inputs": _RLC_IN, "outputs": _Z_OUT, "primary": "impedance",
    "conditions": ["R >= 0, L > 0, C > 0, f > 0", "sinusoidal steady state"],
    "method": "Z = R + j(wL - 1/(wC)) with w = 2 pi f", "latex": r"Z = R + j\left(\omega L - \frac{1}{\omega C}\right)",
    "run": series_rlc_impedance, "verify": _verify_impedance(True), "parse": _parse_rlc("series"),
    "reference": _reference_impedance(True), "wrong": _wrong_impedance(True), "random_inputs": _random_rlc,
    "templates": ["Find the impedance of a series RLC circuit with R = 100 ohm, L = 50 mH and C = 10 uF at 1 kHz."],
    "examples": [{"inputs": {"R": 100, "L": 0.05, "C": 1e-5, "f": 1000},
                  "outputs": {"real": 100.0, "imag": 2 * math.pi * 1000 * 0.05 - 1 / (2 * math.pi * 1000 * 1e-5)}},
                 {"inputs": {"R": 3, "L": 4 / (2 * math.pi * 50), "C": 1 / (2 * math.pi * 50 * 8), "f": 50},
                  "outputs": {"impedance": 5.0, "imag": -4.0}}],
    "questions": [{"q": "Find the impedance of a series RLC circuit with R = 100 ohm, L = 50 mH and C = 10 uF at 1 kHz.",
                   "outputs": {"real": 100.0, "imag": 2 * math.pi * 1000 * 0.05 - 1 / (2 * math.pi * 1000 * 1e-5)}},
                  {"q": "What is the impedance of a series RLC circuit with R = 10 ohm, L = 0.1 H and C = 100 uF at 50 Hz?",
                   "outputs": {"real": 10.0, "imag": 2 * math.pi * 50 * 0.1 - 1 / (2 * math.pi * 50 * 1e-4)}},
                  {"q": "Find the impedance of a series RLC circuit with R = 100 ohm, L = 50 mH and C = 10 uF.", "outcome": "NEEDS_INPUT"},
                  {"q": "Find the impedance of a series RLC circuit with R = 100 ohm, L = 0 mH and C = 10 uF at 1 kHz.", "outcome": "INVALID_INPUT"}]}

PROCEDURES["parallel_rlc_impedance"] = {
    "name": "Impedance of a parallel RLC circuit", "topic": "AC circuits / impedance",
    "purpose": "The complex impedance of R, L and C in parallel at a frequency: magnitude, phase angle and the two reactances.",
    "inputs": _RLC_IN, "outputs": _Z_OUT, "primary": "impedance",
    "conditions": ["R, L, C, f > 0", "sinusoidal steady state"],
    "method": "Y = 1/R + j(wC - 1/(wL)), Z = 1/Y with w = 2 pi f",
    "latex": r"\frac{1}{Z} = \frac{1}{R} + j\left(\omega C - \frac{1}{\omega L}\right)",
    "run": parallel_rlc_impedance, "verify": _verify_impedance(False), "parse": _parse_rlc("parallel"),
    "reference": _reference_impedance(False), "wrong": _wrong_impedance(False), "random_inputs": _random_rlc,
    "templates": ["Find the impedance of a parallel RLC circuit with R = 1 kohm, L = 10 mH and C = 1 uF at 1 kHz."],
    "examples": [{"inputs": {"R": 1000, "L": 0.01, "C": 1e-6, "f": 1000},
                  "outputs": {"impedance": abs(1 / complex(1e-3, 2 * math.pi * 1000 * 1e-6 - 1 / (2 * math.pi * 1000 * 0.01)))}},
                 {"inputs": {"R": 50, "L": 1 / (2 * math.pi * 1000), "C": 1e-3 / (2 * math.pi), "f": 1000},
                  "outputs": {"impedance": 50.0, "phase_deg": 0.0}}],
    "questions": [{"q": "Find the impedance of a parallel RLC circuit with R = 1 kohm, L = 10 mH and C = 1 uF at 1 kHz.",
                   "outputs": {"impedance": abs(1 / complex(1e-3, 2 * math.pi * 1000 * 1e-6 - 1 / (2 * math.pi * 1000 * 0.01)))}},
                  {"q": "Find the impedance of a parallel RLC circuit with R = 0 ohm, L = 10 mH and C = 1 uF at 1 kHz.", "outcome": "INVALID_INPUT"}]}


# ------------------------------------------------------------------------------------------------ phasors
_POLAR = rf"({_NUM_NB})\s*(?:∠|/_|<|\s+at\s+an?\s+angle\s+of\s+|\s+at\s+)\s*({_NUM_NB})\s*(?:°|deg(?:rees?)?)?"


def polar_to_rectangular(magnitude, angle_deg) -> dict:
    if not (math.isfinite(magnitude) and math.isfinite(angle_deg)):
        raise ValueError("magnitude and angle must be finite")
    if magnitude < 0:
        raise ValueError("the magnitude must not be negative")
    a = math.radians(angle_deg)
    return {"real": magnitude * math.cos(a), "imag": magnitude * math.sin(a)}


def rectangular_to_polar(real, imag) -> dict:
    if real == 0 and imag == 0:
        raise ValueError("the phasor 0 has no angle")
    return {"magnitude": math.hypot(real, imag), "angle_deg": math.degrees(math.atan2(imag, real))}


def _verify_p2r(inputs, outputs):
    m, a = inputs["magnitude"], inputs["angle_deg"]
    z = complex(outputs["real"], outputs["imag"])
    d = (math.degrees(cmath.phase(z)) - a + 180) % 360 - 180 if abs(z) > 0 else 0.0
    return abs(abs(z) - m) / max(1.0, m) + (abs(d) / 180 if m > 0 else 0.0)


def _verify_r2p(inputs, outputs):
    m, a = outputs["magnitude"], math.radians(outputs["angle_deg"])
    z = complex(inputs["real"], inputs["imag"])
    return abs(complex(m * math.cos(a), m * math.sin(a)) - z) / max(1.0, abs(z)) + (not -180 < outputs["angle_deg"] <= 180)


def parse_p2r(question):
    q = question.lower()
    if not re.search(r"\brectangular\b|\bcartesian\b", q) or "polar form" in q:
        return None
    m = re.search(_POLAR, question)
    return {"magnitude": float(m.group(1)), "angle_deg": float(m.group(2))} if m else None


_RECT = rf"({_NUM_NB})\s*([+-])\s*(?:j\s*({_NUM_NB})|({_NUM_NB})?\s*\*?\s*[ji])(?![\w])"


def parse_r2p(question):
    q = question.lower()
    if not re.search(r"\bpolar\b", q) or re.search(r"\brectangular form\b", q):
        return None
    m = re.search(_RECT, question)
    if not m:
        return None
    im = float(m.group(3) or m.group(4) or 1.0) * (1 if m.group(2) == "+" else -1)
    return {"real": float(m.group(1)), "imag": im}


PROCEDURES["polar_to_rectangular"] = {
    "name": "Phasor from polar to rectangular form", "topic": "AC circuits / phasors",
    "purpose": "The real and imaginary parts of a phasor given as a magnitude and an angle in degrees.",
    "inputs": [{"symbol": "magnitude", "kind": "number", "required": True, "meaning": "magnitude (non-negative)"},
               {"symbol": "angle_deg", "kind": "number", "required": True, "meaning": "angle in degrees"}],
    "outputs": [{"symbol": "real", "kind": "number", "meaning": "real part"}, {"symbol": "imag", "kind": "number", "meaning": "imaginary part"}],
    "primary": "real", "conditions": ["magnitude >= 0"], "method": "x = M cos(theta), y = M sin(theta)",
    "latex": r"M\angle\theta = M\cos\theta + jM\sin\theta",
    "run": polar_to_rectangular, "verify": _verify_p2r, "parse": parse_p2r,
    "reference": lambda magnitude, angle_deg: (lambda z: {"real": float(z.real), "imag": float(z.imag)})(
        __import__("numpy").exp(1j * __import__("numpy").deg2rad(angle_deg)) * magnitude),
    "wrong": lambda magnitude, angle_deg: {"real": magnitude * math.cos(angle_deg), "imag": magnitude * math.sin(angle_deg)},
    "random_inputs": lambda rng: {"magnitude": round(rng.uniform(0.1, 500), 3), "angle_deg": round(rng.uniform(-180, 180), 2)},
    "templates": ["Convert 10∠30° to rectangular form.", "Convert the phasor 5 at 53.13 degrees to rectangular form."],
    "examples": [{"inputs": {"magnitude": 10, "angle_deg": 30}, "outputs": {"real": 5 * math.sqrt(3), "imag": 5.0}},
                 {"inputs": {"magnitude": 2, "angle_deg": -90}, "outputs": {"real": 0.0, "imag": -2.0}}],
    "questions": [{"q": "Convert 10∠30° to rectangular form.", "outputs": {"real": 5 * math.sqrt(3), "imag": 5.0}},
                  {"q": "Convert the phasor 5 at 53.13 degrees to rectangular form.", "outputs": {"real": 5 * math.cos(math.radians(53.13))}},
                  {"q": "Convert -3∠45° to rectangular form.", "outcome": "INVALID_INPUT"}]}

PROCEDURES["rectangular_to_polar"] = {
    "name": "Phasor from rectangular to polar form", "topic": "AC circuits / phasors",
    "purpose": "The magnitude and angle (degrees, in (-180, 180]) of a phasor given as real + j imaginary.",
    "inputs": [{"symbol": "real", "kind": "number", "required": True, "meaning": "real part"},
               {"symbol": "imag", "kind": "number", "required": True, "meaning": "imaginary part"}],
    "outputs": [{"symbol": "magnitude", "kind": "number", "meaning": "magnitude"}, {"symbol": "angle_deg", "kind": "number", "meaning": "angle in degrees"}],
    "primary": "magnitude", "conditions": ["not both parts zero"], "method": "M = hypot(x, y), theta = atan2(y, x) (the quadrant kept)",
    "latex": r"x + jy = \sqrt{x^2+y^2}\,\angle\,\operatorname{atan2}(y, x)",
    "run": rectangular_to_polar, "verify": _verify_r2p, "parse": parse_r2p,
    "reference": lambda real, imag: {"magnitude": float(abs(complex(real, imag))), "angle_deg": float(__import__("numpy").angle(complex(real, imag), deg=True))},
    "wrong": lambda real, imag: {"magnitude": math.hypot(real, imag), "angle_deg": math.atan2(imag, real)},
    "random_inputs": lambda rng: {"real": round(rng.uniform(-100, 100), 2), "imag": round(rng.uniform(-100, 100), 2)},
    "templates": ["Convert 3 + 4j to polar form.", "Convert -2 - j2 to polar form."],
    "examples": [{"inputs": {"real": 3, "imag": 4}, "outputs": {"magnitude": 5.0, "angle_deg": math.degrees(math.atan2(4, 3))}},
                 {"inputs": {"real": -2, "imag": -2}, "outputs": {"magnitude": math.sqrt(8), "angle_deg": -135.0}}],
    "questions": [{"q": "Convert 3 + 4j to polar form.", "outputs": {"magnitude": 5.0}},
                  {"q": "Convert -2 - j2 to polar form.", "outputs": {"magnitude": math.sqrt(8), "angle_deg": -135.0}},
                  {"q": "Convert 0 + 0j to polar form.", "outcome": "INVALID_INPUT"}]}


def phasor_sum(amplitudes, phases_deg) -> dict:
    if len(amplitudes) != len(phases_deg):
        raise ValueError("give one phase for every amplitude")
    if len(amplitudes) < 2:
        raise ValueError("give at least two phasors")
    if any(a < 0 for a in amplitudes):
        raise ValueError("amplitudes must not be negative")
    z = sum(a * cmath.exp(1j * math.radians(p)) for a, p in zip(amplitudes, phases_deg))
    if abs(z) <= 1e-12 * max(amplitudes):
        return {"amplitude": 0.0, "phase_deg": 0.0, "real": 0.0, "imag": 0.0}
    return {"amplitude": abs(z), "phase_deg": math.degrees(cmath.phase(z)), "real": z.real, "imag": z.imag}


def _verify_phasor_sum(inputs, outputs):
    """The sum of the time functions A_k cos(wt + phi_k) must equal A cos(wt + phi) at 8 instants of the cycle."""
    A, p = outputs["amplitude"], math.radians(outputs["phase_deg"])
    scale = sum(inputs["amplitudes"]) or 1.0
    worst = 0.0
    for k in range(8):
        th = 2 * math.pi * k / 8 + 0.1
        lhs = math.fsum(a * math.cos(th + math.radians(q)) for a, q in zip(inputs["amplitudes"], inputs["phases_deg"]))
        worst = max(worst, abs(lhs - A * math.cos(th + p)) / scale)
    return worst


def _reference_phasor_sum(amplitudes, phases_deg):
    """INDEPENDENT reference: the sum of the sinusoids sampled at 64 points of one period and its fundamental taken
    from numpy.fft (amplitude 2|X1|/N, phase angle(X1))."""
    import numpy as np
    n = 64
    t = np.arange(n) / n
    x = sum(a * np.cos(2 * np.pi * t + np.deg2rad(p)) for a, p in zip(amplitudes, phases_deg))
    X1 = np.fft.fft(x)[1]
    return {"amplitude": float(2 * abs(X1) / n), "phase_deg": float(np.angle(X1, deg=True))}


def parse_phasor_sum(question):
    q = question.lower()
    if not re.search(r"\b(add|sum)\b", q) or not re.search(r"\bphasors?\b", q):
        return None
    found = re.findall(_POLAR, question)
    if len(found) < 2:
        return None
    return {"amplitudes": [float(a) for a, _ in found], "phases_deg": [float(b) for _, b in found]}


PROCEDURES["phasor_sum"] = {
    "name": "Sum of sinusoids by phasor addition", "topic": "AC circuits / phasors",
    "purpose": "The amplitude and phase of the sum of sinusoids of one frequency, A_k cos(wt + phi_k), by adding their phasors.",
    "inputs": [{"symbol": "amplitudes", "type": "vector", "min_length": 2, "max_length": 50, "required": True, "meaning": "amplitudes (non-negative)"},
               {"symbol": "phases_deg", "type": "vector", "min_length": 2, "max_length": 50, "required": True, "meaning": "phase angles in degrees"}],
    "outputs": [{"symbol": "amplitude", "kind": "number", "meaning": "amplitude of the sum"},
                {"symbol": "phase_deg", "kind": "number", "meaning": "phase of the sum in degrees"},
                {"symbol": "real", "kind": "number", "meaning": "real part of the resultant phasor"},
                {"symbol": "imag", "kind": "number", "meaning": "imaginary part of the resultant phasor"}],
    "primary": "amplitude", "conditions": ["one phase per amplitude", "one common frequency"],
    "method": "complex addition of the phasors A_k e^{j phi_k}", "latex": r"\sum_k A_k\angle\phi_k = \sum_k A_k e^{j\phi_k}",
    "run": phasor_sum, "verify": _verify_phasor_sum, "parse": parse_phasor_sum, "reference": _reference_phasor_sum,
    "wrong": lambda amplitudes, phases_deg: {"amplitude": float(sum(amplitudes)), "phase_deg": sum(phases_deg) / len(phases_deg)},
    "random_inputs": lambda rng: (lambda n: {"amplitudes": [round(rng.uniform(0.5, 100), 2) for _ in range(n)],
                                             "phases_deg": [round(rng.uniform(-180, 180), 1) for _ in range(n)]})(rng.randint(2, 5)),
    "templates": ["Add the phasors 10∠30° and 5∠-45°.", "Find the sum of the phasors 3∠0°, 4∠90° and 2∠180°."],
    "examples": [{"inputs": {"amplitudes": "3, 4", "phases_deg": "0, 90"}, "outputs": {"amplitude": 5.0, "phase_deg": math.degrees(math.atan2(4, 3))}},
                 {"inputs": {"amplitudes": [1, 1, 1], "phases_deg": [0, 120, -120]}, "outputs": {"amplitude": 0.0}}],
    "questions": [{"q": "Find the sum of the phasors 3∠0°, 4∠90° and 2∠180°.", "outputs": {"amplitude": math.hypot(1, 4)}},
                  {"q": "Add the phasors 10∠30° and 5∠-45°.",
                   "outputs": {"amplitude": abs(10 * cmath.exp(1j * math.radians(30)) + 5 * cmath.exp(-1j * math.radians(45)))}},
                  {"q": "Add the phasors -10∠30° and 5∠-45°.", "outcome": "INVALID_INPUT"}]}


# ------------------------------------------------------------------------------------------------ AC power
def complex_power(V, V_phase, I, I_phase) -> dict:
    if not (V > 0 and I > 0):
        raise ValueError("the rms voltage and current must be positive")
    S = cmath.rect(V, math.radians(V_phase)) * cmath.rect(I, math.radians(I_phase)).conjugate()
    pf = S.real / abs(S)
    nature = "unity" if abs(S.imag) <= 1e-12 * abs(S) else ("lagging" if S.imag > 0 else "leading")
    return {"P": S.real, "Q": S.imag, "S": abs(S), "power_factor": pf, "nature": nature}


def _verify_cpower(inputs, outputs):
    V, I = inputs["V"], inputs["I"]
    th = math.radians(inputs["V_phase"] - inputs["I_phase"])
    S = V * I
    return (abs(outputs["P"] - S * math.cos(th)) / S + abs(outputs["Q"] - S * math.sin(th)) / S + _rel(outputs["S"], S)
            + abs(outputs["power_factor"] - outputs["P"] / outputs["S"]) + abs(outputs["P"] ** 2 + outputs["Q"] ** 2 - outputs["S"] ** 2) / S ** 2)


def _reference_cpower(V, V_phase, I, I_phase):
    """INDEPENDENT reference: the instantaneous power v(t) i(t) of the two sinusoids averaged over one period
    (512 samples, numpy) gives P; the average of v(t) i(t + T/4) gives Q; S = Vrms Irms."""
    import numpy as np
    n = 512
    wt = 2 * np.pi * np.arange(n) / n
    v = np.sqrt(2) * V * np.cos(wt + np.deg2rad(V_phase))
    i = np.sqrt(2) * I * np.cos(wt + np.deg2rad(I_phase))
    i_q = np.sqrt(2) * I * np.cos(wt + np.pi / 2 + np.deg2rad(I_phase))
    P, Q = float(np.mean(v * i)), float(np.mean(v * i_q))
    return {"P": P, "Q": Q, "S": float(V * I), "power_factor": P / float(V * I)}


def parse_cpower(question):
    q = question.lower()
    if not re.search(r"\b(complex|apparent|reactive|real|active) power\b", q) or "∠" not in question and " at " not in q:
        return None
    if "three-phase" in q or "three phase" in q or "correct" in q:
        return None
    mv = re.search(rf"\bV\s*=\s*{_POLAR}\s*V\b", question)
    mi = re.search(rf"\bI\s*=\s*{_POLAR}\s*A\b", question)
    out = {}
    if mv:
        out.update(V=float(mv.group(1)), V_phase=float(mv.group(2)))
    if mi:
        out.update(I=float(mi.group(1)), I_phase=float(mi.group(2)))
    return out or None


PROCEDURES["complex_power"] = {
    "name": "Complex, real, reactive and apparent power from voltage and current phasors", "topic": "AC circuits / AC power",
    "purpose": "S = V I* for rms phasors: real power P, reactive power Q, apparent power |S|, the power factor and whether it lags or leads.",
    "inputs": [{"symbol": "V", "kind": "voltage", "required": True, "meaning": "rms voltage magnitude"},
               {"symbol": "V_phase", "kind": "number", "required": True, "meaning": "voltage phase angle in degrees"},
               {"symbol": "I", "kind": "current", "required": True, "meaning": "rms current magnitude"},
               {"symbol": "I_phase", "kind": "number", "required": True, "meaning": "current phase angle in degrees"}],
    "outputs": [{"symbol": "P", "kind": "power", "meaning": "real (active) power in W"},
                {"symbol": "Q", "kind": "power", "meaning": "reactive power in var (positive: inductive)"},
                {"symbol": "S", "kind": "power", "meaning": "apparent power in VA"},
                {"symbol": "power_factor", "kind": "number", "meaning": "power factor P/|S|"},
                {"symbol": "nature", "kind": "text", "meaning": "lagging (inductive), leading (capacitive) or unity"}],
    "primary": "P", "conditions": ["rms phasors", "sinusoidal steady state"],
    "method": "S = V I* (complex conjugate of the current)", "latex": r"S = V I^* = P + jQ,\quad \mathrm{pf} = \frac{P}{|S|}",
    "run": complex_power, "verify": _verify_cpower, "parse": parse_cpower, "reference": _reference_cpower,
    "wrong": lambda V, V_phase, I, I_phase: (lambda S: {"P": S.real, "Q": S.imag, "S": abs(S), "power_factor": S.real / abs(S)})(
        cmath.rect(V, math.radians(V_phase)) * cmath.rect(I, math.radians(I_phase))),
    "random_inputs": lambda rng: {"V": round(rng.uniform(100, 480), 1), "V_phase": round(rng.uniform(-60, 60), 1),
                                  "I": round(rng.uniform(0.5, 50), 2), "I_phase": round(rng.uniform(-90, 90), 1)},
    "templates": ["Find the complex power for V = 230∠0° V and I = 10∠-30° A."],
    "examples": [{"inputs": {"V": 230, "V_phase": 0, "I": 10, "I_phase": -30},
                  "outputs": {"P": 2300 * math.cos(math.radians(30)), "Q": 1150.0, "S": 2300.0, "nature": "lagging"}},
                 {"inputs": {"V": 120, "V_phase": 10, "I": 5, "I_phase": 40}, "outputs": {"Q": -300.0, "nature": "leading"}}],
    "questions": [{"q": "Find the complex power for V = 230∠0° V and I = 10∠-30° A.", "outputs": {"S": 2300.0, "Q": 1150.0}},
                  {"q": "Find the real power for V = 120∠10° V and I = 5∠40° A.", "outputs": {"P": 600 * math.cos(math.radians(30))}},
                  {"q": "Find the complex power for V = 230∠0° V.", "outcome": "NEEDS_INPUT"}]}


def power_factor_correction(P, V, f, pf_initial, pf_target) -> dict:
    _check_positive(P=P, V=V, f=f)
    if not (0 < pf_initial <= 1 and 0 < pf_target <= 1):
        raise ValueError("power factors must be between 0 and 1")
    if pf_target <= pf_initial:
        raise ValueError("the target power factor must be higher than the present one")
    q1 = P * math.tan(math.acos(pf_initial))
    q2 = P * math.tan(math.acos(pf_target))
    qc = q1 - q2
    C = qc / (2 * math.pi * f * V * V)
    return {"C": C, "C_uF": C * 1e6, "Qc": qc, "Q_initial": q1, "Q_final": q2}


def _verify_pfc(inputs, outputs):
    """With the capacitor added, the source's power factor must be the target (the load's current plus the
    capacitor's current j w C V, as phasors)."""
    P, V, f = inputs["P"], inputs["V"], inputs["f"]
    q1 = P * math.tan(math.acos(inputs["pf_initial"]))
    I = complex(P, -q1) / V                        # conj(S/V) with V at 0 degrees
    It = I + 1j * 2 * math.pi * f * outputs["C"] * V
    pf = math.cos(cmath.phase(It))
    return (abs(pf - inputs["pf_target"]) + _rel(outputs["C_uF"], outputs["C"] * 1e6)
            + _rel(outputs["Qc"], 2 * math.pi * f * outputs["C"] * V * V) + _rel(outputs["Q_initial"], q1))


def _reference_pfc(P, V, f, pf_initial, pf_target):
    """INDEPENDENT reference: the capacitance found by scipy.optimize.brentq so that the phasor sum of the load
    current and the capacitor current (numpy complex) has the target power factor."""
    import numpy as np
    from scipy import optimize
    phi1 = np.arccos(pf_initial)
    I_load = P / (V * pf_initial) * np.exp(-1j * phi1)
    target = np.arccos(pf_target)
    g = lambda C: -np.angle(I_load + 1j * 2 * np.pi * f * C * V) - target       # noqa: E731
    hi = abs(I_load.imag) / (2 * np.pi * f * V)
    C = optimize.brentq(g, 0.0, hi, xtol=1e-300, rtol=4 * np.finfo(float).eps, maxiter=500)
    return {"C": float(C), "C_uF": float(C * 1e6), "Qc": float(2 * np.pi * f * C * V * V)}


def parse_pfc(question):
    q = question.lower()
    if "power factor" not in q or not re.search(r"\b(correct|improve|raise)", q):
        return None
    out = {}
    p = re.search(rf"({_NUM_NB})\s*(kW|MW|W)\b", question)
    if p:
        out["P"] = float(p.group(1)) * {"W": 1, "kW": 1e3, "MW": 1e6}[p.group(2)]
    v = re.search(rf"({_NUM_NB})\s*(kV|V)\b", question)
    if v:
        out["V"] = float(v.group(1)) * (1e3 if v.group(2) == "kV" else 1)
    fr = re.search(rf"({_NUM_NB})\s*Hz\b", question)
    if fr:
        out["f"] = float(fr.group(1))
    a = re.search(rf"\bpower factor (?:of|is)\s*({_NUM_NB})", q)
    b = re.search(rf"\bto\s*(?:a power factor of\s*)?({_NUM_NB})(?!\s*(?:k?v|hz|w))", q)
    if a:
        out["pf_initial"] = float(a.group(1))
    if b:
        out["pf_target"] = float(b.group(1))
    return out or None


PROCEDURES["power_factor_correction"] = {
    "name": "Power-factor correction capacitor", "topic": "AC circuits / AC power",
    "purpose": "The shunt capacitance (and its reactive power) that raises a lagging load's power factor to a target value.",
    "inputs": [{"symbol": "P", "kind": "power", "required": True, "meaning": "real power of the load"},
               {"symbol": "V", "kind": "voltage", "required": True, "meaning": "rms supply voltage across the capacitor"},
               {"symbol": "f", "kind": "frequency", "required": True, "meaning": "supply frequency"},
               {"symbol": "pf_initial", "kind": "number", "required": True, "meaning": "present (lagging) power factor"},
               {"symbol": "pf_target", "kind": "number", "required": True, "meaning": "target (lagging) power factor"}],
    "outputs": [{"symbol": "C", "kind": "capacitance", "meaning": "correction capacitance"},
                {"symbol": "C_uF", "kind": "number", "meaning": "correction capacitance in microfarads"},
                {"symbol": "Qc", "kind": "power", "meaning": "reactive power the capacitor supplies, in var"},
                {"symbol": "Q_initial", "kind": "power", "meaning": "reactive power before correction, in var"},
                {"symbol": "Q_final", "kind": "power", "meaning": "reactive power after correction, in var"}],
    "primary": "C_uF", "conditions": ["single-phase, sinusoidal", "both power factors lagging", "target > present"],
    "method": "Qc = P (tan acos pf1 - tan acos pf2), C = Qc / (2 pi f V^2)",
    "latex": r"C = \frac{P(\tan\phi_1 - \tan\phi_2)}{2\pi f V^2}",
    "run": power_factor_correction, "verify": _verify_pfc, "parse": parse_pfc, "reference": _reference_pfc,
    "wrong": lambda P, V, f, pf_initial, pf_target: (lambda r: dict(r, C=r["Qc"] / (f * V * V), C_uF=r["Qc"] / (f * V * V) * 1e6))(
        power_factor_correction(P, V, f, pf_initial, pf_target)),
    "random_inputs": lambda rng: (lambda a: {"P": round(rng.uniform(1e3, 1e5), -1), "V": rng.choice([120.0, 230.0, 240.0, 400.0, 480.0]),
                                             "f": rng.choice([50.0, 60.0]), "pf_initial": a,
                                             "pf_target": round(rng.uniform(a + 0.02, 1.0), 3)})(round(rng.uniform(0.5, 0.9), 3)),
    "templates": ["A load draws 10 kW at a power factor of 0.7 from a 230 V, 50 Hz supply. What capacitance corrects the power factor to 0.95?"],
    "examples": [{"inputs": {"P": 10000, "V": 230, "f": 50, "pf_initial": 0.7, "pf_target": 0.95},
                  "outputs": {"Qc": 10000 * (math.tan(math.acos(0.7)) - math.tan(math.acos(0.95)))}},
                 {"inputs": {"P": 1000, "V": 120, "f": 60, "pf_initial": 0.6, "pf_target": 1.0},
                  "outputs": {"Qc": 1000 * 4 / 3, "C_uF": 1e6 * (4000 / 3) / (2 * math.pi * 60 * 14400)}}],
    "questions": [{"q": "A load draws 10 kW at a power factor of 0.7 from a 230 V, 50 Hz supply. What capacitance corrects the power factor to 0.95?",
                   "outputs": {"Qc": 10000 * (math.tan(math.acos(0.7)) - math.tan(math.acos(0.95)))}},
                  {"q": "A motor uses 1 kW at a power factor of 0.6 on a 120 V, 60 Hz line. Find the capacitor that improves the power factor to 1.",
                   "outputs": {"Qc": 4000 / 3}},
                  {"q": "A load draws 10 kW at a power factor of 0.9 from a 230 V, 50 Hz supply. What capacitance corrects the power factor to 0.8?",
                   "outcome": "INVALID_INPUT"}]}


def three_phase_load(VL, R, X, connection) -> dict:
    if not (VL > 0 and math.isfinite(VL)):
        raise ValueError("the line voltage must be positive")
    if R < 0 or (R == 0 and X == 0):
        raise ValueError("the phase impedance must have R >= 0 and not be zero")
    Z = complex(R, X)
    if connection == "wye":
        vph = VL / math.sqrt(3)
        iph = vph / abs(Z)
        il = iph
    else:
        vph = VL
        iph = vph / abs(Z)
        il = math.sqrt(3) * iph
    P, Q = 3 * iph * iph * R, 3 * iph * iph * X
    return {"phase_voltage": vph, "phase_current": iph, "line_current": il, "P": P, "Q": Q, "S": 3 * vph * iph,
            "power_factor": R / abs(Z)}


def _verify_3ph(inputs, outputs):
    VL, R, X = inputs["VL"], inputs["R"], inputs["X"]
    Z = abs(complex(R, X))
    S = math.sqrt(3) * VL * outputs["line_current"]          # apparent power from line quantities
    want_vph = VL / math.sqrt(3) if inputs["connection"] == "wye" else VL
    return (_rel(outputs["phase_voltage"], want_vph) + _rel(outputs["phase_current"] * Z, outputs["phase_voltage"])
            + _rel(outputs["S"], S) + abs(outputs["P"] - S * R / Z) / S + abs(outputs["Q"] - S * X / Z) / S
            + abs(outputs["power_factor"] - R / Z))


def _reference_3ph(VL, R, X, connection):
    """INDEPENDENT reference: the three-phase circuit solved by nodal analysis (numpy, complex): three sources
    120 degrees apart of line voltage VL; for wye the load's neutral is an unknown node, for delta each branch sits
    between two lines; line currents and powers read from the solution."""
    import numpy as np
    vp = VL / np.sqrt(3)
    E = np.array([vp, vp * np.exp(-2j * np.pi / 3), vp * np.exp(2j * np.pi / 3)])
    Z = complex(R, X)
    if connection == "wye":
        vn = np.linalg.solve(np.array([[3 / Z]]), np.array([np.sum(E / Z)]))[0]   # KCL at the floating neutral
        Iline = (E - vn) / Z
        Vph = E - vn
        Iph = Iline
    else:
        Vab, Vbc, Vca = E[0] - E[1], E[1] - E[2], E[2] - E[0]
        Iab, Ibc, Ica = Vab / Z, Vbc / Z, Vca / Z
        Iline = np.array([Iab - Ica, Ibc - Iab, Ica - Ibc])
        Vph = np.array([Vab, Vbc, Vca])
        Iph = np.array([Iab, Ibc, Ica])
    S = np.sum(Vph * np.conj(Iph))
    return {"phase_voltage": float(abs(Vph[0])), "phase_current": float(abs(Iph[0])), "line_current": float(abs(Iline[0])),
            "P": float(S.real), "Q": float(S.imag), "S": float(abs(S))}


def parse_3ph(question):
    q = question.lower()
    if not re.search(r"\bthree[- ]phase\b", q):
        return None
    conn = "delta" if "delta" in q else ("wye" if re.search(r"\b(wye|star|y-connected)\b", q) else None)
    out = {}
    if conn:
        out["connection"] = conn
    v = re.search(rf"({_NUM_NB})\s*(kV|V)\b", question)
    if v:
        out["VL"] = float(v.group(1)) * (1e3 if v.group(2) == "kV" else 1)
    out.update(_named_values(question, ["R", "X"], "resistance"))
    return out if len(out) > 1 else None


PROCEDURES["three_phase_load"] = {
    "name": "Balanced three-phase load (wye or delta)", "topic": "AC circuits / three-phase power",
    "purpose": "Phase voltage, phase and line currents, real, reactive and apparent power and the power factor of a balanced load with impedance R + jX per phase.",
    "inputs": [{"symbol": "VL", "kind": "voltage", "required": True, "meaning": "rms line-to-line voltage"},
               {"symbol": "R", "kind": "resistance", "required": True, "meaning": "resistance per phase"},
               {"symbol": "X", "kind": "resistance", "required": True, "meaning": "reactance per phase (positive inductive, negative capacitive)"},
               {"symbol": "connection", "type": "choice", "choices": ["wye", "delta"], "required": True, "meaning": "load connection"}],
    "outputs": [{"symbol": "phase_voltage", "kind": "voltage", "meaning": "voltage across each phase of the load"},
                {"symbol": "phase_current", "kind": "current", "meaning": "current in each phase of the load"},
                {"symbol": "line_current", "kind": "current", "meaning": "current in each line"},
                {"symbol": "P", "kind": "power", "meaning": "total real power in W"},
                {"symbol": "Q", "kind": "power", "meaning": "total reactive power in var"},
                {"symbol": "S", "kind": "power", "meaning": "total apparent power in VA"},
                {"symbol": "power_factor", "kind": "number", "meaning": "power factor R/|Z|"}],
    "primary": "line_current", "conditions": ["balanced load and supply", "R >= 0"],
    "method": "wye: V_ph = V_L/sqrt(3), I_L = I_ph; delta: V_ph = V_L, I_L = sqrt(3) I_ph; P = 3 I_ph^2 R",
    "latex": r"P = \sqrt{3}\,V_L I_L \cos\phi",
    "run": three_phase_load, "verify": _verify_3ph, "parse": parse_3ph, "reference": _reference_3ph,
    "wrong": lambda VL, R, X, connection: dict(three_phase_load(VL, R, X, "delta" if connection == "wye" else "wye")),
    "random_inputs": lambda rng: {"VL": rng.choice([208.0, 400.0, 415.0, 480.0, 11000.0]), "R": round(rng.uniform(1, 50), 2),
                                  "X": round(rng.uniform(-30, 40), 2), "connection": rng.choice(["wye", "delta"])},
    "templates": ["A balanced wye-connected load has R = 8 ohm and X = 6 ohm per phase on a 400 V three-phase supply. Find the line current."],
    "examples": [{"inputs": {"VL": 400, "R": 8, "X": 6, "connection": "wye"},
                  "outputs": {"line_current": 400 / math.sqrt(3) / 10, "power_factor": 0.8}},
                 {"inputs": {"VL": 400, "R": 8, "X": 6, "connection": "delta"}, "outputs": {"line_current": math.sqrt(3) * 40, "P": 3 * 1600 * 8}}],
    "questions": [{"q": "A balanced wye-connected load has R = 8 ohm and X = 6 ohm per phase on a 400 V three-phase supply. Find the line current.",
                   "outputs": {"line_current": 400 / math.sqrt(3) / 10}},
                  {"q": "A balanced delta-connected three-phase load with R = 8 ohm and X = 6 ohm per phase is fed at 400 V. Find the total power.",
                   "outputs": {"P": 38400.0}},
                  {"q": "A balanced three-phase load with R = 8 ohm and X = 6 ohm per phase is fed at 400 V. Find the line current.", "outcome": "NEEDS_INPUT"}]}


# ================================================================================ operational amplifiers (ideal)
def _runs(question, kind):
    """Every list of values with units of `kind` written in the question: [(start, end, [SI values])]."""
    ualt = _unit_alt(kind)
    toks = []
    for m in re.finditer(_NUMRE, question):
        um = re.match(rf"\s*({ualt})(?![A-Za-zΩ])", question[m.end():])
        toks.append({"s": m.start(), "e": m.end() + (um.end() if um else 0), "v": float(m.group(0)), "u": um.group(1) if um else None})
    runs, cur = [], []
    for t in toks:
        if cur and not re.fullmatch(r"\s*(?:,|and|,\s*and)?\s*", question[cur[-1]["e"]:t["s"]]):
            runs.append(cur)
            cur = []
        cur.append(t)
    if cur:
        runs.append(cur)
    out = []
    for r in runs:
        if r[-1]["u"] is None:
            continue
        unit, vals = None, []
        for t in reversed(r):
            unit = t["u"] or unit
            vals.append(t["v"] * _UNITS[kind][unit])
        out.append((r[0]["s"], r[-1]["e"], vals[::-1]))
    return out


def summing_amplifier(V, R, Rf) -> dict:
    if len(V) != len(R):
        raise ValueError("give one input resistor for every input voltage")
    if any(r <= 0 for r in R) or not Rf > 0:
        raise ValueError("resistances must be positive")
    terms = [Rf / r * v for v, r in zip(V, R)]
    return {"Vout": -math.fsum(terms), "gains": [-Rf / r for r in R]}


def _ideal_opamp_mna(resistors, sources, inverting, noninverting, output):
    """numpy modified nodal analysis with the ideal op-amp stamp (v+ = v-, the output current free)."""
    import numpy as np
    nodes = sorted(({n for a, b, _ in resistors for n in (a, b)} | {n for n, _ in sources} | {inverting, noninverting, output}) - {"0"})
    idx = {n: i for i, n in enumerate(nodes)}
    m = len(nodes) + len(sources) + 1
    A = np.zeros((m, m))
    z = np.zeros(m)
    for a, b, r in resistors:
        g = 1 / r
        for p, q, s in ((a, a, g), (b, b, g), (a, b, -g), (b, a, -g)):
            if p in idx and q in idx:
                A[idx[p], idx[q]] += s
    for k, (n, v) in enumerate(sources):           # voltage sources from ground to node n
        r = len(nodes) + k
        A[idx[n], r] += 1
        A[r, idx[n]] += 1
        z[r] = v
    r = m - 1                                      # the op-amp: output current unknown, constraint v+ - v- = 0
    A[idx[output], r] += 1
    if inverting in idx:
        A[r, idx[inverting]] -= 1
    if noninverting in idx:
        A[r, idx[noninverting]] += 1
    x = np.linalg.solve(A, z)
    return {n: x[i] for n, i in idx.items()}


def _reference_summing(V, R, Rf):
    """INDEPENDENT reference: modified nodal analysis of the circuit (numpy) with the ideal op-amp as a nullor."""
    res = [(f"in{k}", "n", r) for k, r in enumerate(R)] + [("n", "out", Rf)]
    src = [(f"in{k}", v) for k, v in enumerate(V)]
    sol = _ideal_opamp_mna(res, src, "n", "0", "out")
    return {"Vout": float(sol["out"])}


def _verify_summing(inputs, outputs):
    """Kirchhoff's current law at the virtual-ground node: the input currents V_k/R_k and the feedback current
    Vout/Rf sum to zero."""
    terms = [v / r for v, r in zip(inputs["V"], inputs["R"])] + [outputs["Vout"] / inputs["Rf"]]
    scale = math.fsum(abs(t) for t in terms) or 1.0
    return abs(math.fsum(terms)) / scale + max(abs(g + inputs["Rf"] / r) for g, r in zip(outputs.get("gains", [-inputs["Rf"] / r for r in inputs["R"]]), inputs["R"]))


def parse_summing(question):
    q = question.lower()
    if not re.search(r"\bsumming amplifier\b", q):
        return None
    # every stated voltage and every input resistor is bound, in the order written, whether they come as two lists or
    # as pairs ("10000 ohm carrying -1.76 V, 47000 ohm carrying 4.46 V"); unequal counts are refused by the input check.
    # Before 2026-10-23 only the first run of each was read and the rest silently dropped (tools/ask_sweep.py).
    out = {}
    volts = [v for _s, _e, vals in _runs(question, "voltage") for v in vals]
    fb = re.search(r"\bfeedback resist(?:or|ance)\b", question, re.I)
    res = []
    for s, e, vals in _runs(question, "resistance"):
        if fb and s > fb.end() and len(vals) == 1 and "Rf" not in out:
            out["Rf"] = vals[0]
        else:
            res += vals
    if volts:
        out["V"] = volts
    if res:
        out["R"] = res
    return out or None                                  # unequal counts: the input check refuses them with its reason


PROCEDURES["summing_amplifier"] = {
    "name": "Inverting summing amplifier output", "topic": "operational amplifiers / summing amplifier",
    "purpose": "The output of an ideal inverting summing amplifier: Vout = -Rf (V1/R1 + V2/R2 + ...).",
    "inputs": [{"symbol": "V", "type": "vector", "min_length": 1, "max_length": 20, "required": True, "meaning": "input voltages in volts"},
               {"symbol": "R", "type": "vector", "min_length": 1, "max_length": 20, "required": True, "meaning": "input resistors in ohms (one per input)"},
               {"symbol": "Rf", "kind": "resistance", "required": True, "meaning": "feedback resistor"}],
    "outputs": [{"symbol": "Vout", "kind": "voltage", "meaning": "output voltage"},
                {"symbol": "gains", "kind": "number", "meaning": "gain of each input, -Rf/R_k"}],
    "primary": "Vout", "conditions": ["ideal op-amp (infinite gain, no saturation)", "non-inverting input grounded"],
    "method": "virtual ground at the inverting input; Kirchhoff's current law there", "latex": r"V_{out} = -R_f\sum_k \frac{V_k}{R_k}",
    "run": summing_amplifier, "verify": _verify_summing, "parse": parse_summing, "reference": _reference_summing,
    "wrong": lambda V, R, Rf: {"Vout": math.fsum(Rf / r * v for v, r in zip(V, R)), "gains": [Rf / r for r in R]},
    "random_inputs": lambda rng: (lambda n: {"V": [round(rng.uniform(-5, 5), 2) for _ in range(n)], "R": [_e12(rng, 3, 5) for _ in range(n)],
                                             "Rf": _e12(rng, 3, 5)})(rng.randint(1, 4)),
    "templates": ["An inverting summing amplifier has inputs of 1 V, 2 V and -0.5 V through input resistors of 10 kohm, 20 kohm and 10 kohm, and a feedback resistor of 100 kohm. Find the output voltage."],
    "examples": [{"inputs": {"V": "1, 2, -0.5", "R": "10000, 20000, 10000", "Rf": 100000}, "outputs": {"Vout": -15.0}},
                 {"inputs": {"V": [0.2, 0.3], "R": [1000, 1000], "Rf": 10000}, "outputs": {"Vout": -5.0}}],
    "questions": [{"q": "An inverting summing amplifier has inputs of 1 V, 2 V and -0.5 V through input resistors of 10 kohm, 20 kohm and 10 kohm, and a feedback resistor of 100 kohm. Find the output voltage.",
                   "outputs": {"Vout": -15.0}},
                  {"q": "A summing amplifier has inputs of 0.2 V and 0.3 V through input resistors of 1 kohm and 1 kohm, and a feedback resistor of 10 kohm. Find the output voltage.",
                   "outputs": {"Vout": -5.0}},
                  {"q": "A summing amplifier has inputs of 0.2 V and 0.3 V through input resistors of 1 kohm, 1 kohm and 2 kohm, and a feedback resistor of 10 kohm. Find the output voltage.",
                   "outcome": "INVALID_INPUT"}]}


def difference_amplifier(V1, V2, R1, R2, R3, R4) -> dict:
    _check_positive(R1=R1, R2=R2, R3=R3, R4=R4)
    vout = R4 / (R3 + R4) * (1 + R2 / R1) * V2 - R2 / R1 * V1
    return {"Vout": vout, "gain_inverting": -R2 / R1, "gain_noninverting": R4 / (R3 + R4) * (1 + R2 / R1)}


def _verify_diffamp(inputs, outputs):
    """Kirchhoff's current law at both op-amp inputs with v+ = v- (divider at the + input, currents at the - input)."""
    V1, V2, R1, R2, R3, R4 = (inputs[k] for k in ("V1", "V2", "R1", "R2", "R3", "R4"))
    vo = outputs["Vout"]
    vp = V2 * R4 / (R3 + R4)
    terms = [(V1 - vp) / R1, (vo - vp) / R2]
    return abs(math.fsum(terms)) / (math.fsum(abs(t) for t in terms) or 1.0) + _rel(outputs["gain_inverting"], -R2 / R1)


def _reference_diffamp(V1, V2, R1, R2, R3, R4):
    """INDEPENDENT reference: modified nodal analysis of the four-resistor circuit (numpy) with the ideal op-amp stamp."""
    sol = _ideal_opamp_mna([("a", "n", R1), ("n", "out", R2), ("b", "p", R3), ("p", "0", R4)], [("a", V1), ("b", V2)], "n", "p", "out")
    return {"Vout": float(sol["out"])}


def parse_diffamp(question):
    if not re.search(r"\b(difference|differential) amplifier\b", question, re.I):
        return None
    out = _named_values(question, ["R1", "R2", "R3", "R4"], "resistance")
    out.update(_named_values(question, ["V1", "V2"], "voltage"))
    return out or None


PROCEDURES["difference_amplifier"] = {
    "name": "Difference amplifier output (four-resistor op-amp circuit)", "topic": "operational amplifiers / difference amplifier",
    "purpose": "The output of the classic difference amplifier: V1 through R1 to the inverting input, R2 from output to that input, V2 through R3 to the non-inverting input, R4 from it to ground.",
    "inputs": [{"symbol": "V1", "kind": "voltage", "required": True, "meaning": "voltage at the inverting side"},
               {"symbol": "V2", "kind": "voltage", "required": True, "meaning": "voltage at the non-inverting side"},
               {"symbol": "R1", "kind": "resistance", "required": True, "meaning": "input resistor to the inverting input"},
               {"symbol": "R2", "kind": "resistance", "required": True, "meaning": "feedback resistor"},
               {"symbol": "R3", "kind": "resistance", "required": True, "meaning": "input resistor to the non-inverting input"},
               {"symbol": "R4", "kind": "resistance", "required": True, "meaning": "resistor from the non-inverting input to ground"}],
    "outputs": [{"symbol": "Vout", "kind": "voltage", "meaning": "output voltage"},
                {"symbol": "gain_inverting", "kind": "number", "meaning": "gain from V1, -R2/R1"},
                {"symbol": "gain_noninverting", "kind": "number", "meaning": "gain from V2, R4/(R3+R4) (1 + R2/R1)"}],
    "primary": "Vout", "conditions": ["ideal op-amp (infinite gain, no saturation)"],
    "method": "superposition with v+ = v-", "latex": r"V_{out} = \frac{R_4}{R_3+R_4}\left(1+\frac{R_2}{R_1}\right)V_2 - \frac{R_2}{R_1}V_1",
    "run": difference_amplifier, "verify": _verify_diffamp, "parse": parse_diffamp, "reference": _reference_diffamp,
    "wrong": lambda V1, V2, R1, R2, R3, R4: {"Vout": R2 / R1 * (V2 - V1), "gain_inverting": -R2 / R1, "gain_noninverting": R2 / R1},
    "random_inputs": lambda rng: {"V1": round(rng.uniform(-5, 5), 2), "V2": round(rng.uniform(-5, 5), 2), "R1": _e12(rng, 3, 4),
                                  "R2": _e12(rng, 3, 5), "R3": _e12(rng, 3, 4), "R4": _e12(rng, 3, 5)},
    "templates": ["A difference amplifier has R1 = 10 kohm, R2 = 100 kohm, R3 = 10 kohm, R4 = 100 kohm, V1 = 1 V and V2 = 1.5 V. Find the output voltage."],
    "examples": [{"inputs": {"V1": 1, "V2": 1.5, "R1": 1e4, "R2": 1e5, "R3": 1e4, "R4": 1e5}, "outputs": {"Vout": 5.0}},
                 {"inputs": {"V1": 2, "V2": 1, "R1": 1e3, "R2": 2e3, "R3": 1e3, "R4": 1e3}, "outputs": {"Vout": 1.5 - 4.0}}],
    "questions": [{"q": "A difference amplifier has R1 = 10 kohm, R2 = 100 kohm, R3 = 10 kohm, R4 = 100 kohm, V1 = 1 V and V2 = 1.5 V. Find the output voltage.",
                   "outputs": {"Vout": 5.0}},
                  {"q": "A difference amplifier has R1 = 10 kohm, R2 = 100 kohm, R3 = 10 kohm, V1 = 1 V and V2 = 1.5 V. Find the output voltage.",
                   "outcome": "NEEDS_INPUT"}]}


# ================================================================================ diodes and transistors (DC bias)
def bjt_divider_bias(VCC, R1, R2, RC, RE, beta, VBE) -> dict:
    _check_positive(VCC=VCC, R1=R1, R2=R2, RC=RC, RE=RE, beta=beta, VBE=VBE)
    vth = VCC * R2 / (R1 + R2)
    rth = R1 * R2 / (R1 + R2)
    ib = (vth - VBE) / (rth + (beta + 1) * RE)
    if ib <= 0:
        raise ValueError("the base-emitter junction is not forward biased: the transistor is cut off")
    ic, ie = beta * ib, (beta + 1) * ib
    ve = ie * RE
    vc = VCC - ic * RC
    vce = vc - ve
    if vce < 0.2:
        raise ValueError("VCE would be below 0.2 V: the transistor is saturated and the active-region model does not apply")
    return {"IC": ic, "IC_mA": ic * 1e3, "IB": ib, "IE": ie, "VB": ve + VBE, "VE": ve, "VC": vc, "VCE": vce}


def _verify_bjt(inputs, outputs):
    """Kirchhoff's current law at the base node (divider currents vs base current), VBE across the junction,
    IC = beta IB, KVL around the collector loop."""
    VCC, R1, R2, RC, RE, beta, VBE = (inputs[k] for k in ("VCC", "R1", "R2", "RC", "RE", "beta", "VBE"))
    ib, vb, ve, vc = outputs["IB"], outputs["VB"], outputs["VE"], outputs["VC"]
    i1, i2 = (VCC - vb) / R1, vb / R2
    ic = outputs["IC"]
    return (abs(i1 - i2 - ib) / max(i1, 1e-300) + abs(vb - ve - VBE) / VCC + _rel(ic, beta * ib)
            + abs(ve - (beta + 1) * ib * RE) / VCC + abs(VCC - ic * RC - vc) / VCC + _rel(outputs["IC_mA"], ic * 1e3)
            + abs(outputs["VCE"] - (vc - ve)) / VCC)


def _reference_bjt(VCC, R1, R2, RC, RE, beta, VBE):
    """INDEPENDENT reference: modified nodal analysis of the full bias circuit (numpy): the base-emitter junction as
    a VBE source, the collector as a current-controlled current source beta*IB."""
    import numpy as np
    # unknowns: vB, vC, vE, iVCC, iVBE (iVBE = IB, flowing from base to emitter)
    gc, g1, g2, ge = 1 / RC, 1 / R1, 1 / R2, 1 / RE
    A = np.array([
        [g1 + g2, 0, 0, 0, 1.0],                      # base: (vB - VCC)/R1 + vB/R2 + IB = 0
        [0, gc, 0, 0, beta],                           # collector: (vC - VCC)/RC + beta IB = 0
        [0, 0, ge, 0, -(1 + beta)],                    # emitter: vE/RE - (1+beta) IB = 0
        [1.0, 0, -1.0, 0, 0],                          # vB - vE = VBE
        [0, 0, 0, 1.0, 0]])
    z = np.array([VCC * g1, VCC * gc, 0.0, VBE, 0.0])
    vb, vc, ve, _, ib = np.linalg.solve(A, z)
    return {"IC": float(beta * ib), "IC_mA": float(beta * ib * 1e3), "IB": float(ib), "VE": float(ve), "VC": float(vc), "VCE": float(vc - ve)}


def parse_bjt(question):
    q = question.lower()
    if not re.search(r"\b(bjt|transistor)\b", q) or not re.search(r"\b(divider|bias)", q):
        return None
    out = _named_values(question, ["VCC", "VBE"], "voltage")
    out.update(_named_values(question, ["R1", "R2", "RC", "RE"], "resistance"))
    b = re.search(rf"\b(?:beta|β|hFE)\s*=\s*({_NUM_NB})", question)
    if b:
        out["beta"] = float(b.group(1))
    return out or None


PROCEDURES["bjt_divider_bias"] = {
    "name": "DC operating point of a voltage-divider biased BJT", "topic": "diodes and transistors / BJT biasing",
    "purpose": "Base, collector and emitter currents and the node voltages of an NPN transistor with divider bias (R1 to VCC, R2 to ground), collector resistor RC and emitter resistor RE.",
    "inputs": [{"symbol": "VCC", "kind": "voltage", "required": True, "meaning": "supply voltage"},
               {"symbol": "R1", "kind": "resistance", "required": True, "meaning": "divider resistor from VCC to the base"},
               {"symbol": "R2", "kind": "resistance", "required": True, "meaning": "divider resistor from the base to ground"},
               {"symbol": "RC", "kind": "resistance", "required": True, "meaning": "collector resistor"},
               {"symbol": "RE", "kind": "resistance", "required": True, "meaning": "emitter resistor"},
               {"symbol": "beta", "kind": "number", "required": True, "meaning": "DC current gain"},
               {"symbol": "VBE", "kind": "voltage", "required": True, "meaning": "base-emitter voltage in conduction (e.g. 0.7 V)"}],
    "outputs": [{"symbol": "IC", "kind": "current", "meaning": "collector current"},
                {"symbol": "IC_mA", "kind": "number", "meaning": "collector current in milliamperes"},
                {"symbol": "IB", "kind": "current", "meaning": "base current"}, {"symbol": "IE", "kind": "current", "meaning": "emitter current"},
                {"symbol": "VB", "kind": "voltage", "meaning": "base voltage"}, {"symbol": "VE", "kind": "voltage", "meaning": "emitter voltage"},
                {"symbol": "VC", "kind": "voltage", "meaning": "collector voltage"}, {"symbol": "VCE", "kind": "voltage", "meaning": "collector-emitter voltage"}],
    "primary": "IC_mA", "conditions": ["NPN in the forward-active region (VCE >= 0.2 V)", "constant VBE and beta"],
    "method": "Thevenin equivalent of the divider, then IB = (VTH - VBE)/(RTH + (beta+1) RE) (the exact analysis, base current included)",
    "latex": r"I_B = \frac{V_{TH} - V_{BE}}{R_{TH} + (\beta+1)R_E},\quad V_{TH} = V_{CC}\frac{R_2}{R_1+R_2}",
    "run": bjt_divider_bias, "verify": _verify_bjt, "parse": parse_bjt, "reference": _reference_bjt,
    "wrong": lambda VCC, R1, R2, RC, RE, beta, VBE: (lambda ie: {"IC": ie, "IC_mA": ie * 1e3, "IB": ie / beta, "IE": ie, "VB": ie * RE + VBE, "VE": ie * RE,
                                                                 "VC": VCC - ie * RC, "VCE": VCC - ie * (RC + RE)})(
        (VCC * R2 / (R1 + R2) - VBE) / RE),
    "random_inputs": lambda rng: {"VCC": rng.choice([5.0, 9.0, 12.0, 15.0, 24.0]), "R1": _e12(rng, 4, 4), "R2": _e12(rng, 3, 4),
                                  "RC": _e12(rng, 2, 3), "RE": _e12(rng, 2, 3), "beta": float(rng.randint(50, 300)), "VBE": 0.7},
    "templates": ["A voltage-divider biased BJT has VCC = 12 V, R1 = 10 kohm, R2 = 2.2 kohm, RC = 3.6 kohm, RE = 1 kohm, beta = 100 and VBE = 0.7 V. Find the collector current."],
    "examples": [{"inputs": {"VCC": 12, "R1": 1e4, "R2": 2200, "RC": 3600, "RE": 1000, "beta": 100, "VBE": 0.7},
                  "outputs": {"IB": (12 * 2200 / 12200 - 0.7) / (1e4 * 2200 / 12200 + 101 * 1000)}},
                 {"inputs": {"VCC": 10, "R1": 1e4, "R2": 1e4, "RC": 1000, "RE": 1000, "beta": 99, "VBE": 0.7},
                  "outputs": {"IB": 4.3 / (5000 + 100000)}}],
    "questions": [{"q": "A voltage-divider biased BJT has VCC = 12 V, R1 = 10 kohm, R2 = 2.2 kohm, RC = 3.6 kohm, RE = 1 kohm, beta = 100 and VBE = 0.7 V. Find the collector current.",
                   "outputs": {"IC": 100 * (12 * 2200 / 12200 - 0.7) / (1e4 * 2200 / 12200 + 101 * 1000)}},
                  {"q": "A voltage-divider biased BJT has VCC = 12 V, R1 = 1 kohm, R2 = 10 kohm, RC = 10 kohm, RE = 100 ohm, beta = 200 and VBE = 0.7 V. Find the collector current.",
                   "outcome": "INVALID_INPUT"},
                  {"q": "A voltage-divider biased BJT has VCC = 12 V, R1 = 10 kohm, R2 = 2.2 kohm, RC = 3.6 kohm, RE = 1 kohm and VBE = 0.7 V. Find the collector current.",
                   "outcome": "NEEDS_INPUT"}]}


def zener_regulator(Vin, R, Vz, RL) -> dict:
    _check_positive(Vin=Vin, R=R, Vz=Vz, RL=RL)
    if Vin * RL / (R + RL) <= Vz:
        raise ValueError("without the Zener the load voltage would not reach Vz: the diode is not in breakdown (no regulation)")
    ir, il = (Vin - Vz) / R, Vz / RL
    iz = ir - il
    return {"VL": Vz, "IR": ir, "IL": il, "IZ": iz, "IZ_mA": iz * 1e3, "PZ": Vz * iz}


def _verify_zener(inputs, outputs):
    Vin, R, Vz, RL = inputs["Vin"], inputs["R"], inputs["Vz"], inputs["RL"]
    ir, il, iz = outputs["IR"], outputs["IL"], outputs["IZ"]
    return (abs(Vin - ir * R - outputs["VL"]) / Vin + abs(outputs["VL"] - Vz) / Vz + abs(ir - il - iz) / ir
            + abs(il * RL - outputs["VL"]) / Vz + _rel(outputs["IZ_mA"], iz * 1e3) + abs(outputs["PZ"] - Vz * iz) / (Vz * ir) + (iz <= 0))


def _reference_zener(Vin, R, Vz, RL):
    """INDEPENDENT reference: nodal analysis (numpy) with the Zener in breakdown as an ideal voltage source Vz across
    the load; its current is the Zener current."""
    import numpy as np
    # unknowns: vL, i_z (current into the Zener source from node L to ground)
    A = np.array([[1 / R + 1 / RL, 1.0], [1.0, 0.0]])
    z = np.array([Vin / R, Vz])
    vl, iz = np.linalg.solve(A, z)
    return {"VL": float(vl), "IZ": float(iz), "IZ_mA": float(iz * 1e3), "PZ": float(vl * iz)}


def parse_zener(question):
    if not re.search(r"\bzener\b", question, re.I):
        return None
    out = {}
    for sym, phrases, kind in (("Vin", ["input voltage", "supply voltage", "source voltage"], "voltage"),
                               ("R", ["series resistor", "series resistance"], "resistance"),
                               ("Vz", ["zener voltage", "Zener voltage", "breakdown voltage"], "voltage"),
                               ("RL", ["load resistance", "load resistor", "load of"], "resistance")):
        v = _phrase_value(question, phrases, kind)
        if v is not None:
            out[sym] = v
    return out or None


PROCEDURES["zener_regulator"] = {
    "name": "Zener diode shunt regulator", "topic": "diodes and transistors / Zener regulator",
    "purpose": "Load voltage and the currents of a Zener shunt regulator (series resistor R from the input, Zener and load in parallel).",
    "inputs": [{"symbol": "Vin", "kind": "voltage", "required": True, "meaning": "input voltage"},
               {"symbol": "R", "kind": "resistance", "required": True, "meaning": "series resistor"},
               {"symbol": "Vz", "kind": "voltage", "required": True, "meaning": "Zener (breakdown) voltage"},
               {"symbol": "RL", "kind": "resistance", "required": True, "meaning": "load resistance"}],
    "outputs": [{"symbol": "VL", "kind": "voltage", "meaning": "load voltage"},
                {"symbol": "IR", "kind": "current", "meaning": "current in the series resistor"},
                {"symbol": "IL", "kind": "current", "meaning": "load current"},
                {"symbol": "IZ", "kind": "current", "meaning": "Zener current"},
                {"symbol": "IZ_mA", "kind": "number", "meaning": "Zener current in milliamperes"},
                {"symbol": "PZ", "kind": "power", "meaning": "power dissipated in the Zener"}],
    "primary": "IZ_mA", "conditions": ["the Zener is in breakdown (ideal: constant Vz)", "positive values"],
    "method": "I_R = (Vin - Vz)/R, I_L = Vz/RL, I_Z = I_R - I_L", "latex": r"I_Z = \frac{V_{in} - V_Z}{R} - \frac{V_Z}{R_L}",
    "run": zener_regulator, "verify": _verify_zener, "parse": parse_zener, "reference": _reference_zener,
    "wrong": lambda Vin, R, Vz, RL: (lambda ir: {"VL": Vz, "IR": ir, "IL": Vz / RL, "IZ": ir, "IZ_mA": ir * 1e3, "PZ": Vz * ir})((Vin - Vz) / R),
    "random_inputs": lambda rng: (lambda vz: {"Vin": round(vz * rng.uniform(1.6, 3), 1), "R": _e12(rng, 2, 2), "Vz": vz,
                                              "RL": _e12(rng, 3, 3)})(rng.choice([3.3, 5.1, 6.2, 9.1, 12.0])),
    "templates": ["A Zener regulator has an input voltage of 20 V, a series resistor of 1 kohm, a Zener voltage of 12 V and a load resistance of 2 kohm. Find the Zener current."],
    "examples": [{"inputs": {"Vin": 20, "R": 1000, "Vz": 12, "RL": 2000}, "outputs": {"IZ": 0.002, "IL": 0.006, "IR": 0.008}},
                 {"inputs": {"Vin": 12, "R": 220, "Vz": 5.1, "RL": 1000}, "outputs": {"IZ": 6.9 / 220 - 0.0051}}],
    "questions": [{"q": "A Zener regulator has an input voltage of 20 V, a series resistor of 1 kohm, a Zener voltage of 12 V and a load resistance of 2 kohm. Find the Zener current.",
                   "outputs": {"IZ": 0.002}},
                  {"q": "A Zener regulator has an input voltage of 15 V, a series resistor of 1 kohm, a Zener voltage of 12 V and a load resistance of 1 kohm. Find the Zener current.",
                   "outcome": "INVALID_INPUT"},
                  {"q": "A Zener regulator has an input voltage of 20 V, a Zener voltage of 12 V and a load resistance of 2 kohm. Find the Zener current.",
                   "outcome": "NEEDS_INPUT"}]}


# ================================================================================ transients: series RLC step response
def series_rlc_step(R, L, C, V, t) -> dict:
    _check_positive(R=R, L=L, C=C)
    if not (math.isfinite(V) and math.isfinite(t) and t >= 0):
        raise ValueError("the step voltage must be finite and the time zero or positive")
    a, w0 = R / (2 * L), 1 / math.sqrt(L * C)
    z = a / w0
    if abs(z - 1) <= 1e-9:
        vc = V * (1 - (1 + a * t) * math.exp(-a * t))
        i = V / L * t * math.exp(-a * t)
        kind = "critically damped"
    elif z < 1:
        wd = math.sqrt(w0 * w0 - a * a)
        e = math.exp(-a * t)
        vc = V * (1 - e * (math.cos(wd * t) + a / wd * math.sin(wd * t)))
        i = V / (wd * L) * e * math.sin(wd * t)
        kind = "underdamped"
    else:
        r = math.sqrt(a * a - w0 * w0)
        s1, s2 = -a + r, -a - r
        e1, e2 = math.exp(s1 * t), math.exp(s2 * t)
        vc = V * (1 + (s2 * e1 - s1 * e2) / (s1 - s2))
        i = V / (L * (s1 - s2)) * (e1 - e2)
        kind = "overdamped"
    return {"vC": vc, "i": i, "i_mA": i * 1e3, "alpha": a, "omega0": w0, "zeta": z, "response": kind}


def _verify_rlc_step(inputs, outputs):
    """Kirchhoff's voltage law at the instant t: V = R i + L di/dt + vC, with di/dt from the energy balance replaced
    by a central difference of the procedure's own response; plus the definitions of alpha, omega0 and zeta."""
    R, L, C, V, t = (inputs[k] for k in ("R", "L", "C", "V", "t"))
    a, w0 = R / (2 * L), 1 / math.sqrt(L * C)
    h = 1e-4 / max(a, w0)
    f = lambda tt: series_rlc_step(R, L, C, V, max(tt, 0.0))                   # noqa: E731
    lo, hi = f(t - h), f(t + h)
    didt = (hi["i"] - lo["i"]) / (2 * h) if t >= h else (hi["i"] - outputs["i"]) / h
    dvdt = (hi["vC"] - lo["vC"]) / (2 * h) if t >= h else (hi["vC"] - outputs["vC"]) / h
    scale = abs(V) or 1.0
    kvl = abs(V - R * outputs["i"] - L * didt - outputs["vC"]) / scale
    cap = abs(C * dvdt - outputs["i"]) * R / scale                              # i = C dvC/dt
    here = f(t)
    same = abs(here["vC"] - outputs["vC"]) / scale + abs(here["i"] - outputs["i"]) * R / scale
    return (same + (kvl if kvl > 1e-5 else 0.0) + (cap if cap > 1e-5 else 0.0) + _rel(outputs["alpha"], a) + _rel(outputs["omega0"], w0)
            + _rel(outputs["zeta"], a / w0) + _rel(outputs["i_mA"], outputs["i"] * 1e3))


def _reference_rlc_step(R, L, C, V, t):
    """INDEPENDENT reference: scipy.integrate.solve_ivp (DOP853, rtol 1e-13) of L di/dt = V - R i - vC,
    C dvC/dt = i from rest."""
    from scipy import integrate
    if t == 0:
        return {"vC": 0.0, "i": 0.0, "i_mA": 0.0}
    sol = integrate.solve_ivp(lambda tt, y: [(V - R * y[0] - y[1]) / L, y[0] / C], (0, t), [0.0, 0.0], method="DOP853",
                              rtol=1e-13, atol=1e-15 * max(1.0, abs(V)))
    i, vc = sol.y[0][-1], sol.y[1][-1]
    return {"vC": float(vc), "i": float(i), "i_mA": float(i * 1e3), "alpha": R / (2 * L), "omega0": 1 / (L * C) ** 0.5}


def parse_rlc_step(question):
    q = question.lower()
    if not re.search(r"\bseries rlc\b", q) or "step" not in q:
        return None
    out = _rlc_parts(question)
    out.pop("f", None)
    v = re.search(rf"({_NUM_NB})\s*V\s+step\b|\bstep (?:of|voltage of)\s*({_NUM_NB})\s*V\b", question)
    if v:
        out["V"] = float(v.group(1) or v.group(2))
    tt = re.search(rf"\bt\s*=\s*({_NUM_NB})\s*({_unit_alt('time')})(?![A-Za-z])(?![^.?]*\bt\s*=)", question)
    if tt:
        out["t"] = float(tt.group(1)) * _UNITS["time"][tt.group(2)]
    return out or None


PROCEDURES["series_rlc_step"] = {
    "name": "Step response of a series RLC circuit", "topic": "transients / second-order circuits",
    "purpose": "Capacitor voltage and loop current at time t after a DC step V is applied to a series RLC circuit at rest, with the damping classification.",
    "inputs": [{"symbol": "R", "kind": "resistance", "required": True, "meaning": "resistance"},
               {"symbol": "L", "kind": "inductance", "required": True, "meaning": "inductance"},
               {"symbol": "C", "kind": "capacitance", "required": True, "meaning": "capacitance"},
               {"symbol": "V", "kind": "voltage", "required": True, "meaning": "step voltage applied at t = 0"},
               {"symbol": "t", "kind": "time", "required": True, "meaning": "time after the step"}],
    "outputs": [{"symbol": "vC", "kind": "voltage", "meaning": "capacitor voltage at t"},
                {"symbol": "i", "kind": "current", "meaning": "loop current at t"},
                {"symbol": "i_mA", "kind": "number", "meaning": "loop current in milliamperes"},
                {"symbol": "alpha", "kind": "number", "meaning": "neper frequency R/(2L) in 1/s"},
                {"symbol": "omega0", "kind": "angular_frequency", "meaning": "undamped resonant frequency 1/sqrt(LC)"},
                {"symbol": "zeta", "kind": "number", "meaning": "damping ratio alpha/omega0"},
                {"symbol": "response", "kind": "text", "meaning": "underdamped, critically damped or overdamped"}],
    "primary": "vC", "conditions": ["circuit at rest before the step (vC = 0, i = 0)", "ideal components"],
    "method": "closed-form solution of L C vC'' + R C vC' + vC = V by the roots of the characteristic equation",
    "latex": r"LC\,\ddot v_C + RC\,\dot v_C + v_C = V,\quad \alpha = \frac{R}{2L},\ \omega_0 = \frac{1}{\sqrt{LC}}",
    "run": series_rlc_step, "verify": _verify_rlc_step, "parse": parse_rlc_step, "reference": _reference_rlc_step,
    "wrong": lambda R, L, C, V, t: dict(series_rlc_step(2 * R, L, C, V, t), alpha=R / (2 * L)),
    "random_inputs": lambda rng: (lambda R, L, C: {"R": R, "L": L, "C": C, "V": round(rng.uniform(1, 24), 1),
                                                   "t": round(rng.uniform(0.05, 3) * min(max(2 * L / R, (L * C) ** 0.5), 20 * 2 * math.pi * (L * C) ** 0.5), 12)})(
        _e12(rng, 0, 3), _e12(rng, -3, 0), _e12(rng, -7, -4)),
    "templates": ["A series RLC circuit with R = 10 ohm, L = 0.1 H and C = 100 uF is connected to a 12 V step. Find the capacitor voltage at t = 5 ms."],
    "examples": [{"inputs": {"R": 10, "L": 0.1, "C": 1e-4, "V": 12, "t": 0.005}, "outputs": {"zeta": 50 / 316.22776601683796, "response": "underdamped"}},
                 {"inputs": {"R": 1000, "L": 0.1, "C": 1e-6, "V": 5, "t": 0.001}, "outputs": {"response": "overdamped"}}],
    "questions": [{"q": "A series RLC circuit with R = 10 ohm, L = 0.1 H and C = 100 uF is connected to a 12 V step. Find the capacitor voltage at t = 5 ms.",
                   "outputs": {"response": "underdamped"}},
                  {"q": "A series RLC circuit with R = 10 ohm, L = 0.1 H and C = 100 uF is connected to a 12 V step. Find the capacitor voltage.",
                   "outcome": "NEEDS_INPUT"},
                  {"q": "A series RLC circuit with R = 0 ohm, L = 0.1 H and C = 100 uF is connected to a 12 V step. Find the capacitor voltage at t = 5 ms.",
                   "outcome": "INVALID_INPUT"}]}


# ================================================================================================ digital logic
_BOOL_RE = re.compile(r"\s*(?:(AND|OR|NOT|XOR)(?![A-Za-z0-9_])|([A-Za-z_][A-Za-z0-9_]*)|([01])(?![\w.])|(&&|\|\||[&|^~!¬·*+()'⊕]))", re.I)


def _bool_tokens(text):
    pos, out = 0, []
    text = text.strip()
    while pos < len(text):
        m = _BOOL_RE.match(text, pos)
        if not m or m.end() == pos:
            raise ValueError(f"cannot read the Boolean expression at '{text[pos:pos + 12]}'")
        kw, var, const, op = m.groups()
        if kw:
            out.append(kw.upper())
        elif var:
            out.append(("v", var))
        elif const:
            out.append(("c", int(const)))
        else:
            out.append({"&&": "AND", "&": "AND", "·": "AND", "*": "AND", "||": "OR", "|": "OR", "+": "OR", "^": "XOR",
                        "⊕": "XOR", "~": "NOT", "!": "NOT", "¬": "NOT"}.get(op, op))
        pos = m.end()
    if not out:
        raise ValueError("the expression is empty")
    return out


def _bool_parse(text):
    """Precedence: NOT (prefix ~ ! NOT, postfix ') > AND (& * · AND) > XOR (^ XOR) > OR (| + OR); parentheses group.
    Juxtaposition (AB) is not read as AND: write A·B or A AND B."""
    toks = _bool_tokens(text)
    pos = [0]

    def peek():
        return toks[pos[0]] if pos[0] < len(toks) else None

    def take():
        if pos[0] >= len(toks):
            raise ValueError("the Boolean expression ends too early")
        pos[0] += 1
        return toks[pos[0] - 1]

    def binary(op, sub):
        def f():
            items = [sub()]
            while peek() == op:
                take()
                items.append(sub())
            return items[0] if len(items) == 1 else (op.lower(), items)
        return f

    def unary():
        if peek() == "NOT":
            take()
            return ("not", unary())
        t = take()
        if t == "(":
            node = orx()
            if take() != ")":
                raise ValueError("unbalanced parentheses")
        elif isinstance(t, tuple):
            node = ("var", t[1]) if t[0] == "v" else ("const", t[1])
        else:
            raise ValueError(f"unexpected {t!r} in the Boolean expression")
        while peek() == "'":
            take()
            node = ("not", node)
        return node
    andx = binary("AND", unary)
    xorx = binary("XOR", andx)
    orx = binary("OR", xorx)
    tree = orx()
    if pos[0] != len(toks):
        raise ValueError(f"unexpected {toks[pos[0]]!r} in the Boolean expression")
    return tree


def _bool_vars(tree, acc=None):
    acc = set() if acc is None else acc
    if tree[0] == "var":
        acc.add(tree[1])
    elif tree[0] == "not":
        _bool_vars(tree[1], acc)
    elif tree[0] in ("and", "or", "xor"):
        for x in tree[1]:
            _bool_vars(x, acc)
    return acc


def _bool_eval(tree, env):
    t = tree[0]
    if t == "var":
        return env[tree[1]]
    if t == "const":
        return tree[1]
    if t == "not":
        return 1 - _bool_eval(tree[1], env)
    vals = [_bool_eval(x, env) for x in tree[1]]
    if t == "and":
        return int(all(vals))
    if t == "or":
        return int(any(vals))
    return sum(vals) % 2


def _assignments(text):
    """'A=1, B=0, C=1' -> {'A': 1, ...}."""
    out = {}
    for name, v in re.findall(r"([A-Za-z_][A-Za-z0-9_]*)\s*=\s*([^,;\s]+)", text):
        if v not in ("0", "1"):
            raise ValueError(f"{name} must be 0 or 1")
        out[name] = int(v)
    return out


def boolean_evaluate(expression, values) -> dict:
    tree = _bool_parse(expression)
    env = _assignments(values)
    missing = sorted(_bool_vars(tree) - set(env))
    if missing:
        raise ValueError("give a value for " + ", ".join(missing))
    return {"value": float(_bool_eval(tree, env))}


def _to_sympy_text(expr):
    """An independent rewriting of the written expression into Python's operators for sympy (reference only)."""
    s = expr
    s = re.sub(r"\bAND\b|&&|·|\*", " & ", s, flags=re.I)
    s = re.sub(r"\bXOR\b|⊕", " ^ ", s, flags=re.I)
    s = re.sub(r"\bOR\b|\|\||\+", " | ", s, flags=re.I)
    s = re.sub(r"\bNOT\b|!|¬", " ~", s, flags=re.I)
    while "'" in s:
        i = s.index("'")
        j = i - 1
        while j >= 0 and s[j] == " ":
            j -= 1
        if s[j] == ")":
            depth, k = 0, j
            while True:
                depth += {")": 1, "(": -1}.get(s[k], 0)
                if depth == 0:
                    break
                k -= 1
            s = s[:k] + "(~" + s[k:j + 1] + ")" + s[i + 1:]
        else:
            k = j
            while k > 0 and (s[k - 1].isalnum() or s[k - 1] == "_"):
                k -= 1
            s = s[:k] + "(~" + s[k:j + 1] + ")" + s[i + 1:]
    s = re.sub(r"(?<![\w.])1(?![\w.])", " _T ", s)
    s = re.sub(r"(?<![\w.])0(?![\w.])", " _F ", s)
    return s


def _sympy_bool(expr):
    import sympy as sp
    names = set(re.findall(r"[A-Za-z_][A-Za-z0-9_]*", _to_sympy_text(expr))) - {"_T", "_F"}
    loc = {n: sp.Symbol(n) for n in names}
    loc.update({"_T": sp.true, "_F": sp.false})
    return sp.sympify(_to_sympy_text(expr), locals=loc, convert_xor=False), loc


def _reference_bool_eval(expression, values):
    """INDEPENDENT reference: the expression rewritten into Python operators, parsed by sympy and evaluated by
    substitution."""
    import sympy as sp
    e, loc = _sympy_bool(expression)
    env = {loc[k]: (sp.true if v else sp.false) for k, v in _assignments(values).items() if k in loc}
    return {"value": 1.0 if bool(e.subs(env)) else 0.0}


def _random_bool_expr(rng, names=("A", "B", "C", "D"), depth=0):
    if depth >= 3 or (depth and rng.random() < 0.35):
        v = rng.choice(names)
        return v + "'" if rng.random() < 0.3 else v
    op = rng.choice([" AND ", " OR ", " XOR ", " + ", " · ", " AND "])
    parts = [_random_bool_expr(rng, names, depth + 1) for _ in range(rng.randint(2, 3))]
    s = op.join(parts)
    if rng.random() < 0.2:
        return f"NOT ({s})"
    return f"({s})" if depth else s


def _random_assign(rng, expr):
    names = sorted(set(re.findall(r"\b[A-D]\b", expr)))
    return ", ".join(f"{n}={rng.randint(0, 1)}" for n in names)


def parse_bool_eval(question):
    m = re.search(r"\bevaluate\s+(?:the\s+(?:boolean|logic)\s+expression\s+)?(.+?)\s+(?:for|with|when|where)\s+((?:[A-Za-z_]\w*\s*=\s*[^,;\s]+\s*(?:,|and)?\s*)+)\s*[.?]?\s*$", question, re.I)
    if not m:
        return None
    return {"expression": m.group(1).strip(), "values": re.sub(r"\s+and\s+", ", ", m.group(2).strip().rstrip(".?"))}


def _wrong_bool_eval(expression, values):
    """A deliberately wrong reading: OR evaluated before AND (precedence reversed), via the sympy text with & and |
    exchanged and the groups re-bracketed by Python's own precedence."""
    tree = _bool_parse(expression)

    def flip(t):
        if t[0] in ("var", "const"):
            return t
        if t[0] == "not":
            return ("not", flip(t[1]))
        return ({"and": "or", "or": "and", "xor": "xor"}[t[0]], [flip(x) for x in t[1]])
    return {"value": float(_bool_eval(flip(tree), _assignments(values)))}


PROCEDURES["boolean_evaluate"] = {
    "name": "Evaluate a Boolean expression", "topic": "digital logic / Boolean algebra",
    "purpose": "The value (0 or 1) of a Boolean expression for given values of its variables.",
    "inputs": [{"symbol": "expression", "type": "text", "min_length": 1, "max_length": 500, "required": True,
                "meaning": "the expression: AND (&, ·, *), OR (|, +), XOR (^), NOT (~, !, NOT, or a postfix '), parentheses; "
                           "NOT > AND > XOR > OR"},
               {"symbol": "values", "type": "text", "min_length": 3, "max_length": 500, "required": True,
                "meaning": "the variables' values, e.g. 'A=1, B=0, C=1'"}],
    "outputs": [{"symbol": "value", "kind": "number", "meaning": "value of the expression (0 or 1)"}],
    "primary": "value", "conditions": ["every variable given 0 or 1", "products written with an operator (A·B, not AB)"],
    "method": "recursive-descent parse with NOT > AND > XOR > OR, then evaluation", "latex": r"F(A, B, \dots) \in \{0, 1\}",
    "run": boolean_evaluate, "verify": lambda i, o: abs(o["value"] - boolean_evaluate(i["expression"], i["values"])["value"]),
    "parse": parse_bool_eval, "reference": _reference_bool_eval, "wrong": _wrong_bool_eval,
    "random_inputs": lambda rng: (lambda e: {"expression": e, "values": _random_assign(rng, e)})(_random_bool_expr(rng)),
    "templates": ["Evaluate A AND (B OR NOT C) for A=1, B=0, C=0.", "Evaluate A·B' + C for A=1, B=0, C=0."],
    "examples": [{"inputs": {"expression": "A AND (B OR NOT C)", "values": "A=1, B=0, C=0"}, "outputs": {"value": 1.0}},
                 {"inputs": {"expression": "A·B' + C", "values": "A=1, B=1, C=0"}, "outputs": {"value": 0.0}},
                 {"inputs": {"expression": "A + B · C", "values": "A=0, B=1, C=0"}, "outputs": {"value": 0.0}}],
    "questions": [{"q": "Evaluate A AND (B OR NOT C) for A=1, B=0, C=0.", "outputs": {"value": 1.0}},
                  {"q": "Evaluate A·B' + C for A=1, B=0, C=0.", "outputs": {"value": 1.0}},
                  {"q": "Evaluate (A XOR B)' for A=1, B=1.", "outputs": {"value": 1.0}},
                  {"q": "Evaluate A AND B for A=1.", "outcome": "INVALID_INPUT"},
                  {"q": "Evaluate A AND B for A=1, B=2.", "outcome": "INVALID_INPUT"}]}


# ------------------------------------------------------------------------------------------------ minimization
def _prime_implicants(n, ones):
    """Quine-McCluskey: prime implicants as (value, mask) with mask bits = eliminated variables."""
    current = {(m, 0) for m in ones}
    primes = set()
    while current:
        nxt, used = set(), set()
        lst = sorted(current)
        for i in range(len(lst)):
            for j in range(i + 1, len(lst)):
                (v1, m1), (v2, m2) = lst[i], lst[j]
                d = v1 ^ v2
                if m1 == m2 and d and d & (d - 1) == 0:
                    nxt.add((v1 & ~d, m1 | d))
                    used.update((lst[i], lst[j]))
        primes |= current - used
        current = nxt
    return sorted(primes)


def _covers(imp, m):
    v, mask = imp
    return (m & ~mask) == v


def _literals(imp, n):
    return n - bin(imp[1]).count("1")


def _term_text(imp, names):
    n = len(names)
    v, mask = imp
    lits = [names[i] + ("" if (v >> (n - 1 - i)) & 1 else "'") for i in range(n) if not (mask >> (n - 1 - i)) & 1]
    return "·".join(lits) if lits else "1"


def _min_cover(n, on, dc):
    on, dc = sorted(set(on)), sorted(set(dc) - set(on))
    if not on:
        return []
    primes = _prime_implicants(n, set(on) | set(dc))
    essential = []
    for m in on:
        cov = [p for p in primes if _covers(p, m)]
        if len(cov) == 1 and cov[0] not in essential:
            essential.append(cov[0])
    left = [m for m in on if not any(_covers(p, m) for p in essential)]
    rest = [p for p in primes if p not in essential]
    best = None
    if left:
        for k in range(1, len(rest) + 1):
            for combo in itertools.combinations(rest, k):
                if all(any(_covers(p, m) for p in combo) for m in left):
                    key = (k, sum(_literals(p, n) for p in combo), sorted(combo))
                    if best is None or key < best:
                        best = key
            if best is not None:
                break
    chosen = essential + (list(best[2]) if best else [])
    return chosen


def _sop_out(n, names, on, dc=()):
    cover = _min_cover(n, on, dc)
    if not cover:
        text = "0"
    elif any(p[1] == (1 << n) - 1 for p in cover):
        text, cover = "1", [(0, (1 << n) - 1)]
    else:
        text = " + ".join(_term_text(p, names) for p in cover)
    lits = sum(_literals(p, n) for p in cover)
    if text not in ("0", "1"):
        text = " + ".join(sorted(text.split(" + ")))      # terms in alphabetical order of their text
    return {"minimal_sop": text, "terms": float(0 if text == "0" else len(cover)), "literals": float(0 if text == "1" else lits)}


def _canonical_sop(n, names, on):
    return " + ".join(_term_text((m, 0), names) for m in on) or "0"


def truth_table_sop(expression) -> dict:
    tree = _bool_parse(expression)
    names = sorted(_bool_vars(tree))
    n = len(names)
    if n == 0:
        raise ValueError("the expression has no variables")
    if n > 4:
        raise ValueError("minimization is offered for up to 4 variables")
    on = [m for m in range(2 ** n) if _bool_eval(tree, {nm: (m >> (n - 1 - i)) & 1 for i, nm in enumerate(names)})]
    out = {"variables": ", ".join(names), "minterms": [float(m) for m in on],
           "maxterms": [float(m) for m in range(2 ** n) if m not in on], "canonical_sop": _canonical_sop(n, names, on)}
    out.update(_sop_out(n, names, on))
    return out


def _sop_check(text, n, names, on, dc=()):
    """The SOP text read back: same truth table on every row outside the don't-cares; its term and literal counts."""
    tree = _bool_parse(text)
    bad = 0
    for m in range(2 ** n):
        if m in dc:
            continue
        env = {nm: (m >> (n - 1 - i)) & 1 for i, nm in enumerate(names)}
        bad += _bool_eval(tree, env) != (m in on)
    if text in ("0", "1"):
        return bad, (0 if text == "0" else 1), 0
    terms = text.split(" + ")
    return bad, len(terms), sum(len(t.split("·")) for t in terms)


def _verify_tt(inputs, outputs):
    tree = _bool_parse(inputs["expression"])
    names = sorted(_bool_vars(tree))
    n = len(names)
    on = [m for m in range(2 ** n) if _bool_eval(tree, {nm: (m >> (n - 1 - i)) & 1 for i, nm in enumerate(names)})]
    bad, t, lit = _sop_check(outputs["minimal_sop"], n, names, set(on))
    bad2, _, _ = _sop_check(outputs["canonical_sop"], n, names, set(on))
    best = _sop_out(n, names, on)                     # minimality: the exact cover's counts
    return (bad + bad2 + abs(outputs["terms"] - t) + abs(outputs["literals"] - lit)
            + abs(outputs["terms"] - best["terms"]) + abs(outputs["literals"] - best["literals"])
            + (outputs["minterms"] != [float(m) for m in on]) + (outputs["variables"] != ", ".join(names)))


def _brute_min(n, on, dc=()):
    """INDEPENDENT minimum: every cube over n variables (3^n), the implicants, the primes among them, then an
    exhaustive search over sets of primes for the smallest cover (fewest terms, then fewest literals)."""
    on, care = set(on), set(on) | set(dc)
    if not on:
        return 0, 0
    cubes = []
    for spec in itertools.product((0, 1, None), repeat=n):
        rows = [m for m in range(2 ** n) if all(s is None or ((m >> (n - 1 - i)) & 1) == s for i, s in enumerate(spec))]
        if set(rows) <= care:
            cubes.append((frozenset(rows), sum(s is not None for s in spec)))
    primes = [c for c in cubes if not any(c[0] < d[0] for d in cubes)]
    for k in range(1, len(primes) + 1):
        best = None
        for combo in itertools.combinations(primes, k):
            if on <= frozenset().union(*(c[0] for c in combo)):
                lits = sum(c[1] for c in combo)
                best = lits if best is None else min(best, lits)
        if best is not None:
            return k, best
    return None


def _reference_tt(expression):
    """INDEPENDENT reference: the truth table from sympy (the expression rewritten into Python operators), and the
    minimum term and literal counts by exhaustive search over prime cubes."""
    import sympy as sp
    e, loc = _sympy_bool(expression)
    names = sorted(loc[k].name for k in loc if k not in ("_T", "_F"))
    n = len(names)
    on = []
    for m in range(2 ** n):
        env = {sp.Symbol(nm): (sp.true if (m >> (n - 1 - i)) & 1 else sp.false) for i, nm in enumerate(names)}
        if bool(e.subs(env)):
            on.append(m)
    k, lits = _brute_min(n, on)
    if len(on) == 2 ** n:
        k, lits = 1, 0
    return {"minterms": [float(m) for m in on], "terms": float(k), "literals": float(lits)}


def _wrong_tt(expression):
    r = truth_table_sop(expression)
    n = len(r["variables"].split(", "))
    return dict(r, minimal_sop=r["canonical_sop"], terms=float(len(r["minterms"])), literals=float(n * len(r["minterms"])))


def parse_tt(question):
    q = question.lower()
    if not re.search(r"\b(minimi[sz]e|simplify|minimal sum[- ]of[- ]products|truth table|minimal sop)\b", q):
        return None
    if re.search(r"σ|∑|\bminterms?\b", q):
        return None
    m = re.search(r"(?:\bminimal\s+sum[- ]of[- ]products\s+of|\bminimal\s+sop\s+of|\btruth\s+table\s+of|\bsimplify|\bminimi[sz]e)\s+"
                  r"(?:the\s+(?:boolean|logic)\s+(?:expression|function)\s+)?(?:F\s*=\s*)?(.+?)\s*[.?]?\s*$", question, re.I)
    if not m:
        return None
    expr = m.group(1).strip()
    try:
        _bool_parse(expr)
    except ValueError:
        return None
    return {"expression": expr}


PROCEDURES["truth_table_sop"] = {
    "name": "Truth table and minimal sum of products of a Boolean expression", "topic": "digital logic / minimization",
    "purpose": "The minterms (truth table), the canonical sum of products and a minimal sum of products (fewest terms, then fewest literals) of an expression in up to 4 variables.",
    "inputs": [PROCEDURES["boolean_evaluate"]["inputs"][0]],
    "outputs": [{"symbol": "variables", "kind": "text", "meaning": "the variables in truth-table order (first = most significant bit)"},
                {"symbol": "minterms", "kind": "number", "meaning": "rows where the expression is 1"},
                {"symbol": "maxterms", "kind": "number", "meaning": "rows where it is 0"},
                {"symbol": "canonical_sop", "kind": "text", "meaning": "sum of all minterms"},
                {"symbol": "minimal_sop", "kind": "text", "meaning": "a minimal sum of products (A' means NOT A, · means AND)"},
                {"symbol": "terms", "kind": "number", "meaning": "number of product terms in the minimal form"},
                {"symbol": "literals", "kind": "number", "meaning": "number of literals in the minimal form"}],
    "primary": "terms", "conditions": ["1 to 4 variables"],
    "method": "truth table; Quine-McCluskey prime implicants; essential primes, then an exact search for the smallest cover",
    "latex": r"F = \sum m(\dots)",
    "run": truth_table_sop, "verify": _verify_tt, "parse": parse_tt, "reference": _reference_tt, "wrong": _wrong_tt,
    "random_inputs": lambda rng: {"expression": _random_bool_expr(rng)},
    "templates": ["Simplify A·B + A·B' + A'·B.", "Find the minimal sum of products of (A + B)·(A + C)."],
    "examples": [{"inputs": {"expression": "A·B + A·B' + A'·B"}, "outputs": {"minimal_sop": "A + B", "terms": 2.0, "literals": 2.0, "minterms": [1.0, 2.0, 3.0]}},
                 {"inputs": {"expression": "(A + B)·(A + C)"}, "outputs": {"minimal_sop": "A + B·C", "terms": 2.0, "literals": 3.0}},
                 {"inputs": {"expression": "A XOR B"}, "outputs": {"terms": 2.0, "literals": 4.0}}],
    "questions": [{"q": "Simplify A·B + A·B' + A'·B.", "outputs": {"terms": 2.0, "literals": 2.0}},
                  {"q": "Find the minimal sum of products of (A + B)·(A + C).", "outputs": {"terms": 2.0, "literals": 3.0}},
                  {"q": "Simplify A·B·C·D·E + A'.", "outcome": "INVALID_INPUT"}]}


def minterms_to_sop(minterms, variables, dont_cares=None) -> dict:
    n = int(variables)
    if not 1 <= n <= 4:
        raise ValueError("minimization is offered for 1 to 4 variables")
    on = sorted({int(m) for m in minterms})
    dc = sorted({int(m) for m in (dont_cares or [])})
    for m in list(minterms) + list(dont_cares or []):
        if m != int(m) or not 0 <= m < 2 ** n:
            raise ValueError(f"minterm numbers must be whole numbers from 0 to {2 ** n - 1}")
    if set(on) & set(dc):
        raise ValueError("a row cannot be both a minterm and a don't-care")
    names = "ABCD"[:n]
    out = _sop_out(n, names, on, dc)
    out["variables"] = ", ".join(names)
    return out


def _verify_m2s(inputs, outputs):
    n = int(inputs["variables"])
    dc = {int(m) for m in (inputs.get("dont_cares") or [])}
    bad, t, lit = _sop_check(outputs["minimal_sop"], n, "ABCD"[:n], {int(m) for m in inputs["minterms"]}, dc)
    best = _sop_out(n, "ABCD"[:n], sorted({int(m) for m in inputs["minterms"]}), sorted(dc))
    return bad + abs(outputs["terms"] - t) + abs(outputs["literals"] - lit) + abs(outputs["terms"] - best["terms"]) + abs(outputs["literals"] - best["literals"])


def parse_m2s(question):
    m = re.search(r"(?:Σ|∑|sum of)\s*m\s*\(([^)]*)\)", question, re.I) or re.search(r"\bminterms?\s+((?:\d+\s*,\s*)*\d+)", question, re.I)
    if not m or not re.search(r"\b(minimi[sz]e|simplify|minimal)\b", question, re.I):
        return None
    out = {"minterms": m.group(1)}
    d = re.search(r"\bd\s*\(([^)]*)\)|don'?t[- ]cares?\s+((?:\d+\s*,\s*)*\d+)", question, re.I)
    if d:
        out["dont_cares"] = d.group(1) or d.group(2)
    f = re.search(r"\bF\s*\(\s*([A-Za-z](?:\s*,\s*[A-Za-z])*)\s*\)", question)
    v = re.search(r"\b(\d+)[- ]variables?\b|\bof\s+(\d+)\s+variables\b", question)
    if f:
        names = [x.strip() for x in f.group(1).split(",")]
        if names != list("ABCD"[:len(names)]):
            return None                                   # other variable names: not this template
        out["variables"] = len(names)
    elif v:
        out["variables"] = int(v.group(1) or v.group(2))
    return out


PROCEDURES["minterms_to_sop"] = {
    "name": "Minimal sum of products from minterms (with don't-cares)", "topic": "digital logic / minimization",
    "purpose": "A minimal sum of products (fewest terms, then fewest literals) of F(A, B, C, D) = Σm(...) + d(...), the Karnaugh-map result.",
    "inputs": [{"symbol": "minterms", "type": "vector", "min_length": 0, "max_length": 16, "required": True, "meaning": "minterm numbers (A is the most significant bit)"},
               {"symbol": "variables", "type": "integer", "min": 1, "max": 4, "required": True, "meaning": "number of variables (1 to 4)"},
               {"symbol": "dont_cares", "type": "vector", "min_length": 0, "max_length": 16, "required": False, "meaning": "don't-care rows"}],
    "outputs": [{"symbol": "minimal_sop", "kind": "text", "meaning": "a minimal sum of products in A, B, C, D (A' = NOT A)"},
                {"symbol": "terms", "kind": "number", "meaning": "number of product terms"},
                {"symbol": "literals", "kind": "number", "meaning": "number of literals"},
                {"symbol": "variables", "kind": "text", "meaning": "the variables"}],
    "primary": "terms", "conditions": ["1 to 4 variables", "minterms and don't-cares disjoint and in range"],
    "method": "Quine-McCluskey prime implicants over minterms and don't-cares; exact minimum cover of the minterms",
    "latex": r"F = \sum m(\dots) + \sum d(\dots)",
    "run": minterms_to_sop, "verify": _verify_m2s, "parse": parse_m2s,
    "reference": lambda minterms, variables, dont_cares=None: (lambda r: {"terms": float(r[0]), "literals": float(r[1])})(
        (lambda on, dc, n: (1, 0) if len(set(on) | set(dc)) == 2 ** n and on else _brute_min(n, on, dc))(
            [int(m) for m in minterms], [int(m) for m in (dont_cares or [])], int(variables))),
    "wrong": lambda minterms, variables, dont_cares=None: (lambda r: dict(r, terms=float(len(minterms)), literals=float(int(variables) * len(minterms)),
                                                                          minimal_sop=_canonical_sop(int(variables), "ABCD"[:int(variables)], sorted(int(m) for m in minterms))))(
        minterms_to_sop(minterms, variables, dont_cares)),
    "random_inputs": lambda rng: (lambda n: (lambda rows: {"variables": n, "minterms": sorted(rows[:rng.randint(2, max(2, 2 ** n - 3))]),
                                                           "dont_cares": sorted(rows[-rng.randint(0, 2):]) if n > 2 and rng.random() < 0.5 else []})(
        rng.sample(range(2 ** n), 2 ** n)))(rng.choice([3, 4, 4])),
    "templates": ["Simplify F(A,B,C) = Σm(0, 1, 2, 5, 6, 7).", "Minimize F(A,B,C,D) = Σm(1, 3, 7, 11, 15) + d(0, 2, 5)."],
    "examples": [{"inputs": {"minterms": "0, 1, 2, 5, 6, 7", "variables": 3}, "outputs": {"terms": 3.0, "literals": 6.0}},
                 {"inputs": {"minterms": "1, 3, 7, 11, 15", "variables": 4, "dont_cares": "0, 2, 5"}, "outputs": {"terms": 2.0, "literals": 4.0}},
                 {"inputs": {"minterms": "0, 2, 8, 10", "variables": 4}, "outputs": {"minimal_sop": "B'·D'", "terms": 1.0}}],
    "questions": [{"q": "Simplify F(A,B,C) = Σm(0, 1, 2, 5, 6, 7).", "outputs": {"terms": 3.0, "literals": 6.0}},
                  {"q": "Minimize F(A,B,C,D) = Σm(1, 3, 7, 11, 15) + d(0, 2, 5).", "outputs": {"terms": 2.0, "literals": 4.0}},
                  {"q": "Simplify F(A,B,C) = Σm(0, 1, 9).", "outcome": "INVALID_INPUT"},
                  {"q": "Simplify the function with minterms 1, 3, 5.", "outcome": "NEEDS_INPUT"}]}


def boolean_equivalence(expression1, expression2) -> dict:
    t1, t2 = _bool_parse(expression1), _bool_parse(expression2)
    names = sorted(_bool_vars(t1) | _bool_vars(t2))
    if len(names) > 12:
        raise ValueError("at most 12 variables")
    n = len(names)
    for m in range(2 ** n):
        env = {nm: (m >> (n - 1 - i)) & 1 for i, nm in enumerate(names)}
        if _bool_eval(t1, env) != _bool_eval(t2, env):
            return {"equivalent": False, "counterexample": ", ".join(f"{k}={v}" for k, v in env.items())}
    return {"equivalent": True, "counterexample": "none"}


def _verify_equiv(inputs, outputs):
    t1, t2 = _bool_parse(inputs["expression1"]), _bool_parse(inputs["expression2"])
    if not isinstance(outputs["equivalent"], bool):
        return 1.0
    if outputs["equivalent"]:
        return float(not boolean_equivalence(inputs["expression1"], inputs["expression2"])["equivalent"])
    env = _assignments(outputs["counterexample"])
    try:
        return float(_bool_eval(t1, env) == _bool_eval(t2, env))
    except KeyError:
        return 1.0


def _reference_equiv(expression1, expression2):
    """INDEPENDENT reference: sympy.satisfiable of Xor(e1, e2) (a SAT search): equivalent exactly when unsatisfiable."""
    import sympy as sp
    e1, _ = _sympy_bool(expression1)
    e2, _ = _sympy_bool(expression2)
    return {"equivalent": sp.satisfiable(sp.Xor(e1, e2)) is False}


def _random_equiv(rng):
    e = _random_bool_expr(rng, ("A", "B", "C"))
    if rng.random() < 0.3:
        return {"expression1": e, "expression2": f"NOT (NOT ({e}))"}
    row = [rng.randint(0, 1) for _ in range(3)]
    while row in ([0, 0, 0], [1, 1, 1]):
        row = [rng.randint(0, 1) for _ in range(3)]
    term = " · ".join(v + ("" if b else "'") for v, b in zip("ABC", row))
    return {"expression1": e, "expression2": f"({e}) XOR ({term})"}


def parse_equiv(question):
    m = re.search(r"\b(?:is|are)\s+(.+?)\s+(?:and|equivalent to|equal to)\s+(.+?)(?:\s+equivalent)?\s*\?\s*$", question, re.I)
    if not m or not re.search(r"\bequivalent\b", question, re.I):
        return None
    a, b = m.group(1).strip(), m.group(2).strip()
    try:
        _bool_parse(a)
        _bool_parse(b)
    except ValueError:
        return None
    return {"expression1": a, "expression2": b}


PROCEDURES["boolean_equivalence"] = {
    "name": "Equivalence of two Boolean expressions", "topic": "digital logic / Boolean algebra",
    "purpose": "Whether two Boolean expressions are equal for every input, with a counterexample row when they are not.",
    "inputs": [{"symbol": "expression1", "type": "text", "min_length": 1, "max_length": 500, "required": True, "meaning": "first expression (same notation as boolean_evaluate)"},
               {"symbol": "expression2", "type": "text", "min_length": 1, "max_length": 500, "required": True, "meaning": "second expression"}],
    "outputs": [{"symbol": "equivalent", "kind": "boolean", "meaning": "true when the two agree on every row"},
                {"symbol": "counterexample", "kind": "text", "meaning": "the first row where they differ ('none' if equivalent)"}],
    "primary": "equivalent", "conditions": ["at most 12 variables"],
    "method": "exhaustive comparison over the truth table", "latex": r"F_1 \equiv F_2 \iff F_1 \oplus F_2 = 0",
    "run": boolean_equivalence, "verify": _verify_equiv, "parse": parse_equiv, "reference": _reference_equiv,
    "wrong": lambda expression1, expression2: (lambda t1, t2, names: {"equivalent": all(
        _bool_eval(t1, {n: b for n in names}) == _bool_eval(t2, {n: b for n in names}) for b in (0, 1)), "counterexample": "none"})(
        _bool_parse(expression1), _bool_parse(expression2), _bool_vars(_bool_parse(expression1)) | _bool_vars(_bool_parse(expression2))),
    "random_inputs": _random_equiv,
    "templates": ["Is (A·B)' equivalent to A' + B'?", "Are A + A'·B and A + B equivalent?"],
    "examples": [{"inputs": {"expression1": "(A·B)'", "expression2": "A' + B'"}, "outputs": {"equivalent": True}},
                 {"inputs": {"expression1": "A + B", "expression2": "A XOR B"}, "outputs": {"equivalent": False, "counterexample": "A=1, B=1"}}],
    "questions": [{"q": "Is (A·B)' equivalent to A' + B'?", "outputs": {"equivalent": True}},
                  {"q": "Are A + A'·B and A + B equivalent?", "outputs": {"equivalent": True}},
                  {"q": "Is A + B equivalent to A XOR B?", "outputs": {"equivalent": False}}]}


# ------------------------------------------------------------------------------------------------ number systems
_BASES = {"2": 2, "8": 8, "10": 10, "16": 16}
_BASE_WORDS = {"binary": "2", "octal": "8", "decimal": "10", "hexadecimal": "16", "hex": "16"}


def _clean_digits(value, base):
    v = value.strip().replace("_", "").replace(" ", "")
    pre = {"0b": 2, "0o": 8, "0x": 16}
    if v[:2].lower() in pre:
        if pre[v[:2].lower()] != base:
            raise ValueError(f"the prefix {v[:2]} does not match base {base}")
        v = v[2:]
    if not v or any(c not in "0123456789abcdef"[:base] for c in v.lower()):
        raise ValueError(f"'{value}' is not a whole number in base {base}")
    return v.lower()


def base_conversion(value, base) -> dict:
    b = _BASES[str(base)]
    n = int(_clean_digits(value, b), b)
    if n >= 2 ** 53:
        raise ValueError("values up to 2^53 - 1 are supported")
    return {"decimal": float(n), "binary": format(n, "b"), "octal": format(n, "o"), "hexadecimal": format(n, "X")}


def _verify_base(inputs, outputs):
    b = _BASES[str(inputs["base"])]
    n = int(_clean_digits(inputs["value"], b), b)
    return (abs(outputs["decimal"] - n) / max(1, n) + (int(outputs["binary"], 2) != n) + (int(outputs["octal"], 8) != n)
            + (int(outputs["hexadecimal"], 16) != n))


def _reference_base(value, base):
    """INDEPENDENT reference: the digits' positional value by numpy.polyval on exact integers, and the strings in
    each base from numpy.base_repr."""
    import numpy as np
    b = _BASES[str(base)]
    digits = [int(c, 16) for c in _clean_digits(value, b)]
    n = int(np.polyval(np.array(digits, dtype=object), b))
    return {"decimal": float(n), "binary": np.base_repr(n, 2), "octal": np.base_repr(n, 8), "hexadecimal": np.base_repr(n, 16)}


def _wrong_base(value, base):
    b = _BASES[str(base)]
    d = _clean_digits(value, b)[::-1]                                  # the digits read least significant first
    return base_conversion(d, base)


def parse_base(question):
    q = question.lower()
    if re.search(r"gray|two'?s|complement", q):
        return None
    m = re.search(r"\b(?:convert|what is|express)\s+(?:the\s+)?(?:(binary|octal|decimal|hexadecimal|hex)\s+(?:number\s+)?)?([0-9a-fA-FxXoObB_]+)\s+"
                  r"(?:from\s+(binary|octal|decimal|hexadecimal|hex)\s+)?(?:to|in|into)\s+(binary|octal|decimal|hexadecimal|hex)\b", question, re.I)
    if not m:
        return None
    src = (m.group(1) or m.group(3) or "").lower()
    val = m.group(2)
    pre = {"0b": "2", "0o": "8", "0x": "16"}.get(val[:2].lower())
    out = {"value": val}
    if src:
        out["base"] = _BASE_WORDS[src]
    elif pre:
        out["base"] = pre
    elif m.group(4).lower() != "decimal":
        out["base"] = "10"
    return out


PROCEDURES["base_conversion"] = {
    "name": "Number base conversion (binary, octal, decimal, hexadecimal)", "topic": "digital logic / number systems",
    "purpose": "A whole number written in base 2, 8, 10 or 16, given in all four bases.",
    "inputs": [{"symbol": "value", "type": "text", "min_length": 1, "max_length": 70, "required": True, "meaning": "the digits (0b, 0o, 0x prefixes allowed)"},
               {"symbol": "base", "type": "choice", "choices": ["2", "8", "10", "16"], "required": True, "meaning": "the base the value is written in"}],
    "outputs": [{"symbol": "decimal", "kind": "number", "meaning": "value in base 10"}, {"symbol": "binary", "kind": "text", "meaning": "base 2"},
                {"symbol": "octal", "kind": "text", "meaning": "base 8"}, {"symbol": "hexadecimal", "kind": "text", "meaning": "base 16 (upper-case digits)"}],
    "primary": "decimal", "conditions": ["a non-negative whole number below 2^53", "digits valid for the base"],
    "method": "positional value, then repeated division by the target base", "latex": r"N = \sum_i d_i\, b^i",
    "run": base_conversion, "verify": _verify_base, "parse": parse_base, "reference": _reference_base, "wrong": _wrong_base,
    "random_inputs": lambda rng: (lambda n, b: {"value": format(n, {"2": "b", "8": "o", "10": "d", "16": "x"}[b]), "base": b})(
        rng.randint(10, 2 ** rng.randint(5, 40)), rng.choice(["2", "8", "10", "16"])),
    "templates": ["Convert 1011 from binary to decimal.", "Convert 255 to hexadecimal.", "Convert 0x1F to binary."],
    "examples": [{"inputs": {"value": "1011", "base": "2"}, "outputs": {"decimal": 11.0, "hexadecimal": "B"}},
                 {"inputs": {"value": "255", "base": "10"}, "outputs": {"hexadecimal": "FF", "binary": "11111111", "octal": "377"}},
                 {"inputs": {"value": "0x1F", "base": "16"}, "outputs": {"decimal": 31.0, "binary": "11111"}}],
    "questions": [{"q": "Convert 1011 from binary to decimal.", "outputs": {"decimal": 11.0}},
                  {"q": "Convert 255 to hexadecimal.", "outputs": {"hexadecimal": "FF"}},
                  {"q": "Convert 0x1F to binary.", "outputs": {"binary": "11111"}},
                  {"q": "Convert the octal number 777 to decimal.", "outputs": {"decimal": 511.0}},
                  {"q": "Convert 1021 from binary to decimal.", "outcome": "INVALID_INPUT"},
                  {"q": "Convert 1011 to decimal.", "outcome": "NEEDS_INPUT"}]}


def twos_complement_encode(value, bits) -> dict:
    v, n = int(value), int(bits)
    if not 2 <= n <= 64:
        raise ValueError("the width must be 2 to 64 bits")
    if not -(2 ** (n - 1)) <= v < 2 ** (n - 1):
        raise ValueError(f"{v} does not fit in {n}-bit two's complement (range {-(2 ** (n - 1))} to {2 ** (n - 1) - 1})")
    u = v % (2 ** n)
    return {"pattern": format(u, f"0{n}b"), "hexadecimal": format(u, f"0{(n + 3) // 4}X"), "unsigned": float(u)}


def _verify_tc_enc(inputs, outputs):
    p, n = outputs["pattern"], int(inputs["bits"])
    if len(p) != n or set(p) - {"0", "1"}:
        return 1.0
    val = int(p, 2) - (2 ** n if p[0] == "1" else 0)
    return float(val != int(inputs["value"])) + float(int(outputs["hexadecimal"], 16) != int(p, 2)) + abs(outputs["unsigned"] - int(p, 2))


PROCEDURES["twos_complement_encode"] = {
    "name": "Two's complement representation of a signed integer", "topic": "digital logic / number systems",
    "purpose": "The n-bit two's complement bit pattern (and its hexadecimal form) of a signed whole number.",
    "inputs": [{"symbol": "value", "type": "integer", "required": True, "meaning": "the signed whole number"},
               {"symbol": "bits", "type": "integer", "min": 2, "max": 64, "required": True, "meaning": "word width in bits"}],
    "outputs": [{"symbol": "pattern", "kind": "text", "meaning": "the bit pattern, most significant bit first"},
                {"symbol": "hexadecimal", "kind": "text", "meaning": "the pattern in hexadecimal"},
                {"symbol": "unsigned", "kind": "number", "meaning": "the pattern read as an unsigned number"}],
    "primary": "unsigned", "conditions": ["-2^(n-1) <= value < 2^(n-1)"],
    "method": "value modulo 2^n (for a negative number: invert the bits of |value| and add 1)", "latex": r"x_{2c} = x \bmod 2^n",
    "run": twos_complement_encode, "verify": _verify_tc_enc, "parse": None,
    "reference": lambda value, bits: (lambda np: {"pattern": np.binary_repr(int(value), width=int(bits)),
                                                  "unsigned": float(int(np.binary_repr(int(value), width=int(bits)), 2))})(__import__("numpy")),
    "wrong": lambda value, bits: (lambda u: {"pattern": format(u, f"0{int(bits)}b"), "hexadecimal": format(u, "X"), "unsigned": float(u)})(
        (2 ** int(bits) - 1 - abs(int(value))) if int(value) < 0 else int(value)),
    "random_inputs": lambda rng: (lambda n: {"value": rng.randint(-(2 ** (n - 1)), -1) if rng.random() < 0.85 else rng.randint(0, 2 ** (n - 1) - 1),
                                             "bits": n})(rng.choice([4, 8, 12, 16, 32, 64])),
    "templates": ["Write -5 in 8-bit two's complement.", "Find the 16-bit two's complement representation of -1234."],
    "examples": [{"inputs": {"value": -5, "bits": 8}, "outputs": {"pattern": "11111011", "hexadecimal": "FB", "unsigned": 251.0}},
                 {"inputs": {"value": 100, "bits": 8}, "outputs": {"pattern": "01100100"}},
                 {"inputs": {"value": -1234, "bits": 16}, "outputs": {"hexadecimal": "FB2E"}}],
    "questions": [{"q": "Write -5 in 8-bit two's complement.", "outputs": {"pattern": "11111011"}},
                  {"q": "Find the 16-bit two's complement representation of -1234.", "outputs": {"hexadecimal": "FB2E"}},
                  {"q": "Write -200 in 8-bit two's complement.", "outcome": "INVALID_INPUT"}]}


def _parse_tc_enc(question):
    m = re.search(r"(?<![\w.])(-?\d+)\s+(?:in|as|to|into)\s+(\d+)[- ]bit\s+two'?s[- ]complement\b", question, re.I)
    if m:
        return {"value": int(m.group(1)), "bits": int(m.group(2))}
    m = re.search(r"\b(\d+)[- ]bit\s+two'?s[- ]complement\s+(?:representation\s+|form\s+|pattern\s+)?of\s+(-?\d+)\b", question, re.I)
    return {"value": int(m.group(2)), "bits": int(m.group(1))} if m else None


PROCEDURES["twos_complement_encode"]["parse"] = _parse_tc_enc


def twos_complement_decode(pattern) -> dict:
    p = pattern.strip()
    if not p or set(p) - {"0", "1"} or len(p) > 64:
        raise ValueError("the pattern must be 1 to 64 binary digits")
    n = len(p)
    return {"value": float(int(p, 2) - (2 ** n if p[0] == "1" else 0)), "bits": float(n)}


def _reference_tc_dec(pattern):
    """INDEPENDENT reference: the bits weighted by -2^(n-1), 2^(n-2), ..., 1 (numpy dot product on exact integers)."""
    import numpy as np
    bits = np.array([int(c) for c in pattern.strip()], dtype=object)
    n = len(bits)
    w = np.array([-(2 ** (n - 1))] + [2 ** (n - 1 - i) for i in range(1, n)], dtype=object)
    return {"value": float(int(bits.dot(w)))}


def _parse_tc_dec(question):
    m = re.search(r"\b(?:(\d+)[- ]bit\s+)?two'?s[- ]complement\s+(?:binary\s+)?(?:number|pattern|value|word|binary)?\s*([01]+)\b", question, re.I)
    if not m or not re.search(r"\b(decimal|value|represent|decode|signed)\b", question, re.I):
        return None
    if m.group(1) and int(m.group(1)) != len(m.group(2)):
        return {"_out_of_scope": "the stated width differs from the number of digits written"}
    return {"pattern": m.group(2)}


PROCEDURES["twos_complement_decode"] = {
    "name": "Value of a two's complement bit pattern", "topic": "digital logic / number systems",
    "purpose": "The signed value of an n-bit two's complement pattern (n = the number of digits written).",
    "inputs": [{"symbol": "pattern", "type": "text", "min_length": 1, "max_length": 64, "required": True, "meaning": "the bits, most significant first"}],
    "outputs": [{"symbol": "value", "kind": "number", "meaning": "signed value"}, {"symbol": "bits", "kind": "number", "meaning": "word width"}],
    "primary": "value", "conditions": ["only 0 and 1", "at most 64 bits"],
    "method": "unsigned value minus 2^n when the sign bit is 1", "latex": r"x = -b_{n-1}2^{n-1} + \sum_{i<n-1} b_i 2^i",
    "run": twos_complement_decode, "verify": lambda i, o: abs(o["value"] - twos_complement_decode(i["pattern"])["value"]) / max(1.0, abs(o["value"])),
    "parse": _parse_tc_dec, "reference": _reference_tc_dec,
    "wrong": lambda pattern: {"value": float(int(pattern, 2)), "bits": float(len(pattern))},
    "random_inputs": lambda rng: (lambda n: {"pattern": ("1" if rng.random() < 0.85 else "0") + "".join(rng.choice("01") for _ in range(n - 1))})(
        rng.choice([4, 8, 12, 16, 32])),
    "templates": ["What decimal value does the 8-bit two's complement number 11111011 represent?"],
    "examples": [{"inputs": {"pattern": "11111011"}, "outputs": {"value": -5.0}}, {"inputs": {"pattern": "0111"}, "outputs": {"value": 7.0}},
                 {"inputs": {"pattern": "1000"}, "outputs": {"value": -8.0}}],
    "questions": [{"q": "What decimal value does the 8-bit two's complement number 11111011 represent?", "outputs": {"value": -5.0}},
                  {"q": "Decode the two's complement pattern 1000.", "outputs": {"value": -8.0}},
                  {"q": "What decimal value does the 8-bit two's complement number 1111 represent?", "outcome": "OUT_OF_SCOPE"}]}


def _gray_table(n):
    g = ["0", "1"]
    for _ in range(n - 1):
        g = ["0" + x for x in g] + ["1" + x for x in reversed(g)]
    return g


def gray_code(value, direction) -> dict:
    v = value.strip()
    if not v or set(v) - {"0", "1"} or len(v) > 32:
        raise ValueError("the value must be 1 to 32 binary digits")
    n = len(v)
    x = int(v, 2)
    if direction == "to_gray":
        r = x ^ (x >> 1)
    else:
        r, s = x, x >> 1
        while s:
            r ^= s
            s >>= 1
    return {"result": format(r, f"0{n}b"), "decimal": float(r)}


def _verify_gray(inputs, outputs):
    v, r = inputs["value"].strip(), outputs["result"]
    if len(r) != len(v):
        return 1.0
    g, b = (r, v) if inputs["direction"] == "to_gray" else (v, r)
    gi, bi = int(g, 2), int(b, 2)
    return float(gi != bi ^ (bi >> 1)) + abs(outputs["decimal"] - int(r, 2))


def _reference_gray(value, direction):
    """INDEPENDENT reference: the reflected Gray code listed by its reflection construction (the n-bit list is the
    (n-1)-bit list prefixed 0, then reversed and prefixed 1); conversion is indexing into or searching that list."""
    v = value.strip()
    table = _gray_table(len(v))
    r = table[int(v, 2)] if direction == "to_gray" else format(table.index(v), f"0{len(v)}b")
    return {"result": r, "decimal": float(int(r, 2))}


def _parse_gray(question):
    m = re.search(r"\bconvert\s+(?:the\s+)?binary\s+(?:number\s+)?([01]+)\s+(?:to|into)\s+gray\b", question, re.I)
    if m:
        return {"value": m.group(1), "direction": "to_gray"}
    m = re.search(r"\bconvert\s+(?:the\s+)?gray[- ]code\s+(?:number\s+)?([01]+)\s+(?:to|into)\s+binary\b", question, re.I)
    return {"value": m.group(1), "direction": "to_binary"} if m else None


PROCEDURES["gray_code"] = {
    "name": "Binary to Gray code and Gray code to binary", "topic": "digital logic / number systems",
    "purpose": "The reflected binary (Gray) code of a binary word, or the binary word of a Gray code, with the same width.",
    "inputs": [{"symbol": "value", "type": "text", "min_length": 1, "max_length": 32, "required": True, "meaning": "the bits"},
               {"symbol": "direction", "type": "choice", "choices": ["to_gray", "to_binary"], "required": True, "meaning": "conversion direction"}],
    "outputs": [{"symbol": "result", "kind": "text", "meaning": "the converted bits"}, {"symbol": "decimal", "kind": "number", "meaning": "the result read as an unsigned number"}],
    "primary": "decimal", "conditions": ["1 to 32 bits"],
    "method": "g = b XOR (b >> 1); back: b = g XOR (g >> 1) XOR (g >> 2) ...", "latex": r"g = b \oplus (b \gg 1)",
    "run": gray_code, "verify": _verify_gray, "parse": _parse_gray, "reference": _reference_gray,
    "wrong": lambda value, direction: (lambda x, n: {"result": format((x ^ (x << 1)) % (2 ** n), f"0{n}b"), "decimal": float((x ^ (x << 1)) % (2 ** n))})(
        int(value, 2), len(value)),
    "random_inputs": lambda rng: {"value": "1" + "".join(rng.choice("01") for _ in range(rng.randint(2, 12))), "direction": rng.choice(["to_gray", "to_binary"])},
    "templates": ["Convert the binary number 1011 to Gray code.", "Convert the Gray code 1110 to binary."],
    "examples": [{"inputs": {"value": "1011", "direction": "to_gray"}, "outputs": {"result": "1110"}},
                 {"inputs": {"value": "1110", "direction": "to_binary"}, "outputs": {"result": "1011"}}],
    "questions": [{"q": "Convert the binary number 1011 to Gray code.", "outputs": {"result": "1110"}},
                  {"q": "Convert the Gray code 1110 to binary.", "outputs": {"result": "1011"}},
                  {"q": "Convert the binary number 1012 to Gray code.", "outcome": "UNRECOGNIZED_FORMAT"}]}


# ================================================================== formula capabilities asked as '<phrase> with A = .., B = ..'
_EXTRA_UNITS = {"transconductance": {"A/V^2": 1.0, "mA/V^2": 1e-3, "uA/V^2": 1e-6, "µA/V^2": 1e-6}}


def _kv_parse(must, inputs, forbid=None):
    """The one documented question shape: a capability phrase (regex `must`), then name = value pairs. A physical
    input needs one of its kind's units; a plain number takes none. An unknown name, a missing unit or any number not
    in a pair makes the reading refuse (None)."""
    by = {i["symbol"]: i for i in inputs}

    def parse(question):
        if not re.search(must, question, re.I) or (forbid and re.search(forbid, question, re.I)):
            return None
        out, spans = {}, []
        for m in re.finditer(rf"(?<![\w'])([A-Za-z][A-Za-z0-9_]*)\s*=\s*({_NUM_NB})", question):
            name, val = m.group(1), float(m.group(2))
            if name not in by:
                return None
            kind = by[name].get("kind", "number")
            table = _UNITS.get(kind) or _EXTRA_UNITS.get(by[name].get("units", ""))
            if kind != "number" and not table:
                return None                               # a physical quantity whose units this reader does not know
            end = m.end()
            if table:
                um = re.match(rf"\s*({'|'.join(re.escape(u) for u in sorted(table, key=len, reverse=True))})(?![A-Za-zΩ])", question[end:])
                if not um:
                    return None
                val *= table[um.group(1)]
                end += um.end()
            out[name] = val
            spans.append((m.start(), end))
        for n in re.finditer(_NUMRE, question):
            if not any(s <= n.start() < e for s, e in spans):
                return None
        return out or None
    return parse


def _fspec(name, topic, purpose, inputs, outputs, primary, conditions, method, latex, run, verify, reference, wrong, rnd,
           phrase, templates, examples, questions, forbid=None):
    return {"name": name, "topic": topic, "purpose": purpose, "inputs": inputs, "outputs": outputs, "primary": primary,
            "conditions": conditions, "method": method, "latex": latex, "run": run, "verify": verify,
            "parse": _kv_parse(phrase, inputs, forbid), "reference": reference, "wrong": wrong, "random_inputs": rnd,
            "templates": templates, "examples": examples, "questions": questions}


def _inp(sym, kind, meaning, **kw):
    return {"symbol": sym, "kind": kind, "required": True, "meaning": meaning, **kw}


def _out(sym, kind, meaning):
    return {"symbol": sym, "kind": kind, "meaning": meaning}


def _netlist_solve(text):
    """Reference helper: node voltages and source currents of a netlist, solved exactly by sympy."""
    v, i = _sympy_solve_circuit(_netlist(text))
    return {k: float(x) for k, x in v.items()}, {k: float(x) for k, x in i.items()}


# --------------------------------------------------------------------------------------------- dividers and bridges
def voltage_divider(Vin, R1, R2) -> dict:
    _check_positive(R1=R1, R2=R2)
    return {"Vout": Vin * R2 / (R1 + R2), "I": Vin / (R1 + R2)}


PROCEDURES["voltage_divider"] = _fspec(
    "Voltage divider", "DC circuits / dividers", "The output voltage across R2 of two resistors in series across Vin (R1 on top, R2 to ground), and the current.",
    [_inp("Vin", "voltage", "input voltage"), _inp("R1", "resistance", "top resistor (from the input to the output)"),
     _inp("R2", "resistance", "bottom resistor (from the output to ground)")],
    [_out("Vout", "voltage", "output voltage across R2 (unloaded)"), _out("I", "current", "current through the divider")],
    "Vout", ["no load on the output", "R1, R2 > 0"], "Vout = Vin R2 / (R1 + R2)", r"V_{out} = V_{in}\frac{R_2}{R_1 + R_2}",
    voltage_divider,
    lambda i, o: abs(o["I"] * i["R1"] + o["Vout"] - i["Vin"]) / max(abs(i["Vin"]), 1e-300) + abs(o["Vout"] - o["I"] * i["R2"]) / max(abs(i["Vin"]), 1e-300),
    lambda Vin, R1, R2: (lambda v, c: {"Vout": v["2"], "I": -c["V1"]})(*_netlist_solve(f"V1 1 0 {Vin!r}; R1 1 2 {R1!r}; R2 2 0 {R2!r}")),
    lambda Vin, R1, R2: {"Vout": Vin * R1 / (R1 + R2), "I": Vin / (R1 + R2)},
    lambda rng: {"Vin": round(rng.uniform(1, 30), 1), "R1": _e12(rng, 1, 5), "R2": _e12(rng, 1, 5)},
    r"\bvoltage divider\b",
    ["Find the output of a voltage divider with Vin = 12 V, R1 = 1 kohm, R2 = 2 kohm."],
    [{"inputs": {"Vin": 12, "R1": 1000, "R2": 2000}, "outputs": {"Vout": 8.0, "I": 0.004}},
     {"inputs": {"Vin": 5, "R1": 10000, "R2": 10000}, "outputs": {"Vout": 2.5}}],
    [{"q": "Find the output of a voltage divider with Vin = 12 V, R1 = 1 kohm, R2 = 2 kohm.", "outputs": {"Vout": 8.0}},
     {"q": "A voltage divider has Vin = 5 V, R1 = 10 kohm and R2 = 10 kohm. What is the output voltage?", "outputs": {"Vout": 2.5}},
     {"q": "Find the output of a voltage divider with Vin = 12 V, R1 = 1 kohm.", "outcome": "NEEDS_INPUT"},
     {"q": "Find the output of a voltage divider with Vin = 12 V, R1 = 1 kohm, R2 = 2 kohm, R3 = 5 kohm.", "outcome": "UNRECOGNIZED_FORMAT"}])


def current_divider(I, R1, R2) -> dict:
    _check_positive(R1=R1, R2=R2)
    return {"I1": I * R2 / (R1 + R2), "I2": I * R1 / (R1 + R2)}


PROCEDURES["current_divider"] = _fspec(
    "Current divider", "DC circuits / dividers", "How a current I splits between two resistors in parallel.",
    [_inp("I", "current", "total current into the parallel pair"), _inp("R1", "resistance", "first branch resistor"),
     _inp("R2", "resistance", "second branch resistor")],
    [_out("I1", "current", "current through R1"), _out("I2", "current", "current through R2")],
    "I1", ["R1, R2 > 0"], "I1 = I R2/(R1 + R2)", r"I_1 = I\frac{R_2}{R_1 + R_2}",
    current_divider,
    lambda i, o: abs(o["I1"] + o["I2"] - i["I"]) / max(abs(i["I"]), 1e-300) + abs(o["I1"] * i["R1"] - o["I2"] * i["R2"]) / max(abs(i["I"] * i["R1"]), 1e-300),
    lambda I, R1, R2: (lambda v: {"I1": v["1"] / R1, "I2": v["1"] / R2})(_netlist_solve(f"I1 0 1 {I!r}; R1 1 0 {R1!r}; R2 1 0 {R2!r}")[0]),
    lambda I, R1, R2: {"I1": I * R1 / (R1 + R2), "I2": I * R2 / (R1 + R2)},
    lambda rng: {"I": round(rng.uniform(0.01, 10), 3), "R1": _e12(rng, 0, 4), "R2": _e12(rng, 0, 4)},
    r"\bcurrent divider\b",
    ["Find the branch currents of a current divider with I = 6 A, R1 = 4 ohm, R2 = 2 ohm."],
    [{"inputs": {"I": 6, "R1": 4, "R2": 2}, "outputs": {"I1": 2.0, "I2": 4.0}}, {"inputs": {"I": 1, "R1": 100, "R2": 300}, "outputs": {"I1": 0.75}}],
    [{"q": "Find the branch currents of a current divider with I = 6 A, R1 = 4 ohm, R2 = 2 ohm.", "outputs": {"I1": 2.0, "I2": 4.0}},
     {"q": "Find the branch currents of a current divider with I = 6 A, R1 = 4 ohm, R2 = 0 ohm.", "outcome": "INVALID_INPUT"}])


def max_power_transfer(Vth, Rth) -> dict:
    _check_positive(Rth=Rth)
    return {"RL": Rth, "Pmax": Vth * Vth / (4 * Rth), "efficiency": 0.5}


def _reference_mpt(Vth, Rth):
    """INDEPENDENT reference: the load power P(RL) = Vth^2 RL/(Rth + RL)^2 differentiated by sympy and its stationary
    point solved exactly."""
    import sympy as sp
    RL = sp.Symbol("RL", positive=True)
    v, r = sp.nsimplify(Vth, rational=True), sp.nsimplify(Rth, rational=True)
    P = v ** 2 * RL / (r + RL) ** 2
    best = [x for x in sp.solve(sp.diff(P, RL), RL) if x > 0][0]
    return {"RL": float(best), "Pmax": float(P.subs(RL, best))}


PROCEDURES["max_power_transfer"] = _fspec(
    "Maximum power transfer", "DC circuits / Thevenin and Norton equivalents", "The load resistance that draws the most power from a source with Thevenin voltage Vth and resistance Rth, and that power.",
    [_inp("Vth", "voltage", "Thevenin (open-circuit) voltage"), _inp("Rth", "resistance", "Thevenin resistance")],
    [_out("RL", "resistance", "load resistance for maximum power (= Rth)"), _out("Pmax", "power", "maximum power delivered to the load"),
     _out("efficiency", "number", "fraction of the source power reaching the load at that point")],
    "Pmax", ["a resistive load", "Rth > 0"], "RL = Rth, Pmax = Vth^2/(4 Rth)", r"P_{max} = \frac{V_{th}^2}{4R_{th}}",
    max_power_transfer,
    lambda i, o: _rel(o["RL"], i["Rth"]) + _rel(o["Pmax"], i["Vth"] ** 2 * o["RL"] / (i["Rth"] + o["RL"]) ** 2) + abs(o["efficiency"] - 0.5),
    _reference_mpt, lambda Vth, Rth: {"RL": Rth, "Pmax": Vth * Vth / Rth, "efficiency": 1.0},
    lambda rng: {"Vth": round(rng.uniform(1, 50), 2), "Rth": _e12(rng, 0, 4)},
    r"\bmaximum power\b",
    ["Find the maximum power transfer for Vth = 12 V, Rth = 6 ohm."],
    [{"inputs": {"Vth": 12, "Rth": 6}, "outputs": {"RL": 6.0, "Pmax": 6.0}}, {"inputs": {"Vth": 10, "Rth": 50}, "outputs": {"Pmax": 0.5}}],
    [{"q": "Find the maximum power transfer for Vth = 12 V, Rth = 6 ohm.", "outputs": {"Pmax": 6.0, "RL": 6.0}},
     {"q": "What is the maximum power delivered to a load from a source with Vth = 10 V and Rth = 50 ohm?", "outputs": {"Pmax": 0.5}}])


def wheatstone_bridge(Vs, R1, R2, R3, R4) -> dict:
    _check_positive(R1=R1, R2=R2, R3=R3, R4=R4)
    vout = Vs * (R2 / (R1 + R2) - R4 / (R3 + R4))
    return {"Vout": vout, "balanced": abs(R1 * R4 - R2 * R3) <= 1e-12 * R1 * R4, "R4_balance": R2 * R3 / R1}


PROCEDURES["wheatstone_bridge"] = _fspec(
    "Wheatstone bridge output", "DC circuits / bridges", "The output voltage of a Wheatstone bridge (left arm R1 over R2, right arm R3 over R4, supply Vs), whether it is balanced, and the R4 that balances it.",
    [_inp("Vs", "voltage", "bridge supply voltage"), _inp("R1", "resistance", "left arm, top"), _inp("R2", "resistance", "left arm, bottom"),
     _inp("R3", "resistance", "right arm, top"), _inp("R4", "resistance", "right arm, bottom")],
    [_out("Vout", "voltage", "voltage of the left midpoint relative to the right midpoint"), _out("balanced", "boolean", "true when R1 R4 = R2 R3"),
     _out("R4_balance", "resistance", "the R4 that would balance the bridge")],
    "Vout", ["an unloaded output (ideal voltmeter)"], "difference of the two divider outputs", r"V_{out} = V_s\left(\frac{R_2}{R_1+R_2} - \frac{R_4}{R_3+R_4}\right)",
    wheatstone_bridge,
    lambda i, o: abs(o["Vout"] - i["Vs"] * (i["R2"] / (i["R1"] + i["R2"]) - i["R4"] / (i["R3"] + i["R4"]))) / max(abs(i["Vs"]), 1e-300) + _rel(o["R4_balance"] * i["R1"], i["R2"] * i["R3"]),
    lambda Vs, R1, R2, R3, R4: (lambda v: {"Vout": v["2"] - v["3"]})(_netlist_solve(f"V1 1 0 {Vs!r}; R1 1 2 {R1!r}; R2 2 0 {R2!r}; R3 1 3 {R3!r}; R4 3 0 {R4!r}")[0]),
    lambda Vs, R1, R2, R3, R4: dict(wheatstone_bridge(Vs, R1, R2, R3, R4), Vout=-wheatstone_bridge(Vs, R1, R2, R3, R4)["Vout"]),
    lambda rng: {"Vs": round(rng.uniform(1, 24), 1), "R1": _e12(rng, 1, 4), "R2": _e12(rng, 1, 4), "R3": _e12(rng, 1, 4), "R4": _e12(rng, 1, 4)},
    r"\bwheatstone\b",
    ["Find the output of a Wheatstone bridge with Vs = 10 V, R1 = 100 ohm, R2 = 200 ohm, R3 = 150 ohm, R4 = 250 ohm."],
    [{"inputs": {"Vs": 10, "R1": 100, "R2": 200, "R3": 150, "R4": 250}, "outputs": {"Vout": 10 * (2 / 3 - 250 / 400), "R4_balance": 300.0}},
     {"inputs": {"Vs": 5, "R1": 100, "R2": 100, "R3": 220, "R4": 220}, "outputs": {"Vout": 0.0, "balanced": True}}],
    [{"q": "Find the output of a Wheatstone bridge with Vs = 10 V, R1 = 100 ohm, R2 = 200 ohm, R3 = 150 ohm, R4 = 250 ohm.",
      "outputs": {"Vout": 10 * (2 / 3 - 250 / 400)}},
     {"q": "Find the output of a Wheatstone bridge with Vs = 10 V, R1 = 100 ohm, R2 = 200 ohm, R3 = 150 ohm.", "outcome": "NEEDS_INPUT"}])


# --------------------------------------------------------------------------------------------- filters and resonance
def rc_filter(R, C, f, filter) -> dict:                                       # noqa: A002
    _check_positive(R=R, C=C, f=f)
    fc = 1 / (2 * math.pi * R * C)
    x = f / fc
    h = 1 / complex(1, x) if filter == "lowpass" else complex(0, x) / complex(1, x)
    return {"fc": fc, "gain": abs(h), "gain_db": 20 * math.log10(abs(h)), "phase_deg": math.degrees(cmath.phase(h))}


def _reference_rcf(R, C, f, filter):                                          # noqa: A002
    """INDEPENDENT reference: scipy.signal.freqs of 1/(RCs + 1) or RCs/(RCs + 1); the cutoff found by scipy brentq where
    |H| = 1/sqrt(2)."""
    import numpy as np
    from scipy import optimize, signal
    b = [1.0] if filter == "lowpass" else [R * C, 0.0]
    a = [R * C, 1.0]
    mag = lambda ff: abs(signal.freqs(b, a, worN=[2 * np.pi * ff])[1][0]) - 2 ** -0.5      # noqa: E731
    guess = 1 / (R * C)
    fc = optimize.brentq(mag, guess * 1e-4, guess * 10, xtol=1e-300, rtol=8.9e-16, maxiter=500)
    h = complex(signal.freqs(b, a, worN=[2 * np.pi * f])[1][0])
    return {"fc": float(fc), "gain": abs(h), "gain_db": float(20 * np.log10(abs(h))), "phase_deg": float(np.angle(h, deg=True))}


_RCF_IN = [_inp("R", "resistance", "resistance"), _inp("C", "capacitance", "capacitance"), _inp("f", "frequency", "signal frequency"),
           {"symbol": "filter", "type": "choice", "choices": ["lowpass", "highpass"], "required": True, "meaning": "first-order low-pass or high-pass"}]


def _parse_rcf(question):
    q = question.lower()
    kind = "lowpass" if re.search(r"low[- ]?pass", q) else ("highpass" if re.search(r"high[- ]?pass", q) else None)
    if not re.search(r"\brc\b", q) or not kind:
        return None
    base = _kv_parse(r"\brc\b", _RCF_IN[:3])(re.sub(r"(?i)\b(low|high)[- ]?pass\b", "", question))
    return dict(base, filter=kind) if base else None


PROCEDURES["rc_filter"] = {
    "name": "First-order RC filter: cutoff, gain and phase", "topic": "AC circuits / filters",
    "purpose": "The cutoff frequency 1/(2 pi R C) of an RC low-pass or high-pass filter and its gain (ratio and dB) and phase at a frequency.",
    "inputs": _RCF_IN,
    "outputs": [_out("fc", "frequency", "cutoff (-3 dB) frequency"), _out("gain", "number", "|Vout/Vin| at f"),
                _out("gain_db", "number", "gain in decibels"), _out("phase_deg", "number", "phase of Vout relative to Vin, degrees")],
    "primary": "fc", "conditions": ["unloaded output", "R, C, f > 0"], "method": "H = 1/(1 + j f/fc) (low-pass) or (j f/fc)/(1 + j f/fc) (high-pass)",
    "latex": r"f_c = \frac{1}{2\pi RC},\quad H_{LP} = \frac{1}{1 + j f/f_c}",
    "run": rc_filter,
    "verify": lambda i, o: (lambda w, rc: (lambda h: _rel(o["gain"], abs(h)) + abs(o["phase_deg"] - math.degrees(cmath.phase(h))) / 180
                                           + _rel(o["fc"] * 2 * math.pi * rc, 1.0) + abs(o["gain_db"] - 20 * math.log10(abs(h))) / max(1.0, abs(o["gain_db"])))(
        (1 / (1 + 1j * w * rc)) if i["filter"] == "lowpass" else (1j * w * rc / (1 + 1j * w * rc))))(2 * math.pi * i["f"], i["R"] * i["C"]),
    "parse": _parse_rcf, "reference": _reference_rcf,
    "wrong": lambda R, C, f, filter: (lambda r: dict(r, fc=1 / (R * C)))(rc_filter(R, C, f, filter)),
    "random_inputs": lambda rng: {"R": _e12(rng, 1, 5), "C": _e12(rng, -9, -5), "f": _e12(rng, 0, 5), "filter": rng.choice(["lowpass", "highpass"])},
    "templates": ["An RC low-pass filter has R = 1 kohm and C = 100 nF. Find the gain at f = 2 kHz."],
    "examples": [{"inputs": {"R": 1000, "C": 1e-7, "f": 1000 / (2 * math.pi * 1e-4) / 1000, "filter": "lowpass"},
                  "outputs": {"fc": 1 / (2 * math.pi * 1e-4), "gain": 2 ** -0.5, "phase_deg": -45.0}},
                 {"inputs": {"R": 1000, "C": 1e-6, "f": 159.15494309189535, "filter": "highpass"}, "outputs": {"phase_deg": 45.0}}],
    "questions": [{"q": "An RC low-pass filter has R = 1 kohm and C = 100 nF. Find the gain at f = 2 kHz.",
                   "outputs": {"fc": 1 / (2 * math.pi * 1e-4), "gain": 1 / abs(complex(1, 2000 * 2 * math.pi * 1e-4))}},
                  {"q": "An RC high-pass filter has R = 10 kohm and C = 10 nF. Find its response at f = 100 Hz.",
                   "outputs": {"fc": 1 / (2 * math.pi * 1e-4)}},
                  {"q": "An RC low-pass filter has R = 1 kohm and C = 100 nF. Find the gain.", "outcome": "NEEDS_INPUT"}]}


def _resonance(R, L, C, series):
    _check_positive(R=R, L=L, C=C)
    w0 = 1 / math.sqrt(L * C)
    a = R / (2 * L) if series else 1 / (2 * R * C)       # half the bandwidth in rad/s
    w1, w2 = -a + math.sqrt(a * a + w0 * w0), a + math.sqrt(a * a + w0 * w0)
    tp = 2 * math.pi
    return {"f0": w0 / tp, "Q": w0 / (2 * a), "bandwidth": (w2 - w1) / tp, "f_low": w1 / tp, "f_high": w2 / tp}


def _verify_res(series):
    def verify(i, o):
        R, L, C = i["R"], i["L"], i["C"]
        def Y(f):                                       # series: the current per volt; parallel: the voltage per amp
            w = 2 * math.pi * f
            return 1 / complex(R, w * L - 1 / (w * C)) if series else 1 / complex(1 / R, w * C - 1 / (w * L))
        peak = abs(Y(o["f0"]))
        return (abs(Y(o["f0"]).imag) / peak + abs(abs(Y(o["f_low"])) / peak - 2 ** -0.5) + abs(abs(Y(o["f_high"])) / peak - 2 ** -0.5)
                + _rel(o["bandwidth"], o["f_high"] - o["f_low"]) + _rel(o["Q"], o["f0"] / o["bandwidth"]))
    return verify


def _reference_res(series):
    def reference(R, L, C):
        """INDEPENDENT reference: the response (series: admittance Cs/(LCs^2 + RCs + 1); parallel: impedance
        Ls/(RLCs^2... ) via scipy.signal.freqs; the resonance where its phase is zero and the half-power frequencies
        found by scipy brentq; Q = f0/bandwidth."""
        import numpy as np
        from scipy import optimize, signal
        b, a = ([C, 0.0], [L * C, R * C, 1.0]) if series else ([R * L, 0.0], [R * L * C, L, R])
        H = lambda f: complex(signal.freqs(b, a, worN=[2 * np.pi * f])[1][0])                # noqa: E731
        g = 1 / (2 * np.pi * np.sqrt(L * C))
        kw = dict(xtol=1e-300, rtol=8.9e-16, maxiter=500)
        f0 = optimize.brentq(lambda f: np.angle(H(f)), g * 0.3, g * 3, **kw)
        pk = abs(H(f0))
        lo = optimize.brentq(lambda f: abs(H(f)) - pk / np.sqrt(2), g * 1e-6, f0, **kw)
        hi = optimize.brentq(lambda f: abs(H(f)) - pk / np.sqrt(2), f0, g * 1e6, **kw)
        return {"f0": float(f0), "f_low": float(lo), "f_high": float(hi), "bandwidth": float(hi - lo), "Q": float(f0 / (hi - lo))}
    return reference


for _key, _series, _word in (("series_rlc_resonance", True, "series"), ("parallel_rlc_resonance", False, "parallel")):
    PROCEDURES[_key] = _fspec(
        f"Resonance, Q factor and bandwidth of a {_word} RLC circuit", "AC circuits / resonance",
        f"The resonant frequency, quality factor, half-power bandwidth and half-power frequencies of a {_word} RLC circuit.",
        [_inp("R", "resistance", "resistance"), _inp("L", "inductance", "inductance"), _inp("C", "capacitance", "capacitance")],
        [_out("f0", "frequency", "resonant frequency"), _out("Q", "number", "quality factor"), _out("bandwidth", "frequency", "half-power bandwidth"),
         _out("f_low", "frequency", "lower half-power frequency"), _out("f_high", "frequency", "upper half-power frequency")],
        "Q", ["ideal R, L, C"], ("Q = (1/R) sqrt(L/C), bandwidth = R/(2 pi L)" if _series else "Q = R sqrt(C/L), bandwidth = 1/(2 pi R C)"),
        (r"f_0 = \frac{1}{2\pi\sqrt{LC}},\ Q = \frac{1}{R}\sqrt{\frac{L}{C}}" if _series else r"f_0 = \frac{1}{2\pi\sqrt{LC}},\ Q = R\sqrt{\frac{C}{L}}"),
        (lambda R, L, C, s=_series: _resonance(R, L, C, s)), _verify_res(_series), _reference_res(_series),
        (lambda R, L, C, s=_series: _resonance(R, L, C, not s)),
        lambda rng: {"R": _e12(rng, 0, 3), "L": _e12(rng, -4, -1), "C": _e12(rng, -9, -6)},
        rf"(?=.*\b{_word} rlc\b)(?=.*\b(?:resonan|q factor|quality factor|bandwidth))",
        [f"Find the resonance and bandwidth of a {_word} RLC circuit with R = 10 ohm, L = 10 mH, C = 1 uF."],
        [{"inputs": {"R": 10, "L": 0.01, "C": 1e-6}, "outputs": {"f0": 1 / (2 * math.pi * 1e-4),
                                                              "Q": (10 if _series else 0.1), "bandwidth": (10 / (2 * math.pi * 0.01) if _series else 1 / (2 * math.pi * 1e-5))}},
         {"inputs": {"R": 1000, "L": 0.001, "C": 1e-9}, "outputs": {"Q": (1 / 1000 * 1000 if _series else 1000 * 1e-3)}}],
        [{"q": f"Find the resonance and bandwidth of a {_word} RLC circuit with R = 10 ohm, L = 10 mH, C = 1 uF.",
          "outputs": {"Q": 10.0 if _series else 0.1}},
         {"q": f"What is the Q factor of a {_word} RLC circuit with R = 10 ohm, L = 10 mH, C = 0 uF?", "outcome": "INVALID_INPUT"}],
        forbid=r"\bimpedance\b|\bstep\b")


def first_order_transient(V0, Vf, tau, t) -> dict:
    _check_positive(tau=tau)
    if t < 0:
        raise ValueError("the time must be zero or positive")
    e = math.exp(-t / tau)
    return {"v": Vf + (V0 - Vf) * e, "fraction_complete": 1 - e}


def _reference_fot(V0, Vf, tau, t):
    """INDEPENDENT reference: scipy.integrate.solve_ivp of tau dv/dt = Vf - v from v(0) = V0 (DOP853, rtol 1e-13)."""
    from scipy import integrate
    if t == 0:
        return {"v": float(V0)}
    sol = integrate.solve_ivp(lambda tt, y: [(Vf - y[0]) / tau], (0, t), [V0], method="DOP853", rtol=1e-13, atol=1e-14 * max(abs(V0), abs(Vf), 1.0))
    return {"v": float(sol.y[0][-1])}


PROCEDURES["first_order_transient"] = _fspec(
    "First-order (RC or RL) transient response", "transients / first-order circuits",
    "The value at time t of any voltage in a first-order circuit going from its initial value V0 toward its final value Vf with time constant tau.",
    [_inp("V0", "voltage", "initial value (at t = 0+)"), _inp("Vf", "voltage", "final (steady-state) value"), _inp("tau", "time", "time constant (RC or L/R)"),
     _inp("t", "time", "time after the switching")],
    [_out("v", "voltage", "value at time t"), _out("fraction_complete", "number", "fraction of the change completed, 1 - exp(-t/tau)")],
    "v", ["a single time constant", "a step change at t = 0"], "v(t) = Vf + (V0 - Vf) exp(-t/tau)", r"v(t) = V_f + (V_0 - V_f)e^{-t/\tau}",
    first_order_transient,
    lambda i, o: abs(o["v"] - (i["Vf"] + (i["V0"] - i["Vf"]) * (1 - o["fraction_complete"]))) / max(abs(i["V0"]), abs(i["Vf"]), 1e-300)
    + abs(math.log(max(1e-300, 1 - o["fraction_complete"])) + i["t"] / i["tau"]) / max(1.0, i["t"] / i["tau"]),
    _reference_fot, lambda V0, Vf, tau, t: {"v": V0 + (Vf - V0) * math.exp(-t / tau), "fraction_complete": 1 - math.exp(-t / tau)},
    lambda rng: (lambda tau: {"V0": round(rng.uniform(-10, 10), 2), "Vf": round(rng.uniform(-15, 15), 2), "tau": tau, "t": round(tau * rng.uniform(0.1, 4), 15)})(_e12(rng, -6, -1)),
    r"\b(transient|first[- ]order)\b",
    ["Find the first-order transient value with V0 = 0 V, Vf = 10 V, tau = 1 ms, t = 2 ms."],
    [{"inputs": {"V0": 0, "Vf": 10, "tau": 1e-3, "t": 2e-3}, "outputs": {"v": 10 * (1 - math.exp(-2))}},
     {"inputs": {"V0": 5, "Vf": 0, "tau": 2, "t": 2}, "outputs": {"v": 5 * math.exp(-1)}}],
    [{"q": "Find the first-order transient value with V0 = 0 V, Vf = 10 V, tau = 1 ms, t = 2 ms.", "outputs": {"v": 10 * (1 - math.exp(-2))}},
     {"q": "Find the first-order transient value with V0 = 0 V, Vf = 10 V, tau = 0 ms, t = 2 ms.", "outcome": "INVALID_INPUT"}])


# --------------------------------------------------------------------------------------------- op-amp stages
PROCEDURES["inverting_amplifier"] = _fspec(
    "Inverting amplifier gain and output", "operational amplifiers / basic stages", "The voltage gain -Rf/Rin of an ideal inverting amplifier and its output for an input voltage.",
    [_inp("Rin", "resistance", "input resistor"), _inp("Rf", "resistance", "feedback resistor"), _inp("Vin", "voltage", "input voltage")],
    [_out("gain", "number", "closed-loop voltage gain"), _out("Vout", "voltage", "output voltage")],
    "Vout", ["ideal op-amp, no saturation"], "gain = -Rf/Rin", r"A_v = -\frac{R_f}{R_{in}}",
    lambda Rin, Rf, Vin: (_check_positive(Rin=Rin, Rf=Rf), {"gain": -Rf / Rin, "Vout": -Rf / Rin * Vin})[1],
    lambda i, o: abs(i["Vin"] / i["Rin"] + o["Vout"] / i["Rf"]) / max(abs(i["Vin"] / i["Rin"]), 1e-300) + _rel(o["gain"], -i["Rf"] / i["Rin"]),
    lambda Rin, Rf, Vin: {"Vout": float(_ideal_opamp_mna([("in", "n", Rin), ("n", "out", Rf)], [("in", Vin)], "n", "0", "out")["out"])},
    lambda Rin, Rf, Vin: {"gain": -Rf / Rin - 1, "Vout": -(1 + Rf / Rin) * Vin},
    lambda rng: {"Rin": _e12(rng, 2, 5), "Rf": _e12(rng, 2, 6), "Vin": round(rng.uniform(-2, 2), 3) or 0.5},
    r"\binverting amplifier\b", ["Find the output of an inverting amplifier with Rin = 10 kohm, Rf = 47 kohm, Vin = 0.5 V."],
    [{"inputs": {"Rin": 1e4, "Rf": 4.7e4, "Vin": 0.5}, "outputs": {"gain": -4.7, "Vout": -2.35}}, {"inputs": {"Rin": 1e3, "Rf": 1e4, "Vin": -0.2}, "outputs": {"Vout": 2.0}}],
    [{"q": "Find the output of an inverting amplifier with Rin = 10 kohm, Rf = 47 kohm, Vin = 0.5 V.", "outputs": {"Vout": -2.35}},
     {"q": "Find the output of an inverting amplifier with Rin = 10 kohm, Rf = 47 kohm.", "outcome": "NEEDS_INPUT"}],
    forbid=r"\bnon[- ]?inverting\b|\bsumming\b")

PROCEDURES["noninverting_amplifier"] = _fspec(
    "Non-inverting amplifier gain and output", "operational amplifiers / basic stages", "The voltage gain 1 + Rf/Rg of an ideal non-inverting amplifier and its output.",
    [_inp("Rg", "resistance", "resistor from the inverting input to ground"), _inp("Rf", "resistance", "feedback resistor"), _inp("Vin", "voltage", "input voltage (at the non-inverting input)")],
    [_out("gain", "number", "closed-loop voltage gain"), _out("Vout", "voltage", "output voltage")],
    "Vout", ["ideal op-amp, no saturation"], "gain = 1 + Rf/Rg", r"A_v = 1 + \frac{R_f}{R_g}",
    lambda Rg, Rf, Vin: (_check_positive(Rg=Rg, Rf=Rf), {"gain": 1 + Rf / Rg, "Vout": (1 + Rf / Rg) * Vin})[1],
    lambda i, o: abs(o["Vout"] * i["Rg"] / (i["Rg"] + i["Rf"]) - i["Vin"]) / max(abs(i["Vin"]), 1e-300) + _rel(o["gain"], 1 + i["Rf"] / i["Rg"]),
    lambda Rg, Rf, Vin: {"Vout": float(_ideal_opamp_mna([("n", "0", Rg), ("n", "out", Rf), ("in", "p", 1.0)], [("in", Vin)], "n", "p", "out")["out"])},
    lambda Rg, Rf, Vin: {"gain": Rf / Rg, "Vout": Rf / Rg * Vin},
    lambda rng: {"Rg": _e12(rng, 2, 5), "Rf": _e12(rng, 2, 6), "Vin": round(rng.uniform(-2, 2), 3) or 0.5},
    r"\bnon[- ]?inverting amplifier\b", ["Find the output of a non-inverting amplifier with Rg = 10 kohm, Rf = 90 kohm, Vin = 0.2 V."],
    [{"inputs": {"Rg": 1e4, "Rf": 9e4, "Vin": 0.2}, "outputs": {"gain": 10.0, "Vout": 2.0}}, {"inputs": {"Rg": 1e3, "Rf": 1e3, "Vin": 1}, "outputs": {"Vout": 2.0}}],
    [{"q": "Find the output of a non-inverting amplifier with Rg = 10 kohm, Rf = 90 kohm, Vin = 0.2 V.", "outputs": {"Vout": 2.0}},
     {"q": "Find the output of a non-inverting amplifier with Rg = 0 kohm, Rf = 90 kohm, Vin = 0.2 V.", "outcome": "INVALID_INPUT"}])


def opamp_integrator(R, C, Vin, t, V0) -> dict:
    _check_positive(R=R, C=C)
    if t < 0:
        raise ValueError("the time must be zero or positive")
    return {"Vout": V0 - Vin * t / (R * C), "slope": -Vin / (R * C)}


def _reference_integrator(R, C, Vin, t, V0):
    """INDEPENDENT reference: the capacitor's charging current Vin/R (virtual ground) integrated by
    scipy.integrate.solve_ivp: d(vC)/dt = (Vin/R)/C, with Vout = -vC + V0."""
    from scipy import integrate
    if t == 0:
        return {"Vout": float(V0)}
    sol = integrate.solve_ivp(lambda tt, y: [Vin / R / C], (0, t), [0.0], method="DOP853", rtol=1e-13, atol=1e-15)
    return {"Vout": float(V0 - sol.y[0][-1])}


PROCEDURES["opamp_integrator"] = _fspec(
    "Op-amp integrator output for a constant input", "operational amplifiers / integrator", "The output of an ideal inverting integrator (R in, C feedback) after a constant input Vin has been applied for time t, from an initial output V0.",
    [_inp("R", "resistance", "input resistor"), _inp("C", "capacitance", "feedback capacitor"), _inp("Vin", "voltage", "constant input voltage"),
     _inp("t", "time", "integration time"), _inp("V0", "voltage", "output voltage at t = 0")],
    [_out("Vout", "voltage", "output voltage at t"), _out("slope", "number", "output slope dVout/dt in V/s")],
    "Vout", ["ideal op-amp, no saturation", "constant input"], "Vout = V0 - Vin t/(RC)", r"V_{out}(t) = V_0 - \frac{1}{RC}\int_0^t V_{in}\,dt",
    opamp_integrator,
    lambda i, o: abs(o["Vout"] - i["V0"] - o["slope"] * i["t"]) / max(abs(i["V0"]), abs(o["Vout"]), 1e-300) + _rel(o["slope"] * i["R"] * i["C"], -i["Vin"]),
    _reference_integrator, lambda R, C, Vin, t, V0: {"Vout": V0 + Vin * t / (R * C), "slope": Vin / (R * C)},
    lambda rng: (lambda R, C: {"R": R, "C": C, "Vin": round(rng.uniform(-2, 2), 2) or 1.0, "t": round(R * C * rng.uniform(0.1, 3), 12),
                               "V0": round(rng.uniform(-1, 1), 2)})(_e12(rng, 3, 5), _e12(rng, -8, -6)),
    r"\bintegrator\b", ["Find the output of an op-amp integrator with R = 10 kohm, C = 1 uF, Vin = 0.5 V, t = 5 ms, V0 = 0 V."],
    [{"inputs": {"R": 1e4, "C": 1e-6, "Vin": 0.5, "t": 0.005, "V0": 0}, "outputs": {"Vout": -0.25, "slope": -50.0}},
     {"inputs": {"R": 1e5, "C": 1e-7, "Vin": -1, "t": 0.02, "V0": 1}, "outputs": {"Vout": 3.0}}],
    [{"q": "Find the output of an op-amp integrator with R = 10 kohm, C = 1 uF, Vin = 0.5 V, t = 5 ms, V0 = 0 V.", "outputs": {"Vout": -0.25}},
     {"q": "Find the output of an op-amp integrator with R = 10 kohm, C = 1 uF, Vin = 0.5 V, t = 5 ms.", "outcome": "NEEDS_INPUT"}])


# --------------------------------------------------------------------------------------------- transformer
def transformer(Np, Ns, Vp, RL) -> dict:
    _check_positive(Np=Np, Ns=Ns, RL=RL)
    vs = Vp * Ns / Np
    i_s = vs / RL
    return {"Vs": vs, "Is": i_s, "Ip": i_s * Ns / Np, "Rin": RL * (Np / Ns) ** 2, "turns_ratio": Np / Ns}


def _reference_transformer(Np, Ns, Vp, RL):
    """INDEPENDENT reference: the ideal transformer as two constraint equations (v2 = (Ns/Np) v1, Np i1 = Ns i2) in a
    nodal system with the source and the load, solved by numpy.linalg."""
    import numpy as np
    n = Ns / Np
    # unknowns: v1, v2, i1 (into the primary), i2 (out of the secondary into the load)
    A = np.array([[1, 0, 0, 0], [-n, 1, 0, 0], [0, 0, Np, -Ns], [0, 1, 0, -RL]], float)
    z = np.array([Vp, 0, 0, 0], float)
    v1, v2, i1, i2 = np.linalg.solve(A, z)
    return {"Vs": float(v2), "Is": float(i2), "Ip": float(i1), "Rin": float(v1 / i1)}


PROCEDURES["transformer"] = _fspec(
    "Ideal transformer: voltages, currents and reflected resistance", "AC circuits / transformers",
    "Secondary voltage and current, primary current and the load resistance seen from the primary for an ideal transformer with a resistive load.",
    [_inp("Np", "number", "primary turns"), _inp("Ns", "number", "secondary turns"), _inp("Vp", "voltage", "primary (rms) voltage"),
     _inp("RL", "resistance", "load resistance on the secondary")],
    [_out("Vs", "voltage", "secondary voltage"), _out("Is", "current", "secondary (load) current"), _out("Ip", "current", "primary current"),
     _out("Rin", "resistance", "load resistance reflected to the primary"), _out("turns_ratio", "number", "Np/Ns")],
    "Vs", ["ideal transformer (no losses, no magnetizing current)"], "Vs = Vp Ns/Np, Ip = Is Ns/Np, Rin = RL (Np/Ns)^2",
    r"\frac{V_s}{V_p} = \frac{N_s}{N_p},\quad R_{in} = R_L\left(\frac{N_p}{N_s}\right)^2",
    transformer,
    lambda i, o: (abs(o["Vs"] * i["Np"] - i["Vp"] * i["Ns"]) / max(abs(i["Vp"] * i["Ns"]), 1e-300) + abs(o["Is"] * i["RL"] - o["Vs"]) / max(abs(o["Vs"]), 1e-300)
                  + abs(i["Vp"] * o["Ip"] - o["Vs"] * o["Is"]) / max(abs(o["Vs"] * o["Is"]), 1e-300) + _rel(o["Rin"] * o["Ip"], i["Vp"])),
    _reference_transformer, lambda Np, Ns, Vp, RL: (lambda vs: {"Vs": vs, "Is": vs / RL, "Ip": vs / RL * Np / Ns, "Rin": RL * (Ns / Np) ** 2})(Vp * Np / Ns),
    lambda rng: {"Np": float(rng.randint(20, 2000)), "Ns": float(rng.randint(10, 1000)), "Vp": rng.choice([12.0, 120.0, 230.0, 2400.0]), "RL": _e12(rng, 0, 3)},
    r"\btransformer\b", ["Find the secondary voltage of a transformer with Np = 500, Ns = 50, Vp = 230 V, RL = 10 ohm."],
    [{"inputs": {"Np": 500, "Ns": 50, "Vp": 230, "RL": 10}, "outputs": {"Vs": 23.0, "Is": 2.3, "Ip": 0.23, "Rin": 1000.0}},
     {"inputs": {"Np": 100, "Ns": 200, "Vp": 12, "RL": 48}, "outputs": {"Vs": 24.0, "Rin": 12.0}}],
    [{"q": "Find the secondary voltage of a transformer with Np = 500, Ns = 50, Vp = 230 V, RL = 10 ohm.", "outputs": {"Vs": 23.0}},
     {"q": "Find the secondary voltage of a transformer with Np = 500, Ns = 0, Vp = 230 V, RL = 10 ohm.", "outcome": "INVALID_INPUT"}])


# --------------------------------------------------------------------------------------------- semiconductor devices
def mosfet_saturation(kn, WL, VGS, VTH, VDS) -> dict:
    _check_positive(kn=kn, WL=WL)
    vov = VGS - VTH
    if vov <= 0:
        raise ValueError("VGS does not exceed the threshold voltage: the transistor is off")
    if VDS < vov:
        raise ValueError("VDS < VGS - VTH: the transistor is in the triode region, not saturation")
    idd = 0.5 * kn * WL * vov * vov
    return {"ID": idd, "ID_mA": idd * 1e3, "VOV": vov, "gm": 2 * idd / vov}


def _reference_mosfet(kn, WL, VGS, VTH, VDS):
    """INDEPENDENT reference: the gradual-channel model, ID = kn (W/L) * integral of (VGS - VTH - V) dV from 0 to the
    pinch-off voltage VGS - VTH (scipy.integrate.quad); gm by differentiating the integral numerically is avoided:
    gm = d ID/d VGS from the integral's closed derivative kn (W/L) (VGS - VTH) by quad of 1 over the same range."""
    from scipy import integrate
    vov = VGS - VTH
    i, _ = integrate.quad(lambda v: kn * WL * (vov - v), 0, vov, epsabs=0, epsrel=1e-13)
    g, _ = integrate.quad(lambda v: kn * WL, 0, vov, epsabs=0, epsrel=1e-13)
    return {"ID": float(i), "ID_mA": float(i * 1e3), "gm": float(g)}


PROCEDURES["mosfet_saturation"] = _fspec(
    "MOSFET drain current in saturation (square law)", "diodes and transistors / MOSFET", "Drain current, overdrive voltage and transconductance of an n-channel MOSFET in saturation.",
    [_inp("kn", "number", "process transconductance parameter mu_n Cox in A/V^2", units="transconductance"), _inp("WL", "number", "aspect ratio W/L"),
     _inp("VGS", "voltage", "gate-source voltage"), _inp("VTH", "voltage", "threshold voltage"), _inp("VDS", "voltage", "drain-source voltage")],
    [_out("ID", "current", "drain current"), _out("ID_mA", "number", "drain current in milliamperes"), _out("VOV", "voltage", "overdrive VGS - VTH"),
     _out("gm", "number", "transconductance in A/V (siemens)")],
    "ID_mA", ["saturation: VGS > VTH and VDS >= VGS - VTH", "no channel-length modulation"], "ID = (1/2) kn (W/L) (VGS - VTH)^2",
    r"I_D = \tfrac{1}{2}k_n'\frac{W}{L}(V_{GS} - V_{TH})^2",
    mosfet_saturation,
    lambda i, o: (_rel(o["ID"], 0.5 * i["kn"] * i["WL"] * o["VOV"] ** 2) + abs(o["VOV"] - (i["VGS"] - i["VTH"])) / abs(i["VGS"])
                  + _rel(o["gm"] * o["VOV"], 2 * o["ID"]) + _rel(o["ID_mA"], o["ID"] * 1e3)),
    _reference_mosfet, lambda kn, WL, VGS, VTH, VDS: (lambda r: dict(r, ID=2 * r["ID"], ID_mA=2 * r["ID_mA"]))(mosfet_saturation(kn, WL, VGS, VTH, VDS)),
    lambda rng: (lambda vth, vgs: {"kn": rng.choice([50e-6, 100e-6, 200e-6, 400e-6]), "WL": float(rng.randint(1, 100)), "VGS": vgs, "VTH": vth,
                                   "VDS": round(vgs - vth + rng.uniform(0.1, 5), 2)})(round(rng.uniform(0.3, 1.2), 2), round(rng.uniform(1.4, 5), 2)),
    r"\bmosfet\b",
    ["Find the drain current of a MOSFET with kn = 200 uA/V^2, WL = 10, VGS = 2 V, VTH = 0.7 V, VDS = 5 V."],
    [{"inputs": {"kn": 2e-4, "WL": 10, "VGS": 2, "VTH": 0.7, "VDS": 5}, "outputs": {"ID": 0.5 * 2e-3 * 1.3 ** 2, "gm": 2e-3 * 1.3}},
     {"inputs": {"kn": 1e-4, "WL": 20, "VGS": 1.5, "VTH": 0.5, "VDS": 1}, "outputs": {"ID_mA": 1.0}}],
    [{"q": "Find the drain current of a MOSFET with kn = 200 uA/V^2, WL = 10, VGS = 2 V, VTH = 0.7 V, VDS = 5 V.", "outputs": {"ID": 0.5 * 2e-3 * 1.3 ** 2}},
     {"q": "Find the drain current of a MOSFET with kn = 200 uA/V^2, WL = 10, VGS = 2 V, VTH = 0.7 V, VDS = 0.5 V.", "outcome": "INVALID_INPUT"},
     {"q": "Find the drain current of a MOSFET with kn = 200 uA/V^2, WL = 10, VGS = 0.5 V, VTH = 0.7 V, VDS = 5 V.", "outcome": "INVALID_INPUT"}])


def diode_circuit(Vs, R, Is, n, VT) -> dict:
    _check_positive(Vs=Vs, R=R, Is=Is, n=n, VT=VT)
    f = lambda i: Vs - i * R - n * VT * math.log1p(i / Is)                    # noqa: E731  decreasing in i
    lo, hi = 0.0, Vs / R
    for _ in range(400):
        mid = 0.5 * (lo + hi)
        if mid in (lo, hi):
            break
        lo, hi = (mid, hi) if f(mid) > 0 else (lo, mid)
    i = 0.5 * (lo + hi)
    for _ in range(3):                                                         # Newton polish
        d = -R - n * VT / (Is + i)
        i -= f(i) / d
    return {"I": i, "I_mA": i * 1e3, "VD": n * VT * math.log1p(i / Is), "VR": i * R}


def _reference_diode(Vs, R, Is, n, VT):
    """INDEPENDENT reference: the closed-form solution of the Shockley diode and resistor in series by the Wright
    omega function (scipy.special.wrightomega, W(e^z)): I = (n VT/R) omega(ln(Is R/(n VT)) + (Vs + Is R)/(n VT)) - Is."""
    import numpy as np
    from scipy import special
    nvt = n * VT
    i = nvt / R * float(np.real(special.wrightomega(np.log(Is * R / nvt) + (Vs + Is * R) / nvt))) - Is
    return {"I": i, "I_mA": i * 1e3, "VD": Vs - i * R}


PROCEDURES["diode_circuit"] = _fspec(
    "Operating point of a diode in series with a resistor (Shockley equation)", "diodes and transistors / diodes",
    "The current and the diode voltage of a forward-biased diode I = Is (exp(VD/(n VT)) - 1) in series with a resistor across a DC source.",
    [_inp("Vs", "voltage", "source voltage (forward)"), _inp("R", "resistance", "series resistor"), _inp("Is", "current", "saturation current"),
     _inp("n", "number", "ideality factor"), _inp("VT", "voltage", "thermal voltage kT/q (about 25.85 mV at 300 K)")],
    [_out("I", "current", "circuit current"), _out("I_mA", "number", "current in milliamperes"), _out("VD", "voltage", "diode voltage"),
     _out("VR", "voltage", "resistor voltage")],
    "I_mA", ["forward bias (Vs > 0)", "the Shockley model without series resistance or breakdown"],
    "bisection then Newton on Vs = I R + n VT ln(1 + I/Is)", r"V_s = IR + nV_T\ln\left(1 + \frac{I}{I_s}\right)",
    diode_circuit,
    lambda i, o: (abs(i["Vs"] - o["I"] * i["R"] - o["VD"]) / i["Vs"] + abs(o["I"] - i["Is"] * math.expm1(o["VD"] / (i["n"] * i["VT"]))) / max(o["I"], 1e-300)
                  + _rel(o["I_mA"], o["I"] * 1e3)),
    _reference_diode, lambda Vs, R, Is, n, VT: (lambda i: {"I": i, "I_mA": i * 1e3, "VD": 0.7, "VR": Vs - 0.7})((Vs - 0.7) / R),
    lambda rng: {"Vs": round(rng.uniform(1.5, 20), 2), "R": _e12(rng, 2, 4), "Is": rng.choice([1e-14, 1e-13, 1e-12, 1e-9]), "n": rng.choice([1.0, 1.5, 2.0]), "VT": 0.025852},
    r"\bdiode\b", ["Find the current of a diode circuit with Vs = 5 V, R = 1 kohm, Is = 1e-14 A, n = 1, VT = 25.85 mV."],
    [{"inputs": {"Vs": 5, "R": 1000, "Is": 1e-14, "n": 1, "VT": 0.02585}, "outputs": {}},
     {"inputs": {"Vs": 10, "R": 10000, "Is": 1e-12, "n": 2, "VT": 0.02585}, "outputs": {}}],
    [{"q": "Find the current of a diode circuit with Vs = 5 V, R = 1 kohm, Is = 1e-14 A, n = 1, VT = 25.85 mV.", "outputs": {}},
     {"q": "Find the current of a diode circuit with Vs = 5 V, R = 1 kohm, n = 1, VT = 25.85 mV.", "outcome": "NEEDS_INPUT"}],
    forbid=r"\bzener\b")


# --------------------------------------------------------------------------------------------- signals and conversion
_SHAPES = {"sine": 0.5, "square": 1.0, "triangle": 1 / 3, "sawtooth": 1 / 3}


def rms_waveform(shape, amplitude, offset=0.0) -> dict:
    if amplitude < 0:
        raise ValueError("the amplitude must not be negative")
    rms = math.sqrt(offset * offset + _SHAPES[shape] * amplitude * amplitude)
    return {"rms": rms, "average": offset, "ac_rms": amplitude * math.sqrt(_SHAPES[shape]),
            "crest_factor": (abs(offset) + amplitude) / rms if rms else 0.0}


def _wave(shape, A, off, x):                             # x in [0, 1): one period
    if shape == "sine":
        return off + A * math.sin(2 * math.pi * x)
    if shape == "square":
        return off + (A if x < 0.5 else -A)
    if shape == "triangle":
        return off + A * (4 * x - 1 if x < 0.5 else 3 - 4 * x)
    return off + A * (2 * x - 1)


def _reference_rms(shape, amplitude, offset=0.0):
    """INDEPENDENT reference: the mean of v(t)^2 over one period by scipy.integrate.quad (split at the waveform's
    corners), and the mean of v(t)."""
    from scipy import integrate
    pts = [0.0, 0.25, 0.5, 0.75, 1.0]
    sq = sum(integrate.quad(lambda x: _wave(shape, amplitude, offset, x) ** 2, a, b, epsabs=0, epsrel=1e-13)[0] for a, b in zip(pts, pts[1:]))
    av = sum(integrate.quad(lambda x: _wave(shape, amplitude, offset, x), a, b, epsabs=0, epsrel=1e-13)[0] for a, b in zip(pts, pts[1:]))
    return {"rms": sq ** 0.5, "average": 0.0 if abs(av) <= 1e-12 * max(amplitude, abs(offset)) else av}   # quadrature residue of 0


def _parse_rms(question):
    m = re.search(r"\b(sine|sinusoidal|square|triangle|triangular|sawtooth)\s+(?:wave|waveform|voltage)\b", question, re.I)
    if not m or not re.search(r"\brms\b", question, re.I):
        return None
    shape = {"sinusoidal": "sine", "triangular": "triangle"}.get(m.group(1).lower(), m.group(1).lower())
    base = _kv_parse(r"\brms\b", PROCEDURES_RMS_IN[1:])(question)
    if base is None:
        return None
    return dict(base, shape=shape)


PROCEDURES_RMS_IN = [{"symbol": "shape", "type": "choice", "choices": ["sine", "square", "triangle", "sawtooth"], "required": True, "meaning": "waveform shape"},
                     _inp("amplitude", "voltage", "peak amplitude about the average"),
                     {"symbol": "offset", "kind": "voltage", "required": False, "meaning": "DC offset (average value); 0 when not given"}]
PROCEDURES["rms_waveform"] = {
    "name": "RMS and average value of a periodic waveform", "topic": "AC circuits / waveforms",
    "purpose": "The rms value (with DC offset), average, AC rms and crest factor of a sine, square, triangle or sawtooth wave.",
    "inputs": PROCEDURES_RMS_IN,
    "outputs": [_out("rms", "voltage", "rms value including the offset"), _out("average", "voltage", "average value"),
                _out("ac_rms", "voltage", "rms of the AC part"), _out("crest_factor", "number", "peak over rms")],
    "primary": "rms", "conditions": ["symmetric waveform of the stated shape about its offset"],
    "method": "Vrms = sqrt(offset^2 + k A^2) with k = 1/2 (sine), 1 (square), 1/3 (triangle, sawtooth)",
    "latex": r"V_{rms} = \sqrt{\frac{1}{T}\int_0^T v^2\,dt}",
    "run": rms_waveform,
    "verify": lambda i, o: (lambda off: _rel(o["rms"] ** 2, off ** 2 + _SHAPES[i["shape"]] * i["amplitude"] ** 2) + abs(o["average"] - off) / max(o["rms"], 1e-300)
                            + _rel(o["ac_rms"] ** 2 + off ** 2, o["rms"] ** 2))(i.get("offset", 0.0)),
    "parse": _parse_rms, "reference": _reference_rms,
    "wrong": lambda shape, amplitude, offset=0.0: (lambda r: {"rms": r, "average": offset, "ac_rms": amplitude / math.sqrt(2),
                                                              "crest_factor": (abs(offset) + amplitude) / r if r else 0.0})(math.sqrt(offset * offset + amplitude * amplitude / 2)),
    "random_inputs": lambda rng: {"shape": rng.choice(["square", "triangle", "sawtooth", "square", "sine"]), "amplitude": round(rng.uniform(0.5, 100), 2),
                                  "offset": round(rng.uniform(-20, 20), 2) if rng.random() < 0.5 else 0.0},
    "templates": ["Find the rms value of a square wave with amplitude = 5 V, offset = 0 V."],
    "examples": [{"inputs": {"shape": "sine", "amplitude": 10, "offset": 0}, "outputs": {"rms": 10 / math.sqrt(2)}},
                 {"inputs": {"shape": "square", "amplitude": 5, "offset": 0}, "outputs": {"rms": 5.0}},
                 {"inputs": {"shape": "triangle", "amplitude": 3, "offset": 4}, "outputs": {"rms": math.sqrt(16 + 3)}}],
    "questions": [{"q": "Find the rms value of a square wave with amplitude = 5 V, offset = 0 V.", "outputs": {"rms": 5.0}},
                  {"q": "Find the rms value of a triangle wave with amplitude = 3 V, offset = 4 V.", "outputs": {"rms": math.sqrt(19)}},
                  {"q": "Find the rms value of a sine wave with amplitude = 10 V.", "outputs": {"rms": 10 / math.sqrt(2)}}]}


def adc_conversion(Vin, Vref, bits) -> dict:
    n = int(bits)
    if not 1 <= n <= 32:
        raise ValueError("the resolution must be 1 to 32 bits")
    _check_positive(Vref=Vref)
    if not 0 <= Vin <= Vref:
        raise ValueError("the input must be between 0 and Vref")
    lsb = Vref / 2 ** n
    code = min(int(math.floor(Vin / lsb)), 2 ** n - 1)
    return {"code": float(code), "binary": format(code, f"0{n}b"), "lsb": lsb, "lsb_mV": lsb * 1e3, "quantized": code * lsb}


def _reference_adc(Vin, Vref, bits):
    """INDEPENDENT reference: numpy.digitize of the input against the 2^n - 1 decision levels k Vref/2^n."""
    import numpy as np
    n = int(bits)
    edges = np.arange(1, 2 ** n) * (Vref / 2 ** n) if n <= 20 else None
    code = int(np.digitize([Vin], edges)[0]) if edges is not None else min(int(np.floor(Vin * 2 ** n / Vref)), 2 ** n - 1)
    return {"code": float(code), "lsb": Vref / 2 ** n, "binary": np.binary_repr(code, width=n)}


PROCEDURES["adc_conversion"] = {
    "name": "Analog-to-digital conversion: output code and resolution", "topic": "digital logic / data conversion",
    "purpose": "The output code (truncating converter), its binary form, the LSB size and the quantized voltage of an ideal n-bit ADC with reference Vref.",
    "inputs": [_inp("Vin", "voltage", "input voltage (0 to Vref)"), _inp("Vref", "voltage", "full-scale reference voltage"),
               {"symbol": "bits", "type": "integer", "min": 1, "max": 32, "required": True, "meaning": "resolution in bits"}],
    "outputs": [_out("code", "number", "output code"), _out("binary", "text", "code in binary"), _out("lsb", "voltage", "LSB size Vref/2^n"),
                _out("lsb_mV", "number", "LSB in millivolts"), _out("quantized", "voltage", "voltage of the code, code x LSB")],
    "primary": "code", "conditions": ["0 <= Vin <= Vref", "truncating (floor) quantizer; Vref itself gives the top code"],
    "method": "code = floor(Vin / LSB), capped at 2^n - 1", "latex": r"\mathrm{code} = \left\lfloor \frac{V_{in}}{V_{ref}/2^n} \right\rfloor",
    "run": adc_conversion,
    "verify": lambda i, o: (float(not (o["code"] * o["lsb"] <= i["Vin"] * (1 + 1e-12) and (i["Vin"] < (o["code"] + 1) * o["lsb"] or o["code"] == 2 ** int(i["bits"]) - 1)))
                            + _rel(o["lsb"] * 2 ** int(i["bits"]), i["Vref"]) + float(int(o["binary"], 2) != o["code"]) + _rel(o["lsb_mV"], o["lsb"] * 1e3)),
    "parse": _kv_parse(r"\b(adc|analog[- ]to[- ]digital)\b", [_inp("Vin", "voltage", ""), _inp("Vref", "voltage", ""), {"symbol": "bits", "kind": "number"}]),
    "reference": _reference_adc,
    "wrong": lambda Vin, Vref, bits: (lambda lsb, c: {"code": float(c), "binary": format(c, f"0{int(bits)}b"), "lsb": lsb, "lsb_mV": lsb * 1e3,
                                                      "quantized": c * lsb})(Vref / (2 ** int(bits) - 1), min(round(Vin / (Vref / (2 ** int(bits) - 1))), 2 ** int(bits) - 1)),
    "random_inputs": lambda rng: (lambda vref: {"Vref": vref, "Vin": round(rng.uniform(0, vref), 4), "bits": rng.choice([4, 8, 10, 12])})(rng.choice([1.0, 2.5, 3.3, 5.0, 10.0])),
    "templates": ["Find the ADC output code for Vin = 1.2 V, Vref = 5 V, bits = 8."],
    "examples": [{"inputs": {"Vin": 1.2, "Vref": 5, "bits": 8}, "outputs": {"code": 61.0, "binary": "00111101"}},
                 {"inputs": {"Vin": 5, "Vref": 5, "bits": 4}, "outputs": {"code": 15.0, "lsb": 0.3125}}],
    "questions": [{"q": "Find the ADC output code for Vin = 1.2 V, Vref = 5 V, bits = 8.", "outputs": {"code": 61.0}},
                  {"q": "Find the ADC output code for Vin = 6 V, Vref = 5 V, bits = 8.", "outcome": "INVALID_INPUT"}]}


# --------------------------------------------------------------------------------------------- state space
def _charpoly_adj(A):
    """Faddeev-LeVerrier: the characteristic polynomial coefficients of A and the matrices N_k of
    adj(sI - A) = sum N_k s^(n-1-k)."""
    n = len(A)
    I = [[float(i == j) for j in range(n)] for i in range(n)]
    M = [row[:] for row in I]
    coeffs, Ns = [1.0], []
    for k in range(1, n + 1):
        Ns.append(M)
        AM = [[sum(A[i][t] * M[t][j] for t in range(n)) for j in range(n)] for i in range(n)]
        c = -sum(AM[i][i] for i in range(n)) / k
        coeffs.append(c)
        M = [[AM[i][j] + c * I[i][j] for j in range(n)] for i in range(n)]
    return coeffs, Ns


def _snap(cs):
    s = max((abs(c) for c in cs), default=0.0)
    return [0.0 if abs(c) <= 1e-12 * s else c for c in cs]


def state_space_to_tf(A, B, C, D) -> dict:
    n = len(A)
    if any(len(r) != n for r in A) or len(B) != n or len(C) != n:
        raise ValueError("A must be n x n and B, C must have n entries")
    den, Ns = _charpoly_adj(A)
    num = [D * d for d in den]
    for k, N in enumerate(Ns):
        num[k + 1] += sum(C[i] * N[i][j] * B[j] for i in range(n) for j in range(n))
    return {"num": _snap(num), "den": _snap(den), "order": float(n)}


def _verify_ss(inputs, outputs):
    """At two test points s, C (sI - A)^-1 B + D (solved by Gaussian elimination) must equal num(s)/den(s)."""
    A, B, C, D = inputs["A"], inputs["B"], inputs["C"], inputs["D"]
    n = len(A)
    worst = 0.0
    for s in (complex(0.37, 1.3), complex(-2.1, 0.7)):
        M = [[(s if i == j else 0) - A[i][j] for j in range(n)] for i in range(n)]
        try:
            x = _gauss(M, [complex(b) for b in B])
        except ValueError:
            continue
        g = sum(c * xi for c, xi in zip(C, x)) + D
        h = _pval(outputs["num"], s) / _pval(outputs["den"], s)
        worst = max(worst, abs(g - h) / max(abs(g), 1e-12))
    return worst + abs(outputs["den"][0] - 1.0) + abs(outputs["order"] - n) + float(len(outputs["num"]) != n + 1)


def _reference_ss(A, B, C, D):
    """INDEPENDENT reference: scipy.signal.ss2tf (characteristic polynomial from the eigenvalues; numerator from
    det(sI - A + B C))."""
    import numpy as np
    from scipy import signal
    num, den = signal.ss2tf(np.array(A, float), np.array(B, float).reshape(-1, 1), np.array(C, float).reshape(1, -1), np.array([[D]], float))
    return {"num": _snap([float(x) for x in num[0]]), "den": _snap([float(x) for x in den])}


PROCEDURES["state_space_to_tf"] = {
    "name": "Transfer function of a single-input single-output state-space model", "topic": "control systems / state space",
    "purpose": "G(s) = C (sI - A)^-1 B + D as numerator and denominator coefficients for x' = Ax + Bu, y = Cx + Du.",
    "inputs": [{"symbol": "A", "type": "matrix", "required": True, "meaning": "state matrix (n x n)"},
               {"symbol": "B", "type": "vector", "min_length": 1, "max_length": 10, "required": True, "meaning": "input vector (n entries)"},
               {"symbol": "C", "type": "vector", "min_length": 1, "max_length": 10, "required": True, "meaning": "output vector (n entries)"},
               {"symbol": "D", "kind": "number", "required": True, "meaning": "feedthrough"}],
    "outputs": [_out("num", "number", "numerator coefficients (highest power first, length n + 1)"),
                _out("den", "number", "denominator coefficients det(sI - A), monic"), _out("order", "number", "system order n")],
    "primary": "order", "conditions": ["a square A with B and C of matching size", "coefficients below 1e-12 of the largest are written as 0"],
    "method": "Faddeev-LeVerrier recursion for det(sI - A) and adj(sI - A)", "latex": r"G(s) = C(sI - A)^{-1}B + D",
    "run": state_space_to_tf, "verify": _verify_ss, "parse": None, "reference": _reference_ss,
    "wrong": lambda A, B, C, D: state_space_to_tf(A, [-b for b in B], C, D),           # the input vector's sign lost
    "random_inputs": lambda rng: (lambda n: {"A": [[float(rng.randint(-5, 5)) for _ in range(n)] for _ in range(n)],
                                             "B": [float(rng.randint(-3, 3)) or 1.0 for _ in range(n)], "C": [float(rng.randint(-3, 3)) or 1.0 for _ in range(n)],
                                             "D": float(rng.choice([0, 0, 1]))})(rng.randint(2, 4)),
    "templates": ["Find the transfer function of the state-space model A = [[0, 1], [-2, -3]], B = [0, 1], C = [1, 0], D = 0."],
    "examples": [{"inputs": {"A": "[[0, 1], [-2, -3]]", "B": "[0, 1]", "C": "[1, 0]", "D": 0}, "outputs": {"num": [0.0, 0.0, 1.0], "den": [1.0, 3.0, 2.0]}},
                 {"inputs": {"A": [[-1, 0], [0, -2]], "B": [1, 1], "C": [1, 1], "D": 1}, "outputs": {"num": [1.0, 5.0, 5.0], "den": [1.0, 3.0, 2.0]}}],
    "questions": [{"q": "Find the transfer function of the state-space model A = [[0, 1], [-2, -3]], B = [0, 1], C = [1, 0], D = 0.",
                   "outputs": {"den": [1.0, 3.0, 2.0]}},
                  {"q": "Find the transfer function of the state-space model A = [[0, 1], [-2, -3]], B = [0, 1, 2], C = [1, 0], D = 0.", "outcome": "INVALID_INPUT"}]}


def _parse_ss(question):
    if not re.search(r"\bstate[- ]space\b", question, re.I) or not re.search(r"\btransfer function\b", question, re.I):
        return None
    out = {}
    a = re.search(r"\bA\s*=\s*(\[\s*\[.*?\]\s*\])", question)
    for k in ("B", "C"):
        m = re.search(rf"\b{k}\s*=\s*(\[[^\[\]]*\])", question)
        if m:
            out[k] = m.group(1)
    d = re.search(rf"\bD\s*=\s*({_NUM_NB})", question)
    if a:
        out["A"] = a.group(1)
    if d:
        out["D"] = float(d.group(1))
    return out or None


PROCEDURES["state_space_to_tf"]["parse"] = _parse_ss


def _rank(M, tol=1e-9):
    R = [row[:] for row in M]
    rows, cols = len(R), len(R[0])
    scale = max(abs(x) for r in R for x in r) or 1.0
    rank, r = 0, 0
    for c in range(cols):
        p = max(range(r, rows), key=lambda i: abs(R[i][c]), default=None)
        if p is None or abs(R[p][c]) <= tol * scale:
            continue
        R[r], R[p] = R[p], R[r]
        for i in range(r + 1, rows):
            f = R[i][c] / R[r][c]
            R[i] = [x - f * y for x, y in zip(R[i], R[r])]
        r += 1
        rank += 1
        if r == rows:
            break
    return rank


def _ctrb_obsv(A, B, C):
    n = len(A)
    cols, v = [], list(B)
    for _ in range(n):
        cols.append(v)
        v = [sum(A[i][j] * v[j] for j in range(n)) for i in range(n)]
    rows, w = [], list(C)
    for _ in range(n):
        rows.append(w)
        w = [sum(w[i] * A[i][j] for i in range(n)) for j in range(n)]
    return [list(x) for x in zip(*cols)], rows


def controllability_observability(A, B, C) -> dict:
    n = len(A)
    if any(len(r) != n for r in A) or len(B) != n or len(C) != n:
        raise ValueError("A must be n x n and B, C must have n entries")
    Wc, Wo = _ctrb_obsv(A, B, C)
    rc, ro = _rank(Wc), _rank(Wo)
    return {"controllability_rank": float(rc), "observability_rank": float(ro), "controllable": rc == n, "observable": ro == n}


def _verify_co(inputs, outputs):
    """The ranks re-derived from the Gram determinants' signs is not robust; instead the rank is confirmed by an
    independent elimination on the transposed matrices (row rank = column rank)."""
    Wc, Wo = _ctrb_obsv(inputs["A"], inputs["B"], inputs["C"])
    n = len(inputs["A"])
    rc, ro = _rank([list(x) for x in zip(*Wc)]), _rank([list(x) for x in zip(*Wo)])
    return abs(outputs["controllability_rank"] - rc) + abs(outputs["observability_rank"] - ro) + (outputs["controllable"] != (rc == n)) + (outputs["observable"] != (ro == n))


def _reference_co(A, B, C):
    """INDEPENDENT reference: python-control's ctrb and obsv matrices and numpy.linalg.matrix_rank (singular values)."""
    import control
    import numpy as np
    A_, B_, C_ = np.array(A, float), np.array(B, float).reshape(-1, 1), np.array(C, float).reshape(1, -1)
    rc, ro = int(np.linalg.matrix_rank(control.ctrb(A_, B_))), int(np.linalg.matrix_rank(control.obsv(A_, C_)))
    return {"controllability_rank": float(rc), "observability_rank": float(ro), "controllable": rc == len(A), "observable": ro == len(A)}


def _random_co(rng):
    n = rng.randint(3, 4)
    A = [[float(rng.randint(-4, 4)) for _ in range(n)] for _ in range(n)]
    B = [float(rng.randint(-2, 2)) for _ in range(n)]
    C = [float(rng.randint(-2, 2)) for _ in range(n)]
    if rng.random() < 0.3:                                  # an uncontrollable or unobservable mode
        A = [[float(rng.randint(-4, 4)) if j < n - 1 else 0.0 for j in range(n)] for _ in range(n - 1)] + [[0.0] * (n - 1) + [-1.0]]
        B[-1] = 0.0
    if not any(B):
        B[0] = 1.0
    if not any(C):
        C[0] = 1.0
    return {"A": A, "B": B, "C": C}


def _parse_co(question):
    if not re.search(r"\b(controllab|observab)", question, re.I):
        return None
    out = {}
    a = re.search(r"\bA\s*=\s*(\[\s*\[.*?\]\s*\])", question)
    if a:
        out["A"] = a.group(1)
    for k in ("B", "C"):
        m = re.search(rf"\b{k}\s*=\s*(\[[^\[\]]*\])", question)
        if m:
            out[k] = m.group(1)
    return out or None


PROCEDURES["controllability_observability"] = {
    "name": "Controllability and observability of a single-input single-output system", "topic": "control systems / state space",
    "purpose": "The ranks of the controllability matrix [B AB ... A^(n-1)B] and the observability matrix [C; CA; ...; CA^(n-1)], and whether the system is controllable and observable.",
    "inputs": [{"symbol": "A", "type": "matrix", "required": True, "meaning": "state matrix (n x n)"},
               {"symbol": "B", "type": "vector", "min_length": 1, "max_length": 10, "required": True, "meaning": "input vector"},
               {"symbol": "C", "type": "vector", "min_length": 1, "max_length": 10, "required": True, "meaning": "output vector"}],
    "outputs": [_out("controllability_rank", "number", "rank of the controllability matrix"), _out("observability_rank", "number", "rank of the observability matrix"),
                _out("controllable", "boolean", "full controllability rank"), _out("observable", "boolean", "full observability rank")],
    "primary": "controllability_rank", "conditions": ["small integer or well-scaled entries (rank decided at a relative tolerance of 1e-9)"],
    "method": "Kalman rank tests by Gaussian elimination with partial pivoting", "latex": r"\mathcal{C} = [B\ AB\ \cdots\ A^{n-1}B],\ \mathcal{O} = [C;\ CA;\ \cdots;\ CA^{n-1}]",
    "run": controllability_observability, "verify": _verify_co, "parse": _parse_co, "reference": _reference_co,
    "wrong": lambda A, B, C: (lambda r: {"controllability_rank": float(min(2, r["controllability_rank"])), "observability_rank": float(min(2, r["observability_rank"])),
                                         "controllable": min(2, r["controllability_rank"]) == len(A), "observable": min(2, r["observability_rank"]) == len(A)})(
        controllability_observability(A, B, C)),
    "random_inputs": _random_co,
    "templates": ["Is the system with A = [[0, 1], [-2, -3]], B = [0, 1], C = [1, 0] controllable and observable?"],
    "examples": [{"inputs": {"A": "[[0, 1], [-2, -3]]", "B": "[0, 1]", "C": "[1, 0]"}, "outputs": {"controllable": True, "observable": True}},
                 {"inputs": {"A": [[-1, 0], [0, -2]], "B": [1, 0], "C": [1, 1]}, "outputs": {"controllability_rank": 1.0, "controllable": False, "observable": True}}],
    "questions": [{"q": "Is the system with A = [[0, 1], [-2, -3]], B = [0, 1], C = [1, 0] controllable and observable?", "outputs": {"controllable": True}},
                  {"q": "Is the system with A = [[-1, 0], [0, -2]], B = [1, 0], C = [1, 1] controllable?", "outputs": {"controllable": False}}]}


# ================================================================================================ further formula capabilities
_UNITS["angular_frequency"] = {"rad/s": 1.0, "krad/s": 1e3}
_UNITS["torque"] = {"N*m": 1.0, "N·m": 1.0, "Nm": 1.0, "mN*m": 1e-3, "mN·m": 1e-3}
def inductor_energy(L, I) -> dict:
    _check_positive(L=L)
    return {"energy": 0.5 * L * I * I, "flux_linkage": L * I}


def _reference_ind_energy(L, I):
    """INDEPENDENT reference: the work done by the source while the current ramps from 0 to I in 1 s, the integral of
    v i dt with v = L di/dt (scipy.integrate.quad)."""
    from scipy import integrate
    w, _ = integrate.quad(lambda t: (L * I) * (I * t), 0, 1, epsabs=0, epsrel=1e-13)
    return {"energy": float(w), "flux_linkage": L * I}


PROCEDURES["inductor_energy"] = _fspec(
    "Energy stored in an inductor", "AC circuits / energy storage", "The magnetic energy L I^2/2 and the flux linkage L I of an inductor carrying current I.",
    [_inp("L", "inductance", "inductance"), _inp("I", "current", "current")],
    [_out("energy", "energy", "stored energy"), _out("flux_linkage", "number", "flux linkage L I in weber-turns")],
    "energy", ["a linear inductor (no saturation)"], "W = L I^2 / 2", r"W = \tfrac{1}{2}LI^2",
    inductor_energy, lambda i, o: _rel(o["energy"], 0.5 * o["flux_linkage"] * i["I"]) + _rel(o["flux_linkage"], i["L"] * i["I"]),
    _reference_ind_energy, lambda L, I: {"energy": L * I * I, "flux_linkage": L * I},
    lambda rng: {"L": _e12(rng, -5, 0), "I": round(rng.uniform(0.01, 20), 3)},
    r"\binductor\b.*\benergy\b|\benergy\b.*\binductor\b",
    ["Find the energy stored in an inductor with L = 50 mH, I = 2 A."],
    [{"inputs": {"L": 0.05, "I": 2}, "outputs": {"energy": 0.1}}, {"inputs": {"L": 1e-3, "I": 10}, "outputs": {"energy": 0.05, "flux_linkage": 0.01}}],
    [{"q": "Find the energy stored in an inductor with L = 50 mH, I = 2 A.", "outputs": {"energy": 0.1}},
     {"q": "Find the energy stored in an inductor with L = 50 mH.", "outcome": "NEEDS_INPUT"}])


def first_order_time_to_value(V0, Vf, tau, v) -> dict:
    _check_positive(tau=tau)
    if V0 == Vf:
        raise ValueError("the initial and final values are equal: nothing changes")
    frac = (v - V0) / (Vf - V0)
    if not 0 <= frac < 1:
        raise ValueError("the value is never reached: it must lie between V0 and Vf (Vf itself only as t goes to infinity)")
    return {"t": -tau * math.log1p(-frac), "time_constants": -math.log1p(-frac)}


def _reference_ttv(V0, Vf, tau, v):
    """INDEPENDENT reference: the response integrated by scipy.integrate.solve_ivp with an event at v(t) = v."""
    from scipy import integrate
    if v == V0:
        return {"t": 0.0}
    ev = lambda t, y: y[0] - v                                                 # noqa: E731
    ev.terminal = True
    sol = integrate.solve_ivp(lambda t, y: [(Vf - y[0]) / tau], (0, 60 * tau), [V0], method="DOP853", rtol=1e-13,
                              atol=1e-14 * max(abs(V0), abs(Vf), 1.0), events=ev)
    return {"t": float(sol.t_events[0][0])}


PROCEDURES["first_order_time_to_value"] = _fspec(
    "Time for a first-order response to reach a value", "transients / first-order circuits",
    "How long a first-order (RC or RL) response from V0 toward Vf with time constant tau takes to reach a value v.",
    [_inp("V0", "voltage", "initial value"), _inp("Vf", "voltage", "final value"), _inp("tau", "time", "time constant"), _inp("v", "voltage", "value to reach")],
    [_out("t", "time", "time to reach v"), _out("time_constants", "number", "the same time in time constants")],
    "t", ["v between V0 and Vf"], "t = -tau ln(1 - (v - V0)/(Vf - V0))", r"t = \tau\ln\frac{V_f - V_0}{V_f - v}",
    first_order_time_to_value,
    lambda i, o: abs(i["Vf"] + (i["V0"] - i["Vf"]) * math.exp(-o["t"] / i["tau"]) - i["v"]) / max(abs(i["V0"]), abs(i["Vf"]), 1e-300) + _rel(o["time_constants"] * i["tau"], o["t"]),
    _reference_ttv, lambda V0, Vf, tau, v: {"t": tau * (v - V0) / (Vf - V0), "time_constants": (v - V0) / (Vf - V0)},
    lambda rng: (lambda v0, vf: {"V0": v0, "Vf": vf, "tau": _e12(rng, -6, 0), "v": round(v0 + (vf - v0) * rng.uniform(0.05, 0.97), 6)})(
        round(rng.uniform(-10, 10), 2), round(rng.uniform(-10, 10), 2) + 25.0),
    r"\b(how long|time to reach|time taken|when does)\b",
    ["How long does a first-order response with V0 = 0 V, Vf = 10 V, tau = 2 ms take to reach v = 6.32 V?"],
    [{"inputs": {"V0": 0, "Vf": 10, "tau": 0.002, "v": 5}, "outputs": {"t": 0.002 * math.log(2)}},
     {"inputs": {"V0": 12, "Vf": 0, "tau": 1, "v": 3}, "outputs": {"t": math.log(4)}}],
    [{"q": "How long does a first-order response with V0 = 0 V, Vf = 10 V, tau = 2 ms take to reach v = 5 V?", "outputs": {"t": 0.002 * math.log(2)}},
     {"q": "How long does a first-order response with V0 = 0 V, Vf = 10 V, tau = 2 ms take to reach v = 12 V?", "outcome": "INVALID_INPUT"}])


def _lead(z, p):
    _check_positive(z=z, p=p)
    if not p > z:
        raise ValueError("a lead compensator has its zero below its pole (p > z)")
    wm = math.sqrt(z * p)
    phi = math.degrees(math.asin((p - z) / (p + z)))
    # C(s) = (s + z)/(s + p) normalised to unit gain at high frequency: |C(j wm)| = sqrt(z/p)
    return {"phase_max_deg": phi, "w_max": wm, "gain_at_w_max_db": 10 * math.log10(z / p)}


def _reference_lead(z, p):
    """INDEPENDENT reference: the phase of (s + z)/(s + p) from scipy.signal.freqs, its maximum located by scipy brentq
    on the analytic derivative of the phase, d/dw [atan(w/z) - atan(w/p)] = 0."""
    import numpy as np
    from scipy import optimize, signal
    dphi = lambda w: z / (z * z + w * w) - p / (p * p + w * w)                # noqa: E731
    w = optimize.brentq(dphi, z, p, xtol=1e-300, rtol=8.9e-16, maxiter=500)
    h = complex(signal.freqs([1.0, z], [1.0, p], worN=[w])[1][0])
    return {"phase_max_deg": float(np.angle(h, deg=True)), "w_max": float(w), "gain_at_w_max_db": float(20 * np.log10(abs(h)))}


PROCEDURES["lead_compensator"] = _fspec(
    "Maximum phase lead of a lead compensator", "control systems / compensation",
    "For C(s) = (s + z)/(s + p) with p > z: the maximum phase lead, the frequency where it occurs, and the gain there.",
    [_inp("z", "angular_frequency", "zero (corner) frequency"), _inp("p", "angular_frequency", "pole (corner) frequency, p > z")],
    [_out("phase_max_deg", "number", "maximum phase lead in degrees"), _out("w_max", "angular_frequency", "frequency of maximum phase, sqrt(z p)"),
     _out("gain_at_w_max_db", "number", "gain of (s+z)/(s+p) there, in dB")],
    "phase_max_deg", ["p > z > 0"], "phi_max = asin((p - z)/(p + z)) at w = sqrt(z p)", r"\phi_{max} = \sin^{-1}\frac{p - z}{p + z},\ \omega_m = \sqrt{zp}",
    _lead,
    lambda i, o: (abs(math.degrees(math.atan(o["w_max"] / i["z"]) - math.atan(o["w_max"] / i["p"])) - o["phase_max_deg"]) / 90
                  + abs(i["z"] / (i["z"] ** 2 + o["w_max"] ** 2) - i["p"] / (i["p"] ** 2 + o["w_max"] ** 2)) * o["w_max"]
                  + abs(o["gain_at_w_max_db"] - 10 * math.log10((i["z"] ** 2 + o["w_max"] ** 2) / (i["p"] ** 2 + o["w_max"] ** 2))) / max(1.0, abs(o["gain_at_w_max_db"]))),
    _reference_lead,
    lambda z, p: {"phase_max_deg": math.degrees(math.atan((p - z) / (p + z))), "w_max": (z + p) / 2, "gain_at_w_max_db": 10 * math.log10(z / p)},
    lambda rng: (lambda z: {"z": z, "p": round(z * rng.uniform(1.5, 30), 3)})(round(10 ** rng.uniform(-1, 2), 3)),
    r"\blead compensator\b",
    ["Find the maximum phase lead of a lead compensator with z = 1 rad/s, p = 10 rad/s."],
    [{"inputs": {"z": 1, "p": 10}, "outputs": {"phase_max_deg": math.degrees(math.asin(9 / 11)), "w_max": math.sqrt(10)}},
     {"inputs": {"z": 2, "p": 18}, "outputs": {"w_max": 6.0, "phase_max_deg": math.degrees(math.asin(0.8))}}],
    [{"q": "Find the maximum phase lead of a lead compensator with z = 1 rad/s, p = 10 rad/s.", "outputs": {"w_max": math.sqrt(10)}},
     {"q": "Find the maximum phase lead of a lead compensator with z = 10 rad/s, p = 1 rad/s.", "outcome": "INVALID_INPUT"}])


def dc_motor(V, R, Kt, Ke, T, b) -> dict:
    _check_positive(R=R, Kt=Kt, Ke=Ke)
    if b < 0:
        raise ValueError("the friction coefficient must not be negative")
    # V = R I + Ke w ; Kt I = b w + T
    w = (Kt * V - R * T) / (Kt * Ke + R * b)
    i = (T + b * w) / Kt
    return {"speed": w, "rpm": w * 60 / (2 * math.pi), "current": i, "input_power": V * i, "output_power": T * w,
            "efficiency": (T * w) / (V * i) if V * i > 0 else 0.0}


def _reference_motor(V, R, Kt, Ke, T, b):
    """INDEPENDENT reference: the motor's electrical and mechanical equations L di/dt = V - R i - Ke w,
    J dw/dt = Kt i - b w - T integrated by scipy solve_ivp until the transient has died out (the steady state does not
    depend on the small L and J used)."""
    from scipy import integrate
    L, J = 0.5 * R, (Kt * Ke + R * b) / R             # electrical and mechanical time constants of order 1 s
    rhs = lambda t, y: [(V - R * y[0] - Ke * y[1]) / L, (Kt * y[0] - b * y[1] - T) / J]   # noqa: E731
    sol = integrate.solve_ivp(rhs, (0, 90.0), [0.0, 0.0], method="DOP853", rtol=1e-13, atol=1e-15)
    i, w = sol.y[0][-1], sol.y[1][-1]
    return {"speed": float(w), "current": float(i)}


PROCEDURES["dc_motor"] = _fspec(
    "Steady-state speed and current of a permanent-magnet DC motor", "control systems / electromechanical systems",
    "Speed, current, powers and efficiency of a DC motor with armature resistance R, torque constant Kt, back-emf constant Ke, viscous friction b and load torque T at supply voltage V.",
    [_inp("V", "voltage", "armature supply voltage"), _inp("R", "resistance", "armature resistance"), _inp("Kt", "number", "torque constant in N*m/A"),
     _inp("Ke", "number", "back-emf constant in V*s/rad"), _inp("T", "torque", "load torque"), _inp("b", "number", "viscous friction in N*m*s/rad")],
    [_out("speed", "angular_frequency", "steady-state speed"), _out("rpm", "number", "speed in revolutions per minute"), _out("current", "current", "armature current"),
     _out("input_power", "power", "electrical input power"), _out("output_power", "power", "mechanical output power to the load"),
     _out("efficiency", "number", "output over input power")],
    "speed", ["steady state", "linear model (no saturation, constant parameters)"],
    "solve V = R I + Ke w and Kt I = b w + T", r"\omega = \frac{K_tV - RT}{K_tK_e + Rb}",
    dc_motor,
    lambda i, o: (abs(i["V"] - i["R"] * o["current"] - i["Ke"] * o["speed"]) / max(abs(i["V"]), 1e-300)
                  + abs(i["Kt"] * o["current"] - i["b"] * o["speed"] - i["T"]) / max(abs(i["Kt"] * o["current"]), abs(i["T"]), 1e-300)
                  + _rel(o["rpm"] * 2 * math.pi / 60, o["speed"]) + abs(o["input_power"] - i["V"] * o["current"]) / max(abs(i["V"] * o["current"]), 1e-300)),
    _reference_motor,
    lambda V, R, Kt, Ke, T, b: (lambda w: {"speed": w, "current": T / Kt, "rpm": w * 60 / (2 * math.pi), "input_power": V * T / Kt, "output_power": T * w, "efficiency": 0.0})((V - R * T / Kt) / Ke),
    lambda rng: (lambda Kt: {"V": rng.choice([6.0, 12.0, 24.0, 48.0]), "R": round(rng.uniform(0.2, 5), 2), "Kt": Kt, "Ke": Kt,
                             "T": round(rng.uniform(0.0, 0.3) * Kt * 2, 4), "b": round(rng.uniform(1e-4, 5e-3), 5)})(round(rng.uniform(0.01, 0.3), 3)),
    r"\bdc motor\b",
    ["Find the steady-state speed of a DC motor with V = 12 V, R = 1 ohm, Kt = 0.05, Ke = 0.05, T = 0.02 N*m, b = 0.0001."],
    [{"inputs": {"V": 12, "R": 1, "Kt": 0.05, "Ke": 0.05, "T": 0.02, "b": 0.0001}, "outputs": {"speed": (0.05 * 12 - 0.02) / (0.0025 + 0.0001)}},
     {"inputs": {"V": 24, "R": 2, "Kt": 0.1, "Ke": 0.1, "T": 0, "b": 0}, "outputs": {"speed": 240.0, "current": 0.0}}],
    [{"q": "Find the steady-state speed of a DC motor with V = 12 V, R = 1 ohm, Kt = 0.05, Ke = 0.05, T = 0.02 N*m, b = 0.0001.",
      "outputs": {"speed": (0.05 * 12 - 0.02) / (0.0025 + 0.0001)}},
     {"q": "Find the steady-state speed of a DC motor with V = 12 V, R = 1 ohm, Kt = 0.05, Ke = 0.05, T = 0.02 N*m.", "outcome": "NEEDS_INPUT"}])


# ================================================================================ core course calculations (2026-11-12)
# Basic quantities and energy cost, resistance and temperature, cells, capacitors, magnetic circuits, induction,
# instruments, sinusoids, series and parallel a.c. circuits with a supply, two-wattmeter power, d.c. machines and plane
# waves. Constants: mu0 = 4 pi 1e-7 H/m and eps0 = 1/(mu0 c^2) (see the module docstring).
_MU0 = 4e-7 * math.pi
_C0 = 299792458.0
_EPS0 = 1.0 / (_MU0 * _C0 * _C0)
_TWO_PI = 2.0 * math.pi
_UNITS["time"].update({"h": 3600.0, "hours": 3600.0, "hour": 3600.0, "min": 60.0, "minutes": 60.0, "minute": 60.0})
_UNITS["length"] = {"m": 1.0, "cm": 1e-2, "mm": 1e-3, "um": 1e-6, "µm": 1e-6, "km": 1e3}
_UNITS["area"] = {"m^2": 1.0, "m2": 1.0, "cm^2": 1e-4, "cm2": 1e-4, "mm^2": 1e-6, "mm2": 1e-6}
_UNITS["magnetic_flux"] = _unit_table(["Wb"])
_UNITS["temperature"] = {"K": 1.0}
_EXTRA_UNITS["rotational_speed"] = {"rev/s": 1.0, "rps": 1.0, "r/s": 1.0, "rpm": 1.0 / 60.0, "rev/min": 1.0 / 60.0, "r/min": 1.0 / 60.0}
_EXTRA_UNITS["conductivity"] = {"S/m": 1.0, "kS/m": 1e3, "MS/m": 1e6, "mS/m": 1e-3}


def _celsius_to_kelvin(question):
    """'20 °C' (also degC, deg C, degrees C, degrees Celsius) -> '293.15 K', so that the reader takes every temperature
    in kelvin. A coefficient written per degree ('0.004 °C^-1') is left alone and is then refused as an unread number."""
    pat = rf"({_NUM_NB})\s*(?:°\s*C|degC|deg\s+C|degrees?\s+C(?:elsius)?)\b(?!\s*(?:\^|-1|⁻))"
    return re.sub(pat, lambda m: repr(float(m.group(1)) + 273.15) + " K", question)


# --------------------------------------------------------------------------------------------- energy and its cost
def electrical_energy_cost(P, t, tariff) -> dict:
    _check_positive(P=P, t=t)
    if not (math.isfinite(tariff) and tariff >= 0):
        raise ValueError("the tariff must be zero or positive")
    energy = P * t
    kwh = energy / 3.6e6
    return {"energy": energy, "energy_kWh": kwh, "cost": kwh * tariff}


def _verify_energy_cost(i, o):
    return (_rel(o["energy"], i["P"] * i["t"]) + _rel(o["energy_kWh"] * 3.6e6, o["energy"])
            + abs(o["cost"] - o["energy_kWh"] * i["tariff"]) / max(abs(o["cost"]), o["energy_kWh"] * i["tariff"], 1e-300))


def _reference_energy_cost(P, t, tariff):
    """INDEPENDENT reference (weak: a definition): the energy as the integral of the power over the time
    (scipy.integrate.quad) and the kilowatt-hour taken from scipy.constants (1000 W for one hour)."""
    from scipy import constants, integrate
    energy, _ = integrate.quad(lambda x: P, 0.0, t, epsabs=0, epsrel=1e-13)
    kwh = energy / (1000.0 * constants.hour)
    return {"energy": float(energy), "energy_kWh": float(kwh), "cost": float(kwh * tariff)}


PROCEDURES["electrical_energy_cost"] = _fspec(
    "Electrical energy and its cost", "basic electrical quantities / energy",
    "The energy a load of power P uses in a time t, in joules and in kilowatt-hours, and its cost at a tariff per kWh.",
    [_inp("P", "power", "power of the load"), _inp("t", "time", "running time (h and min accepted in questions)"),
     _inp("tariff", "number", "price of one kWh (any currency)")],
    [_out("energy", "energy", "energy used, in joules"), _out("energy_kWh", "number", "energy used, in kilowatt-hours"),
     _out("cost", "number", "cost of the energy, in the tariff's currency")],
    "cost", ["constant power", "P > 0, t > 0, tariff >= 0"], "W = P t; kWh = W / 3.6e6; cost = kWh x tariff",
    r"W = Pt,\quad \mathrm{cost} = \frac{Pt}{3.6\times10^{6}\ \mathrm{J/kWh}}\times\mathrm{tariff}",
    electrical_energy_cost, _verify_energy_cost, _reference_energy_cost,
    lambda P, t, tariff: {"energy": P * t, "energy_kWh": P * t / 3600.0, "cost": P * t / 3600.0 * tariff},   # Wh taken as kWh
    lambda rng: {"P": round(10 ** rng.uniform(0.7, 3.7), 1), "t": round(rng.uniform(0.1, 500), 2) * 3600.0, "tariff": round(rng.uniform(0.05, 0.5), 3)},
    r"\benergy cost\b|\bcost of (?:the )?(?:electrical )?energy\b",
    ["Find the energy cost with P = 2 kW, t = 5 h, tariff = 0.15 per kWh."],
    [{"inputs": {"P": 2000, "t": 18000, "tariff": 0.15}, "outputs": {"energy": 3.6e7, "energy_kWh": 10.0, "cost": 1.5}},
     {"inputs": {"P": 100, "t": 36000, "tariff": 0.2}, "outputs": {"energy": 3.6e6, "energy_kWh": 1.0, "cost": 0.2}}],
    [{"q": "Find the energy cost with P = 2 kW, t = 5 h, tariff = 0.15 per kWh.", "outputs": {"energy_kWh": 10.0, "cost": 1.5}},
     {"q": "What is the cost of energy for P = 100 W, t = 10 hours, tariff = 0.2 per kWh?", "outputs": {"energy": 3.6e6, "cost": 0.2}},
     {"q": "Find the energy cost with P = 2 kW, t = 5 h.", "outcome": "NEEDS_INPUT"},
     {"q": "Find the energy cost with P = 2 kW, t = 5 h, tariff = -0.15 per kWh.", "outcome": "INVALID_INPUT"}])


# --------------------------------------------------------------------------------------------- resistance and temperature
def resistance_at_temperature(R1, alpha1, T1, T2) -> dict:
    _check_positive(R1=R1, T1=T1, T2=T2)
    if not math.isfinite(alpha1):
        raise ValueError("alpha1 must be a finite number")
    g = 1.0 + alpha1 * (T2 - T1)
    if g <= 0:
        raise ValueError("the linear model gives no positive resistance at T2: the change of temperature is too large for this coefficient")
    return {"R2": R1 * g, "alpha2": alpha1 / g, "change": R1 * alpha1 * (T2 - T1)}


def _verify_r_temp(i, o):
    R1, a1 = i["R1"], i["alpha1"]
    return (_rel(o["R2"], R1 * (1.0 + a1 * (i["T2"] - i["T1"]))) + abs(o["alpha2"] * o["R2"] - a1 * R1) / max(abs(a1 * R1), 1e-300)
            + abs(o["change"] - (o["R2"] - R1)) / R1)


def _reference_r_temp(R1, alpha1, T1, T2):
    """INDEPENDENT reference: R2 by integrating the constant slope dR/dT = alpha1 R1 from T1 to T2
    (scipy.integrate.quad); alpha2 found by scipy.optimize.brentq as the coefficient referred to T2 that brings R2
    back to R1, R1 = R2 (1 + alpha2 (T1 - T2))."""
    from scipy import integrate, optimize
    dr, _ = integrate.quad(lambda T: alpha1 * R1, T1, T2, epsabs=0, epsrel=1e-13)
    R2 = R1 + dr
    if T1 == T2:
        a2 = alpha1
    else:
        a2 = optimize.brentq(lambda a: R2 * (1.0 + a * (T1 - T2)) - R1, -10.0, 10.0, xtol=1e-300, rtol=8.9e-16, maxiter=500)
    return {"R2": float(R2), "alpha2": float(a2), "change": float(dr)}


def _random_r_temp(rng):
    T1 = round(273.15 + rng.uniform(0, 40), 2)
    d = rng.uniform(5, 200) if rng.random() < 0.7 else -rng.uniform(5, 60)
    return {"R1": _e12(rng, 0, 4), "alpha1": rng.choice([0.0039, 0.00393, 0.0043, 0.0045, 0.004, 0.0068, 0.0006]), "T1": T1, "T2": round(T1 + d, 2)}


_RT_IN = [_inp("R1", "resistance", "resistance at the reference temperature T1"),
          _inp("alpha1", "number", "temperature coefficient of resistance referred to T1, per K (= per degree C)"),
          _inp("T1", "temperature", "reference temperature (degrees C accepted in questions)"), _inp("T2", "temperature", "new temperature")]

PROCEDURES["resistance_at_temperature"] = _fspec(
    "Resistance at a new temperature (temperature coefficient)", "basic electrical quantities / resistance variation",
    "The resistance of a conductor at a temperature T2 from its resistance R1 at T1 and its temperature coefficient referred to T1, and the coefficient referred to T2.",
    _RT_IN,
    [_out("R2", "resistance", "resistance at T2"), _out("alpha2", "number", "temperature coefficient referred to T2, per K"),
     _out("change", "resistance", "change of resistance R2 - R1")],
    "R2", ["a linear R(T) over the range", "R1 > 0, temperatures above absolute zero"], "R2 = R1 (1 + alpha1 (T2 - T1)); alpha2 = alpha1 / (1 + alpha1 (T2 - T1))",
    r"R_2 = R_1\left[1 + \alpha_1 (T_2 - T_1)\right]",
    resistance_at_temperature, _verify_r_temp, _reference_r_temp,
    lambda R1, alpha1, T1, T2: (lambda g: {"R2": R1 * g, "alpha2": alpha1 / g, "change": R1 * (g - 1.0)})(1.0 + alpha1 * T2),   # T2 not referred to T1
    _random_r_temp,
    r"\bresistance at (?:a |the )?new temperature\b",
    ["Find the resistance at a new temperature with R1 = 50 ohm, alpha1 = 0.004, T1 = 20 °C, T2 = 80 °C."],
    [{"inputs": {"R1": 50, "alpha1": 0.004, "T1": 293.15, "T2": 353.15}, "outputs": {"R2": 62.0, "alpha2": 0.004 / 1.24, "change": 12.0}},
     {"inputs": {"R1": 100, "alpha1": 0.0043, "T1": 273.15, "T2": 373.15}, "outputs": {"R2": 143.0, "change": 43.0}}],
    [{"q": "Find the resistance at a new temperature with R1 = 50 ohm, alpha1 = 0.004, T1 = 20 °C, T2 = 80 °C.", "outputs": {"R2": 62.0}},
     {"q": "Find the resistance at a new temperature with R1 = 100 ohm, alpha1 = 0.0043, T1 = 0 °C, T2 = 100 °C.", "outputs": {"R2": 143.0, "alpha2": 0.0043 / 1.43}},
     {"q": "Find the resistance at a new temperature with R1 = 50 ohm, alpha1 = 0.004, T1 = 20 °C.", "outcome": "NEEDS_INPUT"},
     {"q": "Find the resistance at a new temperature with R1 = 50 ohm, alpha1 = 0.004, T1 = 20 °C, T2 = -300 °C.", "outcome": "INVALID_INPUT"}],
    forbid=r"\btemperature coefficient from\b")


def _parse_r_temp(question):
    return _kv_parse(r"\bresistance at (?:a |the )?new temperature\b", _RT_IN, r"\btemperature coefficient from\b")(_celsius_to_kelvin(question))


PROCEDURES["resistance_at_temperature"]["parse"] = _parse_r_temp


def temperature_coefficient_from_resistances(R1, T1, R2, T2) -> dict:
    _check_positive(R1=R1, T1=T1, R2=R2, T2=T2)
    if T1 == T2:
        raise ValueError("the two temperatures must differ")
    slope = (R2 - R1) / (T2 - T1)
    return {"alpha1": slope / R1, "alpha2": slope / R2, "slope": slope}


def _verify_tc_from(i, o):
    R1, R2 = i["R1"], i["R2"]
    s = o["slope"]
    return (abs(s * (i["T2"] - i["T1"]) - (R2 - R1)) / max(abs(R2 - R1), 1e-12 * R1) + abs(o["alpha1"] * R1 - s) / max(abs(s), 1e-300)
            + abs(o["alpha2"] * R2 - s) / max(abs(s), 1e-300))


def _reference_tc_from(R1, T1, R2, T2):
    """INDEPENDENT reference: the straight line R(T) through the two readings fitted by numpy.polyfit; each
    coefficient is its slope over the fitted resistance at that temperature (numpy.polyval)."""
    import numpy as np
    k, b = np.polyfit([T1, T2], [R1, R2], 1)
    return {"slope": float(k), "alpha1": float(k / np.polyval([k, b], T1)), "alpha2": float(k / np.polyval([k, b], T2))}


def _random_tc_from(rng):
    r = _random_r_temp(rng)
    R1 = _e12(rng, 1, 4)
    return {"R1": R1, "T1": r["T1"], "R2": round(R1 * (1.0 + r["alpha1"] * (r["T2"] - r["T1"])), 6), "T2": r["T2"]}


_TCF_IN = [_inp("R1", "resistance", "resistance at T1"), _inp("T1", "temperature", "first temperature (degrees C accepted in questions)"),
           _inp("R2", "resistance", "resistance at T2"), _inp("T2", "temperature", "second temperature")]

PROCEDURES["temperature_coefficient_from_resistances"] = _fspec(
    "Temperature coefficient of resistance from two readings", "basic electrical quantities / resistance variation",
    "The temperature coefficient of resistance referred to each of two temperatures, from the resistances measured at them.",
    _TCF_IN,
    [_out("alpha1", "number", "temperature coefficient referred to T1, per K"), _out("alpha2", "number", "temperature coefficient referred to T2, per K"),
     _out("slope", "number", "change of resistance per kelvin, ohm/K")],
    "alpha1", ["a linear R(T) between the readings", "T1 != T2"], "alpha1 = (R2 - R1) / (R1 (T2 - T1))",
    r"\alpha_1 = \frac{R_2 - R_1}{R_1 (T_2 - T_1)}",
    temperature_coefficient_from_resistances, _verify_tc_from, _reference_tc_from,
    lambda R1, T1, R2, T2: (lambda s: {"alpha1": s / R2, "alpha2": s / R1, "slope": s})((R2 - R1) / (T2 - T1)),   # referred to the wrong reading
    _random_tc_from,
    r"\btemperature coefficient from\b",
    ["Find the temperature coefficient from R1 = 100 ohm, T1 = 20 °C, R2 = 120 ohm, T2 = 70 °C."],
    [{"inputs": {"R1": 100, "T1": 293.15, "R2": 120, "T2": 343.15}, "outputs": {"alpha1": 0.004, "alpha2": 1 / 300, "slope": 0.4}},
     {"inputs": {"R1": 50, "T1": 273.15, "R2": 69.25, "T2": 373.15}, "outputs": {"alpha1": 0.00385, "slope": 0.1925}}],
    [{"q": "Find the temperature coefficient from R1 = 100 ohm, T1 = 20 °C, R2 = 120 ohm, T2 = 70 °C.", "outputs": {"alpha1": 0.004}},
     {"q": "Find the temperature coefficient from R1 = 50 ohm, T1 = 0 °C, R2 = 69.25 ohm, T2 = 100 °C.", "outputs": {"alpha1": 0.00385}},
     {"q": "Find the temperature coefficient from R1 = 100 ohm, T1 = 20 °C, T2 = 70 °C.", "outcome": "NEEDS_INPUT"},
     {"q": "Find the temperature coefficient from R1 = 100 ohm, T1 = 20 °C, R2 = 120 ohm, T2 = 20 °C.", "outcome": "INVALID_INPUT"}],
    forbid=r"\bnew temperature\b")


def _parse_tc_from(question):
    return _kv_parse(r"\btemperature coefficient from\b", _TCF_IN, r"\bnew temperature\b")(_celsius_to_kelvin(question))


PROCEDURES["temperature_coefficient_from_resistances"]["parse"] = _parse_tc_from


# --------------------------------------------------------------------------------------------- cells and batteries
def cell_battery(E, r, n, connection, RL) -> dict:
    _check_positive(E=E, r=r, RL=RL)
    if not (math.isfinite(n) and n == int(n) and 1 <= n <= 1000):
        raise ValueError("the number of cells must be a whole number from 1 to 1000")
    if connection == "series":
        emf, rint = n * E, n * r
    else:
        emf, rint = E, r / n
    cur = emf / (rint + RL)
    vt = cur * RL
    return {"emf_total": emf, "r_total": rint, "I": cur, "V_terminal": vt, "lost_volts": cur * rint, "P_load": cur * vt}


def _verify_cells(i, o):
    E, r, n, RL = i["E"], i["r"], i["n"], i["RL"]
    want_e = n * E if i["connection"] == "series" else E
    want_r = n * r if i["connection"] == "series" else r / n
    return (_rel(o["emf_total"], want_e) + _rel(o["r_total"], want_r) + abs(o["emf_total"] - o["I"] * (o["r_total"] + RL)) / o["emf_total"]
            + _rel(o["V_terminal"], o["I"] * RL) + abs(o["lost_volts"] - o["I"] * o["r_total"]) / o["emf_total"] + _rel(o["P_load"], o["I"] * o["V_terminal"]))


def _reference_cells(E, r, n, connection, RL):
    """INDEPENDENT reference: every cell drawn as its own source with its own internal resistor, wired in series or in
    parallel, solved exactly by sympy (Kirchhoff's laws in rational arithmetic) with and without the load; the combined
    emf is the open-circuit voltage and the combined internal resistance follows from the loaded solution."""
    k = int(n)
    parts = []
    if connection == "series":
        prev = "0"
        for j in range(1, k + 1):
            parts.append(f"V{j} p{j} {prev} {E!r}")
            parts.append(f"R{j} p{j} q{j} {r!r}")
            prev = f"q{j}"
        node = prev
    else:
        for j in range(1, k + 1):
            parts.append(f"V{j} p{j} 0 {E!r}")
            parts.append(f"R{j} p{j} t {r!r}")
        node = "t"
    v_open = _netlist_solve("; ".join(parts))[0][node]
    v_load = _netlist_solve("; ".join(parts + [f"RL {node} 0 {RL!r}"]))[0][node]
    cur = v_load / RL
    return {"emf_total": v_open, "r_total": (v_open - v_load) / cur, "I": cur, "V_terminal": v_load, "lost_volts": v_open - v_load, "P_load": cur * v_load}


_CELL_IN = [_inp("E", "voltage", "emf of one cell"), _inp("r", "resistance", "internal resistance of one cell"),
            _inp("n", "number", "number of identical cells"),
            {"symbol": "connection", "type": "choice", "choices": ["series", "parallel"], "required": True, "meaning": "how the cells are connected"},
            _inp("RL", "resistance", "load resistance")]


def _parse_cells(question):
    q = question.lower()
    if not re.search(r"\bcells\b", q):
        return None
    ser, par = re.search(r"\bin series\b", q), re.search(r"\bin parallel\b", q)
    if ser and par:
        return None
    base = _kv_parse(r"\bcells\b", [x for x in _CELL_IN if x["symbol"] != "connection"])(question)
    if base is None:
        return None
    if ser or par:
        base["connection"] = "series" if ser else "parallel"
    return base


PROCEDURES["cell_battery"] = {
    "name": "Cells with internal resistance: series and parallel batteries on a load", "topic": "basic electrical quantities / cells and batteries",
    "purpose": "The combined emf and internal resistance of n identical cells in series or in parallel, the load current, the terminal voltage, the lost volts and the load power.",
    "inputs": _CELL_IN,
    "outputs": [_out("emf_total", "voltage", "emf of the battery"), _out("r_total", "resistance", "internal resistance of the battery"),
                _out("I", "current", "load current"), _out("V_terminal", "voltage", "terminal voltage (across the load)"),
                _out("lost_volts", "voltage", "voltage lost inside the battery, I r"), _out("P_load", "power", "power delivered to the load")],
    "primary": "V_terminal", "conditions": ["identical cells", "E, r, RL > 0", "n a whole number"],
    "method": "series: E_b = n E, r_b = n r; parallel: E_b = E, r_b = r/n; I = E_b/(r_b + R_L), V = I R_L",
    "latex": r"I = \frac{E_b}{r_b + R_L},\quad V = E_b - I r_b",
    "run": cell_battery, "verify": _verify_cells, "parse": _parse_cells, "reference": _reference_cells,
    "wrong": lambda E, r, n, connection, RL: (lambda emf: {"emf_total": emf, "r_total": r, "I": emf / (r + RL), "V_terminal": emf * RL / (r + RL),
                                                           "lost_volts": emf * r / (r + RL), "P_load": (emf / (r + RL)) ** 2 * RL})(n * E if connection == "series" else E),
    "random_inputs": lambda rng: {"E": rng.choice([1.2, 1.5, 2.0, 2.1, 3.7]), "r": round(rng.uniform(0.02, 1.0), 3), "n": float(rng.randint(2, 8)),
                                  "connection": rng.choice(["series", "parallel"]), "RL": _e12(rng, 0, 1)},
    "templates": ["Find the terminal voltage of identical cells in series with E = 1.5 V, r = 0.5 ohm, n = 4, RL = 10 ohm."],
    "examples": [{"inputs": {"E": 1.5, "r": 0.5, "n": 4, "connection": "series", "RL": 10},
                  "outputs": {"emf_total": 6.0, "r_total": 2.0, "I": 0.5, "V_terminal": 5.0, "lost_volts": 1.0, "P_load": 2.5}},
                 {"inputs": {"E": 2, "r": 0.3, "n": 3, "connection": "parallel", "RL": 1.9},
                  "outputs": {"emf_total": 2.0, "r_total": 0.1, "I": 1.0, "V_terminal": 1.9, "lost_volts": 0.1}}],
    "questions": [{"q": "Find the terminal voltage of identical cells in series with E = 1.5 V, r = 0.5 ohm, n = 4, RL = 10 ohm.", "outputs": {"V_terminal": 5.0, "I": 0.5}},
                  {"q": "Find the current from identical cells in parallel with E = 2 V, r = 0.3 ohm, n = 3, RL = 1.9 ohm.", "outputs": {"I": 1.0}},
                  {"q": "Find the terminal voltage of identical cells in series with E = 1.5 V, r = 0.5 ohm, n = 4.", "outcome": "NEEDS_INPUT"},
                  {"q": "Find the terminal voltage of identical cells in series with E = 1.5 V, r = 0.5 ohm, n = 2.5, RL = 10 ohm.", "outcome": "INVALID_INPUT"}]}


# --------------------------------------------------------------------------------------------- capacitors
def parallel_plate_capacitor(V, d, A, eps_r, n=2.0) -> dict:
    _check_positive(V=V, d=d, A=A)
    if not (math.isfinite(eps_r) and eps_r >= 1):
        raise ValueError("the relative permittivity must be at least 1")
    if not (math.isfinite(n) and n == int(n) and 2 <= n <= 1000):
        raise ValueError("the number of plates must be a whole number from 2 to 1000")
    cap = _EPS0 * eps_r * A * (n - 1) / d
    field = V / d
    return {"C": cap, "Q": cap * V, "E": field, "D": _EPS0 * eps_r * field, "energy": 0.5 * cap * V * V}


def _verify_plate_cap(i, o):
    gaps = i.get("n", 2.0) - 1
    return (_rel(o["E"] * i["d"], i["V"]) + _rel(o["D"], _EPS0 * i["eps_r"] * o["E"]) + _rel(o["D"] * gaps * i["A"], o["Q"])
            + _rel(o["Q"], o["C"] * i["V"]) + _rel(o["energy"], 0.5 * o["Q"] * i["V"]))


def _reference_plate_cap(V, d, A, eps_r, n=2.0):
    """INDEPENDENT reference (field-energy route): the energy density eps E^2/2 of the uniform field E = V/d integrated
    across each dielectric gap (scipy.integrate.quad) and summed over the n - 1 gaps; C = 2W/V^2, Q = CV and D from
    Gauss's law, D = Q/((n - 1) A). eps0 = 1/(mu0 c^2) from scipy.constants.c with mu0 = 4 pi 1e-7."""
    from scipy import constants, integrate
    eps = eps_r / (4e-7 * math.pi * constants.c ** 2)
    field = V / d
    gaps = int(n) - 1
    w1, _ = integrate.quad(lambda x: 0.5 * eps * field * field * A, 0.0, d, epsabs=0, epsrel=1e-13)
    energy = gaps * w1
    cap = 2.0 * energy / (V * V)
    return {"C": float(cap), "Q": float(cap * V), "E": float(field), "D": float(cap * V / (gaps * A)), "energy": float(energy)}


PROCEDURES["parallel_plate_capacitor"] = _fspec(
    "Parallel-plate capacitor: capacitance, charge, field, flux density and energy", "capacitors / parallel plates",
    "The capacitance of a capacitor of n parallel plates with a dielectric, and its charge, electric field strength, electric flux density and stored energy at a voltage V.",
    [_inp("V", "voltage", "voltage across the capacitor"), _inp("d", "length", "separation of adjacent plates (dielectric thickness)"),
     _inp("A", "area", "overlapping area of one plate"), _inp("eps_r", "number", "relative permittivity of the dielectric (1 for air)"),
     _inp("n", "number", "number of plates (2 when not given)", required=False)],
    [_out("C", "capacitance", "capacitance"), _out("Q", "charge", "charge"), _out("E", "electric_field", "electric field strength V/d"),
     _out("D", "number", "electric flux density in C/m^2"), _out("energy", "energy", "stored energy")],
    "C", ["uniform field (plate size much larger than d)", "eps_r >= 1, n >= 2 plates"],
    "C = eps0 eps_r A (n - 1)/d; Q = CV; E = V/d; D = eps0 eps_r E; W = CV^2/2",
    r"C = \frac{\varepsilon_0\varepsilon_r A (n - 1)}{d},\quad D = \varepsilon_0\varepsilon_r E,\quad W = \tfrac{1}{2}CV^2",
    parallel_plate_capacitor, _verify_plate_cap, _reference_plate_cap,
    lambda V, d, A, eps_r, n=2.0: (lambda cap: {"C": cap, "Q": cap * V, "E": V / d, "D": _EPS0 * eps_r * V / d, "energy": 0.5 * cap * V * V})(_EPS0 * eps_r * A * n / d),
    lambda rng: {"V": rng.choice([10.0, 50.0, 100.0, 200.0, 500.0]), "d": round(rng.uniform(0.05, 2.0), 3) * 1e-3, "A": round(rng.uniform(1, 100), 2) * 1e-4,
                 "eps_r": rng.choice([1.0, 2.2, 2.5, 3.5, 5.0, 6.0, 10.0]), "n": float(rng.randint(2, 10))},
    r"\b(?:parallel[- ])?plate capacitor\b",
    ["Find the capacitance of a parallel-plate capacitor with V = 200 V, d = 0.1 mm, A = 4 cm^2, eps_r = 5."],
    [{"inputs": {"V": 200, "d": 1e-4, "A": 4e-4, "eps_r": 5, "n": 2},
      "outputs": {"C": 20 * 8.854187817620389e-12, "Q": 4000 * 8.854187817620389e-12, "E": 2e6, "D": 1e7 * 8.854187817620389e-12, "energy": 4e5 * 8.854187817620389e-12}},
     {"inputs": {"V": 100, "d": 5e-4, "A": 1e-3, "eps_r": 2.5, "n": 11}, "outputs": {"C": 50 * 8.854187817620389e-12, "E": 2e5}}],
    [{"q": "Find the capacitance of a parallel-plate capacitor with V = 200 V, d = 0.1 mm, A = 4 cm^2, eps_r = 5.", "outputs": {"C": 20 * 8.854187817620389e-12}},
     {"q": "Find the charge on a parallel-plate capacitor with V = 100 V, d = 0.5 mm, A = 10 cm^2, eps_r = 2.5, n = 11.", "outputs": {"Q": 5000 * 8.854187817620389e-12}},
     {"q": "Find the capacitance of a parallel-plate capacitor with V = 200 V, A = 4 cm^2, eps_r = 5.", "outcome": "NEEDS_INPUT"},
     {"q": "Find the capacitance of a parallel-plate capacitor with V = 200 V, d = 0.1 mm, A = 4 cm^2, eps_r = 0.5.", "outcome": "INVALID_INPUT"}])


# --------------------------------------------------------------------------------------------- magnetic circuits
def magnetic_circuit(N, lengths, areas, mu_r, flux=0.0, current=0.0) -> dict:
    _check_positive(N=N)
    k = len(lengths)
    if k == 0 or len(areas) != k or len(mu_r) != k:
        raise ValueError("give one length, one area and one relative permeability for every section")
    for j in range(k):
        if not (math.isfinite(lengths[j]) and lengths[j] > 0 and math.isfinite(areas[j]) and areas[j] > 0):
            raise ValueError("every length and every area must be positive")
        if not (math.isfinite(mu_r[j]) and mu_r[j] >= 1):
            raise ValueError("every relative permeability must be at least 1")
    if not (math.isfinite(flux) and math.isfinite(current)) or flux < 0 or current < 0:
        raise ValueError("the flux and the current must not be negative")
    given = 0
    if flux > 0:
        given += 1
    if current > 0:
        given += 1
    if given != 1:
        raise ValueError("give exactly one of the flux and the current")
    rel = 0.0
    for j in range(k):
        rel += lengths[j] / (_MU0 * mu_r[j] * areas[j])
    if flux > 0:
        phi = flux
        mmf = phi * rel
        cur = mmf / N
    else:
        cur = current
        mmf = N * cur
        phi = mmf / rel
    B = []
    H = []
    for j in range(k):
        b = phi / areas[j]
        B.append(b)
        H.append(b / (_MU0 * mu_r[j]))
    return {"reluctance": rel, "mmf": mmf, "current": cur, "flux": phi, "B": B, "H": H}


def _verify_magnetic(i, o):
    """Ampere's law around the circuit (sum of H l = N I), B = flux/area and H = B/(mu0 mu_r) in every section, and
    the given quantity reproduced."""
    k = len(i["lengths"])
    if len(o["B"]) != k or len(o["H"]) != k:
        return 1.0
    res = 0.0
    amp = 0.0
    for j in range(k):
        res += _rel(o["B"][j], o["flux"] / i["areas"][j]) + _rel(o["H"][j] * _MU0 * i["mu_r"][j], o["B"][j])
        amp += o["H"][j] * i["lengths"][j]
    res += _rel(amp, i["N"] * o["current"]) + _rel(o["mmf"], i["N"] * o["current"]) + _rel(o["reluctance"] * o["flux"], o["mmf"])
    if i.get("flux", 0.0) > 0:
        res += _rel(o["flux"], i["flux"])
    else:
        res += _rel(o["current"], i.get("current", 0.0))
    return res


def _reference_magnetic(N, lengths, areas, mu_r, flux=0.0, current=0.0):
    """INDEPENDENT reference: Ampere's circuital law as a line integral, mmf = integral of H dl along the flux path
    with H piecewise (B = flux/area, H = B/(mu0 mu_r)), integrated by scipy.integrate.quad section by section; with
    the current given, the flux is found by scipy.optimize.brentq so that the integral equals N I."""
    from scipy import integrate, optimize
    mu0 = 4e-7 * math.pi

    def line(phi):
        tot, start = 0.0, 0.0
        for l, a, m in zip(lengths, areas, mu_r):
            v, _ = integrate.quad(lambda x, a=a, m=m: phi / a / (mu0 * m), start, start + l, epsabs=0, epsrel=1e-13)
            tot += v
            start += l
        return tot
    if flux > 0:
        phi = flux
        mmf = line(phi)
    else:
        mmf = N * current
        hi = 1e-6
        for _ in range(400):
            if line(hi) >= mmf:
                break
            hi *= 10.0
        phi = optimize.brentq(lambda p: line(p) - mmf, 0.0, hi, xtol=1e-300, rtol=8.9e-16, maxiter=500)
    B = [phi / a for a in areas]
    return {"mmf": float(mmf), "current": float(mmf / N), "flux": float(phi), "reluctance": float(mmf / phi), "B": B,
            "H": [b / (mu0 * m) for b, m in zip(B, mu_r)]}


def _random_magnetic(rng):
    k = rng.randint(1, 2)
    lengths = [round(rng.uniform(0.1, 1.0), 3) for _ in range(k)]
    areas = [round(rng.uniform(1, 10), 2) * 1e-4 for _ in range(k)]
    mu = [float(rng.choice([300, 500, 800, 1000, 1500, 2000, 3000])) for _ in range(k)]
    if rng.random() < 0.6:                                  # an air gap
        lengths.append(round(rng.uniform(0.2, 3.0), 2) * 1e-3)
        areas.append(areas[0])
        mu.append(1.0)
    out = {"N": float(rng.randint(50, 2000)), "lengths": lengths, "areas": areas, "mu_r": mu}
    if rng.random() < 0.5:
        out["flux"] = round(rng.uniform(0.1, 2.0), 3) * 1e-3
    else:
        out["current"] = round(rng.uniform(0.2, 10), 2)
    return out


_MAG_IN = [_inp("N", "number", "number of turns of the magnetising coil"),
           {"symbol": "lengths", "type": "vector", "min_length": 1, "max_length": 10, "required": True,
            "meaning": "mean length of the flux path in each section, in m (an air gap is a section)"},
           {"symbol": "areas", "type": "vector", "min_length": 1, "max_length": 10, "required": True, "meaning": "cross-sectional area of each section, in m^2"},
           {"symbol": "mu_r", "type": "vector", "min_length": 1, "max_length": 10, "required": True, "meaning": "relative permeability of each section (1 for air)"},
           _inp("flux", "magnetic_flux", "flux to produce (give this or the current)", required=False),
           _inp("current", "current", "coil current (give this or the flux)", required=False)]


def _parse_magnetic(question):
    if not re.search(r"\bmagnetic circuit\b", question, re.I):
        return None
    lists = {}

    def grab(m):
        lists[m.group(1)] = m.group(2)
        return " "
    rest = re.sub(r"\b(lengths|areas|mu_r)\s*=\s*(\[[^\[\]]*\])", grab, question)
    if "[" in rest or "]" in rest:
        return None
    base = _kv_parse(r"\bmagnetic circuit\b", [x for x in _MAG_IN if x["symbol"] in ("N", "flux", "current")])(rest)
    if base is None:
        if re.search(_NUMRE, rest):
            return None
        base = {}
    base.update(lists)
    return base or None


PROCEDURES["magnetic_circuit"] = {
    "name": "Series magnetic circuit: reluctance, magnetomotive force, flux density and field strength", "topic": "magnetic circuits / series circuits",
    "purpose": "The current a coil of N turns needs to set up a flux in a series magnetic circuit of sections (iron paths and air gaps), or the flux a current produces, with the reluctance, the mmf, and B and H in every section.",
    "inputs": _MAG_IN,
    "outputs": [_out("reluctance", "number", "total reluctance in A/Wb (ampere-turns per weber)"), _out("mmf", "number", "magnetomotive force N I in ampere-turns"),
                _out("current", "current", "coil current"), _out("flux", "magnetic_flux", "flux in the circuit"),
                _out("B", "number", "flux density of each section in T"), _out("H", "number", "magnetic field strength of each section in A/m")],
    "primary": "current", "conditions": ["linear materials: a constant relative permeability in each section (no B-H curve)", "no leakage or fringing",
                                          "exactly one of the flux and the current"],
    "method": "S = sum l/(mu0 mu_r A); F = N I = flux S; B = flux/A; H = B/(mu0 mu_r)",
    "latex": r"S = \sum_k \frac{l_k}{\mu_0\mu_{r,k}A_k},\quad NI = \Phi S = \sum_k H_k l_k",
    "run": magnetic_circuit, "verify": _verify_magnetic, "parse": _parse_magnetic, "reference": _reference_magnetic,
    "wrong": lambda N, lengths, areas, mu_r, flux=0.0, current=0.0: magnetic_circuit(N, lengths, areas, [1.0 for _ in mu_r], flux, current),   # mu_r forgotten
    "random_inputs": _random_magnetic,
    "templates": ["Find the current for a magnetic circuit with N = 500, lengths = [0.4, 0.001], areas = [5e-4, 5e-4], mu_r = [800, 1], flux = 0.6 mWb."],
    "examples": [{"inputs": {"N": 500, "lengths": [0.4], "areas": [5e-4], "mu_r": [800], "flux": 6e-4},
                  "outputs": {"reluctance": 0.4 / (4e-7 * math.pi * 800 * 5e-4), "current": 6e-4 * 0.4 / (4e-7 * math.pi * 800 * 5e-4) / 500,
                              "B": [1.2], "H": [1.2 / (4e-7 * math.pi * 800)]}},
                 {"inputs": {"N": 500, "lengths": [0.4, 0.001], "areas": [5e-4, 5e-4], "mu_r": [800, 1], "flux": 6e-4},
                  "outputs": {"mmf": 6e-4 * (0.4 / (4e-7 * math.pi * 800 * 5e-4) + 0.001 / (4e-7 * math.pi * 5e-4))}},
                 {"inputs": {"N": 200, "lengths": [0.5], "areas": [4e-4], "mu_r": [1000], "current": 2},
                  "outputs": {"mmf": 400.0, "flux": 400 / (0.5 / (4e-7 * math.pi * 1000 * 4e-4))}}],
    "questions": [{"q": "Find the current for a magnetic circuit with N = 500, lengths = [0.4, 0.001], areas = [5e-4, 5e-4], mu_r = [800, 1], flux = 0.6 mWb.",
                   "outputs": {"current": 6e-4 * (0.4 / (4e-7 * math.pi * 800 * 5e-4) + 0.001 / (4e-7 * math.pi * 5e-4)) / 500}},
                  {"q": "Find the flux of a magnetic circuit with N = 200, lengths = [0.5], areas = [4e-4], mu_r = [1000], current = 2 A.",
                   "outputs": {"flux": 400 / (0.5 / (4e-7 * math.pi * 1000 * 4e-4))}},
                  {"q": "Find the current for a magnetic circuit with N = 500, lengths = [0.4], areas = [5e-4], mu_r = [800], flux = 0.6 mWb, current = 2 A.",
                   "outcome": "INVALID_INPUT"},
                  {"q": "Find the current for a magnetic circuit with N = 500, lengths = [0.4, 0.001], areas = [5e-4], mu_r = [800, 1], flux = 0.6 mWb.",
                   "outcome": "INVALID_INPUT"}]}


# --------------------------------------------------------------------------------------------- induction and inductance
def coil_inductance(N, length, area, mu_r) -> dict:
    _check_positive(N=N, length=length, area=area)
    if not (math.isfinite(mu_r) and mu_r >= 1):
        raise ValueError("the relative permeability must be at least 1")
    rel = length / (_MU0 * mu_r * area)
    return {"L": N * N / rel, "reluctance": rel, "flux_per_ampere": N / rel}


def _verify_coil_l(i, o):
    return (_rel(o["L"] * o["reluctance"], i["N"] * i["N"]) + _rel(o["reluctance"] * _MU0 * i["mu_r"] * i["area"], i["length"])
            + _rel(o["flux_per_ampere"] * i["N"], o["L"]))


def _reference_coil_l(N, length, area, mu_r):
    """INDEPENDENT reference (field-energy route): at a coil current of 1 A, H = N I/l and B = mu0 mu_r H; the
    energy density B H/2 integrated over the core volume (scipy.integrate.quad along the length) gives W and
    L = 2W/I^2; the reluctance is N^2/L and the flux per ampere B A."""
    from scipy import integrate
    H = N / length
    B = 4e-7 * math.pi * mu_r * H
    W, _ = integrate.quad(lambda x: 0.5 * B * H * area, 0.0, length, epsabs=0, epsrel=1e-13)
    L = 2.0 * W
    return {"L": float(L), "reluctance": float(N * N / L), "flux_per_ampere": float(B * area)}


PROCEDURES["coil_inductance"] = _fspec(
    "Inductance of a coil on a uniform core", "electromagnetic induction / inductance",
    "The self-inductance N^2/S of a coil of N turns on a uniform core (or a long air-cored coil), with the reluctance of the flux path and the flux per ampere.",
    [_inp("N", "number", "number of turns"), _inp("length", "length", "length of the flux path (coil length for an air-cored solenoid)"),
     _inp("area", "area", "cross-sectional area of the core"), _inp("mu_r", "number", "relative permeability of the core (1 for air)")],
    [_out("L", "inductance", "self-inductance"), _out("reluctance", "number", "reluctance of the flux path in A/Wb"),
     _out("flux_per_ampere", "number", "flux per ampere of coil current, Wb/A")],
    "L", ["uniform field along the path, no leakage (a long coil or a closed core)", "linear core"],
    "S = l/(mu0 mu_r A); L = N^2/S = mu0 mu_r N^2 A/l", r"L = \frac{N^2}{S} = \frac{\mu_0\mu_r N^2 A}{l}",
    coil_inductance, _verify_coil_l, _reference_coil_l,
    lambda N, length, area, mu_r: (lambda rel: {"L": N / rel, "reluctance": rel, "flux_per_ampere": N / rel})(length / (_MU0 * mu_r * area)),   # N not squared
    lambda rng: {"N": float(rng.randint(10, 2000)), "length": round(rng.uniform(0.05, 1.0), 3), "area": round(rng.uniform(0.5, 20), 2) * 1e-4,
                 "mu_r": rng.choice([1.0, 1.0, 100.0, 500.0, 1000.0, 2000.0])},
    r"\binductance of (?:a |the )?coil\b",
    ["Find the inductance of a coil with N = 500, length = 0.2 m, area = 4 cm^2, mu_r = 1."],
    [{"inputs": {"N": 500, "length": 0.2, "area": 4e-4, "mu_r": 1}, "outputs": {"L": 2e-4 * math.pi, "reluctance": 0.2 / (4e-7 * math.pi * 4e-4)}},
     {"inputs": {"N": 200, "length": 0.5, "area": 1e-3, "mu_r": 500}, "outputs": {"L": 0.016 * math.pi}}],
    [{"q": "Find the inductance of a coil with N = 500, length = 0.2 m, area = 4 cm^2, mu_r = 1.", "outputs": {"L": 2e-4 * math.pi}},
     {"q": "What is the inductance of a coil with N = 200, length = 50 cm, area = 10 cm^2, mu_r = 500?", "outputs": {"L": 0.016 * math.pi}},
     {"q": "Find the inductance of a coil with N = 500, length = 0.2 m, mu_r = 1.", "outcome": "NEEDS_INPUT"},
     {"q": "Find the inductance of a coil with N = 500, length = 0.2 m, area = 4 cm^2, mu_r = 0.5.", "outcome": "INVALID_INPUT"}],
    forbid=r"\bself[- ]inductance\b|\bcapacitor\b")


def self_inductance(N, flux, I) -> dict:
    _check_positive(N=N, flux=flux, I=I)
    return {"L": N * flux / I, "flux_linkage": N * flux}


def _verify_self_l(i, o):
    return _rel(o["L"] * i["I"], o["flux_linkage"]) + _rel(o["flux_linkage"], i["N"] * i["flux"])


def _reference_self_l(N, flux, I):
    """INDEPENDENT reference (weak: energy route of the definition): with the flux linkage proportional to the
    current, the work done against the induced emf while the current rises to I is the integral of N phi(i) di
    (scipy.integrate.quad); L = 2W/I^2."""
    from scipy import integrate
    W, _ = integrate.quad(lambda x: N * flux * x / I, 0.0, I, epsabs=0, epsrel=1e-13)
    L = 2.0 * W / (I * I)
    return {"L": float(L), "flux_linkage": float(L * I)}


PROCEDURES["self_inductance"] = _fspec(
    "Self-inductance from flux linkage", "electromagnetic induction / inductance",
    "The self-inductance L = N phi/I of a coil of N turns in which a current I sets up a flux phi.",
    [_inp("N", "number", "number of turns"), _inp("flux", "magnetic_flux", "flux linking every turn"), _inp("I", "current", "coil current")],
    [_out("L", "inductance", "self-inductance"), _out("flux_linkage", "number", "flux linkage N phi in weber-turns")],
    "L", ["the flux links every turn", "a linear core (flux proportional to the current)"], "L = N phi / I", r"L = \frac{N\Phi}{I}",
    self_inductance, _verify_self_l, _reference_self_l,
    lambda N, flux, I: {"L": flux / I, "flux_linkage": N * flux},                    # turns forgotten
    lambda rng: {"N": float(rng.randint(10, 3000)), "flux": round(rng.uniform(0.01, 10), 3) * 1e-3, "I": round(rng.uniform(0.1, 20), 2)},
    r"\bself[- ]inductance\b",
    ["Find the self-inductance with N = 200, flux = 5 mWb, I = 2 A."],
    [{"inputs": {"N": 200, "flux": 5e-3, "I": 2}, "outputs": {"L": 0.5, "flux_linkage": 1.0}},
     {"inputs": {"N": 1000, "flux": 2e-4, "I": 0.5}, "outputs": {"L": 0.4, "flux_linkage": 0.2}}],
    [{"q": "Find the self-inductance with N = 200, flux = 5 mWb, I = 2 A.", "outputs": {"L": 0.5}},
     {"q": "What is the self-inductance of a coil with N = 1000, flux = 0.2 mWb, I = 0.5 A?", "outputs": {"L": 0.4}},
     {"q": "Find the self-inductance with N = 200, I = 2 A.", "outcome": "NEEDS_INPUT"},
     {"q": "Find the self-inductance with N = 200, flux = 5 mWb, I = 0 A.", "outcome": "INVALID_INPUT"}])


def faraday_emf(N, dPhi, dt) -> dict:
    _check_positive(N=N, dt=dt)
    if not math.isfinite(dPhi):
        raise ValueError("the change of flux must be a finite number")
    rate = dPhi / dt
    return {"emf": N * abs(rate), "emf_signed": -N * rate + 0.0, "flux_rate": rate, "linkage_change": N * dPhi}


def _verify_faraday(i, o):
    N, d, t = i["N"], i["dPhi"], i["dt"]
    big = max(N * abs(d), 1e-300)
    return (abs(o["emf"] * t - N * abs(d)) / big + abs(o["emf_signed"] * t + N * d) / big
            + abs(o["flux_rate"] * t - d) / max(abs(d), 1e-300) + abs(o["linkage_change"] - N * d) / big)


def _reference_faraday(N, dPhi, dt):
    """INDEPENDENT reference: the flux written as a function of time (a uniform change of dPhi over dt) and the emf
    e = -N dphi/dt obtained by sympy differentiation, in exact rational arithmetic."""
    import sympy as sp
    t = sp.Symbol("t")
    phi = _exact(dPhi) * t / _exact(dt)
    rate = sp.diff(phi, t)
    e = -_exact(N) * rate
    return {"emf": float(abs(e)), "emf_signed": float(e), "flux_rate": float(rate), "linkage_change": float(_exact(N) * _exact(dPhi))}


PROCEDURES["faraday_emf"] = _fspec(
    "Induced emf from a change of flux (Faraday's law)", "electromagnetic induction / Faraday's law",
    "The average emf induced in a coil of N turns when the flux linking it changes by dPhi in a time dt, its sign by Lenz's law, and the change of flux linkage.",
    [_inp("N", "number", "number of turns"), _inp("dPhi", "magnetic_flux", "change of flux (negative for a decrease)"),
     _inp("dt", "time", "time taken by the change")],
    [_out("emf", "voltage", "magnitude of the average induced emf"), _out("emf_signed", "voltage", "induced emf with Lenz's sign, -N dPhi/dt"),
     _out("flux_rate", "number", "rate of change of flux in Wb/s"), _out("linkage_change", "number", "change of flux linkage N dPhi in weber-turns")],
    "emf", ["a uniform rate of change (the result is the average emf)", "the flux links every turn"],
    "e = -N dPhi/dt", r"e = -N\frac{\Delta\Phi}{\Delta t}",
    faraday_emf, _verify_faraday, _reference_faraday,
    lambda N, dPhi, dt: {"emf": abs(dPhi / dt), "emf_signed": -dPhi / dt, "flux_rate": dPhi / dt, "linkage_change": dPhi},   # turns forgotten
    lambda rng: {"N": float(rng.randint(10, 2000)), "dPhi": (round(rng.uniform(-50, 50), 2) or 1.0) * 1e-3, "dt": round(rng.uniform(1, 500), 1) * 1e-3},
    r"\binduced emf\b|\bfaraday\b",
    ["Find the induced emf with N = 200, dPhi = 25 mWb, dt = 50 ms."],
    [{"inputs": {"N": 200, "dPhi": 0.025, "dt": 0.05}, "outputs": {"emf": 100.0, "emf_signed": -100.0, "flux_rate": 0.5, "linkage_change": 5.0}},
     {"inputs": {"N": 500, "dPhi": -4e-3, "dt": 0.02}, "outputs": {"emf": 100.0, "emf_signed": 100.0, "flux_rate": -0.2}}],
    [{"q": "Find the induced emf with N = 200, dPhi = 25 mWb, dt = 50 ms.", "outputs": {"emf": 100.0}},
     {"q": "Find the induced emf with N = 500, dPhi = -4 mWb, dt = 20 ms.", "outputs": {"emf": 100.0, "emf_signed": 100.0}},
     {"q": "Find the induced emf with N = 200, dPhi = 25 mWb.", "outcome": "NEEDS_INPUT"},
     {"q": "Find the induced emf with N = 200, dPhi = 25 mWb, dt = 0 ms.", "outcome": "INVALID_INPUT"}],
    forbid=r"\bmutual\b|\bgenerated\b")


def mutual_inductance(L1, L2, k, dI1, dt) -> dict:
    _check_positive(L1=L1, L2=L2, dt=dt)
    if not (math.isfinite(k) and 0 <= k <= 1):
        raise ValueError("the coupling coefficient must be between 0 and 1")
    if not math.isfinite(dI1):
        raise ValueError("the change of current must be a finite number")
    m = k * math.sqrt(L1 * L2)
    rate = abs(dI1) / dt
    return {"M": m, "E1": L1 * rate, "E2": m * rate, "L_aiding": L1 + L2 + 2.0 * m, "L_opposing": L1 + L2 - 2.0 * m}


def _verify_mutual(i, o):
    L1, L2, k = i["L1"], i["L2"], i["k"]
    rate = abs(i["dI1"]) / i["dt"]
    big = max(L1 * rate, 1e-300)
    return (abs(o["M"] ** 2 - k * k * L1 * L2) / (L1 * L2) + abs(o["E1"] - L1 * rate) / big + abs(o["E2"] - o["M"] * rate) / big
            + abs(o["L_aiding"] + o["L_opposing"] - 2.0 * (L1 + L2)) / (L1 + L2) + abs(o["L_aiding"] - o["L_opposing"] - 4.0 * o["M"]) / (L1 + L2))


def _reference_mutual(L1, L2, k, dI1, dt):
    """INDEPENDENT reference (weak for M itself): M = k times the geometric mean of L1 and L2 (scipy.stats.gmean);
    the emfs as the inductance matrix [[L1, M], [M, L2]] times the current rates (numpy), and the series inductances
    as the quadratic forms of that matrix with (1, 1) and (1, -1)."""
    import numpy as np
    from scipy import stats
    m = k * float(stats.gmean([L1, L2]))
    Lm = np.array([[L1, m], [m, L2]])
    v = Lm @ np.array([abs(dI1) / dt, 0.0])
    a, b = np.array([1.0, 1.0]), np.array([1.0, -1.0])
    return {"M": m, "E1": float(v[0]), "E2": float(v[1]), "L_aiding": float(a @ Lm @ a), "L_opposing": float(b @ Lm @ b)}


PROCEDURES["mutual_inductance"] = _fspec(
    "Mutual inductance, induced emfs and coupled coils in series", "electromagnetic induction / mutual inductance",
    "The mutual inductance M = k sqrt(L1 L2) of two coupled coils, the emfs induced in coil 1 and in coil 2 when the current in coil 1 changes by dI1 in dt, and the inductance of the two coils in series aiding and opposing.",
    [_inp("L1", "inductance", "self-inductance of coil 1"), _inp("L2", "inductance", "self-inductance of coil 2"),
     _inp("k", "number", "coupling coefficient (0 to 1)"), _inp("dI1", "current", "change of the current in coil 1"),
     _inp("dt", "time", "time taken by the change")],
    [_out("M", "inductance", "mutual inductance"), _out("E1", "voltage", "magnitude of the emf self-induced in coil 1"),
     _out("E2", "voltage", "magnitude of the emf induced in coil 2"), _out("L_aiding", "inductance", "series aiding L1 + L2 + 2M"),
     _out("L_opposing", "inductance", "series opposing L1 + L2 - 2M")],
    "M", ["linear coils", "a uniform rate of change of current (average emfs)", "0 <= k <= 1"],
    "M = k sqrt(L1 L2); E1 = L1 dI1/dt; E2 = M dI1/dt; L = L1 + L2 +/- 2M", r"M = k\sqrt{L_1L_2},\quad E_2 = M\frac{\Delta I_1}{\Delta t}",
    mutual_inductance, _verify_mutual, _reference_mutual,
    lambda L1, L2, k, dI1, dt: (lambda m, rate: {"M": m, "E1": L1 * rate, "E2": m * rate, "L_aiding": L1 + L2 + 2 * m, "L_opposing": L1 + L2 - 2 * m})(
        k * (L1 + L2) / 2.0, abs(dI1) / dt),                                                  # arithmetic instead of geometric mean
    lambda rng: (lambda L1: {"L1": L1, "L2": round(L1 * rng.choice([2.0, 3.0, 4.0, 5.0, 10.0]), 12), "k": round(rng.uniform(0.1, 0.95), 3),
                             "dI1": round(rng.uniform(-10, 10), 2) or 1.0, "dt": round(rng.uniform(1, 100), 1) * 1e-3})(_e12(rng, -3, -1)),
    r"\bmutual inductance\b",
    ["Find the mutual inductance with L1 = 0.2 H, L2 = 0.8 H, k = 0.5, dI1 = 4 A, dt = 20 ms."],
    [{"inputs": {"L1": 0.2, "L2": 0.8, "k": 0.5, "dI1": 4, "dt": 0.02}, "outputs": {"M": 0.2, "E1": 40.0, "E2": 40.0, "L_aiding": 1.4, "L_opposing": 0.6}},
     {"inputs": {"L1": 0.01, "L2": 0.04, "k": 0.9, "dI1": 2, "dt": 1e-3}, "outputs": {"M": 0.018, "E1": 20.0, "E2": 36.0, "L_aiding": 0.086, "L_opposing": 0.014}}],
    [{"q": "Find the mutual inductance with L1 = 0.2 H, L2 = 0.8 H, k = 0.5, dI1 = 4 A, dt = 20 ms.", "outputs": {"M": 0.2, "E2": 40.0}},
     {"q": "Find the mutual inductance with L1 = 10 mH, L2 = 40 mH, k = 0.9, dI1 = 2 A, dt = 1 ms.", "outputs": {"M": 0.018, "E2": 36.0}},
     {"q": "Find the mutual inductance with L1 = 0.2 H, L2 = 0.8 H, k = 0.5, dI1 = 4 A.", "outcome": "NEEDS_INPUT"},
     {"q": "Find the mutual inductance with L1 = 0.2 H, L2 = 0.8 H, k = 1.2, dI1 = 4 A, dt = 20 ms.", "outcome": "INVALID_INPUT"}])


# --------------------------------------------------------------------------------------------- measuring instruments
def ammeter_shunt(Ifs, Rm, I) -> dict:
    _check_positive(Ifs=Ifs, Rm=Rm, I=I)
    if I <= Ifs:
        raise ValueError("the new range must be larger than the meter's full-scale current")
    vfs = Ifs * Rm
    rsh = vfs / (I - Ifs)
    return {"Rsh": rsh, "multiplying_factor": I / Ifs, "V_fs": vfs, "R_total": vfs / I}


def _verify_ammeter(i, o):
    Ifs, Rm, I = i["Ifs"], i["Rm"], i["I"]
    return (_rel(Ifs * Rm, (I - Ifs) * o["Rsh"]) + _rel(o["multiplying_factor"] * Ifs, I) + _rel(o["V_fs"], Ifs * Rm)
            + _rel(o["R_total"] * (Rm + o["Rsh"]), Rm * o["Rsh"]))


def _reference_ammeter(Ifs, Rm, I):
    """INDEPENDENT reference: the meter and a trial shunt in parallel, fed by the current I, solved by modified nodal
    analysis; scipy.optimize.brentq finds the shunt that leaves exactly the full-scale current in the meter."""
    from scipy import optimize

    def solve(rs):
        return _mna([("I", "I1", "0", "1", I), ("R", "Rm", "1", "0", Rm), ("R", "Rs", "1", "0", rs)])[0]
    rs = optimize.brentq(lambda x: solve(x)["1"] / Rm - Ifs, Rm * 1e-12, Rm * 1e6, xtol=1e-300, rtol=8.9e-16, maxiter=500)
    v = solve(rs)["1"]
    return {"Rsh": float(rs), "multiplying_factor": float(I * Rm / v), "V_fs": float(v), "R_total": float(v / I)}


PROCEDURES["ammeter_shunt"] = _fspec(
    "Ammeter shunt resistance", "measuring instruments / range extension",
    "The shunt resistance that extends a moving-coil meter of full-scale current Ifs and resistance Rm to read a current I at full scale.",
    [_inp("Ifs", "current", "full-scale deflection current of the meter"), _inp("Rm", "resistance", "resistance of the meter"),
     _inp("I", "current", "required full-scale reading of the ammeter")],
    [_out("Rsh", "resistance", "shunt resistance"), _out("multiplying_factor", "number", "range multiplication I/Ifs"),
     _out("V_fs", "voltage", "voltage across the meter at full scale"), _out("R_total", "resistance", "resistance of the extended ammeter")],
    "Rsh", ["I > Ifs", "the meter's resistance constant"], "Rsh = Ifs Rm / (I - Ifs)", r"R_{sh} = \frac{I_{fs}R_m}{I - I_{fs}}",
    ammeter_shunt, _verify_ammeter, _reference_ammeter,
    lambda Ifs, Rm, I: (lambda rs: {"Rsh": rs, "multiplying_factor": I / Ifs, "V_fs": Ifs * Rm, "R_total": Rm * rs / (Rm + rs)})(Rm * (I / Ifs - 1.0)),   # multiplier formula
    lambda rng: (lambda ifs: {"Ifs": ifs, "Rm": _e12(rng, 0, 3), "I": round(ifs * rng.choice([10, 20, 50, 100, 200, 1000]), 9)})(
        rng.choice([50e-6, 100e-6, 1e-3, 5e-3, 10e-3])),
    r"\bammeter\b",
    ["Find the shunt for an ammeter with Ifs = 1 mA, Rm = 100 ohm, I = 1 A."],
    [{"inputs": {"Ifs": 1e-3, "Rm": 100, "I": 1}, "outputs": {"Rsh": 0.1 / 0.999, "multiplying_factor": 1000.0, "V_fs": 0.1, "R_total": 0.1}},
     {"inputs": {"Ifs": 0.01, "Rm": 5, "I": 2}, "outputs": {"Rsh": 0.05 / 1.99, "R_total": 0.025}}],
    [{"q": "Find the shunt for an ammeter with Ifs = 1 mA, Rm = 100 ohm, I = 1 A.", "outputs": {"Rsh": 0.1 / 0.999}},
     {"q": "Find the shunt for an ammeter with Ifs = 10 mA, Rm = 5 ohm, I = 2 A.", "outputs": {"Rsh": 0.05 / 1.99}},
     {"q": "Find the shunt for an ammeter with Ifs = 1 mA, I = 1 A.", "outcome": "NEEDS_INPUT"},
     {"q": "Find the shunt for an ammeter with Ifs = 1 mA, Rm = 100 ohm, I = 0.5 mA.", "outcome": "INVALID_INPUT"}],
    forbid=r"\bvoltmeter\b")


def voltmeter_multiplier(Ifs, Rm, V) -> dict:
    _check_positive(Ifs=Ifs, Rm=Rm, V=V)
    vfs = Ifs * Rm
    if V <= vfs:
        raise ValueError("the new range must be larger than the meter's full-scale voltage Ifs Rm")
    total = V / Ifs
    return {"Rmult": total - Rm, "R_total": total, "sensitivity": 1.0 / Ifs, "V_fs": vfs}


def _verify_voltmeter(i, o):
    Ifs, Rm, V = i["Ifs"], i["Rm"], i["V"]
    return (_rel(Ifs * (Rm + o["Rmult"]), V) + _rel(o["R_total"], Rm + o["Rmult"]) + _rel(o["sensitivity"] * Ifs, 1.0) + _rel(o["V_fs"], Ifs * Rm))


def _reference_voltmeter(Ifs, Rm, V):
    """INDEPENDENT reference: the series loop (multiplier Rx and meter Rm across the measured voltage V) carries
    I(Rx) = V/(Rx + Rm) by Kirchhoff's voltage law; scipy.optimize.brentq finds the Rx at which that loop current is
    exactly the full-scale current. (A nodal solve was ill-conditioned at the tiny lower bracket and was replaced.)"""
    from scipy import optimize

    def meter(rx):
        return V / (rx + Rm)
    rx = optimize.brentq(lambda x: meter(x) - Ifs, 0.0, 2.0 * V / Ifs, xtol=1e-300, rtol=8.9e-16, maxiter=500)
    cur = meter(rx)
    return {"Rmult": float(rx), "R_total": float(V / cur), "sensitivity": float(V / cur / V), "V_fs": float(cur * Rm)}


PROCEDURES["voltmeter_multiplier"] = _fspec(
    "Voltmeter multiplier resistance", "measuring instruments / range extension",
    "The series (multiplier) resistance that makes a moving-coil meter of full-scale current Ifs and resistance Rm read a voltage V at full scale, and the voltmeter's sensitivity.",
    [_inp("Ifs", "current", "full-scale deflection current of the meter"), _inp("Rm", "resistance", "resistance of the meter"),
     _inp("V", "voltage", "required full-scale reading of the voltmeter")],
    [_out("Rmult", "resistance", "multiplier (series) resistance"), _out("R_total", "resistance", "resistance of the voltmeter"),
     _out("sensitivity", "number", "sensitivity in ohm per volt, 1/Ifs"), _out("V_fs", "voltage", "full-scale voltage of the bare meter")],
    "Rmult", ["V > Ifs Rm"], "Rmult = V/Ifs - Rm", r"R_{mult} = \frac{V}{I_{fs}} - R_m",
    voltmeter_multiplier, _verify_voltmeter, _reference_voltmeter,
    lambda Ifs, Rm, V: (lambda rx: {"Rmult": rx, "R_total": rx + Rm, "sensitivity": 1.0 / Ifs, "V_fs": Ifs * Rm})(Rm / (V / (Ifs * Rm) - 1.0)),   # shunt formula
    lambda rng: (lambda ifs, rm: {"Ifs": ifs, "Rm": rm, "V": float(f"{ifs * rm * rng.choice([5, 10, 50, 100, 1000]):.6g}")})(
        rng.choice([50e-6, 100e-6, 1e-3, 5e-3, 10e-3]), _e12(rng, 0, 3)),
    r"\bvoltmeter\b",
    ["Find the multiplier for a voltmeter with Ifs = 1 mA, Rm = 100 ohm, V = 10 V."],
    [{"inputs": {"Ifs": 1e-3, "Rm": 100, "V": 10}, "outputs": {"Rmult": 9900.0, "R_total": 10000.0, "sensitivity": 1000.0, "V_fs": 0.1}},
     {"inputs": {"Ifs": 50e-6, "Rm": 2000, "V": 100}, "outputs": {"Rmult": 1998000.0, "sensitivity": 20000.0}}],
    [{"q": "Find the multiplier for a voltmeter with Ifs = 1 mA, Rm = 100 ohm, V = 10 V.", "outputs": {"Rmult": 9900.0}},
     {"q": "Find the multiplier for a voltmeter with Ifs = 50 uA, Rm = 2 kohm, V = 100 V.", "outputs": {"Rmult": 1998000.0, "sensitivity": 20000.0}},
     {"q": "Find the multiplier for a voltmeter with Ifs = 1 mA, V = 10 V.", "outcome": "NEEDS_INPUT"},
     {"q": "Find the multiplier for a voltmeter with Ifs = 1 mA, Rm = 100 ohm, V = 0.05 V.", "outcome": "INVALID_INPUT"}],
    forbid=r"\bammeter\b")


# --------------------------------------------------------------------------------------------- sinusoidal waveforms
def _check_phase(phi):
    if not (math.isfinite(phi) and abs(phi) <= 2.0 * math.pi):
        raise ValueError("the phase angle must be between -2 pi and 2 pi rad")


def sinusoid_values(Vm, w, phi, t) -> dict:
    _check_positive(Vm=Vm, w=w)
    _check_phase(phi)
    if not math.isfinite(t):
        raise ValueError("the time must be a finite number")
    rel = "in phase" if phi == 0 else ("leads" if phi > 0 else "lags")
    return {"v": Vm * math.sin(w * t + phi), "f": w / _TWO_PI, "T": _TWO_PI / w, "phase_deg": math.degrees(phi) + 0.0,
            "relation": rel, "time_shift": phi / w + 0.0, "rms": Vm / math.sqrt(2.0), "average": 2.0 * Vm / math.pi,
            "form_factor": math.pi / (2.0 * math.sqrt(2.0)), "peak_factor": math.sqrt(2.0)}


def _verify_sinusoid(i, o):
    Vm, w, phi, t = i["Vm"], i["w"], i["phi"], i["t"]
    want = "in phase" if phi == 0 else ("leads" if phi > 0 else "lags")
    return (abs(o["v"] - Vm * math.sin(w * t + phi)) / Vm + _rel(o["f"] * _TWO_PI, w) + _rel(o["T"] * o["f"], 1.0)
            + abs(math.radians(o["phase_deg"]) - phi) / max(1.0, abs(phi)) + abs(o["time_shift"] * w - phi) / max(1.0, abs(phi))
            + _rel(o["rms"] * o["peak_factor"], Vm) + _rel(o["peak_factor"] ** 2, 2.0) + _rel(o["average"] * math.pi, 2.0 * Vm)
            + _rel(o["form_factor"] * o["average"], o["rms"]) + float(o["relation"] != want))


def _reference_sinusoid(Vm, w, phi, t):
    """INDEPENDENT reference: the rms and the half-cycle average from the mean of v^2 and of |v| over one period
    (scipy.integrate.quad, split at the zeros); the value at t as the imaginary part of the phasor Vm e^(j(wt + phi))
    (numpy); the time shift as minus the rising zero crossing found by scipy.optimize.brentq."""
    import numpy as np
    from scipy import integrate, optimize
    T = 2 * np.pi / w
    zeros = sorted(x for x in ((k * np.pi - phi) / w for k in range(-3, 6)) if 0 < x < T)
    kw = dict(epsabs=0, epsrel=1e-13, limit=200, points=zeros or None)
    sq, _ = integrate.quad(lambda x: (Vm * np.sin(w * x + phi)) ** 2, 0.0, T, **kw)
    ab, _ = integrate.quad(lambda x: abs(Vm * np.sin(w * x + phi)), 0.0, T, **kw)
    rms, avg = (sq / T) ** 0.5, ab / T
    t0 = optimize.brentq(lambda x: np.sin(w * x + phi), -phi / w - T / 4, -phi / w + T / 4, xtol=1e-300, rtol=8.9e-16, maxiter=500)
    return {"v": float((Vm * np.exp(1j * (w * t + phi))).imag), "f": float(1 / T), "T": float(T), "rms": float(rms), "average": float(avg),
            "form_factor": float(rms / avg), "peak_factor": float(Vm / rms), "time_shift": float(-t0)}


def _wrong_sinusoid(Vm, w, phi, t):                     # the phase angle dropped from the value, the shift not divided by w
    out = sinusoid_values(Vm, w, phi, t)
    out["v"] = Vm * math.sin(w * t)
    out["time_shift"] = phi
    return out


def _random_sinusoid(rng):
    return {"Vm": round(rng.uniform(1, 400), 1), "w": round(_TWO_PI * rng.choice([50, 60, 100, 400, 1000]), 3),
            "phi": round(rng.choice([-1, 1]) * rng.uniform(0.2, 1.5), 3), "t": round(rng.uniform(0.0, 0.05), 5)}


_SIN_IN = [_inp("Vm", "voltage", "amplitude (peak value)"), _inp("w", "angular_frequency", "angular frequency omega"),
           _inp("phi", "number", "phase angle in radians (positive: leads Vm sin wt)")]

PROCEDURES["sinusoid_values"] = _fspec(
    "Sinusoidal waveform v = Vm sin(wt + phi): frequency, period, phase, value at a time, form and peak factors", "alternating voltages and currents / sinusoids",
    "For v = Vm sin(wt + phi): the frequency, period, phase angle in degrees and whether it leads or lags Vm sin wt, the time shift, the value at a time t, the rms and half-cycle average values and the form and peak factors.",
    _SIN_IN + [_inp("t", "time", "time at which to evaluate v")],
    [_out("v", "voltage", "instantaneous value at t"), _out("f", "frequency", "frequency w/(2 pi)"), _out("T", "time", "period"),
     _out("phase_deg", "number", "phase angle in degrees"), _out("relation", "text", "leads, lags or in phase (relative to Vm sin wt)"),
     _out("time_shift", "time", "time shift phi/w (positive: leads)"), _out("rms", "voltage", "rms value Vm/sqrt(2)"),
     _out("average", "voltage", "average over a half cycle, 2 Vm/pi"), _out("form_factor", "number", "form factor rms/average"),
     _out("peak_factor", "number", "peak factor Vm/rms")],
    "v", ["a pure sinusoid", "|phi| <= 2 pi rad"], "v = Vm sin(wt + phi); f = w/(2 pi); T = 1/f; Vrms = Vm/sqrt 2; Vav = 2 Vm/pi",
    r"v = V_m\sin(\omega t + \phi),\quad f = \frac{\omega}{2\pi},\quad V_{rms} = \frac{V_m}{\sqrt 2}",
    sinusoid_values, _verify_sinusoid, _reference_sinusoid, _wrong_sinusoid, _random_sinusoid,
    r"\bsinusoid(?:al)?\b",
    ["Find the values of the sinusoid v = Vm sin(wt + phi) with Vm = 200 V, w = 314.159 rad/s, phi = 0, t = 2.5 ms."],
    [{"inputs": {"Vm": 200, "w": 100 * math.pi, "phi": 0, "t": 0.0025},
      "outputs": {"v": 100 * math.sqrt(2), "f": 50.0, "T": 0.02, "relation": "in phase", "rms": 100 * math.sqrt(2), "average": 400 / math.pi,
                  "form_factor": math.pi / (2 * math.sqrt(2)), "peak_factor": math.sqrt(2)}},
     {"inputs": {"Vm": 10, "w": 1000, "phi": -0.5, "t": 0.002},
      "outputs": {"v": 10 * math.sin(1.5), "relation": "lags", "time_shift": -0.0005, "phase_deg": math.degrees(-0.5)}}],
    [{"q": "Find the values of the sinusoid v = Vm sin(wt + phi) with Vm = 200 V, w = 314.159 rad/s, phi = 0, t = 2.5 ms.",
      "outputs": {"v": 200 * math.sin(314.159 * 0.0025), "f": 314.159 / (2 * math.pi)}},
     {"q": "Find the values of the sinusoidal voltage with Vm = 10 V, w = 1000 rad/s, phi = -0.5, t = 2 ms.", "outputs": {"v": 10 * math.sin(1.5)}},
     {"q": "Find the values of the sinusoid with Vm = 10 V, w = 1000 rad/s, t = 2 ms.", "outcome": "NEEDS_INPUT"},
     {"q": "Find the values of the sinusoid with Vm = -5 V, w = 1000 rad/s, phi = 0, t = 2 ms.", "outcome": "INVALID_INPUT"}],
    forbid=r"\bfirst reach|\brms\b")


def sinusoid_time_to_value(Vm, w, phi, v) -> dict:
    _check_positive(Vm=Vm, w=w)
    _check_phase(phi)
    if not (math.isfinite(v) and abs(v) <= Vm):
        raise ValueError("a sinusoid never reaches a value beyond its amplitude")
    r = v / Vm
    if r > 1.0:
        r = 1.0
    if r < -1.0:
        r = -1.0
    a = math.asin(r)
    best = 10.0
    for th in [a, math.pi - a]:
        x = th - phi
        while x < 0:
            x += _TWO_PI
        while x >= _TWO_PI:
            x -= _TWO_PI
        if x <= 1e-12 or _TWO_PI - x <= 1e-12:
            x = 0.0
        if x < best:
            best = x
    slope = math.cos(best + phi)
    direction = "rising" if slope > 1e-9 else ("falling" if slope < -1e-9 else "peak")
    return {"t": best / w, "T": _TWO_PI / w, "direction": direction}


def _verify_sin_time(i, o):
    """The value is reached at t, t lies in the first period, v(s) - v keeps one sign on [0, t) (no earlier
    crossing, checked at 200 points) and the stated direction matches the slope."""
    Vm, w, phi, v = i["Vm"], i["w"], i["phi"], i["v"]
    t = o["t"]
    T = _TWO_PI / w
    res = abs(Vm * math.sin(w * t + phi) - v) / Vm + _rel(o["T"], T) + float(t < 0 or t >= T)
    first = Vm * math.sin(phi) - v
    for k in range(1, 200):
        g = Vm * math.sin(w * t * k / 200.0 + phi) - v
        if g * first < 0 and abs(g) > 1e-9 * Vm and abs(first) > 1e-9 * Vm:
            res += 1.0
            break
    c = math.cos(w * t + phi)
    want = "rising" if c > 1e-9 else ("falling" if c < -1e-9 else "peak")
    return res + float(o["direction"] != want)


def _reference_sin_time(Vm, w, phi, v):
    """INDEPENDENT reference: v(t) - v sampled at 4001 points of the first period, the first sign change bracketed and
    refined by scipy.optimize.brentq."""
    import numpy as np
    from scipy import optimize
    T = 2 * np.pi / w
    g = lambda x: Vm * np.sin(w * x + phi) - v                                  # noqa: E731
    if g(0.0) == 0:
        t = 0.0
    else:
        grid = np.linspace(0.0, T, 4001)
        vals = g(grid)
        j = int(np.argmax(np.sign(vals[1:]) != np.sign(vals[0])) + 1)
        t = 0.0 + optimize.brentq(g, grid[j - 1], grid[j], xtol=1e-300, rtol=8.9e-16, maxiter=500) if vals[j] != 0 else float(grid[j])
    c = np.cos(w * t + phi)
    return {"t": float(t), "T": float(T), "direction": "rising" if c > 1e-9 else ("falling" if c < -1e-9 else "peak")}


PROCEDURES["sinusoid_time_to_value"] = _fspec(
    "Time at which a sinusoid first reaches a value", "alternating voltages and currents / sinusoids",
    "The first time t >= 0 at which v = Vm sin(wt + phi) equals a given value, the period, and whether v is rising or falling there.",
    _SIN_IN + [_inp("v", "voltage", "value to reach (|v| <= Vm)")],
    [_out("t", "time", "first time at which v is reached"), _out("T", "time", "period"), _out("direction", "text", "rising, falling or peak at that time")],
    "t", ["|v| <= Vm", "|phi| <= 2 pi rad", "t counted from 0"], "wt + phi = asin(v/Vm) or pi - asin(v/Vm), reduced to the first period; the smaller time",
    r"\omega t + \phi = \sin^{-1}\frac{v}{V_m}\ \text{or}\ \pi - \sin^{-1}\frac{v}{V_m}",
    sinusoid_time_to_value, _verify_sin_time, _reference_sin_time,
    lambda Vm, w, phi, v: sinusoid_time_to_value(Vm, w, 0.0, v),                  # the phase angle ignored
    lambda rng: (lambda b: dict(b, v=round(b["Vm"] * rng.uniform(-0.95, 0.95), 3)))({k: x for k, x in _random_sinusoid(rng).items() if k != "t"}),
    r"(?=.*\bsinusoid)(?=.*\bfirst reach)",
    ["When does the sinusoid with Vm = 100 V, w = 314.159 rad/s, phi = 0 first reach v = 50 V?"],
    [{"inputs": {"Vm": 100, "w": 100 * math.pi, "phi": 0, "v": 50}, "outputs": {"t": 1 / 600, "T": 0.02, "direction": "rising"}},
     {"inputs": {"Vm": 10, "w": 1000, "phi": 0.5, "v": 0}, "outputs": {"t": (math.pi - 0.5) / 1000, "direction": "falling"}}],
    [{"q": "When does the sinusoid with Vm = 100 V, w = 314.159 rad/s, phi = 0 first reach v = 50 V?", "outputs": {"t": math.asin(0.5) / 314.159}},
     {"q": "When does the sinusoid with Vm = 10 V, w = 1000 rad/s, phi = 0.5 first reach v = 0 V?", "outputs": {"t": (math.pi - 0.5) / 1000}},
     {"q": "When does the sinusoid with Vm = 100 V, w = 314.159 rad/s, phi = 0 first reach v = 150 V?", "outcome": "INVALID_INPUT"}])


# --------------------------------------------------------------------------------------------- a.c. circuits with a supply
def _ac_check(V, f, R, circuit, L, C):
    _check_positive(V=V, f=f, R=R)
    if not (math.isfinite(L) and math.isfinite(C)) or L < 0 or C < 0:
        raise ValueError("L and C must not be negative")
    if circuit == "RL" and not (L > 0 and C == 0):
        raise ValueError("an R-L circuit needs an inductance L > 0 and no capacitor")
    if circuit == "RC" and not (C > 0 and L == 0):
        raise ValueError("an R-C circuit needs a capacitance C > 0 and no inductor")
    if circuit == "RLC" and not (L > 0 and C > 0):
        raise ValueError("an R-L-C circuit needs both L > 0 and C > 0")


def series_ac_supply(V, f, R, circuit, L=0.0, C=0.0) -> dict:
    _ac_check(V, f, R, circuit, L, C)
    w = _TWO_PI * f
    xl = w * L if L > 0 else 0.0
    xc = 1.0 / (w * C) if C > 0 else 0.0
    x = xl - xc
    if abs(x) <= 1e-12 * max(xl, xc):          # at resonance the net reactance is exactly 0, not rounding residue
        x = 0.0
    z = math.hypot(R, x)
    cur = V / z
    nature = "unity" if x == 0 else ("lagging" if x > 0 else "leading")
    return {"Z": z, "I": cur, "VR": cur * R, "VL": cur * xl, "VC": cur * xc, "XL": xl, "XC": xc,
            "phase_deg": math.degrees(math.atan2(x, R)) + 0.0, "power_factor": R / z, "P": cur * cur * R, "Q": cur * cur * x + 0.0,
            "S": V * cur, "nature": nature}


def _verify_series_ac(i, o):
    """Kirchhoff's voltage law with phasors (VR and VL - VC in quadrature add to V), each element's law, the power
    triangle and the phase angle."""
    V, R = i["V"], i["R"]
    L, C = i.get("L", 0.0), i.get("C", 0.0)
    w = _TWO_PI * i["f"]
    cur, S = o["I"], o["S"]
    xc = 1.0 / (w * C) if C > 0 else 0.0
    d = o["VL"] - o["VC"]
    want = "lagging" if d > 1e-12 * V else ("leading" if d < -1e-12 * V else "unity")
    return (abs(math.hypot(o["VR"], d) - V) / V + _rel(o["VR"], cur * R) + abs(o["VL"] - cur * w * L) / V + abs(o["VC"] - cur * xc) / V
            + _rel(o["Z"] * cur, V) + abs(o["XL"] - w * L) / o["Z"] + abs(o["XC"] - xc) / o["Z"] + _rel(S, V * cur)
            + abs(o["P"] - V * cur * o["power_factor"]) / S + abs(o["P"] ** 2 + o["Q"] ** 2 - S * S) / (S * S)
            + abs(o["power_factor"] - math.cos(math.radians(o["phase_deg"]))) + abs(o["Q"] - cur * d) / S + float(o["nature"] != want))


def _reference_series_ac(V, f, R, circuit, L=0.0, C=0.0):
    """INDEPENDENT reference: the circuit's admittance as a rational function of s (R-L: 1/(Ls + R); R-C:
    Cs/(RCs + 1); R-L-C: Cs/(LCs^2 + RCs + 1)) evaluated on the jw axis by scipy.signal.freqs, the element voltages
    from the current phasor (numpy), and P and Q as averages of v(t) i(t) over one period (512 samples)."""
    import numpy as np
    from scipy import signal
    w = 2 * np.pi * f
    if circuit == "RL":
        b, a = [1.0], [L, R]
    elif circuit == "RC":
        b, a = [C, 0.0], [R * C, 1.0]
    else:
        b, a = [C, 0.0], [L * C, R * C, 1.0]
    Y = complex(signal.freqs(b, a, worN=[w])[1][0])
    Iph = V * Y
    cur = abs(Iph)
    ph = -float(np.angle(Iph))
    n = 512
    th = 2 * np.pi * np.arange(n) / n
    v = np.sqrt(2) * V * np.cos(th)
    P = float(np.mean(v * np.sqrt(2) * cur * np.cos(th - ph)))
    Q = float(np.mean(v * np.sqrt(2) * cur * np.cos(th - ph + np.pi / 2)))
    return {"Z": 1 / abs(Y), "I": cur, "VR": abs(Iph * R), "VL": abs(Iph * 1j * w * L), "VC": abs(Iph / (1j * w * C)) if C > 0 else 0.0,
            "phase_deg": float(np.degrees(ph)), "power_factor": P / (V * cur), "P": P, "Q": Q, "S": V * cur}


def parallel_ac_supply(V, f, R, circuit, L=0.0, C=0.0) -> dict:
    _ac_check(V, f, R, circuit, L, C)
    w = _TWO_PI * f
    ir = V / R
    il = V / (w * L) if L > 0 else 0.0
    ic = V * w * C if C > 0 else 0.0
    b = il - ic
    if abs(b) <= 1e-12 * max(il, ic):          # at resonance the net reactive current is exactly 0
        b = 0.0
    cur = math.hypot(ir, b)
    nature = "unity" if b == 0 else ("lagging" if b > 0 else "leading")
    return {"IR": ir, "IL": il, "IC": ic, "I": cur, "Z": V / cur, "phase_deg": math.degrees(math.atan2(b, ir)) + 0.0,
            "power_factor": ir / cur, "P": V * ir, "Q": V * b + 0.0, "S": V * cur, "nature": nature}


def _verify_parallel_ac(i, o):
    """Kirchhoff's current law with phasors (IR and IL - IC in quadrature add to I), each branch's law, the power
    triangle and the phase angle."""
    V, R = i["V"], i["R"]
    L, C = i.get("L", 0.0), i.get("C", 0.0)
    w = _TWO_PI * i["f"]
    cur, S = o["I"], o["S"]
    il = V / (w * L) if L > 0 else 0.0
    d = o["IL"] - o["IC"]
    want = "lagging" if d > 1e-12 * cur else ("leading" if d < -1e-12 * cur else "unity")
    return (abs(math.hypot(o["IR"], d) - cur) / cur + _rel(o["IR"] * R, V) + abs(o["IL"] - il) / cur + abs(o["IC"] - V * w * C) / cur
            + _rel(o["Z"] * cur, V) + _rel(S, V * cur) + abs(o["P"] - V * cur * o["power_factor"]) / S
            + abs(o["P"] ** 2 + o["Q"] ** 2 - S * S) / (S * S) + abs(o["power_factor"] - math.cos(math.radians(o["phase_deg"])))
            + abs(o["Q"] - V * d) / S + float(o["nature"] != want))


def _reference_parallel_ac(V, f, R, circuit, L=0.0, C=0.0):
    """INDEPENDENT reference: the branch admittances 1/R, 1/(jwL) and jwC added in complex arithmetic (numpy), the
    branch and supply current phasors from them, and P and Q as averages of v(t) i(t) over one period."""
    import numpy as np
    w = 2 * np.pi * f
    yl = 1 / (1j * w * L) if L > 0 else 0j
    yc = 1j * w * C if C > 0 else 0j
    Iph = V * (1 / R + yl + yc)
    cur = abs(Iph)
    ph = -float(np.angle(Iph))
    n = 512
    th = 2 * np.pi * np.arange(n) / n
    v = np.sqrt(2) * V * np.cos(th)
    P = float(np.mean(v * np.sqrt(2) * cur * np.cos(th - ph)))
    Q = float(np.mean(v * np.sqrt(2) * cur * np.cos(th - ph + np.pi / 2)))
    return {"IR": V / R, "IL": abs(V * yl), "IC": abs(V * yc), "I": cur, "Z": V / cur, "phase_deg": float(np.degrees(ph)),
            "power_factor": P / (V * cur), "P": P, "Q": Q, "S": V * cur}


def _random_ac_supply(rng):
    c = rng.choice(["RL", "RC", "RLC"])
    out = {"V": rng.choice([12.0, 24.0, 100.0, 120.0, 230.0, 240.0]), "f": rng.choice([50.0, 60.0, 400.0, 1000.0]), "R": _e12(rng, 0, 3), "circuit": c}
    if c != "RC":
        out["L"] = _e12(rng, -3, -1)
    if c != "RL":
        out["C"] = _e12(rng, -7, -5)
    return out


_AC_IN = [_inp("V", "voltage", "rms supply voltage"), _inp("f", "frequency", "supply frequency"), _inp("R", "resistance", "resistance"),
          {"symbol": "circuit", "type": "choice", "choices": ["RL", "RC", "RLC"], "required": True, "meaning": "the elements: R-L, R-C or R-L-C"},
          _inp("L", "inductance", "inductance (R-L and R-L-C circuits)", required=False),
          _inp("C", "capacitance", "capacitance (R-C and R-L-C circuits)", required=False)]
_AC_KV = [x for x in _AC_IN if x["symbol"] != "circuit"]


def _ac_supply_parse(arrangement):
    pat = rf"\b{arrangement}\s+(R-L-C|R-L|R-C)\b"

    def parse(question):
        found = {m.upper() for m in re.findall(pat, question, re.I)}
        if len(found) != 1 or not re.search(r"\bsupply\b", question, re.I) or re.search(r"\bstep\b", question, re.I):
            return None
        rest = re.sub(pat, arrangement, question, flags=re.I)
        base = _kv_parse(r"\bsupply\b", _AC_KV)(rest)
        if base is None:
            return None
        base["circuit"] = next(iter(found)).replace("-", "")
        return base
    return parse


_AC_COMMON_OUT = [_out("phase_deg", "number", "phase angle of the supply current behind the voltage, degrees (negative: leading)"),
                  _out("power_factor", "number", "power factor cos(phase)"), _out("P", "power", "real power in W"),
                  _out("Q", "number", "reactive power in var (positive: inductive)"), _out("S", "number", "apparent power in VA"),
                  _out("nature", "text", "lagging, leading or unity power factor")]

PROCEDURES["series_ac_supply"] = {
    "name": "Series R-L, R-C or R-L-C circuit on an a.c. supply", "topic": "single-phase a.c. circuits / series circuits",
    "purpose": "The impedance, current, voltage across each element, phase angle, power factor and real, reactive and apparent power of a series R-L, R-C or R-L-C circuit connected to a sinusoidal supply.",
    "inputs": _AC_IN,
    "outputs": [_out("Z", "resistance", "impedance |Z|"), _out("I", "current", "rms current"), _out("VR", "voltage", "voltage across R"),
                _out("VL", "voltage", "voltage across L (0 without an inductor)"), _out("VC", "voltage", "voltage across C (0 without a capacitor)"),
                _out("XL", "resistance", "inductive reactance wL"), _out("XC", "resistance", "capacitive reactance 1/(wC)")] + _AC_COMMON_OUT,
    "primary": "I", "conditions": ["sinusoidal steady state, rms values", "ideal L and C", "R > 0"],
    "method": "Z = sqrt(R^2 + (XL - XC)^2); I = V/Z; VR = IR, VL = I XL, VC = I XC; phase = atan((XL - XC)/R); P = I^2 R",
    "latex": r"Z = \sqrt{R^2 + (X_L - X_C)^2},\quad I = \frac{V}{Z},\quad \cos\phi = \frac{R}{Z}",
    "run": series_ac_supply, "verify": _verify_series_ac, "parse": _ac_supply_parse("series"), "reference": _reference_series_ac,
    "wrong": lambda V, f, R, circuit, L=0.0, C=0.0: series_ac_supply(V, f / _TWO_PI, R, circuit, L, C),   # f used as w
    "random_inputs": _random_ac_supply,
    "templates": ["A series R-L circuit with R = 30 ohm and L = 0.1 H is connected to a supply of V = 240 V, f = 50 Hz. Find the current and the power factor."],
    "examples": [{"inputs": {"V": 250, "f": 50, "R": 30, "circuit": "RL", "L": 40 / (2 * math.pi * 50)},
                  "outputs": {"Z": 50.0, "I": 5.0, "VR": 150.0, "VL": 200.0, "VC": 0.0, "power_factor": 0.6, "P": 750.0, "Q": 1000.0, "S": 1250.0,
                              "phase_deg": math.degrees(math.atan2(4, 3)), "nature": "lagging"}},
                 {"inputs": {"V": 100, "f": 50, "R": 8, "circuit": "RC", "C": 1 / (2 * math.pi * 50 * 6)},
                  "outputs": {"Z": 10.0, "I": 10.0, "VR": 80.0, "VC": 60.0, "power_factor": 0.8, "P": 800.0, "Q": -600.0, "nature": "leading"}},
                 {"inputs": {"V": 230, "f": 50, "R": 10, "circuit": "RLC", "L": 0.1, "C": 1e-4},
                  "outputs": {"I": 230 / math.hypot(10, 10 * math.pi - 100 / math.pi), "nature": "leading"}}],
    "questions": [{"q": "A series R-L circuit with R = 30 ohm and L = 0.1 H is connected to a supply of V = 240 V, f = 50 Hz. Find the current and the power factor.",
                   "outputs": {"I": 240 / math.hypot(30, 10 * math.pi), "power_factor": 30 / math.hypot(30, 10 * math.pi)}},
                  {"q": "A series R-C circuit with R = 8 ohm and C = 500 uF is connected to a supply of V = 100 V, f = 50 Hz. Find the current.",
                   "outputs": {"I": 100 / math.hypot(8, 20 / math.pi)}},
                  {"q": "A series R-L-C circuit with R = 10 ohm, L = 0.1 H and C = 100 uF is connected to a supply of V = 230 V, f = 50 Hz. Find the current.",
                   "outputs": {"I": 230 / math.hypot(10, 10 * math.pi - 100 / math.pi)}},
                  {"q": "A series R-L circuit with R = 30 ohm and L = 0.1 H is connected to a supply of V = 240 V. Find the current.", "outcome": "NEEDS_INPUT"},
                  {"q": "A series R-L circuit with R = 30 ohm, L = 0.1 H and C = 100 uF is connected to a supply of V = 240 V, f = 50 Hz. Find the current.",
                   "outcome": "INVALID_INPUT"}]}

PROCEDURES["parallel_ac_supply"] = {
    "name": "Parallel R-L, R-C or R-L-C circuit on an a.c. supply", "topic": "single-phase a.c. circuits / parallel circuits",
    "purpose": "The branch currents, supply current, impedance, phase angle, power factor and real, reactive and apparent power of R in parallel with L, with C, or with both, across a sinusoidal supply.",
    "inputs": _AC_IN,
    "outputs": [_out("IR", "current", "current in R"), _out("IL", "current", "current in L (0 without an inductor)"),
                _out("IC", "current", "current in C (0 without a capacitor)"), _out("I", "current", "supply current"),
                _out("Z", "resistance", "impedance V/I")] + _AC_COMMON_OUT,
    "primary": "I", "conditions": ["sinusoidal steady state, rms values", "ideal L and C (a coil with resistance: use coil_capacitor_parallel)", "R > 0"],
    "method": "IR = V/R, IL = V/(wL), IC = V wC; I = sqrt(IR^2 + (IL - IC)^2); phase = atan((IL - IC)/IR)",
    "latex": r"I = \sqrt{I_R^2 + (I_L - I_C)^2},\quad \cos\phi = \frac{I_R}{I}",
    "run": parallel_ac_supply, "verify": _verify_parallel_ac, "parse": _ac_supply_parse("parallel"), "reference": _reference_parallel_ac,
    "wrong": lambda V, f, R, circuit, L=0.0, C=0.0: parallel_ac_supply(V, f / _TWO_PI, R, circuit, L, C),   # f used as w
    "random_inputs": _random_ac_supply,
    "templates": ["A parallel R-L circuit with R = 25 ohm and L = 0.1 H is connected to a supply of V = 100 V, f = 50 Hz. Find the supply current."],
    "examples": [{"inputs": {"V": 100, "f": 50, "R": 25, "circuit": "RL", "L": (100 / 3) / (2 * math.pi * 50)},
                  "outputs": {"IR": 4.0, "IL": 3.0, "IC": 0.0, "I": 5.0, "Z": 20.0, "power_factor": 0.8, "P": 400.0, "Q": 300.0, "S": 500.0, "nature": "lagging"}},
                 {"inputs": {"V": 60, "f": 50, "R": 12, "circuit": "RC", "C": 0.2 / (2 * math.pi * 50)},
                  "outputs": {"IR": 5.0, "IC": 12.0, "I": 13.0, "power_factor": 5 / 13, "Q": -720.0, "S": 780.0, "nature": "leading"}}],
    "questions": [{"q": "A parallel R-L circuit with R = 25 ohm and L = 0.1 H is connected to a supply of V = 100 V, f = 50 Hz. Find the supply current.",
                   "outputs": {"I": math.hypot(4, 10 / math.pi)}},
                  {"q": "A parallel R-L-C circuit with R = 100 ohm, L = 0.5 H and C = 20 uF is connected to a supply of V = 230 V, f = 50 Hz. Find the supply current.",
                   "outputs": {"I": math.hypot(2.3, 230 / (50 * math.pi) - 0.46 * math.pi)}},
                  {"q": "A parallel R-L circuit with R = 25 ohm and L = 0.1 H is connected to a supply of f = 50 Hz. Find the supply current.", "outcome": "NEEDS_INPUT"},
                  {"q": "A parallel R-C circuit with R = 12 ohm, L = 0.1 H and C = 100 uF is connected to a supply of V = 60 V, f = 50 Hz. Find the supply current.",
                   "outcome": "INVALID_INPUT"}]}


def coil_capacitor_parallel(V, f, R, L, C) -> dict:
    _check_positive(V=V, f=f, R=R, L=L, C=C)
    if R * R >= L / C:
        raise ValueError("the coil's resistance must be below sqrt(L/C), or the circuit has no resonance")
    w = _TWO_PI * f
    xl = w * L
    d = R * R + xl * xl
    g = R / d
    bl = xl / d
    bc = w * C
    b = bc - bl
    if abs(b) <= 1e-12 * max(bc, bl):          # exactly at resonance the susceptance is 0, not rounding residue
        b = 0.0
    y = math.hypot(g, b)
    wr = math.sqrt(1.0 / (L * C) - R * R / (L * L))
    rd = L / (C * R)
    return {"I_coil": V / math.sqrt(d), "I_C": V * bc, "I": V * y, "Z": 1.0 / y, "phase_deg": math.degrees(math.atan2(-b, g)) + 0.0,
            "P": V * V * g, "f_r": wr / _TWO_PI, "R_dynamic": rd, "I_resonance": V / rd, "Q": wr * L / R}


def _verify_coil_cap(i, o):
    """Kirchhoff's current law with phasors at the supply frequency (coil current plus capacitor current), and at
    the stated resonant frequency: zero susceptance, the dynamic resistance as 1/G there, and Q = wr L/R."""
    V, R, L, C = i["V"], i["R"], i["L"], i["C"]
    w = _TWO_PI * i["f"]
    d = R * R + (w * L) ** 2
    re_, im_ = V * R / d, -V * w * L / d + V * w * C
    res = (_rel(o["I_coil"], V / math.sqrt(d)) + _rel(o["I_C"], V * w * C) + _rel(o["I"], math.hypot(re_, im_)) + _rel(o["Z"] * o["I"], V)
           + abs(math.radians(o["phase_deg"]) - math.atan2(-im_, re_)) / math.pi + _rel(o["P"], V * re_))
    wr = _TWO_PI * o["f_r"]
    dr = R * R + (wr * L) ** 2
    return (res + abs(wr * C - wr * L / dr) / (wr * C) + _rel(o["R_dynamic"], dr / R) + _rel(o["I_resonance"] * o["R_dynamic"], V)
            + _rel(o["Q"] * R, wr * L))


def _reference_coil_cap(V, f, R, L, C):
    """INDEPENDENT reference: the admittance Y(f) = 1/(R + jwL) + jwC in complex arithmetic (numpy); the resonant
    frequency where its imaginary part vanishes found by scipy.optimize.brentq below 1/(2 pi sqrt(LC)); the dynamic
    resistance 1/Re Y there, and Q as the current magnification I_C/I at resonance."""
    import numpy as np
    from scipy import optimize
    Y = lambda ff: 1 / (R + 1j * 2 * np.pi * ff * L) + 1j * 2 * np.pi * ff * C       # noqa: E731
    y = Y(f)
    f0 = 1 / (2 * np.pi * np.sqrt(L * C))
    fr = optimize.brentq(lambda ff: Y(ff).imag, f0 * 1e-9, f0, xtol=1e-300, rtol=8.9e-16, maxiter=500)
    yr = Y(fr)
    return {"I_coil": float(abs(V / (R + 1j * 2 * np.pi * f * L))), "I_C": float(V * 2 * np.pi * f * C), "I": float(abs(V * y)), "Z": float(1 / abs(y)),
            "phase_deg": float(-np.angle(y, deg=True)), "P": float(V * V * y.real), "f_r": float(fr), "R_dynamic": float(1 / yr.real),
            "I_resonance": float(V * yr.real), "Q": float((V * 2 * np.pi * fr * C) / (V * abs(yr)))}


def _wrong_coil_cap(V, f, R, L, C):                     # the ideal-circuit resonance 1/(2 pi sqrt(LC)), the coil's R ignored
    out = coil_capacitor_parallel(V, f, R, L, C)
    w0 = 1.0 / math.sqrt(L * C)
    out["f_r"] = w0 / _TWO_PI
    out["Q"] = w0 * L / R
    return out


def _random_coil_cap(rng):
    L, C = _e12(rng, -3, -1), _e12(rng, -8, -5)
    return {"V": rng.choice([12.0, 100.0, 230.0, 240.0]), "f": rng.choice([50.0, 60.0, 400.0, 1000.0, 10000.0]),
            "R": round(math.sqrt(L / C) * math.sqrt(rng.uniform(0.1, 0.7)), 3), "L": L, "C": C}


PROCEDURES["coil_capacitor_parallel"] = _fspec(
    "Coil with resistance in parallel with a capacitor: currents, resonance and dynamic resistance", "single-phase a.c. circuits / parallel resonance",
    "For a coil (R in series with L) in parallel with a capacitor C across a supply: the coil, capacitor and supply currents, the phase angle and power at the supply frequency, and the resonant frequency, dynamic resistance L/(CR), current at resonance and Q factor.",
    [_inp("V", "voltage", "rms supply voltage"), _inp("f", "frequency", "supply frequency"), _inp("R", "resistance", "resistance of the coil"),
     _inp("L", "inductance", "inductance of the coil"), _inp("C", "capacitance", "capacitance")],
    [_out("I_coil", "current", "coil current"), _out("I_C", "current", "capacitor current"), _out("I", "current", "supply current"),
     _out("Z", "resistance", "impedance of the network V/I"), _out("phase_deg", "number", "phase angle of the supply current behind the voltage, degrees"),
     _out("P", "power", "real power (dissipated in the coil's resistance)"), _out("f_r", "frequency", "resonant frequency"),
     _out("R_dynamic", "resistance", "dynamic resistance L/(CR) at resonance"), _out("I_resonance", "current", "supply current at resonance"),
     _out("Q", "number", "Q factor at resonance, wr L/R (current magnification)")],
    "f_r", ["R^2 < L/C (a resonance exists)", "sinusoidal steady state, ideal capacitor"],
    "Y = 1/(R + jwL) + jwC; fr = (1/2 pi) sqrt(1/(LC) - R^2/L^2); RD = L/(CR); Q = wr L/R",
    r"f_r = \frac{1}{2\pi}\sqrt{\frac{1}{LC} - \frac{R^2}{L^2}},\quad R_D = \frac{L}{CR}",
    coil_capacitor_parallel, _verify_coil_cap, _reference_coil_cap, _wrong_coil_cap, _random_coil_cap,
    r"\bcoil\b(?=.*\bcapacitor\b)(?=.*\bparallel\b)",
    ["A coil with R = 10 ohm and L = 0.1 H is in parallel with a capacitor C = 10 uF across a supply of V = 100 V, f = 50 Hz. Find the supply current and the resonant frequency."],
    [{"inputs": {"V": 100, "f": 50, "R": 10, "L": 0.1, "C": 1e-5},
      "outputs": {"I_coil": 100 / math.hypot(10, 10 * math.pi), "I_C": 100 * 2 * math.pi * 50 * 1e-5,
                  "I": abs(100 / complex(10, 10 * math.pi) + 1j * 100 * 2 * math.pi * 50 * 1e-5),
                  "f_r": math.sqrt(1 / (0.1 * 1e-5) - 100 / 0.01) / (2 * math.pi), "R_dynamic": 1000.0, "I_resonance": 0.1,
                  "Q": math.sqrt(1 / (0.1 * 1e-5) - 100 / 0.01) * 0.1 / 10}},
     {"inputs": {"V": 240, "f": 50, "R": 50, "L": 0.5, "C": 1e-6},
      "outputs": {"f_r": math.sqrt(1 / (0.5 * 1e-6) - 2500 / 0.25) / (2 * math.pi), "R_dynamic": 10000.0, "I_resonance": 0.024}}],
    [{"q": "A coil with R = 10 ohm and L = 0.1 H is in parallel with a capacitor C = 10 uF across a supply of V = 100 V, f = 50 Hz. Find the supply current and the resonant frequency.",
      "outputs": {"f_r": math.sqrt(1 / (0.1 * 1e-5) - 100 / 0.01) / (2 * math.pi), "R_dynamic": 1000.0}},
     {"q": "A coil with R = 10 ohm and L = 0.1 H is in parallel with a capacitor C = 10 uF across a supply of V = 100 V. Find the supply current.", "outcome": "NEEDS_INPUT"},
     {"q": "A coil with R = 200 ohm and L = 0.1 H is in parallel with a capacitor C = 10 uF across a supply of V = 100 V, f = 50 Hz. Find the resonant frequency.",
      "outcome": "INVALID_INPUT"}],
    forbid=r"\bseries\b")


# --------------------------------------------------------------------------------------------- three-phase measurement
def two_wattmeter(W1, W2) -> dict:
    if not (math.isfinite(W1) and math.isfinite(W2)):
        raise ValueError("the readings must be finite numbers")
    p = W1 + W2
    if p <= 0:
        raise ValueError("the sum of the two readings (the total power) must be positive")
    q = math.sqrt(3.0) * abs(W1 - W2)
    s = math.hypot(p, q)
    return {"P": p, "Q": q, "S": s, "power_factor": p / s, "phase_deg": math.degrees(math.atan2(q, p))}


def _verify_two_watt(i, o):
    W1, W2 = i["W1"], i["W2"]
    s = o["S"]
    return (_rel(o["P"], W1 + W2) + abs(o["Q"] - math.sqrt(3.0) * abs(W1 - W2)) / s + abs(s * s - o["P"] ** 2 - o["Q"] ** 2) / (s * s)
            + abs(o["power_factor"] - o["P"] / s) + abs(math.tan(math.radians(o["phase_deg"])) * o["P"] - o["Q"]) / s)


def _reference_two_watt(W1, W2):
    """INDEPENDENT reference: a balanced star load modelled with phasors (numpy, abc sequence); the two wattmeters in
    lines a and c with their voltage coils to line b read Re((Va - Vb) Ia*) and Re((Vc - Vb) Ic*). The load angle that
    gives the measured ratio (W1 - W2)/(W1 + W2) is found by scipy.optimize.brentq and the model scaled to W1 + W2."""
    import numpy as np
    from scipy import optimize
    a = np.exp(-2j * np.pi / 3)

    def readings(phi):
        Ia, Ic = np.exp(-1j * phi), a * a * np.exp(-1j * phi)
        return float(((1 - a) * np.conj(Ia)).real), float(((a * a - a) * np.conj(Ic)).real)
    hi, lo = max(W1, W2), min(W1, W2)
    target = (hi - lo) / (hi + lo)

    def g(phi):
        r1, r2 = readings(phi)
        return (max(r1, r2) - min(r1, r2)) / (r1 + r2) - target
    phi = 0.0 if target == 0 else optimize.brentq(g, 0.0, np.pi / 2 - 1e-9, xtol=1e-300, rtol=8.9e-16, maxiter=500)
    r1, r2 = readings(phi)
    S = 3.0 * (W1 + W2) / (r1 + r2)
    return {"P": float(S * np.cos(phi)), "Q": float(S * np.sin(phi)), "S": float(S), "power_factor": float(np.cos(phi)), "phase_deg": float(np.degrees(phi))}


def _random_two_watt(rng):
    phi, vi = math.radians(rng.uniform(5, 80)), rng.uniform(300, 30000)          # load angle and VL IL
    a, b = round(vi * math.cos(math.radians(30) - phi), 1), round(vi * math.cos(math.radians(30) + phi), 1)
    return {"W1": a, "W2": b} if rng.random() < 0.5 else {"W1": b, "W2": a}


PROCEDURES["two_wattmeter"] = _fspec(
    "Three-phase power by the two-wattmeter method", "three-phase systems / power measurement",
    "The total power, the power factor, the load angle and the reactive and apparent power of a balanced three-phase load from the readings of two wattmeters.",
    [_inp("W1", "power", "reading of the first wattmeter (negative if it reads backwards)"), _inp("W2", "power", "reading of the second wattmeter")],
    [_out("P", "power", "total power W1 + W2"), _out("Q", "number", "total reactive power in var (magnitude)"),
     _out("S", "number", "total apparent power in VA"), _out("power_factor", "number", "power factor of the load"),
     _out("phase_deg", "number", "load angle in degrees (lag or lead needs the phase sequence)")],
    "power_factor", ["a balanced load and a symmetrical supply", "W1 + W2 > 0"],
    "P = W1 + W2; tan(phi) = sqrt(3) (W1 - W2)/(W1 + W2)", r"P = W_1 + W_2,\quad \tan\phi = \sqrt{3}\,\frac{W_1 - W_2}{W_1 + W_2}",
    two_wattmeter, _verify_two_watt, _reference_two_watt,
    lambda W1, W2: (lambda p, q: {"P": p, "Q": q, "S": math.hypot(p, q), "power_factor": p / math.hypot(p, q), "phase_deg": math.degrees(math.atan2(q, p))})(
        W1 + W2, abs(W1 - W2)),                                                    # sqrt(3) forgotten
    _random_two_watt,
    r"\btwo[- ]wattmeter\b",
    ["Two-wattmeter method: W1 = 3 kW, W2 = 1 kW. Find the total power and the power factor."],
    [{"inputs": {"W1": 3000, "W2": 1000}, "outputs": {"P": 4000.0, "Q": 2000 * math.sqrt(3), "S": math.hypot(4000, 2000 * math.sqrt(3)),
                                                      "power_factor": 4000 / math.hypot(4000, 2000 * math.sqrt(3)), "phase_deg": math.degrees(math.atan(math.sqrt(3) / 2))}},
     {"inputs": {"W1": 5000, "W2": 0}, "outputs": {"P": 5000.0, "S": 10000.0, "power_factor": 0.5, "phase_deg": 60.0}},
     {"inputs": {"W1": 2000, "W2": 2000}, "outputs": {"Q": 0.0, "power_factor": 1.0, "phase_deg": 0.0}}],
    [{"q": "Two-wattmeter method: W1 = 3 kW, W2 = 1 kW. Find the total power and the power factor.",
      "outputs": {"P": 4000.0, "power_factor": 4000 / math.hypot(4000, 2000 * math.sqrt(3))}},
     {"q": "Two-wattmeter method: W1 = 5 kW, W2 = -1 kW. Find the power factor.", "outputs": {"power_factor": 4000 / math.hypot(4000, 6000 * math.sqrt(3))}},
     {"q": "Two-wattmeter method: W1 = 3 kW. Find the total power.", "outcome": "NEEDS_INPUT"},
     {"q": "Two-wattmeter method: W1 = 1 kW, W2 = -2 kW. Find the power factor.", "outcome": "INVALID_INPUT"}])


# --------------------------------------------------------------------------------------------- d.c. machines
def dc_machine_emf(poles, flux, Z, n, winding) -> dict:
    _check_positive(flux=flux, Z=Z, n=n)
    if not (math.isfinite(poles) and poles == int(poles) and poles >= 2 and int(poles) % 2 == 0):
        raise ValueError("the number of poles must be an even whole number of at least 2")
    if Z != int(Z):
        raise ValueError("the number of armature conductors must be a whole number")
    paths = poles if winding == "lap" else 2.0
    per = poles * flux * n
    return {"E": per * Z / paths, "paths": float(paths), "conductor_emf": per}


def _verify_dc_emf(i, o):
    poles = i["poles"]
    want = poles if i["winding"] == "lap" else 2.0
    return _rel(o["conductor_emf"], poles * i["flux"] * i["n"]) + _rel(o["E"] * o["paths"], o["conductor_emf"] * i["Z"]) + abs(o["paths"] - want)


def _reference_dc_emf(poles, flux, Z, n, winding):
    """INDEPENDENT reference: the emf of one conductor averaged over a revolution as the integral of |B(theta)| l r
    d(theta) times n (scipy.integrate.quad), for a flux density B = Bm sin(p theta/2) scaled so that each pole carries
    the flux phi; the conductors in series per parallel path (Z/a, a = poles for lap, 2 for wave) add."""
    from scipy import integrate
    p = int(poles)
    amp = flux * p / 4.0                                   # Bm l r, from phi = integral over one pole of Bm l r sin(p theta/2)
    pts = [2 * math.pi * k / p for k in range(1, p)]
    tot, _ = integrate.quad(lambda th: abs(amp * math.sin(p * th / 2)), 0.0, 2 * math.pi, points=pts, limit=200, epsabs=0, epsrel=1e-13)
    e = n * tot
    paths = p if winding == "lap" else 2
    return {"E": float(e * Z / paths), "paths": float(paths), "conductor_emf": float(e)}


_DCE_IN = [_inp("poles", "number", "number of poles (even)"), _inp("flux", "magnetic_flux", "useful flux per pole"),
           _inp("Z", "number", "number of armature conductors"), _inp("n", "number", "speed in rev/s (rpm accepted in questions)", units="rotational_speed"),
           {"symbol": "winding", "type": "choice", "choices": ["lap", "wave"], "required": True, "meaning": "armature winding: lap or wave"}]


def _parse_dc_emf(question):
    q = question.lower()
    if not re.search(r"\bgenerated emf\b", q) or re.search(r"\bshunt\b", q):
        return None
    lap, wave = re.search(r"\blap\b", q), re.search(r"\bwave\b", q)
    if lap and wave:
        return None
    base = _kv_parse(r"\bgenerated emf\b", _DCE_IN[:4])(question)
    if base is None:
        return None
    if lap or wave:
        base["winding"] = "lap" if lap else "wave"
    return base


PROCEDURES["dc_machine_emf"] = {
    "name": "Generated emf of a d.c. machine (lap or wave winding)", "topic": "d.c. machines / emf equation",
    "purpose": "The emf generated in the armature of a d.c. machine (the back emf of a motor) from its poles, flux per pole, armature conductors, speed and winding.",
    "inputs": _DCE_IN,
    "outputs": [_out("E", "voltage", "generated emf"), _out("paths", "number", "parallel paths: the number of poles for lap, 2 for wave"),
                _out("conductor_emf", "voltage", "average emf of one conductor, poles x flux x n")],
    "primary": "E", "conditions": ["an even number of poles", "speed in rev/s", "simplex windings"],
    "method": "E = poles flux n Z / a with a = poles (lap) or 2 (wave)", "latex": r"E = \frac{p\,\Phi\,n\,Z}{a},\quad a = p\ (\text{lap}),\ a = 2\ (\text{wave})",
    "run": dc_machine_emf, "verify": _verify_dc_emf, "parse": _parse_dc_emf, "reference": _reference_dc_emf,
    "wrong": lambda poles, flux, Z, n, winding: dc_machine_emf(poles, flux, Z, n, "wave" if winding == "lap" else "lap"),   # windings swapped
    "random_inputs": lambda rng: {"poles": float(rng.choice([4, 6, 8])), "flux": round(rng.uniform(5, 60), 1) * 1e-3, "Z": float(rng.randint(100, 1200)),
                                  "n": round(rng.uniform(8, 30), 2), "winding": rng.choice(["lap", "wave"])},
    "templates": ["Find the generated emf of a lap-wound d.c. machine with poles = 4, flux = 25 mWb, Z = 600, n = 20 rev/s."],
    "examples": [{"inputs": {"poles": 4, "flux": 0.025, "Z": 600, "n": 20, "winding": "lap"}, "outputs": {"E": 300.0, "paths": 4.0, "conductor_emf": 2.0}},
                 {"inputs": {"poles": 6, "flux": 0.02, "Z": 400, "n": 15, "winding": "wave"}, "outputs": {"E": 360.0, "paths": 2.0}}],
    "questions": [{"q": "Find the generated emf of a lap-wound d.c. machine with poles = 4, flux = 25 mWb, Z = 600, n = 20 rev/s.", "outputs": {"E": 300.0}},
                  {"q": "Find the generated emf of a wave-wound d.c. machine with poles = 6, flux = 20 mWb, Z = 400, n = 900 rpm.", "outputs": {"E": 360.0}},
                  {"q": "Find the generated emf of a d.c. machine with poles = 4, flux = 25 mWb, Z = 600, n = 20 rev/s.", "outcome": "NEEDS_INPUT"},
                  {"q": "Find the generated emf of a lap-wound d.c. machine with poles = 5, flux = 25 mWb, Z = 600, n = 20 rev/s.", "outcome": "INVALID_INPUT"}]}


def dc_shunt_generator(V, IL, Ra, Rf) -> dict:
    _check_positive(V=V, Ra=Ra, Rf=Rf)
    if not (math.isfinite(IL) and IL >= 0):
        raise ValueError("the load current must be zero or positive")
    i_f = V / Rf
    ia = IL + i_f
    e = V + ia * Ra
    return {"If": i_f, "Ia": ia, "E": e, "P_out": V * IL, "P_generated": e * ia, "copper_loss": ia * ia * Ra + V * i_f}


def _verify_dc_gen(i, o):
    V, IL, Ra, Rf = i["V"], i["IL"], i["Ra"], i["Rf"]
    pg = o["P_generated"]
    return (_rel(o["If"] * Rf, V) + _rel(o["Ia"], IL + o["If"]) + _rel(o["E"], V + o["Ia"] * Ra) + _rel(pg, o["E"] * o["Ia"])
            + _rel(pg, o["P_out"] + o["copper_loss"]) + abs(o["P_out"] - V * IL) / pg + abs(o["copper_loss"] - o["Ia"] ** 2 * Ra - o["If"] ** 2 * Rf) / pg)


def _reference_dc_gen(V, IL, Ra, Rf):
    """INDEPENDENT reference: the generator as a circuit (a source E behind Ra, the shunt field Rf and the load V/IL
    across the terminals) solved by modified nodal analysis; scipy.optimize.brentq finds the E that gives the terminal
    voltage V. Currents and powers are read from that solution."""
    from scipy import optimize

    def solve(e):
        els = [("V", "VE", "1", "0", e), ("R", "Ra", "1", "2", Ra), ("R", "Rf", "2", "0", Rf)]
        if IL > 0:
            els.append(("R", "RL", "2", "0", V / IL))
        return _mna(els)
    hi = 2.0 * V * (1.0 + Ra / Rf + Ra * IL / V)
    e = optimize.brentq(lambda x: solve(x)[0]["2"] - V, 0.0, hi, xtol=1e-300, rtol=8.9e-16, maxiter=500)
    volts, cur = solve(e)
    ia, vt = -cur["VE"], volts["2"]
    return {"E": float(e), "Ia": float(ia), "If": float(vt / Rf), "P_out": float(vt * vt * IL / V), "P_generated": float(e * ia),
            "copper_loss": float(ia * ia * Ra + vt * vt / Rf)}


def _random_dc_gen(rng):
    V, IL = rng.choice([110.0, 220.0, 250.0, 440.0, 500.0]), round(rng.uniform(10, 200), 1)
    return {"V": V, "IL": IL, "Ra": round(V * rng.uniform(0.02, 0.08) / IL, 4), "Rf": round(V / (IL * rng.uniform(0.01, 0.05)), 1)}


PROCEDURES["dc_shunt_generator"] = _fspec(
    "D.C. shunt generator: field and armature currents, generated emf and powers", "d.c. machines / generators",
    "The field current, armature current, generated emf, output power, power generated and copper losses of a shunt-wound d.c. generator delivering a load current at a terminal voltage.",
    [_inp("V", "voltage", "terminal voltage"), _inp("IL", "current", "load current"), _inp("Ra", "resistance", "armature resistance"),
     _inp("Rf", "resistance", "shunt field resistance")],
    [_out("If", "current", "field current V/Rf"), _out("Ia", "current", "armature current IL + If"), _out("E", "voltage", "generated emf V + Ia Ra"),
     _out("P_out", "power", "power delivered to the load V IL"), _out("P_generated", "power", "electrical power generated E Ia"),
     _out("copper_loss", "power", "armature and field copper losses")],
    "E", ["steady state", "brush drop neglected (or included in Ra)"], "If = V/Rf; Ia = IL + If; E = V + Ia Ra",
    r"I_a = I_L + I_f,\quad E = V + I_aR_a",
    dc_shunt_generator, _verify_dc_gen, _reference_dc_gen,
    lambda V, IL, Ra, Rf: (lambda i_f, ia: {"If": i_f, "Ia": ia, "E": V - ia * Ra, "P_out": V * IL, "P_generated": (V - ia * Ra) * ia,
                                            "copper_loss": ia * ia * Ra + V * i_f})(V / Rf, IL + V / Rf),   # the motor's sign
    _random_dc_gen,
    r"\bshunt generator\b",
    ["Find the generated emf of a shunt generator with V = 250 V, IL = 100 A, Ra = 0.1 ohm, Rf = 125 ohm."],
    [{"inputs": {"V": 250, "IL": 100, "Ra": 0.1, "Rf": 125},
      "outputs": {"If": 2.0, "Ia": 102.0, "E": 260.2, "P_out": 25000.0, "P_generated": 26540.4, "copper_loss": 1540.4}},
     {"inputs": {"V": 220, "IL": 0, "Ra": 0.5, "Rf": 110}, "outputs": {"If": 2.0, "Ia": 2.0, "E": 221.0, "P_out": 0.0, "P_generated": 442.0}}],
    [{"q": "Find the generated emf of a shunt generator with V = 250 V, IL = 100 A, Ra = 0.1 ohm, Rf = 125 ohm.", "outputs": {"E": 260.2, "Ia": 102.0}},
     {"q": "Find the generated emf of a shunt generator with V = 220 V, IL = 0 A, Ra = 0.5 ohm, Rf = 110 ohm.", "outputs": {"E": 221.0}},
     {"q": "Find the generated emf of a shunt generator with V = 250 V, IL = 100 A, Ra = 0.1 ohm.", "outcome": "NEEDS_INPUT"},
     {"q": "Find the generated emf of a shunt generator with V = 250 V, IL = 100 A, Ra = 0 ohm, Rf = 125 ohm.", "outcome": "INVALID_INPUT"}])


def dc_shunt_motor(V, I, Ra, Rf, n, losses=0.0) -> dict:
    _check_positive(V=V, I=I, Ra=Ra, Rf=Rf, n=n)
    if not (math.isfinite(losses) and losses >= 0):
        raise ValueError("the losses must be zero or positive")
    i_f = V / Rf
    ia = I - i_f
    if ia <= 0:
        raise ValueError("the supply current must exceed the field current V/Rf")
    e = V - ia * Ra
    if e <= 0:
        raise ValueError("the armature drop Ia Ra reaches the supply voltage: there is no positive back emf")
    pc = e * ia
    pout = pc - losses
    if pout < 0:
        raise ValueError("the stated losses exceed the power converted")
    return {"If": i_f, "Ia": ia, "E": e, "P_converted": pc, "torque": pc / (_TWO_PI * n), "P_out": pout, "efficiency": pout / (V * I)}


def _verify_dc_motor(i, o):
    V, I, Ra, Rf, n = i["V"], i["I"], i["Ra"], i["Rf"], i["n"]
    pc = o["P_converted"]
    return (_rel(o["If"] * Rf, V) + _rel(o["If"] + o["Ia"], I) + _rel(o["E"] + o["Ia"] * Ra, V) + _rel(pc, o["E"] * o["Ia"])
            + _rel(o["torque"] * _TWO_PI * n, pc) + abs(o["P_out"] - pc + i.get("losses", 0.0)) / pc + abs(o["efficiency"] - o["P_out"] / (V * I)))


def _reference_dc_motor(V, I, Ra, Rf, n, losses=0.0):
    """INDEPENDENT reference: the motor as a circuit (supply V, shunt field Rf, and the armature Ra in series with a
    back-emf source E) solved by modified nodal analysis; scipy.optimize.brentq finds the E at which the supply
    delivers I. The power converted is the power the back-emf source absorbs; the torque is that power over 2 pi n."""
    from scipy import optimize

    def solve(e):
        return _mna([("V", "VS", "1", "0", V), ("R", "Rf", "1", "0", Rf), ("R", "Ra", "1", "2", Ra), ("V", "VE", "2", "0", e)])
    e = optimize.brentq(lambda x: -solve(x)[1]["VS"] - I, 0.0, V, xtol=1e-300, rtol=8.9e-16, maxiter=500)
    volts, cur = solve(e)
    ia = cur["VE"]
    pc = e * ia
    return {"If": float(volts["1"] / Rf), "Ia": float(ia), "E": float(e), "P_converted": float(pc), "torque": float(pc / (2 * math.pi * n)),
            "P_out": float(pc - losses), "efficiency": float((pc - losses) / (V * I))}


def _random_dc_motor(rng):
    V = rng.choice([110.0, 220.0, 230.0, 250.0, 440.0, 500.0])
    rf = round(V / rng.uniform(0.5, 5.0), 1)
    ia = rng.uniform(5, 100)
    out = {"V": V, "I": round(ia + V / rf, 3), "Ra": round(V * rng.uniform(0.02, 0.1) / ia, 4), "Rf": rf, "n": round(rng.uniform(10, 30), 2)}
    if rng.random() < 0.5:
        out["losses"] = round(rng.uniform(0.0, 0.05) * V * ia, 1)
    return out


PROCEDURES["dc_shunt_motor"] = _fspec(
    "D.C. shunt motor: back emf, torque, output and efficiency", "d.c. machines / motors",
    "The field and armature currents, back emf, power converted, electromagnetic torque, output power and efficiency of a shunt-wound d.c. motor taking a current I from a supply V at a speed n.",
    [_inp("V", "voltage", "supply voltage"), _inp("I", "current", "current taken from the supply"), _inp("Ra", "resistance", "armature resistance"),
     _inp("Rf", "resistance", "shunt field resistance"), _inp("n", "number", "speed in rev/s (rpm accepted in questions)", units="rotational_speed"),
     _inp("losses", "power", "iron, friction and windage losses (0 when not given)", required=False)],
    [_out("If", "current", "field current V/Rf"), _out("Ia", "current", "armature current I - If"), _out("E", "voltage", "back emf V - Ia Ra"),
     _out("P_converted", "power", "power converted to mechanical form E Ia"), _out("torque", "torque", "electromagnetic torque E Ia/(2 pi n)"),
     _out("P_out", "power", "output power (converted power minus the stated losses)"), _out("efficiency", "number", "efficiency P_out/(V I)")],
    "E", ["steady state", "brush drop neglected (or included in Ra)"], "If = V/Rf; Ia = I - If; E = V - Ia Ra; T = E Ia/(2 pi n)",
    r"E = V - I_aR_a,\quad T = \frac{EI_a}{2\pi n},\quad \eta = \frac{EI_a - P_{loss}}{VI}",
    dc_shunt_motor, _verify_dc_motor, _reference_dc_motor,
    lambda V, I, Ra, Rf, n, losses=0.0: (lambda i_f, ia: (lambda e: {"If": i_f, "Ia": ia, "E": e, "P_converted": e * ia, "torque": e * ia / (_TWO_PI * n),
                                                                      "P_out": e * ia - losses, "efficiency": (e * ia - losses) / (V * I)})(V + ia * Ra))(
        V / Rf, I - V / Rf),                                                         # the generator's sign
    _random_dc_motor,
    r"\bshunt motor\b",
    ["A shunt motor takes I = 30 A from V = 220 V with Ra = 0.4 ohm, Rf = 110 ohm, n = 1000 rpm, losses = 500 W. Find the back emf, torque and efficiency."],
    [{"inputs": {"V": 220, "I": 30, "Ra": 0.4, "Rf": 110, "n": 1000 / 60, "losses": 500},
      "outputs": {"If": 2.0, "Ia": 28.0, "E": 208.8, "P_converted": 5846.4, "torque": 5846.4 / (2 * math.pi * 1000 / 60), "P_out": 5346.4, "efficiency": 5346.4 / 6600}},
     {"inputs": {"V": 250, "I": 20, "Ra": 0.5, "Rf": 250, "n": 20},
      "outputs": {"If": 1.0, "Ia": 19.0, "E": 240.5, "P_converted": 4569.5, "torque": 4569.5 / (40 * math.pi), "efficiency": 0.9139}}],
    [{"q": "A shunt motor takes I = 30 A from V = 220 V with Ra = 0.4 ohm, Rf = 110 ohm, n = 1000 rpm, losses = 500 W. Find the back emf, torque and efficiency.",
      "outputs": {"E": 208.8, "efficiency": 5346.4 / 6600}},
     {"q": "A shunt motor takes I = 20 A from V = 250 V with Ra = 0.5 ohm, Rf = 250 ohm, n = 20 rev/s. Find the back emf and the torque.",
      "outputs": {"E": 240.5, "torque": 4569.5 / (40 * math.pi)}},
     {"q": "A shunt motor takes I = 30 A from V = 220 V with Ra = 0.4 ohm, Rf = 110 ohm. Find the back emf and the torque.", "outcome": "NEEDS_INPUT"},
     {"q": "A shunt motor takes I = 1 A from V = 220 V with Ra = 0.4 ohm, Rf = 110 ohm, n = 1000 rpm. Find the back emf.", "outcome": "INVALID_INPUT"}],
    forbid=r"\bnew speed\b")


def dc_motor_speed_change(V, Ra, Ia1, n1, Ia2, flux_ratio=1.0) -> dict:
    _check_positive(V=V, Ra=Ra, n1=n1, flux_ratio=flux_ratio)
    if not (math.isfinite(Ia1) and Ia1 >= 0 and math.isfinite(Ia2) and Ia2 >= 0):
        raise ValueError("the armature currents must be zero or positive")
    e1 = V - Ia1 * Ra
    e2 = V - Ia2 * Ra
    if e1 <= 0 or e2 <= 0:
        raise ValueError("the armature drop Ia Ra reaches the supply voltage: there is no positive back emf")
    n2 = n1 * (e2 / e1) / flux_ratio
    return {"E1": e1, "E2": e2, "n2": n2, "n2_rpm": 60.0 * n2, "speed_change": n2 / n1 - 1.0}


def _verify_dc_speed(i, o):
    V, Ra, n1 = i["V"], i["Ra"], i["n1"]
    k = i.get("flux_ratio", 1.0)
    return (_rel(o["E1"] + i["Ia1"] * Ra, V) + _rel(o["E2"] + i["Ia2"] * Ra, V) + _rel(o["E2"] * n1, o["E1"] * k * o["n2"])
            + _rel(o["n2_rpm"], 60.0 * o["n2"]) + abs(o["speed_change"] - (o["n2"] / n1 - 1.0)))


def _reference_dc_speed(V, Ra, Ia1, n1, Ia2, flux_ratio=1.0):
    """INDEPENDENT reference (weak: a re-solve of the machine equations): the machine constant k = E1/(phi1 w1) from
    the first state (phi1 = 1), then the new angular speed solved by scipy.optimize.brentq from V = Ia2 Ra + k phi2 w2."""
    from scipy import optimize
    w1 = 2 * math.pi * n1
    k = (V - Ia1 * Ra) / w1
    w2 = optimize.brentq(lambda w: V - Ia2 * Ra - k * flux_ratio * w, 0.0, 1e3 * w1 / flux_ratio, xtol=1e-300, rtol=8.9e-16, maxiter=500)
    n2 = w2 / (2 * math.pi)
    return {"E1": float(k * w1), "E2": float(k * flux_ratio * w2), "n2": float(n2), "n2_rpm": float(60 * n2), "speed_change": float(n2 / n1 - 1)}


def _random_dc_speed(rng):
    V = rng.choice([110.0, 220.0, 230.0, 250.0, 440.0, 500.0])
    ia1 = round(rng.uniform(10, 50), 2)
    out = {"V": V, "Ra": round(V * rng.uniform(0.03, 0.08) / ia1, 4), "Ia1": ia1, "n1": round(rng.uniform(10, 25), 2), "Ia2": round(ia1 * rng.uniform(1.5, 2.5), 2)}
    if rng.random() < 0.7:
        out["flux_ratio"] = round(rng.uniform(0.7, 0.95), 3)
    return out


PROCEDURES["dc_motor_speed_change"] = _fspec(
    "New speed of a d.c. motor after a change of load or flux", "d.c. machines / speed",
    "The back emfs before and after, and the new speed, of a d.c. motor on a constant supply whose armature current changes from Ia1 to Ia2 and whose flux changes by a ratio (speed proportional to E/flux).",
    [_inp("V", "voltage", "supply voltage across the armature"), _inp("Ra", "resistance", "armature resistance"), _inp("Ia1", "current", "first armature current"),
     _inp("n1", "number", "first speed in rev/s (rpm accepted in questions)", units="rotational_speed"), _inp("Ia2", "current", "new armature current"),
     _inp("flux_ratio", "number", "new flux over the first flux (1 when unchanged)", required=False)],
    [_out("E1", "voltage", "first back emf"), _out("E2", "voltage", "new back emf"), _out("n2", "number", "new speed in rev/s"),
     _out("n2_rpm", "number", "new speed in rpm"), _out("speed_change", "number", "fractional change of speed n2/n1 - 1")],
    "n2_rpm", ["a constant supply voltage across the armature", "brush drop neglected"], "E = V - Ia Ra; n2/n1 = (E2/E1)(phi1/phi2)",
    r"\frac{n_2}{n_1} = \frac{E_2}{E_1}\cdot\frac{\Phi_1}{\Phi_2}",
    dc_motor_speed_change, _verify_dc_speed, _reference_dc_speed,
    lambda V, Ra, Ia1, n1, Ia2, flux_ratio=1.0: (lambda e1, e2: (lambda n2: {"E1": e1, "E2": e2, "n2": n2, "n2_rpm": 60 * n2, "speed_change": n2 / n1 - 1})(
        n1 * (e1 / e2) * flux_ratio))(V - Ia1 * Ra, V - Ia2 * Ra),                 # both ratios inverted
    _random_dc_speed,
    r"\bnew speed\b",
    ["Find the new speed of a d.c. motor with V = 220 V, Ra = 0.5 ohm, Ia1 = 20 A, n1 = 1000 rpm, Ia2 = 40 A."],
    [{"inputs": {"V": 220, "Ra": 0.5, "Ia1": 20, "n1": 1000 / 60, "Ia2": 40},
      "outputs": {"E1": 210.0, "E2": 200.0, "n2": 1000 / 60 * 200 / 210, "n2_rpm": 1000 * 200 / 210, "speed_change": 200 / 210 - 1}},
     {"inputs": {"V": 250, "Ra": 0.2, "Ia1": 50, "n1": 20, "Ia2": 50, "flux_ratio": 0.8}, "outputs": {"n2": 25.0, "n2_rpm": 1500.0, "speed_change": 0.25}}],
    [{"q": "Find the new speed of a d.c. motor with V = 220 V, Ra = 0.5 ohm, Ia1 = 20 A, n1 = 1000 rpm, Ia2 = 40 A.", "outputs": {"n2_rpm": 1000 * 200 / 210}},
     {"q": "Find the new speed of a d.c. motor with V = 250 V, Ra = 0.2 ohm, Ia1 = 50 A, n1 = 1200 rpm, Ia2 = 50 A, flux_ratio = 0.8.", "outputs": {"n2_rpm": 1500.0}},
     {"q": "Find the new speed of a d.c. motor with V = 220 V, Ra = 0.5 ohm, Ia1 = 20 A, n1 = 1000 rpm.", "outcome": "NEEDS_INPUT"},
     {"q": "Find the new speed of a d.c. motor with V = 220 V, Ra = 0.5 ohm, Ia1 = 20 A, n1 = 1000 rpm, Ia2 = 40 A, flux_ratio = 0.", "outcome": "INVALID_INPUT"}])


# --------------------------------------------------------------------------------------------- plane electromagnetic waves
def plane_wave_lossless(f, eps_r, mu_r) -> dict:
    _check_positive(f=f)
    if not (math.isfinite(eps_r) and eps_r >= 1 and math.isfinite(mu_r) and mu_r >= 1):
        raise ValueError("the relative permittivity and permeability must be at least 1")
    idx = math.sqrt(eps_r * mu_r)
    v = _C0 / idx
    lam = v / f
    return {"v": v, "wavelength": lam, "eta": _MU0 * _C0 * math.sqrt(mu_r / eps_r), "beta": _TWO_PI / lam, "n": idx}


def _verify_wave(i, o):
    er, mr, f = i["eps_r"], i["mu_r"], i["f"]
    return (_rel(o["v"] ** 2 * er * mr * _EPS0 * _MU0, 1.0) + _rel(o["wavelength"] * f, o["v"]) + _rel(o["eta"] ** 2 * _EPS0 * er, _MU0 * mr)
            + _rel(o["beta"] * o["wavelength"], _TWO_PI) + _rel(o["n"] * o["v"], _C0))


def _reference_wave(f, eps_r, mu_r):
    """INDEPENDENT reference: the propagation constant gamma = sqrt(jw mu (jw eps)) and the wave impedance
    sqrt(mu/eps) in complex arithmetic (numpy), with c from scipy.constants and the stated mu0 = 4 pi 1e-7,
    eps0 = 1/(mu0 c^2); v = w/beta, wavelength 2 pi/beta, n = c beta/w."""
    import numpy as np
    from scipy import constants
    mu = 4e-7 * np.pi * mu_r
    eps = eps_r / (4e-7 * np.pi * constants.c ** 2)
    w = 2 * np.pi * f
    beta = abs(float(np.sqrt(complex(1j * w * mu) * complex(1j * w * eps)).imag))
    eta = float(np.sqrt(complex(mu / eps)).real)
    return {"v": float(w / beta), "wavelength": float(2 * np.pi / beta), "eta": eta, "beta": beta, "n": float(constants.c * beta / w)}


PROCEDURES["plane_wave_lossless"] = _fspec(
    "Plane wave in a lossless medium: speed, wavelength and wave impedance", "electromagnetics / plane waves",
    "The phase velocity, wavelength, intrinsic (wave) impedance, phase constant and refractive index of a uniform plane wave in a lossless medium.",
    [_inp("f", "frequency", "frequency"), _inp("eps_r", "number", "relative permittivity"), _inp("mu_r", "number", "relative permeability")],
    [_out("v", "speed", "phase velocity"), _out("wavelength", "length", "wavelength in the medium"), _out("eta", "resistance", "intrinsic impedance"),
     _out("beta", "number", "phase constant in rad/m"), _out("n", "number", "refractive index sqrt(eps_r mu_r)")],
    "wavelength", ["a lossless, linear, homogeneous medium (conductivity 0)", "eps_r, mu_r >= 1"],
    "v = c/sqrt(eps_r mu_r); wavelength = v/f; eta = eta0 sqrt(mu_r/eps_r)", r"v = \frac{c}{\sqrt{\varepsilon_r\mu_r}},\quad \lambda = \frac{v}{f},\quad \eta = \sqrt{\frac{\mu}{\varepsilon}}",
    plane_wave_lossless, _verify_wave, _reference_wave,
    lambda f, eps_r, mu_r: (lambda v: {"v": v, "wavelength": v / f, "eta": _MU0 * _C0 * math.sqrt(mu_r / eps_r), "beta": _TWO_PI * f / v, "n": eps_r * mu_r})(
        _C0 / (eps_r * mu_r)),                                                      # square root forgotten
    lambda rng: {"f": float(rng.choice([1e6, 1e7, 1e8, 3e8, 1e9, 2.45e9, 1e10])), "eps_r": rng.choice([1.5, 2.1, 2.25, 4.0, 9.0, 81.0]),
                 "mu_r": rng.choice([1.0, 1.0, 1.0, 2.0, 50.0])},
    r"(?=.*\bplane wave\b)(?=.*\blossless\b)",
    ["A plane wave in a lossless medium has f = 100 MHz, eps_r = 4, mu_r = 1. Find the wavelength and the wave impedance."],
    [{"inputs": {"f": 1e8, "eps_r": 4, "mu_r": 1},
      "outputs": {"v": 149896229.0, "wavelength": 1.49896229, "eta": 2e-7 * math.pi * 299792458.0, "beta": 2 * math.pi / 1.49896229, "n": 2.0}},
     {"inputs": {"f": 2.45e9, "eps_r": 81, "mu_r": 1}, "outputs": {"v": 299792458.0 / 9, "wavelength": 299792458.0 / 9 / 2.45e9}}],
    [{"q": "A plane wave in a lossless medium has f = 100 MHz, eps_r = 4, mu_r = 1. Find the wavelength and the wave impedance.",
      "outputs": {"wavelength": 1.49896229, "eta": 2e-7 * math.pi * 299792458.0}},
     {"q": "A plane wave in a lossless medium has f = 100 MHz, mu_r = 1. Find the wavelength.", "outcome": "NEEDS_INPUT"},
     {"q": "A plane wave in a lossless medium has f = 100 MHz, eps_r = 0.5, mu_r = 1. Find the wavelength.", "outcome": "INVALID_INPUT"}],
    forbid=r"\bsigma\b|\bskin depth\b|\blossy\b|\bconducting\b")


def plane_wave_lossy(f, eps_r, mu_r, sigma) -> dict:
    _check_positive(f=f, sigma=sigma)
    if not (math.isfinite(eps_r) and eps_r >= 1 and math.isfinite(mu_r) and mu_r >= 1):
        raise ValueError("the relative permittivity and permeability must be at least 1")
    w = _TWO_PI * f
    eps = _EPS0 * eps_r
    mu = _MU0 * mu_r
    x = sigma / (w * eps)
    s = math.hypot(1.0, x)
    a0 = w * math.sqrt(mu * eps / 2.0)
    alpha = a0 * x / math.sqrt(s + 1.0)          # = a0 sqrt(s - 1) without the cancellation of s - 1 for a small loss tangent
    beta = a0 * math.sqrt(s + 1.0)
    return {"alpha": alpha, "beta": beta, "skin_depth": 1.0 / alpha, "v": w / beta, "wavelength": _TWO_PI / beta,
            "eta": math.sqrt(w * mu / math.hypot(sigma, w * eps)), "eta_angle_deg": 0.5 * math.degrees(math.atan2(sigma, w * eps)), "loss_tangent": x}


def _verify_lossy(i, o):
    """gamma = alpha + j beta must satisfy gamma^2 = jw mu (sigma + jw eps) (real and imaginary parts), eta^2 =
    jw mu/(sigma + jw eps) in magnitude and angle, and the derived lengths and speeds."""
    f, sg = i["f"], i["sigma"]
    w = _TWO_PI * f
    eps = _EPS0 * i["eps_r"]
    mu = _MU0 * i["mu_r"]
    a, b = o["alpha"], o["beta"]
    return (abs(a * a - b * b + w * w * mu * eps) / (b * b) + abs(2 * a * b - w * mu * sg) / (w * mu * sg)
            + _rel(o["skin_depth"] * a, 1.0) + _rel(o["v"] * b, w) + _rel(o["wavelength"] * b, _TWO_PI)
            + _rel(o["eta"] ** 4 * (sg * sg + (w * eps) ** 2), (w * mu) ** 2) + abs(2 * math.radians(o["eta_angle_deg"]) - math.atan2(sg, w * eps)) / math.pi
            + _rel(o["loss_tangent"] * w * eps, sg))


def _reference_lossy(f, eps_r, mu_r, sigma):
    """INDEPENDENT reference: gamma = sqrt(jw mu (sigma + jw eps)) and eta = sqrt(jw mu/(sigma + jw eps)) by numpy's
    complex square root (principal branch), with c from scipy.constants and the stated mu0, eps0 = 1/(mu0 c^2)."""
    import numpy as np
    from scipy import constants
    mu = 4e-7 * np.pi * mu_r
    eps = eps_r / (4e-7 * np.pi * constants.c ** 2)
    w = 2 * np.pi * f
    g = complex(np.sqrt(complex(1j * w * mu) * complex(sigma + 1j * w * eps)))
    eta = complex(np.sqrt(complex(1j * w * mu) / complex(sigma + 1j * w * eps)))
    return {"alpha": g.real, "beta": g.imag, "skin_depth": 1 / g.real, "v": w / g.imag, "wavelength": 2 * np.pi / g.imag,
            "eta": abs(eta), "eta_angle_deg": float(np.angle(eta, deg=True)), "loss_tangent": sigma / (w * eps)}


def _hand_gamma(f, eps_r, mu_r, sigma):
    """Worked-example route: gamma by Python's complex power (jw mu (sigma + jw eps)) ** 0.5."""
    w = 2 * math.pi * f
    return (1j * w * 4e-7 * math.pi * mu_r * (sigma + 1j * w * eps_r * 8.854187817620389e-12)) ** 0.5


PROCEDURES["plane_wave_lossy"] = _fspec(
    "Plane wave in a conducting (lossy) medium: attenuation, skin depth, speed and wave impedance", "electromagnetics / plane waves",
    "The attenuation and phase constants, skin depth, phase velocity, wavelength, intrinsic impedance (magnitude and angle) and loss tangent of a uniform plane wave in a medium of conductivity sigma.",
    [_inp("f", "frequency", "frequency"), _inp("eps_r", "number", "relative permittivity"), _inp("mu_r", "number", "relative permeability"),
     _inp("sigma", "number", "conductivity in S/m", units="conductivity")],
    [_out("alpha", "number", "attenuation constant in Np/m"), _out("beta", "number", "phase constant in rad/m"), _out("skin_depth", "length", "skin depth 1/alpha"),
     _out("v", "speed", "phase velocity w/beta"), _out("wavelength", "length", "wavelength 2 pi/beta"), _out("eta", "resistance", "magnitude of the intrinsic impedance"),
     _out("eta_angle_deg", "number", "angle of the intrinsic impedance in degrees (0 to 45)"), _out("loss_tangent", "number", "loss tangent sigma/(w eps)")],
    "skin_depth", ["a linear, homogeneous medium", "sigma > 0 (a lossless medium: plane_wave_lossless)", "eps_r, mu_r >= 1"],
    "gamma = alpha + j beta = sqrt(jw mu (sigma + jw eps)); delta = 1/alpha; eta = sqrt(jw mu/(sigma + jw eps))",
    r"\gamma = \alpha + j\beta = \sqrt{j\omega\mu(\sigma + j\omega\varepsilon)},\quad \delta = \frac{1}{\alpha}",
    plane_wave_lossy, _verify_lossy, _reference_lossy,
    lambda f, eps_r, mu_r, sigma: plane_wave_lossy(f / _TWO_PI, eps_r, mu_r, sigma),   # f used as w
    lambda rng: (lambda m: {"f": float(rng.choice([50, 60, 1e3, 1e4, 1e5, 1e6, 1e7, 1e8, 1e9])), "eps_r": m[0], "mu_r": m[1], "sigma": m[2]})(
        rng.choice([(81.0, 1.0, 4.0), (1.0, 1.0, 5.8e7), (10.0, 1.0, 0.01), (1.0, 1.0, 3.5e7), (80.0, 1.0, 0.005), (15.0, 1.0, 0.05), (1.0, 200.0, 1e7)])),
    r"\bskin depth\b|\b(?:lossy|conducting)\b",
    ["Find the skin depth of a plane wave in a conducting medium with f = 1 MHz, eps_r = 1, mu_r = 1, sigma = 5.8e7 S/m."],
    [{"inputs": {"f": 1e6, "eps_r": 1, "mu_r": 1, "sigma": 5.8e7},
      "outputs": {"alpha": _hand_gamma(1e6, 1, 1, 5.8e7).real, "beta": _hand_gamma(1e6, 1, 1, 5.8e7).imag, "skin_depth": 1 / _hand_gamma(1e6, 1, 1, 5.8e7).real}},
     {"inputs": {"f": 1e6, "eps_r": 10, "mu_r": 1, "sigma": 0.01},
      "outputs": {"alpha": _hand_gamma(1e6, 10, 1, 0.01).real, "wavelength": 2 * math.pi / _hand_gamma(1e6, 10, 1, 0.01).imag}}],
    [{"q": "Find the skin depth of a plane wave in a conducting medium with f = 1 MHz, eps_r = 1, mu_r = 1, sigma = 5.8e7 S/m.",
      "outputs": {"skin_depth": 1 / _hand_gamma(1e6, 1, 1, 5.8e7).real}},
     {"q": "Find the attenuation of a plane wave in a lossy medium with f = 1 MHz, eps_r = 10, mu_r = 1, sigma = 0.01 S/m.",
      "outputs": {"alpha": _hand_gamma(1e6, 10, 1, 0.01).real}},
     {"q": "Find the skin depth of a plane wave in a conducting medium with f = 1 MHz, eps_r = 1, mu_r = 1.", "outcome": "NEEDS_INPUT"},
     {"q": "Find the skin depth of a plane wave in a conducting medium with f = 1 MHz, eps_r = 1, mu_r = 1, sigma = 0 S/m.", "outcome": "INVALID_INPUT"}],
    forbid=r"\blossless\b")


# ================================================================================ AC network analysis (core gap, 2026-11-12)
# Phasor (sinusoidal steady-state) versions of the DC solvers above: mesh equations with complex impedances, node
# voltages of an AC netlist, and the AC Thevenin and Norton equivalents. Conventions: phasors are rms (powers are
# P + jQ = V I*); angles in degrees, in (-180, 180]; Z = R + jX (X > 0 inductive); an inductor's impedance is j w L and
# a capacitor's 1/(j w C) with w = 2 pi f. Netlist: R<name> <node> <node> <ohms>, L<name> <node> <node> <henries>,
# C<name> <node> <node> <farads>, V<name> <node+> <node-> <rms volts> [<angle deg>], I<name> <node> <node> <rms amperes>
# [<angle deg>] (flowing from the first node through the source to the second); node 0 is ground. Complex division is
# written out (the same bits on every Python version); results are rounded to 12 significant digits and parts below
# 1e-12 of their group's largest magnitude are written as exactly 0 (an angle of a zero phasor is 0).
_AC_FREQ = rf"\b(?:at|f\s*=|frequency\s*(?:of|=|is)?)\s*({_NUM_NB})\s*({_unit_alt('frequency')})(?![A-Za-z])"


def _cdiv(a, b):
    """a / b for complex numbers by the textbook formula."""
    d = b.real * b.real + b.imag * b.imag
    if d == 0:
        raise ValueError("division by a zero impedance")
    return complex((a.real * b.real + a.imag * b.imag) / d, (a.imag * b.real - a.real * b.imag) / d)


def _cabs(z):
    return math.hypot(z.real, z.imag)


def _r12(x):
    if x == 0 or not math.isfinite(x):
        return 0.0 if x == 0 else x
    return float(f"{x:.12g}") + 0.0


def _rect_deg(m, deg):
    """m at an angle in degrees, exact at multiples of 90 degrees."""
    a = deg % 360.0
    if a == 0:
        return complex(m, 0.0)
    if a == 90:
        return complex(0.0, m)
    if a == 180:
        return complex(-m, 0.0)
    if a == 270:
        return complex(0.0, -m)
    t = math.radians(a)
    return complex(m * math.cos(t), m * math.sin(t))


def _csnap(z, scale):
    re_, im = z.real, z.imag
    if abs(re_) <= 1e-12 * scale:
        re_ = 0.0
    if abs(im) <= 1e-12 * scale:
        im = 0.0
    return complex(re_ + 0.0, im + 0.0)


def _phasor_parts(z, scale):
    """(real, imag, magnitude, angle in degrees) of a phasor, parts below 1e-12 of `scale` snapped to 0."""
    s = _csnap(z, scale)
    m = math.hypot(s.real, s.imag)
    ang = math.degrees(math.atan2(s.imag, s.real)) if m > 0 else 0.0
    if ang == -180.0:
        ang = 180.0
    return _r12(s.real), _r12(s.imag), _r12(m), _r12(ang)


def _cgauss(A, b):
    """Solve A x = b for complex A and b by Gaussian elimination with partial pivoting (plain Python, the complex
    division written out). A singular system is a ValueError."""
    n = len(A)
    M = []
    for i in range(n):
        row = []
        for j in range(n):
            row.append(complex(A[i][j]))
        row.append(complex(b[i]))
        M.append(row)
    scale = 0.0
    for i in range(n):
        for j in range(n):
            scale = max(scale, M[i][j].real * M[i][j].real + M[i][j].imag * M[i][j].imag)
    if scale == 0:
        raise ValueError("the system is singular: the equations do not determine every unknown")
    for c in range(n):
        p = c
        best = M[c][c].real * M[c][c].real + M[c][c].imag * M[c][c].imag
        for r in range(c + 1, n):
            m2 = M[r][c].real * M[r][c].real + M[r][c].imag * M[r][c].imag
            if m2 > best:
                p = r
                best = m2
        if best <= 1e-24 * scale:
            raise ValueError("the system is singular: the equations do not determine every unknown")
        if p != c:
            M[c], M[p] = M[p], M[c]
        for r in range(c + 1, n):
            f = _cdiv(M[r][c], M[c][c])
            if f != 0:
                for k in range(c, n + 1):
                    M[r][k] = M[r][k] - f * M[c][k]
    x = [complex(0.0, 0.0)] * n
    for r in range(n - 1, -1, -1):
        s = M[r][n]
        for k in range(r + 1, n):
            s = s - M[r][k] * x[k]
        x[r] = _cdiv(s, M[r][r])
    return x


def _polar_err(o, p):
    """|stated magnitude - |z|| and the angle difference (degrees / 180) of the outputs p_real, p_imag, p_mag, p_deg."""
    z = complex(o[p + "_real"], o[p + "_imag"])
    m = _cabs(z)
    e = abs(o[p + "_mag"] - m) / max(1e-300, abs(o[p + "_mag"]), m)
    if m > 0:
        e += abs((o[p + "_deg"] - math.degrees(math.atan2(z.imag, z.real)) + 180.0) % 360.0 - 180.0) / 180.0
    else:
        e += abs(o[p + "_deg"]) / 180.0
    return e


# ------------------------------------------------------------------------------------------------ AC mesh equations
def _ac_mesh_out(I):
    scale = 0.0
    for z in I:
        scale = max(scale, _cabs(z))
    re_, im, mag, deg = [], [], [], []
    for z in I:
        p = _phasor_parts(z, scale)
        re_.append(p[0])
        im.append(p[1])
        mag.append(p[2])
        deg.append(p[3])
    return {"currents_real": re_, "currents_imag": im, "currents_mag": mag, "currents_deg": deg,
            "first_mag": mag[0], "first_deg": deg[0]}


def ac_mesh_currents(R, X, V_mag, V_deg) -> dict:
    n = len(R)
    if n == 0:
        raise ValueError("give at least one mesh")
    for row in R:
        if len(row) != n:
            raise ValueError("the resistance matrix must be square")
    if len(X) != n:
        raise ValueError("the reactance matrix must have the shape of the resistance matrix")
    for row in X:
        if len(row) != n:
            raise ValueError("the reactance matrix must have the shape of the resistance matrix")
    if len(V_mag) != n or len(V_deg) != n:
        raise ValueError(f"give {n} source magnitudes and {n} source angles, one per mesh")
    for k in range(n):
        if not (math.isfinite(V_mag[k]) and math.isfinite(V_deg[k]) and V_mag[k] >= 0):
            raise ValueError("source magnitudes must be zero or positive and the angles finite (write a sign as 180 degrees)")
        for j in range(n):
            if not (math.isfinite(R[k][j]) and math.isfinite(X[k][j])):
                raise ValueError("the matrices must hold finite numbers")
    Z = []
    for i in range(n):
        row = []
        for j in range(n):
            row.append(complex(R[i][j], X[i][j]))
        Z.append(row)
    V = []
    for k in range(n):
        V.append(_rect_deg(V_mag[k], V_deg[k]))
    return _ac_mesh_out(_cgauss(Z, V))


def _verify_ac_mesh(inputs, outputs):
    """Kirchhoff's voltage law in phasor form: every row of Z I = V, with the stated currents; and each current's
    magnitude and angle agree with its real and imaginary parts."""
    R, X, Vm, Vd = inputs["R"], inputs["X"], inputs["V_mag"], inputs["V_deg"]
    n = len(R)
    for k in ("currents_real", "currents_imag", "currents_mag", "currents_deg"):
        if k not in outputs or len(outputs[k]) != n:
            return math.inf
    I = []
    for k in range(n):
        I.append(complex(outputs["currents_real"][k], outputs["currents_imag"][k]))
    worst = 0.0
    for r in range(n):
        acc = complex(0.0, 0.0)
        sc = 0.0
        for c in range(n):
            t = complex(R[r][c], X[r][c]) * I[c]
            acc = acc + t
            sc += _cabs(t)
        v = _rect_deg(Vm[r], Vd[r])
        sc += _cabs(v)
        if sc > 0:
            worst = max(worst, _cabs(acc - v) / sc)
    for k in range(n):
        o = {"c_real": outputs["currents_real"][k], "c_imag": outputs["currents_imag"][k],
             "c_mag": outputs["currents_mag"][k], "c_deg": outputs["currents_deg"][k]}
        worst = max(worst, _polar_err(o, "c"))
    return worst + _rel(outputs["first_mag"], outputs["currents_mag"][0]) + abs(outputs["first_deg"] - outputs["currents_deg"][0]) / 180.0


def _reference_ac_mesh(R, X, V_mag, V_deg):
    """INDEPENDENT reference: numpy.linalg.solve (LAPACK, complex128) of (R + jX) I = V with V = |V| e^{j theta}."""
    import numpy as np
    Z = np.array(R, float) + 1j * np.array(X, float)
    V = np.array(V_mag, float) * np.exp(1j * np.deg2rad(np.array(V_deg, float)))
    I = np.linalg.solve(Z, V)
    return _ac_mesh_out([complex(z) for z in I])


def _random_ac_mesh(rng):
    m = _random_mesh(rng)
    R = m["A"]
    n = len(R)
    X = [[0.0] * n for _ in range(n)]
    for i in range(n):
        X[i][i] += rng.choice([1, -1]) * _e12(rng, 0, 3)
        for j in range(i + 1, n):
            if rng.random() < 0.5:
                s = rng.choice([1, -1]) * _e12(rng, 0, 2)
                X[i][i] += s
                X[j][j] += s
                X[i][j] -= s
                X[j][i] -= s
    return {"R": R, "X": [[round(x, 9) for x in row] for row in X], "V_mag": [abs(v) for v in m["b"]],
            "V_deg": [float(round(rng.uniform(-180, 180))) for _ in range(n)]}


def parse_ac_mesh(question):
    if not re.search(r"\bmesh\b", question, re.I) or not re.search(_AC_HINT, question, re.I):
        return None
    out, rest = {}, question
    for key, words in (("R", ["resistance matrix"]), ("X", ["reactance matrix"]), ("V_mag", ["source magnitudes"]),
                       ("V_deg", ["source angles"])):
        t = _bracket_after(question, words)
        if t is not None:
            out[key] = t
            rest = rest.replace(t, " ", 1)
    if not out or re.search(r"\d", rest):
        return None                                  # a number outside the four written lists is never guessed
    return out


_AC_MESH_Z = complex(10, 5) * complex(8, -3) - complex(-4, -2) ** 2      # 83 - 6j, the determinant of example 2

PROCEDURES["ac_mesh_currents"] = {
    "name": "AC mesh currents from the phasor mesh equations (complex impedances)", "topic": "AC circuits / AC network analysis",
    "purpose": "Solve the mesh equations Z I = V of an AC circuit in phasor form: the mesh resistance and reactance matrices (Z = R + jX) and the source phasors give every mesh current's magnitude and angle.",
    "inputs": [{"symbol": "R", "type": "matrix", "required": True,
                "meaning": "mesh resistance matrix in ohms (real part of the mesh impedance matrix)"},
               {"symbol": "X", "type": "matrix", "required": True,
                "meaning": "mesh reactance matrix in ohms (imaginary part: + wL for an inductor, - 1/(wC) for a capacitor; shared terms with a minus sign)"},
               {"symbol": "V_mag", "type": "vector", "min_length": 1, "max_length": 30, "required": True,
                "meaning": "rms magnitude of the net source voltage around each mesh, in volts (in the mesh-current direction)"},
               {"symbol": "V_deg", "type": "vector", "min_length": 1, "max_length": 30, "required": True,
                "meaning": "angle of each mesh's net source voltage, in degrees"}],
    "outputs": [{"symbol": "currents_mag", "kind": "number", "meaning": "rms magnitudes of the mesh currents in amperes, in row order"},
                {"symbol": "currents_deg", "kind": "number", "meaning": "angles of the mesh currents in degrees"},
                {"symbol": "currents_real", "kind": "number", "meaning": "real parts of the mesh currents in amperes"},
                {"symbol": "currents_imag", "kind": "number", "meaning": "imaginary parts of the mesh currents in amperes"},
                {"symbol": "first_mag", "kind": "current", "meaning": "magnitude of the first mesh current"},
                {"symbol": "first_deg", "kind": "number", "meaning": "angle of the first mesh current in degrees"}],
    "primary": "first_mag", "conditions": ["square matrices of one shape", "one source phasor per mesh (magnitude >= 0)",
                                           "a non-singular impedance matrix", "sinusoidal steady state, one frequency"],
    "method": "complex Gaussian elimination with partial pivoting on (R + jX) I = V",
    "latex": r"(\mathbf{R} + j\mathbf{X})\,\mathbf{I} = \mathbf{V}",
    "run": ac_mesh_currents, "verify": _verify_ac_mesh, "parse": parse_ac_mesh, "reference": _reference_ac_mesh,
    "wrong": lambda R, X, V_mag, V_deg: ac_mesh_currents(R, [[-x for x in row] for row in X], V_mag, V_deg),   # j and -j confused
    "random_inputs": _random_ac_mesh,
    "templates": ["Solve the AC mesh equations with resistance matrix [[10, -4], [-4, 8]], reactance matrix [[5, -2], [-2, -3]], "
                  "source magnitudes [10, 0] and source angles [0, 0].",
                  "Find the AC mesh currents for resistance matrix [[3]], reactance matrix [[4]], source magnitudes [10] and source angles [0]."],
    "examples": [{"inputs": {"R": [[3.0]], "X": [[4.0]], "V_mag": [10.0], "V_deg": [0.0]},
                  "outputs": {"currents_real": [1.2], "currents_imag": [-1.6], "currents_mag": [2.0],
                              "currents_deg": [math.degrees(math.atan2(-1.6, 1.2))], "first_mag": 2.0}},
                 {"inputs": {"R": [[10.0, -4.0], [-4.0, 8.0]], "X": [[5.0, -2.0], [-2.0, -3.0]], "V_mag": [10.0, 0.0], "V_deg": [0.0, 0.0]},
                  "outputs": {"currents_real": [6820 / 6925, 3200 / 6925], "currents_imag": [-2010 / 6925, 1900 / 6925],
                              "first_mag": abs(complex(80, -30) / _AC_MESH_Z)}}],
    "questions": [{"q": "Solve the AC mesh equations with resistance matrix [[10, -4], [-4, 8]], reactance matrix [[5, -2], [-2, -3]], "
                        "source magnitudes [10, 0] and source angles [0, 0].",
                   "outputs": {"currents_real": [6820 / 6925, 3200 / 6925], "currents_imag": [-2010 / 6925, 1900 / 6925]}},
                  {"q": "Find the AC mesh currents for resistance matrix [[3]], reactance matrix [[4]], source magnitudes [10] and source angles [0].",
                   "outputs": {"first_mag": 2.0, "first_deg": math.degrees(math.atan2(-1.6, 1.2))}},
                  {"q": "Solve the AC mesh equations with resistance matrix [[1, 2], [2, 4]], reactance matrix [[0, 0], [0, 0]], "
                        "source magnitudes [1, 1] and source angles [0, 0].", "outcome": "INVALID_INPUT"},
                  {"q": "Solve the AC mesh equations with resistance matrix [[10, -4], [-4, 8]], reactance matrix [[5, -2], [-2, -3]] "
                        "and source magnitudes [10, 0].", "outcome": "NEEDS_INPUT"}]}


# ------------------------------------------------------------------------------------------------ AC netlists
def _ac_netlist(text):
    """'V1 1 0 10 30; R1 1 2 100; L1 2 0 0.1; C1 2 0 1e-6' -> (type, name, node, node, value, angle in degrees)."""
    els, names = [], set()
    for part in re.split(r"[;\n]+", text):
        p = part.strip().rstrip(".").strip()
        if not p:
            continue
        toks = p.split()
        t = toks[0][0].upper()
        if t not in "RLCVI":
            raise ValueError(f"element '{toks[0]}': only R, L, C, voltage sources (V) and current sources (I)")
        if (t in "RLC" and len(toks) != 4) or (t in "VI" and len(toks) not in (4, 5)):
            raise ValueError(f"cannot read the element '{p}': write <name> <node> <node> <value> (sources: [<angle in degrees>])")
        name, a, b = toks[0], toks[1], toks[2]
        if name.upper() in names:
            raise ValueError(f"element name '{name}' is used twice")
        names.add(name.upper())
        try:
            val = float(toks[3])
            ph = float(toks[4]) if len(toks) == 5 else 0.0
        except ValueError:
            raise ValueError(f"element '{name}': its value is not a number") from None
        if not (math.isfinite(val) and math.isfinite(ph)):
            raise ValueError(f"element '{name}': the value must be finite")
        if t in "RLC" and val <= 0:
            raise ValueError(f"element '{name}' must have a positive value")
        if t in "VI" and val < 0:
            raise ValueError(f"source '{name}': give a non-negative magnitude (a sign is an angle of 180 degrees)")
        if a == b:
            raise ValueError(f"element '{name}' connects node {a} to itself")
        els.append((t, name, a, b, val, ph))
    if not any(e[0] in "RLC" for e in els):
        raise ValueError("the circuit needs at least one resistor, inductor or capacitor")
    if not any("0" in (e[2], e[3]) for e in els):
        raise ValueError("the circuit needs a ground node named 0")
    return els


def _ac_admittance(t, val, w):
    if t == "R":
        return complex(1.0 / val, 0.0)
    if t == "L":
        return complex(0.0, -1.0 / (w * val))
    return complex(0.0, w * val)


def _ac_mna(els, w):
    """Modified nodal analysis with complex admittances: node voltage phasors and voltage-source current phasors
    (positive from node+ through the source to node-)."""
    nodes = sorted({n for e in els for n in (e[2], e[3]) if n != "0"}, key=_node_key)
    idx = {n: i for i, n in enumerate(nodes)}
    vs = [e for e in els if e[0] == "V"]
    N = len(nodes) + len(vs)
    A = [[complex(0.0, 0.0)] * N for _ in range(N)]
    z = [complex(0.0, 0.0)] * N
    for t, name, a, b, val, ph in els:
        ia, ib = idx.get(a), idx.get(b)
        if t in "RLC":
            y = _ac_admittance(t, val, w)
            for i, j, s in ((ia, ia, y), (ib, ib, y), (ia, ib, -y), (ib, ia, -y)):
                if i is not None and j is not None:
                    A[i][j] = A[i][j] + s
        elif t == "I":
            src = _rect_deg(val, ph)
            if ia is not None:
                z[ia] = z[ia] - src
            if ib is not None:
                z[ib] = z[ib] + src
    for k, (t, name, a, b, val, ph) in enumerate(vs):
        r = len(nodes) + k
        ia, ib = idx.get(a), idx.get(b)
        if ia is not None:
            A[ia][r] = A[ia][r] + 1.0
            A[r][ia] = A[r][ia] + 1.0
        if ib is not None:
            A[ib][r] = A[ib][r] - 1.0
            A[r][ib] = A[r][ib] - 1.0
        z[r] = _rect_deg(val, ph)
    A0 = []
    for row in A:
        A0.append(list(row))
    x = _cgauss(A0, z)
    # one step of iterative refinement: near a resonance the system is ill-conditioned and a single elimination left
    # ~6e-9 relative between two solves of the same circuit (box certification, 2026-11-12)
    res = []
    for i in range(N):
        s = z[i]
        for j in range(N):
            s = s - A0[i][j] * x[j]
        res.append(s)
    d = _cgauss(A0, res)
    for i in range(N):
        x[i] = x[i] + d[i]
    vmax = max((_cabs(x[idx[n]]) for n in nodes), default=0.0)
    imax = max((_cabs(x[len(nodes) + k]) for k in range(len(vs))), default=0.0)
    volts = {"0": complex(0.0, 0.0)}
    for n in nodes:
        volts[n] = _csnap(x[idx[n]], vmax)
    cur = {}
    for k, e in enumerate(vs):
        cur[e[1]] = _csnap(x[len(nodes) + k], imax)
    return volts, cur


def _ac_source_power(els, volts, cur):
    """Complex power delivered by the sources, sum of -V I* (voltage sources) and (V_b - V_a) I* (current sources)."""
    s = complex(0.0, 0.0)
    for t, name, a, b, val, ph in els:
        if t == "V":
            s = s - _rect_deg(val, ph) * cur[name].conjugate()
        elif t == "I":
            s = s + (volts[b] - volts[a]) * _rect_deg(val, ph).conjugate()
    return s


def _ac_absorbed_power(els, volts, w):
    """Complex power taken by the passive elements: |V|^2/R, j |V|^2/(wL), -j wC |V|^2."""
    p, q = 0.0, 0.0
    for t, name, a, b, val, ph in els:
        if t in "RLC":
            dv = volts[a] - volts[b]
            v2 = dv.real * dv.real + dv.imag * dv.imag
            if t == "R":
                p += v2 / val
            elif t == "L":
                q += v2 / (w * val)
            else:
                q -= v2 * w * val
    return complex(p, q)


def _ac_nodes_out(volts, cur, S):
    vmax = 0.0
    for v in volts.values():
        vmax = max(vmax, _cabs(v))
    imax = 0.0
    for c in cur.values():
        imax = max(imax, _cabs(c))
    vr, vi, vm, vd, cr, ci = {}, {}, {}, {}, {}, {}
    for n, v in volts.items():
        if n != "0":
            p = _phasor_parts(v, vmax)
            vr[n], vi[n], vm[n], vd[n] = p[0], p[1], p[2], p[3]
    for k, c in cur.items():
        p = _phasor_parts(c, imax)
        cr[k], ci[k] = p[0], p[1]
    sc = max(abs(S.real), abs(S.imag))
    P = 0.0 if abs(S.real) <= 1e-12 * sc else S.real
    Q = 0.0 if abs(S.imag) <= 1e-12 * sc else S.imag
    return {"node_voltages_mag": vm, "node_voltages_deg": vd, "node_voltages_real": vr, "node_voltages_imag": vi,
            "source_currents_real": cr, "source_currents_imag": ci, "P_delivered": _r12(P), "Q_delivered": _r12(Q)}


def ac_node_voltages(netlist, f) -> dict:
    _check_positive(f=f)
    els = _ac_netlist(netlist)
    volts, cur = _ac_mna(els, _TWO_PI * f)
    return _ac_nodes_out(volts, cur, _ac_source_power(els, volts, cur))


def _verify_ac_nodes(inputs, outputs):
    """The circuit solved again: every stated phasor (real, imaginary, magnitude, angle) within 1e-9 of its group's
    scale, and the stated P and Q equal to the power the resistors, inductors and capacitors take (P = sum |V|^2/R,
    Q = sum |V|^2/(wL) - sum wC |V|^2): conservation of complex power."""
    els = _ac_netlist(inputs["netlist"])
    w = _TWO_PI * inputs["f"]
    volts, cur = _ac_mna(els, w)
    for k in ("node_voltages_mag", "node_voltages_deg", "node_voltages_real", "node_voltages_imag",
              "source_currents_real", "source_currents_imag", "P_delivered", "Q_delivered"):
        if k not in outputs:
            return math.inf
    nodes = [n for n in volts if n != "0"]
    if sorted(outputs["node_voltages_real"]) != sorted(nodes) or sorted(outputs["source_currents_real"]) != sorted(cur):
        return math.inf
    vmax = max([_cabs(v) for v in volts.values()] + [1e-300])
    imax = max([_cabs(c) for c in cur.values()] + [1e-300])
    worst = 0.0
    for n in nodes:
        v = volts[n]
        o = {"v_real": outputs["node_voltages_real"][n], "v_imag": outputs["node_voltages_imag"][n],
             "v_mag": outputs["node_voltages_mag"][n], "v_deg": outputs["node_voltages_deg"][n]}
        worst = max(worst, (abs(o["v_real"] - v.real) + abs(o["v_imag"] - v.imag)) / vmax)
        if _cabs(v) > 1e-9 * vmax:
            worst = max(worst, _polar_err(o, "v"))
    for k, c in cur.items():
        worst = max(worst, (abs(outputs["source_currents_real"][k] - c.real) + abs(outputs["source_currents_imag"][k] - c.imag)) / imax)
    S = _ac_absorbed_power(els, volts, w)
    sc = max(abs(S.real), abs(S.imag), abs(outputs["P_delivered"]), abs(outputs["Q_delivered"]), 1e-300)
    return worst + (abs(outputs["P_delivered"] - S.real) + abs(outputs["Q_delivered"] - S.imag)) / sc


def _np_ac_solve(els, w, extra=()):
    """INDEPENDENT reference solver: Kirchhoff's current law written node by node with each element's IMPEDANCE
    (R, j w L, 1/(j w C)) and the voltage sources as constraint rows, solved by numpy.linalg.solve (LAPACK, complex128)."""
    import numpy as np
    els = list(els) + list(extra)
    nodes = sorted({n for e in els for n in (e[2], e[3]) if n != "0"}, key=_node_key)
    vsrc = [e for e in els if e[0] == "V"]
    col = {u: k for k, u in enumerate(nodes + [e[1] for e in vsrc])}
    rows, rhs = [], []
    for n in nodes:
        row = np.zeros(len(col), complex)
        c = 0j
        for t, name, a, b, val, ph in els:
            if n not in (a, b):
                continue
            sgn = 1.0 if a == n else -1.0                # current leaving node n
            if t in "RLC":
                zel = val if t == "R" else (1j * w * val if t == "L" else 1.0 / (1j * w * val))
                if a != "0":
                    row[col[a]] += sgn / zel
                if b != "0":
                    row[col[b]] -= sgn / zel
            elif t == "V":
                row[col[name]] += sgn
            else:
                c -= sgn * val * np.exp(1j * np.deg2rad(ph))
        rows.append(row)
        rhs.append(c)
    for t, name, a, b, val, ph in vsrc:
        row = np.zeros(len(col), complex)
        if a != "0":
            row[col[a]] += 1.0
        if b != "0":
            row[col[b]] -= 1.0
        rows.append(row)
        rhs.append(val * np.exp(1j * np.deg2rad(ph)))
    try:
        x = np.linalg.solve(np.array(rows), np.array(rhs))
    except np.linalg.LinAlgError as exc:
        raise ValueError("the circuit equations have no unique solution") from exc
    volts = {"0": 0j}
    for n in nodes:
        volts[n] = complex(x[col[n]])
    return volts, {e[1]: complex(x[col[e[1]]]) for e in vsrc}


def _reference_ac_nodes(netlist, f):
    els = _ac_netlist(netlist)
    w = 2 * math.pi * f
    volts, cur = _np_ac_solve(els, w)
    return _ac_nodes_out(volts, cur, _ac_absorbed_power(els, volts, w))


def _wrong_ac_nodes(netlist, f):
    """The frequency used as if it were the angular frequency (w = f)."""
    els = _ac_netlist(netlist)
    volts, cur = _ac_mna(els, f)
    return _ac_nodes_out(volts, cur, _ac_source_power(els, volts, cur))


def _random_ac_netlist(rng, port=False):
    for _ in range(100):
        f = rng.choice([50.0, 60.0, 400.0, 1000.0, 10000.0])
        w = 2 * math.pi * f
        n = rng.randint(2, 4)
        nodes = [str(i) for i in range(n + 1)]
        pairs = [(nodes[i], nodes[rng.randrange(0, i)]) for i in range(1, n + 1)]
        for _ in range(rng.randint(0, n)):
            pairs.append(tuple(rng.sample(nodes, 2)))
        parts, counts = [], {"R": 0, "L": 0, "C": 0}
        for k, (a, b) in enumerate(pairs):
            t = "R" if k == 0 else ("L" if k == 1 else rng.choice("RLC"))
            if k == 1 and rng.random() < 0.5:
                t = "C"
            Xv = _e12(rng, 1, 3)                        # 10 ohm .. 8.2 kohm
            val = Xv if t == "R" else (Xv / w if t == "L" else 1.0 / (w * Xv))
            counts[t] += 1
            parts.append(f"{t}{counts[t]} {a} {b} {float(f'{val:.3g}'):g}")
        a, b = rng.sample(nodes, 2)
        parts.append(f"V1 {a} {b} {round(rng.uniform(1, 24), 1):g} {round(rng.uniform(-180, 180)):d}")
        if rng.random() < 0.5:
            c, d = rng.sample(nodes, 2)
            parts.append(f"I1 {c} {d} {round(rng.uniform(0.001, 0.05), 4):g} {round(rng.uniform(-180, 180)):d}")
        text = "; ".join(parts)
        if not port:
            return {"netlist": text, "f": f}
        pa, pb = rng.sample(nodes, 2)
        if {pa, pb} == {a, b}:                           # the source directly across the terminals: Zth = 0
            continue
        return {"netlist": text, "node_a": pa, "node_b": pb, "f": f}
    raise ValueError("no admissible circuit drawn")


def parse_ac_nodes(question):
    if re.search(r"\b(thevenin|th[eé]venin|norton)\b", question, re.I) or not re.search(_AC_HINT, question, re.I):
        return None
    m = re.search(r"\b(?:node voltages|voltage at every node|nodal analysis)\b.*?\bcircuit\s*:\s*(.+?)\s*$", question, re.I | re.S)
    if not m:
        return None
    head = question[:m.start(1)]
    out = {"netlist": m.group(1)}
    mf = re.search(_AC_FREQ, head)
    if mf:
        out["f"] = float(mf.group(1)) * _UNITS["frequency"][mf.group(2)]
        head = head[:mf.start()] + " " + head[mf.end():]
    if re.search(r"\d", head):
        return None                                  # a number before the netlist that is not the frequency
    return out


_AC_NETLIST_IN = {"symbol": "netlist", "type": "text", "min_length": 5, "max_length": 2000, "required": True,
                  "meaning": "elements separated by ';': R<name> <node> <node> <ohms>, L<name> <node> <node> <henries>, "
                             "C<name> <node> <node> <farads>, V<name> <node+> <node-> <rms volts> [<angle deg>], "
                             "I<name> <node> <node> <rms amperes> [<angle deg>] (flowing from the first node through the source "
                             "to the second); node 0 is ground"}
_AC_F_IN = {"symbol": "f", "kind": "frequency", "required": True, "meaning": "frequency of the sources"}
_PI2 = math.pi ** 2

PROCEDURES["ac_node_voltages"] = {
    "name": "Node voltages of an AC circuit from its netlist (phasor nodal analysis)", "topic": "AC circuits / AC network analysis",
    "purpose": "Every node-voltage phasor (magnitude, angle, real and imaginary parts), every voltage-source current and the real and reactive power delivered, for a circuit of R, L, C and independent sinusoidal sources of one frequency.",
    "inputs": [_AC_NETLIST_IN, _AC_F_IN],
    "outputs": [{"symbol": "node_voltages_mag", "kind": "number", "meaning": "rms magnitude of every node voltage relative to node 0, in volts"},
                {"symbol": "node_voltages_deg", "kind": "number", "meaning": "angle of every node voltage, in degrees"},
                {"symbol": "node_voltages_real", "kind": "number", "meaning": "real part of every node voltage, in volts"},
                {"symbol": "node_voltages_imag", "kind": "number", "meaning": "imaginary part of every node voltage, in volts"},
                {"symbol": "source_currents_real", "kind": "number", "meaning": "real part of the current through each voltage source from + to -, in amperes"},
                {"symbol": "source_currents_imag", "kind": "number", "meaning": "imaginary part of that current, in amperes"},
                {"symbol": "P_delivered", "kind": "power", "meaning": "real power delivered by the sources (W)"},
                {"symbol": "Q_delivered", "kind": "number", "meaning": "reactive power delivered by the sources, in var (positive: taken by inductance)"}],
    "primary": "P_delivered", "conditions": ["a ground node 0", "R, L, C positive", "f > 0, one frequency for every source",
                                             "a unique solution (no floating node, no loop of voltage sources)"],
    "method": "modified nodal analysis with complex admittances 1/R, 1/(j w L), j w C, solved by complex Gaussian elimination",
    "latex": r"\mathbf{Y}(j\omega)\,\mathbf{V} = \mathbf{I}_s,\quad Y_L = \frac{1}{j\omega L},\ Y_C = j\omega C",
    "run": ac_node_voltages, "verify": _verify_ac_nodes, "parse": parse_ac_nodes, "reference": _reference_ac_nodes,
    "wrong": _wrong_ac_nodes, "random_inputs": lambda rng: _random_ac_netlist(rng),
    "templates": ["Find the AC node voltages at 50 Hz of the circuit: V1 1 0 10; R1 1 2 3; L1 2 0 0.01.",
                  "Use AC nodal analysis at 1 kHz on the circuit: I1 0 1 0.1 90; R1 1 0 100; C1 1 0 1e-5."],
    "examples": [{"inputs": {"netlist": "V1 1 0 10; R1 1 2 3; L1 2 0 0.01", "f": 50.0},
                  "outputs": {"node_voltages_real": {"1": 10.0, "2": 10 * _PI2 / (9 + _PI2)},
                              "node_voltages_imag": {"1": 0.0, "2": 30 * math.pi / (9 + _PI2)},
                              "P_delivered": 300 / (9 + _PI2), "Q_delivered": 100 * math.pi / (9 + _PI2)}},
                 {"inputs": {"netlist": "I1 0 1 0.1 90; R1 1 0 100; C1 1 0 1e-5", "f": 1000.0},
                  "outputs": {"node_voltages_real": {"1": (0.1j / complex(0.01, 2 * math.pi * 1000 * 1e-5)).real},
                              "node_voltages_imag": {"1": (0.1j / complex(0.01, 2 * math.pi * 1000 * 1e-5)).imag},
                              "P_delivered": abs(0.1j / complex(0.01, 2 * math.pi * 1000 * 1e-5)) ** 2 / 100}}],
    "questions": [{"q": "Find the AC node voltages at 50 Hz of the circuit: V1 1 0 10; R1 1 2 3; L1 2 0 0.01.",
                   "outputs": {"node_voltages_real": {"1": 10.0, "2": 10 * _PI2 / (9 + _PI2)}, "P_delivered": 300 / (9 + _PI2)}},
                  {"q": "Use AC nodal analysis at 1 kHz on the circuit: I1 0 1 0.1 90; R1 1 0 100; C1 1 0 1e-5.",
                   "outputs": {"P_delivered": abs(0.1j / complex(0.01, 2 * math.pi * 1000 * 1e-5)) ** 2 / 100}},
                  {"q": "Find the AC node voltages at 50 Hz of the circuit: V1 1 0 10; R1 1 2 3; L1 2 0 -0.01.", "outcome": "INVALID_INPUT"},
                  {"q": "Find the AC node voltages of the circuit: V1 1 0 10; R1 1 2 3; L1 2 0 0.01.", "outcome": "NEEDS_INPUT"}]}


# ------------------------------------------------------------------------------------------------ AC Thevenin and Norton
def _ac_thevenin(netlist, node_a, node_b, f):
    _check_positive(f=f)
    els = _ac_netlist(netlist)
    nodes = {n for e in els for n in (e[2], e[3])}
    for n in (node_a, node_b):
        if n not in nodes:
            raise ValueError(f"node {n} is not in the circuit")
    if node_a == node_b:
        raise ValueError("the two terminals must be different nodes")
    w = _TWO_PI * f
    volts, _ = _ac_mna(els, w)
    vmax = max(_cabs(v) for v in volts.values())
    vth = _csnap(volts[node_a] - volts[node_b], vmax)
    dead = [(t, n, a, b, 0.0, 0.0) for t, n, a, b, v, ph in els if t == "V"] + [e for e in els if e[0] in "RLC"]
    dead.append(("I", "Itest", node_b, node_a, 1.0, 0.0))     # 1 A into node a, out of node b
    v2, _ = _ac_mna(dead, w)
    zmax = max(_cabs(v) for v in v2.values())
    zth = _csnap(v2[node_a] - v2[node_b], zmax)
    scale = 0.0
    for e in els:
        if e[0] in "RLC":
            scale = max(scale, _cabs(_cdiv(complex(1.0, 0.0), _ac_admittance(e[0], e[4], w))))
    if _cabs(zth) <= 1e-12 * scale:
        raise ValueError("an ideal voltage source (or a short at this frequency) is across the terminals: the Thevenin impedance is 0")
    return vth, zth


def _ac_port_out(prefix_v, v, prefix_z, z):
    pv = _phasor_parts(v, _cabs(v))
    pz = _phasor_parts(z, _cabs(z))
    return {prefix_v + "_real": pv[0], prefix_v + "_imag": pv[1], prefix_v + "_mag": pv[2], prefix_v + "_deg": pv[3],
            prefix_z + "_real": pz[0], prefix_z + "_imag": pz[1], prefix_z + "_mag": pz[2], prefix_z + "_deg": pz[3]}


def ac_thevenin_equivalent(netlist, node_a, node_b, f) -> dict:
    vth, zth = _ac_thevenin(netlist, node_a, node_b, f)
    return _ac_port_out("Vth", vth, "Zth", zth)


def ac_norton_equivalent(netlist, node_a, node_b, f) -> dict:
    vth, zth = _ac_thevenin(netlist, node_a, node_b, f)
    return _ac_port_out("In", _cdiv(vth, zth), "Zn", zth)


def _ac_port_residual(inputs, vth, zth):
    """The full circuit solved twice: open, the terminal voltage must be Vth; with a 1 A test current driven into node
    a and out of node b, it must rise by exactly Zth (superposition)."""
    a, b = inputs["node_a"], inputs["node_b"]
    els = _ac_netlist(inputs["netlist"])
    w = _TWO_PI * inputs["f"]
    v1, _ = _ac_mna(els, w)
    v2, _ = _ac_mna(els + [("I", "Itest_check", b, a, 1.0, 0.0)], w)
    vmax = max(_cabs(v) for v in v1.values())
    # the open-circuit voltage snapped exactly as run snaps it (a value that is 0 by the circuit, e.g. through a
    # dangling branch, comes out of the solve as ~1e-15 of the node voltages; box certification, 2026-11-12)
    o1 = _csnap(v1[a] - v1[b], vmax)
    dz = (v2[a] - v2[b]) - (v1[a] - v1[b])
    return (_cabs(o1 - vth) / max(_cabs(o1), _cabs(vth), 1e-300)
            + _cabs(dz - zth) / max(_cabs(dz), _cabs(zth), 1e-300))


def _verify_ac_thevenin(inputs, outputs):
    for k in ("Vth", "Zth"):
        for s in ("_real", "_imag", "_mag", "_deg"):
            if k + s not in outputs:
                return math.inf
    vth = complex(outputs["Vth_real"], outputs["Vth_imag"])
    zth = complex(outputs["Zth_real"], outputs["Zth_imag"])
    return _ac_port_residual(inputs, vth, zth) + _polar_err(outputs, "Vth") + _polar_err(outputs, "Zth")


def _verify_ac_norton(inputs, outputs):
    for k in ("In", "Zn"):
        for s in ("_real", "_imag", "_mag", "_deg"):
            if k + s not in outputs:
                return math.inf
    zn = complex(outputs["Zn_real"], outputs["Zn_imag"])
    vth = complex(outputs["In_real"], outputs["In_imag"]) * zn
    return _ac_port_residual(inputs, vth, zn) + _polar_err(outputs, "In") + _polar_err(outputs, "Zn")


def _reference_ac_pair(netlist, node_a, node_b, f):
    """INDEPENDENT reference (numpy, complex, impedance-form KCL): the open-circuit voltage of the full circuit, and the
    impedance as 1 V divided by the current a 1 V TEST VOLTAGE SOURCE across the terminals draws with the independent
    sources removed (voltage sources shorted, current sources opened) — a different test from the shipped 1 A current."""
    els = _ac_netlist(netlist)
    w = 2 * math.pi * f
    volts, _ = _np_ac_solve(els, w)
    voc = volts[node_a] - volts[node_b]
    dead = [(t, n, a, b, 0.0, 0.0) for t, n, a, b, v, ph in els if t == "V"] + [e for e in els if e[0] in "RLC"]
    _, c2 = _np_ac_solve(dead, w, [("V", "Vtest_ref", node_a, node_b, 1.0, 0.0)])
    zth = 1.0 / (-c2["Vtest_ref"])                  # the source's current runs from + through it to -: the load current is -i
    vmax = max(abs(v) for v in volts.values())
    return _csnap(complex(voc), vmax), complex(zth)


def _wrong_ac_pair(netlist, node_a, node_b, f):
    """The frequency used as if it were the angular frequency (w = f)."""
    return _ac_thevenin(netlist, node_a, node_b, f / _TWO_PI)


def parse_ac_port(kind):
    def parse(question):
        if not re.search(rf"\b{kind}\b", question, re.I) or not re.search(_AC_HINT, question, re.I):
            return None
        mc = re.search(r"\bcircuit\s*:\s*(.+?)\s*$", question, re.I | re.S)
        if not mc:
            return None
        head = question[:mc.start(1)]
        out = {"netlist": mc.group(1)}
        mn = re.search(r"\bbetween\s+nodes?\s+(\w+)\s+and\s+(\w+)\b", head, re.I)
        if mn:
            out["node_a"], out["node_b"] = mn.group(1), mn.group(2)
            head = head[:mn.start()] + " " + head[mn.end():]
        mf = re.search(_AC_FREQ, head)
        if mf:
            out["f"] = float(mf.group(1)) * _UNITS["frequency"][mf.group(2)]
            head = head[:mf.start()] + " " + head[mf.end():]
        if re.search(r"\d", head):
            return None
        return out
    return parse


_AC_PORT_INPUTS = [_AC_NETLIST_IN,
                   {"symbol": "node_a", "type": "text", "min_length": 1, "max_length": 20, "required": True,
                    "meaning": "the terminal node (the + side of the equivalent)"},
                   {"symbol": "node_b", "type": "text", "min_length": 1, "max_length": 20, "required": True,
                    "meaning": "the reference terminal node"},
                   _AC_F_IN]


def _port_outputs(v, z, vkind, vword):
    return [{"symbol": v + "_mag", "kind": vkind, "meaning": f"rms magnitude of the {vword}"},
            {"symbol": v + "_deg", "kind": "number", "meaning": f"angle of the {vword} in degrees"},
            {"symbol": v + "_real", "kind": vkind, "meaning": f"real part of the {vword}"},
            {"symbol": v + "_imag", "kind": vkind, "meaning": f"imaginary part of the {vword}"},
            {"symbol": z + "_real", "kind": "resistance", "meaning": "resistance (real part of the equivalent impedance)"},
            {"symbol": z + "_imag", "kind": "resistance", "meaning": "reactance (imaginary part of the equivalent impedance, + inductive)"},
            {"symbol": z + "_mag", "kind": "resistance", "meaning": "magnitude of the equivalent impedance"},
            {"symbol": z + "_deg", "kind": "number", "meaning": "angle of the equivalent impedance in degrees"}]


_ZRL = complex(0.0, math.pi)                                     # j w L of the examples' 10 mH at 50 Hz (w L = pi ohm)
_VRC = cmath.rect(5.0, math.radians(30.0)) / complex(1.0, 2 * math.pi)  # the RC example: w R C = 2 pi

PROCEDURES["ac_thevenin_equivalent"] = {
    "name": "Thevenin equivalent of an AC circuit between two nodes (phasors, complex impedance)",
    "topic": "AC circuits / AC Thevenin and Norton equivalents",
    "purpose": "The Thevenin voltage phasor and impedance Zth = R + jX seen between two nodes of an AC circuit of R, L, C and independent sinusoidal sources at one frequency.",
    "inputs": _AC_PORT_INPUTS, "outputs": _port_outputs("Vth", "Zth", "voltage", "Thevenin (open-circuit) voltage, node a relative to node b"),
    "primary": "Vth_mag", "conditions": ["both nodes in the circuit", "no ideal voltage source (or zero impedance) directly across the terminals",
                                         "a unique solution (no floating node)", "f > 0"],
    "method": "open-circuit voltage by phasor modified nodal analysis; impedance by a 1 A test current with the independent sources set to zero",
    "latex": r"\mathbf{V}_{th} = \mathbf{V}_a - \mathbf{V}_b\big|_{open},\quad Z_{th} = \frac{\mathbf{V}_a - \mathbf{V}_b}{1\,\mathrm{A}}\big|_{sources\ off}",
    "run": ac_thevenin_equivalent, "verify": _verify_ac_thevenin, "parse": parse_ac_port("thevenin"),
    "reference": lambda netlist, node_a, node_b, f: (lambda vz: _ac_port_out("Vth", vz[0], "Zth", vz[1]))(_reference_ac_pair(netlist, node_a, node_b, f)),
    "wrong": lambda netlist, node_a, node_b, f: (lambda vz: _ac_port_out("Vth", vz[0], "Zth", vz[1]))(_wrong_ac_pair(netlist, node_a, node_b, f)),
    "random_inputs": lambda rng: _random_ac_netlist(rng, port=True),
    "templates": ["Find the AC Thevenin equivalent between nodes 2 and 0 at 50 Hz of the circuit: V1 1 0 10; R1 1 2 3; L1 2 0 0.01.",
                  "Find the AC Thevenin equivalent between nodes 2 and 0 at 1 kHz of the circuit: V1 1 0 5 30; R1 1 2 1000; C1 2 0 1e-6."],
    "examples": [{"inputs": {"netlist": "V1 1 0 10; R1 1 2 3; L1 2 0 0.01", "node_a": "2", "node_b": "0", "f": 50.0},
                  "outputs": {"Vth_real": 10 * _PI2 / (9 + _PI2), "Vth_imag": 30 * math.pi / (9 + _PI2),
                              "Zth_real": 3 * _PI2 / (9 + _PI2), "Zth_imag": 9 * math.pi / (9 + _PI2)}},
                 {"inputs": {"netlist": "V1 1 0 5 30; R1 1 2 1000; C1 2 0 1e-6", "node_a": "2", "node_b": "0", "f": 1000.0},
                  "outputs": {"Vth_real": _VRC.real, "Vth_imag": _VRC.imag, "Vth_mag": 5 / math.hypot(1, 2 * math.pi),
                              "Zth_real": (1000 / complex(1.0, 2 * math.pi)).real, "Zth_imag": (1000 / complex(1.0, 2 * math.pi)).imag}}],
    "questions": [{"q": "Find the AC Thevenin equivalent between nodes 2 and 0 at 50 Hz of the circuit: V1 1 0 10; R1 1 2 3; L1 2 0 0.01.",
                   "outputs": {"Vth_real": 10 * _PI2 / (9 + _PI2), "Zth_imag": 9 * math.pi / (9 + _PI2)}},
                  {"q": "Find the AC Thevenin equivalent between nodes 2 and 0 at 1 kHz of the circuit: V1 1 0 5 30; R1 1 2 1000; C1 2 0 1e-6.",
                   "outputs": {"Vth_mag": 5 / math.hypot(1, 2 * math.pi)}},
                  {"q": "Find the AC Thevenin equivalent between nodes 1 and 0 at 50 Hz of the circuit: V1 1 0 10; R1 1 2 3; L1 2 0 0.01.",
                   "outcome": "INVALID_INPUT"},
                  {"q": "Find the AC Thevenin equivalent between nodes 2 and 0 of the circuit: V1 1 0 10; R1 1 2 3; L1 2 0 0.01.",
                   "outcome": "NEEDS_INPUT"}]}

PROCEDURES["ac_norton_equivalent"] = {
    "name": "Norton equivalent of an AC circuit between two nodes (phasors, complex impedance)",
    "topic": "AC circuits / AC Thevenin and Norton equivalents",
    "purpose": "The Norton current phasor and impedance seen between two nodes of an AC circuit of R, L, C and independent sinusoidal sources at one frequency.",
    "inputs": _AC_PORT_INPUTS, "outputs": _port_outputs("In", "Zn", "current", "Norton (short-circuit) current from node a to node b"),
    "primary": "In_mag", "conditions": ["both nodes in the circuit", "no ideal voltage source (or zero impedance) directly across the terminals",
                                        "a unique solution (no floating node)", "f > 0"],
    "method": "Thevenin voltage and impedance by phasor modified nodal analysis, then I_N = V_th / Z_th",
    "latex": r"\mathbf{I}_N = \frac{\mathbf{V}_{th}}{Z_{th}},\quad Z_N = Z_{th}",
    "run": ac_norton_equivalent, "verify": _verify_ac_norton, "parse": parse_ac_port("norton"),
    "reference": lambda netlist, node_a, node_b, f: (lambda vz: _ac_port_out("In", vz[0] / vz[1], "Zn", vz[1]))(_reference_ac_pair(netlist, node_a, node_b, f)),
    "wrong": lambda netlist, node_a, node_b, f: (lambda vz: _ac_port_out("In", _cdiv(vz[0], vz[1]), "Zn", vz[1]))(_wrong_ac_pair(netlist, node_a, node_b, f)),
    "random_inputs": lambda rng: _random_ac_netlist(rng, port=True),
    "templates": ["Find the AC Norton equivalent between nodes 2 and 0 at 50 Hz of the circuit: V1 1 0 10; R1 1 2 3; L1 2 0 0.01."],
    "examples": [{"inputs": {"netlist": "V1 1 0 10; R1 1 2 3; L1 2 0 0.01", "node_a": "2", "node_b": "0", "f": 50.0},
                  "outputs": {"In_real": 10 / 3, "In_imag": 0.0, "In_mag": 10 / 3,
                              "Zn_real": 3 * _PI2 / (9 + _PI2), "Zn_imag": 9 * math.pi / (9 + _PI2)}},
                 {"inputs": {"netlist": "V1 1 0 5 30; R1 1 2 1000; C1 2 0 1e-6", "node_a": "2", "node_b": "0", "f": 1000.0},
                  "outputs": {"In_mag": 0.005, "In_deg": 30.0}}],
    "questions": [{"q": "Find the AC Norton equivalent between nodes 2 and 0 at 50 Hz of the circuit: V1 1 0 10; R1 1 2 3; L1 2 0 0.01.",
                   "outputs": {"In_mag": 10 / 3}},
                  {"q": "Find the AC Norton equivalent between nodes 1 and 0 at 50 Hz of the circuit: V1 1 0 10; R1 1 0 3.",
                   "outcome": "INVALID_INPUT"}]}
