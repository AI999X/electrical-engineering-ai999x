# Electrical Engineering: the desktop app, user guide

**Electrical Engineering**, desktop app 1.6.5 for Windows and Linux. **Creator / engineer:** Ibnul Jaif Farabi · **Company:** AI999X · support@ai999x.com

Electrical Engineering covers DC and AC circuits, transients, phasors and power, operational amplifiers, diodes and transistors, digital logic, and control systems (transfer functions, stability, Bode). Its desktop app holds all 92 calculations in 17 topics: pick one, change its inputs and the answer, a live scene and charts follow at once. It runs on your own computer with no network connection and nothing else to install. The pictures in this guide were taken in this app.

## Contents

- 1. Get it and start it
- 2. The window
- 3. Compute a calculation
- 4. Ask in words
- 5. The live scene and the charts
- 6. Light and dark
- 7. Every calculation
- 8. Other ways to use it

## 1. Get it and start it

| system | download |
|---|---|
| Windows x86-64 | https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-electrical-engineering-1.6.5-windows-x86_64.zip |
| Linux x86-64 | https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-electrical-engineering-1.6.5-linux-x86_64.tar.gz |

Windows: Download ai999x-electrical-engineering-1.6.5-windows-x86_64.zip, right-click it → Extract All, then open the folder and start ai999x-electrical-engineering.exe. If Windows SmartScreen asks, choose More info → Run anyway.

Linux:

```shell
curl -LO https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-electrical-engineering-1.6.5-linux-x86_64.tar.gz
curl -LO https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
tar xzf ai999x-electrical-engineering-1.6.5-linux-x86_64.tar.gz
./ai999x-electrical-engineering-1.6.5-linux-x86_64/ai999x-electrical-engineering
```

Checksums of every download: https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/SHA256SUMS. Install steps for every app: https://www.ai999x.com/desktop-cli/

## 2. The window

