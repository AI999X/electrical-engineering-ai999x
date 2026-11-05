//! Electrical Engineering's own live figures (ax_engine::figkit): every capability's published figure kinds, drawn for
//! the visitor's inputs from what the model computed (circuits labelled with the solved values, Bode diagrams, logic
//! maps, ...). Written by hand for the Rust build (not generated). A figure the inputs make impossible is left out.

use crate::figures_charts as ch;
use ax_engine::figkit::{self as k, eq, q, Canvas, Solved, P};
use ax_engine::figures::Figure;

/// The figures for one solved request (the dispatcher ax_engine calls: Pack::figures).
pub fn live(s: &Solved) -> Vec<Figure> {
    let v: Vec<Option<Figure>> = match s.key {
        // circuits
        "bjt_divider_bias" => vec![bjt(s), ch::bjt_family(s)],
        "voltage_divider" => vec![voltage_divider(s)],
        "current_divider" => vec![current_divider(s)],
        "zener_regulator" => vec![zener(s)],
        "diode_circuit" => vec![diode_circuit(s)],
        "max_power_transfer" => vec![max_power(s)],
        "thevenin_equivalent" => vec![thevenin(s)],
        "norton_equivalent" => vec![norton(s)],
        "series_resistance" => vec![chain(s, "R", 'R', false)],
        "series_inductance" => vec![chain(s, "L", 'L', false)],
        "series_capacitance" => vec![chain(s, "C", 'C', false)],
        "parallel_resistance" => vec![chain(s, "R", 'R', true)],
        "parallel_inductance" => vec![chain(s, "L", 'L', true)],
        "parallel_capacitance" => vec![chain(s, "C", 'C', true)],
        "series_rlc_impedance" => vec![rlc(s, false, true)],
        "series_rlc_resonance" => vec![rlc(s, false, true)],
        "series_rlc_step" => vec![rlc(s, false, false), ch::rlc_step_response(s), ch::rlc_step_decision(s)],
        "parallel_rlc_impedance" => vec![rlc(s, true, true)],
        "parallel_rlc_resonance" => vec![rlc(s, true, true)],
        "rc_filter" => vec![rc_filter(s)],
        "inverting_amplifier" => vec![inverting(s)],
        "noninverting_amplifier" => vec![noninverting(s)],
        "difference_amplifier" => vec![difference(s)],
        "summing_amplifier" => vec![summing(s)],
        "opamp_integrator" => vec![integrator(s)],
        "delta_to_wye" => vec![delta_wye(s, true)],
        "wye_to_delta" => vec![delta_wye(s, false)],
        "wheatstone_bridge" => vec![wheatstone(s)],
        "transformer" => vec![transformer(s)],
        "three_phase_load" => vec![three_phase(s)],
        "dc_motor" => vec![dc_motor(s)],
        "resistor_network" => vec![resistor_network(s)],
        // charts, logic and control
        _ => ch::live(s),
    };
    v.into_iter().flatten().collect()
}

const OHM: &str = "ohm";

fn v(s: &Solved, sym: &str) -> Option<f64> {
    s.inp(sym)
}

/// A source drawn on the left, + at the top: from (x, top) to (x, bottom).
fn source_left(c: &mut Canvas, x: f64, top: f64, bottom: f64, name: &str, value: &str, ac: bool) {
    c.vsource((x, top), (x, bottom), ac);
    c.tag(x - 24.0, (top + bottom) / 2.0, "end", name, value);
}

fn bjt(s: &Solved) -> Option<Figure> {
    let (vcc, r1, r2, rc, re) = (v(s, "VCC")?, v(s, "R1")?, v(s, "R2")?, v(s, "RC")?, v(s, "RE")?);
    let beta = v(s, "beta")?;
    let (vb, ve, vc, ic, vce) = (s.out("VB")?, s.out("VE")?, s.out("VC")?, s.out("IC")?, s.out("VCE")?);
    let mut c = Canvas::new(330.0);
    c.line((120.0, 50.0), (266.0, 50.0));
    c.bold(193.0, 40.0, "middle", &eq("VCC", vcc, "V"));
    c.resistor((120.0, 50.0), (120.0, 170.0));
    c.resistor((120.0, 170.0), (120.0, 290.0));
    let (b, col, emi) = c.npn((254.0, 170.0));
    c.line((120.0, 170.0), b);
    c.dot((120.0, 170.0));
    c.resistor((col.0, 50.0), col);
    c.resistor(emi, (emi.0, 290.0));
    c.line((120.0, 290.0), (emi.0, 290.0));
    c.ground((192.0, 290.0));
    c.tag(98.0, 108.0, "end", "R1", &q(r1, OHM));
    c.tag(98.0, 228.0, "end", "R2", &q(r2, OHM));
    c.tag(290.0, 84.0, "start", "RC", &q(rc, OHM));
    c.tag(290.0, 252.0, "start", "RE", &q(re, OHM));
    c.value(130.0, 190.0, "start", &eq("VB", vb, "V"));
    c.value(276.0, 126.0, "start", &eq("VC", vc, "V"));
    c.value(276.0, 222.0, "start", &eq("VE", ve, "V"));
    c.value(376.0, 156.0, "start", &eq("IC", ic, "A"));
    c.value(376.0, 174.0, "start", &eq("VCE", vce, "V"));
    c.name(376.0, 192.0, "start", &format!("β = {}", k::num(beta)));
    Some(s.figure("schematic", "Transistor bias circuit with its operating point",
                  &format!("Voltage-divider bias of an npn transistor, labelled with the operating point computed for \
                            these inputs: VB = {}, VE = {}, VC = {}, IC = {}, VCE = {}.", q(vb, "V"), q(ve, "V"),
                           q(vc, "V"), q(ic, "A"), q(vce, "V")), c.body(), c.h))
}

fn voltage_divider(s: &Solved) -> Option<Figure> {
    let (vin, r1, r2) = (v(s, "Vin")?, v(s, "R1")?, v(s, "R2")?);
    let (vout, i) = (s.out("Vout")?, s.out("I")?);
    let mut c = Canvas::new(300.0);
    source_left(&mut c, 110.0, 60.0, 250.0, "Vin", &q(vin, "V"), false);
    c.line((110.0, 60.0), (300.0, 60.0));
    c.resistor((300.0, 60.0), (300.0, 155.0));
    c.resistor((300.0, 155.0), (300.0, 250.0));
    c.line((110.0, 250.0), (420.0, 250.0));
    c.line((300.0, 155.0), (420.0, 155.0));
    c.dot((300.0, 155.0));
    c.terminal((420.0, 155.0), "");
    c.terminal((420.0, 250.0), "");
    c.ground((205.0, 250.0));
    c.arrow((180.0, 60.0), (235.0, 60.0));
    c.value(207.0, 50.0, "middle", &eq("I", i, "A"));
    c.tag(322.0, 107.0, "start", "R1", &q(r1, OHM));
    c.tag(322.0, 202.0, "start", "R2", &q(r2, OHM));
    c.bold(430.0, 198.0, "middle", &eq("Vout", vout, "V"));
    c.line((420.0, 165.0), (420.0, 180.0));
    c.line((420.0, 214.0), (420.0, 240.0));
    Some(s.figure("schematic", "Voltage divider with its solved output and current",
                  &format!("Voltage divider: Vin across R1 (top) and R2 (to ground), no load. Labelled with what was \
                            computed for these inputs: Vout = {} across R2, I = {} through both.", q(vout, "V"),
                           q(i, "A")), c.body(), c.h))
}

