# electrical-engineering-ai999x — offline, verified electrical engineering solver with step-by-step answers

[![tests](https://github.com/AI999X/electrical-engineering-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/electrical-engineering-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/electrical-engineering-ai999x)](https://pypi.org/project/electrical-engineering-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/electrical-engineering-ai999x)

**[Install](https://pypi.org/project/electrical-engineering-ai999x/)** · **[Capability guide](https://huggingface.co/AI999X/electrical-engineering-ai999x/tree/main/docs)** · **[Evaluation](https://huggingface.co/AI999X/electrical-engineering-ai999x/tree/main/evaluation)** · **[AI999X on Hugging Face](https://huggingface.co/AI999X)** · **[Website](https://ai999x.com)** · **[Support: Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi)**

License: Apache-2.0 · Author: Ibnul Jaif Farabi (creator and engineer) · Organization: AI999X · Version 1.0.0

Field: **Engineering** · Tasks: Question Answering, Token Classification, Text Classification, Sentence Similarity, Feature Extraction

electrical-engineering-ai999x is an offline, deterministic Python solver that answers electrical engineering questions with verified results from 64 certified capabilities, gives 326 documented answers identically on Windows and Linux, and refuses rather than guesses when an answer cannot be verified.

**electrical-engineering-ai999x** is a compact, deterministic electrical engineering engine for Python. It reads a question in plain English (or a typed call), picks a certified electrical engineering capability, computes the answer exactly, checks it, and returns the result with its steps, units, LaTeX and assumptions — or an honest refusal that says what is missing. It runs fully offline on any CPU, needs no GPU, no API key and no internet, and is built to give **zero wrong answers**: when it is not sure, it refuses instead of guessing.

[![Buy me a coffee](https://img.shields.io/badge/Buy%20me%20a%20coffee-support%20development-FFDD00?logo=buymeacoffee&logoColor=black)](https://buymeacoffee.com/ibnuljaiffarabi) ![License](https://img.shields.io/badge/license-Apache--2.0-blue) ![Python](https://img.shields.io/badge/python-3.10%20%7C%203.11%20%7C%203.12%20%7C%203.13%20%7C%203.14-blue) ![Offline](https://img.shields.io/badge/runs-offline%20on%20CPU-green)

## Highlights

- **Verified, not generated.** Every answer is computed by a certified electrical engineering capability and checked (substituted back, dimension-checked or compared with an independent computation) before it is returned.
- **Zero wrong answers by design.** Seven explicit outcomes — SOLVED, NEEDS_INPUT, AMBIGUOUS, INVALID_INPUT, OUT_OF_SCOPE, UNRECOGNIZED_FORMAT, INTERNAL_ERROR — never a silent guess. Every one of its 326 documented answers is reproduced exactly on every tested platform.
- **Plain-English questions and a guaranteed typed interface.** `ask()` reads documented question forms; `solve(capability_id, **inputs)` always solves a supported task with valid inputs.
- **Explains itself.** Steps, formulas in LaTeX, units, assumptions and the model used, ready for notebooks, tutoring apps and homework checkers.
- **Small and portable.** Pure Python (sympy, mpmath); identical answers on Windows and Linux (Python 3.10, 3.11, 3.12, 3.13, 3.14).
- **Coverage:** DC and AC circuits, transients, phasors and power, operational amplifiers, diodes and transistors, digital logic, and control systems (transfer functions, stability, Bode).

## Model overview

| Property | Value |
|---|---|
| Package | `electrical-engineering-ai999x` 1.0.0 (import `electrical_engineering_ai999x`) |
| Type | Deterministic neuro-symbolic electrical engineering engine (rule-based reading + certified symbolic capabilities) |
| Capabilities | 64 certified procedures |
| Knowledge pack | version 1.0.0 |
| Golden answers | 326 documented answers, identical on every tested platform |
| Platforms tested | Windows (Python 3.13), Linux x86-64 (Python 3.10, 3.11, 3.12, 3.13, 3.14) |
| Runtime | CPU only, offline; no GPU, no network, no API key |
| License | Apache-2.0 |
| Creator / engineer | Ibnul Jaif Farabi |
| Organization | AI999X |

## Quickstart

```bash
pip install electrical-engineering-ai999x
```

```python
import electrical_engineering_ai999x as electrical_engineering

electrical_engineering.ask("Find the Thevenin equivalent between nodes 2 and 0 of the circuit: V1 1 0 12; R1 1 2 1000; R2 2 0 2000.").show()
# -> SOLVED — Vth = 8, Rth = 666.667  (thevenin_equivalent)
electrical_engineering.ask("Find the gain margin of G(s) = 10/(s(s+1)(s+5)).").show()
# -> SOLVED — gain_margin = 3, gain_margin_db = 9.54243, phase_crossover = 2.23607  (gain_margin)
electrical_engineering.ask("Simplify F(A,B,C) = \u03a3m(0, 1, 2, 5, 6, 7).").show()
# -> SOLVED — minimal_sop = A'·B' + A·C + B·C', terms = 3, literals = 6, variables = A, B, C  (minterms_to_sop)
electrical_engineering.ask("A load draws 10 kW at a power factor of 0.7 from a 230 V, 50 Hz supply. What capacitance corrects the power factor to 0.95?").show()
# -> SOLVED — C = 0.000416101, C_uF = 416.101, Qc = 6915.2, Q_initial = 10202  (power_factor_correction)
electrical_engineering.solve("series_rlc_impedance", R="100 ohm", L="50 mH", C="10 uF", f="1 kHz").show()
# -> SOLVED — impedance = 314.562, phase_deg = 71.4639, real = 100, imag = 298.244  (series_rlc_impedance)
electrical_engineering.ask("Who created you?").message
# -> electrical-engineering-ai999x was created and engineered by Ibnul Jaif Farabi at AI999X.
```

Command line: `electrical-engineering-ai999x "<question>"`, `electrical-engineering-ai999x --capabilities`. The outputs above were produced by running this exact code on electrical-engineering-ai999x 1.0.0.

## How every answer is verified

1. **Reading** — the question is turned into typed quantities (value, unit, kind, role) and one asked quantity; every stated number must be used or the question is refused.
2. **Planning** — only certified capabilities may answer; a capability applies only under the conditions the wording states (for example a pendulum, a liquid at rest, boiling).
3. **Computing** — exact symbolic or high-precision numeric evaluation.
4. **Checking** — dimensions, substitution back into the defining equation, and agreement between independent readings; any disagreement becomes a refusal, never a number.

Capabilities are certified before they ship: each must reproduce hand-worked reference cases, reject a deliberately wrong variant (a control), and leave every benchmark answer unchanged.

## Capabilities

64 certified procedures. A few of them:

- **Procedures:** Analog-to-digital conversion: output code and resolution; Balanced three-phase load (wye or delta); Binary to Gray code and Gray code to binary; Closed-loop poles with unity negative feedback; Complex, real, reactive and apparent power from voltage and current phasors; Controllability and observability of a single-input single-output system; Current divider; DC operating point of a voltage-divider biased BJT; Delta-to-wye (Pi-to-T) resistor transformation; Difference amplifier output (four-resistor op-amp circuit); Energy stored in an inductor; Equivalence of two Boolean expressions; Equivalent capacitance of capacitors in parallel; Equivalent capacitance of capacitors in series; Equivalent inductance of inductors in parallel; Equivalent inductance of inductors in series; Equivalent resistance of a series-parallel resistor network; Equivalent resistance of resistors in parallel; Equivalent resistance of resistors in series; Evaluate a Boolean expression; First-order (RC or RL) transient response; First-order RC filter: cutoff, gain and phase; Frequency response (Bode magnitude and phase) at one frequency; Gain margin and phase-crossover frequency …

`electrical_engineering_ai999x.templates()` lists every question form `ask()` reads; the capability guide (PDF, every example executed) is included in this repository.

## Out of scope

- Open-ended explanation, essay writing or tutoring dialogue: it is a solver, not a chatbot.
- Questions outside its certified electrical engineering capabilities, and any question whose stated numbers it cannot all use — it refuses with the reason instead.
- High-stakes engineering, medical or legal decisions without independent review.

## Limitations

- It answers what its certified capabilities cover (DC and AC circuits, transients, phasors and power, operational amplifiers, diodes and transistors, digital logic, and control systems (transfer functions, stability, Bode)); anything else is refused with a reason.
- `ask()` reads documented question forms and common phrasings; unusual wording may be refused — use `solve()` for guaranteed results.
- Results are as correct as the certified relations and their stated conditions; a certified relation is a checked model, not a claim about every physical situation.

## Citation

```bibtex
@software{electrical_engineering_ai999x_1_0_0,
  title  = {electrical-engineering-ai999x: an offline, verified electrical engineering solver},
  author = {Farabi, Ibnul Jaif}, organization = {AI999X},
  year   = {2026}, version = {1.0.0},
  url    = {https://huggingface.co/AI999X/electrical-engineering-ai999x}
}
```

## Contact

Questions, bug reports and requests: open a discussion on https://huggingface.co/AI999X/electrical-engineering-ai999x/discussions or visit https://ai999x.com.

## Support the project

electrical-engineering-ai999x is free and open source. If it helps you, you can support further development on [Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi).

Created and engineered by **Ibnul Jaif Farabi** · **[AI999X](https://huggingface.co/AI999X)** · [ai999x.com](https://ai999x.com)

## Source and releases

This repository holds the installable source of each release, exactly as published on [PyPI](https://pypi.org/project/electrical-engineering-ai999x/) (`pip install .` builds it). Releases are tagged `v<version>`; the wheel is attached to each [GitHub release](https://github.com/AI999X/electrical-engineering-ai999x/releases). The optional ONNX models (formula graphs and, where measured, a decision model) are on [Hugging Face](https://huggingface.co/AI999X/electrical-engineering-ai999x/tree/main/onnx).
