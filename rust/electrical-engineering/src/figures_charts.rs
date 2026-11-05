//! Electrical Engineering's own live figures, part two: charts, logic and control (Bode diagrams, root locus, step
//! responses, Karnaugh maps, gate diagrams, bit patterns, value fields over two inputs). Each is computed by the model
//! itself for the visitor's inputs (further solves through ax_engine::figkit::Solved), never drawn from a formula
//! written here.

use ax_engine::figkit::{self as k, eq, num, q, Canvas, Solved, P};
use ax_engine::figures::Figure;
use ax_engine::{Answer, Input};
use serde_json::{json, Value};

pub fn live(s: &Solved) -> Vec<Option<Figure>> {
    match s.key {
        "adc_conversion" => vec![adc(s)],
        "base_conversion" => vec![base_bits(s)],
        "gray_code" => vec![gray_bits(s)],
        "twos_complement_decode" => vec![twos_bits(s, true)],
        "twos_complement_encode" => vec![twos_bits(s, false)],
        "bode_point" | "gain_margin" | "phase_margin" => vec![bode(s)],
        "boolean_equivalence" => vec![equivalence_truth(s)],
        "boolean_evaluate" => vec![gates(s)],
        "closed_loop_poles" => vec![locus(s)],
        "complex_power" => vec![power_triangle(s), power_decision(s)],
        "controllability_observability" => vec![ctrl_matrix(s)],
        "mesh_currents" => vec![linear_matrix(s, "currents", false), linear_matrix(s, "currents", true),
                                linear_vectors(s, "currents")],
        "nodal_voltages" => vec![linear_matrix(s, "voltages", false), linear_vectors(s, "voltages")],
        "state_space_to_tf" => vec![ss_matrix(s), ss_series(s)],
        "dc_node_voltages" => vec![node_bars(s)],
        "inductor_energy" => vec![field2(s, "I", Rng::Near, "L", Rng::Near, "energy", "Energy stored in an inductor over \
                                                                                    current and inductance")],
        "lead_compensator" => vec![field2(s, "z", Rng::Decade, "p", Rng::Decade, "phase_max_deg",
                                          "Maximum phase lead over the zero and pole frequencies")],
        "polar_to_rectangular" => vec![field2(s, "magnitude", Rng::Near, "angle_deg", Rng::Around(60.0), "real",
                                              "Real part over magnitude and angle")],
        "rectangular_to_polar" => vec![field2(s, "real", Rng::Near, "imag", Rng::Near, "magnitude",
                                              "Magnitude over the real and imaginary parts")],
        "rms_waveform" => vec![field2(s, "amplitude", Rng::Near, "offset", Rng::Half("amplitude"), "rms",
                                      "RMS value over amplitude and offset")],
        "minterms_to_sop" | "truth_table_sop" => vec![kmap(s)],
        "mosfet_saturation" => vec![mosfet(s)],
        "phasor_sum" => vec![phasors(s), phasor_sensitivity(s)],
        "routh_hurwitz" => vec![routh(s)],
        "second_order_step" => vec![second_order(s)],
        "steady_state_error" => vec![loop_block(s)],
        _ => vec![],
    }
}

/// The unit of an input as the pack names it ("" for a pure number).
fn in_unit(s: &Solved, sym: &str) -> String {
    let kind = s.p.procedure(s.key).and_then(|p| p.inputs.iter().find(|i| i.symbol == sym)).map(|i| i.kind.clone());
    match kind.and_then(|kd| s.p.kinds.get(&kd).map(|x| x.unit.clone())) {
        Some(u) if u != "1" => u,
        _ => String::new(),
    }
}

fn label(sym: &str, unit: &str) -> String {
    let u = match unit {
        "ohm" => "Ω",
        x => x,
    };
    if u.is_empty() { sym.to_string() } else { format!("{sym} ({u})") }
}

/// A number or a true/false output as 1/0.
fn truthy(a: &Answer, sym: &str) -> Option<f64> {
    match &a.values.iter().find(|x| x.0 == sym)?.1 {
        Value::Bool(b) => Some(if *b { 1.0 } else { 0.0 }),
        v => v.as_f64(),
    }
}

fn text_of(a: &Answer, sym: &str) -> Option<String> {
    match &a.values.iter().find(|x| x.0 == sym)?.1 {
        Value::String(t) => Some(t.clone()),
        Value::Bool(b) => Some(if *b { "yes".into() } else { "no".into() }),
        Value::Null => None,
        v => Some(v.to_string()),
    }
}

// ------------------------------------------------------------------------------------------------ curves
pub fn bjt_family(s: &Solved) -> Option<Figure> {
    let (vcc, re, ic) = (s.inp("VCC")?, s.inp("RE")?, s.out("IC_mA")?);
    if !(re > 0.0) {
        return None;
    }
    let xs = k::span(re * 0.5, re * 1.5, 21, false);
    let series = s.family("RE", &xs, "IC_mA", "VCC", &[vcc, vcc * 0.5, vcc * 2.0], "V");
    if series.is_empty() {
        return None;
    }
    let body = k::lines(&series, "Emitter resistor RE (Ω)", "Collector current IC (mA)", &eq("IC", ic / 1000.0, "A"),
                        Some((re, ic)), &format!("RE = {}", q(re, "ohm")), false);
    Some(s.chart("curve", "Collector current against the emitter resistor, for three supply voltages",
                 &format!("IC against RE from {} to {}, one curve for each supply voltage (the one given, half and \
                           twice it), each point computed by the model; marked: the inputs given, IC = {}.",
                          q(re * 0.5, "ohm"), q(re * 1.5, "ohm"), q(ic / 1000.0, "A")), &body))
}

fn adc(s: &Solved) -> Option<Figure> {
    let (vin, vref) = (s.inp("Vin")?, s.inp("Vref")?);
    let bits = s.val("bits")?.int();
    let level = s.out("quantized")?;
    if !(vref > 0.0) {
        return None;
    }
    let xs = k::span(0.0, vref, 81, false);
    let mut series = vec![(format!("{bits} bits"), s.curve("Vin", &xs, "quantized", &[]))];
    if bits > 5 {
        let mut pts = vec![];
        for x in &xs {
            if let Some(a) = s.with_input(&[("Vin", Input::Float(*x)), ("bits", Input::Int(3))]) {
                if let Some(y) = k::num_of(&a, "quantized") {
                    pts.push((*x, y));
                }
            }
        }
        series.push(("3 bits (steps visible)".into(), pts));
    }
    series.push(("Vin itself".into(), vec![(0.0, 0.0), (vref, vref)]));
    series.retain(|x| x.1.len() >= 2);
    let body = k::lines(&series, "Input voltage Vin (V)", "Quantized level (V)",
                        &format!("code {} = {}", s.out("code").map(num).unwrap_or_default(),
                                 s.out_text("binary").unwrap_or_default()),
                        Some((vin, level)), &eq("level", level, "V"), false);
    Some(s.chart("transfer", "Converter staircase: quantized level against input",
                 &format!("Transfer staircase of the converter: the quantized level computed for 81 input voltages from \
                           0 to Vref = {}, with the {bits}-bit resolution given{}; marked: Vin = {}, level {}.",
                          q(vref, "V"), if bits > 5 { " and with 3 bits so the steps are visible" } else { "" },
                          q(vin, "V"), q(level, "V")), &body))
}