fn current_divider(s: &Solved) -> Option<Figure> {
    let (i, r1, r2) = (v(s, "I")?, v(s, "R1")?, v(s, "R2")?);
    let (i1, i2) = (s.out("I1")?, s.out("I2")?);
    let mut c = Canvas::new(280.0);
    c.isource((100.0, 230.0), (100.0, 70.0));
    c.tag(76.0, 150.0, "end", "I", &q(i, "A"));
    c.line((100.0, 70.0), (380.0, 70.0));
    c.line((100.0, 230.0), (380.0, 230.0));
    for (x, name, r, cur, cname) in [(240.0, "R1", r1, i1, "I1"), (380.0, "R2", r2, i2, "I2")] {
        c.resistor((x, 70.0), (x, 230.0));
        c.dot((x, 70.0));
        c.arrow((x, 76.0), (x, 104.0));
        c.tag(x + 20.0, 140.0, "start", name, &q(r, OHM));
        c.value(x + 20.0, 175.0, "start", &eq(cname, cur, "A"));
    }
    c.ground((240.0, 230.0));
    Some(s.figure("schematic", "Current divider with its solved branch currents",
                  &format!("A current source I feeding R1 in parallel with R2; the branch currents are those computed \
                            for these inputs: I1 = {}, I2 = {}.", q(i1, "A"), q(i2, "A")), c.body(), c.h))
}

fn zener(s: &Solved) -> Option<Figure> {
    let (vin, r, vz, rl) = (v(s, "Vin")?, v(s, "R")?, v(s, "Vz")?, v(s, "RL")?);
    let (vl, ir, il, iz) = (s.out("VL")?, s.out("IR")?, s.out("IL")?, s.out("IZ")?);
    let mut c = Canvas::new(290.0);
    source_left(&mut c, 80.0, 70.0, 230.0, "Vin", &q(vin, "V"), false);
    c.line((80.0, 70.0), (110.0, 70.0));
    c.resistor((110.0, 70.0), (230.0, 70.0));
    c.line((230.0, 70.0), (420.0, 70.0));
    c.tag(170.0, 44.0, "middle", "R", &q(r, OHM));
    c.diode((270.0, 230.0), (270.0, 70.0), true);
    c.dot((270.0, 70.0));
    c.tag(292.0, 140.0, "start", "Vz", &q(vz, "V"));
    c.resistor((420.0, 70.0), (420.0, 230.0));
    c.tag(440.0, 140.0, "start", "RL", &q(rl, OHM));
    c.line((80.0, 230.0), (420.0, 230.0));
    c.ground((350.0, 230.0));
    c.value(170.0, 100.0, "middle", &eq("IR", ir, "A"));
    c.value(292.0, 180.0, "start", &eq("IZ", iz, "A"));
    c.value(440.0, 180.0, "start", &eq("IL", il, "A"));
    c.bold(345.0, 60.0, "middle", &eq("VL", vl, "V"));
    Some(s.figure("schematic", "Zener regulator with its solved currents",
                  &format!("Zener shunt regulator: Vin through the series resistor R to the Zener diode in parallel with \
                            the load RL. Labelled with what was computed for these inputs: VL = {}, IR = {}, IZ = {}, \
                            IL = {}.", q(vl, "V"), q(ir, "A"), q(iz, "A"), q(il, "A")), c.body(), c.h))
}

fn diode_circuit(s: &Solved) -> Option<Figure> {
    let (vs, r) = (v(s, "Vs")?, v(s, "R")?);
    let (i, vd, vr) = (s.out("I")?, s.out("VD")?, s.out("VR")?);
    let mut c = Canvas::new(280.0);
    source_left(&mut c, 90.0, 70.0, 230.0, "Vs", &q(vs, "V"), false);
    c.line((90.0, 70.0), (160.0, 70.0));
    c.diode((160.0, 70.0), (260.0, 70.0), false);
    c.line((260.0, 70.0), (380.0, 70.0));
    c.resistor((380.0, 70.0), (380.0, 230.0));
    c.line((90.0, 230.0), (380.0, 230.0));
    c.ground((235.0, 230.0));
    c.arrow((290.0, 70.0), (340.0, 70.0));
    c.value(315.0, 58.0, "middle", &eq("I", i, "A"));
    c.value(210.0, 104.0, "middle", &eq("VD", vd, "V"));
    c.tag(402.0, 132.0, "start", "R", &q(r, OHM));
    c.value(402.0, 172.0, "start", &eq("VR", vr, "V"));
    Some(s.figure("schematic", "Diode circuit with its operating point",
                  &format!("A diode in series with R across Vs. Labelled with the operating point computed for these \
                            inputs: I = {}, diode voltage VD = {}, VR = {}.", q(i, "A"), q(vd, "V"), q(vr, "V")),
                  c.body(), c.h))
}

/// A source with its series resistance (Thévenin form) to terminals a and b at x = `xt`; returns the canvas.
fn thevenin_form(vth: f64, rth: f64, xt: f64) -> Canvas {
    let mut c = Canvas::new(290.0);
    source_left(&mut c, 80.0, 70.0, 230.0, "Vth", &q(vth, "V"), false);
    c.line((80.0, 70.0), (110.0, 70.0));
    c.resistor((110.0, 70.0), (220.0, 70.0));
    c.tag(165.0, 44.0, "middle", "Rth", &q(rth, OHM));
    c.line((220.0, 70.0), (xt, 70.0));
    c.line((80.0, 230.0), (xt, 230.0));
    c
}

fn max_power(s: &Solved) -> Option<Figure> {
    let (vth, rth) = (v(s, "Vth")?, v(s, "Rth")?);
    let (rl, pmax, eff) = (s.out("RL")?, s.out("Pmax")?, s.out("efficiency")?);
    let mut c = thevenin_form(vth, rth, 360.0);
    c.resistor((360.0, 70.0), (360.0, 230.0));
    c.tag(382.0, 132.0, "start", "RL", &q(rl, OHM));
    c.value(382.0, 172.0, "start", &eq("Pmax", pmax, "W"));
    c.value(382.0, 190.0, "start", &format!("efficiency = {}", k::num(eff)));
    Some(s.figure("schematic", "Load for maximum power transfer",
                  &format!("A source Vth with internal resistance Rth and the load computed for maximum power, RL = {}, \
                            which then takes Pmax = {} at efficiency {}.", q(rl, OHM), q(pmax, "W"), k::num(eff)),
                  c.body(), c.h))
}

