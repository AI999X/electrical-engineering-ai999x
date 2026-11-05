# electrical-engineering-ai999x — offline, verified electrical engineering solver with step-by-step answers

[![tests](https://github.com/AI999X/electrical-engineering-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/electrical-engineering-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/electrical-engineering-ai999x)](https://pypi.org/project/electrical-engineering-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/electrical-engineering-ai999x)

**[Desktop app & command line](https://www.ai999x.com/desktop-cli/)** · **[Compute in your browser](https://www.ai999x.com/models/electrical-engineering-ai999x/#try)** · **[Install (Python)](https://pypi.org/project/electrical-engineering-ai999x/)** · **[Guides](https://huggingface.co/AI999X/electrical-engineering-ai999x/tree/main/docs)** · **[Evaluation](https://huggingface.co/AI999X/electrical-engineering-ai999x/tree/main/evaluation)** · **[AI999X on Hugging Face](https://huggingface.co/AI999X)** · **[Website](https://ai999x.com)** · **[Support: Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi)**

License: Apache-2.0 · Author: Ibnul Jaif Farabi (creator and engineer) · Organization: AI999X · Version 1.0.3

Field: **Engineering** · Tasks: Question Answering, Token Classification, Text Classification, Sentence Similarity, Feature Extraction

electrical-engineering-ai999x is an offline, deterministic Python solver that answers electrical engineering questions with verified results from 64 certified capabilities, gives 326 documented answers identically on Windows and Linux, and refuses rather than guesses when an answer cannot be verified.

**electrical-engineering-ai999x** is a compact, deterministic electrical engineering engine for Python. It reads a question in plain English (or a typed call), picks a certified electrical engineering capability, computes the answer exactly, checks it, and returns the result with its steps, units, LaTeX and assumptions — or an honest refusal that says what is missing. It runs fully offline on any CPU, needs no GPU, no API key and no internet, and is built to give **zero wrong answers**: when it is not sure, it refuses instead of guessing.

[![Buy me a coffee](https://img.shields.io/badge/Buy%20me%20a%20coffee-support%20development-FFDD00?logo=buymeacoffee&logoColor=black)](https://buymeacoffee.com/ibnuljaiffarabi) ![License](https://img.shields.io/badge/license-Apache--2.0-blue) ![Python](https://img.shields.io/badge/python-3.10%20%7C%203.11%20%7C%203.12%20%7C%203.13%20%7C%203.14-blue) ![Offline](https://img.shields.io/badge/runs-offline%20on%20CPU-green) ![Rust](https://img.shields.io/badge/Rust-Windows%20%7C%20Linux%20%7C%20WebAssembly-orange)

**Run it your way.** electrical-engineering-ai999x comes as the AI999X desktop app and as the command-line program `electrical-engineering` for Windows and Linux, runs in your browser as WebAssembly, and installs as a Python package. The desktop app, the command line and the browser run the same compiled Rust program on your own device: offline, no Python needed.

## Get it

| Way to run it | For | Get it |
|---|---|---|
| **AI999X Electrical Engineering desktop app** 1.2.0 | Windows, Linux — this model in its own app: search, ask in words, forms, live figures, explanations, offline | [Windows (x86-64)](https://www.ai999x.com/downloads/desktop/1.2.0/ai999x-electrical-engineering-1.2.0-windows-x86_64.zip) · [Linux (x86-64)](https://www.ai999x.com/downloads/desktop/1.2.0/ai999x-electrical-engineering-1.2.0-linux-x86_64.tar.gz) |
| **AI999X desktop app** 1.2.0 | Windows, Linux — every AI999X model in one app: search, ask in words, forms, live figures, explanations, offline | [Windows (x86-64)](https://www.ai999x.com/downloads/desktop/1.2.0/ai999x-1.2.0-windows-x86_64.zip) · [Linux (x86-64)](https://www.ai999x.com/downloads/desktop/1.2.0/ai999x-1.2.0-linux-x86_64.tar.gz) |
| **Command line** (`electrical-engineering`) | Windows, Linux — one program per model, offline, no Python | [Windows (x86-64)](https://www.ai999x.com/downloads/desktop/1.2.0/ai999x-cli-1.2.0-windows-x86_64.zip) · [Linux (x86-64)](https://www.ai999x.com/downloads/desktop/1.2.0/ai999x-cli-1.2.0-linux-x86_64.tar.gz) |
| **In your browser** | any device — nothing to install | [Compute on the model page](https://www.ai999x.com/models/electrical-engineering-ai999x/#try) |
| **Python** | Python 3.10+ | `pip install electrical-engineering-ai999x` |

Checksums: [SHA256SUMS](https://www.ai999x.com/downloads/desktop/1.2.0/SHA256SUMS) · every version: [https://www.ai999x.com/downloads/desktop/latest/](https://www.ai999x.com/downloads/desktop/latest/) · install guide: [desktop & command line](https://www.ai999x.com/desktop-cli/)

## Rust versions

Electrical Engineering also comes as native programs compiled from Rust, version 1.2.0 of the AI999X desktop app and command line: they run offline on your computer and need no Python. Each file is in this repository's `rust/` folder, on the [GitHub release v1.0.3](https://github.com/AI999X/electrical-engineering-ai999x/releases/tag/v1.0.3) and on the website.

| Program | System | File | Size | Download | Use |
|---|---|---|---|---|---|
| Desktop app (Electrical Engineering) | Windows x86-64 | `ai999x-electrical-engineering-1.2.0-windows-x86_64.zip` | 7.4 MB | [Hub](https://huggingface.co/AI999X/electrical-engineering-ai999x/resolve/main/rust/ai999x-electrical-engineering-1.2.0-windows-x86_64.zip) · [GitHub](https://github.com/AI999X/electrical-engineering-ai999x/releases/download/v1.0.3/ai999x-electrical-engineering-1.2.0-windows-x86_64.zip) · [website](https://www.ai999x.com/downloads/desktop/1.2.0/ai999x-electrical-engineering-1.2.0-windows-x86_64.zip) | unzip, then double-click `ai999x-electrical-engineering.exe` (no installer) |
| Desktop app (Electrical Engineering) | Linux x86-64 | `ai999x-electrical-engineering-1.2.0-linux-x86_64.tar.gz` | 8.3 MB | [Hub](https://huggingface.co/AI999X/electrical-engineering-ai999x/resolve/main/rust/ai999x-electrical-engineering-1.2.0-linux-x86_64.tar.gz) · [GitHub](https://github.com/AI999X/electrical-engineering-ai999x/releases/download/v1.0.3/ai999x-electrical-engineering-1.2.0-linux-x86_64.tar.gz) · [website](https://www.ai999x.com/downloads/desktop/1.2.0/ai999x-electrical-engineering-1.2.0-linux-x86_64.tar.gz) | extract, then run `./ai999x-electrical-engineering` (X11 or Wayland desktop) |
| Command line (`electrical-engineering`) | Windows x86-64 | `ai999x-electrical-engineering-cli-1.2.0-windows-x86_64.zip` | 3.1 MB | [Hub](https://huggingface.co/AI999X/electrical-engineering-ai999x/resolve/main/rust/ai999x-electrical-engineering-cli-1.2.0-windows-x86_64.zip) · [GitHub](https://github.com/AI999X/electrical-engineering-ai999x/releases/download/v1.0.3/ai999x-electrical-engineering-cli-1.2.0-windows-x86_64.zip) · [website (all models)](https://www.ai999x.com/downloads/desktop/1.2.0/ai999x-cli-1.2.0-windows-x86_64.zip) | unzip, then `electrical-engineering.exe ask "<question>"` or `electrical-engineering.exe solve <capability> name=value` |
| Command line (`electrical-engineering`) | Linux x86-64 | `ai999x-electrical-engineering-cli-1.2.0-linux-x86_64.tar.gz` | 3.1 MB | [Hub](https://huggingface.co/AI999X/electrical-engineering-ai999x/resolve/main/rust/ai999x-electrical-engineering-cli-1.2.0-linux-x86_64.tar.gz) · [GitHub](https://github.com/AI999X/electrical-engineering-ai999x/releases/download/v1.0.3/ai999x-electrical-engineering-cli-1.2.0-linux-x86_64.tar.gz) · [website (all models)](https://www.ai999x.com/downloads/desktop/1.2.0/ai999x-cli-1.2.0-linux-x86_64.tar.gz) | extract, then `./electrical-engineering ask "<question>"` or `./electrical-engineering solve <capability> name=value` |

The command-line archive holds `electrical-engineering` with a README of its commands and examples, the licence and the notice. Checksums: [`rust/SHA256SUMS`](https://huggingface.co/AI999X/electrical-engineering-ai999x/resolve/main/rust/SHA256SUMS) (`sha256sum -c SHA256SUMS`).

## Quick start

### Desktop app

Download **AI999X Electrical Engineering** for your system (above), unpack it and start it; type a question in words or pick a capability, fill in the form and press Compute. Figures and an explanation of the answer appear beside it. The AI999X desktop app has the same, with Electrical Engineering among all AI999X models.

Linux, from a terminal:

```bash
curl -LO https://www.ai999x.com/downloads/desktop/1.2.0/ai999x-electrical-engineering-1.2.0-linux-x86_64.tar.gz
curl -LO https://www.ai999x.com/downloads/desktop/1.2.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
tar xzf ai999x-electrical-engineering-1.2.0-linux-x86_64.tar.gz
./ai999x-electrical-engineering-1.2.0-linux-x86_64/ai999x-electrical-engineering
```

### Command line

Linux:

```bash
curl -LO https://www.ai999x.com/downloads/desktop/1.2.0/ai999x-cli-1.2.0-linux-x86_64.tar.gz
curl -LO https://www.ai999x.com/downloads/desktop/1.2.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
mkdir -p ~/.local/share && tar xzf ai999x-cli-1.2.0-linux-x86_64.tar.gz -C ~/.local/share
echo 'export PATH="$HOME/.local/share/ai999x-cli-1.2.0-linux-x86_64:$PATH"' >> ~/.bashrc && source ~/.bashrc
electrical-engineering --version
```

Windows (PowerShell):

```powershell
$F = "ai999x-cli-1.2.0-windows-x86_64.zip"
Invoke-WebRequest "https://www.ai999x.com/downloads/desktop/1.2.0/$F" -OutFile $F
Invoke-WebRequest "https://www.ai999x.com/downloads/desktop/1.2.0/SHA256SUMS" -OutFile SHA256SUMS
(Get-FileHash $F -Algorithm SHA256).Hash.ToLower() -eq ((Select-String -SimpleMatch $F SHA256SUMS).Line -split ' ')[0]
Expand-Archive $F -DestinationPath "$env:LOCALAPPDATA\AI999X" -Force
$D = "$env:LOCALAPPDATA\AI999X\ai999x-cli-1.2.0-windows-x86_64"; $env:Path += ";$D"
[Environment]::SetEnvironmentVariable("Path", [Environment]::GetEnvironmentVariable("Path", "User") + ";$D", "User")
electrical-engineering --version
```

Every command below was run as shown; the text under it is what the program printed.

**Ask in words** — Write the question as you would say it: the program reads the quantities and their units, picks the capability and answers.

```console
$ electrical-engineering ask "Find the Thevenin equivalent between nodes 2 and 0 of the circuit: V1 1 0 12; R1 1 2 1000; R2 2 0 2000."
Read as: Thevenin equivalent of a DC circuit between two nodes
Vth = 8 V
Rth = 666.6666667 ohm
  by: Thevenin equivalent of a DC circuit between two nodes  (open-circuit voltage by modified nodal analysis; resistance by a 1 A test current with the independent sources set to zero)
  node_a = 2
  node_b = 0
  netlist = V1 1 0 12; R1 1 2 1000; R2 2 0 2000.
  assumes: both nodes in the circuit; no ideal voltage source directly across the terminals; a unique solution (no floating node)
```

**What a capability needs** — The equation or method, every input and output with its SI unit. Read this before solve: it tells you the input names.

```console
$ electrical-engineering info thevenin_equivalent
Thevenin equivalent of a DC circuit between two nodes
  open-circuit voltage by modified nodal analysis; resistance by a 1 A test current with the independent sources set to zero
  in   netlist  elements separated by ';': R<name> <node> <node> <ohms>, V<name> <node+> <node-> <volts>, I<name> <node> <node> <amperes>; node 0 is ground (text) []
  in   node_a   the terminal node (the + side of the equivalent) (text) []
  in   node_b   the reference terminal node (text) []
  out  Vth      Thevenin (open-circuit) voltage, node a relative to node b [V]
  out  Rth      Thevenin resistance [ohm]
```

**Solve** — Inputs as name=value, each with any unit of the right kind. The answer lists every output, the method used, each input converted to SI, and the conditions the answer assumes.

```console
$ electrical-engineering solve thevenin_equivalent node_a=2 node_b=0 "netlist=V1 1 0 12; R1 1 2 1000; R2 2 0 2000."
Vth = 8 V
Rth = 666.6666667 ohm
  by: Thevenin equivalent of a DC circuit between two nodes  (open-circuit voltage by modified nodal analysis; resistance by a 1 A test current with the independent sources set to zero)
  node_a = 2
  node_b = 0
  netlist = V1 1 0 12; R1 1 2 1000; R2 2 0 2000.
  assumes: both nodes in the circuit; no ideal voltage source directly across the terminals; a unique solution (no floating node)
```

**Draw the figures** — The live figures for your own inputs, written as SVG files into the folder given with --out (here the current folder); open them in any browser. Without --out the SVG text comes back as JSON.

```console
$ electrical-engineering figures thevenin_equivalent node_a=2 node_b=0 "netlist=V1 1 0 12; R1 1 2 1000; R2 2 0 2000." --out .
[
  {
    "slug": "thevenin-equivalent-live-1",
    "title": "Thevenin (open-circuit) voltage, node a relat... as the number 0 in node_b (0) varies",
    "caption": "Thevenin (open-circuit) voltage, node a relat... as the number in node_b runs from 0 to 1, every other input as you entered it (2 solutions). Marked: your the number in node_b = 0 gives Vth = 8. Your inputs: node_a = 2, node_b = 0, netlist = V1 1 0 12; R1 1 2 1000; R....",
    "file": "./thevenin-equivalent-live-1.svg"
  },
  {
    "slug": "thevenin-equivalent-live-2",
    "title": "Thevenin (open-circuit) voltage, node a relat... as the number 1 in netlist (V1 1 0 12; R1 1 2 100...) varies",
    "caption": "Thevenin (open-circuit) voltage, node a relat... as the number in netlist runs from 1 to 5, every other input as you entered it (4 solutions). Values in this range where the inputs admit no answer (1 of 5) are left out. Marked: your the number in netlist = 1 gives Vth = 8. Your inputs: node_a = 2, node_b = 0, netlist = V1 1 0 12; R1 1 2 1000; R....",
    "file": "./thevenin-equivalent-live-2.svg"
  }
]
```

**Many questions at once** — One JSON request per line in, one JSON answer per line out, in order: for scripts, notebooks and agents (the program starts once). In PowerShell: Get-Content cases.jsonl | electrical-engineering batch

`cases.jsonl`:

```json
{"key": "thevenin_equivalent", "inputs": {"node_a": "2", "node_b": "0", "netlist": "V1 1 0 12; R1 1 2 1000; R2 2 0 2000."}}
{"key": "gain_margin", "inputs": {"num": "[10]", "den": "[1, 6, 5, 0]"}}
```

```console
$ electrical-engineering batch < cases.jsonl
{"outcome":"SOLVED","message":"","value":8.0,"unit":"V","solved_for":"Vth","values":[["Vth",8.0,"V"],["Rth",666.6666666666666,"ohm"]],"target":"Vth","phrase":"Thevenin (open-circuit) voltage, node a relative to node b","assumptions":["both nodes in the circuit","no ideal voltage source directly across the terminals","a unique solution (no floating node)"],"name":"Thevenin equivalent of a DC circuit between two nodes","equation":"open-circuit voltage by modified nodal analysis; resistance by a 1 A test current with the independent sources set to zero"}
{"outcome":"SOLVED","message":"","value":3.0,"unit":"","solved_for":"gain_margin","values":[["gain_margin",3.0,""],["gain_margin_db",9.542425094393248,""],["phase_crossover",2.23606797749979,"rad/s"]],"target":"gain_margin","phrase":"gain margin as a ratio","assumptions":["a proper G(s)","exactly one phase crossover with G negative real"],"name":"Gain margin and phase-crossover frequency","equation":"the phase crossover as the positive real root of Im(N(jw) conj D(jw)) with Re G < 0 (Aberth iteration, Newton-polished)"}
```

### In your browser

Open [Electrical Engineering on ai999x.com](https://www.ai999x.com/models/electrical-engineering-ai999x/#try), choose a capability and press **Compute**: the model runs on your own device as WebAssembly; nothing is sent anywhere. Change any input (any unit of the right kind) and the answer, its explanation and the figures follow.

In your own page: `electrical_engineering_ai999x.wasm` and the loader `ax-wasm.js` (https://www.ai999x.com/compute/rust/ax-wasm.js).

### Python

```bash
pip install electrical-engineering-ai999x
```

## Worked examples in every form

The same worked example from the command line, from Python and in the browser. Each command-line output is what the program printed.

### Balanced three-phase load (wye or delta)

Phase voltage, phase and line currents, real, reactive and apparent power and the power factor of a balanced load with impedance R + jX per phase.

Command line:

```console
$ electrical-engineering solve three_phase_load connection=wye "VL=400 V" "R=8 ohm" "X=6 ohm"
phase_voltage = 230.9401077 V
phase_current = 23.09401077 A
line_current = 23.09401077 A
P = 12800 W
Q = 9600 W
S = 16000 W
power_factor = 0.8
  by: Balanced three-phase load (wye or delta)  (wye: V_ph = V_L/sqrt(3), I_L = I_ph; delta: V_ph = V_L, I_L = sqrt(3) I_ph; P = 3 I_ph^2 R)
  VL = 400 V
  R = 8 ohm
  X = 6 ohm
  connection = wye
  assumes: balanced load and supply; R >= 0
```

Python:

```python
import electrical_engineering_ai999x as m

a = m.solve("three_phase_load", connection="wye", VL="400 V", R="8 ohm", X="6 ohm")
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/electrical-engineering-ai999x/#cap-three_phase_load) and press **Compute in your browser**.

_What this means:_ This calculation determines the electrical characteristics of a balanced three-phase load connected in a wye configuration. Using a line-to-line voltage of 400 V and per-phase impedance defined by resistance 8 ohm and reactance 6 ohm, it computes the phase voltage as 230.94 V, phase current as 23.094 A, and line current as 23.094 A. The resulting total real power is 12800 W, reactive power is 9600 W, apparent power is 16000 W, and the power factor is 0.8. These values indicate how much useful work is performed versus how much energy oscillates between source and load.

### Complex, real, reactive and apparent power from voltage and current phasors

S = V I* for rms phasors: real power P, reactive power Q, apparent power |S|, the power factor and whether it lags or leads.

Command line:

```console
$ electrical-engineering solve complex_power "V=230 V" V_phase=0 "I=10 A" I_phase=-30
P = 1991.858429 W
Q = 1150 W
S = 2300 W
power_factor = 0.8660254038
nature = lagging
  by: Complex, real, reactive and apparent power from voltage and current phasors  (S = V I* (complex conjugate of the current))
  V = 230 V
  V_phase = 0
  I = 10 A
  I_phase = -30
  assumes: rms phasors; sinusoidal steady state
```

Python:

```python
import electrical_engineering_ai999x as m

a = m.solve("complex_power", V="230 V", V_phase="0", I="10 A", I_phase="-30")
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/electrical-engineering-ai999x/#cap-complex_power) and press **Compute in your browser**.

_What this means:_ This calculation determines the total electrical power flow using voltage and current phasors. Multiplying the voltage phasor by the conjugate of the current phasor gives the complex power 2300 W. The real component represents active power 1991.86 W, which performs useful work, while the imaginary component indicates reactive power 1150 W, associated with energy storage in magnetic or electric fields. The ratio of active to apparent power defines the power factor 0.866025, indicating how efficiently the circuit utilizes the supplied energy.

### Energy stored in an inductor

The magnetic energy L I^2/2 and the flux linkage L I of an inductor carrying current I.

Command line:

```console
$ electrical-engineering solve inductor_energy "L=0.05 H" "I=2 A"
energy = 0.1 J
flux_linkage = 0.1
  by: Energy stored in an inductor  (W = L I^2 / 2)
  L = 0.05 H
  I = 2 A
  assumes: a linear inductor (no saturation)
```

Python:

```python
import electrical_engineering_ai999x as m

a = m.solve("inductor_energy", L="0.05 H", I="2 A")
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/electrical-engineering-ai999x/#cap-inductor_energy) and press **Compute in your browser**.

_What this means:_ This calculation determines the magnetic energy stored in an inductor and its flux linkage based on the given inductance of 0.05 H and current of 2 A. The resulting energy of 0.1 J represents the work done to establish the magnetic field, while the flux linkage of 0.1 indicates the total magnetic flux coupled with the coil turns. These values are critical for analyzing circuit transients and power conversion efficiency.

### First-order RC filter: cutoff, gain and phase

The cutoff frequency 1/(2 pi R C) of an RC low-pass or high-pass filter and its gain (ratio and dB) and phase at a frequency.

Command line:

```console
$ electrical-engineering solve rc_filter "R=1000 ohm" "C=1e-07 F" "f=2000 Hz" filter=lowpass
fc = 1591.549431 Hz
gain = 0.6226769923
gain_db = -4.11474362
phase_deg = -51.48811275
  by: First-order RC filter: cutoff, gain and phase  (H = 1/(1 + j f/fc) (low-pass) or (j f/fc)/(1 + j f/fc) (high-pass))
  R = 1000 ohm
  C = 1e-07 F
  f = 2000 Hz
  filter = lowpass
  assumes: unloaded output; R, C, f > 0
```

Python:

```python
import electrical_engineering_ai999x as m

a = m.solve("rc_filter", R="1000 ohm", C="1e-07 F", f="2000 Hz", filter="lowpass")
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/electrical-engineering-ai999x/#cap-rc_filter) and press **Compute in your browser**.

_What this means:_ This calculation determines the cutoff frequency, magnitude gain, decibel gain, and phase shift for a first-order RC filter. Using resistance 1000 ohm, capacitance 1e-07 F, and signal frequency 2000 Hz, it finds that the cutoff frequency is 1591.55 Hz. The resulting voltage gain ratio is 0.622677, which corresponds to -4.11474 in decibels, with a phase shift of -51.4881 degrees between output and input.

## Documentation

- **Rust user guide** — the command line, every capability with a command and what it printed: [PDF](https://huggingface.co/AI999X/electrical-engineering-ai999x/blob/main/docs/electrical_engineering_ai999x_rust_guide.pdf) · [Markdown](https://huggingface.co/AI999X/electrical-engineering-ai999x/blob/main/docs/electrical_engineering_ai999x_rust_guide.md)
- **Python guide** — every capability with a Python example: [PDF](https://huggingface.co/AI999X/electrical-engineering-ai999x/blob/main/docs/electrical_engineering_ai999x_capabilities.pdf) · [Markdown](https://huggingface.co/AI999X/electrical-engineering-ai999x/blob/main/docs/electrical_engineering_ai999x_capabilities.md)
- **Desktop app & command line** — downloads, install, ask and search, figures, offline use, checksums: https://www.ai999x.com/desktop-cli/
- **Model page** — compute in your browser, every capability explained: https://www.ai999x.com/models/electrical-engineering-ai999x/

## Highlights

- **Verified, not generated.** Every answer is computed by a certified electrical engineering capability and checked (substituted back, dimension-checked or compared with an independent computation) before it is returned.
- **Zero wrong answers by design.** Seven explicit outcomes — SOLVED, NEEDS_INPUT, AMBIGUOUS, INVALID_INPUT, OUT_OF_SCOPE, UNRECOGNIZED_FORMAT, INTERNAL_ERROR — never a silent guess. Every one of its 326 documented answers is reproduced exactly on every tested platform.
- **Plain-English questions and a guaranteed typed interface.** `ask()` reads documented question forms; `solve(capability_id, **inputs)` always solves a supported task with valid inputs.
- **Explains itself.** Steps, formulas in LaTeX, units, assumptions and the model used, ready for notebooks, tutoring apps and homework checkers.
- **Native and in the browser.** Every capability also computes natively in Rust: the desktop app and the `electrical-engineering` command line for Windows and Linux, and WebAssembly on the model page — no Python, no network.
- **Small and portable.** Pure Python (sympy, mpmath); identical answers on Windows and Linux (Python 3.10, 3.11, 3.12, 3.13, 3.14).
- **Coverage:** DC and AC circuits, transients, phasors and power, operational amplifiers, diodes and transistors, digital logic, and control systems (transfer functions, stability, Bode).

## Model overview

| Property | Value |
|---|---|
| Package | `electrical-engineering-ai999x` 1.0.3 (import `electrical_engineering_ai999x`) |
| Type | Deterministic neuro-symbolic electrical engineering engine (rule-based reading + certified symbolic capabilities) |
| Capabilities | 64 certified procedures |
| Knowledge pack | version 1.0.0 |
| Golden answers | 326 documented answers, identical on every tested platform |
| Platforms tested | Windows (Python 3.13), Linux x86-64 (Python 3.10, 3.11, 3.12, 3.13, 3.14) |
| Runtime | CPU only, offline; no GPU, no network, no API key |
| License | Apache-2.0 |
| Creator / engineer | Ibnul Jaif Farabi |
| Organization | AI999X |

## Python quick start

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

The Python package also installs a Python command, `electrical-engineering-ai999x "<question>"`; for the native program use `electrical-engineering` (above). The outputs above were produced by running this exact code on electrical-engineering-ai999x 1.0.3.

## Find the right capability, and draw it

`electrical_engineering.search()` finds which certified capability a question is about — across **every** AI999X model, so a question for another model points you to it. It runs **[ai999x-encoder](https://huggingface.co/AI999X/ai999x-encoder)**, AI999X's own capability encoder (4.7M parameters, a 4.9 MB ONNX model, offline; the root `model.safetensors` of this repository): `pip install "electrical-engineering-ai999x[search]"`. `ask(question, suggest=True)` adds the same pointers to a question it does not answer; an answer itself never changes.

```python
import electrical_engineering_ai999x as electrical_engineering

electrical_engineering.search("Solve x^2 - 5x + 6 = 0", k=2)
# -> Roots of a quadratic equation (math-foundations-ai999x, 0.63); Roots of a cubic equation (math-foundations-ai999x, 0.59)
```

## How every answer is verified

1. **Reading** — the question is turned into typed quantities (value, unit, kind, role) and one asked quantity; every stated number must be used or the question is refused.
2. **Planning** — only certified capabilities may answer; a capability applies only under the conditions the wording states (for example a pendulum, a liquid at rest, boiling).
3. **Computing** — exact symbolic or high-precision numeric evaluation.
4. **Checking** — dimensions, substitution back into the defining equation, and agreement between independent readings; any disagreement becomes a refusal, never a number.

Capabilities are certified before they ship: each must reproduce hand-worked reference cases, reject a deliberately wrong variant (a control), and leave every benchmark answer unchanged.

## Python worked examples, with their steps

Each question was put to `electrical-engineering-ai999x`; the answer is the one it returned, with the steps it took.

**Thevenin equivalent of a DC circuit between two nodes** · _DC circuits_

> Find the Thevenin equivalent between nodes 2 and 0 of the circuit: V1 1 0 12; R1 1 2 1000; R2 2 0 2000.

→ **Vth = 8 V, Rth = 666.6666667 ohm**

<details><summary>Steps</summary>

- Thevenin equivalent of a DC circuit between two nodes: node_a = 2, node_b = 0, netlist = V1 1 0 12; R1 1 2 1000; R2 2 0 2000. → Vth = 8 V, Rth = 666.6666667 ohm

</details>

**Gain margin and phase-crossover frequency** · _Control systems_

> Find the gain margin of G(s) = 10/(s(s+1)(s+5)).

→ **gain_margin = 3, gain_margin_db = 9.542425094, phase_crossover = 2.236067977 rad/s**

<details><summary>Steps</summary>

- Gain margin and phase-crossover frequency: num = [10], den = [1, 6, 5, 0] → gain_margin = 3, gain_margin_db = 9.542425094, phase_crossover = 2.236067977 rad/s

</details>

**Minimal sum of products from minterms (with don't-cares)** · _Digital logic_

> Simplify F(A,B,C) = Σm(0, 1, 2, 5, 6, 7).

→ **minimal_sop = A'·B' + A·C + B·C', terms = 3, literals = 6, variables = A, B, C**

<details><summary>Steps</summary>

- Minimal sum of products from minterms (with don't-cares): minterms = [0, 1, 2, 5, 6, 7], variables = 3 → minimal_sop = A'·B' + A·C + B·C', terms = 3, literals = 6, variables = A, B, C

</details>

**Power-factor correction capacitor** · _AC circuits_

> A load draws 10 kW at a power factor of 0.7 from a 230 V, 50 Hz supply. What capacitance corrects the power factor to 0.95?

→ **C = 0.000416101396 F, C_uF = 416.101396, Qc = 6915.19956 W, Q_initial = 10202.04061 W, Q_final = 3286.841052 W**

<details><summary>Steps</summary>

- Power-factor correction capacitor: P = 10000 W, V = 230 V, f = 50 Hz, pf_initial = 0.7, pf_target = 0.95 → C = 0.000416101396 F, C_uF = 416.101396, Qc = 6915.19956 W, Q_initial = 10202.04061 W, Q_final = 3286.841052 W

</details>

**Impedance of a series RLC circuit** · _AC circuits_

`m.solve("series_rlc_impedance", R="100 ohm", L="50 mH", C="10 uF", f="1 kHz")`

→ **impedance = 314.5621512 ohm, phase_deg = 71.46389381, real = 100 ohm, imag = 298.243771 ohm, XL = 314.1592654 ohm, XC = 15.91549431 ohm**

<details><summary>Steps</summary>

- Impedance of a series RLC circuit: R = 100 ohm, L = 50 mH, C = 10 uF, f = 1 kHz → impedance = 314.5621512 ohm, phase_deg = 71.46389381 ohm, real = 100 ohm, imag = 298.243771 ohm, XL = 314.1592654 ohm, XC = 15.91549431 ohm

</details>

**DC operating point of a voltage-divider biased BJT** · _Diodes and transistors_

> A voltage-divider biased BJT has VCC = 12 V, R1 = 10 kohm, R2 = 2.2 kohm, RC = 3.6 kohm, RE = 1 kohm, beta = 100 and VBE = 0.7 V. Find the collector current.

→ **IC = 0.001424015309 A, IC_mA = 1.424015309, IB = 1.424015309e-05 A, IE = 0.001438255462 A, VB = 2.138255462 V, VE = 1.438255462 V, VC = 6.873544889 V, VCE = 5.435289428 V**

<details><summary>Steps</summary>

- DC operating point of a voltage-divider biased BJT: VCC = 12 V, VBE = 0.7 V, R1 = 10000 ohm, R2 = 2200 ohm, RC = 3600 ohm, RE = 1000 ohm, beta = 100 → IC = 0.001424015309 A, IC_mA = 1.424015309, IB = 1.424015309e-05 A, IE = 0.001438255462 A, VB = 2.138255462 V, VE = 1.438255462 V, VC = 6.873544889 V, VCE = 5.435289428 V

</details>

## Figures

Every point was computed by `electrical-engineering-ai999x`.

**Series RLC step response: under-, critically and over-damped**

![Series RLC step response: under-, critically and over-damped — computed by electrical-engineering-ai999x](https://huggingface.co/AI999X/electrical-engineering-ai999x/resolve/main/figures/series-rlc-step-response.svg)

_Capacitor voltage of a series RLC circuit switched onto V = 12 V (L = 0.1 H, C = 0.0001 F) for three resistances, 121 instants each computed by the package: R = 10 Ω: ζ = 0.158114, underdamped; R = 63.2456 Ω: ζ = 1, critically damped; R = 150 Ω: ζ = 2.37171, overdamped. Marked: t = 0.005 s, vC = 10.4144 V (R = 10 Ω)._

**Bode magnitude and gain margin of a third-order loop**

![Bode magnitude and gain margin of a third-order loop — computed by electrical-engineering-ai999x](https://huggingface.co/AI999X/electrical-engineering-ai999x/resolve/main/figures/bode-point-bode.svg)

_Bode magnitude of G(s) = 10 / (s^3 + 6s^2 + 5s), 101 frequencies from 0.01 to 100 rad/s computed by the package. At the phase crossover ω = 2.23607 rad/s the gain is -9.54243 dB, so the gain margin returned by the package is 9.54243 dB (a factor 3); marked: ω = 2.23607 rad/s, |G| = -9.54243 dB._

**Phasors and their sum on a polar chart**

![Phasors and their sum on a polar chart — computed by electrical-engineering-ai999x](https://huggingface.co/AI999X/electrical-engineering-ai999x/resolve/main/figures/phasor-sum-phasor.svg)

_Three phasors 3∠0°, 4∠90° and 2∠210° on a polar chart (each converted to rectangular form by the package; rings every 1 unit of amplitude) and their sum returned by the package: 3.25695 ∠ 67.0887° = 1.26795 + 3j. Marked: the tip of the sum, real = 1.26795, imaginary = 3._

**Power triangle: real, reactive and apparent power**

![Power triangle: real, reactive and apparent power — computed by electrical-engineering-ai999x](https://huggingface.co/AI999X/electrical-engineering-ai999x/resolve/main/figures/complex-power-triangle.svg)

_Power triangle of a load with V = 230 V ∠ 0° and I = 10 A ∠ -30°, from the complex power the package returned: real power P = 1991.86 W, reactive power Q = 1150 var (lagging), apparent power S = 2300 VA, power factor 0.866025. Marked: the tip of S._

**Mesh analysis as matrices: resistances, sources, currents**

![Mesh analysis as matrices: resistances, sources, currents — computed by electrical-engineering-ai999x](https://huggingface.co/AI999X/electrical-engineering-ai999x/resolve/main/figures/mesh-currents-heatmap.svg)

_Mesh analysis of a three-loop circuit as matrices: the mesh-resistance matrix R (self resistances on the diagonal, shared resistances negative off it) and the loop EMFs E are the stated circuit (R I = E); the mesh currents I are the solution returned by the package: I1 = 0.864914 A, I2 = 0.320943 A, I3 = -0.157752 A (a negative current flows against the assumed direction). Each panel is shaded by its own largest magnitude; negative values in the second colour._

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
@software{electrical_engineering_ai999x_1_0_3,
  title  = {electrical-engineering-ai999x: an offline, verified electrical engineering solver},
  author = {Farabi, Ibnul Jaif}, organization = {AI999X},
  year   = {2026}, version = {1.0.3},
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