fn mosfet(s: &Solved) -> Option<Figure> {
    let (vgs, vth) = (s.inp("VGS")?, s.inp("VTH")?);
    let id = s.out("ID_mA")?;
    let hi = (vgs * 1.5).max(vth + 1.0).max(vgs);
    let xs = k::span(vth, hi, 41, false);
    let pts = s.curve("VGS", &xs, "ID_mA", &[]);
    if pts.len() < 2 {
        return None;
    }
    let body = k::lines_y(&[("ID".into(), pts)], "Gate-source voltage VGS (V)", "Drain current ID (mA)",
                          &format!("overdrive {}", q(s.out("VOV").unwrap_or(f64::NAN), "V")), Some((vgs, id)),
                          &eq("ID", id / 1000.0, "A"), false, true);
    Some(s.chart("transfer", "MOSFET transfer characteristic in saturation",
                 &format!("Drain current in saturation against the gate-source voltage from VTH = {} to {}, each point \
                           computed by the model (points outside saturation are left out); marked: VGS = {}, ID = {}.",
                          q(vth, "V"), q(hi, "V"), q(vgs, "V"), q(id / 1000.0, "A")), &body))
}

/// The response at one frequency (the model's own bode_point): (|G| in dB, phase in degrees).
fn response(s: &Solved, nm: &[f64], dn: &[f64], w: f64) -> Option<(f64, f64)> {
    let a = s.solve_other("bode_point", &[("num", Input::Json(json!(nm))), ("den", Input::Json(json!(dn))),
                                          ("w", Input::Float(w))])?;
    Some((k::num_of(&a, "magnitude_db")?, k::num_of(&a, "phase_deg")?))
}

fn bode(s: &Solved) -> Option<Figure> {
    let (nm, dn) = (s.vec("num")?, s.vec("den")?);
    // the frequency the figure centres on and what is marked there: (ω, |G| dB, phase)
    let (wc, mark_db, mark_ph, header, title, what) = match s.key {
        "bode_point" => {
            let w = s.inp("w")?;
            let (m, p) = (s.out("magnitude_db")?, s.out("phase_deg")?);
            (w, Some(m), Some(p), format!("|G| = {} dB, phase {}° at ω = {}", num(m), num(p), q(w, "rad/s")),
             "Bode diagram with the point computed", format!("the point computed at ω = {}", q(w, "rad/s")))
        }
        "gain_margin" => {
            let w = s.out("phase_crossover")?;
            let gm = s.out("gain_margin_db")?;
            (w, Some(-gm), Some(-180.0), format!("gain margin {} dB at ω = {}", num(gm), q(w, "rad/s")),
             "Bode diagram with the gain margin",
             format!("the phase crossover ω = {} and the gain margin computed, {} dB", q(w, "rad/s"), num(gm)))
        }
        _ => {
            let w = s.out("gain_crossover")?;
            let pm = s.out("phase_margin")?;
            (w, Some(0.0), Some(-180.0 + pm), format!("phase margin {}° at ω = {}", num(pm), q(w, "rad/s")),
             "Bode diagram with the phase margin",
             format!("the gain crossover ω = {} and the phase margin computed, {}°", q(w, "rad/s"), num(pm)))
        }
    };
    if !(wc > 0.0) {
        return None;
    }
    let ws = k::span(wc / 100.0, wc * 100.0, 61, true);
    let mut mag = vec![];
    let mut ph = vec![];
    for w in ws {
        if let Some((m, p)) = response(s, &nm, &dn, w) {
            mag.push((w, m));
            ph.push((w, p));
        }
    }
    if mag.len() < 2 {
        return None;
    }
    // the phase unwrapped (no jumps of 360°), then placed so that it passes through the marked phase
    for i in 1..ph.len() {
        while ph[i].1 - ph[i - 1].1 > 180.0 {
            ph[i].1 -= 360.0;
        }
        while ph[i].1 - ph[i - 1].1 < -180.0 {
            ph[i].1 += 360.0;
        }
    }
    if let Some(mp) = mark_ph {
        let near = ph.iter().min_by(|a, b| (a.0.ln() - wc.ln()).abs().partial_cmp(&(b.0.ln() - wc.ln()).abs())
            .unwrap_or(std::cmp::Ordering::Equal))?.1;
        let shift = ((mp - near) / 360.0).round() * 360.0;
        for p in ph.iter_mut() {
            p.1 += shift;
        }
    }
    let top = k::lines(&[("|G|".into(), mag)], "", "Magnitude (dB)", &header, mark_db.map(|m| (wc, m)),
                       &mark_db.map(|m| format!("{} dB", num(m))).unwrap_or_default(), true);
    let bottom = k::lines(&[("phase".into(), ph)], "Angular frequency ω (rad/s)", "Phase (°)", "",
                          mark_ph.map(|p| (wc, p)), &mark_ph.map(|p| format!("{}°", num(p))).unwrap_or_default(), true);
    let (body, h) = k::stack(&[top, bottom]);
    Some(s.figure("bode", title,
                  &format!("Bode magnitude (top) and phase (bottom, unwrapped) of G(s) = ({}) / ({}) at 61 frequencies \
                            from {} to {}, each computed by the model's frequency response. Marked: {what}.",
                           poly(&nm), poly(&dn), q(wc / 100.0, "rad/s"), q(wc * 100.0, "rad/s")), &body, h))
}

/// A polynomial from its coefficients, highest power first: [1, 3, 2] -> "s² + 3s + 2".
pub fn poly(c: &[f64]) -> String {
    const SUP: [&str; 10] = ["⁰", "¹", "²", "³", "⁴", "⁵", "⁶", "⁷", "⁸", "⁹"];
    let n = c.len();
    let mut out = String::new();
    for (i, x) in c.iter().enumerate() {
        let p = n - 1 - i;
        if *x == 0.0 {
            continue;
        }
        let sign = if out.is_empty() { if *x < 0.0 { "-" } else { "" } } else if *x < 0.0 { " - " } else { " + " };
        let a = x.abs();
        let coef = if a == 1.0 && p > 0 { String::new() } else { num(a) };
        let pw = match p {
            0 => String::new(),
            1 => "s".into(),
            _ => format!("s{}", p.to_string().chars().map(|d| SUP[d as usize - '0' as usize]).collect::<String>()),
        };
        out.push_str(&format!("{sign}{coef}{pw}"));
    }
    if out.is_empty() { "0".into() } else { out }
}

fn locus(s: &Solved) -> Option<Figure> {
    let k0 = s.inp("K").unwrap_or(1.0);
    if !(k0 > 0.0) {
        return None;
    }
    let mine_poles = k::cx_list(s.a, "poles")?;
    let mut dots = vec![];
    for kk in k::span(k0 / 100.0, k0 * 100.0, 41, true) {
        if let Some(a) = s.with(&[("K", kk)]) {
            if let Some(p) = k::cx_list(&a, "poles") {
                dots.extend(p);
            }
        }
    }
    let stable = s.out_text("stable").unwrap_or_default();
    let mine: Vec<(String, P)> = mine_poles.iter().map(|z| (cx(*z), *z)).collect();
    let body = k::plane(&dots, &mine, "Real part", "Imaginary part",
                        &format!("poles at K = {} (stable: {stable})", num(k0)));
    Some(s.chart("locus", "Root locus: closed-loop poles as the gain grows",
                 &format!("Closed-loop poles of K·G(s) under unity feedback for 41 gains K from {} to {} (logarithmic \
                           steps), each set computed by the model. Marked: the poles at K = {}: {}.", num(k0 / 100.0),
                          num(k0 * 100.0), num(k0), mine.iter().map(|m| m.0.clone()).collect::<Vec<_>>().join(", ")),
                 &body))
}