fn nodes_note(s: &Solved) -> String {
    match (s.txt("node_a"), s.txt("node_b")) {
        (Some(a), Some(b)) => format!("seen between node {a} (a) and node {b} (b)"),
        _ => String::new(),
    }
}

fn thevenin(s: &Solved) -> Option<Figure> {
    let (vth, rth) = (s.out("Vth")?, s.out("Rth")?);
    let mut c = thevenin_form(vth, rth, 340.0);
    c.terminal((340.0, 70.0), "a");
    c.terminal((340.0, 230.0), "");
    c.name(340.0, 252.0, "middle", "b");
    let note = nodes_note(s);
    c.name(260.0, 274.0, "middle", &note);
    c.bold(420.0, 140.0, "middle", &eq("Vth", vth, "V"));
    c.bold(420.0, 162.0, "middle", &eq("Rth", rth, OHM));
    Some(s.figure("schematic", "Thévenin equivalent between the two nodes",
                  &format!("The Thévenin equivalent computed for this circuit {note}: a source Vth = {} in series with \
                            Rth = {}.", q(vth, "V"), q(rth, OHM)), c.body(), c.h))
}

fn norton(s: &Solved) -> Option<Figure> {
    let (in_, rn) = (s.out("In")?, s.out("Rn")?);
    let mut c = Canvas::new(290.0);
    c.isource((100.0, 230.0), (100.0, 70.0));
    c.tag(76.0, 150.0, "end", "In", &q(in_, "A"));
    c.line((100.0, 70.0), (340.0, 70.0));
    c.line((100.0, 230.0), (340.0, 230.0));
    c.resistor((230.0, 70.0), (230.0, 230.0));
    c.dot((230.0, 70.0));
    c.dot((230.0, 230.0));
    c.tag(252.0, 150.0, "start", "Rn", &q(rn, OHM));
    c.terminal((340.0, 70.0), "a");
    c.terminal((340.0, 230.0), "");
    c.name(340.0, 252.0, "middle", "b");
    let note = nodes_note(s);
    c.name(260.0, 274.0, "middle", &note);
    c.bold(420.0, 140.0, "middle", &eq("In", in_, "A"));
    c.bold(420.0, 162.0, "middle", &eq("Rn", rn, OHM));
    Some(s.figure("schematic", "Norton equivalent between the two nodes",
                  &format!("The Norton equivalent computed for this circuit {note}: a current In = {} in parallel with \
                            Rn = {}.", q(in_, "A"), q(rn, OHM)), c.body(), c.h))
}

/// "uF" -> "µF" (the unit a list of capacitances or inductances is given in).
fn unit_text(u: &str) -> String {
    match u.strip_prefix('u') {
        Some(rest) if !rest.is_empty() => format!("µ{rest}"),
        _ => u.to_string(),
    }
}

/// Elements of one kind (R, L or C) in series or in parallel, with the computed equivalent.
fn chain(s: &Solved, sym: &str, kind: char, parallel: bool) -> Option<Figure> {
    let vals = s.vec(sym)?;
    let eqv = s.out("equivalent")?;
    let n = vals.len();
    if n == 0 || n > if parallel { 6 } else { 8 } {
        return None;
    }
    let unit = if kind == 'R' { "Ω".to_string() } else { unit_text(&s.out_text("unit").unwrap_or_default()) };
    let val = |x: f64| if unit.is_empty() { k::num(x) } else { format!("{} {unit}", k::num(x)) };
    let draw = |c: &mut Canvas, a: P, b: P| match kind {
        'L' => c.inductor(a, b),
        'C' => c.capacitor(a, b),
        _ => c.resistor(a, b),
    };
    let mut c;
    if parallel {
        c = Canvas::new(290.0);
        let slot = 380.0 / n as f64;
        let x_last = 110.0 + slot * (n as f64 - 0.5);
        c.terminal((50.0, 70.0), "a");
        c.terminal((50.0, 230.0), "");
        c.name(50.0, 252.0, "middle", "b");
        c.line((54.0, 70.0), (x_last, 70.0));
        c.line((54.0, 230.0), (x_last, 230.0));
        for (i, x) in vals.iter().enumerate() {
            let xi = 110.0 + slot * (i as f64 + 0.5);
            draw(&mut c, (xi, 70.0), (xi, 230.0));
            c.dot((xi, 70.0));
            c.dot((xi, 230.0));
            c.tag(xi + 18.0, 140.0, "start", &format!("{sym}{}", i + 1), &val(*x));
        }
        c.bold(260.0, 276.0, "middle", &format!("equivalent = {}", val(eqv)));
    } else {
        c = Canvas::new(190.0);
        let slot = 420.0 / n as f64;
        c.terminal((40.0, 100.0), "a");
        c.terminal((480.0, 100.0), "b");
        for (i, x) in vals.iter().enumerate() {
            let x0 = 50.0 + slot * i as f64;
            let x1 = x0 + slot;
            draw(&mut c, (x0 - if i == 0 { 6.0 } else { 0.0 }, 100.0), (x1 - if i + 1 == n { 6.0 } else { 0.0 }, 100.0));
            c.name((x0 + x1) / 2.0, 58.0, "middle", &format!("{sym}{}", i + 1));
            c.value((x0 + x1) / 2.0, 75.0, "middle", &val(*x));
        }
        c.bold(260.0, 160.0, "middle", &format!("equivalent = {}", val(eqv)));
    }
    let how = if parallel { "parallel" } else { "series" };
    Some(s.figure("schematic", &format!("{n} elements in {how} and their equivalent"),
                  &format!("The {n} elements as given, connected in {how} between terminals a and b, labelled with the \
                            equivalent computed for them: {}.", val(eqv)), c.body(), c.h))
}

