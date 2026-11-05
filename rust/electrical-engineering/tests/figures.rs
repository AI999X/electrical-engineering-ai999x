//! Every Electrical Engineering capability's own live figures (src/figures.rs): its worked example gives each
//! published figure kind, a changed input changes the drawing, and no drawing holds a non-finite number or text
//! outside its canvas.

use ax_engine::figures::{figures, Figure};
use ax_engine::Input;
use electrical_engineering_ai999x::pack;

/// The published figure kinds of every capability (publish/website/models/electrical-engineering-ai999x/figures,
/// except the generic "curve"), as tools/figure_parity.py lists them.
const KINDS: &[(&str, &[&str])] = &[
    ("adc_conversion", &["transfer"]),
    ("base_conversion", &["bits"]),
    ("bjt_divider_bias", &["schematic", "curve"]),
    ("bode_point", &["bode"]),
    ("boolean_equivalence", &["truth"]),
    ("boolean_evaluate", &["gates"]),
    ("closed_loop_poles", &["locus"]),
    ("complex_power", &["decision", "triangle"]),
    ("controllability_observability", &["matrix"]),
    ("current_divider", &["schematic"]),
    ("dc_motor", &["schematic"]),
    ("dc_node_voltages", &["bars"]),
    ("delta_to_wye", &["schematic"]),
    ("difference_amplifier", &["schematic"]),
    ("diode_circuit", &["schematic"]),
    ("gain_margin", &["bode"]),
    ("gray_code", &["bits"]),
    ("inductor_energy", &["field"]),
    ("inverting_amplifier", &["schematic"]),
    ("lead_compensator", &["field"]),
    ("max_power_transfer", &["schematic"]),
    ("mesh_currents", &["heatmap", "matrix", "vectors"]),
    ("minterms_to_sop", &["kmap"]),
    ("mosfet_saturation", &["transfer"]),
    ("nodal_voltages", &["matrix", "vectors"]),
    ("noninverting_amplifier", &["schematic"]),
    ("norton_equivalent", &["schematic"]),
    ("opamp_integrator", &["schematic"]),
    ("parallel_capacitance", &["schematic"]),
    ("parallel_inductance", &["schematic"]),
    ("parallel_resistance", &["schematic"]),
    ("parallel_rlc_impedance", &["schematic"]),
    ("parallel_rlc_resonance", &["schematic"]),
    ("phase_margin", &["bode"]),
    ("phasor_sum", &["phasor", "sensitivity"]),
    ("polar_to_rectangular", &["field"]),
    ("rc_filter", &["schematic"]),
    ("rectangular_to_polar", &["field"]),
    ("resistor_network", &["schematic"]),
    ("rms_waveform", &["field"]),
    ("routh_hurwitz", &["routh"]),
    ("second_order_step", &["step"]),
    ("series_capacitance", &["schematic"]),
    ("series_inductance", &["schematic"]),
    ("series_resistance", &["schematic"]),
    ("series_rlc_impedance", &["schematic"]),
    ("series_rlc_resonance", &["schematic"]),
    ("series_rlc_step", &["decision", "response", "schematic"]),
    ("state_space_to_tf", &["matrix", "series"]),
    ("steady_state_error", &["block"]),
    ("summing_amplifier", &["schematic"]),
    ("thevenin_equivalent", &["schematic"]),
    ("three_phase_load", &["schematic"]),
    ("transformer", &["schematic"]),
    ("truth_table_sop", &["kmap"]),
    ("twos_complement_decode", &["bits"]),
    ("twos_complement_encode", &["bits"]),
    ("voltage_divider", &["schematic"]),
    ("wheatstone_bridge", &["schematic"]),
    ("wye_to_delta", &["schematic"]),
    ("zener_regulator", &["schematic"]),
];

