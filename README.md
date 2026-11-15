# Electrical Engineering (electrical-engineering-ai999x)

[![tests](https://github.com/AI999X/electrical-engineering-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/electrical-engineering-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/electrical-engineering-ai999x)](https://pypi.org/project/electrical-engineering-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/electrical-engineering-ai999x)

Electrical Engineering covers DC and AC circuits, transients, phasors and power, operational amplifiers, diodes and transistors, digital logic, and control systems (transfer functions, stability, Bode): 92 calculations in 17 topics, each with its formula, its steps and its units, and a plain reason instead of a number when an input is missing or out of range. One compiled engine, four ways to use it: the desktop app, the command line, the browser and Python. Everything runs on your own computer, offline.

[![The Electrical Engineering desktop app: Energy stored in an inductor, with its worked example and live scene](https://www.ai999x.com/?ax_shot=electrical-engineering-ai999x/hero.webp)](https://www.ai999x.com/models/electrical-engineering-ai999x/)

## Four ways to use it

| | |
|---|---|
| **Desktop app** (Windows, Linux) | [Windows](https://downloads.ai999x.com/desktop/latest/ai999x-windows-x86_64.zip) · [Linux](https://downloads.ai999x.com/desktop/latest/ai999x-linux-x86_64.tar.gz) |
| **Command line** `electrical-engineering` (Windows, Linux) | [Windows](https://downloads.ai999x.com/desktop/latest/ai999x-cli-windows-x86_64.zip) · [Linux](https://downloads.ai999x.com/desktop/latest/ai999x-cli-linux-x86_64.tar.gz) |
| **Browser** (any device) | [Try it on the model page](https://www.ai999x.com/models/electrical-engineering-ai999x/#try) |
| **Python** 3.10+ | `pip install electrical-engineering-ai999x`: on Windows and Linux (x86-64) `pip` installs the same compiled engine; on other systems (macOS, ARM) it installs a pure-Python build that gives the same answers |

## Example

```python
import electrical_engineering_ai999x as electrical_engineering
a = electrical_engineering.ask("Find the Thevenin equivalent between nodes 2 and 0 of the circuit: V1 1 0 12; R1 1 2 1000; R2 2 0 2000.")
print(a.outcome, a.value, a.unit)
```

Output:

```text
SOLVED 8.0 V
```

Every calculation, with its formula, a worked example and its figures: **[Electrical Engineering on ai999x.com](https://www.ai999x.com/models/electrical-engineering-ai999x/)**. Install guides for the app and the command line: [https://www.ai999x.com/desktop-cli/](https://www.ai999x.com/desktop-cli/).

Version 1.6.3 · License: Apache-2.0 · Author: Ibnul Jaif Farabi · Organization: AI999X · Contact: support@ai999x.com · [Support the project](https://buymeacoffee.com/ibnuljaiffarabi)

*The Future Is Self-Improving.*

## Source and releases

This repository holds the installable source of each release, exactly as published on [PyPI](https://pypi.org/project/electrical-engineering-ai999x/) (`pip install .` builds it). Releases are tagged `v1.6.3`; the [GitHub release](https://github.com/AI999X/electrical-engineering-ai999x/releases/tag/v1.6.3) carries the wheel, the desktop app and the command-line program for Windows and Linux, and their checksums.

## Build the program from source

`rust/` holds the Rust source of the `electrical-engineering` command-line program and browser module: the AI999X engine (`rust/ax-engine`), this model (`rust/electrical-engineering`) and its data (`rust/models/electrical_engineering/pack.json`). With Rust 1.80 or later:

```bash
cd rust
cargo build --release -p electrical-engineering-ai999x
./target/release/electrical-engineering --help
cargo test -p electrical-engineering-ai999x
```