/// A series or parallel RLC circuit with its source and the computed values listed under it; `ac`: an AC source (a
/// step response has a DC source switched on).
fn rlc(s: &Solved, parallel: bool, ac: bool) -> Option<Figure> {
    let (r, l, cap) = (v(s, "R")?, v(s, "L")?, v(s, "C")?);
    let mut c = Canvas::new(330.0);
    let src = match (v(s, "f"), v(s, "V")) {
        (Some(f), _) => ("f", q(f, "Hz")),
        (None, Some(vv)) => ("V", q(vv, "V")),
        _ => ("", String::new()),
    };
    c.vsource((100.0, 80.0), (100.0, 230.0), ac);
    c.tag(76.0, 155.0, "end", src.0, &src.1);
    if parallel {
        c.line((100.0, 80.0), (420.0, 80.0));
        c.line((100.0, 230.0), (420.0, 230.0));
        for (x, name, val, kind) in [(210.0, "R", q(r, OHM), 'R'), (315.0, "L", q(l, "H"), 'L'), (420.0, "C", q(cap, "F"), 'C')] {
            match kind {
                'R' => c.resistor((x, 80.0), (x, 230.0)),
                'L' => c.inductor((x, 80.0), (x, 230.0)),
                _ => c.capacitor((x, 80.0), (x, 230.0)),
            }
            if x < 420.0 {
                c.dot((x, 80.0));
                c.dot((x, 230.0));
            }
            c.tag(x + 20.0, 150.0, "start", name, &val);
        }
    } else {
        c.line((100.0, 80.0), (130.0, 80.0));
        c.resistor((130.0, 80.0), (230.0, 80.0));
        c.line((230.0, 80.0), (250.0, 80.0));
        c.inductor((250.0, 80.0), (350.0, 80.0));
        c.line((350.0, 80.0), (420.0, 80.0));
        c.capacitor((420.0, 80.0), (420.0, 230.0));
        c.line((100.0, 230.0), (420.0, 230.0));
        c.tag(180.0, 50.0, "middle", "R", &q(r, OHM));
        c.tag(300.0, 50.0, "middle", "L", &q(l, "H"));
        c.tag(442.0, 150.0, "start", "C", &q(cap, "F"));
    }
    // the computed values, two to a line
    let shown: Vec<String> = match s.key {
        key if key.ends_with("impedance") => vec![eq("|Z|", s.out("impedance")?, OHM),
                                              format!("phase = {}°", k::num(s.out("phase_deg")?)),
                                              eq("XL", s.out("XL")?, OHM), eq("XC", s.out("XC")?, OHM)],
        key if key.ends_with("resonance") => vec![eq("f0", s.out("f0")?, "Hz"), format!("Q = {}", k::num(s.out("Q")?)),
                                              eq("bandwidth", s.out("bandwidth")?, "Hz")],
        _ => vec![eq("vC", s.out("vC")?, "V"), eq("i", s.out("i")?, "A"),
                  format!("ζ = {}", k::num(s.out("zeta")?)), s.out_text("response").unwrap_or_default()],
    };
    for (i, pair) in shown.chunks(2).enumerate() {
        c.value(260.0, 272.0 + 20.0 * i as f64, "middle", &pair.join("    "));
    }
    let what = if parallel { "parallel" } else { "series" };
    Some(s.figure("schematic", &format!("The {what} RLC circuit with its computed values"),
                  &format!("The {what} RLC circuit with R, L and C as given, labelled with what was computed for these \
                            inputs: {}.", shown.join("; ")), c.body(), c.h))
}

fn rc_filter(s: &Solved) -> Option<Figure> {
    let (r, cap) = (v(s, "R")?, v(s, "C")?);
    let lowpass = s.txt("filter").map(|t| t.to_lowercase() != "highpass").unwrap_or(true);
    let (fc, gdb, ph) = (s.out("fc")?, s.out("gain_db")?, s.out("phase_deg")?);
    let mut c = Canvas::new(300.0);
    c.vsource((100.0, 80.0), (100.0, 230.0), true);
    if let Some(f) = v(s, "f") {
        c.tag(76.0, 155.0, "end", "f", &q(f, "Hz"));
    }
    c.line((100.0, 80.0), (130.0, 80.0));
    if lowpass {
        c.resistor((130.0, 80.0), (240.0, 80.0));
        c.tag(185.0, 50.0, "middle", "R", &q(r, OHM));
        c.line((240.0, 80.0), (400.0, 80.0));
        c.capacitor((310.0, 80.0), (310.0, 230.0));
        c.tag(334.0, 150.0, "start", "C", &q(cap, "F"));
    } else {
        c.capacitor((130.0, 80.0), (240.0, 80.0));
        c.tag(185.0, 44.0, "middle", "C", &q(cap, "F"));
        c.line((240.0, 80.0), (400.0, 80.0));
        c.resistor((310.0, 80.0), (310.0, 230.0));
        c.tag(334.0, 150.0, "start", "R", &q(r, OHM));
    }
    c.dot((310.0, 80.0));
    c.dot((310.0, 230.0));
    c.line((100.0, 230.0), (400.0, 230.0));
    c.terminal((404.0, 80.0), "");
    c.terminal((404.0, 230.0), "");
    c.bold(440.0, 160.0, "middle", "Vout");
    let what = if lowpass { "low-pass" } else { "high-pass" };
    c.value(260.0, 268.0, "middle", &format!("{what}: fc = {}", q(fc, "Hz")));
    c.value(260.0, 288.0, "middle", &format!("gain = {} dB    phase = {}°", k::num(gdb), k::num(ph)));
    Some(s.figure("schematic", "The RC filter circuit with its computed values",
                  &format!("The {what} RC filter with R and C as given, labelled with what was computed for these \
                            inputs: fc = {}, gain {} dB, phase {}°.", q(fc, "Hz"), k::num(gdb), k::num(ph)),
                  c.body(), c.h))
}

/// The op-amp at its usual place: inverting input (236, 142), non-inverting (236, 178), output (340, 160).
fn op(c: &mut Canvas) -> (P, P, P) {
    c.opamp(250.0, 160.0)
}

/// The feedback path from the inverting node at x = `xn` over the top (height y) to the output, through `draw`.
fn feedback(c: &mut Canvas, xn: f64, y: f64, out: P) -> (P, P) {
    c.wire(&[(xn, 142.0), (xn, y), (xn + 20.0, y)]);
    let a = (xn + 20.0, y);
    let b = (out.0 + 30.0, y);
    let _ = b;
    (a, (370.0, y))
}

fn output(c: &mut Canvas, out: P, y_fb: f64, vout: f64) {
    c.wire(&[(370.0, y_fb), (370.0, out.1)]);
    c.line(out, (440.0, out.1));
    c.dot((370.0, out.1));
    c.terminal((444.0, out.1), "");
    c.bold(444.0, out.1 + 30.0, "middle", &eq("Vout", vout, "V"));
}

fn inverting(s: &Solved) -> Option<Figure> {
    let (rin, rf, vin) = (v(s, "Rin")?, v(s, "Rf")?, v(s, "Vin")?);
    let (gain, vout) = (s.out("gain")?, s.out("Vout")?);
    let mut c = Canvas::new(280.0);
    let (inm, inp, out) = op(&mut c);
    c.terminal((50.0, 142.0), "");
    c.tag(50.0, 118.0, "middle", "Vin", &q(vin, "V"));
    c.resistor((54.0, 142.0), (180.0, 142.0));
    c.tag(117.0, 170.0, "middle", "Rin", &q(rin, OHM));
    c.line((180.0, 142.0), inm);
    c.dot((210.0, 142.0));
    let (a, b) = feedback(&mut c, 210.0, 80.0, out);
    c.resistor(a, b);
    c.tag(300.0, 50.0, "middle", "Rf", &q(rf, OHM));
    output(&mut c, out, 80.0, vout);
    c.wire(&[inp, (215.0, 178.0), (215.0, 200.0)]);
    c.ground((215.0, 200.0));
    c.value(300.0, 250.0, "middle", &format!("gain = {}", k::num(gain)));
    Some(s.figure("schematic", "Inverting amplifier with its gain and output",
                  &format!("Inverting amplifier with Rin, the feedback resistor Rf and the input Vin as given, the \
                            non-inverting input grounded. Labelled with what was computed for these inputs: gain = {}, \
                            Vout = {}.", k::num(gain), q(vout, "V")), c.body(), c.h))
}