![The Electrical Engineering app, opened on Thevenin equivalent of a DC circuit between two nodes with its worked example.](https://www.ai999x.com/?ax_shot=electrical-engineering-ai999x/hero.webp)

*The Electrical Engineering app, opened on Thevenin equivalent of a DC circuit between two nodes with its worked example.*

On the left, a search box and the topics with their calculations. In the middle, the chosen calculation: its answer at the top, then the live scene (or its charts), what the answer means and the formula. On the right, its inputs, each with a slider where that makes sense, and the Compute and Reset buttons. At the bottom, a box for a question in words.

## 3. Compute a calculation

Open DC circuits › Thevenin equivalent of a DC circuit between two nodes (or type part of its name in the search box). It opens with a worked example filled in: “Find the Thevenin equivalent between nodes 2 and 0 of the circuit: V1 1 0 12; R1 1 2 1000; R2 2 0 2000.” The answer: Vth = 8 V, Rth = 666.6666667 ohm.

Change any input (type a number with its unit, or drag its slider) and the answer, the scene and the charts follow; Compute recomputes after typing, Reset brings back the worked example. An input outside the range the formula holds for is refused with the reason, never answered with a guessed number.

![Thevenin equivalent of a DC circuit between two nodes: the live scene of the worked example.](https://www.ai999x.com/?ax_shot=electrical-engineering-ai999x/scene-1.webp)

*Thevenin equivalent of a DC circuit between two nodes: the live scene of the worked example.*

## 4. Ask in words

Type a question as you would say it, for example “Find the Thevenin equivalent between nodes 2 and 0 of the circuit: V1 1 0 12; R1 1 2 1000; R2 2 0 2000.” The app reads the quantities and their units, picks the calculation and answers; the command-line program reads it the same way and printed:

```text
Read as: Thevenin equivalent of a DC circuit between two nodes
Vth = 8 V
Rth = 666.6666667 ohm
```

When a question names a quantity the app cannot place, it says what is missing instead of guessing.

## 5. The live scene and the charts

Every calculation draws a scene from its answer: it moves as an input changes. The Charts button shows charts that vary one input around your values; Enlarge opens one in a large view. Scenes from other topics:

![Gain margin and phase-crossover frequency](https://www.ai999x.com/?ax_shot=electrical-engineering-ai999x/scene-2.webp)

*Gain margin and phase-crossover frequency*

![Minimal sum of products from minterms (with don't-cares)](https://www.ai999x.com/?ax_shot=electrical-engineering-ai999x/scene-3.webp)

*Minimal sum of products from minterms (with don't-cares)*

![Power-factor correction capacitor](https://www.ai999x.com/?ax_shot=electrical-engineering-ai999x/scene-4.webp)

*Power-factor correction capacitor*

![Time at which a sinusoid first reaches a value](https://www.ai999x.com/?ax_shot=electrical-engineering-ai999x/scene-5.webp)

*Time at which a sinusoid first reaches a value*

![Cells with internal resistance: series and parallel batteries on a load](https://www.ai999x.com/?ax_shot=electrical-engineering-ai999x/scene-6.webp)

*Cells with internal resistance: series and parallel batteries on a load*

![Parallel-plate capacitor: capacitance, charge, field, flux density and energy](https://www.ai999x.com/?ax_shot=electrical-engineering-ai999x/scene-7.webp)

*Parallel-plate capacitor: capacitance, charge, field, flux density and energy*

![D.C. shunt generator: field and armature currents, generated emf and powers](https://www.ai999x.com/?ax_shot=electrical-engineering-ai999x/scene-8.webp)

*D.C. shunt generator: field and armature currents, generated emf and powers*

![Thevenin equivalent of a DC circuit between two nodes: a chart, enlarged.](https://www.ai999x.com/?ax_shot=electrical-engineering-ai999x/chart-1.webp)

*Thevenin equivalent of a DC circuit between two nodes: a chart, enlarged.*

![Gain margin and phase-crossover frequency: a chart, enlarged.](https://www.ai999x.com/?ax_shot=electrical-engineering-ai999x/chart-2.webp)

*Gain margin and phase-crossover frequency: a chart, enlarged.*

## 6. Light and dark

The app follows a light or a dark theme (the switch is in its top bar).

![The Electrical Engineering app in the dark theme.](https://www.ai999x.com/?ax_shot=electrical-engineering-ai999x/hero-dark.webp)

*The Electrical Engineering app in the dark theme.*

## 7. Every calculation

92 calculations in 17 topics; each has its own entry on the model's page (https://www.ai999x.com/models/electrical-engineering-ai999x/) with its formula, worked example and charts.

| topic | calculations |
|---|---|
| AC circuits | AC mesh currents from the phasor mesh equations (complex impedances), Balanced three-phase load (wye or delta), Complex, real, reactive and apparent power from voltage and current phasors, Energy stored in an inductor, First-order RC filter: cutoff, gain and phase, Ideal transformer: voltages, currents and reflected resistance, Impedance of a parallel RLC circuit, Impedance of a series RLC circuit, Node voltages of an AC circuit from its netlist (phasor nodal analysis), Norton equivalent of an AC circuit between two nodes (phasors, complex impedance), Phasor from polar to rectangular form, Phasor from rectangular to polar form, Power-factor correction capacitor, Resonance, Q factor and bandwidth of a parallel RLC circuit, Resonance, Q factor and bandwidth of a series RLC circuit, RMS and average value of a periodic waveform, Sum of sinusoids by phasor addition, Thevenin equivalent of an AC circuit between two nodes (phasors, complex impedance) |
| Alternating voltages and currents | Sinusoidal waveform v = Vm sin(wt + phi): frequency, period, phase, value at a time, form and peak factors, Time at which a sinusoid first reaches a value |
| Basic electrical quantities | Cells with internal resistance: series and parallel batteries on a load, Electrical energy and its cost, Resistance at a new temperature (temperature coefficient), Temperature coefficient of resistance from two readings |
| Capacitors | Parallel-plate capacitor: capacitance, charge, field, flux density and energy |
| Control systems | Closed-loop poles with unity negative feedback, Controllability and observability of a single-input single-output system, Frequency response (Bode magnitude and phase) at one frequency, Gain margin and phase-crossover frequency, Maximum phase lead of a lead compensator, Phase margin and gain-crossover frequency, Routh-Hurwitz stability test, Steady-state error of a unity-feedback system, Steady-state speed and current of a permanent-magnet DC motor, Step-response specifications of a standard second-order system, Transfer function of a single-input single-output state-space model |
| D.c. machines | D.C. shunt generator: field and armature currents, generated emf and powers, D.C. shunt motor: back emf, torque, output and efficiency, Generated emf of a d.c. machine (lap or wave winding), New speed of a d.c. motor after a change of load or flux |
| DC circuits | Current divider, Delta-to-wye (Pi-to-T) resistor transformation, Equivalent capacitance of capacitors in parallel, Equivalent capacitance of capacitors in series, Equivalent inductance of inductors in parallel, Equivalent inductance of inductors in series, Equivalent resistance of a series-parallel resistor network, Equivalent resistance of resistors in parallel, Equivalent resistance of resistors in series, Maximum power transfer, Mesh currents from the mesh equations (Kirchhoff's voltage law), Node voltages from the nodal equations (Kirchhoff's current law), Node voltages of a DC resistive circuit from its netlist, Norton equivalent of a DC circuit between two nodes, Thevenin equivalent of a DC circuit between two nodes, Voltage divider, Wheatstone bridge output, Wye-to-delta (T-to-Pi) resistor transformation |
| Digital logic | Analog-to-digital conversion: output code and resolution, Binary to Gray code and Gray code to binary, Equivalence of two Boolean expressions, Evaluate a Boolean expression, Minimal sum of products from minterms (with don't-cares), Number base conversion (binary, octal, decimal, hexadecimal), Truth table and minimal sum of products of a Boolean expression, Two's complement representation of a signed integer, Value of a two's complement bit pattern |
| Diodes and transistors | DC operating point of a voltage-divider biased BJT, MOSFET drain current in saturation (square law), Operating point of a diode in series with a resistor (Shockley equation), Zener diode shunt regulator |
| Electromagnetic induction | Induced emf from a change of flux (Faraday's law), Inductance of a coil on a uniform core, Mutual inductance, induced emfs and coupled coils in series, Self-inductance from flux linkage |
| Electromagnetics | Plane wave in a conducting (lossy) medium: attenuation, skin depth, speed and wave impedance, Plane wave in a lossless medium: speed, wavelength and wave impedance |
| Magnetic circuits | Series magnetic circuit: reluctance, magnetomotive force, flux density and field strength |
| Measuring instruments | Ammeter shunt resistance, Voltmeter multiplier resistance |
| Operational amplifiers | Difference amplifier output (four-resistor op-amp circuit), Inverting amplifier gain and output, Inverting summing amplifier output, Non-inverting amplifier gain and output, Op-amp integrator output for a constant input |
| Single-phase a.c. circuits | Coil with resistance in parallel with a capacitor: currents, resonance and dynamic resistance, Parallel R-L, R-C or R-L-C circuit on an a.c. supply, Series R-L, R-C or R-L-C circuit on an a.c. supply |
| Three-phase systems | Three-phase power by the two-wattmeter method |
| Transients | First-order (RC or RL) transient response, Step response of a series RLC circuit, Time for a first-order response to reach a value |

## 8. Other ways to use it

The same engine runs on the command line (`electrical-engineering`, its own guide), in Python (`pip install electrical-engineering-ai999x`, `import electrical_engineering_ai999x`, its own guide) and in the browser on https://www.ai999x.com/models/electrical-engineering-ai999x/.

Licence: Apache-2.0 · AI999X · https://www.ai999x.com/models/electrical-engineering-ai999x/