/// Inputs to change for a capability whose inputs are not plain quantities (a second, different example).
const OTHER: &[(&str, &str, &str)] = &[
    ("base_conversion", "value", "1101"),
    ("gray_code", "value", "0110"),
    ("twos_complement_decode", "pattern", "10000110"),
    ("twos_complement_encode", "value", "-77"),
    ("boolean_equivalence", "expression2", "A' · B'"),
    ("boolean_evaluate", "values", "A=1, B=1, C=1"),
    ("truth_table_sop", "expression", "A·B + A'·B'"),
    ("minterms_to_sop", "minterms", "1, 3, 5, 7"),
    ("dc_node_voltages", "netlist", "V1 1 0 10; R1 1 2 1000; R2 2 0 3000"),
    ("thevenin_equivalent", "netlist", "V1 1 0 10; R1 1 2 1000; R2 2 0 3000"),
    ("norton_equivalent", "netlist", "V1 1 0 10; R1 1 2 1000; R2 2 0 3000"),
    ("resistor_network", "network", "(10 || 40) + 15"),
    ("controllability_observability", "A", "[[0, 1], [-5, -2]]"),
    ("state_space_to_tf", "A", "[[0, 1], [-5, -2]]"),
    ("mesh_currents", "b", "[10, 5]"),
    ("nodal_voltages", "b", "[1, 3]"),
    ("routh_hurwitz", "coefficients", "1, 4, 6, 4, 1"),
    ("closed_loop_poles", "den", "1, 2, 0"),
    ("gain_margin", "num", "20"),
    ("phase_margin", "num", "9"),
    ("steady_state_error", "num", "4"),
    ("phasor_sum", "phases_deg", "0, 120"),
];

/// The arguments of a structured example: solve('key', a='1 V', b=8, c="(A·B)'").
fn example(key: &str) -> Vec<(String, Input)> {
    let p = pack();
    let ex = &p.procedure(key).expect(key).structured_example;
    let cs: Vec<char> = ex.chars().collect();
    let mut i = ex.find(',').map(|x| ex[..x].chars().count() + 1).unwrap_or(cs.len());
    let mut out = vec![];
    while i < cs.len() {
        while i < cs.len() && (cs[i] == ' ' || cs[i] == ',') {
            i += 1;
        }
        if i >= cs.len() || cs[i] == ')' {
            break;
        }
        let st = i;
        while cs[i] != '=' {
            i += 1;
        }
        let name: String = cs[st..i].iter().collect();
        i += 1;
        let value: String;
        if cs[i] == '\'' || cs[i] == '"' {
            let qch = cs[i];
            let s0 = i + 1;
            i += 1;
            while cs[i] != qch {
                i += 1;
            }
            value = cs[s0..i].iter().collect();
            i += 1;
        } else {
            let s0 = i;
            while i < cs.len() && cs[i] != ',' && cs[i] != ')' {
                i += 1;
            }
            value = cs[s0..i].iter().collect();
        }
        out.push((name.trim().to_string(), Input::Text(value.trim().to_string())));
    }
    out
}

fn kind_of(f: &Figure) -> &str {
    &f.kind
}

/// Every text inside the canvas, no non-finite number (ax_engine::figkit::check_figure).
fn check_texts(f: &Figure) {
    if let Err(e) = ax_engine::figkit::check_figure(f) {
        panic!("{e}");
    }
}

#[test]
fn every_capability_draws_its_published_kinds() {
    let p = pack();
    let mut missing = vec![];
    for (key, kinds) in KINDS {
        let given = example(key);
        let figs = figures(p, key, &given, None, None);
        for kind in *kinds {
            match figs.iter().find(|f| kind_of(f) == *kind) {
                Some(f) => {
                    assert_eq!(f.slug, format!("{}-{kind}", key.replace('_', "-")));
                    assert!(!f.svg.contains("NaN") && !f.svg.contains("inf"), "{key} {kind}: a non-finite number");
                    check_texts(f);
                }
                None => missing.push(format!("{key}: {kind}")),
            }
        }
        // the model's own figures come first
        if let Some(i) = figs.iter().position(|f| f.slug.contains("-live-")) {
            assert!(figs[i..].iter().all(|f| f.slug.contains("-live-")), "{key}: generic figures before own ones");
        }
    }
    assert!(missing.is_empty(), "missing figures: {missing:?}");
}