fn noninverting(s: &Solved) -> Option<Figure> {
    let (rg, rf, vin) = (v(s, "Rg")?, v(s, "Rf")?, v(s, "Vin")?);
    let (gain, vout) = (s.out("gain")?, s.out("Vout")?);
    let mut c = Canvas::new(300.0);
    let (inm, inp, out) = op(&mut c);
    // Vin enters the + input from below; Rg runs from the − node to ground on the left
    c.terminal((60.0, 236.0), "");
    c.tag(60.0, 214.0, "middle", "Vin", &q(vin, "V"));
    c.wire(&[(64.0, 236.0), (222.0, 236.0), (222.0, 178.0), inp]);
    c.line(inm, (210.0, 142.0));
    c.dot((210.0, 142.0));
    c.resistor((210.0, 142.0), (100.0, 142.0));
    c.wire(&[(100.0, 142.0), (80.0, 142.0), (80.0, 170.0)]);
    c.ground((80.0, 170.0));
    c.tag(155.0, 118.0, "middle", "Rg", &q(rg, OHM));
    let (a, b) = feedback(&mut c, 210.0, 70.0, out);
    c.resistor(a, b);
    c.tag(300.0, 40.0, "middle", "Rf", &q(rf, OHM));
    output(&mut c, out, 70.0, vout);
    c.value(300.0, 268.0, "middle", &format!("gain = {}", k::num(gain)));
    Some(s.figure("schematic", "Non-inverting amplifier with its gain and output",
                  &format!("Non-inverting amplifier: Vin on the + input, Rg from the − input to ground and the feedback \
                            resistor Rf, as given. Labelled with what was computed for these inputs: gain = {}, \
                            Vout = {}.", k::num(gain), q(vout, "V")), c.body(), c.h))
}

fn difference(s: &Solved) -> Option<Figure> {
    let (v1, v2) = (v(s, "V1")?, v(s, "V2")?);
    let (r1, r2, r3, r4) = (v(s, "R1")?, v(s, "R2")?, v(s, "R3")?, v(s, "R4")?);
    let vout = s.out("Vout")?;
    let mut c = Canvas::new(320.0);
    let (inm, inp, out) = op(&mut c);
    c.terminal((40.0, 142.0), "");
    c.value(40.0, 128.0, "middle", &eq("V1", v1, "V"));
    c.resistor((44.0, 142.0), (170.0, 142.0));
    c.tag(120.0, 117.0, "middle", "R1", &q(r1, OHM));
    c.line((170.0, 142.0), inm);
    c.dot((205.0, 142.0));
    let (a, b) = feedback(&mut c, 205.0, 80.0, out);
    c.resistor(a, b);
    c.tag(295.0, 50.0, "middle", "R2", &q(r2, OHM));
    output(&mut c, out, 80.0, vout);
    c.terminal((40.0, 210.0), "");
    c.value(40.0, 196.0, "middle", &eq("V2", v2, "V"));
    c.resistor((44.0, 210.0), (170.0, 210.0));
    c.tag(120.0, 237.0, "middle", "R3", &q(r3, OHM));
    c.wire(&[(170.0, 210.0), (222.0, 210.0), (222.0, 178.0), inp]);
    c.dot((222.0, 210.0));
    c.resistor((222.0, 210.0), (222.0, 290.0));
    c.ground((222.0, 290.0));
    c.tag(244.0, 252.0, "start", "R4", &q(r4, OHM));
    let (gi, gn) = (s.out("gain_inverting"), s.out("gain_noninverting"));
    if let (Some(gi), Some(gn)) = (gi, gn) {
        c.value(400.0, 260.0, "middle", &format!("gains {} and {}", k::num(gi), k::num(gn)));
    }
    Some(s.figure("schematic", "Difference amplifier with its output",
                  &format!("Difference amplifier: V1 through R1 to the − input with feedback R2; V2 through R3 to the + \
                            input with R4 to ground, as given. Labelled with the output computed for these inputs: \
                            Vout = {}.", q(vout, "V")), c.body(), c.h))
}

fn summing(s: &Solved) -> Option<Figure> {
    let vs = s.vec("V")?;
    let rs = s.vec("R")?;
    let rf = v(s, "Rf")?;
    let vout = s.out("Vout")?;
    let gains = s.out_list("gains").unwrap_or_default();
    let n = vs.len();
    if n == 0 || n > 4 || rs.len() != n {
        return None;
    }
    let ys: Vec<f64> = (0..n).map(|i| 112.0 + 62.0 * i as f64).collect();
    let h = (ys[n - 1] + 70.0).max(290.0);
    let mut c = Canvas::new(h);
    let (inm, inp, out) = op(&mut c);
    for (i, y) in ys.iter().enumerate() {
        c.terminal((40.0, *y), "");
        c.value(40.0, y - 12.0, "start", &eq(&format!("V{}", i + 1), vs[i], "V"));
        c.resistor((44.0, *y), (170.0, *y));
        c.name(150.0, y + 26.0, "middle", &format!("R{} = {}", i + 1, q(rs[i], OHM)));
        c.line((170.0, *y), (205.0, *y));
        c.dot((205.0, *y));
    }
    let bottom = ys[n - 1].max(142.0);
    c.line((205.0, 60.0), (205.0, bottom));
    c.line((205.0, 142.0), inm);
    c.wire(&[inp, (226.0, 178.0), (226.0, 200.0)]);
    c.ground((226.0, 200.0));
    c.line((205.0, 60.0), (225.0, 60.0));
    c.resistor((225.0, 60.0), (370.0, 60.0));
    c.tag(298.0, 32.0, "middle", "Rf", &q(rf, OHM));
    output(&mut c, out, 60.0, vout);
    if !gains.is_empty() {
        let g: Vec<String> = gains.iter().map(|x| k::num(*x)).collect();
        c.value(400.0, h - 22.0, "middle", &format!("gains: {}", g.join(", ")));
    }
    Some(s.figure("schematic", "Summing amplifier with each input's gain and the output",
                  &format!("Inverting summing amplifier: each input Vi through its resistor Ri to the summing node, \
                            feedback Rf, as given. Labelled with what was computed for these inputs: the gain of each \
                            input and Vout = {}.", q(vout, "V")), c.body(), c.h))
}