fn cx((re, im): P) -> String {
    if im == 0.0 { num(re) } else if im > 0.0 { format!("{} + {}j", num(re), num(im)) } else {
        format!("{} - {}j", num(re), num(-im))
    }
}

fn power_triangle(s: &Solved) -> Option<Figure> {
    let (p, qq, sp, pf) = (s.out("P")?, s.out("Q")?, s.out("S")?, s.out("power_factor")?);
    let nature = s.out_text("nature").unwrap_or_default();
    if p == 0.0 && qq == 0.0 {
        return None;
    }
    let sc = (300.0 / p.abs().max(1e-300)).min(180.0 / qq.abs().max(1e-300));
    let o: P = (if p >= 0.0 { 110.0 } else { 410.0 }, if qq >= 0.0 { 250.0 } else { 70.0 });
    let pe = (o.0 + p * sc, o.1);
    let tip = (pe.0, o.1 - qq * sc);
    let mut c = Canvas::new(300.0);
    c.arrow(o, pe);
    if qq != 0.0 {
        c.arrow(pe, tip);
    }
    c.arrow(o, tip);
    c.value((o.0 + pe.0) / 2.0, o.1 + if qq >= 0.0 { 22.0 } else { -10.0 }, "middle", &eq("P", p, "W"));
    c.value(pe.0 + if p >= 0.0 { 10.0 } else { -10.0 }, (pe.1 + tip.1) / 2.0, if p >= 0.0 { "start" } else { "end" },
            &format!("{} ({nature})", eq("Q", qq, "var")));
    c.bold((o.0 + tip.0) / 2.0 - if p >= 0.0 { 14.0 } else { -14.0 }, (o.1 + tip.1) / 2.0 - 10.0,
           if p >= 0.0 { "end" } else { "start" }, &eq("S", sp, "VA"));
    c.name(260.0, 290.0, "middle", &format!("power factor {}  (angle {}°)", num(pf), num(qq.atan2(p).to_degrees())));
    Some(s.figure("triangle", "Power triangle: real, reactive and apparent power",
                  &format!("Power triangle of the load from the complex power computed for these inputs: real power \
                            P = {}, reactive power Q = {} ({nature}), apparent power S = {}, power factor {}.",
                           q(p, "W"), q(qq, "var"), q(sp, "VA"), num(pf)), c.body(), c.h))
}

fn power_decision(s: &Solved) -> Option<Figure> {
    let (vp, ip) = (s.inp("V_phase")?, s.inp("I_phase")?);
    let xs = k::span(-90.0, 90.0, 13, false);
    let ys = k::span(-90.0, 90.0, 13, false);
    let mut cells = vec![];
    let mut names: Vec<String> = vec![];
    for (i, x) in xs.iter().enumerate() {
        for (j, y) in ys.iter().enumerate() {
            if let Some(a) = s.with(&[("I_phase", *x), ("V_phase", *y)]) {
                if let Some(t) = text_of(&a, "nature") {
                    if !names.contains(&t) {
                        names.push(t.clone());
                    }
                    cells.push((i, j, t));
                }
            }
        }
    }
    if cells.is_empty() {
        return None;
    }
    let body = k::decision(&xs, &ys, &cells, &names, (ip, vp), "Current phase angle (°)", "Voltage phase angle (°)",
                           &format!("{} here", s.out_text("nature").unwrap_or_default()));
    Some(s.chart("decision", "Where the load turns from lagging to leading",
                 &format!("The nature of the load (lagging, leading or unity) over the current and voltage phase angles \
                           from -90° to 90°: each of the {} cells is one computation of the model. Marked: the angles \
                           given.", cells.len()), &body))
}