/// Numbers in a text multiplied by f ("12 V" -> "16.44 V"; "3, 6" -> "4.11, 8.22").
fn scale_text(t: &str, f: f64) -> Option<String> {
    let mut out = String::new();
    let mut cur = String::new();
    let mut changed = false;
    let flush = |cur: &mut String, out: &mut String, changed: &mut bool| {
        if !cur.is_empty() {
            match cur.parse::<f64>() {
                Ok(x) if x != 0.0 => {
                    out.push_str(&format!("{}", x * f));
                    *changed = true;
                }
                _ => out.push_str(cur),
            }
            cur.clear();
        }
    };
    for c in t.chars() {
        if c.is_ascii_digit() || c == '.' || (c == '-' && cur.is_empty()) || ((c == 'e' || c == 'E') && !cur.is_empty()) {
            cur.push(c);
        } else {
            flush(&mut cur, &mut out, &mut changed);
            out.push(c);
        }
    }
    flush(&mut cur, &mut out, &mut changed);
    changed.then_some(out)
}

#[test]
fn a_changed_input_changes_the_drawing() {
    let p = pack();
    let mut unchanged = vec![];
    for (key, kinds) in KINDS {
        let given = example(key);
        let base = figures(p, key, &given, None, None);
        // the other example: a listed replacement, else each number input scaled until the model solves it
        let mut tries: Vec<Vec<(String, Input)>> = vec![];
        if let Some((_, sym, val)) = OTHER.iter().find(|o| o.0 == *key) {
            let mut g = given.clone();
            for e in g.iter_mut() {
                if e.0 == *sym {
                    e.1 = Input::Text(val.to_string());
                }
            }
            tries.push(g);
        }
        for f in [1.37, 0.73] {
            for i in 0..given.len() {
                if let Input::Text(t) = &given[i].1 {
                    if let Some(t2) = scale_text(t, f) {
                        let mut g = given.clone();
                        g[i].1 = Input::Text(t2);
                        tries.push(g);
                    }
                }
            }
        }
        let mut ok = false;
        for g in tries {
            let a = ax_engine::solve(p, key, &g, None);
            if !a.solved() {
                continue;
            }
            let figs = figures(p, key, &g, None, None);
            let differs = kinds.iter().all(|kind| {
                match (base.iter().find(|f| f.kind == *kind), figs.iter().find(|f| f.kind == *kind)) {
                    (Some(a), Some(b)) => a.svg != b.svg,
                    _ => false,
                }
            });
            if differs {
                ok = true;
                for f in figs.iter().filter(|f| !f.slug.contains("-live-")) {
                    assert!(!f.svg.contains("NaN"), "{key}: a non-finite number");
                    check_texts(f);
                }
                break;
            }
        }
        if !ok {
            unchanged.push(*key);
        }
    }
    assert!(unchanged.is_empty(), "drawing did not follow a changed input: {unchanged:?}");
}

#[test]
fn an_impossible_input_draws_nothing_wrong() {
    // a saturated transistor is refused by the model: no figure at all, never a wrong one
    let p = pack();
    let given: Vec<(String, Input)> = [("VCC", "12 V"), ("R1", "1000 ohm"), ("R2", "10000 ohm"), ("RC", "10000 ohm"),
                                       ("RE", "1000 ohm"), ("beta", "100"), ("VBE", "0.7 V")]
        .iter().map(|(k, v)| (k.to_string(), Input::Text(v.to_string()))).collect();
    let a = ax_engine::solve(p, "bjt_divider_bias", &given, None);
    let figs = figures(p, "bjt_divider_bias", &given, None, None);
    if !a.solved() {
        assert!(figs.is_empty());
    } else {
        let vce = a.values.iter().find(|v| v.0 == "VCE").and_then(|v| v.1.as_f64()).unwrap();
        assert!(vce >= 0.0, "a solved operating point with a negative VCE");
    }
}