fn integrator(s: &Solved) -> Option<Figure> {
    let (r, cap, vin, t) = (v(s, "R")?, v(s, "C")?, v(s, "Vin")?, v(s, "t")?);
    let vout = s.out("Vout")?;
    let mut c = Canvas::new(230.0);
    let (inm, inp, out) = op(&mut c);
    c.terminal((50.0, 142.0), "");
    c.tag(50.0, 118.0, "middle", "Vin", &q(vin, "V"));
    c.resistor((54.0, 142.0), (180.0, 142.0));
    c.tag(117.0, 170.0, "middle", "R", &q(r, OHM));
    c.line((180.0, 142.0), inm);
    c.dot((210.0, 142.0));
    let (a, b) = feedback(&mut c, 210.0, 80.0, out);
    c.capacitor(a, b);
    c.tag(300.0, 40.0, "middle", "C", &q(cap, "F"));
    c.wire(&[(370.0, 80.0), (370.0, out.1)]);
    c.line(out, (440.0, out.1));
    c.dot((370.0, out.1));
    c.terminal((444.0, out.1), "Vout");
    c.wire(&[inp, (215.0, 178.0), (215.0, 196.0)]);
    c.ground((215.0, 196.0));
    // below: the output over time, each instant computed by the model
    if !(t > 0.0) {
        return None;
    }
    let ts = k::span(0.0, 2.0 * t, 21, false);
    let pts = s.curve("t", &ts, "Vout", &[]);
    if pts.len() < 2 {
        return None;
    }
    let chart = k::lines(&[("Vout".into(), pts)], "Time t (s)", "Vout (V)", "", Some((t, vout)),
                         &eq("Vout", vout, "V"), false);
    let body = format!("{}<g transform=\"translate(0,230)\">{}</g>", c.body(), chart);
    Some(s.figure("schematic", "Op-amp integrator and its output ramp",
                  &format!("Op-amp integrator with R, C and the constant input Vin as given and, below, Vout computed \
                            at 21 instants from 0 to {} (marked: t = {}, Vout = {}).", q(2.0 * t, "s"), q(t, "s"),
                           q(vout, "V")), &body, 570.0))
}

/// The corners of a triangle a (top), b (bottom left), c (bottom right) centred at x.
fn tri(x: f64) -> (P, P, P) {
    ((x, 60.0), (x - 90.0, 220.0), (x + 90.0, 220.0))
}

/// A delta network (resistors on the sides) and a wye network (resistors to a star point), side by side.
fn delta_wye(s: &Solved, delta_given: bool) -> Option<Figure> {
    let (rab, rbc, rca, ra, rb, rc) = if delta_given {
        (v(s, "Rab")?, v(s, "Rbc")?, v(s, "Rca")?, s.out("Ra")?, s.out("Rb")?, s.out("Rc")?)
    } else {
        (s.out("Rab")?, s.out("Rbc")?, s.out("Rca")?, v(s, "Ra")?, v(s, "Rb")?, v(s, "Rc")?)
    };
    let mut c = Canvas::new(270.0);
    let (a, b, cc) = tri(130.0);
    c.resistor(a, b);
    c.resistor(b, cc);
    c.resistor(cc, a);
    for (p, n) in [(a, "a"), (b, "b"), (cc, "c")] {
        c.dot(p);
        c.name(p.0, if n == "a" { p.1 - 10.0 } else { p.1 + 22.0 }, "middle", n);
    }
    c.tag(66.0, 130.0, "end", "Rab", &q(rab, OHM));
    c.tag(194.0, 130.0, "start", "Rca", &q(rca, OHM));
    c.tag(130.0, 248.0, "middle", "Rbc", &q(rbc, OHM));
    let (a2, b2, c2) = tri(390.0);
    let o = (390.0, 160.0);
    c.resistor(a2, o);
    c.resistor(b2, o);
    c.resistor(c2, o);
    c.dot(o);
    for (p, n) in [(a2, "a"), (b2, "b"), (c2, "c")] {
        c.dot(p);
        c.name(p.0, if n == "a" { p.1 - 10.0 } else { p.1 + 22.0 }, "middle", n);
    }
    c.tag(402.0, 106.0, "start", "Ra", &q(ra, OHM));
    c.tag(330.0, 172.0, "end", "Rb", &q(rb, OHM));
    c.tag(450.0, 172.0, "start", "Rc", &q(rc, OHM));
    c.bold(260.0, 150.0, "middle", "=");
    c.bold(40.0, 30.0, "start", "Delta");
    c.bold(470.0, 30.0, "end", "Wye");
    let (title, cap) = if delta_given {
        ("Delta to wye with the returned resistances",
         format!("The delta network as given (Rab, Rbc, Rca) and the equivalent wye computed for it: Ra = {}, Rb = {}, \
                  Rc = {}, seen between the same terminals a, b, c.", q(ra, OHM), q(rb, OHM), q(rc, OHM)))
    } else {
        ("Wye to delta with the returned resistances",
         format!("The wye network as given (Ra, Rb, Rc) and the equivalent delta computed for it: Rab = {}, \
                  Rbc = {}, Rca = {}, seen between the same terminals a, b, c.", q(rab, OHM), q(rbc, OHM), q(rca, OHM)))
    };
    Some(s.figure("schematic", title, &cap, c.body(), c.h))
}

fn wheatstone(s: &Solved) -> Option<Figure> {
    let (vs, r1, r2, r3, r4) = (v(s, "Vs")?, v(s, "R1")?, v(s, "R2")?, v(s, "R3")?, v(s, "R4")?);
    let vout = s.out("Vout")?;
    let balanced = s.out_text("balanced").unwrap_or_default();
    let r4b = s.out("R4_balance");
    let mut c = Canvas::new(320.0);
    let (t, l, r, b) = ((280.0, 60.0), (180.0, 160.0), (380.0, 160.0), (280.0, 260.0));
    source_left(&mut c, 70.0, 60.0, 260.0, "Vs", &q(vs, "V"), false);
    c.line((70.0, 60.0), t);
    c.line((70.0, 260.0), b);
    c.resistor(t, l);
    c.resistor(l, b);
    c.resistor(t, r);
    c.resistor(r, b);
    for p in [t, l, r, b] {
        c.dot(p);
    }
    c.tag(214.0, 96.0, "end", "R1", &q(r1, OHM));
    c.tag(214.0, 236.0, "end", "R2", &q(r2, OHM));
    c.tag(346.0, 96.0, "start", "R3", &q(r3, OHM));
    c.tag(346.0, 236.0, "start", "R4", &q(r4, OHM));
    c.dashed(l, r);
    c.bold(280.0, 152.0, "middle", &eq("Vout", vout, "V"));
    let mut tail = format!("balanced: {balanced}");
    if let Some(x) = r4b {
        tail.push_str(&format!("; balance at R4 = {}", q(x, OHM)));
    }
    c.value(280.0, 300.0, "middle", &tail);
    Some(s.figure("schematic", "Wheatstone bridge with its solved output",
                  &format!("Wheatstone bridge: Vs across R1, R2 (left arm, top to bottom) and R3, R4 (right arm), as \
                            given. Labelled with what was computed for these inputs: the voltage between the arm \
                            midpoints Vout = {}, {tail}.", q(vout, "V")), c.body(), c.h))
}