// ------------------------------------------------------------------------------------------------ fields
enum Rng {
    /// half to one and a half times the value (-1 to 1 for zero)
    Near,
    /// a tenth to ten times (logarithmic)
    Decade,
    /// the value plus or minus this much
    Around(f64),
    /// the value plus or minus half of another input (at least 1)
    Half(&'static str),
}

fn range_of(s: &Solved, x0: f64, r: &Rng) -> (f64, f64, bool) {
    match r {
        Rng::Near if x0 != 0.0 => {
            let (a, b) = (x0 * 0.5, x0 * 1.5);
            (a.min(b), a.max(b), false)
        }
        Rng::Near => (-1.0, 1.0, false),
        Rng::Decade if x0 > 0.0 => (x0 / 10.0, x0 * 10.0, true),
        Rng::Decade => (-1.0, 1.0, false),
        Rng::Around(d) => (x0 - d, x0 + d, false),
        Rng::Half(other) => {
            let d = (s.inp(other).unwrap_or(1.0).abs() / 2.0).max(1.0);
            (x0 - d, x0 + d, false)
        }
    }
}

fn field2(s: &Solved, xs_: &str, xr: Rng, ys_: &str, yr: Rng, out: &str, title: &str) -> Option<Figure> {
    let (x0, y0) = (s.inp(xs_)?, s.inp(ys_)?);
    let z0 = s.out(out)?;
    let (xl, xh, xlog) = range_of(s, x0, &xr);
    let (yl, yh, ylog) = range_of(s, y0, &yr);
    let xs = k::span(xl, xh, 13, xlog);
    let ys = k::span(yl, yh, 11, ylog);
    let mut vals = vec![];
    let mut n = 0;
    for x in &xs {
        let mut col = vec![];
        for y in &ys {
            let v = s.with(&[(xs_, *x), (ys_, *y)]).and_then(|a| k::num_of(&a, out));
            n += v.is_some() as usize;
            col.push(v);
        }
        vals.push(col);
    }
    if n < 4 {
        return None;
    }
    let meaning = s.p.procedure(s.key).and_then(|p| p.outputs.iter().find(|o| o.symbol == out))
        .map(|o| o.meaning.clone()).unwrap_or_else(|| out.to_string());
    let (ux, uy) = (in_unit(s, xs_), in_unit(s, ys_));
    let body = k::field(&xs, &ys, &vals, (x0, y0), &label(xs_, &ux), &label(ys_, &uy), &format!("{out} = {}", num(z0)),
                        (xlog, ylog));
    Some(s.chart("field", title,
                 &format!("{meaning} ({out}) over {xs_} from {} to {} and {ys_} from {} to {}{}: each of the {} cells is \
                           one computation of the model, shaded by its value (darker is larger; a blank cell: inputs \
                           the model refuses). Marked: the inputs given, {out} = {}.", num(xl), num(xh), num(yl),
                          num(yh), if xlog || ylog { " (logarithmic axes)" } else { "" }, n, num(z0)), &body))
}

// ------------------------------------------------------------------------------------------------ matrices
fn labels(n: usize) -> Vec<String> {
    (1..=n).map(|i| i.to_string()).collect()
}

fn linear_matrix(s: &Solved, out: &str, augmented: bool) -> Option<Figure> {
    let a = s.mat("A")?;
    let b = s.vec("b")?;
    let x = s.out_list(out)?;
    let n = a.len();
    if n == 0 || n > 10 || b.len() != n || x.len() != n || a.iter().any(|r| r.len() != n) {
        return None;
    }
    let (m, cols, kind, title) = if augmented || s.key == "nodal_voltages" {
        let m: Vec<Vec<f64>> = (0..n).map(|i| a[i].iter().copied().chain([f64::NAN, b[i], f64::NAN, x[i]]).collect())
            .collect();
        let mut cols = labels(n);
        cols.extend(["".into(), "b".into(), "".into(), if out == "currents" { "I".into() } else { "V".into() }]);
        (m, cols, if augmented { "heatmap" } else { "matrix" }, "The equations as matrices: A x = b and the solution")
    } else {
        (a.clone(), labels(n), "matrix", "The coefficient matrix of the equations")
    };
    let xs: Vec<String> = x.iter().map(|v| num(*v)).collect();
    let body = k::heat(&m, &labels(n), &cols, &format!("{out} = {}", xs.join(", ")));
    Some(s.chart(kind, title,
                 &format!("The coefficient matrix A as given{}, each cell shaded by its magnitude (negative values in \
                           the second colour); the solution computed: {out} = {}.",
                          if m[0].len() > n { ", the right-hand side b and the solution" } else { "" }, xs.join(", ")),
                 &body))
}

fn linear_vectors(s: &Solved, out: &str) -> Option<Figure> {
    let b = s.vec("b")?;
    let x = s.out_list(out)?;
    if b.len() != 2 || x.len() != 2 {
        return None;
    }
    let v = vec![("b".to_string(), (b[0], b[1]), false), (out.to_string(), (x[0], x[1]), true)];
    let body = k::arrows(&v, &format!("{out} = ({}, {})", num(x[0]), num(x[1])));
    Some(s.chart("vectors", "The right-hand side and the solution as plane vectors",
                 &format!("The right-hand side b = ({}, {}) as given and the solution computed, {out} = ({}, {}), as \
                           arrows from the origin.", num(b[0]), num(b[1]), num(x[0]), num(x[1])), &body))
}

fn ctrl_matrix(s: &Solved) -> Option<Figure> {
    let a = s.mat("A")?;
    let b = s.vec("B")?;
    let c = s.vec("C")?;
    let n = a.len();
    if n == 0 || n > 10 || b.len() != n || c.len() != n || a.iter().any(|r| r.len() != n) {
        return None;
    }
    let mut m: Vec<Vec<f64>> = (0..n).map(|i| a[i].iter().copied().chain([f64::NAN, b[i]]).collect()).collect();
    m.push(vec![f64::NAN; n + 2]);
    m.push(c.iter().copied().chain([f64::NAN, f64::NAN]).collect());
    let mut rows = labels(n);
    rows.extend(["".into(), "C".into()]);
    let mut cols = labels(n);
    cols.extend(["".into(), "B".into()]);
    let (cr, or) = (s.out("controllability_rank")?, s.out("observability_rank")?);
    let body = k::heat(&m, &rows, &cols, &format!("ranks: controllability {}, observability {}", num(cr), num(or)));
    Some(s.chart("matrix", "The system matrices A, B and C",
                 &format!("The state matrix A with the input vector B beside it and the output vector C below, as \
                           given, each cell shaded by its magnitude; computed: controllability rank {}, observability \
                           rank {} (order {n}).", num(cr), num(or)), &body))
}

fn ss_matrix(s: &Solved) -> Option<Figure> {
    let a = s.mat("A")?;
    let b = s.vec("B")?;
    let c = s.vec("C")?;
    let d = s.inp("D").unwrap_or(0.0);
    let n = a.len();
    if n == 0 || n > 10 || b.len() != n || c.len() != n || a.iter().any(|r| r.len() != n) {
        return None;
    }
    let mut m: Vec<Vec<f64>> = (0..n).map(|i| a[i].iter().copied().chain([f64::NAN, b[i]]).collect()).collect();
    m.push(vec![f64::NAN; n + 2]);
    m.push(c.iter().copied().chain([f64::NAN, d]).collect());
    let mut rows = labels(n);
    rows.extend(["".into(), "C".into()]);
    let mut cols = labels(n);
    cols.extend(["".into(), "B, D".into()]);
    let (nm, dn) = (s.out_list("num")?, s.out_list("den")?);
    let body = k::heat(&m, &rows, &cols, &format!("G(s) = ({}) / ({})", poly(&nm), poly(&dn)));
    Some(s.chart("matrix", "The state-space model as matrices",
                 &format!("A with B beside it, C and D below, as given, each cell shaded by its magnitude; the transfer \
                           function computed from them: G(s) = ({}) / ({}).", poly(&nm), poly(&dn)), &body))
}

fn ss_series(s: &Solved) -> Option<Figure> {
    let (nm, dn) = (s.out_list("num")?, s.out_list("den")?);
    let n = nm.len().max(dn.len());
    if n == 0 || n > 16 {
        return None;
    }
    let pad = |v: &[f64]| -> Vec<Option<f64>> {
        let mut o = vec![None; n - v.len()];
        o.extend(v.iter().map(|x| Some(*x)));
        o
    };
    let labs: Vec<String> = (0..n).map(|i| poly_power(n - 1 - i)).collect();
    let body = k::bar_groups(&labs, &[("numerator".into(), pad(&nm)), ("denominator".into(), pad(&dn))],
                             "Power of s", "Coefficient", &format!("G(s) = ({}) / ({})", poly(&nm), poly(&dn)));
    Some(s.chart("series", "Transfer function coefficients",
                 &format!("The numerator and denominator coefficients computed from the state-space model, by power of \
                           s: G(s) = ({}) / ({}).", poly(&nm), poly(&dn)), &body))
}

fn poly_power(p: usize) -> String {
    match p {
        0 => "1".into(),
        1 => "s".into(),
        _ => format!("s^{p}"),
    }
}

fn node_bars(s: &Solved) -> Option<Figure> {
    let obj = s.outj("node_voltages")?.as_object()?;
    let mut labs = vec![];
    let mut vals = vec![];
    for (kk, v) in obj.iter().take(24) {
        labs.push(format!("node {kk}"));
        vals.push(v.as_f64().filter(|x| x.is_finite())?);
    }
    if vals.is_empty() {
        return None;
    }
    let shown: Vec<String> = labs.iter().zip(&vals).map(|(l, v)| format!("{l} = {}", q(*v, "V"))).collect();
    let body = k::bars(&labs, &vals, &[], "Node", "Voltage to node 0 (V)", &shown.join(", "));
    Some(s.chart("bars", "Node voltages of the circuit",
                 &format!("The voltage of every node relative to node 0 computed for this netlist, one bar per node: \
                           {}.", shown.join(", ")), &body))
}

// ------------------------------------------------------------------------------------------------ control
fn routh(s: &Solved) -> Option<Figure> {
    let col = s.out_list("first_column")?;
    let n = col.len();
    if n == 0 || n > 24 {
        return None;
    }
    let labs: Vec<String> = (0..n).map(|i| format!("s^{}", n - 1 - i)).collect();
    let changes: Vec<usize> = (1..n).filter(|i| col[*i] * col[i - 1] < 0.0).collect();
    let rhp = s.out("rhp_roots").unwrap_or(changes.len() as f64);
    let body = k::bars(&labs, &col, &changes, "Row of the Routh array", "First-column entry",
                       &format!("{} sign changes: {} right-half-plane roots", changes.len(), num(rhp)));
    Some(s.chart("routh", "Routh array: first column and its sign changes",
                 &format!("The first column of the Routh array computed for the characteristic polynomial with the \
                           coefficients given (highest power first): {}. {} sign change(s) (highlighted), so {} \
                           root(s) in the right half-plane.", col.iter().map(|x| num(*x)).collect::<Vec<_>>().join(", "),
                          changes.len(), num(rhp)), &body))
}

fn second_order(s: &Solved) -> Option<Figure> {
    let (z, wn) = (s.inp("zeta")?, s.inp("wn")?);
    if !(z > 0.0 && wn > 0.0) {
        return None;
    }
    let (tp, os, ts, tr) = (s.out("peak_time"), s.out("overshoot"), s.out("settling_time"), s.out("rise_time"));
    let t_end = match (ts, tp) {
        (Some(a), Some(b)) => 1.5 * a.max(b),
        (Some(a), None) => 1.5 * a,
        _ => 10.0 / (z * wn),
    };
    // the response of the model's own series RLC circuit with this ζ and ωn (L = 1 H, C = 1/ωn², R = 2ζωn, 1 V step)
    let (l, c, r) = (1.0, 1.0 / (wn * wn), 2.0 * z * wn);
    let mut pts = vec![];
    for t in k::span(t_end / 120.0, t_end, 61, false) {
        if let Some(a) = s.solve_other("series_rlc_step", &[("R", Input::Float(r)), ("L", Input::Float(l)),
                                                           ("C", Input::Float(c)), ("V", Input::Float(1.0)),
                                                           ("t", Input::Float(t))]) {
            if let Some(v) = k::num_of(&a, "vC") {
                pts.push((t, v));
            }
        }
    }
    if pts.len() < 2 {
        return None;
    }
    // the 2 % settling band: one series, its two edges broken apart by a NaN point
    let band = vec![(0.0, 1.02), (t_end, 1.02), (t_end, f64::NAN), (0.0, 0.98), (t_end, 0.98)];
    let series = vec![("response".to_string(), pts), ("2 % band".to_string(), band)];
    let mut head = vec![];
    if let Some(o) = os {
        head.push(format!("overshoot {} %", num(o * 100.0)));
    }
    if let Some(t) = ts {
        head.push(format!("ts {}", q(t, "s")));
    }
    if let Some(t) = tr {
        head.push(format!("tr {}", q(t, "s")));
    }
    let marker = match (tp, os) {
        (Some(t), Some(o)) => Some((t, 1.0 + o)),
        _ => None,
    };
    let body = k::lines(&series, "Time t (s)", "Output (unit step)", &head.join(", "), marker,
                        &marker.map(|m| format!("peak {} at {}", num(m.1), q(m.0, "s"))).unwrap_or_default(), false);
    Some(s.chart("step", "Step response with overshoot and settling time",
                 &format!("Unit step response of a second-order system with ζ = {} and ωn = {} rad/s, computed as the \
                           capacitor voltage of a series RLC circuit with the same ζ and ωn at 61 instants; marked from \
                           the metrics computed: {}.", num(z), num(wn), head.join(", ")), &body))
}

fn loop_block(s: &Solved) -> Option<Figure> {
    let (nm, dn) = (s.vec("num")?, s.vec("den")?);
    let input = s.txt("input").unwrap_or_else(|| "step".into());
    let (ess, kc, ty) = (s.out("steady_state_error"), s.out("error_constant"), s.out("system_type"));
    let mut c = Canvas::new(240.0);
    c.name(40.0, 96.0, "middle", "R(s)");
    c.name(40.0, 114.0, "middle", &input);
    c.arrow((62.0, 100.0), (116.0, 100.0));
    c.ring((130.0, 100.0), 14.0, "");
    c.name(112.0, 88.0, "middle", "+");
    c.name(142.0, 128.0, "middle", "−");
    c.arrow((144.0, 100.0), (200.0, 100.0));
    c.name(172.0, 90.0, "middle", "E(s)");
    c.block(200.0, 76.0, 140.0, 48.0, "G(s)");
    c.value(270.0, 144.0, "middle", &format!("({}) / ({})", poly(&nm), poly(&dn)));
    c.arrow((340.0, 100.0), (450.0, 100.0));
    c.name(470.0, 104.0, "middle", "C(s)");
    c.dot((400.0, 100.0));
    c.wire(&[(400.0, 100.0), (400.0, 172.0), (130.0, 172.0)]);
    c.arrow((130.0, 172.0), (130.0, 114.0));
    let mut said = vec![];
    if let Some(e) = ess {
        said.push(format!("steady-state error {}", num(e)));
    }
    if let Some(kk) = kc {
        said.push(format!("error constant {}", num(kk)));
    }
    if let Some(t) = ty {
        said.push(format!("type {}", num(t)));
    }
    if said.is_empty() {
        return None;
    }
    c.bold(260.0, 212.0, "middle", &said.join(", "));
    Some(s.figure("block", "Feedback loop and its steady-state error",
                  &format!("Unity-feedback loop with the open-loop G(s) = ({}) / ({}) as given and a {input} input. \
                            Labelled with what was computed: {}.", poly(&nm), poly(&dn), said.join(", ")),
                  c.body(), c.h))
}

pub fn rlc_step_response(s: &Solved) -> Option<Figure> {
    let (r, l, cap, t) = (s.inp("R")?, s.inp("L")?, s.inp("C")?, s.inp("t")?);
    let vc = s.out("vC")?;
    if !(l > 0.0 && cap > 0.0 && t > 0.0) {
        return None;
    }
    let rc = 2.0 * (l / cap).sqrt();
    let third = if r < rc { 2.4 * rc } else { 0.25 * rc };
    let w0 = 1.0 / (l * cap).sqrt();
    let zmax = [r, rc, third].iter().fold(0.0f64, |m, x| m.max(x / rc));
    let t_end = (2.0 * t).max(10.0 * zmax.max(1.0) / w0);
    let ts = k::span(t_end / 60.0, t_end, 61, false);
    let mut series = vec![];
    for rr in [r, rc, third] {
        if series.iter().any(|x: &(String, Vec<P>, f64)| (x.2 - rr).abs() < 1e-9 * rr.abs().max(1.0)) {
            continue;
        }
        let pts = s.curve("t", &ts, "vC", &[("R", rr)]);
        let verdict = s.with(&[("R", rr)]).and_then(|a| text_of(&a, "response")).unwrap_or_default();
        // a short legend name: the resistance and the first word of its kind ("under", "critical", "over")
        let short = verdict.split([' ', '-']).next().unwrap_or("").trim_end_matches("damped").to_string();
        if pts.len() >= 2 {
            series.push((format!("{} {short}", q(rr, "ohm")), pts, rr));
        }
    }
    if series.is_empty() {
        return None;
    }
    let names: Vec<String> = series.iter().map(|x| x.0.clone()).collect();
    let ser: Vec<(String, Vec<P>)> = series.into_iter().map(|x| (x.0, x.1)).collect();
    let body = k::lines(&ser, "Time t (s)", "Capacitor voltage vC (V)", &eq("vC", vc, "V"), Some((t, vc)),
                        &format!("t = {}", q(t, "s")), false);
    Some(s.chart("response", "Series RLC step response: under-, critically and over-damped",
                 &format!("Capacitor voltage of the series RLC circuit switched onto the source, for the resistance \
                           given and two others ({}), 31 instants each computed by the model. Marked: t = {}, vC = {}.",
                          names.join("; "), q(t, "s"), q(vc, "V")), &body))
}

pub fn rlc_step_decision(s: &Solved) -> Option<Figure> {
    let (r, l, cap) = (s.inp("R")?, s.inp("L")?, s.inp("C")?);
    if !(l > 0.0 && cap > 0.0 && r >= 0.0) {
        return None;
    }
    let rc = 2.0 * (l / cap).sqrt();
    let rhi = (2.0 * r).max(2.5 * rc * 1.5f64.sqrt());
    let xs = k::span(rhi / 40.0, rhi, 11, false);
    let ys = k::span(l / 4.0, l * 1.5, 9, false);
    let mut cells = vec![];
    let mut names: Vec<String> = vec![];
    for (i, x) in xs.iter().enumerate() {
        for (j, y) in ys.iter().enumerate() {
            if let Some(a) = s.with(&[("R", *x), ("L", *y)]) {
                if let Some(t) = text_of(&a, "response") {
                    if !names.contains(&t) {
                        names.push(t.clone());
                    }
                    cells.push((i, j, t));
                }
            }
        }
    }
    if cells.is_empty() {
        return None;
    }
    let body = k::decision(&xs, &ys, &cells, &names, (r, l), "Resistance R (Ω)", "Inductance L (H)",
                           &format!("{} here", s.out_text("response").unwrap_or_default()));
    Some(s.chart("decision", "Where the step response changes its kind",
                 &format!("Underdamped, critically damped or overdamped over R from {} to {} and L from {} to {}: each \
                           of the {} cells is one computation of the model, coloured by the kind it returned. Marked: \
                           the inputs given.", q(xs[0], "ohm"), q(rhi, "ohm"), q(ys[0], "H"), q(l * 1.5, "H"),
                          cells.len()), &body))
}

// ------------------------------------------------------------------------------------------------ phasors
fn phasors(s: &Solved) -> Option<Figure> {
    let (amps, phs) = (s.vec("amplitudes")?, s.vec("phases_deg")?);
    let (re, im, amp, ph) = (s.out("real")?, s.out("imag")?, s.out("amplitude")?, s.out("phase_deg")?);
    if amps.len() != phs.len() || amps.is_empty() || amps.len() > 6 {
        return None;
    }
    let mut v = vec![];
    for (a, p) in amps.iter().zip(&phs) {
        let r = s.solve_other("polar_to_rectangular", &[("magnitude", Input::Float(*a)), ("angle_deg", Input::Float(*p))])?;
        let (x, y) = (k::num_of(&r, "real")?, k::num_of(&r, "imag")?);
        v.push((format!("{}∠{}°", num(*a), num(*p)), (k::snap(x, *a), k::snap(y, *a)), false));
    }
    let (re, im) = (k::snap(re, amp), k::snap(im, amp));
    v.push(("sum".into(), (re, im), true));
    let body = k::arrows(&v, &format!("sum = {} ∠ {}°", num(amp), num(ph)));
    Some(s.chart("phasor", "Phasors and their sum",
                 &format!("The phasors given ({}), each converted to rectangular form by the model, and their sum \
                           computed: {} ∠ {}° = {}.", v[..v.len() - 1].iter().map(|x| x.0.clone()).collect::<Vec<_>>()
                          .join(", "), num(amp), num(ph), cx((re, im))), &body))
}

fn phasor_sensitivity(s: &Solved) -> Option<Figure> {
    let amps = s.vec("amplitudes")?;
    let amp = s.out("amplitude")?;
    let last = amps.len().checked_sub(1)?;
    let a0 = amps[last];
    let (lo, hi) = if a0 > 0.0 { (a0 * 0.5, a0 * 1.5) } else { (0.0, 1.0) };
    let mut pts = vec![];
    for x in k::span(lo, hi, 31, false) {
        let mut list = amps.clone();
        list[last] = x;
        if let Some(a) = s.with_input(&[("amplitudes", Input::Json(json!(list)))]) {
            if let Some(y) = k::num_of(&a, "amplitude") {
                pts.push((x, y));
            }
        }
    }
    if pts.len() < 2 {
        return None;
    }
    let n = last + 1;
    let body = k::lines(&[("amplitude".into(), pts)], &format!("Amplitude {n} of the list"), "Amplitude of the sum",
                        &format!("amplitude = {}", num(amp)), Some((a0, amp)), &format!("given: {}", num(a0)), false);
    Some(s.chart("sensitivity", "Amplitude of the sum as one amplitude moves",
                 &format!("The amplitude of the sum as amplitude {n} moves from {} to {}, the others as given; 31 \
                           computations of the model. Marked: the inputs given, amplitude = {}.", num(lo), num(hi),
                          num(amp)), &body))
}

// ------------------------------------------------------------------------------------------------ digital
fn weights(n: usize, signed: bool) -> Vec<String> {
    if n > 12 {
        return vec![];
    }
    (0..n).map(|j| {
        let p = (n - 1 - j) as u32;
        let w = 1i64 << p;
        if signed && j == 0 { format!("-{w}") } else { w.to_string() }
    }).collect()
}

fn is_bits(t: &str) -> bool {
    !t.is_empty() && t.len() <= 32 && t.chars().all(|c| c == '0' || c == '1')
}

fn base_bits(s: &Solved) -> Option<Figure> {
    let b = s.out_text("binary")?;
    if !is_bits(&b) {
        return None;
    }
    let (d, o, h) = (s.out("decimal")?, s.out_text("octal")?, s.out_text("hexadecimal")?);
    let head = format!("decimal {} = octal {o} = hexadecimal {h}", num(d));
    let (body, hh) = k::bits(&[("binary".into(), b.clone())], &weights(b.len(), false), &head);
    Some(s.figure("bits", "One value in binary, with bit weights",
                  &format!("The value given, as the binary digits computed with the weight of each bit above it; \
                            computed: {head}, binary {b}."), &body, hh))
}

fn gray_bits(s: &Solved) -> Option<Figure> {
    let given = s.txt("value")?;
    let res = s.out_text("result")?;
    if !is_bits(&given) || !is_bits(&res) {
        return None;
    }
    let to_gray = s.txt("direction").map(|d| d != "to_binary").unwrap_or(true);
    let rows = if to_gray { vec![("binary".into(), given.clone()), ("Gray code".into(), res.clone())] } else {
        vec![("Gray code".into(), given.clone()), ("binary".into(), res.clone())]
    };
    let dec = s.out("decimal").map(num).unwrap_or_default();
    let (body, h) = k::bits(&rows, &[], &format!("result {res}, decimal {dec}"));
    Some(s.figure("bits", "Bit patterns: binary and Gray code",
                  &format!("The bit pattern given ({given}) and the code computed, {res} (1 bits shaded); decimal \
                            {dec}."), &body, h))
}

fn twos_bits(s: &Solved, decode: bool) -> Option<Figure> {
    let pat = if decode { s.txt("pattern")? } else { s.out_text("pattern")? };
    if !is_bits(&pat) {
        return None;
    }
    let value = if decode { s.out("value")? } else { s.val("value")?.num() };
    let head = if decode { format!("value = {}", num(value)) } else {
        format!("{} -> {pat}, hexadecimal {}, unsigned {}", num(value), s.out_text("hexadecimal").unwrap_or_default(),
                s.out("unsigned").map(num).unwrap_or_default())
    };
    let (body, h) = k::bits(&[(format!("{} bits", pat.len()), pat.clone())], &weights(pat.len(), true), &head);
    Some(s.figure("bits", "Two's-complement pattern with bit weights",
                  &format!("The {}-bit pattern {pat} with the two's-complement weight of each bit above it (the leading \
                            bit weighs negative); computed: {head}.", pat.len()), &body, h))
}

/// The variable names of a logic expression (identifiers other than the operator words), sorted.
fn variables(t: &str) -> Vec<String> {
    let mut out: Vec<String> = vec![];
    let cs: Vec<char> = t.chars().collect();
    let mut i = 0;
    while i < cs.len() {
        if cs[i].is_ascii_alphabetic() {
            let st = i;
            while i < cs.len() && (cs[i].is_ascii_alphanumeric() || cs[i] == '_') {
                i += 1;
            }
            let w: String = cs[st..i].iter().collect();
            let up = w.to_uppercase();
            if !["AND", "OR", "NOT", "XOR", "NAND", "NOR", "XNOR", "TRUE", "FALSE"].contains(&up.as_str())
                && !out.contains(&w) {
                out.push(w);
            }
        } else {
            i += 1;
        }
    }
    out.sort();
    out
}

fn assignment(vars: &[String], row: usize) -> String {
    let n = vars.len();
    vars.iter().enumerate().map(|(j, v)| format!("{v}={}", (row >> (n - 1 - j)) & 1)).collect::<Vec<_>>().join(", ")
}

fn evaluate(s: &Solved, expr: &str, values: &str) -> Option<f64> {
    let a = s.solve_other("boolean_evaluate", &[("expression", Input::Text(expr.to_string())),
                                                ("values", Input::Text(values.to_string()))])?;
    truthy(&a, "value")
}

fn equivalence_truth(s: &Solved) -> Option<Figure> {
    let (e1, e2) = (s.txt("expression1")?, s.txt("expression2")?);
    let mut vars = variables(&e1);
    for v in variables(&e2) {
        if !vars.contains(&v) {
            vars.push(v);
        }
    }
    vars.sort();
    if vars.is_empty() || vars.len() > 5 {
        return None;
    }
    let rows = 1usize << vars.len();
    let mut c1 = vec![];
    let mut c2 = vec![];
    for r in 0..rows {
        let asg = assignment(&vars, r);
        c1.push(Some(evaluate(s, &e1, &asg)?));
        c2.push(Some(evaluate(s, &e2, &asg)?));
    }
    let agree = (0..rows).filter(|r| c1[*r] == c2[*r]).count();
    let mark = (0..rows).find(|r| c1[*r] != c2[*r]);
    let eqv = s.out_text("equivalent").unwrap_or_default();
    let body = k::truth(&vars, &[("expr 1".into(), c1), ("expr 2".into(), c2)], mark,
                        &format!("equivalent: {eqv} ({agree} of {rows} rows agree)"));
    Some(s.chart("truth", "Truth table of both expressions",
                 &format!("Truth table of expression 1, {e1}, and expression 2, {e2}, over every assignment of {} (rows \
                           numbered by the assignment as a binary number); each entry is the value the model computed. \
                           The columns agree in {agree} of {rows} rows; equivalent: {eqv}.", vars.join(", ")), &body))
}

fn kmap(s: &Solved) -> Option<Figure> {
    let vars: Vec<String> = s.out_text("variables")?.split(',').map(|x| x.trim().to_string())
        .filter(|x| !x.is_empty()).collect();
    let n = vars.len();
    if !(2..=6).contains(&n) {
        return None;
    }
    let mins: Vec<f64> = if s.key == "truth_table_sop" { s.out_list("minterms").unwrap_or_default() } else {
        s.vec("minterms")?
    };
    let dc: Vec<f64> = s.vec("dont_cares").unwrap_or_default();
    let mut vals: Vec<Option<f64>> = vec![Some(0.0); 1 << n];
    for m in &mins {
        let i = *m as usize;
        if i < vals.len() {
            vals[i] = Some(1.0);
        }
    }
    for m in &dc {
        let i = *m as usize;
        if i < vals.len() {
            vals[i] = None;
        }
    }
    let sop = s.out_text("minimal_sop")?;
    let body = k::kmap(&vars, &vals, &format!("F = {sop}"));
    Some(s.chart("kmap", "Karnaugh map with the minimal sum of products",
                 &format!("Karnaugh map over {} (Gray-coded rows and columns): the minterms ({}) marked 1{}; the minimal \
                           sum of products computed: {sop}.", vars.join(", "),
                          mins.iter().map(|m| num(*m)).collect::<Vec<_>>().join(", "),
                          if dc.is_empty() { "" } else { ", the don't-cares blank" }), &body))
}

// ------------------------------------------------------------------------------------------------ logic gates
#[derive(Debug, Clone)]
enum G {
    Var(String),
    Const(bool),
    Not(Box<G>),
    Op(&'static str, Vec<G>),
}

fn lex(t: &str) -> Option<Vec<String>> {
    let mut out = vec![];
    let cs: Vec<char> = t.chars().collect();
    let mut i = 0;
    while i < cs.len() {
        let c = cs[i];
        if c.is_whitespace() {
            i += 1;
        } else if c.is_ascii_alphabetic() {
            let st = i;
            while i < cs.len() && (cs[i].is_ascii_alphanumeric() || cs[i] == '_') {
                i += 1;
            }
            out.push(cs[st..i].iter().collect());
        } else if c == '0' || c == '1' {
            out.push(c.to_string());
            i += 1;
        } else if "()'!~¬&·*∧+|∨^⊕".contains(c) {
            out.push(c.to_string());
            i += 1;
        } else {
            return None;
        }
    }
    Some(out)
}

struct Parser {
    t: Vec<String>,
    i: usize,
}

impl Parser {
    fn peek(&self) -> Option<String> {
        self.t.get(self.i).map(|x| x.to_uppercase())
    }
    fn or(&mut self, d: usize) -> Option<G> {
        let mut v = vec![self.xor(d)?];
        while matches!(self.peek().as_deref(), Some("OR") | Some("+") | Some("|") | Some("∨")) {
            self.i += 1;
            v.push(self.xor(d)?);
        }
        Some(if v.len() == 1 { v.pop()? } else { G::Op("OR", v) })
    }
    fn xor(&mut self, d: usize) -> Option<G> {
        let mut v = vec![self.and(d)?];
        while matches!(self.peek().as_deref(), Some("XOR") | Some("^") | Some("⊕")) {
            self.i += 1;
            v.push(self.and(d)?);
        }
        Some(if v.len() == 1 { v.pop()? } else { G::Op("XOR", v) })
    }
    fn and(&mut self, d: usize) -> Option<G> {
        let mut v = vec![self.not(d)?];
        while matches!(self.peek().as_deref(), Some("AND") | Some("&") | Some("·") | Some("*") | Some("∧")) {
            self.i += 1;
            v.push(self.not(d)?);
        }
        Some(if v.len() == 1 { v.pop()? } else { G::Op("AND", v) })
    }
    fn not(&mut self, d: usize) -> Option<G> {
        if matches!(self.peek().as_deref(), Some("NOT") | Some("!") | Some("~") | Some("¬")) {
            self.i += 1;
            return Some(G::Not(Box::new(self.not(d)?)));
        }
        let mut g = self.atom(d)?;
        while self.peek().as_deref() == Some("'") {
            self.i += 1;
            g = G::Not(Box::new(g));
        }
        Some(g)
    }
    fn atom(&mut self, d: usize) -> Option<G> {
        if d > 10 {
            return None;
        }
        let tok = self.t.get(self.i)?.clone();
        self.i += 1;
        match tok.as_str() {
            "(" => {
                let g = self.or(d + 1)?;
                (self.t.get(self.i)? == ")").then_some(())?;
                self.i += 1;
                Some(g)
            }
            "0" => Some(G::Const(false)),
            "1" => Some(G::Const(true)),
            w if w.chars().next()?.is_ascii_alphabetic()
                && !["AND", "OR", "NOT", "XOR"].contains(&w.to_uppercase().as_str()) => Some(G::Var(w.to_string())),
            _ => None,
        }
    }
}

fn parse_logic(t: &str) -> Option<G> {
    let mut p = Parser { t: lex(t)?, i: 0 };
    let g = p.or(0)?;
    (p.i == p.t.len()).then_some(g)
}

/// The expression written back in words, fully parenthesised (what the model evaluates for each gate).
fn render(g: &G) -> String {
    match g {
        G::Var(v) => v.clone(),
        G::Const(b) => if *b { "1".into() } else { "0".into() },
        G::Not(x) => format!("NOT ({})", render(x)),
        G::Op(op, v) => v.iter().map(|x| format!("({})", render(x))).collect::<Vec<_>>().join(&format!(" {op} ")),
    }
}

fn height(g: &G) -> usize {
    match g {
        G::Var(_) | G::Const(_) => 0,
        G::Not(x) => 1 + height(x),
        G::Op(_, v) => 1 + v.iter().map(height).max().unwrap_or(0),
    }
}

fn leaves(g: &G) -> usize {
    match g {
        G::Var(_) | G::Const(_) => 1,
        G::Not(x) => leaves(x),
        G::Op(_, v) => v.iter().map(leaves).sum(),
    }
}

struct GateDraw<'a> {
    s: &'a Solved<'a>,
    values: String,
    vals: std::collections::HashMap<String, String>,
    c: Canvas,
    leaf: usize,
    dx: f64,
    dy: f64,
}

impl GateDraw<'_> {
    /// draws g; returns its output point
    fn draw(&mut self, g: &G) -> Option<P> {
        match g {
            G::Var(_) | G::Const(_) => {
                let y = 46.0 + self.dy * (self.leaf as f64 + 0.5);
                self.leaf += 1;
                let name = match g {
                    G::Var(v) => format!("{v} = {}", self.vals.get(v).cloned().unwrap_or_else(|| "?".into())),
                    _ => render(g),
                };
                self.c.value(16.0, y + 4.0, "start", &name);
                Some((76.0, y))
            }
            _ => {
                let kids: Vec<&G> = match g {
                    G::Not(x) => vec![x.as_ref()],
                    G::Op(_, v) => v.iter().collect(),
                    _ => vec![],
                };
                let mut outs = vec![];
                for kd in &kids {
                    outs.push(self.draw(kd)?);
                }
                let gx = 90.0 + self.dx * (height(g) as f64 - 1.0) + 40.0;
                let gy = outs.iter().map(|p| p.1).sum::<f64>() / outs.len() as f64;
                let n = outs.len();
                for (i, p) in outs.iter().enumerate() {
                    let iy = if n == 1 { gy } else { gy - 12.0 + 24.0 * i as f64 / (n - 1) as f64 };
                    let mid = gx - 14.0;
                    self.c.wire(&[*p, (mid, p.1), (mid, iy), (gx + 4.0, iy)]);
                }
                let path = match g {
                    G::Not(_) => format!("M{gx:.1},{:.1} L{:.1},{gy:.1} L{gx:.1},{:.1} Z", gy - 13.0, gx + 32.0, gy + 13.0),
                    G::Op("AND", _) => format!("M{gx:.1},{:.1} L{:.1},{:.1} A18,18 0 0 1 {:.1},{:.1} L{gx:.1},{:.1} Z",
                                               gy - 18.0, gx + 22.0, gy - 18.0, gx + 22.0, gy + 18.0, gy + 18.0),
                    _ => format!("M{gx:.1},{:.1} Q{:.1},{:.1} {:.1},{gy:.1} Q{:.1},{:.1} {gx:.1},{:.1} Q{:.1},{gy:.1} {gx:.1},{:.1} Z",
                                 gy - 18.0, gx + 26.0, gy - 18.0, gx + 42.0, gx + 26.0, gy + 18.0, gy + 18.0, gx + 10.0,
                                 gy - 18.0),
                };
                self.c.raw(&format!("<path class=\"axp-ka\" d=\"{path}\"/><path class=\"axp-k\" d=\"{path}\"/>"));
                if matches!(g, G::Op("XOR", _)) {
                    self.c.raw(&format!("<path class=\"axp-k\" d=\"M{:.1},{:.1} Q{:.1},{gy:.1} {:.1},{:.1}\"/>",
                                        gx - 6.0, gy - 18.0, gx + 4.0, gx - 6.0, gy + 18.0));
                }
                if matches!(g, G::Not(_)) {
                    self.c.raw(&format!("<circle class=\"axp-k\" cx=\"{:.1}\" cy=\"{gy:.1}\" r=\"4\"/>", gx + 36.0));
                }
                let out = (gx + 46.0, gy);
                self.c.line((gx + if matches!(g, G::Not(_)) { 40.0 } else { 42.0 }, gy), out);
                // this gate's output as the model evaluates it
                let val = evaluate(self.s, &render(g), &self.values)?;
                self.c.value(gx + 44.0, gy - 8.0, "start", &num(val));
                Some(out)
            }
        }
    }
}

