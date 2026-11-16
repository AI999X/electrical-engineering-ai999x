# Electrical Engineering (electrical-engineering-ai999x)

[![tests](https://github.com/AI999X/electrical-engineering-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/electrical-engineering-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/electrical-engineering-ai999x)](https://pypi.org/project/electrical-engineering-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/electrical-engineering-ai999x)

Electrical Engineering covers DC and AC circuits, transients, phasors and power, operational amplifiers, diodes and transistors, digital logic, and control systems (transfer functions, stability, Bode): 92 calculations in 17 topics, each with its formula, its steps and its units, and a plain reason instead of a number when an input is missing or out of range. One compiled engine, four ways to use it: the desktop app, the command line, the browser and Python. Everything runs on your own computer, offline.

[![The Electrical Engineering desktop app: Energy stored in an inductor, with its worked example and live scene](https://www.ai999x.com/?ax_shot=electrical-engineering-ai999x/hero.webp)](https://www.ai999x.com/models/electrical-engineering-ai999x/)

## Four ways to use it

| | |
|---|---|
| **Desktop app** (Windows, Linux) | [Windows](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.4/ai999x-1.6.4-windows-x86_64.zip) · [Linux](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.4/ai999x-1.6.4-linux-x86_64.tar.gz) |
| **Command line** `electrical-engineering` (Windows, Linux) | [Windows](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.4/ai999x-cli-1.6.4-windows-x86_64.zip) · [Linux](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.4/ai999x-cli-1.6.4-linux-x86_64.tar.gz) |
| **Browser** (any device) | [Try it on the model page](https://www.ai999x.com/models/electrical-engineering-ai999x/#try) |
| **Python** 3.10+ | `pip install electrical-engineering-ai999x`: on Windows and Linux (x86-64) `pip` installs the same compiled engine; on other systems (macOS, ARM) it installs a pure-Python build that gives the same answers |

## Ask in words: 15 examples

Each question below was asked on this release; the comment under it is the line it printed.

```python
import electrical_engineering_ai999x as electrical_engineering

a = electrical_engineering.ask(
    "Find the branch currents of a current divider with I = 6 A, R1 = "
    "4 ohm, R2 = 2 ohm."
)
print(a.outcome, a.values)
# SOLVED {'I1': 2.0, 'I2': 4.0}

a = electrical_engineering.ask(
    "A difference amplifier has R1 = 10 kohm, R2 = 100 kohm, R3 = 10 "
    "kohm, R4 = 100 kohm, V1 = 1 V and V2 = 1.5 V. Find the output "
    "voltage."
)
print(a.outcome, a.values)
# SOLVED {'Vout': 5.0, 'gain_inverting': -10.0, 'gain_noninverting': 10.0}

a = electrical_engineering.ask(
    "Find the first-order transient value with V0 = 0 V, Vf = 10 V, "
    "tau = 1 ms, t = 2 ms."
)
print(a.outcome, a.values)
# SOLVED {'v': 8.646647167633873, 'fraction_complete': 0.8646647167633873}

a = electrical_engineering.ask(
    "Find the energy cost with P = 2 kW, t = 5 h, tariff = 0.15 per "
    "kWh."
)
print(a.outcome, a.values)
# SOLVED {'energy': 36000000.0, 'energy_kWh': 10.0, 'cost': 1.5}

a = electrical_engineering.ask(
    "Convert a delta with Rab = 10 ohm, Rbc = 20 ohm, Rca = 30 ohm to "
    "a wye."
)
print(a.outcome, a.values)
# SOLVED {'Ra': 5.0, 'Rb': 3.3333333333333335, 'Rc': 10.0}

a = electrical_engineering.ask(
    "Convert the binary number 1011 to Gray code."
)
print(a.outcome, a.values)
# SOLVED {'result': '1110', 'decimal': 14.0}

a = electrical_engineering.ask(
    "Find the output of an inverting amplifier with Rin = 10 kohm, Rf "
    "= 47 kohm, Vin = 0.5 V."
)
print(a.outcome, a.values)
# SOLVED {'gain': -4.7, 'Vout': -2.35}

a = electrical_engineering.ask(
    "Find the resistance at a new temperature with R1 = 50 ohm, alpha1 "
    "= 0.004, T1 = 20 °C, T2 = 80 °C."
)
print(a.outcome, a.values)
# SOLVED {'R2': 62.0, 'alpha2': 0.0032258064516129032, 'change': 12.0}

a = electrical_engineering.ask(
    "Find the generated emf of a lap-wound d.c. machine with poles = "
    "4, flux = 25 mWb, Z = 600, n = 20 rev/s."
)
print(a.outcome, a.values)
# SOLVED {'E': 300.0, 'paths': 4.0, 'conductor_emf': 2.0}

a = electrical_engineering.ask(
    "Find the equivalent capacitance of 3 uF and 6 uF capacitors in "
    "parallel."
)
print(a.outcome, a.values)
# SOLVED {'equivalent': 9.0, 'count': 2.0, 'unit': 'uF'}

a = electrical_engineering.ask("Is (A·B)' equivalent to A' + B'?")
print(a.outcome, a.values)
# SOLVED {'equivalent': True, 'counterexample': 'none'}

a = electrical_engineering.ask(
    "Find the energy stored in an inductor with L = 50 mH, I = 2 A."
)
print(a.outcome, a.values)
# SOLVED {'energy': 0.1, 'flux_linkage': 0.1}

a = electrical_engineering.ask(
    "Find the temperature coefficient from R1 = 100 ohm, T1 = 20 °C, "
    "R2 = 120 ohm, T2 = 70 °C."
)
print(a.outcome, a.values)
# SOLVED {'alpha1': 0.004, 'alpha2': 0.0033333333333333335, 'slope': 0.4}

a = electrical_engineering.ask(
    "Find the equivalent capacitance of 3 uF and 6 uF capacitors in "
    "series."
)
print(a.outcome, a.values)
# SOLVED {'equivalent': 2.0, 'count': 2.0, 'unit': 'uF'}

a = electrical_engineering.ask(
    "Evaluate A AND (B OR NOT C) for A=1, B=0, C=0."
)
print(a.outcome, a.values)
# SOLVED {'value': 1.0}
```

## Screenshots

[![The Electrical Engineering desktop app: Energy stored in an inductor, with its worked example and live scene](https://www.ai999x.com/?ax_shot=electrical-engineering-ai999x/hero-dark.webp)](https://www.ai999x.com/models/electrical-engineering-ai999x/)

[![Equivalent resistance of resistors in parallel: the live scene in the Electrical Engineering desktop app](https://www.ai999x.com/?ax_shot=electrical-engineering-ai999x/see-1.webp)](https://www.ai999x.com/models/electrical-engineering-ai999x/)

*Equivalent resistance of resistors in parallel*

[![Evaluate a Boolean expression: the live scene in the Electrical Engineering desktop app](https://www.ai999x.com/?ax_shot=electrical-engineering-ai999x/see-2.webp)](https://www.ai999x.com/models/electrical-engineering-ai999x/)

*Evaluate a Boolean expression*

[![Inverting amplifier gain and output: the live scene in the Electrical Engineering desktop app](https://www.ai999x.com/?ax_shot=electrical-engineering-ai999x/see-3.webp)](https://www.ai999x.com/models/electrical-engineering-ai999x/)

*Inverting amplifier gain and output*

Every calculation, with its formula, a worked example and its figures: **[Electrical Engineering on ai999x.com](https://www.ai999x.com/models/electrical-engineering-ai999x/)**. Install guides for the app and the command line: [https://www.ai999x.com/desktop-cli/](https://www.ai999x.com/desktop-cli/).

Version 1.6.4 · License: Apache-2.0 · Author: Ibnul Jaif Farabi · Organization: AI999X · Contact: support@ai999x.com · [Support the project](https://buymeacoffee.com/ibnuljaiffarabi)

*The Future Is Self-Improving.*

## Source and releases

This repository holds the installable source of each release, exactly as published on [PyPI](https://pypi.org/project/electrical-engineering-ai999x/) (`pip install .` builds it). Releases are tagged `v1.6.4`; the [GitHub release](https://github.com/AI999X/electrical-engineering-ai999x/releases/tag/v1.6.4) carries the wheel, the desktop app and the command-line program for Windows and Linux, and their checksums.

## Build the program from source

`rust/` holds the Rust source of the `electrical-engineering` command-line program and browser module: the AI999X engine (`rust/ax-engine`), this model (`rust/electrical-engineering`) and its data (`rust/models/electrical_engineering/pack.json`). With Rust 1.80 or later:

```bash
cd rust
cargo build --release -p electrical-engineering-ai999x
./target/release/electrical-engineering --help
cargo test -p electrical-engineering-ai999x
```