fn transformer(s: &Solved) -> Option<Figure> {
    let (np, ns, vp, rl) = (v(s, "Np")?, v(s, "Ns")?, v(s, "Vp")?, v(s, "RL")?);
    let (vsec, is, ip, rin, ratio) = (s.out("Vs")?, s.out("Is")?, s.out("Ip")?, s.out("Rin")?, s.out("turns_ratio")?);
    let mut c = Canvas::new(300.0);
    source_left(&mut c, 70.0, 80.0, 220.0, "Vp", &q(vp, "V"), true);
    c.line((70.0, 80.0), (210.0, 80.0));
    c.line((70.0, 220.0), (210.0, 220.0));
    c.inductor((210.0, 220.0), (210.0, 80.0));
    c.inductor((300.0, 80.0), (300.0, 220.0));
    c.line((251.0, 90.0), (251.0, 210.0));
    c.line((259.0, 90.0), (259.0, 210.0));
    c.line((300.0, 80.0), (420.0, 80.0));
    c.line((300.0, 220.0), (420.0, 220.0));
    c.resistor((420.0, 80.0), (420.0, 220.0));
    c.name(190.0, 150.0, "end", &format!("Np = {}", k::num(np)));
    c.name(320.0, 150.0, "start", &format!("Ns = {}", k::num(ns)));
    c.tag(440.0, 140.0, "start", "RL", &q(rl, OHM));
    c.arrow((120.0, 80.0), (165.0, 80.0));
    c.value(142.0, 70.0, "middle", &eq("Ip", ip, "A"));
    c.arrow((340.0, 80.0), (385.0, 80.0));
    c.value(362.0, 70.0, "middle", &eq("Is", is, "A"));
    c.bold(362.0, 244.0, "middle", &eq("Vs", vsec, "V"));
    c.value(260.0, 272.0, "middle", &format!("turns ratio {}    seen from the primary {}", k::num(ratio), q(rin, OHM)));
    Some(s.figure("schematic", "Transformer with load: solved voltages and currents",
                  &format!("Ideal transformer with Np, Ns, the primary voltage Vp and the load RL as given. Labelled with \
                            what was computed for these inputs: Vs = {}, Is = {}, Ip = {}, turns ratio {}, resistance \
                            seen from the primary {}.", q(vsec, "V"), q(is, "A"), q(ip, "A"), k::num(ratio),
                           q(rin, OHM)), c.body(), c.h))
}

fn three_phase(s: &Solved) -> Option<Figure> {
    let (vl, r, x) = (v(s, "VL")?, v(s, "R")?, v(s, "X")?);
    let wye = s.txt("connection").map(|t| t.to_lowercase() != "delta").unwrap_or(true);
    let (vph, iph, il) = (s.out("phase_voltage")?, s.out("phase_current")?, s.out("line_current")?);
    let (p, qq, sp, pf) = (s.out("P")?, s.out("Q")?, s.out("S")?, s.out("power_factor")?);
    let mut c = Canvas::new(300.0);
    let (a, b, cc) = tri(140.0);
    if wye {
        let o = (140.0, 160.0);
        c.resistor(a, o);
        c.resistor(b, o);
        c.resistor(cc, o);
        c.dot(o);
        c.name(150.0, 150.0, "start", "N");
    } else {
        c.resistor(a, b);
        c.resistor(b, cc);
        c.resistor(cc, a);
    }
    for (pt, n) in [(a, "A"), (b, "B"), (cc, "C")] {
        c.dot(pt);
        c.bold(pt.0, if n == "A" { pt.1 - 12.0 } else { pt.1 + 24.0 }, "middle", n);
    }
    c.name(140.0, 290.0, "middle", &format!("each phase Z = {} + j{}", q(r, OHM), q(x, OHM)));
    let lines = [eq("line voltage", vl, "V"), eq("phase voltage", vph, "V"), eq("phase current", iph, "A"),
                 eq("line current", il, "A"), eq("P", p, "W"), eq("Q", qq, "var"), eq("S", sp, "VA"),
                 format!("power factor = {}", k::num(pf))];
    for (i, t) in lines.iter().enumerate() {
        c.value(300.0, 70.0 + 24.0 * i as f64, "start", t);
    }
    let how = if wye { "wye" } else { "delta" };
    Some(s.figure("schematic", &format!("Three-phase {how} load with its currents and power"),
                  &format!("Balanced three-phase {how} load with line voltage VL and each phase R + jX as given. \
                            Labelled with what was computed for these inputs: {}.", lines[1..].join("; ")),
                  c.body(), c.h))
}

fn dc_motor(s: &Solved) -> Option<Figure> {
    let (vv, r, t) = (v(s, "V")?, v(s, "R")?, v(s, "T")?);
    let (speed, cur, pin, pout, eff) = (s.out("speed")?, s.out("current")?, s.out("input_power")?,
                                        s.out("output_power")?, s.out("efficiency")?);
    let mut c = Canvas::new(270.0);
    source_left(&mut c, 70.0, 70.0, 220.0, "V", &q(vv, "V"), false);
    c.line((70.0, 70.0), (100.0, 70.0));
    c.resistor((100.0, 70.0), (200.0, 70.0));
    c.tag(150.0, 44.0, "middle", "R", &q(r, OHM));
    c.wire(&[(200.0, 70.0), (250.0, 70.0), (250.0, 117.0)]);
    c.ring((250.0, 145.0), 28.0, "M");
    c.wire(&[(250.0, 173.0), (250.0, 220.0), (70.0, 220.0)]);
    c.line((278.0, 145.0), (310.0, 145.0));
    c.arrow((215.0, 70.0), (240.0, 70.0));
    c.value(160.0, 100.0, "middle", &eq("I", cur, "A"));
    c.name(312.0, 135.0, "start", &format!("load {}", q(t, "N·m")));
    let lines = [format!("speed = {} rad/s", k::num(speed)), eq("input", pin, "W"), eq("output", pout, "W"),
                 format!("efficiency = {}", k::num(eff))];
    for (i, l) in lines.iter().enumerate() {
        c.value(312.0, 170.0 + 20.0 * i as f64, "start", l);
    }
    Some(s.figure("schematic", "DC motor circuit with its steady state",
                  &format!("Permanent-magnet DC motor with supply V, armature resistance R and the load as given. \
                            Labelled with the steady state computed for these inputs: current {}, {}.", q(cur, "A"),
                           lines.join(", ")), c.body(), c.h))
}

// ------------------------------------------------------------------------------------------------ resistor network
#[derive(Debug)]
enum Net {
    R(f64),
    Series(Vec<Net>),
    Par(Vec<Net>),
}