fn gates(s: &Solved) -> Option<Figure> {
    let expr = s.txt("expression")?;
    let values = s.txt("values")?;
    let result = truthy(s.a, "value")?;
    let g = parse_logic(&expr)?;
    let (h, nl) = (height(&g), leaves(&g));
    if h == 0 || h > 5 || nl > 8 {
        return None;
    }
    let mut vals = std::collections::HashMap::new();
    for part in values.split([',', ';']) {
        if let Some((kk, vv)) = part.split_once('=') {
            vals.insert(kk.trim().to_string(), vv.trim().to_string());
        }
    }
    let dy = 40.0;
    let hh = 46.0 + dy * nl as f64 + 40.0;
    let dx = (330.0 / h as f64).min(100.0);
    let mut d = GateDraw { s, values: values.clone(), vals, c: Canvas::new(hh.max(150.0)), leaf: 0, dx, dy };
    let out = d.draw(&g)?;
    // the drawing agrees with the answer, or it is not shown
    if evaluate(s, &render(&g), &values)? != result {
        return None;
    }
    d.c.line(out, (out.0 + 20.0, out.1));
    d.c.bold(out.0 + 24.0, out.1 + 5.0, "start", &format!("= {}", num(result)));
    d.c.bold(16.0, 24.0, "start", &format!("{expr}  with  {values}"));
    let c = d.c;
    Some(s.figure("gates", "Gate schematic of the expression with its output",
                  &format!("The expression {expr} drawn as gates with the inputs given ({values}); the value on each \
                            gate's output is what the model computed for that part, and the output is {}.",
                           num(result)), c.body(), c.h))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn polynomials_read() {
        assert_eq!(poly(&[1.0, 3.0, 2.0]), "s² + 3s + 2");
        assert_eq!(poly(&[0.0, 0.0, 1.0]), "1");
        assert_eq!(poly(&[1.0, 6.0, 5.0, 0.0]), "s³ + 6s² + 5s");
        assert_eq!(poly(&[-2.0, 1.0]), "-2s + 1");
    }

    #[test]
    fn logic_parses() {
        let g = parse_logic("A AND (B OR NOT C)").unwrap();
        assert_eq!(render(&g), "(A) AND ((B) OR (NOT (C)))");
        assert_eq!(height(&g), 3);
        assert_eq!(leaves(&g), 3);
        let g = parse_logic("(A·B)'").unwrap();
        assert!(matches!(g, G::Not(_)));
        assert!(parse_logic("A AND").is_none());
        assert_eq!(variables("A' + B'·C AND NOT d"), vec!["A", "B", "C", "d"]);
    }

    #[test]
    fn weights_signed() {
        assert_eq!(weights(4, true), vec!["-8", "4", "2", "1"]);
        assert!(weights(16, false).is_empty());
    }
}