impl Net {
    fn value(&self) -> f64 {
        match self {
            Net::R(x) => *x,
            Net::Series(v) => v.iter().map(|n| n.value()).sum(),
            Net::Par(v) => 1.0 / v.iter().map(|n| 1.0 / n.value()).sum::<f64>(),
        }
    }
    fn count(&self) -> usize {
        match self {
            Net::R(_) => 1,
            Net::Series(v) | Net::Par(v) => v.iter().map(|n| n.count()).sum(),
        }
    }
    /// (width, height) of the drawing
    fn size(&self) -> (f64, f64) {
        match self {
            Net::R(_) => (90.0, 56.0),
            Net::Series(v) => (v.iter().map(|n| n.size().0).sum(), v.iter().map(|n| n.size().1).fold(0.0, f64::max)),
            Net::Par(v) => (v.iter().map(|n| n.size().0).fold(0.0, f64::max) + 40.0, v.iter().map(|n| n.size().1).sum()),
        }
    }
    /// draw from (x, y) (the left end of the main line) to the right; `scale` shrinks a wide network
    fn draw(&self, c: &mut Canvas, x: f64, y: f64, labels: &mut Vec<(P, String)>) {
        match self {
            Net::R(v) => {
                c.resistor((x + 10.0, y), (x + 80.0, y));
                c.line((x, y), (x + 10.0, y));
                c.line((x + 80.0, y), (x + 90.0, y));
                labels.push(((x + 45.0, y - 14.0), q(*v, OHM)));
            }
            Net::Series(v) => {
                let mut xx = x;
                for n in v {
                    n.draw(c, xx, y, labels);
                    xx += n.size().0;
                }
            }
            Net::Par(v) => {
                let (w, h) = self.size();
                let mut top = y - h / 2.0;
                let mut ys = vec![];
                for n in v {
                    let (nw, nh) = n.size();
                    let yc = top + nh / 2.0 + 6.0;
                    ys.push(yc);
                    c.line((x + 20.0, yc), (x + 20.0 + (w - 40.0 - nw) / 2.0, yc));
                    n.draw(c, x + 20.0 + (w - 40.0 - nw) / 2.0, yc, labels);
                    c.line((x + 20.0 + (w - 40.0 + nw) / 2.0, yc), (x + w - 20.0, yc));
                    top += nh;
                }
                let (y0, y1) = (ys[0], ys[ys.len() - 1]);
                c.line((x + 20.0, y0), (x + 20.0, y1));
                c.line((x + w - 20.0, y0), (x + w - 20.0, y1));
                c.line((x, y), (x + 20.0, y));
                c.line((x + w - 20.0, y), (x + w, y));
                c.dot((x + 20.0, y));
                c.dot((x + w - 20.0, y));
            }
        }
    }
}

/// The network text: numbers, "+" (series), "||" (parallel, binding tighter) and parentheses.
fn parse_net(t: &str) -> Option<Net> {
    let mut toks: Vec<String> = vec![];
    let cs: Vec<char> = t.chars().collect();
    let mut i = 0;
    while i < cs.len() {
        let ch = cs[i];
        if ch.is_whitespace() {
            i += 1;
        } else if ch == '|' && cs.get(i + 1) == Some(&'|') {
            toks.push("||".into());
            i += 2;
        } else if "+()".contains(ch) {
            toks.push(ch.to_string());
            i += 1;
        } else if ch.is_ascii_digit() || ch == '.' {
            let st = i;
            while i < cs.len() && (cs[i].is_ascii_digit() || cs[i] == '.' || cs[i] == 'e' || cs[i] == 'E'
                || ((cs[i] == '-' || cs[i] == '+') && i > st && (cs[i - 1] == 'e' || cs[i - 1] == 'E'))) {
                i += 1;
            }
            let mut num: f64 = cs[st..i].iter().collect::<String>().parse().ok()?;
            match cs.get(i) {
                Some('k') | Some('K') => { num *= 1e3; i += 1; }
                Some('M') => { num *= 1e6; i += 1; }
                _ => {}
            }
            toks.push(format!("#{num}"));
        } else {
            return None;
        }
    }
    let mut pos = 0;
    let net = expr(&toks, &mut pos, 0)?;
    (pos == toks.len()).then_some(net)
}

fn expr(t: &[String], pos: &mut usize, depth: usize) -> Option<Net> {
    if depth > 12 {
        return None;
    }
    let mut parts = vec![term(t, pos, depth)?];
    while t.get(*pos).map(|x| x == "+").unwrap_or(false) {
        *pos += 1;
        parts.push(term(t, pos, depth)?);
    }
    Some(if parts.len() == 1 { parts.pop()? } else { Net::Series(parts) })
}

fn term(t: &[String], pos: &mut usize, depth: usize) -> Option<Net> {
    let mut parts = vec![factor(t, pos, depth)?];
    while t.get(*pos).map(|x| x == "||").unwrap_or(false) {
        *pos += 1;
        parts.push(factor(t, pos, depth)?);
    }
    Some(if parts.len() == 1 { parts.pop()? } else { Net::Par(parts) })
}

fn factor(t: &[String], pos: &mut usize, depth: usize) -> Option<Net> {
    let tok = t.get(*pos)?.clone();
    *pos += 1;
    if tok == "(" {
        let n = expr(t, pos, depth + 1)?;
        (t.get(*pos)? == ")").then_some(())?;
        *pos += 1;
        Some(n)
    } else {
        let x: f64 = tok.strip_prefix('#')?.parse().ok()?;
        (x > 0.0 && x.is_finite()).then_some(Net::R(x))
    }
}

fn resistor_network(s: &Solved) -> Option<Figure> {
    let text = s.txt("network")?;
    let eqv = s.out("equivalent")?;
    let net = parse_net(&text)?;
    // drawn only when this reading of the text gives the model's own equivalent
    if (net.value() - eqv).abs() > 1e-9 * eqv.abs().max(1.0) {
        return None;
    }
    let (w, h) = net.size();
    if w > 440.0 || h > 260.0 || net.count() > 12 {
        return None;
    }
    let ch = (h + 120.0).max(220.0);
    let mut c = Canvas::new(ch);
    let y = 40.0 + h / 2.0 + 10.0;
    let x0 = (520.0 - w) / 2.0;
    c.terminal((x0 - 20.0, y), "a");
    c.line((x0 - 16.0, y), (x0, y));
    let mut labels = vec![];
    net.draw(&mut c, x0, y, &mut labels);
    c.line((x0 + w, y), (x0 + w + 16.0, y));
    c.terminal((x0 + w + 20.0, y), "b");
    for (p, t) in labels {
        c.value(p.0, p.1, "middle", &t);
    }
    c.bold(260.0, ch - 20.0, "middle", &format!("equivalent resistance = {} ({} resistors)", q(eqv, OHM), net.count()));
    Some(s.figure("schematic", "The resistor network and its equivalent resistance",
                  &format!("The resistor network as written ({text}; + series, || parallel, resistances in ohms), drawn \
                            between terminals a and b and labelled with the equivalent resistance computed for it: {}.",
                           q(eqv, OHM)), c.body(), c.h))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn networks_parse() {
        let n = parse_net("10 + (20 || 30)").unwrap();
        assert!((n.value() - 22.0).abs() < 1e-12);
        assert_eq!(n.count(), 3);
        assert!(parse_net("10 + (20 || ").is_none());
        assert!(parse_net("1k || 1k").map(|n| (n.value() - 500.0).abs() < 1e-9).unwrap());
    }
}
