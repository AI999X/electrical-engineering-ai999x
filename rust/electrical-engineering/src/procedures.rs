//! electrical_engineering: procedures translated from the package's Python by tools/py2rust.py (do not edit; regenerate).
//! Source: models/electrical_engineering/capabilities/procedures.py. Each keeps Python's number rules through
//! ax_engine::py and is checked against the package by tools/rust_accuracy.py before the build is listed.
#![allow(clippy::all, non_snake_case, unused_parens, unused_braces, unused_mut, unused_variables, unreachable_code)]

use ax_engine::py::{self, Py, PyErr, R};
use ax_engine::Val;
use serde_json::Value;
use std::collections::HashMap;

/// The procedures this crate implements (tools/export_rust_pack.py reads this list).
pub const NATIVE: &[&str] = &[
    "adc_conversion",
    "base_conversion",
    "bjt_divider_bias",
    "bode_point",
    "boolean_equivalence",
    "boolean_evaluate",
    "closed_loop_poles",
    "complex_power",
    "controllability_observability",
    "current_divider",
    "dc_motor",
    "dc_node_voltages",
    "delta_to_wye",
    "difference_amplifier",
    "diode_circuit",
    "first_order_time_to_value",
    "first_order_transient",
    "gain_margin",
    "gray_code",
    "inductor_energy",
    "inverting_amplifier",
    "lead_compensator",
    "max_power_transfer",
    "mesh_currents",
    "minterms_to_sop",
    "mosfet_saturation",
    "nodal_voltages",
    "noninverting_amplifier",
    "norton_equivalent",
    "opamp_integrator",
    "parallel_capacitance",
    "parallel_inductance",
    "parallel_resistance",
    "parallel_rlc_impedance",
    "parallel_rlc_resonance",
    "phase_margin",
    "phasor_sum",
    "polar_to_rectangular",
    "power_factor_correction",
    "rc_filter",
    "rectangular_to_polar",
    "resistor_network",
    "rms_waveform",
    "routh_hurwitz",
    "second_order_step",
    "series_capacitance",
    "series_inductance",
    "series_resistance",
    "series_rlc_impedance",
    "series_rlc_resonance",
    "series_rlc_step",
    "state_space_to_tf",
    "steady_state_error",
    "summing_amplifier",
    "thevenin_equivalent",
    "three_phase_load",
    "transformer",
    "truth_table_sop",
    "twos_complement_decode",
    "twos_complement_encode",
    "voltage_divider",
    "wheatstone_bridge",
    "wye_to_delta",
    "zener_regulator",
];

type Out = Result<(Vec<(String, Value)>, f64), String>;

/// The run's result as the engine's answer: a dict of outputs and the residual of the package's own check; a
/// ValueError is the refusal's reason; any other error is an internal error (None).
fn finish(r: R, residual: impl Fn(&Py, &Py) -> f64, v: &HashMap<String, Val>) -> Option<Out> {
    let o = match r {
        Ok(o) => o,
        Err(PyErr::Value(m)) => return Some(Err(m)),
        Err(e) => {
            if std::env::var_os("AX_DEBUG").is_some() {        // the error behind an internal error, for diagnosis
                eprintln!("internal error: {e:?}");
            }
            return None;
        }
    };
    let Py::Dict(d) = &o else { return None };
    let outs: Vec<(String, Value)> = d.borrow().iter().map(|(k, x)| (py::to_str(k), py::to_json(x))).collect();
    let i = Py::dict(v.iter().map(|(k, x)| (Py::str(k), py::from_val(x))).collect());
    let res = residual(&i, &o);
    if std::env::var_os("AX_DEBUG").is_some() && !(res <= 1e-9) {
        eprintln!("check failed: residual {res}");
    }
    Some(Ok((outs, res)))
}

/// The engine's entry point (ax_engine::pack::NativeFn): None for a key that is not one of these procedures.
pub fn native(key: &str, v: &HashMap<String, Val>) -> Option<Out> {
    match key {
        "adc_conversion" => {
            let r = f_adc_conversion(match v.get("Vin") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Vref") { Some(x) => py::from_val(x), None => { return None } }, match v.get("bits") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_12(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "base_conversion" => {
            let r = f_base_conversion(match v.get("value") { Some(x) => py::from_val(x), None => { return None } }, match v.get("base") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_base(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bjt_divider_bias" => {
            let r = f_bjt_divider_bias(match v.get("VCC") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("RC") { Some(x) => py::from_val(x), None => { return None } }, match v.get("RE") { Some(x) => py::from_val(x), None => { return None } }, match v.get("beta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("VBE") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_bjt(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bode_point" => {
            let r = f_bode_point(match v.get("num") { Some(x) => py::from_val(x), None => { return None } }, match v.get("den") { Some(x) => py::from_val(x), None => { return None } }, match v.get("w") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_bode(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "boolean_equivalence" => {
            let r = f_boolean_equivalence(match v.get("expression1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("expression2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_equiv(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "boolean_evaluate" => {
            let r = f_boolean_evaluate(match v.get("expression") { Some(x) => py::from_val(x), None => { return None } }, match v.get("values") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_194(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "closed_loop_poles" => {
            let r = f_closed_loop_poles(match v.get("num") { Some(x) => py::from_val(x), None => { return None } }, match v.get("den") { Some(x) => py::from_val(x), None => { return None } }, match v.get("K") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_poles(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "complex_power" => {
            let r = f_complex_power(match v.get("V") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V_phase") { Some(x) => py::from_val(x), None => { return None } }, match v.get("I") { Some(x) => py::from_val(x), None => { return None } }, match v.get("I_phase") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_cpower(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "controllability_observability" => {
            let r = f_controllability_observability(match v.get("A") { Some(x) => py::from_val(x), None => { return None } }, match v.get("B") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_co(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "current_divider" => {
            let r = f_current_divider(match v.get("I") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_356(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "dc_motor" => {
            let r = f_dc_motor(match v.get("V") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Kt") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Ke") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_366(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "dc_node_voltages" => {
            let r = f_dc_node_voltages(match v.get("netlist") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_nodes(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "delta_to_wye" => {
            let r = f_delta_to_wye(match v.get("Rab") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Rbc") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Rca") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_519(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "difference_amplifier" => {
            let r = f_difference_amplifier(match v.get("V1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R3") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R4") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_diffamp(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "diode_circuit" => {
            let r = f_diode_circuit(match v.get("Vs") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Is") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("VT") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_545(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "first_order_time_to_value" => {
            let r = f_first_order_time_to_value(match v.get("V0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Vf") { Some(x) => py::from_val(x), None => { return None } }, match v.get("tau") { Some(x) => py::from_val(x), None => { return None } }, match v.get("v") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_556(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "first_order_transient" => {
            let r = f_first_order_transient(match v.get("V0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Vf") { Some(x) => py::from_val(x), None => { return None } }, match v.get("tau") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_564(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "gain_margin" => {
            let r = f_gain_margin(match v.get("num") { Some(x) => py::from_val(x), None => { return None } }, match v.get("den") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_gm(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "gray_code" => {
            let r = f_gray_code(match v.get("value") { Some(x) => py::from_val(x), None => { return None } }, match v.get("direction") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_gray(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "inductor_energy" => {
            let r = f_inductor_energy(match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("I") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_680(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "inverting_amplifier" => {
            let r = s_681(match v.get("Rin") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Rf") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Vin") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_687(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "lead_compensator" => {
            let r = f__lead(match v.get("z") { Some(x) => py::from_val(x), None => { return None } }, match v.get("p") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_695(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "max_power_transfer" => {
            let r = f_max_power_transfer(match v.get("Vth") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Rth") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_701(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "mesh_currents" => {
            let r = s_702(match v.get("A") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_735(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "minterms_to_sop" => {
            let r = f_minterms_to_sop(match v.get("minterms") { Some(x) => py::from_val(x), None => { return None } }, match v.get("variables") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dont_cares") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_m2s(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "mosfet_saturation" => {
            let r = f_mosfet_saturation(match v.get("kn") { Some(x) => py::from_val(x), None => { return None } }, match v.get("WL") { Some(x) => py::from_val(x), None => { return None } }, match v.get("VGS") { Some(x) => py::from_val(x), None => { return None } }, match v.get("VTH") { Some(x) => py::from_val(x), None => { return None } }, match v.get("VDS") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_885(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "nodal_voltages" => {
            let r = s_886(match v.get("A") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_919(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "noninverting_amplifier" => {
            let r = s_931(match v.get("Rg") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Rf") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Vin") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_937(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "norton_equivalent" => {
            let r = f_norton_equivalent(match v.get("netlist") { Some(x) => py::from_val(x), None => { return None } }, match v.get("node_a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("node_b") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_norton(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "opamp_integrator" => {
            let r = f_opamp_integrator(match v.get("R") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Vin") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V0") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1083(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "parallel_capacitance" => {
            let r = s_1084({ let mut d = vec![]; for k in ["C", "unit"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1095(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "parallel_inductance" => {
            let r = s_1098({ let mut d = vec![]; for k in ["L", "unit"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1109(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "parallel_resistance" => {
            let r = s_1112({ let mut d = vec![]; for k in ["R"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1123(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "parallel_rlc_impedance" => {
            let r = f_parallel_rlc_impedance(match v.get("R") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C") { Some(x) => py::from_val(x), None => { return None } }, match v.get("f") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1133(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "parallel_rlc_resonance" => {
            let r = s_1135(match v.get("R") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C") { Some(x) => py::from_val(x), None => { return None } }, (|| -> R { Ok(Py::Bool(false)) })().ok()?);
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1142(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "phase_margin" => {
            let r = f_phase_margin(match v.get("num") { Some(x) => py::from_val(x), None => { return None } }, match v.get("den") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_pm(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "phasor_sum" => {
            let r = f_phasor_sum(match v.get("amplitudes") { Some(x) => py::from_val(x), None => { return None } }, match v.get("phases_deg") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_phasor_sum(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "polar_to_rectangular" => {
            let r = f_polar_to_rectangular(match v.get("magnitude") { Some(x) => py::from_val(x), None => { return None } }, match v.get("angle_deg") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_p2r(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "power_factor_correction" => {
            let r = f_power_factor_correction(match v.get("P") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V") { Some(x) => py::from_val(x), None => { return None } }, match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("pf_initial") { Some(x) => py::from_val(x), None => { return None } }, match v.get("pf_target") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_pfc(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "rc_filter" => {
            let r = f_rc_filter(match v.get("R") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C") { Some(x) => py::from_val(x), None => { return None } }, match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("filter") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1283(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "rectangular_to_polar" => {
            let r = f_rectangular_to_polar(match v.get("real") { Some(x) => py::from_val(x), None => { return None } }, match v.get("imag") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_r2p(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "resistor_network" => {
            let r = f_resistor_network(match v.get("network") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_network(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "rms_waveform" => {
            let r = f_rms_waveform(match v.get("shape") { Some(x) => py::from_val(x), None => { return None } }, match v.get("amplitude") { Some(x) => py::from_val(x), None => { return None } }, match v.get("offset") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1336(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "routh_hurwitz" => {
            let r = f_routh_hurwitz(match v.get("coefficients") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_routh(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "second_order_step" => {
            let r = f_second_order_step(match v.get("zeta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("wn") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_step2(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "series_capacitance" => {
            let r = s_1447({ let mut d = vec![]; for k in ["C", "unit"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1458(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "series_inductance" => {
            let r = s_1461({ let mut d = vec![]; for k in ["L", "unit"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1472(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "series_resistance" => {
            let r = s_1475({ let mut d = vec![]; for k in ["R"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1486(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "series_rlc_impedance" => {
            let r = f_series_rlc_impedance(match v.get("R") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C") { Some(x) => py::from_val(x), None => { return None } }, match v.get("f") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1499(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "series_rlc_resonance" => {
            let r = s_1501(match v.get("R") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C") { Some(x) => py::from_val(x), None => { return None } }, (|| -> R { Ok(Py::Bool(true)) })().ok()?);
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1508(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "series_rlc_step" => {
            let r = f_series_rlc_step(match v.get("R") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_rlc_step(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "state_space_to_tf" => {
            let r = f_state_space_to_tf(match v.get("A") { Some(x) => py::from_val(x), None => { return None } }, match v.get("B") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C") { Some(x) => py::from_val(x), None => { return None } }, match v.get("D") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_ss(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "steady_state_error" => {
            let r = f_steady_state_error(match v.get("num") { Some(x) => py::from_val(x), None => { return None } }, match v.get("den") { Some(x) => py::from_val(x), None => { return None } }, match v.get("input") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_sse(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "summing_amplifier" => {
            let r = f_summing_amplifier(match v.get("V") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Rf") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_summing(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "thevenin_equivalent" => {
            let r = f_thevenin_equivalent(match v.get("netlist") { Some(x) => py::from_val(x), None => { return None } }, match v.get("node_a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("node_b") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_thevenin(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "three_phase_load" => {
            let r = f_three_phase_load(match v.get("VL") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R") { Some(x) => py::from_val(x), None => { return None } }, match v.get("X") { Some(x) => py::from_val(x), None => { return None } }, match v.get("connection") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_3ph(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "transformer" => {
            let r = f_transformer(match v.get("Np") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Ns") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Vp") { Some(x) => py::from_val(x), None => { return None } }, match v.get("RL") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1902(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "truth_table_sop" => {
            let r = f_truth_table_sop(match v.get("expression") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_tt(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "twos_complement_decode" => {
            let r = f_twos_complement_decode(match v.get("pattern") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2051(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "twos_complement_encode" => {
            let r = f_twos_complement_encode(match v.get("value") { Some(x) => py::from_val(x), None => { return None } }, match v.get("bits") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_tc_enc(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "voltage_divider" => {
            let r = f_voltage_divider(match v.get("Vin") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2074(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "wheatstone_bridge" => {
            let r = f_wheatstone_bridge(match v.get("Vs") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R3") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R4") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2082(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "wye_to_delta" => {
            let r = f_wye_to_delta(match v.get("Ra") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Rb") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Rc") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2088(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "zener_regulator" => {
            let r = f_zener_regulator(match v.get("Vin") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Vz") { Some(x) => py::from_val(x), None => { return None } }, match v.get("RL") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_zener(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        _ => None,
    }
}

#[allow(non_snake_case)]
fn c__BASES() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::dict(vec![(Py::str("2"), Py::Int(2i64)), (Py::str("8"), Py::Int(8i64)), (Py::str("10"), Py::Int(10i64)), (Py::str("16"), Py::Int(16i64))])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__BOOL_RE() -> R {
    thread_local!(static V: R = (|| -> R { Ok(ax_engine::pyre::module_call("compile", &Py::str("\\s*(?:(AND|OR|NOT|XOR)(?![A-Za-z0-9_])|([A-Za-z_][A-Za-z0-9_]*)|([01])(?![\\w.])|(&&|\\|\\||[&|^~!¬·*+()'⊕]))"), &[], (Py::Int(2i64)).i()?)?) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__INPUT_ORDER() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::dict(vec![(Py::str("step"), Py::Int(0i64)), (Py::str("ramp"), Py::Int(1i64)), (Py::str("parabola"), Py::Int(2i64))])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__SHAPES() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::dict(vec![(Py::str("sine"), Py::Float(0.5)), (Py::str("square"), Py::Float(1.0)), (Py::str("triangle"), Py::Float(0.3333333333333333)), (Py::str("sawtooth"), Py::Float(0.3333333333333333))])) })());
    V.with(|v| v.clone())
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__G(mut v_num: Py, mut v_den: Py, mut v_w: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_s: Py = Py::None;
    v_s = py::mul(&Py::Complex(0.0, 1.0), &v_w.clone())?;
    v_d = f__pval(v_den.clone(), v_s.clone())?;
    if ({ let t1232 = v_d.clone(); Py::Bool({ let t1233 = Py::Int(0i64); if py::eq(&t1232, &t1233) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("G(s) has a pole on the imaginary axis at this frequency"))));
    }
    return Ok(py::truediv(&f__pval(v_num.clone(), v_s.clone())?, &v_d.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__assignments(mut v_text: Py) -> R {
    let mut v_name: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_out = Py::dict(vec![]);
    for t170 in py::iter(&(ax_engine::pyre::module_call("findall", &Py::str("([A-Za-z_][A-Za-z0-9_]*)\\s*=\\s*([^,;\\s]+)"), &[v_text.clone()], 0)?))? {
        let t173 = (t170).items()?;
        if t173.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_name = t173[0].clone();
        v_v = t173[1].clone();
        if ({ let t171 = v_v.clone(); Py::Bool({ let t172 = Py::tuple(vec![Py::str("0"), Py::str("1")]); if !py::contains(&t172, &t171)? { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, " must be 0 or 1".to_string()].concat()))));
        }
        py::setitem(&v_out.clone(), &v_name.clone(), py::int(&v_v.clone())?)?;
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__bisect(mut v_f: Py, mut v_a: Py, mut v_b: Py) -> R {
    let mut v__: Py = Py::None;
    let mut v_fa: Py = Py::None;
    let mut v_fm: Py = Py::None;
    let mut v_m: Py = Py::None;
    v_fa = py::call(&v_f.clone(), &[v_a.clone()])?;
    for t1429 in py::range_iter(&[(Py::Int(300i64)).i()?]) {
        v__ = t1429;
        v_m = py::mul(&Py::Float(0.5), &py::add(&v_a.clone(), &v_b.clone())?)?;
        if ({ let t1430 = v_m.clone(); Py::Bool({ let t1431 = Py::tuple(vec![v_a.clone(), v_b.clone()]); if py::contains(&t1431, &t1430)? { true } else { false } }) }).truthy() {
            break;
        }
        v_fm = py::call(&v_f.clone(), &[v_m.clone()])?;
        if ({ let t1432 = { let t1434 = v_fm.clone(); Py::Bool({ let t1435 = Py::Int(0i64); if py::gt(&t1434, &t1435)? { true } else { false } }) }; Py::Bool({ let t1433 = { let t1436 = v_fa.clone(); Py::Bool({ let t1437 = Py::Int(0i64); if py::gt(&t1436, &t1437)? { true } else { false } }) }; if py::eq(&t1432, &t1433) { true } else { false } }) }).truthy() {
            let t1438 = (Py::tuple(vec![v_m.clone(), v_fm.clone()])).items()?;
            if t1438.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_a = t1438[0].clone();
            v_fa = t1438[1].clone();
        } else {
            v_b = v_m.clone();
        }
    }
    return Ok(py::mul(&Py::Float(0.5), &py::add(&v_a.clone(), &v_b.clone())?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__bool_eval(mut v_tree: Py, mut v_env: Py) -> R {
    let mut v_t: Py = Py::None;
    let mut v_vals: Py = Py::None;
    v_t = py::getitem(&v_tree.clone(), &Py::Int(0i64))?;
    if ({ let t1947 = v_t.clone(); Py::Bool({ let t1948 = Py::str("var"); if py::eq(&t1947, &t1948) { true } else { false } }) }).truthy() {
        return Ok(py::getitem(&v_env.clone(), &py::getitem(&v_tree.clone(), &Py::Int(1i64))?)?);
    }
    if ({ let t1949 = v_t.clone(); Py::Bool({ let t1950 = Py::str("const"); if py::eq(&t1949, &t1950) { true } else { false } }) }).truthy() {
        return Ok(py::getitem(&v_tree.clone(), &Py::Int(1i64))?);
    }
    if ({ let t1951 = v_t.clone(); Py::Bool({ let t1952 = Py::str("not"); if py::eq(&t1951, &t1952) { true } else { false } }) }).truthy() {
        return Ok(py::sub(&Py::Int(1i64), &f__bool_eval(py::getitem(&v_tree.clone(), &Py::Int(1i64))?, v_env.clone())?)?);
    }
    v_vals = { let mut out = vec![]; for t1953 in py::iter(&(py::getitem(&v_tree.clone(), &Py::Int(1i64))?))? { let mut v_x: Py; 
v_x = t1953;
out.push(f__bool_eval(v_x.clone(), v_env.clone())?); } Py::list(out) };
    if ({ let t1954 = v_t.clone(); Py::Bool({ let t1955 = Py::str("and"); if py::eq(&t1954, &t1955) { true } else { false } }) }).truthy() {
        return Ok(py::int(&Py::Bool((v_vals.clone()).items()?.iter().all(|x| x.truthy())))?);
    }
    if ({ let t1956 = v_t.clone(); Py::Bool({ let t1957 = Py::str("or"); if py::eq(&t1956, &t1957) { true } else { false } }) }).truthy() {
        return Ok(py::int(&Py::Bool((v_vals.clone()).items()?.iter().any(|x| x.truthy())))?);
    }
    return Ok(py::modulo(&py::sum(&v_vals.clone(), None)?, &Py::Int(2i64))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__bool_parse(mut v_text: Py) -> R {
    let mut v_andx: Py = Py::None;
    let mut v_binary: Py = Py::None;
    let mut v_peek: Py = Py::None;
    let mut v_pos: Py = Py::None;
    let mut v_take: Py = Py::None;
    let mut v_toks: Py = Py::None;
    let mut v_tree: Py = Py::None;
    let mut v_unary: Py = Py::None;
    let mut v_xorx: Py = Py::None;
    let c_orx = py::cell_new();
    v_toks = f__bool_tokens(v_text.clone())?;
    v_pos = Py::list(vec![Py::Int(0i64)]);
    v_peek = { let v_peek = v_peek.clone(); let v_pos = v_pos.clone(); let v_text = v_text.clone(); let v_toks = v_toks.clone(); let c_orx = c_orx.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            return Ok((if ({ let t1910 = py::getitem(&v_pos.clone(), &Py::Int(0i64))?; Py::Bool({ let t1911 = py::len(&v_toks.clone())?; if py::lt(&t1910, &t1911)? { true } else { false } }) }).truthy() { py::getitem(&v_toks.clone(), &py::getitem(&v_pos.clone(), &Py::Int(0i64))?)? } else { Py::None }));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_take = { let v_peek = v_peek.clone(); let v_pos = v_pos.clone(); let v_take = v_take.clone(); let v_text = v_text.clone(); let v_toks = v_toks.clone(); let c_orx = c_orx.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            if ({ let t1912 = py::getitem(&v_pos.clone(), &Py::Int(0i64))?; Py::Bool({ let t1913 = py::len(&v_toks.clone())?; if py::ge(&t1912, &t1913)? { true } else { false } }) }).truthy() {
                return Err(py::value_error(py::to_str(&Py::str("the Boolean expression ends too early"))));
            }
            let t1914 = v_pos.clone(); let t1915 = Py::Int(0i64);
            py::setitem(&t1914, &t1915, py::iadd(&py::getitem(&t1914, &t1915)?, &Py::Int(1i64))?)?;
            return Ok(py::getitem(&v_toks.clone(), &py::sub(&py::getitem(&v_pos.clone(), &Py::Int(0i64))?, &Py::Int(1i64))?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_binary = { let v_binary = v_binary.clone(); let v_peek = v_peek.clone(); let v_pos = v_pos.clone(); let v_take = v_take.clone(); let v_text = v_text.clone(); let v_toks = v_toks.clone(); let c_orx = c_orx.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_op: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_sub: Py = a.get(1).cloned().unwrap_or(Py::None);
            let mut v_f: Py = Py::None;
            v_f = { let v_binary = v_binary.clone(); let v_f = v_f.clone(); let v_op = v_op.clone(); let v_peek = v_peek.clone(); let v_pos = v_pos.clone(); let v_sub = v_sub.clone(); let v_take = v_take.clone(); let v_text = v_text.clone(); let v_toks = v_toks.clone(); let c_orx = c_orx.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_items: Py = Py::None;
            v_items = Py::list(vec![py::call(&v_sub.clone(), &[])?]);
            while ({ let t1916 = py::call(&v_peek.clone(), &[])?; Py::Bool({ let t1917 = v_op.clone(); if py::eq(&t1916, &t1917) { true } else { false } }) }).truthy() {
                let _ = py::call(&v_take.clone(), &[])?;
                let _ = py::method(&v_items.clone(), "append", &[py::call(&v_sub.clone(), &[])?])?;
            }
            return Ok((if ({ let t1918 = py::len(&v_items.clone())?; Py::Bool({ let t1919 = Py::Int(1i64); if py::eq(&t1918, &t1919) { true } else { false } }) }).truthy() { py::getitem(&v_items.clone(), &Py::Int(0i64))? } else { Py::tuple(vec![py::method(&v_op.clone(), "lower", &[])?, v_items.clone()]) }));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
            return Ok(v_f.clone());
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    let rec1920 = py::rec_cell();
    v_unary = { let v_binary = v_binary.clone(); let v_peek = v_peek.clone(); let v_pos = v_pos.clone(); let v_take = v_take.clone(); let v_text = v_text.clone(); let v_toks = v_toks.clone(); let c_orx = c_orx.clone(); let rec1920 = rec1920.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let v_unary: Py = py::rec_get(&rec1920)?;
            let mut v_node: Py = Py::None;
            let mut v_t: Py = Py::None;
            if ({ let t1921 = py::call(&v_peek.clone(), &[])?; Py::Bool({ let t1922 = Py::str("NOT"); if py::eq(&t1921, &t1922) { true } else { false } }) }).truthy() {
                let _ = py::call(&v_take.clone(), &[])?;
                return Ok(Py::tuple(vec![Py::str("not"), py::call(&v_unary.clone(), &[])?]));
            }
            v_t = py::call(&v_take.clone(), &[])?;
            if ({ let t1923 = v_t.clone(); Py::Bool({ let t1924 = Py::str("("); if py::eq(&t1923, &t1924) { true } else { false } }) }).truthy() {
                v_node = py::call(&py::cell_get(&c_orx)?, &[])?;
                if ({ let t1925 = py::call(&v_take.clone(), &[])?; Py::Bool({ let t1926 = Py::str(")"); if py::ne(&t1925, &t1926) { true } else { false } }) }).truthy() {
                    return Err(py::value_error(py::to_str(&Py::str("unbalanced parentheses"))));
                }
            } else {
                if (Py::Bool(matches!(v_t.clone(), Py::Tuple(_)))).truthy() {
                    v_node = (if ({ let t1927 = py::getitem(&v_t.clone(), &Py::Int(0i64))?; Py::Bool({ let t1928 = Py::str("v"); if py::eq(&t1927, &t1928) { true } else { false } }) }).truthy() { Py::tuple(vec![Py::str("var"), py::getitem(&v_t.clone(), &Py::Int(1i64))?]) } else { Py::tuple(vec![Py::str("const"), py::getitem(&v_t.clone(), &Py::Int(1i64))?]) });
                } else {
                    return Err(py::value_error(py::to_str(&Py::str(&["unexpected ".to_string(), py::format(&Py::str(&py::repr(&v_t.clone())), "")?, " in the Boolean expression".to_string()].concat()))));
                }
            }
            while ({ let t1929 = py::call(&v_peek.clone(), &[])?; Py::Bool({ let t1930 = Py::str("'"); if py::eq(&t1929, &t1930) { true } else { false } }) }).truthy() {
                let _ = py::call(&v_take.clone(), &[])?;
                v_node = Py::tuple(vec![Py::str("not"), v_node.clone()]);
            }
            return Ok(v_node.clone());
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    py::rec_set(&rec1920, &v_unary);
    v_andx = py::call(&v_binary.clone(), &[Py::str("AND"), v_unary.clone()])?;
    v_xorx = py::call(&v_binary.clone(), &[Py::str("XOR"), v_andx.clone()])?;
    py::cell_set(&c_orx, py::call(&v_binary.clone(), &[Py::str("OR"), v_xorx.clone()])?);
    v_tree = py::call(&py::cell_get(&c_orx)?, &[])?;
    if ({ let t1931 = py::getitem(&v_pos.clone(), &Py::Int(0i64))?; Py::Bool({ let t1932 = py::len(&v_toks.clone())?; if py::ne(&t1931, &t1932) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&["unexpected ".to_string(), py::format(&Py::str(&py::repr(&py::getitem(&v_toks.clone(), &py::getitem(&v_pos.clone(), &Py::Int(0i64))?)?)), "")?, " in the Boolean expression".to_string()].concat()))));
    }
    return Ok(v_tree.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__bool_tokens(mut v_text: Py) -> R {
    let mut v_const: Py = Py::None;
    let mut v_kw: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_op: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_pos: Py = Py::None;
    let mut v_var: Py = Py::None;
    let t1903 = (Py::tuple(vec![Py::Int(0i64), Py::list(vec![])])).items()?;
    if t1903.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_pos = t1903[0].clone();
    v_out = t1903[1].clone();
    v_text = py::method(&v_text.clone(), "strip", &[])?;
    while ({ let t1908 = v_pos.clone(); Py::Bool({ let t1909 = py::len(&v_text.clone())?; if py::lt(&t1908, &t1909)? { true } else { false } }) }).truthy() {
        v_m = py::method(&c__BOOL_RE()?, "match", &[v_text.clone(), v_pos.clone()])?;
        if ({ let t1906 = py::not(&v_m.clone()); if t1906.truthy() { t1906 } else { { let t1904 = py::method(&v_m.clone(), "end", &[])?; Py::Bool({ let t1905 = v_pos.clone(); if py::eq(&t1904, &t1905) { true } else { false } }) } } }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str(&["cannot read the Boolean expression at '".to_string(), py::format(&py::slice(&v_text.clone(), Some((v_pos.clone()).i()?), Some((py::add(&v_pos.clone(), &Py::Int(12i64))?).i()?), None)?, "")?, "'".to_string()].concat()))));
        }
        let t1907 = (py::method(&v_m.clone(), "groups", &[])?).items()?;
        if t1907.len() != 4 { return Err(PyErr::Value("unpack".into())); }
        v_kw = t1907[0].clone();
        v_var = t1907[1].clone();
        v_const = t1907[2].clone();
        v_op = t1907[3].clone();
        if (v_kw.clone()).truthy() {
            let _ = py::method(&v_out.clone(), "append", &[py::method(&v_kw.clone(), "upper", &[])?])?;
        } else {
            if (v_var.clone()).truthy() {
                let _ = py::method(&v_out.clone(), "append", &[Py::tuple(vec![Py::str("v"), v_var.clone()])])?;
            } else {
                if (v_const.clone()).truthy() {
                    let _ = py::method(&v_out.clone(), "append", &[Py::tuple(vec![Py::str("c"), py::int(&v_const.clone())?])])?;
                } else {
                    let _ = py::method(&v_out.clone(), "append", &[py::method(&Py::dict(vec![(Py::str("&&"), Py::str("AND")), (Py::str("&"), Py::str("AND")), (Py::str("·"), Py::str("AND")), (Py::str("*"), Py::str("AND")), (Py::str("||"), Py::str("OR")), (Py::str("|"), Py::str("OR")), (Py::str("+"), Py::str("OR")), (Py::str("^"), Py::str("XOR")), (Py::str("⊕"), Py::str("XOR")), (Py::str("~"), Py::str("NOT")), (Py::str("!"), Py::str("NOT")), (Py::str("¬"), Py::str("NOT"))]), "get", &[v_op.clone(), v_op.clone()])?])?;
                }
            }
        }
        v_pos = py::method(&v_m.clone(), "end", &[])?;
    }
    if (py::not(&v_out.clone())).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the expression is empty"))));
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__bool_vars(mut v_tree: Py, mut v_acc: Py) -> R {
    let mut v_x: Py = Py::None;
    v_acc = (if ({ let t1933 = v_acc.clone(); Py::Bool({ let t1934 = Py::None; if py::is(&t1933, &t1934) { true } else { false } }) }).truthy() { py::set(&Py::list(vec![]))? } else { v_acc.clone() });
    if ({ let t1935 = py::getitem(&v_tree.clone(), &Py::Int(0i64))?; Py::Bool({ let t1936 = Py::str("var"); if py::eq(&t1935, &t1936) { true } else { false } }) }).truthy() {
        let _ = py::method(&v_acc.clone(), "add", &[py::getitem(&v_tree.clone(), &Py::Int(1i64))?])?;
    } else {
        if ({ let t1937 = py::getitem(&v_tree.clone(), &Py::Int(0i64))?; Py::Bool({ let t1938 = Py::str("not"); if py::eq(&t1937, &t1938) { true } else { false } }) }).truthy() {
            let _ = f__bool_vars(py::getitem(&v_tree.clone(), &Py::Int(1i64))?, v_acc.clone())?;
        } else {
            if ({ let t1939 = py::getitem(&v_tree.clone(), &Py::Int(0i64))?; Py::Bool({ let t1940 = Py::tuple(vec![Py::str("and"), Py::str("or"), Py::str("xor")]); if py::contains(&t1940, &t1939)? { true } else { false } }) }).truthy() {
                for t1941 in py::iter(&(py::getitem(&v_tree.clone(), &Py::Int(1i64))?))? {
                    v_x = t1941;
                    let _ = f__bool_vars(v_x.clone(), v_acc.clone())?;
                }
            }
        }
    }
    return Ok(v_acc.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__canonical_roots(mut v_roots: Py, mut v_tol: Py) -> R {
    let mut v_cplx: Py = Py::None;
    let mut v_dn: Py = Py::None;
    let mut v_downs: Py = Py::None;
    let mut v_im: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_re_: Py = Py::None;
    let mut v_real: Py = Py::None;
    let mut v_u: Py = Py::None;
    let mut v_ups: Py = Py::None;
    let t218 = (Py::tuple(vec![Py::list(vec![]), Py::list(vec![])])).items()?;
    if t218.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_real = t218[0].clone();
    v_cplx = t218[1].clone();
    for t219 in py::iter(&(v_roots.clone()))? {
        v_r = t219;
        v_r = py::complex(&[v_r.clone()])?;
        let _ = py::method(&(if ({ let t220 = py::abs(&py::imag(&v_r.clone())?)?; Py::Bool({ let t221 = py::mul(&v_tol.clone(), &py::minmax(&[Py::Float(1.0), py::abs(&v_r.clone())?], true)?)?; if py::le(&t220, &t221)? { true } else { false } }) }).truthy() { v_real.clone() } else { v_cplx.clone() }), "append", &[v_r.clone()])?;
    }
    v_out = { let mut out = vec![]; for t222 in py::iter(&(v_real.clone()))? { let mut v_x: Py; 
v_x = t222;
out.push(py::complex(&[py::real(&v_x.clone())?, Py::Float(0.0)])?); } Py::list(out) };
    v_ups = py::sorted_key(&{ let mut out = vec![]; for t223 in py::iter(&(v_cplx.clone()))? { let mut v_x: Py; 
v_x = t223;
if ({ let t224 = py::imag(&v_x.clone())?; Py::Bool({ let t225 = Py::Int(0i64); if py::gt(&t224, &t225)? { true } else { false } }) }).truthy() { out.push(v_x.clone()); }} Py::list(out) }, &{ let v_cplx = v_cplx.clone(); let v_out = v_out.clone(); let v_r = v_r.clone(); let v_real = v_real.clone(); let v_roots = v_roots.clone(); let v_tol = v_tol.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_x: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(Py::tuple(vec![py::real(&v_x.clone())?, py::imag(&v_x.clone())?]));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, (Py::Bool(false)).truthy())?;
    v_downs = { let mut out = vec![]; for t226 in py::iter(&(v_cplx.clone()))? { let mut v_x: Py; 
v_x = t226;
if ({ let t227 = py::imag(&v_x.clone())?; Py::Bool({ let t228 = Py::Int(0i64); if py::lt(&t227, &t228)? { true } else { false } }) }).truthy() { out.push(v_x.clone()); }} Py::list(out) };
    for t229 in py::iter(&(v_ups.clone()))? {
        v_u = t229;
        if (py::not(&v_downs.clone())).truthy() {
            break;
        }
        v_j = py::minmax_key(&[Py::list((py::range_iter(&[(py::len(&v_downs.clone())?).i()?])).into_iter().collect::<Vec<Py>>())], &{ let v_cplx = v_cplx.clone(); let v_downs = v_downs.clone(); let v_out = v_out.clone(); let v_r = v_r.clone(); let v_real = v_real.clone(); let v_roots = v_roots.clone(); let v_tol = v_tol.clone(); let v_u = v_u.clone(); let v_ups = v_ups.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_k: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::abs(&py::sub(&py::getitem(&v_downs.clone(), &v_k.clone())?, &py::method(&v_u.clone(), "conjugate", &[])?)?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, false)?;
        v_dn = py::method(&v_downs.clone(), "pop", &[v_j.clone()])?;
        let t230 = (Py::tuple(vec![py::truediv(&py::add(&py::real(&v_u.clone())?, &py::real(&v_dn.clone())?)?, &Py::Int(2i64))?, py::truediv(&py::sub(&py::imag(&v_u.clone())?, &py::imag(&v_dn.clone())?)?, &Py::Int(2i64))?])).items()?;
        if t230.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_re_ = t230[0].clone();
        v_im = t230[1].clone();
        v_out = py::iadd(&v_out, &Py::list(vec![py::complex(&[v_re_.clone(), py::neg(&v_im.clone())?])?, py::complex(&[v_re_.clone(), v_im.clone()])?]))?;
    }
    v_out = py::iadd(&v_out, &v_downs.clone())?;
    let _ = py::method_sort(&v_out.clone(), Some(&{ let v_cplx = v_cplx.clone(); let v_dn = v_dn.clone(); let v_downs = v_downs.clone(); let v_im = v_im.clone(); let v_j = v_j.clone(); let v_out = v_out.clone(); let v_r = v_r.clone(); let v_re_ = v_re_.clone(); let v_real = v_real.clone(); let v_roots = v_roots.clone(); let v_tol = v_tol.clone(); let v_u = v_u.clone(); let v_ups = v_ups.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_x: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(Py::tuple(vec![py::real(&v_x.clone())?, py::imag(&v_x.clone())?]));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }), false)?;
    return Ok({ let mut out = vec![]; for t231 in py::iter(&(v_out.clone()))? { let mut v_x: Py; 
v_x = t231;
out.push((if ({ let t232 = py::imag(&v_x.clone())?; Py::Bool({ let t233 = Py::Int(0i64); if py::eq(&t232, &t233) { true } else { false } }) }).truthy() { py::real(&v_x.clone())? } else { v_x.clone() })); } Py::list(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__canonical_sop(mut v_n: Py, mut v_names: Py, mut v_on: Py) -> R {
    return Ok({ let t1967 = py::method(&Py::str(" + "), "join", &[{ let mut out = vec![]; for t1964 in py::iter(&(v_on.clone()))? { let mut v_m: Py; 
v_m = t1964;
out.push(f__term_text(Py::tuple(vec![v_m.clone(), Py::Int(0i64)]), v_names.clone())?); } Py::list(out) }])?; if t1967.truthy() { t1967 } else { Py::str("0") } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__charpoly_adj(mut v_A: Py) -> R {
    let mut v_AM: Py = Py::None;
    let mut v_I: Py = Py::None;
    let mut v_M: Py = Py::None;
    let mut v_Ns: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_coeffs: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_n: Py = Py::None;
    v_n = py::len(&v_A.clone())?;
    v_I = { let mut out = vec![]; for t1548 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t1548;
out.push({ let mut out = vec![]; for t1549 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t1549;
out.push(py::float(&{ let t1550 = v_i.clone(); Py::Bool({ let t1551 = v_j.clone(); if py::eq(&t1550, &t1551) { true } else { false } }) })?); } Py::list(out) }); } Py::list(out) };
    v_M = { let mut out = vec![]; for t1552 in py::iter(&(v_I.clone()))? { let mut v_row: Py; 
v_row = t1552;
out.push(py::slice(&v_row.clone(), None, None, None)?); } Py::list(out) };
    let t1553 = (Py::tuple(vec![Py::list(vec![Py::Float(1.0)]), Py::list(vec![])])).items()?;
    if t1553.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_coeffs = t1553[0].clone();
    v_Ns = t1553[1].clone();
    for t1554 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&v_n.clone(), &Py::Int(1i64))?).i()?]) {
        v_k = t1554;
        let _ = py::method(&v_Ns.clone(), "append", &[v_M.clone()])?;
        v_AM = { let mut out = vec![]; for t1555 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t1555;
out.push({ let mut out = vec![]; for t1556 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t1556;
out.push(py::sum(&{ let mut out = vec![]; for t1557 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_t: Py; 
v_t = t1557;
out.push(py::mul(&py::getitem(&py::getitem(&v_A.clone(), &v_i.clone())?, &v_t.clone())?, &py::getitem(&py::getitem(&v_M.clone(), &v_t.clone())?, &v_j.clone())?)?); } Py::list(out) }, None)?); } Py::list(out) }); } Py::list(out) };
        v_c = py::truediv(&py::neg(&py::sum(&{ let mut out = vec![]; for t1558 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t1558;
out.push(py::getitem(&py::getitem(&v_AM.clone(), &v_i.clone())?, &v_i.clone())?); } Py::list(out) }, None)?)?, &v_k.clone())?;
        let _ = py::method(&v_coeffs.clone(), "append", &[v_c.clone()])?;
        v_M = { let mut out = vec![]; for t1559 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t1559;
out.push({ let mut out = vec![]; for t1560 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t1560;
out.push(py::add(&py::getitem(&py::getitem(&v_AM.clone(), &v_i.clone())?, &v_j.clone())?, &py::mul(&v_c.clone(), &py::getitem(&py::getitem(&v_I.clone(), &v_i.clone())?, &v_j.clone())?)?)?); } Py::list(out) }); } Py::list(out) };
    }
    return Ok(Py::tuple(vec![v_coeffs.clone(), v_Ns.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__check_positive(mut v_kw: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    for t2092 in py::iter(&(py::method(&v_kw.clone(), "items", &[])?))? {
        let t2096 = (t2092).items()?;
        if t2096.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2096[0].clone();
        v_v = t2096[1].clone();
        if (py::not(&{ let t2095 = py::math1("isfinite", &v_v.clone())?; if !t2095.truthy() { t2095 } else { { let t2093 = v_v.clone(); Py::Bool({ let t2094 = Py::Int(0i64); if py::gt(&t2093, &t2094)? { true } else { false } }) } } })).truthy() {
            return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_k.clone(), "")?, " must be positive".to_string()].concat()))));
        }
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__check_tf(mut v_num: Py, mut v_den: Py) -> R {
    let mut v_x: Py = Py::None;
    let t1619 = (Py::tuple(vec![f__ptrim(v_num.clone())?, f__ptrim(v_den.clone())?])).items()?;
    if t1619.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_num = t1619[0].clone();
    v_den = t1619[1].clone();
    if (Py::Bool(({ let mut out = vec![]; for t1620 in py::iter(&(v_den.clone()))? { let mut v_x: Py; 
v_x = t1620;
out.push({ let t1621 = v_x.clone(); Py::Bool({ let t1622 = Py::Int(0i64); if py::eq(&t1621, &t1622) { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy()))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the denominator must not be zero"))));
    }
    if (Py::Bool(({ let mut out = vec![]; for t1623 in py::iter(&(v_num.clone()))? { let mut v_x: Py; 
v_x = t1623;
out.push({ let t1624 = v_x.clone(); Py::Bool({ let t1625 = Py::Int(0i64); if py::eq(&t1624, &t1625) { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy()))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the numerator must not be zero"))));
    }
    if ({ let t1626 = py::len(&v_num.clone())?; Py::Bool({ let t1627 = py::len(&v_den.clone())?; if py::gt(&t1626, &t1627)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the transfer function must be proper (numerator degree <= denominator degree)"))));
    }
    for t1628 in py::iter(&(py::add(&v_num.clone(), &v_den.clone())?))? {
        v_x = t1628;
        if (py::not(&py::math1("isfinite", &v_x.clone())?)).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("coefficients must be finite"))));
        }
    }
    return Ok(Py::tuple(vec![v_num.clone(), v_den.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__clean_digits(mut v_value: Py, mut v_base: Py) -> R {
    let mut v_pre: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_v = py::method(&py::method(&py::method(&v_value.clone(), "strip", &[])?, "replace", &[Py::str("_"), Py::str("")])?, "replace", &[Py::str(" "), Py::str("")])?;
    v_pre = Py::dict(vec![(Py::str("0b"), Py::Int(2i64)), (Py::str("0o"), Py::Int(8i64)), (Py::str("0x"), Py::Int(16i64))]);
    if ({ let t23 = py::method(&py::slice(&v_v.clone(), None, Some((Py::Int(2i64)).i()?), None)?, "lower", &[])?; Py::Bool({ let t24 = v_pre.clone(); if py::contains(&t24, &t23)? { true } else { false } }) }).truthy() {
        if ({ let t25 = py::getitem(&v_pre.clone(), &py::method(&py::slice(&v_v.clone(), None, Some((Py::Int(2i64)).i()?), None)?, "lower", &[])?)?; Py::Bool({ let t26 = v_base.clone(); if py::ne(&t25, &t26) { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str(&["the prefix ".to_string(), py::format(&py::slice(&v_v.clone(), None, Some((Py::Int(2i64)).i()?), None)?, "")?, " does not match base ".to_string(), py::format(&v_base.clone(), "")?].concat()))));
        }
        v_v = py::slice(&v_v.clone(), Some((Py::Int(2i64)).i()?), None, None)?;
    }
    if ({ let t30 = py::not(&v_v.clone()); if t30.truthy() { t30 } else { Py::Bool(({ let mut out = vec![]; for t27 in py::iter(&(py::method(&v_v.clone(), "lower", &[])?))? { let mut v_c: Py; 
v_c = t27;
out.push({ let t28 = v_c.clone(); Py::Bool({ let t29 = py::slice(&Py::str("0123456789abcdef"), None, Some((v_base.clone()).i()?), None)?; if !py::contains(&t29, &t28)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())) } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&["'".to_string(), py::format(&v_value.clone(), "")?, "' is not a whole number in base ".to_string(), py::format(&v_base.clone(), "")?].concat()))));
    }
    return Ok(py::method(&v_v.clone(), "lower", &[])?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__covers(mut v_imp: Py, mut v_m: Py) -> R {
    let mut v_mask: Py = Py::None;
    let mut v_v: Py = Py::None;
    let t1984 = (v_imp.clone()).items()?;
    if t1984.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_v = t1984[0].clone();
    v_mask = t1984[1].clone();
    return Ok({ let t1985 = py::bitand(&v_m.clone(), &py::invert(&v_mask.clone())?)?; Py::Bool({ let t1986 = v_v.clone(); if py::eq(&t1985, &t1986) { true } else { false } }) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__ctrb_obsv(mut v_A: Py, mut v_B: Py, mut v_C: Py) -> R {
    let mut v__: Py = Py::None;
    let mut v_cols: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_rows: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_w: Py = Py::None;
    v_n = py::len(&v_A.clone())?;
    let t306 = (Py::tuple(vec![Py::list(vec![]), Py::list((v_B.clone()).items()?)])).items()?;
    if t306.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_cols = t306[0].clone();
    v_v = t306[1].clone();
    for t307 in py::range_iter(&[(v_n.clone()).i()?]) {
        v__ = t307;
        let _ = py::method(&v_cols.clone(), "append", &[v_v.clone()])?;
        v_v = { let mut out = vec![]; for t308 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t308;
out.push(py::sum(&{ let mut out = vec![]; for t309 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t309;
out.push(py::mul(&py::getitem(&py::getitem(&v_A.clone(), &v_i.clone())?, &v_j.clone())?, &py::getitem(&v_v.clone(), &v_j.clone())?)?); } Py::list(out) }, None)?); } Py::list(out) };
    }
    let t310 = (Py::tuple(vec![Py::list(vec![]), Py::list((v_C.clone()).items()?)])).items()?;
    if t310.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_rows = t310[0].clone();
    v_w = t310[1].clone();
    for t311 in py::range_iter(&[(v_n.clone()).i()?]) {
        v__ = t311;
        let _ = py::method(&v_rows.clone(), "append", &[v_w.clone()])?;
        v_w = { let mut out = vec![]; for t312 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t312;
out.push(py::sum(&{ let mut out = vec![]; for t313 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t313;
out.push(py::mul(&py::getitem(&v_w.clone(), &v_i.clone())?, &py::getitem(&py::getitem(&v_A.clone(), &v_i.clone())?, &v_j.clone())?)?); } Py::list(out) }, None)?); } Py::list(out) };
    }
    return Ok(Py::tuple(vec![{ let mut out = vec![]; for t314 in py::iter(&(py::zip(&{ let mut v: Vec<Py> = vec![]; v.extend((v_cols.clone()).items()?); v })?))? { let mut v_x: Py; 
v_x = t314;
out.push(Py::list((v_x.clone()).items()?)); } Py::list(out) }, v_rows.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__gain_crossover(mut v_num: Py, mut v_den: Py) -> R {
    let mut v_P: Py = Py::None;
    let mut v_d_iw: Py = Py::None;
    let mut v_dd: Py = Py::None;
    let mut v_n_iw: Py = Py::None;
    let mut v_nn: Py = Py::None;
    let mut v_ws: Py = Py::None;
    let t1163 = (Py::tuple(vec![f__iw(v_num.clone())?, f__iw(v_den.clone())?])).items()?;
    if t1163.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_n_iw = t1163[0].clone();
    v_d_iw = t1163[1].clone();
    v_nn = { let mut out = vec![]; for t1171 in py::iter(&(f__pmul(v_n_iw.clone(), { let mut out = vec![]; for t1170 in py::iter(&(v_n_iw.clone()))? { let mut v_y: Py; 
v_y = t1170;
out.push(py::method(&v_y.clone(), "conjugate", &[])?); } Py::list(out) })?))? { let mut v_x: Py; 
v_x = t1171;
out.push(py::real(&v_x.clone())?); } Py::list(out) };
    v_dd = { let mut out = vec![]; for t1173 in py::iter(&(f__pmul(v_d_iw.clone(), { let mut out = vec![]; for t1172 in py::iter(&(v_d_iw.clone()))? { let mut v_y: Py; 
v_y = t1172;
out.push(py::method(&v_y.clone(), "conjugate", &[])?); } Py::list(out) })?))? { let mut v_x: Py; 
v_x = t1173;
out.push(py::real(&v_x.clone())?); } Py::list(out) };
    v_P = f__ptrim(f__padd(v_nn.clone(), { let mut out = vec![]; for t1177 in py::iter(&(v_dd.clone()))? { let mut v_x: Py; 
v_x = t1177;
out.push(py::neg(&v_x.clone())?); } Py::list(out) })?)?;
    v_ws = f__positive_real_roots(v_P.clone(), Py::Float(1.0))?;
    return Ok({ let mut out = vec![]; for t1224 in py::iter(&(v_ws.clone()))? { let mut v_w: Py; 
v_w = t1224;
out.push(f__polish({ let v_P = v_P.clone(); let v_d_iw = v_d_iw.clone(); let v_dd = v_dd.clone(); let v_den = v_den.clone(); let v_n_iw = v_n_iw.clone(); let v_nn = v_nn.clone(); let v_num = v_num.clone(); let v_ws = v_ws.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_w: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::sub(&py::abs(&f__G(v_num.clone(), v_den.clone(), v_w.clone())?)?, &Py::Float(1.0))?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, v_w.clone())?); } Py::list(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__gauss(mut v_A: Py, mut v_b: Py) -> R {
    let mut v_M: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_scale: Py = Py::None;
    let mut v_x: Py = Py::None;
    v_n = py::len(&v_A.clone())?;
    if ({ let t1830 = { let t1822 = v_n.clone(); Py::Bool({ let t1823 = Py::Int(0i64); if py::eq(&t1822, &t1823) { true } else { false } }) }; if t1830.truthy() { t1830 } else { { let t1829 = Py::Bool(({ let mut out = vec![]; for t1824 in py::iter(&(v_A.clone()))? { let mut v_r: Py; 
v_r = t1824;
out.push({ let t1825 = py::len(&v_r.clone())?; Py::Bool({ let t1826 = v_n.clone(); if py::ne(&t1825, &t1826) { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())); if t1829.truthy() { t1829 } else { { let t1827 = py::len(&v_b.clone())?; Py::Bool({ let t1828 = v_n.clone(); if py::ne(&t1827, &t1828) { true } else { false } }) } } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the matrix must be square and the vector must have one entry per row"))));
    }
    v_M = { let mut out = vec![]; for t1831 in py::iter(&(py::enumerate(&v_A.clone(), 0)?))? { let mut v_i: Py; let mut v_row: Py; 
let t1832 = (t1831).items()?;
if t1832.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_i = t1832[0].clone();
v_row = t1832[1].clone();
out.push(py::add(&{ let mut out = vec![]; for t1833 in py::iter(&(v_row.clone()))? { let mut v_x: Py; 
v_x = t1833;
out.push((if (Py::Bool(matches!(v_x.clone(), Py::Complex(..)))).truthy() { py::complex(&[v_x.clone()])? } else { py::float(&v_x.clone())? })); } Py::list(out) }, &Py::list(vec![py::getitem(&v_b.clone(), &v_i.clone())?]))?); } Py::list(out) };
    v_scale = { let t1836 = py::minmax(&[{ let mut out = vec![]; for t1834 in py::iter(&(v_A.clone()))? { let mut v_row: Py; 
v_row = t1834;
for t1835 in py::iter(&(v_row.clone()))? { let mut v_x: Py; 
v_x = t1835;
out.push(py::abs(&v_x.clone())?); }} Py::list(out) }], true)?; if t1836.truthy() { t1836 } else { Py::Float(1.0) } };
    for t1837 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_c = t1837;
        v_p = py::minmax_key(&[Py::list((py::range_iter(&[(v_c.clone()).i()?, (v_n.clone()).i()?])).into_iter().collect::<Vec<Py>>())], &{ let v_A = v_A.clone(); let v_M = v_M.clone(); let v_b = v_b.clone(); let v_c = v_c.clone(); let v_n = v_n.clone(); let v_scale = v_scale.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_r: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::abs(&py::getitem(&py::getitem(&v_M.clone(), &v_r.clone())?, &v_c.clone())?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, true)?;
        if ({ let t1838 = py::abs(&py::getitem(&py::getitem(&v_M.clone(), &v_p.clone())?, &v_c.clone())?)?; Py::Bool({ let t1839 = py::mul(&Py::Float(1e-12), &v_scale.clone())?; if py::le(&t1838, &t1839)? { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("the system is singular: the equations do not determine every unknown"))));
        }
        let t1840 = (Py::tuple(vec![py::getitem(&v_M.clone(), &v_p.clone())?, py::getitem(&v_M.clone(), &v_c.clone())?])).items()?;
        if t1840.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        py::setitem(&v_M.clone(), &v_c.clone(), t1840[0].clone())?;
        py::setitem(&v_M.clone(), &v_p.clone(), t1840[1].clone())?;
        for t1841 in py::range_iter(&[(py::add(&v_c.clone(), &Py::Int(1i64))?).i()?, (v_n.clone()).i()?]) {
            v_r = t1841;
            v_f = py::truediv(&py::getitem(&py::getitem(&v_M.clone(), &v_r.clone())?, &v_c.clone())?, &py::getitem(&py::getitem(&v_M.clone(), &v_c.clone())?, &v_c.clone())?)?;
            if ({ let t1842 = v_f.clone(); Py::Bool({ let t1843 = Py::Int(0i64); if py::ne(&t1842, &t1843) { true } else { false } }) }).truthy() {
                for t1844 in py::range_iter(&[(v_c.clone()).i()?, (py::add(&v_n.clone(), &Py::Int(1i64))?).i()?]) {
                    v_k = t1844;
                    let t1845 = py::getitem(&v_M.clone(), &v_r.clone())?; let t1846 = v_k.clone();
                    py::setitem(&t1845, &t1846, py::sub(&py::getitem(&t1845, &t1846)?, &py::mul(&v_f.clone(), &py::getitem(&py::getitem(&v_M.clone(), &v_c.clone())?, &v_k.clone())?)?)?)?;
                }
            }
        }
    }
    v_x = py::mul(&Py::list(vec![Py::Float(0.0)]), &v_n.clone())?;
    for t1847 in py::iter(&(py::reversed(&Py::list((py::range_iter(&[(v_n.clone()).i()?])).into_iter().collect::<Vec<Py>>()))?))? {
        v_r = t1847;
        py::setitem(&v_x.clone(), &v_r.clone(), py::truediv(&py::sub(&py::getitem(&py::getitem(&v_M.clone(), &v_r.clone())?, &v_n.clone())?, &py::sum(&{ let mut out = vec![]; for t1848 in py::range_iter(&[(py::add(&v_r.clone(), &Py::Int(1i64))?).i()?, (v_n.clone()).i()?]) { let mut v_k: Py; 
v_k = t1848;
out.push(py::mul(&py::getitem(&py::getitem(&v_M.clone(), &v_r.clone())?, &v_k.clone())?, &py::getitem(&v_x.clone(), &v_k.clone())?)?); } Py::list(out) }, None)?)?, &py::getitem(&py::getitem(&v_M.clone(), &v_r.clone())?, &v_r.clone())?)?)?;
    }
    return Ok(v_x.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__impedance_out(mut v_Z: Py, mut v_w: Py, mut v_L: Py, mut v_C: Py) -> R {
    return Ok(Py::dict(vec![(Py::str("impedance"), py::abs(&v_Z.clone())?), (Py::str("phase_deg"), py::add(&py::math1("degrees", &py::cmath("phase", &[py::complex(&[py::add(&py::real(&v_Z.clone())?, &Py::Float(0.0))?, py::add(&py::imag(&v_Z.clone())?, &Py::Float(0.0))?])?])?)?, &Py::Float(0.0))?), (Py::str("real"), py::add(&py::real(&v_Z.clone())?, &Py::Float(0.0))?), (Py::str("imag"), py::add(&py::imag(&v_Z.clone())?, &Py::Float(0.0))?), (Py::str("XL"), py::mul(&v_w.clone(), &v_L.clone())?), (Py::str("XC"), py::truediv(&Py::Float(1.0), &py::mul(&v_w.clone(), &v_C.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__iw(mut v_c: Py) -> R {
    let mut v_n: Py = Py::None;
    v_n = py::sub(&py::len(&v_c.clone())?, &Py::Int(1i64))?;
    return Ok({ let mut out = vec![]; for t1161 in py::iter(&(py::enumerate(&v_c.clone(), 0)?))? { let mut v_i: Py; let mut v_x: Py; 
let t1162 = (t1161).items()?;
if t1162.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_i = t1162[0].clone();
v_x = t1162[1].clone();
out.push(py::mul(&v_x.clone(), &py::pow(&Py::Complex(0.0, 1.0), &py::sub(&v_n.clone(), &v_i.clone())?)?)?); } Py::list(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__lead(mut v_z: Py, mut v_p: Py) -> R {
    let mut v_phi: Py = Py::None;
    let mut v_wm: Py = Py::None;
    let _ = f__check_positive(Py::dict(vec![(Py::str("z"), v_z.clone()), (Py::str("p"), v_p.clone())]))?;
    if (py::not(&{ let t693 = v_p.clone(); Py::Bool({ let t694 = v_z.clone(); if py::gt(&t693, &t694)? { true } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a lead compensator has its zero below its pole (p > z)"))));
    }
    v_wm = py::math1("sqrt", &py::mul(&v_z.clone(), &v_p.clone())?)?;
    v_phi = py::math1("degrees", &py::math1("asin", &py::truediv(&py::sub(&v_p.clone(), &v_z.clone())?, &py::add(&v_p.clone(), &v_z.clone())?)?)?)?;
    return Ok(Py::dict(vec![(Py::str("phase_max_deg"), v_phi.clone()), (Py::str("w_max"), v_wm.clone()), (Py::str("gain_at_w_max_db"), py::mul(&Py::Int(10i64), &py::math1("log10", &py::truediv(&v_z.clone(), &v_p.clone())?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__literals(mut v_imp: Py, mut v_n: Py) -> R {
    return Ok(py::sub(&v_n.clone(), &py::method(&py::int_text_base(&py::getitem(&v_imp.clone(), &Py::Int(1i64))?, "bin")?, "count", &[Py::str("1")])?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__min_cover(mut v_n: Py, mut v_on: Py, mut v_dc: Py) -> R {
    let mut v_best: Py = Py::None;
    let mut v_chosen: Py = Py::None;
    let mut v_combo: Py = Py::None;
    let mut v_cov: Py = Py::None;
    let mut v_essential: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_left: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_primes: Py = Py::None;
    let mut v_rest: Py = Py::None;
    let t1968 = (Py::tuple(vec![py::sorted(&py::set(&v_on.clone())?, (Py::Bool(false)).truthy())?, py::sorted(&py::sub(&py::set(&v_dc.clone())?, &py::set(&v_on.clone())?)?, (Py::Bool(false)).truthy())?])).items()?;
    if t1968.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_on = t1968[0].clone();
    v_dc = t1968[1].clone();
    if (py::not(&v_on.clone())).truthy() {
        return Ok(Py::list(vec![]));
    }
    v_primes = f__prime_implicants(v_n.clone(), py::bitor(&py::set(&v_on.clone())?, &py::set(&v_dc.clone())?)?)?;
    v_essential = Py::list(vec![]);
    for t1982 in py::iter(&(v_on.clone()))? {
        v_m = t1982;
        v_cov = { let mut out = vec![]; for t1983 in py::iter(&(v_primes.clone()))? { let mut v_p: Py; 
v_p = t1983;
if (f__covers(v_p.clone(), v_m.clone())?).truthy() { out.push(v_p.clone()); }} Py::list(out) };
        if ({ let t1991 = { let t1987 = py::len(&v_cov.clone())?; Py::Bool({ let t1988 = Py::Int(1i64); if py::eq(&t1987, &t1988) { true } else { false } }) }; if !t1991.truthy() { t1991 } else { { let t1989 = py::getitem(&v_cov.clone(), &Py::Int(0i64))?; Py::Bool({ let t1990 = v_essential.clone(); if !py::contains(&t1990, &t1989)? { true } else { false } }) } } }).truthy() {
            let _ = py::method(&v_essential.clone(), "append", &[py::getitem(&v_cov.clone(), &Py::Int(0i64))?])?;
        }
    }
    v_left = { let mut out = vec![]; for t1992 in py::iter(&(v_on.clone()))? { let mut v_m: Py; 
v_m = t1992;
if (py::not(&Py::Bool(({ let mut out = vec![]; for t1993 in py::iter(&(v_essential.clone()))? { let mut v_p: Py; 
v_p = t1993;
out.push(f__covers(v_p.clone(), v_m.clone())?); } Py::list(out) }).items()?.iter().any(|x| x.truthy())))).truthy() { out.push(v_m.clone()); }} Py::list(out) };
    v_rest = { let mut out = vec![]; for t1994 in py::iter(&(v_primes.clone()))? { let mut v_p: Py; 
v_p = t1994;
if ({ let t1995 = v_p.clone(); Py::Bool({ let t1996 = v_essential.clone(); if !py::contains(&t1996, &t1995)? { true } else { false } }) }).truthy() { out.push(v_p.clone()); }} Py::list(out) };
    v_best = Py::None;
    if (v_left.clone()).truthy() {
        for t1997 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&py::len(&v_rest.clone())?, &Py::Int(1i64))?).i()?]) {
            v_k = t1997;
            for t1998 in py::iter(&(py::itertools("combinations", &[v_rest.clone(), v_k.clone()])?))? {
                v_combo = t1998;
                if (Py::Bool(({ let mut out = vec![]; for t1999 in py::iter(&(v_left.clone()))? { let mut v_m: Py; 
v_m = t1999;
out.push(Py::Bool(({ let mut out = vec![]; for t2000 in py::iter(&(v_combo.clone()))? { let mut v_p: Py; 
v_p = t2000;
out.push(f__covers(v_p.clone(), v_m.clone())?); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))); } Py::list(out) }).items()?.iter().all(|x| x.truthy()))).truthy() {
                    v_key = Py::tuple(vec![v_k.clone(), py::sum(&{ let mut out = vec![]; for t2001 in py::iter(&(v_combo.clone()))? { let mut v_p: Py; 
v_p = t2001;
out.push(f__literals(v_p.clone(), v_n.clone())?); } Py::list(out) }, None)?, py::sorted(&v_combo.clone(), (Py::Bool(false)).truthy())?]);
                    if ({ let t2006 = { let t2002 = v_best.clone(); Py::Bool({ let t2003 = Py::None; if py::is(&t2002, &t2003) { true } else { false } }) }; if t2006.truthy() { t2006 } else { { let t2004 = v_key.clone(); Py::Bool({ let t2005 = v_best.clone(); if py::lt(&t2004, &t2005)? { true } else { false } }) } } }).truthy() {
                        v_best = v_key.clone();
                    }
                }
            }
            if ({ let t2007 = v_best.clone(); Py::Bool({ let t2008 = Py::None; if !py::is(&t2007, &t2008) { true } else { false } }) }).truthy() {
                break;
            }
        }
    }
    v_chosen = py::add(&v_essential.clone(), &(if (v_best.clone()).truthy() { Py::list((py::getitem(&v_best.clone(), &Py::Int(2i64))?).items()?) } else { Py::list(vec![]) }))?;
    return Ok(v_chosen.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__mna(mut v_els: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_N: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_cur: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_ia: Py = Py::None;
    let mut v_ib: Py = Py::None;
    let mut v_idx: Py = Py::None;
    let mut v_imax: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_name: Py = Py::None;
    let mut v_nodes: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_snap: Py = Py::None;
    let mut v_t: Py = Py::None;
    let mut v_val: Py = Py::None;
    let mut v_vmax: Py = Py::None;
    let mut v_volts: Py = Py::None;
    let mut v_vs: Py = Py::None;
    let mut v_x: Py = Py::None;
    let mut v_z: Py = Py::None;
    v_nodes = py::sorted_key(&py::set(&{ let mut out = vec![]; for t1772 in py::iter(&(v_els.clone()))? { let mut v_e: Py; 
v_e = t1772;
for t1773 in py::iter(&(Py::tuple(vec![py::getitem(&v_e.clone(), &Py::Int(2i64))?, py::getitem(&v_e.clone(), &Py::Int(3i64))?])))? { let mut v_n: Py; 
v_n = t1773;
if ({ let t1774 = v_n.clone(); Py::Bool({ let t1775 = Py::str("0"); if py::ne(&t1774, &t1775) { true } else { false } }) }).truthy() { out.push(v_n.clone()); }}} Py::list(out) })?, &py::func(|a: &[Py]| -> R { f__node_key(a.get(0).cloned().unwrap_or(Py::None)) }), (Py::Bool(false)).truthy())?;
    v_idx = { let mut out = vec![]; for t1776 in py::iter(&(py::enumerate(&v_nodes.clone(), 0)?))? { let mut v_i: Py; let mut v_n: Py; 
let t1777 = (t1776).items()?;
if t1777.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_i = t1777[0].clone();
v_n = t1777[1].clone();
out.push((v_n.clone(), v_i.clone())); } Py::dict(out) };
    v_vs = { let mut out = vec![]; for t1778 in py::iter(&(v_els.clone()))? { let mut v_e: Py; 
v_e = t1778;
if ({ let t1779 = py::getitem(&v_e.clone(), &Py::Int(0i64))?; Py::Bool({ let t1780 = Py::str("V"); if py::eq(&t1779, &t1780) { true } else { false } }) }).truthy() { out.push(v_e.clone()); }} Py::list(out) };
    v_N = py::add(&py::len(&v_nodes.clone())?, &py::len(&v_vs.clone())?)?;
    v_A = { let mut out = vec![]; for t1781 in py::range_iter(&[(v_N.clone()).i()?]) { let mut v__: Py; 
v__ = t1781;
out.push(py::mul(&Py::list(vec![Py::Float(0.0)]), &v_N.clone())?); } Py::list(out) };
    v_z = py::mul(&Py::list(vec![Py::Float(0.0)]), &v_N.clone())?;
    for t1782 in py::iter(&(v_els.clone()))? {
        let t1805 = (t1782).items()?;
        if t1805.len() != 5 { return Err(PyErr::Value("unpack".into())); }
        v_t = t1805[0].clone();
        v_name = t1805[1].clone();
        v_a = t1805[2].clone();
        v_b = t1805[3].clone();
        v_val = t1805[4].clone();
        let t1783 = (Py::tuple(vec![py::method(&v_idx.clone(), "get", &[v_a.clone()])?, py::method(&v_idx.clone(), "get", &[v_b.clone()])?])).items()?;
        if t1783.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_ia = t1783[0].clone();
        v_ib = t1783[1].clone();
        if ({ let t1784 = v_t.clone(); Py::Bool({ let t1785 = Py::str("R"); if py::eq(&t1784, &t1785) { true } else { false } }) }).truthy() {
            v_g = py::truediv(&Py::Float(1.0), &v_val.clone())?;
            for t1786 in py::iter(&(Py::tuple(vec![Py::tuple(vec![v_ia.clone(), v_ia.clone(), v_g.clone()]), Py::tuple(vec![v_ib.clone(), v_ib.clone(), v_g.clone()]), Py::tuple(vec![v_ia.clone(), v_ib.clone(), py::neg(&v_g.clone())?]), Py::tuple(vec![v_ib.clone(), v_ia.clone(), py::neg(&v_g.clone())?])])))? {
                let t1794 = (t1786).items()?;
                if t1794.len() != 3 { return Err(PyErr::Value("unpack".into())); }
                v_i = t1794[0].clone();
                v_j = t1794[1].clone();
                v_s = t1794[2].clone();
                if ({ let t1791 = { let t1787 = v_i.clone(); Py::Bool({ let t1788 = Py::None; if !py::is(&t1787, &t1788) { true } else { false } }) }; if !t1791.truthy() { t1791 } else { { let t1789 = v_j.clone(); Py::Bool({ let t1790 = Py::None; if !py::is(&t1789, &t1790) { true } else { false } }) } } }).truthy() {
                    let t1792 = py::getitem(&v_A.clone(), &v_i.clone())?; let t1793 = v_j.clone();
                    py::setitem(&t1792, &t1793, py::iadd(&py::getitem(&t1792, &t1793)?, &v_s.clone())?)?;
                }
            }
        } else {
            if ({ let t1795 = v_t.clone(); Py::Bool({ let t1796 = Py::str("I"); if py::eq(&t1795, &t1796) { true } else { false } }) }).truthy() {
                if ({ let t1797 = v_ia.clone(); Py::Bool({ let t1798 = Py::None; if !py::is(&t1797, &t1798) { true } else { false } }) }).truthy() {
                    let t1799 = v_z.clone(); let t1800 = v_ia.clone();
                    py::setitem(&t1799, &t1800, py::sub(&py::getitem(&t1799, &t1800)?, &v_val.clone())?)?;
                }
                if ({ let t1801 = v_ib.clone(); Py::Bool({ let t1802 = Py::None; if !py::is(&t1801, &t1802) { true } else { false } }) }).truthy() {
                    let t1803 = v_z.clone(); let t1804 = v_ib.clone();
                    py::setitem(&t1803, &t1804, py::iadd(&py::getitem(&t1803, &t1804)?, &v_val.clone())?)?;
                }
            }
        }
    }
    for t1806 in py::iter(&(py::enumerate(&v_vs.clone(), 0)?))? {
        let t1820 = (t1806).items()?;
        if t1820.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1820[0].clone();
        let t1821 = (t1820[1].clone()).items()?;
        if t1821.len() != 5 { return Err(PyErr::Value("unpack".into())); }
        v_t = t1821[0].clone();
        v_name = t1821[1].clone();
        v_a = t1821[2].clone();
        v_b = t1821[3].clone();
        v_val = t1821[4].clone();
        v_r = py::add(&py::len(&v_nodes.clone())?, &v_k.clone())?;
        let t1807 = (Py::tuple(vec![py::method(&v_idx.clone(), "get", &[v_a.clone()])?, py::method(&v_idx.clone(), "get", &[v_b.clone()])?])).items()?;
        if t1807.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_ia = t1807[0].clone();
        v_ib = t1807[1].clone();
        if ({ let t1808 = v_ia.clone(); Py::Bool({ let t1809 = Py::None; if !py::is(&t1808, &t1809) { true } else { false } }) }).truthy() {
            let t1810 = py::getitem(&v_A.clone(), &v_ia.clone())?; let t1811 = v_r.clone();
            py::setitem(&t1810, &t1811, py::iadd(&py::getitem(&t1810, &t1811)?, &Py::Float(1.0))?)?;
            let t1812 = py::getitem(&v_A.clone(), &v_r.clone())?; let t1813 = v_ia.clone();
            py::setitem(&t1812, &t1813, py::iadd(&py::getitem(&t1812, &t1813)?, &Py::Float(1.0))?)?;
        }
        if ({ let t1814 = v_ib.clone(); Py::Bool({ let t1815 = Py::None; if !py::is(&t1814, &t1815) { true } else { false } }) }).truthy() {
            let t1816 = py::getitem(&v_A.clone(), &v_ib.clone())?; let t1817 = v_r.clone();
            py::setitem(&t1816, &t1817, py::sub(&py::getitem(&t1816, &t1817)?, &Py::Float(1.0))?)?;
            let t1818 = py::getitem(&v_A.clone(), &v_r.clone())?; let t1819 = v_ib.clone();
            py::setitem(&t1818, &t1819, py::sub(&py::getitem(&t1818, &t1819)?, &Py::Float(1.0))?)?;
        }
        py::setitem(&v_z.clone(), &v_r.clone(), v_val.clone())?;
    }
    v_x = f__gauss(v_A.clone(), v_z.clone())?;
    v_vmax = { let t = { let mut out = vec![]; for t1849 in py::iter(&(v_nodes.clone()))? { let mut v_n: Py; 
v_n = t1849;
out.push(py::abs(&py::getitem(&v_x.clone(), &py::getitem(&v_idx.clone(), &v_n.clone())?)?)?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } };
    v_imax = { let t = { let mut out = vec![]; for t1850 in py::range_iter(&[(py::len(&v_vs.clone())?).i()?]) { let mut v_k: Py; 
v_k = t1850;
out.push(py::abs(&py::getitem(&v_x.clone(), &py::add(&py::len(&v_nodes.clone())?, &v_k.clone())?)?)?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } };
    v_snap = { let v_A = v_A.clone(); let v_N = v_N.clone(); let v_a = v_a.clone(); let v_b = v_b.clone(); let v_els = v_els.clone(); let v_g = v_g.clone(); let v_i = v_i.clone(); let v_ia = v_ia.clone(); let v_ib = v_ib.clone(); let v_idx = v_idx.clone(); let v_imax = v_imax.clone(); let v_j = v_j.clone(); let v_k = v_k.clone(); let v_name = v_name.clone(); let v_nodes = v_nodes.clone(); let v_r = v_r.clone(); let v_t = v_t.clone(); let v_val = v_val.clone(); let v_vmax = v_vmax.clone(); let v_vs = v_vs.clone(); let v_x = v_x.clone(); let v_z = v_z.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_v: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_s: Py = a.get(1).cloned().unwrap_or(Py::None);
            return Ok((if ({ let t1851 = py::abs(&v_v.clone())?; Py::Bool({ let t1852 = py::mul(&Py::Float(1e-12), &v_s.clone())?; if py::le(&t1851, &t1852)? { true } else { false } }) }).truthy() { Py::Float(0.0) } else { v_v.clone() }));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_volts = { let t1853 = Py::dict(vec![]); py::setitem(&t1853, &Py::str("0"), Py::Float(0.0))?; py::kw_merge(&t1853, &{ let mut out = vec![]; for t1854 in py::iter(&(v_nodes.clone()))? { let mut v_n: Py; 
v_n = t1854;
out.push((v_n.clone(), py::call(&v_snap.clone(), &[py::getitem(&v_x.clone(), &py::getitem(&v_idx.clone(), &v_n.clone())?)?, v_vmax.clone()])?)); } Py::dict(out) })?; t1853 };
    v_cur = { let mut out = vec![]; for t1855 in py::iter(&(py::enumerate(&v_vs.clone(), 0)?))? { let mut v_e: Py; let mut v_k: Py; 
let t1856 = (t1855).items()?;
if t1856.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t1856[0].clone();
v_e = t1856[1].clone();
out.push((py::getitem(&v_e.clone(), &Py::Int(1i64))?, py::call(&v_snap.clone(), &[py::getitem(&v_x.clone(), &py::add(&py::len(&v_nodes.clone())?, &v_k.clone())?)?, v_imax.clone()])?)); } Py::dict(out) };
    return Ok(Py::tuple(vec![v_volts.clone(), v_cur.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__net_count(mut v_t: Py) -> R {
    return Ok((if (Py::Bool(matches!(v_t.clone(), Py::Float(_)))).truthy() { Py::Int(1i64) } else { py::sum(&{ let mut out = vec![]; for t1327 in py::iter(&(py::getitem(&v_t.clone(), &Py::Int(1i64))?))? { let mut v_x: Py; 
v_x = t1327;
out.push(f__net_count(v_x.clone())?); } Py::list(out) }, None)? }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__net_tokens(mut v_text: Py) -> R {
    let mut v_toks: Py = Py::None;
    v_toks = ax_engine::pyre::module_call("findall", &Py::str("\\|\\||\\+|\\(|\\)|(?:\\d+(?:\\.\\d+)?|\\.\\d+)(?:[eE][-+]?\\d+)?|\\S"), &[v_text.clone()], 0)?;
    if ({ let t1295 = py::method(&Py::str(""), "join", &[v_toks.clone()])?; Py::Bool({ let t1296 = ax_engine::pyre::module_call("sub", &Py::str("\\s+"), &[Py::str(""), v_text.clone(), Py::Int(0)], 0)?; if py::ne(&t1295, &t1296) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("cannot read the network expression"))));
    }
    return Ok(v_toks.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__net_tree(mut v_text: Py) -> R {
    let mut v_factor: Py = Py::None;
    let mut v_peek: Py = Py::None;
    let mut v_pos: Py = Py::None;
    let mut v_take: Py = Py::None;
    let mut v_term: Py = Py::None;
    let mut v_toks: Py = Py::None;
    let mut v_tree: Py = Py::None;
    let c_expr = py::cell_new();
    v_toks = f__net_tokens(v_text.clone())?;
    v_pos = Py::list(vec![Py::Int(0i64)]);
    v_peek = { let v_peek = v_peek.clone(); let v_pos = v_pos.clone(); let v_text = v_text.clone(); let v_toks = v_toks.clone(); let c_expr = c_expr.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            return Ok((if ({ let t1297 = py::getitem(&v_pos.clone(), &Py::Int(0i64))?; Py::Bool({ let t1298 = py::len(&v_toks.clone())?; if py::lt(&t1297, &t1298)? { true } else { false } }) }).truthy() { py::getitem(&v_toks.clone(), &py::getitem(&v_pos.clone(), &Py::Int(0i64))?)? } else { Py::None }));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_take = { let v_peek = v_peek.clone(); let v_pos = v_pos.clone(); let v_take = v_take.clone(); let v_text = v_text.clone(); let v_toks = v_toks.clone(); let c_expr = c_expr.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            if ({ let t1299 = py::getitem(&v_pos.clone(), &Py::Int(0i64))?; Py::Bool({ let t1300 = py::len(&v_toks.clone())?; if py::ge(&t1299, &t1300)? { true } else { false } }) }).truthy() {
                return Err(py::value_error(py::to_str(&Py::str("the network expression ends too early (unbalanced parentheses?)"))));
            }
            let t1301 = v_pos.clone(); let t1302 = Py::Int(0i64);
            py::setitem(&t1301, &t1302, py::iadd(&py::getitem(&t1301, &t1302)?, &Py::Int(1i64))?)?;
            return Ok(py::getitem(&v_toks.clone(), &py::sub(&py::getitem(&v_pos.clone(), &Py::Int(0i64))?, &Py::Int(1i64))?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_factor = { let v_factor = v_factor.clone(); let v_peek = v_peek.clone(); let v_pos = v_pos.clone(); let v_take = v_take.clone(); let v_text = v_text.clone(); let v_toks = v_toks.clone(); let c_expr = c_expr.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_node: Py = Py::None;
            let mut v_t: Py = Py::None;
            let mut v_v: Py = Py::None;
            v_t = py::call(&v_peek.clone(), &[])?;
            if ({ let t1303 = v_t.clone(); Py::Bool({ let t1304 = Py::str("("); if py::eq(&t1303, &t1304) { true } else { false } }) }).truthy() {
                let _ = py::call(&v_take.clone(), &[])?;
                v_node = py::call(&py::cell_get(&c_expr)?, &[])?;
                if ({ let t1305 = py::call(&v_take.clone(), &[])?; Py::Bool({ let t1306 = Py::str(")"); if py::ne(&t1305, &t1306) { true } else { false } }) }).truthy() {
                    return Err(py::value_error(py::to_str(&Py::str("unbalanced parentheses in the network expression"))));
                }
                return Ok(v_node.clone());
            }
            if ({ let t1309 = { let t1307 = v_t.clone(); Py::Bool({ let t1308 = Py::None; if py::is(&t1307, &t1308) { true } else { false } }) }; if t1309.truthy() { t1309 } else { py::not(&ax_engine::pyre::module_call("fullmatch", &Py::str("(?:\\d+(?:\\.\\d+)?|\\.\\d+)(?:[eE][-+]?\\d+)?"), &[v_t.clone()], 0)?) } }).truthy() {
                return Err(py::value_error(py::to_str(&Py::str(&["expected a resistance value, found ".to_string(), py::format(&Py::str(&py::repr(&v_t.clone())), "")?].concat()))));
            }
            let _ = py::call(&v_take.clone(), &[])?;
            v_v = py::float(&v_t.clone())?;
            if (py::not(&{ let t1312 = { let t1310 = v_v.clone(); Py::Bool({ let t1311 = Py::Int(0i64); if py::gt(&t1310, &t1311)? { true } else { false } }) }; if !t1312.truthy() { t1312 } else { py::math1("isfinite", &v_v.clone())? } })).truthy() {
                return Err(py::value_error(py::to_str(&Py::str("every resistance must be positive"))));
            }
            return Ok(v_v.clone());
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_term = { let v_factor = v_factor.clone(); let v_peek = v_peek.clone(); let v_pos = v_pos.clone(); let v_take = v_take.clone(); let v_term = v_term.clone(); let v_text = v_text.clone(); let v_toks = v_toks.clone(); let c_expr = c_expr.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_items: Py = Py::None;
            v_items = Py::list(vec![py::call(&v_factor.clone(), &[])?]);
            while ({ let t1313 = py::call(&v_peek.clone(), &[])?; Py::Bool({ let t1314 = Py::str("||"); if py::eq(&t1313, &t1314) { true } else { false } }) }).truthy() {
                let _ = py::call(&v_take.clone(), &[])?;
                let _ = py::method(&v_items.clone(), "append", &[py::call(&v_factor.clone(), &[])?])?;
            }
            return Ok((if ({ let t1315 = py::len(&v_items.clone())?; Py::Bool({ let t1316 = Py::Int(1i64); if py::eq(&t1315, &t1316) { true } else { false } }) }).truthy() { py::getitem(&v_items.clone(), &Py::Int(0i64))? } else { Py::tuple(vec![Py::str("p"), v_items.clone()]) }));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    py::cell_set(&c_expr, { let v_factor = v_factor.clone(); let v_peek = v_peek.clone(); let v_pos = v_pos.clone(); let v_take = v_take.clone(); let v_term = v_term.clone(); let v_text = v_text.clone(); let v_toks = v_toks.clone(); let c_expr = c_expr.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_items: Py = Py::None;
            v_items = Py::list(vec![py::call(&v_term.clone(), &[])?]);
            while ({ let t1317 = py::call(&v_peek.clone(), &[])?; Py::Bool({ let t1318 = Py::str("+"); if py::eq(&t1317, &t1318) { true } else { false } }) }).truthy() {
                let _ = py::call(&v_take.clone(), &[])?;
                let _ = py::method(&v_items.clone(), "append", &[py::call(&v_term.clone(), &[])?])?;
            }
            return Ok((if ({ let t1319 = py::len(&v_items.clone())?; Py::Bool({ let t1320 = Py::Int(1i64); if py::eq(&t1319, &t1320) { true } else { false } }) }).truthy() { py::getitem(&v_items.clone(), &Py::Int(0i64))? } else { Py::tuple(vec![Py::str("s"), v_items.clone()]) }));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) });
    v_tree = py::call(&py::cell_get(&c_expr)?, &[])?;
    if ({ let t1321 = py::getitem(&v_pos.clone(), &Py::Int(0i64))?; Py::Bool({ let t1322 = py::len(&v_toks.clone())?; if py::ne(&t1321, &t1322) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&["unexpected ".to_string(), py::format(&Py::str(&py::repr(&py::getitem(&v_toks.clone(), &py::getitem(&v_pos.clone(), &Py::Int(0i64))?)?)), "")?, " in the network expression".to_string()].concat()))));
    }
    return Ok(v_tree.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__net_value(mut v_t: Py) -> R {
    let mut v_vals: Py = Py::None;
    if (Py::Bool(matches!(v_t.clone(), Py::Float(_)))).truthy() {
        return Ok(v_t.clone());
    }
    v_vals = { let mut out = vec![]; for t1323 in py::iter(&(py::getitem(&v_t.clone(), &Py::Int(1i64))?))? { let mut v_x: Py; 
v_x = t1323;
out.push(f__net_value(v_x.clone())?); } Py::list(out) };
    return Ok((if ({ let t1324 = py::getitem(&v_t.clone(), &Py::Int(0i64))?; Py::Bool({ let t1325 = Py::str("s"); if py::eq(&t1324, &t1325) { true } else { false } }) }).truthy() { py::fsum(&v_vals.clone())? } else { py::truediv(&Py::Float(1.0), &py::fsum(&{ let mut out = vec![]; for t1326 in py::iter(&(v_vals.clone()))? { let mut v_v: Py; 
v_v = t1326;
out.push(py::truediv(&Py::Float(1.0), &v_v.clone())?); } Py::list(out) })?)? }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__netlist(mut v_text: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_els: Py = Py::None;
    let mut v_name: Py = Py::None;
    let mut v_names: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_part: Py = Py::None;
    let mut v_t: Py = Py::None;
    let mut v_toks: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_val: Py = Py::None;
    let t1742 = (Py::tuple(vec![Py::list(vec![]), py::set(&Py::list(vec![]))?])).items()?;
    if t1742.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_els = t1742[0].clone();
    v_names = t1742[1].clone();
    for t1743 in py::iter(&(ax_engine::pyre::module_call("split", &Py::str("[;\\n]+"), &[v_text.clone(), Py::Int(0)], 0)?))? {
        v_part = t1743;
        v_p = py::method(&py::method(&py::method(&v_part.clone(), "strip", &[])?, "rstrip", &[Py::str(".")])?, "strip", &[])?;
        if (py::not(&v_p.clone())).truthy() {
            continue;
        }
        v_toks = py::method(&v_p.clone(), "split", &[])?;
        if ({ let t1744 = py::len(&v_toks.clone())?; Py::Bool({ let t1745 = Py::Int(4i64); if py::ne(&t1744, &t1745) { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str(&["cannot read the element '".to_string(), py::format(&v_p.clone(), "")?, "': write <name> <node> <node> <value>".to_string()].concat()))));
        }
        let t1746 = (v_toks.clone()).items()?;
        if t1746.len() != 4 { return Err(PyErr::Value("unpack".into())); }
        v_name = t1746[0].clone();
        v_a = t1746[1].clone();
        v_b = t1746[2].clone();
        v_v = t1746[3].clone();
        v_t = py::method(&py::getitem(&v_name.clone(), &Py::Int(0i64))?, "upper", &[])?;
        if ({ let t1747 = v_t.clone(); Py::Bool({ let t1748 = Py::str("RVI"); if !py::contains(&t1748, &t1747)? { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str(&["element '".to_string(), py::format(&v_name.clone(), "")?, "': only resistors (R), voltage sources (V) and current sources (I)".to_string()].concat()))));
        }
        if ({ let t1749 = py::method(&v_name.clone(), "upper", &[])?; Py::Bool({ let t1750 = v_names.clone(); if py::contains(&t1750, &t1749)? { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str(&["element name '".to_string(), py::format(&v_name.clone(), "")?, "' is used twice".to_string()].concat()))));
        }
        let _ = py::method(&v_names.clone(), "add", &[py::method(&v_name.clone(), "upper", &[])?])?;
        let t1751: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                v_val = py::float(&v_v.clone())?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t1751 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (matches!(e, PyErr::Value(_))) => {
                return Err(py::value_error(py::to_str(&Py::str(&["element '".to_string(), py::format(&v_name.clone(), "")?, "': '".to_string(), py::format(&v_v.clone(), "")?, "' is not a number".to_string()].concat()))));
            }
            Err(e) => return Err(e),
        }
        if (py::not(&py::math1("isfinite", &v_val.clone())?)).truthy() {
            return Err(py::value_error(py::to_str(&Py::str(&["element '".to_string(), py::format(&v_name.clone(), "")?, "': the value must be finite".to_string()].concat()))));
        }
        if ({ let t1756 = { let t1752 = v_t.clone(); Py::Bool({ let t1753 = Py::str("R"); if py::eq(&t1752, &t1753) { true } else { false } }) }; if !t1756.truthy() { t1756 } else { { let t1754 = v_val.clone(); Py::Bool({ let t1755 = Py::Int(0i64); if py::le(&t1754, &t1755)? { true } else { false } }) } } }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str(&["resistor '".to_string(), py::format(&v_name.clone(), "")?, "' must have a positive resistance".to_string()].concat()))));
        }
        if ({ let t1757 = v_a.clone(); Py::Bool({ let t1758 = v_b.clone(); if py::eq(&t1757, &t1758) { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str(&["element '".to_string(), py::format(&v_name.clone(), "")?, "' connects node ".to_string(), py::format(&v_a.clone(), "")?, " to itself".to_string()].concat()))));
        }
        let _ = py::method(&v_els.clone(), "append", &[Py::tuple(vec![v_t.clone(), v_name.clone(), v_a.clone(), v_b.clone(), v_val.clone()])])?;
    }
    if (py::not(&Py::Bool(({ let mut out = vec![]; for t1759 in py::iter(&(v_els.clone()))? { let mut v_e: Py; 
v_e = t1759;
out.push({ let t1760 = py::getitem(&v_e.clone(), &Py::Int(0i64))?; Py::Bool({ let t1761 = Py::str("R"); if py::eq(&t1760, &t1761) { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the circuit needs at least one resistor"))));
    }
    if (py::not(&Py::Bool(({ let mut out = vec![]; for t1762 in py::iter(&(v_els.clone()))? { let mut v_e: Py; 
v_e = t1762;
out.push({ let t1763 = Py::str("0"); Py::Bool({ let t1764 = Py::tuple(vec![py::getitem(&v_e.clone(), &Py::Int(2i64))?, py::getitem(&v_e.clone(), &Py::Int(3i64))?]); if py::contains(&t1764, &t1763)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the circuit needs a ground node named 0"))));
    }
    return Ok(v_els.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__node_key(mut v_n: Py) -> R {
    return Ok((if (py::method(&v_n.clone(), "isdigit", &[])?).truthy() { Py::tuple(vec![Py::Int(0i64), py::int(&v_n.clone())?, Py::str("")]) } else { Py::tuple(vec![Py::Int(1i64), Py::Int(0i64), v_n.clone()]) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__padd(mut v_a: Py, mut v_b: Py) -> R {
    let mut v_n: Py = Py::None;
    v_n = py::minmax(&[py::len(&v_a.clone())?, py::len(&v_b.clone())?], true)?;
    let t1630 = (Py::tuple(vec![py::add(&py::mul(&Py::list(vec![Py::Float(0.0)]), &py::sub(&v_n.clone(), &py::len(&v_a.clone())?)?)?, &Py::list((v_a.clone()).items()?))?, py::add(&py::mul(&Py::list(vec![Py::Float(0.0)]), &py::sub(&v_n.clone(), &py::len(&v_b.clone())?)?)?, &Py::list((v_b.clone()).items()?))?])).items()?;
    if t1630.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t1630[0].clone();
    v_b = t1630[1].clone();
    return Ok({ let mut out = vec![]; for t1631 in py::iter(&(py::zip(&[v_a.clone(), v_b.clone()])?))? { let mut v_x: Py; let mut v_y: Py; 
let t1632 = (t1631).items()?;
if t1632.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t1632[0].clone();
v_y = t1632[1].clone();
out.push(py::add(&v_x.clone(), &v_y.clone())?); } Py::list(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__pair_resistances_delta(mut v_Rab: Py, mut v_Rbc: Py, mut v_Rca: Py) -> R {
    let mut v_par: Py = Py::None;
    v_par = { let v_Rab = v_Rab.clone(); let v_Rbc = v_Rbc.clone(); let v_Rca = v_Rca.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_x: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_y: Py = a.get(1).cloned().unwrap_or(Py::None);
            return Ok(py::truediv(&py::mul(&v_x.clone(), &v_y.clone())?, &py::add(&v_x.clone(), &v_y.clone())?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    return Ok(Py::dict(vec![(Py::str("ab"), py::call(&v_par.clone(), &[v_Rab.clone(), py::add(&v_Rbc.clone(), &v_Rca.clone())?])?), (Py::str("bc"), py::call(&v_par.clone(), &[v_Rbc.clone(), py::add(&v_Rab.clone(), &v_Rca.clone())?])?), (Py::str("ca"), py::call(&v_par.clone(), &[v_Rca.clone(), py::add(&v_Rab.clone(), &v_Rbc.clone())?])?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__pder(mut v_c: Py) -> R {
    let mut v_n: Py = Py::None;
    v_n = py::sub(&py::len(&v_c.clone())?, &Py::Int(1i64))?;
    return Ok({ let t1647 = { let mut out = vec![]; for t1645 in py::iter(&(py::enumerate(&py::slice(&v_c.clone(), None, Some((py::neg(&Py::Int(1i64))?).i()?), None)?, 0)?))? { let mut v_i: Py; let mut v_x: Py; 
let t1646 = (t1645).items()?;
if t1646.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_i = t1646[0].clone();
v_x = t1646[1].clone();
out.push(py::mul(&v_x.clone(), &py::sub(&v_n.clone(), &v_i.clone())?)?); } Py::list(out) }; if t1647.truthy() { t1647 } else { Py::list(vec![Py::Float(0.0)]) } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__phase_crossover(mut v_num: Py, mut v_den: Py) -> R {
    let mut v_d_iw: Py = Py::None;
    let mut v_im: Py = Py::None;
    let mut v_n_iw: Py = Py::None;
    let mut v_nd: Py = Py::None;
    let mut v_ws: Py = Py::None;
    let t584 = (Py::tuple(vec![f__iw(v_num.clone())?, f__iw(v_den.clone())?])).items()?;
    if t584.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_n_iw = t584[0].clone();
    v_d_iw = t584[1].clone();
    v_nd = f__pmul(v_n_iw.clone(), { let mut out = vec![]; for t591 in py::iter(&(v_d_iw.clone()))? { let mut v_y: Py; 
v_y = t591;
out.push(py::method(&v_y.clone(), "conjugate", &[])?); } Py::list(out) })?;
    v_im = f__ptrim({ let mut out = vec![]; for t592 in py::iter(&(v_nd.clone()))? { let mut v_x: Py; 
v_x = t592;
out.push(py::imag(&v_x.clone())?); } Py::list(out) })?;
    if ({ let t593 = py::len(&v_im.clone())?; Py::Bool({ let t594 = Py::Int(2i64); if py::lt(&t593, &t594)? { true } else { false } }) }).truthy() {
        return Ok(Py::list(vec![]));
    }
    v_ws = { let mut out = vec![]; for t641 in py::iter(&(f__positive_real_roots(v_im.clone(), Py::Float(1.0))?))? { let mut v_w: Py; 
v_w = t641;
if ({ let t642 = py::real(&f__G(v_num.clone(), v_den.clone(), v_w.clone())?)?; Py::Bool({ let t643 = Py::Int(0i64); if py::lt(&t642, &t643)? { true } else { false } }) }).truthy() { out.push(v_w.clone()); }} Py::list(out) };
    return Ok({ let mut out = vec![]; for t646 in py::iter(&(v_ws.clone()))? { let mut v_w: Py; 
v_w = t646;
out.push(f__polish({ let v_d_iw = v_d_iw.clone(); let v_den = v_den.clone(); let v_im = v_im.clone(); let v_n_iw = v_n_iw.clone(); let v_nd = v_nd.clone(); let v_num = v_num.clone(); let v_ws = v_ws.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_w: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::truediv(&py::imag(&f__G(v_num.clone(), v_den.clone(), v_w.clone())?)?, &py::abs(&f__G(v_num.clone(), v_den.clone(), v_w.clone())?)?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, v_w.clone())?); } Py::list(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__pmul(mut v_a: Py, mut v_b: Py) -> R {
    let mut v_i: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_x: Py = Py::None;
    let mut v_y: Py = Py::None;
    v_out = py::mul(&Py::list(vec![Py::Float(0.0)]), &py::sub(&py::add(&py::len(&v_a.clone())?, &py::len(&v_b.clone())?)?, &Py::Int(1i64))?)?;
    for t1164 in py::iter(&(py::enumerate(&v_a.clone(), 0)?))? {
        let t1169 = (t1164).items()?;
        if t1169.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_i = t1169[0].clone();
        v_x = t1169[1].clone();
        for t1165 in py::iter(&(py::enumerate(&v_b.clone(), 0)?))? {
            let t1168 = (t1165).items()?;
            if t1168.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_j = t1168[0].clone();
            v_y = t1168[1].clone();
            let t1166 = v_out.clone(); let t1167 = py::add(&v_i.clone(), &v_j.clone())?;
            py::setitem(&t1166, &t1167, py::iadd(&py::getitem(&t1166, &t1167)?, &py::mul(&v_x.clone(), &v_y.clone())?)?)?;
        }
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__polish(mut v_f: Py, mut v_w: Py) -> R {
    let mut v__: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_fw: Py = Py::None;
    let mut v_h: Py = Py::None;
    let mut v_step: Py = Py::None;
    let mut v_w2: Py = Py::None;
    for t1225 in py::range_iter(&[(Py::Int(30i64)).i()?]) {
        v__ = t1225;
        v_h = py::mul(&Py::Float(1e-07), &v_w.clone())?;
        v_fw = py::call(&v_f.clone(), &[v_w.clone()])?;
        v_d = py::truediv(&py::sub(&py::call(&v_f.clone(), &[py::add(&v_w.clone(), &v_h.clone())?])?, &py::call(&v_f.clone(), &[py::sub(&v_w.clone(), &v_h.clone())?])?)?, &py::mul(&Py::Int(2i64), &v_h.clone())?)?;
        if ({ let t1226 = v_d.clone(); Py::Bool({ let t1227 = Py::Int(0i64); if py::eq(&t1226, &t1227) { true } else { false } }) }).truthy() {
            break;
        }
        v_step = py::truediv(&v_fw.clone(), &v_d.clone())?;
        v_w2 = py::sub(&v_w.clone(), &v_step.clone())?;
        if ({ let t1228 = v_w2.clone(); Py::Bool({ let t1229 = Py::Int(0i64); if py::le(&t1228, &t1229)? { true } else { false } }) }).truthy() {
            break;
        }
        if ({ let t1230 = py::abs(&v_step.clone())?; Py::Bool({ let t1231 = py::mul(&Py::Float(1e-16), &v_w.clone())?; if py::le(&t1230, &t1231)? { true } else { false } }) }).truthy() {
            v_w = v_w2.clone();
            break;
        }
        v_w = v_w2.clone();
    }
    return Ok(v_w.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__positive_real_roots(mut v_c: Py, mut v_scale_w: Py) -> R {
    let mut v_merged: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_x: Py = Py::None;
    v_out = Py::list(vec![]);
    for t1214 in py::iter(&(f__roots(v_c.clone())?))? {
        v_r = t1214;
        v_r = py::complex(&[v_r.clone()])?;
        if ({ let t1219 = { let t1215 = py::real(&v_r.clone())?; Py::Bool({ let t1216 = py::mul(&Py::Float(1e-09), &v_scale_w.clone())?; if py::gt(&t1215, &t1216)? { true } else { false } }) }; if !t1219.truthy() { t1219 } else { { let t1217 = py::abs(&py::imag(&v_r.clone())?)?; Py::Bool({ let t1218 = py::mul(&Py::Float(1e-07), &py::minmax(&[py::abs(&v_r.clone())?, Py::Float(1e-300)], true)?)?; if py::le(&t1217, &t1218)? { true } else { false } }) } } }).truthy() {
            let _ = py::method(&v_out.clone(), "append", &[py::real(&v_r.clone())?])?;
        }
    }
    let _ = py::method_sort(&v_out.clone(), None, false)?;
    v_merged = Py::list(vec![]);
    for t1220 in py::iter(&(v_out.clone()))? {
        v_x = t1220;
        if ({ let t1223 = py::not(&v_merged.clone()); if t1223.truthy() { t1223 } else { { let t1221 = py::abs(&py::sub(&v_x.clone(), &py::getitem(&v_merged.clone(), &py::neg(&Py::Int(1i64))?)?)?)?; Py::Bool({ let t1222 = py::mul(&Py::Float(1e-07), &v_x.clone())?; if py::gt(&t1221, &t1222)? { true } else { false } }) } } }).truthy() {
            let _ = py::method(&v_merged.clone(), "append", &[v_x.clone()])?;
        }
    }
    return Ok(v_merged.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__positive_values(mut v_vals: Py, mut v_what: Py) -> R {
    let mut v_v: Py = Py::None;
    if ({ let t1476 = py::len(&v_vals.clone())?; Py::Bool({ let t1477 = Py::Int(2i64); if py::lt(&t1476, &t1477)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&["give at least two ".to_string(), py::format(&v_what.clone(), "")?].concat()))));
    }
    for t1478 in py::iter(&(v_vals.clone()))? {
        v_v = t1478;
        if (py::not(&{ let t1481 = py::math1("isfinite", &v_v.clone())?; if !t1481.truthy() { t1481 } else { { let t1479 = v_v.clone(); Py::Bool({ let t1480 = Py::Int(0i64); if py::gt(&t1479, &t1480)? { true } else { false } }) } } })).truthy() {
            return Err(py::value_error(py::to_str(&Py::str(&["every ".to_string(), py::format(&(if (py::method(&v_what.clone(), "endswith", &[Py::str("s")])?).truthy() { py::slice(&v_what.clone(), None, Some((py::neg(&Py::Int(1i64))?).i()?), None)? } else { v_what.clone() }), "")?, " must be a positive finite number".to_string()].concat()))));
        }
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__power_delivered(mut v_els: Py, mut v_volts: Py, mut v_cur: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_name: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_t: Py = Py::None;
    let mut v_val: Py = Py::None;
    v_p = Py::Float(0.0);
    for t480 in py::iter(&(v_els.clone()))? {
        let t485 = (t480).items()?;
        if t485.len() != 5 { return Err(PyErr::Value("unpack".into())); }
        v_t = t485[0].clone();
        v_name = t485[1].clone();
        v_a = t485[2].clone();
        v_b = t485[3].clone();
        v_val = t485[4].clone();
        if ({ let t481 = v_t.clone(); Py::Bool({ let t482 = Py::str("V"); if py::eq(&t481, &t482) { true } else { false } }) }).truthy() {
            v_p = py::iadd(&v_p, &py::mul(&py::neg(&v_val.clone())?, &py::getitem(&v_cur.clone(), &v_name.clone())?)?)?;
        } else {
            if ({ let t483 = v_t.clone(); Py::Bool({ let t484 = Py::str("I"); if py::eq(&t483, &t484) { true } else { false } }) }).truthy() {
                v_p = py::iadd(&v_p, &py::mul(&v_val.clone(), &py::sub(&py::getitem(&v_volts.clone(), &v_b.clone())?, &py::getitem(&v_volts.clone(), &v_a.clone())?)?)?)?;
            }
        }
    }
    return Ok(v_p.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__prime_implicants(mut v_n: Py, mut v_ones: Py) -> R {
    let mut v_current: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_lst: Py = Py::None;
    let mut v_m1: Py = Py::None;
    let mut v_m2: Py = Py::None;
    let mut v_nxt: Py = Py::None;
    let mut v_primes: Py = Py::None;
    let mut v_used: Py = Py::None;
    let mut v_v1: Py = Py::None;
    let mut v_v2: Py = Py::None;
    v_current = py::set(&{ let mut out = vec![]; for t1969 in py::iter(&(v_ones.clone()))? { let mut v_m: Py; 
v_m = t1969;
out.push(Py::tuple(vec![v_m.clone(), Py::Int(0i64)])); } Py::list(out) })?;
    v_primes = py::set(&Py::list(vec![]))?;
    while (v_current.clone()).truthy() {
        let t1970 = (Py::tuple(vec![py::set(&Py::list(vec![]))?, py::set(&Py::list(vec![]))?])).items()?;
        if t1970.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_nxt = t1970[0].clone();
        v_used = t1970[1].clone();
        v_lst = py::sorted(&v_current.clone(), (Py::Bool(false)).truthy())?;
        for t1971 in py::range_iter(&[(py::len(&v_lst.clone())?).i()?]) {
            v_i = t1971;
            for t1972 in py::range_iter(&[(py::add(&v_i.clone(), &Py::Int(1i64))?).i()?, (py::len(&v_lst.clone())?).i()?]) {
                v_j = t1972;
                let t1973 = (Py::tuple(vec![py::getitem(&v_lst.clone(), &v_i.clone())?, py::getitem(&v_lst.clone(), &v_j.clone())?])).items()?;
                if t1973.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                let t1974 = (t1973[0].clone()).items()?;
                if t1974.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                v_v1 = t1974[0].clone();
                v_m1 = t1974[1].clone();
                let t1975 = (t1973[1].clone()).items()?;
                if t1975.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                v_v2 = t1975[0].clone();
                v_m2 = t1975[1].clone();
                v_d = py::bitxor(&v_v1.clone(), &v_v2.clone())?;
                if ({ let t1981 = { let t1976 = v_m1.clone(); Py::Bool({ let t1977 = v_m2.clone(); if py::eq(&t1976, &t1977) { true } else { false } }) }; if !t1981.truthy() { t1981 } else { { let t1980 = v_d.clone(); if !t1980.truthy() { t1980 } else { { let t1978 = py::bitand(&v_d.clone(), &py::sub(&v_d.clone(), &Py::Int(1i64))?)?; Py::Bool({ let t1979 = Py::Int(0i64); if py::eq(&t1978, &t1979) { true } else { false } }) } } } } }).truthy() {
                    let _ = py::method(&v_nxt.clone(), "add", &[Py::tuple(vec![py::bitand(&v_v1.clone(), &py::invert(&v_d.clone())?)?, py::bitor(&v_m1.clone(), &v_d.clone())?])])?;
                    let _ = py::method(&v_used.clone(), "update", &[Py::tuple(vec![py::getitem(&v_lst.clone(), &v_i.clone())?, py::getitem(&v_lst.clone(), &v_j.clone())?])])?;
                }
            }
        }
        v_primes = py::bitor(&v_primes, &py::sub(&v_current.clone(), &v_used.clone())?)?;
        v_current = v_nxt.clone();
    }
    return Ok(py::sorted(&v_primes.clone(), (Py::Bool(false)).truthy())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__ptrim(mut v_c: Py) -> R {
    v_c = { let mut out = vec![]; for t1613 in py::iter(&(v_c.clone()))? { let mut v_x: Py; 
v_x = t1613;
out.push(py::float(&v_x.clone())?); } Py::list(out) };
    while ({ let t1618 = { let t1614 = py::len(&v_c.clone())?; Py::Bool({ let t1615 = Py::Int(1i64); if py::gt(&t1614, &t1615)? { true } else { false } }) }; if !t1618.truthy() { t1618 } else { { let t1616 = py::getitem(&v_c.clone(), &Py::Int(0i64))?; Py::Bool({ let t1617 = Py::Int(0i64); if py::eq(&t1616, &t1617) { true } else { false } }) } } }).truthy() {
        v_c = py::slice(&v_c.clone(), Some((Py::Int(1i64)).i()?), None, None)?;
    }
    return Ok(v_c.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__pval(mut v_c: Py, mut v_z: Py) -> R {
    let mut v_acc: Py = Py::None;
    let mut v_x: Py = Py::None;
    v_acc = (if (Py::Bool(matches!(v_z.clone(), Py::Complex(..)))).truthy() { Py::Complex(0.0, 0.0) } else { Py::Float(0.0) });
    for t1652 in py::iter(&(v_c.clone()))? {
        v_x = t1652;
        v_acc = py::add(&py::mul(&v_acc.clone(), &v_z.clone())?, &v_x.clone())?;
    }
    return Ok(v_acc.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__rank(mut v_M: Py, mut v_tol: Py) -> R {
    let mut v_R: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_cols: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_rank: Py = Py::None;
    let mut v_rows: Py = Py::None;
    let mut v_scale: Py = Py::None;
    v_R = { let mut out = vec![]; for t316 in py::iter(&(v_M.clone()))? { let mut v_row: Py; 
v_row = t316;
out.push(py::slice(&v_row.clone(), None, None, None)?); } Py::list(out) };
    let t317 = (Py::tuple(vec![py::len(&v_R.clone())?, py::len(&py::getitem(&v_R.clone(), &Py::Int(0i64))?)?])).items()?;
    if t317.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_rows = t317[0].clone();
    v_cols = t317[1].clone();
    v_scale = { let t320 = py::minmax(&[{ let mut out = vec![]; for t318 in py::iter(&(v_R.clone()))? { let mut v_r: Py; 
v_r = t318;
for t319 in py::iter(&(v_r.clone()))? { let mut v_x: Py; 
v_x = t319;
out.push(py::abs(&v_x.clone())?); }} Py::list(out) }], true)?; if t320.truthy() { t320 } else { Py::Float(1.0) } };
    let t321 = (Py::tuple(vec![Py::Int(0i64), Py::Int(0i64)])).items()?;
    if t321.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_rank = t321[0].clone();
    v_r = t321[1].clone();
    for t322 in py::range_iter(&[(v_cols.clone()).i()?]) {
        v_c = t322;
        v_p = py::minmax_key(&[Py::list((py::range_iter(&[(v_r.clone()).i()?, (v_rows.clone()).i()?])).into_iter().collect::<Vec<Py>>())], &{ let v_M = v_M.clone(); let v_R = v_R.clone(); let v_c = v_c.clone(); let v_cols = v_cols.clone(); let v_r = v_r.clone(); let v_rank = v_rank.clone(); let v_rows = v_rows.clone(); let v_scale = v_scale.clone(); let v_tol = v_tol.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_i: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::abs(&py::getitem(&py::getitem(&v_R.clone(), &v_i.clone())?, &v_c.clone())?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, true)?;
        if ({ let t327 = { let t323 = v_p.clone(); Py::Bool({ let t324 = Py::None; if py::is(&t323, &t324) { true } else { false } }) }; if t327.truthy() { t327 } else { { let t325 = py::abs(&py::getitem(&py::getitem(&v_R.clone(), &v_p.clone())?, &v_c.clone())?)?; Py::Bool({ let t326 = py::mul(&v_tol.clone(), &v_scale.clone())?; if py::le(&t325, &t326)? { true } else { false } }) } } }).truthy() {
            continue;
        }
        let t328 = (Py::tuple(vec![py::getitem(&v_R.clone(), &v_p.clone())?, py::getitem(&v_R.clone(), &v_r.clone())?])).items()?;
        if t328.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        py::setitem(&v_R.clone(), &v_r.clone(), t328[0].clone())?;
        py::setitem(&v_R.clone(), &v_p.clone(), t328[1].clone())?;
        for t329 in py::range_iter(&[(py::add(&v_r.clone(), &Py::Int(1i64))?).i()?, (v_rows.clone()).i()?]) {
            v_i = t329;
            v_f = py::truediv(&py::getitem(&py::getitem(&v_R.clone(), &v_i.clone())?, &v_c.clone())?, &py::getitem(&py::getitem(&v_R.clone(), &v_r.clone())?, &v_c.clone())?)?;
            py::setitem(&v_R.clone(), &v_i.clone(), { let mut out = vec![]; for t330 in py::iter(&(py::zip(&[py::getitem(&v_R.clone(), &v_i.clone())?, py::getitem(&v_R.clone(), &v_r.clone())?])?))? { let mut v_x: Py; let mut v_y: Py; 
let t331 = (t330).items()?;
if t331.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t331[0].clone();
v_y = t331[1].clone();
out.push(py::sub(&v_x.clone(), &py::mul(&v_f.clone(), &v_y.clone())?)?); } Py::list(out) })?;
        }
        v_r = py::iadd(&v_r, &Py::Int(1i64))?;
        v_rank = py::iadd(&v_rank, &Py::Int(1i64))?;
        if ({ let t332 = v_r.clone(); Py::Bool({ let t333 = v_rows.clone(); if py::eq(&t332, &t333) { true } else { false } }) }).truthy() {
            break;
        }
    }
    return Ok(v_rank.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__rel(mut v_a: Py, mut v_b: Py) -> R {
    return Ok(py::truediv(&py::abs(&py::sub(&v_a.clone(), &v_b.clone())?)?, &py::minmax(&[Py::Float(1e-300), py::abs(&v_a.clone())?, py::abs(&v_b.clone())?], true)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__resonance(mut v_R: Py, mut v_L: Py, mut v_C: Py, mut v_series: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_tp: Py = Py::None;
    let mut v_w0: Py = Py::None;
    let mut v_w1: Py = Py::None;
    let mut v_w2: Py = Py::None;
    let _ = f__check_positive(Py::dict(vec![(Py::str("R"), v_R.clone()), (Py::str("L"), v_L.clone()), (Py::str("C"), v_C.clone())]))?;
    v_w0 = py::truediv(&Py::Int(1i64), &py::math1("sqrt", &py::mul(&v_L.clone(), &v_C.clone())?)?)?;
    v_a = (if (v_series.clone()).truthy() { py::truediv(&v_R.clone(), &py::mul(&Py::Int(2i64), &v_L.clone())?)? } else { py::truediv(&Py::Int(1i64), &py::mul(&py::mul(&Py::Int(2i64), &v_R.clone())?, &v_C.clone())?)? });
    let t1507 = (Py::tuple(vec![py::add(&py::neg(&v_a.clone())?, &py::math1("sqrt", &py::add(&py::mul(&v_a.clone(), &v_a.clone())?, &py::mul(&v_w0.clone(), &v_w0.clone())?)?)?)?, py::add(&v_a.clone(), &py::math1("sqrt", &py::add(&py::mul(&v_a.clone(), &v_a.clone())?, &py::mul(&v_w0.clone(), &v_w0.clone())?)?)?)?])).items()?;
    if t1507.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_w1 = t1507[0].clone();
    v_w2 = t1507[1].clone();
    v_tp = py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?;
    return Ok(Py::dict(vec![(Py::str("f0"), py::truediv(&v_w0.clone(), &v_tp.clone())?), (Py::str("Q"), py::truediv(&v_w0.clone(), &py::mul(&Py::Int(2i64), &v_a.clone())?)?), (Py::str("bandwidth"), py::truediv(&py::sub(&v_w2.clone(), &v_w1.clone())?, &v_tp.clone())?), (Py::str("f_low"), py::truediv(&v_w1.clone(), &v_tp.clone())?), (Py::str("f_high"), py::truediv(&v_w2.clone(), &v_tp.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__roots(mut v_coeffs: Py) -> R {
    let mut v__: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_biggest: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_dp: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_ratio: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_x: Py = Py::None;
    let mut v_z: Py = Py::None;
    let mut v_zeros: Py = Py::None;
    v_c = f__ptrim(v_coeffs.clone())?;
    if ({ let t1635 = py::len(&v_c.clone())?; Py::Bool({ let t1636 = Py::Int(2i64); if py::lt(&t1635, &t1636)? { true } else { false } }) }).truthy() {
        return Ok(Py::list(vec![]));
    }
    v_zeros = Py::Int(0i64);
    while ({ let t1641 = { let t1637 = py::len(&v_c.clone())?; Py::Bool({ let t1638 = Py::Int(1i64); if py::gt(&t1637, &t1638)? { true } else { false } }) }; if !t1641.truthy() { t1641 } else { { let t1639 = py::getitem(&v_c.clone(), &py::neg(&Py::Int(1i64))?)?; Py::Bool({ let t1640 = Py::Int(0i64); if py::eq(&t1639, &t1640) { true } else { false } }) } } }).truthy() {
        let _ = py::method(&v_c.clone(), "pop", &[])?;
        v_zeros = py::iadd(&v_zeros, &Py::Int(1i64))?;
    }
    v_n = py::sub(&py::len(&v_c.clone())?, &Py::Int(1i64))?;
    if ({ let t1642 = v_n.clone(); Py::Bool({ let t1643 = Py::Int(0i64); if py::eq(&t1642, &t1643) { true } else { false } }) }).truthy() {
        return Ok(py::mul(&Py::list(vec![Py::Float(0.0)]), &v_zeros.clone())?);
    }
    v_a = { let mut out = vec![]; for t1644 in py::iter(&(v_c.clone()))? { let mut v_x: Py; 
v_x = t1644;
out.push(py::truediv(&v_x.clone(), &py::getitem(&v_c.clone(), &Py::Int(0i64))?)?); } Py::list(out) };
    v_d = f__pder(v_a.clone())?;
    v_r = { let t1648 = py::pow(&py::abs(&py::getitem(&v_a.clone(), &py::neg(&Py::Int(1i64))?)?)?, &py::truediv(&Py::Float(1.0), &v_n.clone())?)?; if t1648.truthy() { t1648 } else { Py::Float(1.0) } };
    v_z = { let mut out = vec![]; for t1649 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_k: Py; 
v_k = t1649;
out.push(py::mul(&v_r.clone(), &py::cmath("exp", &[py::mul(&Py::Complex(0.0, 1.0), &py::add(&py::truediv(&py::mul(&py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?, &v_k.clone())?, &v_n.clone())?, &Py::Float(0.4))?)?])?)?); } Py::list(out) };
    for t1650 in py::range_iter(&[(Py::Int(800i64)).i()?]) {
        v__ = t1650;
        v_biggest = Py::Float(0.0);
        for t1651 in py::range_iter(&[(v_n.clone()).i()?]) {
            v_i = t1651;
            v_p = f__pval(v_a.clone(), py::getitem(&v_z.clone(), &v_i.clone())?)?;
            if ({ let t1653 = v_p.clone(); Py::Bool({ let t1654 = Py::Int(0i64); if py::eq(&t1653, &t1654) { true } else { false } }) }).truthy() {
                continue;
            }
            v_dp = f__pval(v_d.clone(), py::getitem(&v_z.clone(), &v_i.clone())?)?;
            v_ratio = (if ({ let t1655 = v_dp.clone(); Py::Bool({ let t1656 = Py::Int(0i64); if py::ne(&t1655, &t1656) { true } else { false } }) }).truthy() { py::truediv(&v_p.clone(), &v_dp.clone())? } else { py::mul(&v_p.clone(), &Py::Float(0.001))? });
            v_s = py::sum(&{ let mut out = vec![]; for t1657 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t1657;
if ({ let t1662 = { let t1658 = v_j.clone(); Py::Bool({ let t1659 = v_i.clone(); if py::ne(&t1658, &t1659) { true } else { false } }) }; if !t1662.truthy() { t1662 } else { { let t1660 = py::getitem(&v_z.clone(), &v_i.clone())?; Py::Bool({ let t1661 = py::getitem(&v_z.clone(), &v_j.clone())?; if py::ne(&t1660, &t1661) { true } else { false } }) } } }).truthy() { out.push(py::truediv(&Py::Float(1.0), &py::sub(&py::getitem(&v_z.clone(), &v_i.clone())?, &py::getitem(&v_z.clone(), &v_j.clone())?)?)?); }} Py::list(out) }, None)?;
            v_w = py::truediv(&v_ratio.clone(), &py::sub(&Py::Int(1i64), &py::mul(&v_ratio.clone(), &v_s.clone())?)?)?;
            let t1663 = v_z.clone(); let t1664 = v_i.clone();
            py::setitem(&t1663, &t1664, py::sub(&py::getitem(&t1663, &t1664)?, &v_w.clone())?)?;
            v_biggest = py::minmax(&[v_biggest.clone(), py::truediv(&py::abs(&v_w.clone())?, &py::minmax(&[Py::Float(1.0), py::abs(&py::getitem(&v_z.clone(), &v_i.clone())?)?], true)?)?], true)?;
        }
        if ({ let t1665 = v_biggest.clone(); Py::Bool({ let t1666 = Py::Float(1e-16); if py::lt(&t1665, &t1666)? { true } else { false } }) }).truthy() {
            break;
        }
    }
    v_out = Py::list(vec![]);
    for t1667 in py::iter(&(v_z.clone()))? {
        v_x = t1667;
        for t1668 in py::range_iter(&[(Py::Int(2i64)).i()?]) {
            v__ = t1668;
            v_dp = f__pval(v_d.clone(), v_x.clone())?;
            if ({ let t1669 = v_dp.clone(); Py::Bool({ let t1670 = Py::Int(0i64); if py::ne(&t1669, &t1670) { true } else { false } }) }).truthy() {
                v_x = py::sub(&v_x.clone(), &py::truediv(&f__pval(v_a.clone(), v_x.clone())?, &v_dp.clone())?)?;
            }
        }
        let _ = py::method(&v_out.clone(), "append", &[v_x.clone()])?;
    }
    return Ok(py::add(&v_out.clone(), &py::mul(&Py::list(vec![Py::Float(0.0)]), &v_zeros.clone())?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__snap(mut v_cs: Py) -> R {
    let mut v_s: Py = Py::None;
    v_s = { let t = { let mut out = vec![]; for t1569 in py::iter(&(v_cs.clone()))? { let mut v_c: Py; 
v_c = t1569;
out.push(py::abs(&v_c.clone())?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } };
    return Ok({ let mut out = vec![]; for t1570 in py::iter(&(v_cs.clone()))? { let mut v_c: Py; 
v_c = t1570;
out.push((if ({ let t1571 = py::abs(&v_c.clone())?; Py::Bool({ let t1572 = py::mul(&Py::Float(1e-12), &v_s.clone())?; if py::le(&t1571, &t1572)? { true } else { false } }) }).truthy() { Py::Float(0.0) } else { v_c.clone() })); } Py::list(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__sop_check(mut v_text: Py, mut v_n: Py, mut v_names: Py, mut v_on: Py, mut v_dc: Py) -> R {
    let mut v_bad: Py = Py::None;
    let mut v_env: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_terms: Py = Py::None;
    let mut v_tree: Py = Py::None;
    v_tree = f__bool_parse(v_text.clone())?;
    v_bad = Py::Int(0i64);
    for t2024 in py::range_iter(&[(py::pow(&Py::Int(2i64), &v_n.clone())?).i()?]) {
        v_m = t2024;
        if ({ let t2025 = v_m.clone(); Py::Bool({ let t2026 = v_dc.clone(); if py::contains(&t2026, &t2025)? { true } else { false } }) }).truthy() {
            continue;
        }
        v_env = { let mut out = vec![]; for t2027 in py::iter(&(py::enumerate(&v_names.clone(), 0)?))? { let mut v_i: Py; let mut v_nm: Py; 
let t2028 = (t2027).items()?;
if t2028.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_i = t2028[0].clone();
v_nm = t2028[1].clone();
out.push((v_nm.clone(), py::bitand(&py::rshift(&v_m.clone(), &py::sub(&py::sub(&v_n.clone(), &Py::Int(1i64))?, &v_i.clone())?)?, &Py::Int(1i64))?)); } Py::dict(out) };
        v_bad = py::iadd(&v_bad, &{ let t2029 = f__bool_eval(v_tree.clone(), v_env.clone())?; Py::Bool({ let t2030 = { let t2031 = v_m.clone(); Py::Bool({ let t2032 = v_on.clone(); if py::contains(&t2032, &t2031)? { true } else { false } }) }; if py::ne(&t2029, &t2030) { true } else { false } }) })?;
    }
    if ({ let t2033 = v_text.clone(); Py::Bool({ let t2034 = Py::tuple(vec![Py::str("0"), Py::str("1")]); if py::contains(&t2034, &t2033)? { true } else { false } }) }).truthy() {
        return Ok(Py::tuple(vec![v_bad.clone(), (if ({ let t2035 = v_text.clone(); Py::Bool({ let t2036 = Py::str("0"); if py::eq(&t2035, &t2036) { true } else { false } }) }).truthy() { Py::Int(0i64) } else { Py::Int(1i64) }), Py::Int(0i64)]));
    }
    v_terms = py::method(&v_text.clone(), "split", &[Py::str(" + ")])?;
    return Ok(Py::tuple(vec![v_bad.clone(), py::len(&v_terms.clone())?, py::sum(&{ let mut out = vec![]; for t2037 in py::iter(&(v_terms.clone()))? { let mut v_t: Py; 
v_t = t2037;
out.push(py::len(&py::method(&v_t.clone(), "split", &[Py::str("·")])?)?); } Py::list(out) }, None)?]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__sop_out(mut v_n: Py, mut v_names: Py, mut v_on: Py, mut v_dc: Py) -> R {
    let mut v_cover: Py = Py::None;
    let mut v_lits: Py = Py::None;
    let mut v_text: Py = Py::None;
    v_cover = f__min_cover(v_n.clone(), v_on.clone(), v_dc.clone())?;
    if (py::not(&v_cover.clone())).truthy() {
        v_text = Py::str("0");
    } else {
        if (Py::Bool(({ let mut out = vec![]; for t2009 in py::iter(&(v_cover.clone()))? { let mut v_p: Py; 
v_p = t2009;
out.push({ let t2010 = py::getitem(&v_p.clone(), &Py::Int(1i64))?; Py::Bool({ let t2011 = py::sub(&py::lshift(&Py::Int(1i64), &v_n.clone())?, &Py::Int(1i64))?; if py::eq(&t2010, &t2011) { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
            let t2012 = (Py::tuple(vec![Py::str("1"), Py::list(vec![Py::tuple(vec![Py::Int(0i64), py::sub(&py::lshift(&Py::Int(1i64), &v_n.clone())?, &Py::Int(1i64))?])])])).items()?;
            if t2012.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_text = t2012[0].clone();
            v_cover = t2012[1].clone();
        } else {
            v_text = py::method(&Py::str(" + "), "join", &[{ let mut out = vec![]; for t2013 in py::iter(&(v_cover.clone()))? { let mut v_p: Py; 
v_p = t2013;
out.push(f__term_text(v_p.clone(), v_names.clone())?); } Py::list(out) }])?;
        }
    }
    v_lits = py::sum(&{ let mut out = vec![]; for t2014 in py::iter(&(v_cover.clone()))? { let mut v_p: Py; 
v_p = t2014;
out.push(f__literals(v_p.clone(), v_n.clone())?); } Py::list(out) }, None)?;
    if ({ let t2015 = v_text.clone(); Py::Bool({ let t2016 = Py::tuple(vec![Py::str("0"), Py::str("1")]); if !py::contains(&t2016, &t2015)? { true } else { false } }) }).truthy() {
        v_text = py::method(&Py::str(" + "), "join", &[py::sorted(&py::method(&v_text.clone(), "split", &[Py::str(" + ")])?, (Py::Bool(false)).truthy())?])?;
    }
    return Ok(Py::dict(vec![(Py::str("minimal_sop"), v_text.clone()), (Py::str("terms"), py::float(&(if ({ let t2017 = v_text.clone(); Py::Bool({ let t2018 = Py::str("0"); if py::eq(&t2017, &t2018) { true } else { false } }) }).truthy() { Py::Int(0i64) } else { py::len(&v_cover.clone())? }))?), (Py::str("literals"), py::float(&(if ({ let t2019 = v_text.clone(); Py::Bool({ let t2020 = Py::str("1"); if py::eq(&t2019, &t2020) { true } else { false } }) }).truthy() { Py::Int(0i64) } else { v_lits.clone() }))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__term_text(mut v_imp: Py, mut v_names: Py) -> R {
    let mut v_lits: Py = Py::None;
    let mut v_mask: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_n = py::len(&v_names.clone())?;
    let t1965 = (v_imp.clone()).items()?;
    if t1965.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_v = t1965[0].clone();
    v_mask = t1965[1].clone();
    v_lits = { let mut out = vec![]; for t1966 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t1966;
if (py::not(&py::bitand(&py::rshift(&v_mask.clone(), &py::sub(&py::sub(&v_n.clone(), &Py::Int(1i64))?, &v_i.clone())?)?, &Py::Int(1i64))?)).truthy() { out.push(py::add(&py::getitem(&v_names.clone(), &v_i.clone())?, &(if (py::bitand(&py::rshift(&v_v.clone(), &py::sub(&py::sub(&v_n.clone(), &Py::Int(1i64))?, &v_i.clone())?)?, &Py::Int(1i64))?).truthy() { Py::str("") } else { Py::str("'") }))?); }} Py::list(out) };
    return Ok((if (v_lits.clone()).truthy() { py::method(&Py::str("·"), "join", &[v_lits.clone()])? } else { Py::str("1") }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__thevenin(mut v_netlist: Py, mut v_node_a: Py, mut v_node_b: Py) -> R {
    let mut v__: Py = Py::None;
    let mut v_dead: Py = Py::None;
    let mut v_els: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_nodes: Py = Py::None;
    let mut v_rth: Py = Py::None;
    let mut v_scale: Py = Py::None;
    let mut v_v2: Py = Py::None;
    let mut v_volts: Py = Py::None;
    let mut v_vth: Py = Py::None;
    v_els = f__netlist(v_netlist.clone())?;
    v_nodes = py::set(&{ let mut out = vec![]; for t1765 in py::iter(&(v_els.clone()))? { let mut v_e: Py; 
v_e = t1765;
for t1766 in py::iter(&(Py::tuple(vec![py::getitem(&v_e.clone(), &Py::Int(2i64))?, py::getitem(&v_e.clone(), &Py::Int(3i64))?])))? { let mut v_n: Py; 
v_n = t1766;
out.push(v_n.clone()); }} Py::list(out) })?;
    for t1767 in py::iter(&(Py::tuple(vec![v_node_a.clone(), v_node_b.clone()])))? {
        v_n = t1767;
        if ({ let t1768 = v_n.clone(); Py::Bool({ let t1769 = v_nodes.clone(); if !py::contains(&t1769, &t1768)? { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str(&["node ".to_string(), py::format(&v_n.clone(), "")?, " is not in the circuit".to_string()].concat()))));
        }
    }
    if ({ let t1770 = v_node_a.clone(); Py::Bool({ let t1771 = v_node_b.clone(); if py::eq(&t1770, &t1771) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the two terminals must be different nodes"))));
    }
    let t1857 = (f__mna(v_els.clone())?).items()?;
    if t1857.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_volts = t1857[0].clone();
    v__ = t1857[1].clone();
    v_vth = py::sub(&py::getitem(&v_volts.clone(), &v_node_a.clone())?, &py::getitem(&v_volts.clone(), &v_node_b.clone())?)?;
    if ({ let t1858 = py::abs(&v_vth.clone())?; Py::Bool({ let t1859 = py::mul(&Py::Float(1e-12), &py::minmax(&[{ let mut out = vec![]; for t1860 in py::iter(&(py::method(&v_volts.clone(), "values", &[])?))? { let mut v_v: Py; 
v_v = t1860;
out.push(py::abs(&v_v.clone())?); } Py::list(out) }], true)?)?; if py::le(&t1858, &t1859)? { true } else { false } }) }).truthy() {
        v_vth = Py::Float(0.0);
    }
    v_dead = py::add(&{ let mut out = vec![]; for t1861 in py::iter(&(v_els.clone()))? { let mut v_a: Py; let mut v_b: Py; let mut v_n: Py; let mut v_t: Py; let mut v_v: Py; 
let t1862 = (t1861).items()?;
if t1862.len() != 5 { return Err(PyErr::Value("unpack".into())); }
v_t = t1862[0].clone();
v_n = t1862[1].clone();
v_a = t1862[2].clone();
v_b = t1862[3].clone();
v_v = t1862[4].clone();
if ({ let t1863 = v_t.clone(); Py::Bool({ let t1864 = Py::str("V"); if py::eq(&t1863, &t1864) { true } else { false } }) }).truthy() { out.push(Py::tuple(vec![v_t.clone(), v_n.clone(), v_a.clone(), v_b.clone(), Py::Float(0.0)])); }} Py::list(out) }, &{ let mut out = vec![]; for t1865 in py::iter(&(v_els.clone()))? { let mut v_e: Py; 
v_e = t1865;
if ({ let t1866 = py::getitem(&v_e.clone(), &Py::Int(0i64))?; Py::Bool({ let t1867 = Py::str("R"); if py::eq(&t1866, &t1867) { true } else { false } }) }).truthy() { out.push(v_e.clone()); }} Py::list(out) })?;
    let _ = py::method(&v_dead.clone(), "append", &[Py::tuple(vec![Py::str("I"), Py::str("Itest"), v_node_b.clone(), v_node_a.clone(), Py::Float(1.0)])])?;
    let t1868 = (f__mna(v_dead.clone())?).items()?;
    if t1868.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_v2 = t1868[0].clone();
    v__ = t1868[1].clone();
    v_rth = py::sub(&py::getitem(&v_v2.clone(), &v_node_a.clone())?, &py::getitem(&v_v2.clone(), &v_node_b.clone())?)?;
    v_scale = py::minmax(&[{ let mut out = vec![]; for t1869 in py::iter(&(v_els.clone()))? { let mut v_e: Py; 
v_e = t1869;
if ({ let t1870 = py::getitem(&v_e.clone(), &Py::Int(0i64))?; Py::Bool({ let t1871 = Py::str("R"); if py::eq(&t1870, &t1871) { true } else { false } }) }).truthy() { out.push(py::getitem(&v_e.clone(), &Py::Int(4i64))?); }} Py::list(out) }], true)?;
    if ({ let t1872 = py::abs(&v_rth.clone())?; Py::Bool({ let t1873 = py::mul(&Py::Float(1e-12), &v_scale.clone())?; if py::le(&t1872, &t1873)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("an ideal voltage source is across the terminals: the Thevenin resistance is 0"))));
    }
    return Ok(Py::tuple(vec![v_vth.clone(), v_rth.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_3ph(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_R: Py = Py::None;
    let mut v_S: Py = Py::None;
    let mut v_VL: Py = Py::None;
    let mut v_X: Py = Py::None;
    let mut v_Z: Py = Py::None;
    let mut v_want_vph: Py = Py::None;
    let t1894 = (Py::tuple(vec![py::getitem(&v_inputs.clone(), &Py::str("VL"))?, py::getitem(&v_inputs.clone(), &Py::str("R"))?, py::getitem(&v_inputs.clone(), &Py::str("X"))?])).items()?;
    if t1894.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_VL = t1894[0].clone();
    v_R = t1894[1].clone();
    v_X = t1894[2].clone();
    v_Z = py::abs(&py::complex(&[v_R.clone(), v_X.clone()])?)?;
    v_S = py::mul(&py::mul(&py::math1("sqrt", &Py::Int(3i64))?, &v_VL.clone())?, &py::getitem(&v_outputs.clone(), &Py::str("line_current"))?)?;
    v_want_vph = (if ({ let t1895 = py::getitem(&v_inputs.clone(), &Py::str("connection"))?; Py::Bool({ let t1896 = Py::str("wye"); if py::eq(&t1895, &t1896) { true } else { false } }) }).truthy() { py::truediv(&v_VL.clone(), &py::math1("sqrt", &Py::Int(3i64))?)? } else { v_VL.clone() });
    return Ok(py::add(&py::add(&py::add(&py::add(&py::add(&f__rel(py::getitem(&v_outputs.clone(), &Py::str("phase_voltage"))?, v_want_vph.clone())?, &f__rel(py::mul(&py::getitem(&v_outputs.clone(), &Py::str("phase_current"))?, &v_Z.clone())?, py::getitem(&v_outputs.clone(), &Py::str("phase_voltage"))?)?)?, &f__rel(py::getitem(&v_outputs.clone(), &Py::str("S"))?, v_S.clone())?)?, &py::truediv(&py::abs(&py::sub(&py::getitem(&v_outputs.clone(), &Py::str("P"))?, &py::truediv(&py::mul(&v_S.clone(), &v_R.clone())?, &v_Z.clone())?)?)?, &v_S.clone())?)?, &py::truediv(&py::abs(&py::sub(&py::getitem(&v_outputs.clone(), &Py::str("Q"))?, &py::truediv(&py::mul(&v_S.clone(), &v_X.clone())?, &v_Z.clone())?)?)?, &v_S.clone())?)?, &py::abs(&py::sub(&py::getitem(&v_outputs.clone(), &Py::str("power_factor"))?, &py::truediv(&v_R.clone(), &v_Z.clone())?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_base(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_b: Py = Py::None;
    let mut v_n: Py = Py::None;
    v_b = py::getitem(&c__BASES()?, &Py::str(&py::to_str(&py::getitem(&v_inputs.clone(), &Py::str("base"))?)))?;
    v_n = py::int_base(&f__clean_digits(py::getitem(&v_inputs.clone(), &Py::str("value"))?, v_b.clone())?, &v_b.clone())?;
    return Ok(py::add(&py::add(&py::add(&py::truediv(&py::abs(&py::sub(&py::getitem(&v_outputs.clone(), &Py::str("decimal"))?, &v_n.clone())?)?, &py::minmax(&[Py::Int(1i64), v_n.clone()], true)?)?, &{ let t33 = py::int_base(&py::getitem(&v_outputs.clone(), &Py::str("binary"))?, &Py::Int(2i64))?; Py::Bool({ let t34 = v_n.clone(); if py::ne(&t33, &t34) { true } else { false } }) })?, &{ let t35 = py::int_base(&py::getitem(&v_outputs.clone(), &Py::str("octal"))?, &Py::Int(8i64))?; Py::Bool({ let t36 = v_n.clone(); if py::ne(&t35, &t36) { true } else { false } }) })?, &{ let t37 = py::int_base(&py::getitem(&v_outputs.clone(), &Py::str("hexadecimal"))?, &Py::Int(16i64))?; Py::Bool({ let t38 = v_n.clone(); if py::ne(&t37, &t38) { true } else { false } }) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_bjt(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_R1: Py = Py::None;
    let mut v_R2: Py = Py::None;
    let mut v_RC: Py = Py::None;
    let mut v_RE: Py = Py::None;
    let mut v_VBE: Py = Py::None;
    let mut v_VCC: Py = Py::None;
    let mut v_beta: Py = Py::None;
    let mut v_i1: Py = Py::None;
    let mut v_i2: Py = Py::None;
    let mut v_ib: Py = Py::None;
    let mut v_ic: Py = Py::None;
    let mut v_vb: Py = Py::None;
    let mut v_vc: Py = Py::None;
    let mut v_ve: Py = Py::None;
    let t50 = ({ let mut out = vec![]; for t49 in py::iter(&(Py::tuple(vec![Py::str("VCC"), Py::str("R1"), Py::str("R2"), Py::str("RC"), Py::str("RE"), Py::str("beta"), Py::str("VBE")])))? { let mut v_k: Py; 
v_k = t49;
out.push(py::getitem(&v_inputs.clone(), &v_k.clone())?); } Py::list(out) }).items()?;
    if t50.len() != 7 { return Err(PyErr::Value("unpack".into())); }
    v_VCC = t50[0].clone();
    v_R1 = t50[1].clone();
    v_R2 = t50[2].clone();
    v_RC = t50[3].clone();
    v_RE = t50[4].clone();
    v_beta = t50[5].clone();
    v_VBE = t50[6].clone();
    let t51 = (Py::tuple(vec![py::getitem(&v_outputs.clone(), &Py::str("IB"))?, py::getitem(&v_outputs.clone(), &Py::str("VB"))?, py::getitem(&v_outputs.clone(), &Py::str("VE"))?, py::getitem(&v_outputs.clone(), &Py::str("VC"))?])).items()?;
    if t51.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_ib = t51[0].clone();
    v_vb = t51[1].clone();
    v_ve = t51[2].clone();
    v_vc = t51[3].clone();
    let t52 = (Py::tuple(vec![py::truediv(&py::sub(&v_VCC.clone(), &v_vb.clone())?, &v_R1.clone())?, py::truediv(&v_vb.clone(), &v_R2.clone())?])).items()?;
    if t52.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_i1 = t52[0].clone();
    v_i2 = t52[1].clone();
    v_ic = py::getitem(&v_outputs.clone(), &Py::str("IC"))?;
    return Ok(py::add(&py::add(&py::add(&py::add(&py::add(&py::add(&py::truediv(&py::abs(&py::sub(&py::sub(&v_i1.clone(), &v_i2.clone())?, &v_ib.clone())?)?, &py::minmax(&[v_i1.clone(), Py::Float(1e-300)], true)?)?, &py::truediv(&py::abs(&py::sub(&py::sub(&v_vb.clone(), &v_ve.clone())?, &v_VBE.clone())?)?, &v_VCC.clone())?)?, &f__rel(v_ic.clone(), py::mul(&v_beta.clone(), &v_ib.clone())?)?)?, &py::truediv(&py::abs(&py::sub(&v_ve.clone(), &py::mul(&py::mul(&py::add(&v_beta.clone(), &Py::Int(1i64))?, &v_ib.clone())?, &v_RE.clone())?)?)?, &v_VCC.clone())?)?, &py::truediv(&py::abs(&py::sub(&py::sub(&v_VCC.clone(), &py::mul(&v_ic.clone(), &v_RC.clone())?)?, &v_vc.clone())?)?, &v_VCC.clone())?)?, &f__rel(py::getitem(&v_outputs.clone(), &Py::str("IC_mA"))?, py::mul(&v_ic.clone(), &Py::Float(1000.0))?)?)?, &py::truediv(&py::abs(&py::sub(&py::getitem(&v_outputs.clone(), &Py::str("VCE"))?, &py::sub(&v_vc.clone(), &v_ve.clone())?)?)?, &v_VCC.clone())?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_bode(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_den: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_num: Py = Py::None;
    let t71 = (Py::tuple(vec![f__ptrim(py::getitem(&v_inputs.clone(), &Py::str("num"))?)?, f__ptrim(py::getitem(&v_inputs.clone(), &Py::str("den"))?)?])).items()?;
    if t71.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_num = t71[0].clone();
    v_den = t71[1].clone();
    v_g = f__G(v_num.clone(), v_den.clone(), py::getitem(&v_inputs.clone(), &Py::str("w"))?)?;
    v_d = py::sub(&py::math1("degrees", &py::cmath("phase", &[v_g.clone()])?)?, &py::getitem(&v_outputs.clone(), &Py::str("phase_deg"))?)?;
    v_d = py::sub(&py::modulo(&py::add(&v_d.clone(), &Py::Int(180i64))?, &Py::Int(360i64))?, &Py::Int(180i64))?;
    return Ok(py::add(&py::add(&py::add(&f__rel(py::getitem(&v_outputs.clone(), &Py::str("magnitude"))?, py::abs(&v_g.clone())?)?, &py::truediv(&py::abs(&py::sub(&py::getitem(&v_outputs.clone(), &Py::str("magnitude_db"))?, &py::mul(&Py::Int(20i64), &py::math1("log10", &py::abs(&v_g.clone())?)?)?)?)?, &py::minmax(&[Py::Float(1.0), py::abs(&py::getitem(&v_outputs.clone(), &Py::str("magnitude_db"))?)?], true)?)?)?, &py::truediv(&py::abs(&v_d.clone())?, &Py::Int(180i64))?)?, &py::truediv(&py::abs(&py::sub(&py::complex(&[py::method(&v_outputs.clone(), "get", &[Py::str("real"), py::real(&v_g.clone())?])?, py::method(&v_outputs.clone(), "get", &[Py::str("imag"), py::imag(&v_g.clone())?])?])?, &v_g.clone())?)?, &py::abs(&v_g.clone())?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_co(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_Wc: Py = Py::None;
    let mut v_Wo: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_rc: Py = Py::None;
    let mut v_ro: Py = Py::None;
    let t339 = (f__ctrb_obsv(py::getitem(&v_inputs.clone(), &Py::str("A"))?, py::getitem(&v_inputs.clone(), &Py::str("B"))?, py::getitem(&v_inputs.clone(), &Py::str("C"))?)?).items()?;
    if t339.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_Wc = t339[0].clone();
    v_Wo = t339[1].clone();
    v_n = py::len(&py::getitem(&v_inputs.clone(), &Py::str("A"))?)?;
    let t342 = (Py::tuple(vec![f__rank({ let mut out = vec![]; for t340 in py::iter(&(py::zip(&{ let mut v: Vec<Py> = vec![]; v.extend((v_Wc.clone()).items()?); v })?))? { let mut v_x: Py; 
v_x = t340;
out.push(Py::list((v_x.clone()).items()?)); } Py::list(out) }, Py::Float(1e-09))?, f__rank({ let mut out = vec![]; for t341 in py::iter(&(py::zip(&{ let mut v: Vec<Py> = vec![]; v.extend((v_Wo.clone()).items()?); v })?))? { let mut v_x: Py; 
v_x = t341;
out.push(Py::list((v_x.clone()).items()?)); } Py::list(out) }, Py::Float(1e-09))?])).items()?;
    if t342.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_rc = t342[0].clone();
    v_ro = t342[1].clone();
    return Ok(py::add(&py::add(&py::add(&py::abs(&py::sub(&py::getitem(&v_outputs.clone(), &Py::str("controllability_rank"))?, &v_rc.clone())?)?, &py::abs(&py::sub(&py::getitem(&v_outputs.clone(), &Py::str("observability_rank"))?, &v_ro.clone())?)?)?, &{ let t343 = py::getitem(&v_outputs.clone(), &Py::str("controllable"))?; Py::Bool({ let t344 = { let t345 = v_rc.clone(); Py::Bool({ let t346 = v_n.clone(); if py::eq(&t345, &t346) { true } else { false } }) }; if py::ne(&t343, &t344) { true } else { false } }) })?, &{ let t347 = py::getitem(&v_outputs.clone(), &Py::str("observable"))?; Py::Bool({ let t348 = { let t349 = v_ro.clone(); Py::Bool({ let t350 = v_n.clone(); if py::eq(&t349, &t350) { true } else { false } }) }; if py::ne(&t347, &t348) { true } else { false } }) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_cpower(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_I: Py = Py::None;
    let mut v_S: Py = Py::None;
    let mut v_V: Py = Py::None;
    let mut v_th: Py = Py::None;
    let t296 = (Py::tuple(vec![py::getitem(&v_inputs.clone(), &Py::str("V"))?, py::getitem(&v_inputs.clone(), &Py::str("I"))?])).items()?;
    if t296.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_V = t296[0].clone();
    v_I = t296[1].clone();
    v_th = py::math1("radians", &py::sub(&py::getitem(&v_inputs.clone(), &Py::str("V_phase"))?, &py::getitem(&v_inputs.clone(), &Py::str("I_phase"))?)?)?;
    v_S = py::mul(&v_V.clone(), &v_I.clone())?;
    return Ok(py::add(&py::add(&py::add(&py::add(&py::truediv(&py::abs(&py::sub(&py::getitem(&v_outputs.clone(), &Py::str("P"))?, &py::mul(&v_S.clone(), &py::math1("cos", &v_th.clone())?)?)?)?, &v_S.clone())?, &py::truediv(&py::abs(&py::sub(&py::getitem(&v_outputs.clone(), &Py::str("Q"))?, &py::mul(&v_S.clone(), &py::math1("sin", &v_th.clone())?)?)?)?, &v_S.clone())?)?, &f__rel(py::getitem(&v_outputs.clone(), &Py::str("S"))?, v_S.clone())?)?, &py::abs(&py::sub(&py::getitem(&v_outputs.clone(), &Py::str("power_factor"))?, &py::truediv(&py::getitem(&v_outputs.clone(), &Py::str("P"))?, &py::getitem(&v_outputs.clone(), &Py::str("S"))?)?)?)?)?, &py::truediv(&py::abs(&py::sub(&py::add(&py::pow(&py::getitem(&v_outputs.clone(), &Py::str("P"))?, &Py::Int(2i64))?, &py::pow(&py::getitem(&v_outputs.clone(), &Py::str("Q"))?, &Py::Int(2i64))?)?, &py::pow(&py::getitem(&v_outputs.clone(), &Py::str("S"))?, &Py::Int(2i64))?)?)?, &py::pow(&v_S.clone(), &Py::Int(2i64))?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_diffamp(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_R1: Py = Py::None;
    let mut v_R2: Py = Py::None;
    let mut v_R3: Py = Py::None;
    let mut v_R4: Py = Py::None;
    let mut v_V1: Py = Py::None;
    let mut v_V2: Py = Py::None;
    let mut v_terms: Py = Py::None;
    let mut v_vo: Py = Py::None;
    let mut v_vp: Py = Py::None;
    let t529 = ({ let mut out = vec![]; for t528 in py::iter(&(Py::tuple(vec![Py::str("V1"), Py::str("V2"), Py::str("R1"), Py::str("R2"), Py::str("R3"), Py::str("R4")])))? { let mut v_k: Py; 
v_k = t528;
out.push(py::getitem(&v_inputs.clone(), &v_k.clone())?); } Py::list(out) }).items()?;
    if t529.len() != 6 { return Err(PyErr::Value("unpack".into())); }
    v_V1 = t529[0].clone();
    v_V2 = t529[1].clone();
    v_R1 = t529[2].clone();
    v_R2 = t529[3].clone();
    v_R3 = t529[4].clone();
    v_R4 = t529[5].clone();
    v_vo = py::getitem(&v_outputs.clone(), &Py::str("Vout"))?;
    v_vp = py::truediv(&py::mul(&v_V2.clone(), &v_R4.clone())?, &py::add(&v_R3.clone(), &v_R4.clone())?)?;
    v_terms = Py::list(vec![py::truediv(&py::sub(&v_V1.clone(), &v_vp.clone())?, &v_R1.clone())?, py::truediv(&py::sub(&v_vo.clone(), &v_vp.clone())?, &v_R2.clone())?]);
    return Ok(py::add(&py::truediv(&py::abs(&py::fsum(&v_terms.clone())?)?, &{ let t531 = py::fsum(&{ let mut out = vec![]; for t530 in py::iter(&(v_terms.clone()))? { let mut v_t: Py; 
v_t = t530;
out.push(py::abs(&v_t.clone())?); } Py::list(out) })?; if t531.truthy() { t531 } else { Py::Float(1.0) } })?, &f__rel(py::getitem(&v_outputs.clone(), &Py::str("gain_inverting"))?, py::truediv(&py::neg(&v_R2.clone())?, &v_R1.clone())?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_dw(mut v_inputs: Py, mut v_outputs: Py, mut v_forward: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_pd: Py = Py::None;
    let mut v_py: Py = Py::None;
    let mut v_y: Py = Py::None;
    if (v_forward.clone()).truthy() {
        let t2089 = (Py::tuple(vec![Py::tuple(vec![py::getitem(&v_inputs.clone(), &Py::str("Rab"))?, py::getitem(&v_inputs.clone(), &Py::str("Rbc"))?, py::getitem(&v_inputs.clone(), &Py::str("Rca"))?]), Py::tuple(vec![py::getitem(&v_outputs.clone(), &Py::str("Ra"))?, py::getitem(&v_outputs.clone(), &Py::str("Rb"))?, py::getitem(&v_outputs.clone(), &Py::str("Rc"))?])])).items()?;
        if t2089.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_d = t2089[0].clone();
        v_y = t2089[1].clone();
    } else {
        let t2090 = (Py::tuple(vec![Py::tuple(vec![py::getitem(&v_inputs.clone(), &Py::str("Ra"))?, py::getitem(&v_inputs.clone(), &Py::str("Rb"))?, py::getitem(&v_inputs.clone(), &Py::str("Rc"))?]), Py::tuple(vec![py::getitem(&v_outputs.clone(), &Py::str("Rab"))?, py::getitem(&v_outputs.clone(), &Py::str("Rbc"))?, py::getitem(&v_outputs.clone(), &Py::str("Rca"))?])])).items()?;
        if t2090.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_y = t2090[0].clone();
        v_d = t2090[1].clone();
    }
    v_pd = { let mut v: Vec<Py> = vec![]; v.extend((v_d.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![];  let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "Rab") { Some(x) => x, None => { return Err(PyErr::Type("_pair_resistances_delta() missing argument Rab".into())) } } }; let a1 = if 1 < v.len() { v[1].clone() } else { match py::kw_take(&mut d, "Rbc") { Some(x) => x, None => { return Err(PyErr::Type("_pair_resistances_delta() missing argument Rbc".into())) } } }; let a2 = if 2 < v.len() { v[2].clone() } else { match py::kw_take(&mut d, "Rca") { Some(x) => x, None => { return Err(PyErr::Type("_pair_resistances_delta() missing argument Rca".into())) } } }; if v.len() > 3 { return Err(PyErr::Type("_pair_resistances_delta() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("_pair_resistances_delta() got an unexpected keyword argument".into())) } f__pair_resistances_delta(a0, a1, a2)? };
    v_py = Py::dict(vec![(Py::str("ab"), py::add(&py::getitem(&v_y.clone(), &Py::Int(0i64))?, &py::getitem(&v_y.clone(), &Py::Int(1i64))?)?), (Py::str("bc"), py::add(&py::getitem(&v_y.clone(), &Py::Int(1i64))?, &py::getitem(&v_y.clone(), &Py::Int(2i64))?)?), (Py::str("ca"), py::add(&py::getitem(&v_y.clone(), &Py::Int(2i64))?, &py::getitem(&v_y.clone(), &Py::Int(0i64))?)?)]);
    return Ok(py::minmax(&[{ let mut out = vec![]; for t2091 in py::iter(&(v_pd.clone()))? { let mut v_k: Py; 
v_k = t2091;
out.push(f__rel(py::getitem(&v_pd.clone(), &v_k.clone())?, py::getitem(&v_py.clone(), &v_k.clone())?)?); } Py::list(out) }], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_equiv(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_env: Py = Py::None;
    let mut v_t1: Py = Py::None;
    let mut v_t2: Py = Py::None;
    let t132 = (Py::tuple(vec![f__bool_parse(py::getitem(&v_inputs.clone(), &Py::str("expression1"))?)?, f__bool_parse(py::getitem(&v_inputs.clone(), &Py::str("expression2"))?)?])).items()?;
    if t132.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_t1 = t132[0].clone();
    v_t2 = t132[1].clone();
    if (py::not(&Py::Bool(matches!(py::getitem(&v_outputs.clone(), &Py::str("equivalent"))?, Py::Bool(_))))).truthy() {
        return Ok(Py::Float(1.0));
    }
    if (py::getitem(&v_outputs.clone(), &Py::str("equivalent"))?).truthy() {
        return Ok(py::float(&py::not(&py::getitem(&f_boolean_equivalence(py::getitem(&v_inputs.clone(), &Py::str("expression1"))?, py::getitem(&v_inputs.clone(), &Py::str("expression2"))?)?, &Py::str("equivalent"))?))?);
    }
    v_env = f__assignments(py::getitem(&v_outputs.clone(), &Py::str("counterexample"))?)?;
    let t139: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            return Ok(Some(py::float(&{ let t137 = f__bool_eval(v_t1.clone(), v_env.clone())?; Py::Bool({ let t138 = f__bool_eval(v_t2.clone(), v_env.clone())?; if py::eq(&t137, &t138) { true } else { false } }) })?));
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t139 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_gm(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_den: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_num: Py = Py::None;
    let mut v_w: Py = Py::None;
    let t656 = (Py::tuple(vec![f__ptrim(py::getitem(&v_inputs.clone(), &Py::str("num"))?)?, f__ptrim(py::getitem(&v_inputs.clone(), &Py::str("den"))?)?])).items()?;
    if t656.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_num = t656[0].clone();
    v_den = t656[1].clone();
    v_w = py::getitem(&v_outputs.clone(), &Py::str("phase_crossover"))?;
    v_g = f__G(v_num.clone(), v_den.clone(), v_w.clone())?;
    return Ok(py::add(&py::add(&py::add(&py::truediv(&py::abs(&py::imag(&v_g.clone())?)?, &py::abs(&v_g.clone())?)?, &{ let t657 = py::real(&v_g.clone())?; Py::Bool({ let t658 = Py::Int(0i64); if py::ge(&t657, &t658)? { true } else { false } }) })?, &f__rel(py::getitem(&v_outputs.clone(), &Py::str("gain_margin"))?, py::truediv(&Py::Float(1.0), &py::abs(&v_g.clone())?)?)?)?, &py::truediv(&py::abs(&py::sub(&py::getitem(&v_outputs.clone(), &Py::str("gain_margin_db"))?, &py::mul(&Py::Int(20i64), &py::math1("log10", &py::getitem(&v_outputs.clone(), &Py::str("gain_margin"))?)?)?)?)?, &py::minmax(&[Py::Float(1.0), py::abs(&py::getitem(&v_outputs.clone(), &Py::str("gain_margin_db"))?)?], true)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_gray(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_b: Py = Py::None;
    let mut v_bi: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_gi: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_v: Py = Py::None;
    let t666 = (Py::tuple(vec![py::method(&py::getitem(&v_inputs.clone(), &Py::str("value"))?, "strip", &[])?, py::getitem(&v_outputs.clone(), &Py::str("result"))?])).items()?;
    if t666.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_v = t666[0].clone();
    v_r = t666[1].clone();
    if ({ let t667 = py::len(&v_r.clone())?; Py::Bool({ let t668 = py::len(&v_v.clone())?; if py::ne(&t667, &t668) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(1.0));
    }
    let t671 = ((if ({ let t669 = py::getitem(&v_inputs.clone(), &Py::str("direction"))?; Py::Bool({ let t670 = Py::str("to_gray"); if py::eq(&t669, &t670) { true } else { false } }) }).truthy() { Py::tuple(vec![v_r.clone(), v_v.clone()]) } else { Py::tuple(vec![v_v.clone(), v_r.clone()]) })).items()?;
    if t671.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_g = t671[0].clone();
    v_b = t671[1].clone();
    let t672 = (Py::tuple(vec![py::int_base(&v_g.clone(), &Py::Int(2i64))?, py::int_base(&v_b.clone(), &Py::Int(2i64))?])).items()?;
    if t672.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_gi = t672[0].clone();
    v_bi = t672[1].clone();
    return Ok(py::add(&py::float(&{ let t673 = v_gi.clone(); Py::Bool({ let t674 = py::bitxor(&v_bi.clone(), &py::rshift(&v_bi.clone(), &Py::Int(1i64))?)?; if py::ne(&t673, &t674) { true } else { false } }) })?, &py::abs(&py::sub(&py::getitem(&v_outputs.clone(), &Py::str("decimal"))?, &py::int_base(&v_r.clone(), &Py::Int(2i64))?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_m2s(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_bad: Py = Py::None;
    let mut v_best: Py = Py::None;
    let mut v_dc: Py = Py::None;
    let mut v_lit: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_t: Py = Py::None;
    v_n = py::int(&py::getitem(&v_inputs.clone(), &Py::str("variables"))?)?;
    v_dc = py::set(&{ let mut out = vec![]; for t817 in py::iter(&({ let t816 = py::method(&v_inputs.clone(), "get", &[Py::str("dont_cares")])?; if t816.truthy() { t816 } else { Py::list(vec![]) } }))? { let mut v_m: Py; 
v_m = t817;
out.push(py::int(&v_m.clone())?); } Py::list(out) })?;
    let t874 = (f__sop_check(py::getitem(&v_outputs.clone(), &Py::str("minimal_sop"))?, v_n.clone(), py::slice(&Py::str("ABCD"), None, Some((v_n.clone()).i()?), None)?, py::set(&{ let mut out = vec![]; for t873 in py::iter(&(py::getitem(&v_inputs.clone(), &Py::str("minterms"))?))? { let mut v_m: Py; 
v_m = t873;
out.push(py::int(&v_m.clone())?); } Py::list(out) })?, v_dc.clone())?).items()?;
    if t874.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_bad = t874[0].clone();
    v_t = t874[1].clone();
    v_lit = t874[2].clone();
    v_best = f__sop_out(v_n.clone(), py::slice(&Py::str("ABCD"), None, Some((v_n.clone()).i()?), None)?, py::sorted(&py::set(&{ let mut out = vec![]; for t875 in py::iter(&(py::getitem(&v_inputs.clone(), &Py::str("minterms"))?))? { let mut v_m: Py; 
v_m = t875;
out.push(py::int(&v_m.clone())?); } Py::list(out) })?, (Py::Bool(false)).truthy())?, py::sorted(&v_dc.clone(), (Py::Bool(false)).truthy())?)?;
    return Ok(py::add(&py::add(&py::add(&py::add(&v_bad.clone(), &py::abs(&py::sub(&py::getitem(&v_outputs.clone(), &Py::str("terms"))?, &v_t.clone())?)?)?, &py::abs(&py::sub(&py::getitem(&v_outputs.clone(), &Py::str("literals"))?, &v_lit.clone())?)?)?, &py::abs(&py::sub(&py::getitem(&v_outputs.clone(), &Py::str("terms"))?, &py::getitem(&v_best.clone(), &Py::str("terms"))?)?)?)?, &py::abs(&py::sub(&py::getitem(&v_outputs.clone(), &Py::str("literals"))?, &py::getitem(&v_best.clone(), &Py::str("literals"))?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_network(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_tree: Py = Py::None;
    let mut v_walk: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_tree = f__net_tree(py::getitem(&v_inputs.clone(), &Py::str("network"))?)?;
    v_worst = Py::list(vec![Py::Float(0.0)]);
    let rec1328 = py::rec_cell();
    v_walk = { let v_inputs = v_inputs.clone(); let v_outputs = v_outputs.clone(); let v_tree = v_tree.clone(); let v_worst = v_worst.clone(); let rec1328 = rec1328.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_t: Py = a.get(0).cloned().unwrap_or(Py::None);
            let v_walk: Py = py::rec_get(&rec1328)?;
            let mut v_eq: Py = Py::None;
            let mut v_vals: Py = Py::None;
            if (Py::Bool(matches!(v_t.clone(), Py::Float(_)))).truthy() {
                return Ok(v_t.clone());
            }
            v_vals = { let mut out = vec![]; for t1329 in py::iter(&(py::getitem(&v_t.clone(), &Py::Int(1i64))?))? { let mut v_x: Py; 
v_x = t1329;
out.push(py::call(&v_walk.clone(), &[v_x.clone()])?); } Py::list(out) };
            if ({ let t1330 = py::getitem(&v_t.clone(), &Py::Int(0i64))?; Py::Bool({ let t1331 = Py::str("s"); if py::eq(&t1330, &t1331) { true } else { false } }) }).truthy() {
                return Ok(py::fsum(&v_vals.clone())?);
            }
            v_eq = py::truediv(&Py::Float(1.0), &py::fsum(&{ let mut out = vec![]; for t1332 in py::iter(&(v_vals.clone()))? { let mut v_v: Py; 
v_v = t1332;
out.push(py::truediv(&Py::Float(1.0), &v_v.clone())?); } Py::list(out) })?)?;
            py::setitem(&v_worst.clone(), &Py::Int(0i64), py::minmax(&[py::getitem(&v_worst.clone(), &Py::Int(0i64))?, py::abs(&py::sub(&py::fsum(&{ let mut out = vec![]; for t1333 in py::iter(&(v_vals.clone()))? { let mut v_v: Py; 
v_v = t1333;
out.push(py::truediv(&v_eq.clone(), &v_v.clone())?); } Py::list(out) })?, &Py::Float(1.0))?)?], true)?)?;
            return Ok(v_eq.clone());
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    py::rec_set(&rec1328, &v_walk);
    v_want = py::call(&v_walk.clone(), &[v_tree.clone()])?;
    return Ok(py::minmax(&[py::getitem(&v_worst.clone(), &Py::Int(0i64))?, f__rel(py::getitem(&v_outputs.clone(), &Py::str("equivalent"))?, v_want.clone())?, py::abs(&py::sub(&py::method(&v_outputs.clone(), "get", &[Py::str("resistors"), f__net_count(v_tree.clone())?])?, &f__net_count(v_tree.clone())?)?)?], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_nodes(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_V: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_big: Py = Py::None;
    let mut v_cur: Py = Py::None;
    let mut v_diss: Py = Py::None;
    let mut v_els: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_kcl: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_name: Py = Py::None;
    let mut v_nodes: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_scale: Py = Py::None;
    let mut v_t: Py = Py::None;
    let mut v_terms: Py = Py::None;
    let mut v_val: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_els = f__netlist(py::getitem(&v_inputs.clone(), &Py::str("netlist"))?)?;
    v_V = { let t486 = Py::dict(vec![]); py::setitem(&t486, &Py::str("0"), Py::Float(0.0))?; py::kw_merge(&t486, &py::getitem(&v_outputs.clone(), &Py::str("node_voltages"))?)?; t486 };
    v_cur = py::getitem(&v_outputs.clone(), &Py::str("source_currents"))?;
    v_nodes = py::set(&{ let mut out = vec![]; for t487 in py::iter(&(v_els.clone()))? { let mut v_e: Py; 
v_e = t487;
for t488 in py::iter(&(Py::tuple(vec![py::getitem(&v_e.clone(), &Py::Int(2i64))?, py::getitem(&v_e.clone(), &Py::Int(3i64))?])))? { let mut v_n: Py; 
v_n = t488;
out.push(v_n.clone()); }} Py::list(out) })?;
    if ({ let t496 = { let t489 = py::set(&v_V.clone())?; Py::Bool({ let t490 = v_nodes.clone(); if py::ne(&t489, &t490) { true } else { false } }) }; if t496.truthy() { t496 } else { { let t491 = py::set(&v_cur.clone())?; Py::Bool({ let t492 = py::set(&{ let mut out = vec![]; for t493 in py::iter(&(v_els.clone()))? { let mut v_e: Py; 
v_e = t493;
if ({ let t494 = py::getitem(&v_e.clone(), &Py::Int(0i64))?; Py::Bool({ let t495 = Py::str("V"); if py::eq(&t494, &t495) { true } else { false } }) }).truthy() { out.push(py::getitem(&v_e.clone(), &Py::Int(1i64))?); }} Py::list(out) })?; if py::ne(&t491, &t492) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_kcl = { let mut out = vec![]; for t497 in py::iter(&(v_nodes.clone()))? { let mut v_n: Py; 
v_n = t497;
out.push((v_n.clone(), Py::list(vec![]))); } Py::dict(out) };
    let t498 = (Py::tuple(vec![Py::Float(0.0), Py::Float(0.0), Py::list(vec![])])).items()?;
    if t498.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_worst = t498[0].clone();
    v_diss = t498[1].clone();
    v_terms = t498[2].clone();
    for t499 in py::iter(&(v_els.clone()))? {
        let t504 = (t499).items()?;
        if t504.len() != 5 { return Err(PyErr::Value("unpack".into())); }
        v_t = t504[0].clone();
        v_name = t504[1].clone();
        v_a = t504[2].clone();
        v_b = t504[3].clone();
        v_val = t504[4].clone();
        if ({ let t500 = v_t.clone(); Py::Bool({ let t501 = Py::str("R"); if py::eq(&t500, &t501) { true } else { false } }) }).truthy() {
            v_i = py::truediv(&py::sub(&py::getitem(&v_V.clone(), &v_a.clone())?, &py::getitem(&v_V.clone(), &v_b.clone())?)?, &v_val.clone())?;
            v_diss = py::iadd(&v_diss, &py::mul(&py::mul(&v_i.clone(), &v_i.clone())?, &v_val.clone())?)?;
        } else {
            if ({ let t502 = v_t.clone(); Py::Bool({ let t503 = Py::str("V"); if py::eq(&t502, &t503) { true } else { false } }) }).truthy() {
                v_i = py::getitem(&v_cur.clone(), &v_name.clone())?;
                v_worst = py::minmax(&[v_worst.clone(), py::truediv(&py::abs(&py::sub(&py::sub(&py::getitem(&v_V.clone(), &v_a.clone())?, &py::getitem(&v_V.clone(), &v_b.clone())?)?, &v_val.clone())?)?, &py::minmax(&[py::abs(&v_val.clone())?, py::abs(&py::getitem(&v_V.clone(), &v_a.clone())?)?, py::abs(&py::getitem(&v_V.clone(), &v_b.clone())?)?, Py::Float(1e-12)], true)?)?], true)?;
            } else {
                v_i = v_val.clone();
            }
        }
        let _ = py::method(&py::getitem(&v_kcl.clone(), &v_a.clone())?, "append", &[v_i.clone()])?;
        let _ = py::method(&py::getitem(&v_kcl.clone(), &v_b.clone())?, "append", &[py::neg(&v_i.clone())?])?;
    }
    v_big = { let t507 = { let t = { let mut out = vec![]; for t505 in py::iter(&(py::method(&v_kcl.clone(), "values", &[])?))? { let mut v_ts: Py; 
v_ts = t505;
for t506 in py::iter(&(v_ts.clone()))? { let mut v_x: Py; 
v_x = t506;
out.push(py::abs(&v_x.clone())?); }} Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } }; if t507.truthy() { t507 } else { Py::Float(1.0) } };
    for t508 in py::iter(&(py::method(&v_kcl.clone(), "items", &[])?))? {
        let t512 = (t508).items()?;
        if t512.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_n = t512[0].clone();
        v_terms = t512[1].clone();
        if ({ let t509 = v_n.clone(); Py::Bool({ let t510 = Py::str("0"); if py::eq(&t509, &t510) { true } else { false } }) }).truthy() {
            continue;
        }
        v_scale = py::minmax(&[py::fsum(&{ let mut out = vec![]; for t511 in py::iter(&(v_terms.clone()))? { let mut v_x: Py; 
v_x = t511;
out.push(py::abs(&v_x.clone())?); } Py::list(out) })?, v_big.clone()], true)?;
        v_worst = py::minmax(&[v_worst.clone(), py::truediv(&py::abs(&py::fsum(&v_terms.clone())?)?, &v_scale.clone())?], true)?;
    }
    v_p = py::getitem(&v_outputs.clone(), &Py::str("power_delivered"))?;
    v_worst = py::minmax(&[v_worst.clone(), (if ({ let t513 = v_p.clone(); if t513.truthy() { t513 } else { v_diss.clone() } }).truthy() { py::truediv(&py::abs(&py::sub(&v_p.clone(), &v_diss.clone())?)?, &py::minmax(&[py::abs(&v_p.clone())?, py::abs(&v_diss.clone())?, Py::Float(1e-300)], true)?)? } else { Py::Float(0.0) })], true)?;
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_norton(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__verify_thevenin(v_inputs.clone(), Py::dict(vec![(Py::str("Vth"), py::mul(&py::getitem(&v_outputs.clone(), &Py::str("In"))?, &py::getitem(&v_outputs.clone(), &Py::str("Rn"))?)?), (Py::str("Rth"), py::getitem(&v_outputs.clone(), &Py::str("Rn"))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_p2r(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_z: Py = Py::None;
    let t1256 = (Py::tuple(vec![py::getitem(&v_inputs.clone(), &Py::str("magnitude"))?, py::getitem(&v_inputs.clone(), &Py::str("angle_deg"))?])).items()?;
    if t1256.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_m = t1256[0].clone();
    v_a = t1256[1].clone();
    v_z = py::complex(&[py::getitem(&v_outputs.clone(), &Py::str("real"))?, py::getitem(&v_outputs.clone(), &Py::str("imag"))?])?;
    v_d = (if ({ let t1257 = py::abs(&v_z.clone())?; Py::Bool({ let t1258 = Py::Int(0i64); if py::gt(&t1257, &t1258)? { true } else { false } }) }).truthy() { py::sub(&py::modulo(&py::add(&py::sub(&py::math1("degrees", &py::cmath("phase", &[v_z.clone()])?)?, &v_a.clone())?, &Py::Int(180i64))?, &Py::Int(360i64))?, &Py::Int(180i64))? } else { Py::Float(0.0) });
    return Ok(py::add(&py::truediv(&py::abs(&py::sub(&py::abs(&v_z.clone())?, &v_m.clone())?)?, &py::minmax(&[Py::Float(1.0), v_m.clone()], true)?)?, &(if ({ let t1259 = v_m.clone(); Py::Bool({ let t1260 = Py::Int(0i64); if py::gt(&t1259, &t1260)? { true } else { false } }) }).truthy() { py::truediv(&py::abs(&v_d.clone())?, &Py::Int(180i64))? } else { Py::Float(0.0) }))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_pfc(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_I: Py = Py::None;
    let mut v_It: Py = Py::None;
    let mut v_P: Py = Py::None;
    let mut v_V: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_pf: Py = Py::None;
    let mut v_q1: Py = Py::None;
    let t1275 = (Py::tuple(vec![py::getitem(&v_inputs.clone(), &Py::str("P"))?, py::getitem(&v_inputs.clone(), &Py::str("V"))?, py::getitem(&v_inputs.clone(), &Py::str("f"))?])).items()?;
    if t1275.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_P = t1275[0].clone();
    v_V = t1275[1].clone();
    v_f = t1275[2].clone();
    v_q1 = py::mul(&v_P.clone(), &py::math1("tan", &py::math1("acos", &py::getitem(&v_inputs.clone(), &Py::str("pf_initial"))?)?)?)?;
    v_I = py::truediv(&py::complex(&[v_P.clone(), py::neg(&v_q1.clone())?])?, &v_V.clone())?;
    v_It = py::add(&v_I.clone(), &py::mul(&py::mul(&py::mul(&py::mul(&py::mul(&Py::Complex(0.0, 1.0), &Py::Int(2i64))?, &Py::Float(std::f64::consts::PI))?, &v_f.clone())?, &py::getitem(&v_outputs.clone(), &Py::str("C"))?)?, &v_V.clone())?)?;
    v_pf = py::math1("cos", &py::cmath("phase", &[v_It.clone()])?)?;
    return Ok(py::add(&py::add(&py::add(&py::abs(&py::sub(&v_pf.clone(), &py::getitem(&v_inputs.clone(), &Py::str("pf_target"))?)?)?, &f__rel(py::getitem(&v_outputs.clone(), &Py::str("C_uF"))?, py::mul(&py::getitem(&v_outputs.clone(), &Py::str("C"))?, &Py::Float(1000000.0))?)?)?, &f__rel(py::getitem(&v_outputs.clone(), &Py::str("Qc"))?, py::mul(&py::mul(&py::mul(&py::mul(&py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?, &v_f.clone())?, &py::getitem(&v_outputs.clone(), &Py::str("C"))?)?, &v_V.clone())?, &v_V.clone())?)?)?, &f__rel(py::getitem(&v_outputs.clone(), &Py::str("Q_initial"))?, v_q1.clone())?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_phasor_sum(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_lhs: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_scale: Py = Py::None;
    let mut v_th: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1248 = (Py::tuple(vec![py::getitem(&v_outputs.clone(), &Py::str("amplitude"))?, py::math1("radians", &py::getitem(&v_outputs.clone(), &Py::str("phase_deg"))?)?])).items()?;
    if t1248.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_A = t1248[0].clone();
    v_p = t1248[1].clone();
    v_scale = { let t1249 = py::sum(&py::getitem(&v_inputs.clone(), &Py::str("amplitudes"))?, None)?; if t1249.truthy() { t1249 } else { Py::Float(1.0) } };
    v_worst = Py::Float(0.0);
    for t1250 in py::range_iter(&[(Py::Int(8i64)).i()?]) {
        v_k = t1250;
        v_th = py::add(&py::truediv(&py::mul(&py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?, &v_k.clone())?, &Py::Int(8i64))?, &Py::Float(0.1))?;
        v_lhs = py::fsum(&{ let mut out = vec![]; for t1251 in py::iter(&(py::zip(&[py::getitem(&v_inputs.clone(), &Py::str("amplitudes"))?, py::getitem(&v_inputs.clone(), &Py::str("phases_deg"))?])?))? { let mut v_a: Py; let mut v_q: Py; 
let t1252 = (t1251).items()?;
if t1252.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t1252[0].clone();
v_q = t1252[1].clone();
out.push(py::mul(&v_a.clone(), &py::math1("cos", &py::add(&v_th.clone(), &py::math1("radians", &v_q.clone())?)?)?)?); } Py::list(out) })?;
        v_worst = py::minmax(&[v_worst.clone(), py::truediv(&py::abs(&py::sub(&v_lhs.clone(), &py::mul(&v_A.clone(), &py::math1("cos", &py::add(&v_th.clone(), &v_p.clone())?)?)?)?)?, &v_scale.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_pm(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_den: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_num: Py = Py::None;
    let mut v_pm: Py = Py::None;
    let mut v_w: Py = Py::None;
    let t1236 = (Py::tuple(vec![f__ptrim(py::getitem(&v_inputs.clone(), &Py::str("num"))?)?, f__ptrim(py::getitem(&v_inputs.clone(), &Py::str("den"))?)?])).items()?;
    if t1236.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_num = t1236[0].clone();
    v_den = t1236[1].clone();
    v_w = py::getitem(&v_outputs.clone(), &Py::str("gain_crossover"))?;
    v_g = f__G(v_num.clone(), v_den.clone(), v_w.clone())?;
    v_pm = py::sub(&py::modulo(&py::math1("degrees", &py::cmath("phase", &[v_g.clone()])?)?, &Py::Float(360.0))?, &Py::Float(180.0))?;
    return Ok(py::add(&py::abs(&py::sub(&py::abs(&v_g.clone())?, &Py::Float(1.0))?)?, &py::truediv(&py::abs(&py::sub(&py::getitem(&v_outputs.clone(), &Py::str("phase_margin"))?, &v_pm.clone())?)?, &Py::Float(180.0))?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_poles(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_char: Py = Py::None;
    let mut v_den: Py = Py::None;
    let mut v_mr: Py = Py::None;
    let mut v_num: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_poles: Py = Py::None;
    let mut v_terms: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t273 = (Py::tuple(vec![f__ptrim(py::getitem(&v_inputs.clone(), &Py::str("num"))?)?, f__ptrim(py::getitem(&v_inputs.clone(), &Py::str("den"))?)?])).items()?;
    if t273.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_num = t273[0].clone();
    v_den = t273[1].clone();
    v_char = f__ptrim(f__padd(v_den.clone(), { let mut out = vec![]; for t274 in py::iter(&(v_num.clone()))? { let mut v_x: Py; 
v_x = t274;
out.push(py::mul(&py::method(&v_inputs.clone(), "get", &[Py::str("K"), Py::Float(1.0)])?, &v_x.clone())?); } Py::list(out) })?)?;
    v_poles = py::getitem(&v_outputs.clone(), &Py::str("poles"))?;
    if ({ let t275 = py::len(&v_poles.clone())?; Py::Bool({ let t276 = py::sub(&py::len(&v_char.clone())?, &Py::Int(1i64))?; if py::ne(&t275, &t276) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_worst = Py::Float(0.0);
    for t277 in py::iter(&(v_poles.clone()))? {
        v_p = t277;
        v_p = py::complex(&[v_p.clone()])?;
        v_terms = { let mut out = vec![]; for t278 in py::iter(&(py::enumerate(&v_char.clone(), 0)?))? { let mut v_c: Py; let mut v_i: Py; 
let t279 = (t278).items()?;
if t279.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_i = t279[0].clone();
v_c = t279[1].clone();
out.push(py::mul(&v_c.clone(), &py::pow(&v_p.clone(), &py::sub(&py::sub(&py::len(&v_char.clone())?, &Py::Int(1i64))?, &v_i.clone())?)?)?); } Py::list(out) };
        v_worst = py::minmax(&[v_worst.clone(), py::truediv(&py::abs(&py::sum(&v_terms.clone(), None)?)?, &{ let t281 = py::sum(&{ let mut out = vec![]; for t280 in py::iter(&(v_terms.clone()))? { let mut v_t: Py; 
v_t = t280;
out.push(py::abs(&v_t.clone())?); } Py::list(out) }, None)?; if t281.truthy() { t281 } else { Py::Float(1.0) } })?], true)?;
    }
    v_mr = py::minmax(&[{ let mut out = vec![]; for t282 in py::iter(&(v_poles.clone()))? { let mut v_p: Py; 
v_p = t282;
out.push(py::real(&py::complex(&[v_p.clone()])?)?); } Py::list(out) }], true)?;
    return Ok(py::add(&py::add(&v_worst.clone(), &py::abs(&py::sub(&py::getitem(&v_outputs.clone(), &Py::str("max_real_part"))?, &v_mr.clone())?)?)?, &{ let t283 = py::getitem(&v_outputs.clone(), &Py::str("stable"))?; Py::Bool({ let t284 = { let t285 = v_mr.clone(); Py::Bool({ let t286 = Py::Int(0i64); if py::lt(&t285, &t286)? { true } else { false } }) }; if py::ne(&t283, &t284) { true } else { false } }) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_r2p(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_z: Py = Py::None;
    let t1291 = (Py::tuple(vec![py::getitem(&v_outputs.clone(), &Py::str("magnitude"))?, py::math1("radians", &py::getitem(&v_outputs.clone(), &Py::str("angle_deg"))?)?])).items()?;
    if t1291.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_m = t1291[0].clone();
    v_a = t1291[1].clone();
    v_z = py::complex(&[py::getitem(&v_inputs.clone(), &Py::str("real"))?, py::getitem(&v_inputs.clone(), &Py::str("imag"))?])?;
    return Ok(py::add(&py::truediv(&py::abs(&py::sub(&py::complex(&[py::mul(&v_m.clone(), &py::math1("cos", &v_a.clone())?)?, py::mul(&v_m.clone(), &py::math1("sin", &v_a.clone())?)?])?, &v_z.clone())?)?, &py::minmax(&[Py::Float(1.0), py::abs(&v_z.clone())?], true)?)?, &py::not(&{ let t1292 = py::neg(&Py::Int(180i64))?; Py::Bool({ let t1293 = py::getitem(&v_outputs.clone(), &Py::str("angle_deg"))?; if py::lt(&t1292, &t1293)? { { let t1294 = Py::Int(180i64); if py::le(&t1293, &t1294)? { true } else { false } } } else { false } }) }))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_rlc_step(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_C: Py = Py::None;
    let mut v_L: Py = Py::None;
    let mut v_R: Py = Py::None;
    let mut v_V: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_cap: Py = Py::None;
    let mut v_didt: Py = Py::None;
    let mut v_dvdt: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_h: Py = Py::None;
    let mut v_here: Py = Py::None;
    let mut v_hi: Py = Py::None;
    let mut v_kvl: Py = Py::None;
    let mut v_lo: Py = Py::None;
    let mut v_same: Py = Py::None;
    let mut v_scale: Py = Py::None;
    let mut v_t: Py = Py::None;
    let mut v_w0: Py = Py::None;
    let t1527 = ({ let mut out = vec![]; for t1526 in py::iter(&(Py::tuple(vec![Py::str("R"), Py::str("L"), Py::str("C"), Py::str("V"), Py::str("t")])))? { let mut v_k: Py; 
v_k = t1526;
out.push(py::getitem(&v_inputs.clone(), &v_k.clone())?); } Py::list(out) }).items()?;
    if t1527.len() != 5 { return Err(PyErr::Value("unpack".into())); }
    v_R = t1527[0].clone();
    v_L = t1527[1].clone();
    v_C = t1527[2].clone();
    v_V = t1527[3].clone();
    v_t = t1527[4].clone();
    let t1528 = (Py::tuple(vec![py::truediv(&v_R.clone(), &py::mul(&Py::Int(2i64), &v_L.clone())?)?, py::truediv(&Py::Int(1i64), &py::math1("sqrt", &py::mul(&v_L.clone(), &v_C.clone())?)?)?])).items()?;
    if t1528.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t1528[0].clone();
    v_w0 = t1528[1].clone();
    v_h = py::truediv(&Py::Float(0.0001), &py::minmax(&[v_a.clone(), v_w0.clone()], true)?)?;
    v_f = { let v_C = v_C.clone(); let v_L = v_L.clone(); let v_R = v_R.clone(); let v_V = v_V.clone(); let v_a = v_a.clone(); let v_h = v_h.clone(); let v_inputs = v_inputs.clone(); let v_outputs = v_outputs.clone(); let v_t = v_t.clone(); let v_w0 = v_w0.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_tt: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(f_series_rlc_step(v_R.clone(), v_L.clone(), v_C.clone(), v_V.clone(), py::minmax(&[v_tt.clone(), Py::Float(0.0)], true)?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    let t1529 = (Py::tuple(vec![py::call(&v_f.clone(), &[py::sub(&v_t.clone(), &v_h.clone())?])?, py::call(&v_f.clone(), &[py::add(&v_t.clone(), &v_h.clone())?])?])).items()?;
    if t1529.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lo = t1529[0].clone();
    v_hi = t1529[1].clone();
    v_didt = (if ({ let t1530 = v_t.clone(); Py::Bool({ let t1531 = v_h.clone(); if py::ge(&t1530, &t1531)? { true } else { false } }) }).truthy() { py::truediv(&py::sub(&py::getitem(&v_hi.clone(), &Py::str("i"))?, &py::getitem(&v_lo.clone(), &Py::str("i"))?)?, &py::mul(&Py::Int(2i64), &v_h.clone())?)? } else { py::truediv(&py::sub(&py::getitem(&v_hi.clone(), &Py::str("i"))?, &py::getitem(&v_outputs.clone(), &Py::str("i"))?)?, &v_h.clone())? });
    v_dvdt = (if ({ let t1532 = v_t.clone(); Py::Bool({ let t1533 = v_h.clone(); if py::ge(&t1532, &t1533)? { true } else { false } }) }).truthy() { py::truediv(&py::sub(&py::getitem(&v_hi.clone(), &Py::str("vC"))?, &py::getitem(&v_lo.clone(), &Py::str("vC"))?)?, &py::mul(&Py::Int(2i64), &v_h.clone())?)? } else { py::truediv(&py::sub(&py::getitem(&v_hi.clone(), &Py::str("vC"))?, &py::getitem(&v_outputs.clone(), &Py::str("vC"))?)?, &v_h.clone())? });
    v_scale = { let t1534 = py::abs(&v_V.clone())?; if t1534.truthy() { t1534 } else { Py::Float(1.0) } };
    v_kvl = py::truediv(&py::abs(&py::sub(&py::sub(&py::sub(&v_V.clone(), &py::mul(&v_R.clone(), &py::getitem(&v_outputs.clone(), &Py::str("i"))?)?)?, &py::mul(&v_L.clone(), &v_didt.clone())?)?, &py::getitem(&v_outputs.clone(), &Py::str("vC"))?)?)?, &v_scale.clone())?;
    v_cap = py::truediv(&py::mul(&py::abs(&py::sub(&py::mul(&v_C.clone(), &v_dvdt.clone())?, &py::getitem(&v_outputs.clone(), &Py::str("i"))?)?)?, &v_R.clone())?, &v_scale.clone())?;
    v_here = py::call(&v_f.clone(), &[v_t.clone()])?;
    v_same = py::add(&py::truediv(&py::abs(&py::sub(&py::getitem(&v_here.clone(), &Py::str("vC"))?, &py::getitem(&v_outputs.clone(), &Py::str("vC"))?)?)?, &v_scale.clone())?, &py::truediv(&py::mul(&py::abs(&py::sub(&py::getitem(&v_here.clone(), &Py::str("i"))?, &py::getitem(&v_outputs.clone(), &Py::str("i"))?)?)?, &v_R.clone())?, &v_scale.clone())?)?;
    return Ok(py::add(&py::add(&py::add(&py::add(&py::add(&py::add(&v_same.clone(), &(if ({ let t1535 = v_kvl.clone(); Py::Bool({ let t1536 = Py::Float(1e-05); if py::gt(&t1535, &t1536)? { true } else { false } }) }).truthy() { v_kvl.clone() } else { Py::Float(0.0) }))?, &(if ({ let t1537 = v_cap.clone(); Py::Bool({ let t1538 = Py::Float(1e-05); if py::gt(&t1537, &t1538)? { true } else { false } }) }).truthy() { v_cap.clone() } else { Py::Float(0.0) }))?, &f__rel(py::getitem(&v_outputs.clone(), &Py::str("alpha"))?, v_a.clone())?)?, &f__rel(py::getitem(&v_outputs.clone(), &Py::str("omega0"))?, v_w0.clone())?)?, &f__rel(py::getitem(&v_outputs.clone(), &Py::str("zeta"))?, py::truediv(&v_a.clone(), &v_w0.clone())?)?)?, &f__rel(py::getitem(&v_outputs.clone(), &Py::str("i_mA"))?, py::mul(&py::getitem(&v_outputs.clone(), &Py::str("i"))?, &Py::Float(1000.0))?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_routh(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_ch: Py = Py::None;
    let mut v_col: Py = Py::None;
    let mut v_fc: Py = Py::None;
    let mut v_rhp: Py = Py::None;
    let mut v_roots: Py = Py::None;
    let mut v_want: Py = Py::None;
    v_roots = f__roots(py::getitem(&v_inputs.clone(), &Py::str("coefficients"))?)?;
    v_rhp = py::sum(&{ let mut out = vec![]; for t1403 in py::iter(&(v_roots.clone()))? { let mut v_r: Py; 
v_r = t1403;
if ({ let t1404 = py::real(&py::complex(&[v_r.clone()])?)?; Py::Bool({ let t1405 = Py::Int(0i64); if py::gt(&t1404, &t1405)? { true } else { false } }) }).truthy() { out.push(Py::Int(1i64)); }} Py::list(out) }, None)?;
    v_fc = py::method(&v_outputs.clone(), "get", &[Py::str("first_column"), Py::list(vec![])])?;
    v_ch = (if (v_fc.clone()).truthy() { py::sum(&{ let mut out = vec![]; for t1406 in py::iter(&(py::zip(&[v_fc.clone(), py::slice(&v_fc.clone(), Some((Py::Int(1i64)).i()?), None, None)?])?))? { let mut v_x: Py; let mut v_y: Py; 
let t1407 = (t1406).items()?;
if t1407.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t1407[0].clone();
v_y = t1407[1].clone();
if ({ let t1408 = { let t1410 = v_x.clone(); Py::Bool({ let t1411 = Py::Int(0i64); if py::gt(&t1410, &t1411)? { true } else { false } }) }; Py::Bool({ let t1409 = { let t1412 = v_y.clone(); Py::Bool({ let t1413 = Py::Int(0i64); if py::gt(&t1412, &t1413)? { true } else { false } }) }; if py::ne(&t1408, &t1409) { true } else { false } }) }).truthy() { out.push(Py::Int(1i64)); }} Py::list(out) }, None)? } else { v_rhp.clone() });
    v_col = Py::Float(0.0);
    if (v_fc.clone()).truthy() {
        v_want = py::getitem(&f_routh_hurwitz(py::getitem(&v_inputs.clone(), &Py::str("coefficients"))?)?, &Py::str("first_column"))?;
        v_col = (if ({ let t1414 = py::len(&v_want.clone())?; Py::Bool({ let t1415 = py::len(&v_fc.clone())?; if py::ne(&t1414, &t1415) { true } else { false } }) }).truthy() { Py::Float(1.0) } else { py::minmax(&[{ let mut out = vec![]; for t1416 in py::iter(&(py::zip(&[v_fc.clone(), v_want.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t1417 = (t1416).items()?;
if t1417.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t1417[0].clone();
v_b = t1417[1].clone();
out.push(f__rel(v_a.clone(), v_b.clone())?); } Py::list(out) }], true)? });
    }
    return Ok(py::add(&py::add(&py::add(&py::abs(&py::sub(&py::getitem(&v_outputs.clone(), &Py::str("rhp_roots"))?, &v_rhp.clone())?)?, &py::abs(&py::sub(&v_ch.clone(), &v_rhp.clone())?)?)?, &{ let t1418 = py::getitem(&v_outputs.clone(), &Py::str("stable"))?; Py::Bool({ let t1419 = { let t1420 = v_rhp.clone(); Py::Bool({ let t1421 = Py::Int(0i64); if py::eq(&t1420, &t1421) { true } else { false } }) }; if py::ne(&t1418, &t1419) { true } else { false } }) })?, &v_col.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_ss(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_B: Py = Py::None;
    let mut v_C: Py = Py::None;
    let mut v_D: Py = Py::None;
    let mut v_M: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_h: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let mut v_x: Py = Py::None;
    let t1573 = (Py::tuple(vec![py::getitem(&v_inputs.clone(), &Py::str("A"))?, py::getitem(&v_inputs.clone(), &Py::str("B"))?, py::getitem(&v_inputs.clone(), &Py::str("C"))?, py::getitem(&v_inputs.clone(), &Py::str("D"))?])).items()?;
    if t1573.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_A = t1573[0].clone();
    v_B = t1573[1].clone();
    v_C = t1573[2].clone();
    v_D = t1573[3].clone();
    v_n = py::len(&v_A.clone())?;
    v_worst = Py::Float(0.0);
    for t1574 in py::iter(&(Py::tuple(vec![py::complex(&[Py::Float(0.37), Py::Float(1.3)])?, py::complex(&[py::neg(&Py::Float(2.1))?, Py::Float(0.7)])?])))? {
        v_s = t1574;
        v_M = { let mut out = vec![]; for t1575 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t1575;
out.push({ let mut out = vec![]; for t1576 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t1576;
out.push(py::sub(&(if ({ let t1577 = v_i.clone(); Py::Bool({ let t1578 = v_j.clone(); if py::eq(&t1577, &t1578) { true } else { false } }) }).truthy() { v_s.clone() } else { Py::Int(0i64) }), &py::getitem(&py::getitem(&v_A.clone(), &v_i.clone())?, &v_j.clone())?)?); } Py::list(out) }); } Py::list(out) };
        let t1607: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                v_x = f__gauss(v_M.clone(), { let mut out = vec![]; for t1606 in py::iter(&(v_B.clone()))? { let mut v_b: Py; 
v_b = t1606;
out.push(py::complex(&[v_b.clone()])?); } Py::list(out) })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t1607 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (matches!(e, PyErr::Value(_))) => {
                continue;
            }
            Err(e) => return Err(e),
        }
        v_g = py::add(&py::sum(&{ let mut out = vec![]; for t1608 in py::iter(&(py::zip(&[v_C.clone(), v_x.clone()])?))? { let mut v_c: Py; let mut v_xi: Py; 
let t1609 = (t1608).items()?;
if t1609.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_c = t1609[0].clone();
v_xi = t1609[1].clone();
out.push(py::mul(&v_c.clone(), &v_xi.clone())?); } Py::list(out) }, None)?, &v_D.clone())?;
        v_h = py::truediv(&f__pval(py::getitem(&v_outputs.clone(), &Py::str("num"))?, v_s.clone())?, &f__pval(py::getitem(&v_outputs.clone(), &Py::str("den"))?, v_s.clone())?)?;
        v_worst = py::minmax(&[v_worst.clone(), py::truediv(&py::abs(&py::sub(&v_g.clone(), &v_h.clone())?)?, &py::minmax(&[py::abs(&v_g.clone())?, Py::Float(1e-12)], true)?)?], true)?;
    }
    return Ok(py::add(&py::add(&py::add(&v_worst.clone(), &py::abs(&py::sub(&py::getitem(&py::getitem(&v_outputs.clone(), &Py::str("den"))?, &Py::Int(0i64))?, &Py::Float(1.0))?)?)?, &py::abs(&py::sub(&py::getitem(&v_outputs.clone(), &Py::str("order"))?, &v_n.clone())?)?)?, &py::float(&{ let t1611 = py::len(&py::getitem(&v_outputs.clone(), &Py::str("num"))?)?; Py::Bool({ let t1612 = py::add(&v_n.clone(), &Py::Int(1i64))?; if py::ne(&t1611, &t1612) { true } else { false } }) })?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_sse(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_bot: Py = Py::None;
    let mut v_d_: Py = Py::None;
    let mut v_den: Py = Py::None;
    let mut v_ess: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_n_: Py = Py::None;
    let mut v_num: Py = Py::None;
    let mut v_top: Py = Py::None;
    let mut v_want: Py = Py::None;
    v_num = { let mut out = vec![]; for t1698 in py::iter(&(f__ptrim(py::getitem(&v_inputs.clone(), &Py::str("num"))?)?))? { let mut v_x: Py; 
v_x = t1698;
out.push(py::fraction(&[v_x.clone()])?); } Py::list(out) };
    v_den = { let mut out = vec![]; for t1699 in py::iter(&(f__ptrim(py::getitem(&v_inputs.clone(), &Py::str("den"))?)?))? { let mut v_x: Py; 
v_x = t1699;
out.push(py::fraction(&[v_x.clone()])?); } Py::list(out) };
    v_k = py::getitem(&c__INPUT_ORDER()?, &py::getitem(&v_inputs.clone(), &Py::str("input"))?)?;
    v_top = py::slice(&v_den.clone(), None, None, Some((py::neg(&Py::Int(1i64))?).i()?))?;
    let t1700 = (Py::tuple(vec![py::slice(&v_num.clone(), None, None, Some((py::neg(&Py::Int(1i64))?).i()?))?, py::slice(&v_den.clone(), None, None, Some((py::neg(&Py::Int(1i64))?).i()?))?])).items()?;
    if t1700.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_n_ = t1700[0].clone();
    v_d_ = t1700[1].clone();
    v_bot = py::add(&py::mul(&Py::list(vec![py::fraction(&[Py::Int(0i64)])?]), &v_k.clone())?, &{ let mut out = vec![]; for t1701 in py::range_iter(&[(py::minmax(&[py::len(&v_d_.clone())?, py::len(&v_n_.clone())?], true)?).i()?]) { let mut v_i: Py; 
v_i = t1701;
out.push(py::add(&(if ({ let t1702 = v_i.clone(); Py::Bool({ let t1703 = py::len(&v_d_.clone())?; if py::lt(&t1702, &t1703)? { true } else { false } }) }).truthy() { py::getitem(&v_d_.clone(), &v_i.clone())? } else { Py::Int(0i64) }), &(if ({ let t1704 = v_i.clone(); Py::Bool({ let t1705 = py::len(&v_n_.clone())?; if py::lt(&t1704, &t1705)? { true } else { false } }) }).truthy() { py::getitem(&v_n_.clone(), &v_i.clone())? } else { Py::Int(0i64) }))?); } Py::list(out) })?;
    v_a = (match ('g1706: { let mut out = vec![]; for t1707 in py::iter(&(py::enumerate(&v_top.clone(), 0)?))? { let mut v_i: Py; let mut v_x: Py; 
let t1708 = (t1707).items()?;
if t1708.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_i = t1708[0].clone();
v_x = t1708[1].clone();
if ({ let t1709 = v_x.clone(); Py::Bool({ let t1710 = Py::Int(0i64); if py::ne(&t1709, &t1710) { true } else { false } }) }).truthy() { out.push(v_i.clone()); break 'g1706 Py::list(out); }} Py::list(out) }).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) });
    v_b = (match ('g1711: { let mut out = vec![]; for t1712 in py::iter(&(py::enumerate(&v_bot.clone(), 0)?))? { let mut v_i: Py; let mut v_x: Py; 
let t1713 = (t1712).items()?;
if t1713.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_i = t1713[0].clone();
v_x = t1713[1].clone();
if ({ let t1714 = v_x.clone(); Py::Bool({ let t1715 = Py::Int(0i64); if py::ne(&t1714, &t1715) { true } else { false } }) }).truthy() { out.push(v_i.clone()); break 'g1711 Py::list(out); }} Py::list(out) }).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) });
    v_want = (if ({ let t1716 = v_a.clone(); Py::Bool({ let t1717 = v_b.clone(); if py::gt(&t1716, &t1717)? { true } else { false } }) }).truthy() { py::fraction(&[Py::Int(0i64)])? } else { (if ({ let t1718 = v_a.clone(); Py::Bool({ let t1719 = v_b.clone(); if py::eq(&t1718, &t1719) { true } else { false } }) }).truthy() { py::truediv(&py::getitem(&v_top.clone(), &v_a.clone())?, &py::getitem(&v_bot.clone(), &v_b.clone())?)? } else { Py::None }) });
    v_ess = py::getitem(&v_outputs.clone(), &Py::str("steady_state_error"))?;
    if ({ let t1720 = v_want.clone(); Py::Bool({ let t1721 = Py::None; if py::is(&t1720, &t1721) { true } else { false } }) }).truthy() {
        return Ok((if ({ let t1722 = v_ess.clone(); Py::Bool({ let t1723 = Py::str("infinite"); if py::eq(&t1722, &t1723) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { Py::Float(1.0) }));
    }
    if (Py::Bool(matches!(v_ess.clone(), Py::Str(_)))).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::truediv(&py::abs(&py::sub(&v_ess.clone(), &py::float(&v_want.clone())?)?)?, &py::minmax(&[Py::Float(1.0), py::abs(&py::float(&v_want.clone())?)?], true)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_step2(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_dy: Py = Py::None;
    let mut v_q: Py = Py::None;
    let mut v_res: Py = Py::None;
    let mut v_sg: Py = Py::None;
    let mut v_tp: Py = Py::None;
    let mut v_ts: Py = Py::None;
    let mut v_wd: Py = Py::None;
    let mut v_wn: Py = Py::None;
    let mut v_y: Py = Py::None;
    let mut v_z: Py = Py::None;
    let t1444 = (Py::tuple(vec![py::getitem(&v_inputs.clone(), &Py::str("zeta"))?, py::getitem(&v_inputs.clone(), &Py::str("wn"))?])).items()?;
    if t1444.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_z = t1444[0].clone();
    v_wn = t1444[1].clone();
    v_q = py::math1("sqrt", &py::sub(&Py::Int(1i64), &py::mul(&v_z.clone(), &v_z.clone())?)?)?;
    let t1445 = (Py::tuple(vec![py::mul(&v_wn.clone(), &v_q.clone())?, py::mul(&v_z.clone(), &v_wn.clone())?])).items()?;
    if t1445.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_wd = t1445[0].clone();
    v_sg = t1445[1].clone();
    v_y = { let v_inputs = v_inputs.clone(); let v_outputs = v_outputs.clone(); let v_q = v_q.clone(); let v_sg = v_sg.clone(); let v_wd = v_wd.clone(); let v_wn = v_wn.clone(); let v_z = v_z.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_t: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::sub(&Py::Int(1i64), &py::mul(&py::math1("exp", &py::mul(&py::neg(&v_sg.clone())?, &v_t.clone())?)?, &py::add(&py::math1("cos", &py::mul(&v_wd.clone(), &v_t.clone())?)?, &py::mul(&py::truediv(&v_z.clone(), &v_q.clone())?, &py::math1("sin", &py::mul(&v_wd.clone(), &v_t.clone())?)?)?)?)?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_dy = { let v_inputs = v_inputs.clone(); let v_outputs = v_outputs.clone(); let v_q = v_q.clone(); let v_sg = v_sg.clone(); let v_wd = v_wd.clone(); let v_wn = v_wn.clone(); let v_y = v_y.clone(); let v_z = v_z.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_t: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::mul(&py::mul(&py::truediv(&v_wn.clone(), &v_q.clone())?, &py::math1("exp", &py::mul(&py::neg(&v_sg.clone())?, &v_t.clone())?)?)?, &py::math1("sin", &py::mul(&v_wd.clone(), &v_t.clone())?)?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    let t1446 = (Py::tuple(vec![py::getitem(&v_outputs.clone(), &Py::str("peak_time"))?, py::getitem(&v_outputs.clone(), &Py::str("settling_time"))?])).items()?;
    if t1446.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_tp = t1446[0].clone();
    v_ts = t1446[1].clone();
    v_res = Py::list(vec![py::truediv(&py::abs(&py::sub(&py::mul(&v_wd.clone(), &v_tp.clone())?, &Py::Float(std::f64::consts::PI))?)?, &Py::Float(std::f64::consts::PI))?, py::abs(&py::sub(&py::sub(&py::call(&v_y.clone(), &[v_tp.clone()])?, &Py::Int(1i64))?, &py::getitem(&v_outputs.clone(), &Py::str("overshoot"))?)?)?, py::truediv(&py::abs(&py::sub(&py::getitem(&v_outputs.clone(), &Py::str("percent_overshoot"))?, &py::mul(&Py::Int(100i64), &py::getitem(&v_outputs.clone(), &Py::str("overshoot"))?)?)?)?, &Py::Int(100i64))?, py::abs(&py::sub(&py::call(&v_y.clone(), &[py::getitem(&v_outputs.clone(), &Py::str("t10"))?])?, &Py::Float(0.1))?)?, py::abs(&py::sub(&py::call(&v_y.clone(), &[py::getitem(&v_outputs.clone(), &Py::str("t90"))?])?, &Py::Float(0.9))?)?, py::mul(&py::abs(&py::sub(&py::getitem(&v_outputs.clone(), &Py::str("rise_time"))?, &py::sub(&py::getitem(&v_outputs.clone(), &Py::str("t90"))?, &py::getitem(&v_outputs.clone(), &Py::str("t10"))?)?)?)?, &v_wn.clone())?, py::abs(&py::sub(&py::abs(&py::sub(&py::call(&v_y.clone(), &[v_ts.clone()])?, &Py::Int(1i64))?)?, &Py::Float(0.02))?)?, py::minmax(&[Py::Float(0.0), py::sub(&py::math1("exp", &py::truediv(&py::mul(&py::mul(&py::neg(&v_sg.clone())?, &py::add(&py::math1("floor", &py::truediv(&py::mul(&v_ts.clone(), &v_wd.clone())?, &Py::Float(std::f64::consts::PI))?)?, &Py::Int(1i64))?)?, &Py::Float(std::f64::consts::PI))?, &v_wd.clone())?)?, &Py::Float(0.02))?], true)?]);
    return Ok(py::minmax(&[v_res.clone()], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_summing(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_scale: Py = Py::None;
    let mut v_terms: Py = Py::None;
    v_terms = py::add(&{ let mut out = vec![]; for t1735 in py::iter(&(py::zip(&[py::getitem(&v_inputs.clone(), &Py::str("V"))?, py::getitem(&v_inputs.clone(), &Py::str("R"))?])?))? { let mut v_r: Py; let mut v_v: Py; 
let t1736 = (t1735).items()?;
if t1736.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_v = t1736[0].clone();
v_r = t1736[1].clone();
out.push(py::truediv(&v_v.clone(), &v_r.clone())?); } Py::list(out) }, &Py::list(vec![py::truediv(&py::getitem(&v_outputs.clone(), &Py::str("Vout"))?, &py::getitem(&v_inputs.clone(), &Py::str("Rf"))?)?]))?;
    v_scale = { let t1738 = py::fsum(&{ let mut out = vec![]; for t1737 in py::iter(&(v_terms.clone()))? { let mut v_t: Py; 
v_t = t1737;
out.push(py::abs(&v_t.clone())?); } Py::list(out) })?; if t1738.truthy() { t1738 } else { Py::Float(1.0) } };
    return Ok(py::add(&py::truediv(&py::abs(&py::fsum(&v_terms.clone())?)?, &v_scale.clone())?, &py::minmax(&[{ let mut out = vec![]; for t1740 in py::iter(&(py::zip(&[py::method(&v_outputs.clone(), "get", &[Py::str("gains"), { let mut out = vec![]; for t1739 in py::iter(&(py::getitem(&v_inputs.clone(), &Py::str("R"))?))? { let mut v_r: Py; 
v_r = t1739;
out.push(py::truediv(&py::neg(&py::getitem(&v_inputs.clone(), &Py::str("Rf"))?)?, &v_r.clone())?); } Py::list(out) }])?, py::getitem(&v_inputs.clone(), &Py::str("R"))?])?))? { let mut v_g: Py; let mut v_r: Py; 
let t1741 = (t1740).items()?;
if t1741.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_g = t1741[0].clone();
v_r = t1741[1].clone();
out.push(py::abs(&py::add(&v_g.clone(), &py::truediv(&py::getitem(&v_inputs.clone(), &Py::str("Rf"))?, &v_r.clone())?)?)?); } Py::list(out) }], true)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_tc_enc(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_n: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_val: Py = Py::None;
    let t2059 = (Py::tuple(vec![py::getitem(&v_outputs.clone(), &Py::str("pattern"))?, py::int(&py::getitem(&v_inputs.clone(), &Py::str("bits"))?)?])).items()?;
    if t2059.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_p = t2059[0].clone();
    v_n = t2059[1].clone();
    if ({ let t2062 = { let t2060 = py::len(&v_p.clone())?; Py::Bool({ let t2061 = v_n.clone(); if py::ne(&t2060, &t2061) { true } else { false } }) }; if t2062.truthy() { t2062 } else { py::sub(&py::set(&v_p.clone())?, &py::set(&Py::list(vec![Py::str("0"), Py::str("1")]))?)? } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_val = py::sub(&py::int_base(&v_p.clone(), &Py::Int(2i64))?, &(if ({ let t2063 = py::getitem(&v_p.clone(), &Py::Int(0i64))?; Py::Bool({ let t2064 = Py::str("1"); if py::eq(&t2063, &t2064) { true } else { false } }) }).truthy() { py::pow(&Py::Int(2i64), &v_n.clone())? } else { Py::Int(0i64) }))?;
    return Ok(py::add(&py::add(&py::float(&{ let t2065 = v_val.clone(); Py::Bool({ let t2066 = py::int(&py::getitem(&v_inputs.clone(), &Py::str("value"))?)?; if py::ne(&t2065, &t2066) { true } else { false } }) })?, &py::float(&{ let t2067 = py::int_base(&py::getitem(&v_outputs.clone(), &Py::str("hexadecimal"))?, &Py::Int(16i64))?; Py::Bool({ let t2068 = py::int_base(&v_p.clone(), &Py::Int(2i64))?; if py::ne(&t2067, &t2068) { true } else { false } }) })?)?, &py::abs(&py::sub(&py::getitem(&v_outputs.clone(), &Py::str("unsigned"))?, &py::int_base(&v_p.clone(), &Py::Int(2i64))?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_thevenin(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v__: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_els: Py = Py::None;
    let mut v_o1: Py = Py::None;
    let mut v_o2: Py = Py::None;
    let mut v_rth: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_v1: Py = Py::None;
    let mut v_v2: Py = Py::None;
    let mut v_vth: Py = Py::None;
    let t1875 = (Py::tuple(vec![py::getitem(&v_outputs.clone(), &Py::str("Vth"))?, py::getitem(&v_outputs.clone(), &Py::str("Rth"))?])).items()?;
    if t1875.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_vth = t1875[0].clone();
    v_rth = t1875[1].clone();
    let t1876 = (Py::tuple(vec![py::getitem(&v_inputs.clone(), &Py::str("node_a"))?, py::getitem(&v_inputs.clone(), &Py::str("node_b"))?])).items()?;
    if t1876.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t1876[0].clone();
    v_b = t1876[1].clone();
    v_els = f__netlist(py::getitem(&v_inputs.clone(), &Py::str("netlist"))?)?;
    let t1877 = (f__mna(v_els.clone())?).items()?;
    if t1877.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_v1 = t1877[0].clone();
    v__ = t1877[1].clone();
    let t1878 = (f__mna(py::add(&v_els.clone(), &Py::list(vec![Py::tuple(vec![Py::str("I"), Py::str("Itest_check"), v_b.clone(), v_a.clone(), Py::Float(1.0)])]))?)?).items()?;
    if t1878.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_v2 = t1878[0].clone();
    v__ = t1878[1].clone();
    let t1879 = (Py::tuple(vec![py::sub(&py::getitem(&v_v1.clone(), &v_a.clone())?, &py::getitem(&v_v1.clone(), &v_b.clone())?)?, py::sub(&py::getitem(&v_v2.clone(), &v_a.clone())?, &py::getitem(&v_v2.clone(), &v_b.clone())?)?])).items()?;
    if t1879.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_o1 = t1879[0].clone();
    v_o2 = t1879[1].clone();
    v_s = py::minmax(&[py::abs(&v_o1.clone())?, py::abs(&v_o2.clone())?, py::add(&py::abs(&v_vth.clone())?, &py::abs(&v_rth.clone())?)?, Py::Float(1e-300)], true)?;
    return Ok(py::add(&py::truediv(&py::abs(&py::sub(&v_o1.clone(), &v_vth.clone())?)?, &v_s.clone())?, &py::truediv(&py::abs(&py::sub(&v_o2.clone(), &py::add(&v_vth.clone(), &v_rth.clone())?)?)?, &v_s.clone())?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_tt(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v__: Py = Py::None;
    let mut v_bad: Py = Py::None;
    let mut v_bad2: Py = Py::None;
    let mut v_best: Py = Py::None;
    let mut v_lit: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_names: Py = Py::None;
    let mut v_on: Py = Py::None;
    let mut v_t: Py = Py::None;
    let mut v_tree: Py = Py::None;
    v_tree = f__bool_parse(py::getitem(&v_inputs.clone(), &Py::str("expression"))?)?;
    v_names = py::sorted(&f__bool_vars(v_tree.clone(), Py::None)?, (Py::Bool(false)).truthy())?;
    v_n = py::len(&v_names.clone())?;
    v_on = { let mut out = vec![]; for t2021 in py::range_iter(&[(py::pow(&Py::Int(2i64), &v_n.clone())?).i()?]) { let mut v_m: Py; 
v_m = t2021;
if (f__bool_eval(v_tree.clone(), { let mut out = vec![]; for t2022 in py::iter(&(py::enumerate(&v_names.clone(), 0)?))? { let mut v_i: Py; let mut v_nm: Py; 
let t2023 = (t2022).items()?;
if t2023.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_i = t2023[0].clone();
v_nm = t2023[1].clone();
out.push((v_nm.clone(), py::bitand(&py::rshift(&v_m.clone(), &py::sub(&py::sub(&v_n.clone(), &Py::Int(1i64))?, &v_i.clone())?)?, &Py::Int(1i64))?)); } Py::dict(out) })?).truthy() { out.push(v_m.clone()); }} Py::list(out) };
    let t2038 = (f__sop_check(py::getitem(&v_outputs.clone(), &Py::str("minimal_sop"))?, v_n.clone(), v_names.clone(), py::set(&v_on.clone())?, Py::tuple(vec![]))?).items()?;
    if t2038.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_bad = t2038[0].clone();
    v_t = t2038[1].clone();
    v_lit = t2038[2].clone();
    let t2039 = (f__sop_check(py::getitem(&v_outputs.clone(), &Py::str("canonical_sop"))?, v_n.clone(), v_names.clone(), py::set(&v_on.clone())?, Py::tuple(vec![]))?).items()?;
    if t2039.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_bad2 = t2039[0].clone();
    v__ = t2039[1].clone();
    v__ = t2039[2].clone();
    v_best = f__sop_out(v_n.clone(), v_names.clone(), v_on.clone(), Py::tuple(vec![]))?;
    return Ok(py::add(&py::add(&py::add(&py::add(&py::add(&py::add(&py::add(&v_bad.clone(), &v_bad2.clone())?, &py::abs(&py::sub(&py::getitem(&v_outputs.clone(), &Py::str("terms"))?, &v_t.clone())?)?)?, &py::abs(&py::sub(&py::getitem(&v_outputs.clone(), &Py::str("literals"))?, &v_lit.clone())?)?)?, &py::abs(&py::sub(&py::getitem(&v_outputs.clone(), &Py::str("terms"))?, &py::getitem(&v_best.clone(), &Py::str("terms"))?)?)?)?, &py::abs(&py::sub(&py::getitem(&v_outputs.clone(), &Py::str("literals"))?, &py::getitem(&v_best.clone(), &Py::str("literals"))?)?)?)?, &{ let t2040 = py::getitem(&v_outputs.clone(), &Py::str("minterms"))?; Py::Bool({ let t2041 = { let mut out = vec![]; for t2042 in py::iter(&(v_on.clone()))? { let mut v_m: Py; 
v_m = t2042;
out.push(py::float(&v_m.clone())?); } Py::list(out) }; if py::ne(&t2040, &t2041) { true } else { false } }) })?, &{ let t2043 = py::getitem(&v_outputs.clone(), &Py::str("variables"))?; Py::Bool({ let t2044 = py::method(&Py::str(", "), "join", &[v_names.clone()])?; if py::ne(&t2043, &t2044) { true } else { false } }) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_zener(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_R: Py = Py::None;
    let mut v_RL: Py = Py::None;
    let mut v_Vin: Py = Py::None;
    let mut v_Vz: Py = Py::None;
    let mut v_il: Py = Py::None;
    let mut v_ir: Py = Py::None;
    let mut v_iz: Py = Py::None;
    let t2100 = (Py::tuple(vec![py::getitem(&v_inputs.clone(), &Py::str("Vin"))?, py::getitem(&v_inputs.clone(), &Py::str("R"))?, py::getitem(&v_inputs.clone(), &Py::str("Vz"))?, py::getitem(&v_inputs.clone(), &Py::str("RL"))?])).items()?;
    if t2100.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_Vin = t2100[0].clone();
    v_R = t2100[1].clone();
    v_Vz = t2100[2].clone();
    v_RL = t2100[3].clone();
    let t2101 = (Py::tuple(vec![py::getitem(&v_outputs.clone(), &Py::str("IR"))?, py::getitem(&v_outputs.clone(), &Py::str("IL"))?, py::getitem(&v_outputs.clone(), &Py::str("IZ"))?])).items()?;
    if t2101.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_ir = t2101[0].clone();
    v_il = t2101[1].clone();
    v_iz = t2101[2].clone();
    return Ok(py::add(&py::add(&py::add(&py::add(&py::add(&py::add(&py::truediv(&py::abs(&py::sub(&py::sub(&v_Vin.clone(), &py::mul(&v_ir.clone(), &v_R.clone())?)?, &py::getitem(&v_outputs.clone(), &Py::str("VL"))?)?)?, &v_Vin.clone())?, &py::truediv(&py::abs(&py::sub(&py::getitem(&v_outputs.clone(), &Py::str("VL"))?, &v_Vz.clone())?)?, &v_Vz.clone())?)?, &py::truediv(&py::abs(&py::sub(&py::sub(&v_ir.clone(), &v_il.clone())?, &v_iz.clone())?)?, &v_ir.clone())?)?, &py::truediv(&py::abs(&py::sub(&py::mul(&v_il.clone(), &v_RL.clone())?, &py::getitem(&v_outputs.clone(), &Py::str("VL"))?)?)?, &v_Vz.clone())?)?, &f__rel(py::getitem(&v_outputs.clone(), &Py::str("IZ_mA"))?, py::mul(&v_iz.clone(), &Py::Float(1000.0))?)?)?, &py::truediv(&py::abs(&py::sub(&py::getitem(&v_outputs.clone(), &Py::str("PZ"))?, &py::mul(&v_Vz.clone(), &v_iz.clone())?)?)?, &py::mul(&v_Vz.clone(), &v_ir.clone())?)?)?, &{ let t2102 = v_iz.clone(); Py::Bool({ let t2103 = Py::Int(0i64); if py::le(&t2102, &t2103)? { true } else { false } }) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_adc_conversion(mut v_Vin: Py, mut v_Vref: Py, mut v_bits: Py) -> R {
    let mut v_code: Py = Py::None;
    let mut v_lsb: Py = Py::None;
    let mut v_n: Py = Py::None;
    v_n = py::int(&v_bits.clone())?;
    if (py::not(&{ let t1 = Py::Int(1i64); Py::Bool({ let t2 = v_n.clone(); if py::le(&t1, &t2)? { { let t3 = Py::Int(32i64); if py::le(&t2, &t3)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the resolution must be 1 to 32 bits"))));
    }
    let _ = f__check_positive(Py::dict(vec![(Py::str("Vref"), v_Vref.clone())]))?;
    if (py::not(&{ let t9 = Py::Int(0i64); Py::Bool({ let t10 = v_Vin.clone(); if py::le(&t9, &t10)? { { let t11 = v_Vref.clone(); if py::le(&t10, &t11)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the input must be between 0 and Vref"))));
    }
    v_lsb = py::truediv(&v_Vref.clone(), &py::pow(&Py::Int(2i64), &v_n.clone())?)?;
    v_code = py::minmax(&[py::int(&py::math1("floor", &py::truediv(&v_Vin.clone(), &v_lsb.clone())?)?)?, py::sub(&py::pow(&Py::Int(2i64), &v_n.clone())?, &Py::Int(1i64))?], false)?;
    return Ok(Py::dict(vec![(Py::str("code"), py::float(&v_code.clone())?), (Py::str("binary"), Py::str(&py::format(&v_code.clone(), &(Py::str(&["0".to_string(), py::format(&v_n.clone(), "")?, "b".to_string()].concat())).as_str()?)?)), (Py::str("lsb"), v_lsb.clone()), (Py::str("lsb_mV"), py::mul(&v_lsb.clone(), &Py::Float(1000.0))?), (Py::str("quantized"), py::mul(&v_code.clone(), &v_lsb.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_base_conversion(mut v_value: Py, mut v_base: Py) -> R {
    let mut v_b: Py = Py::None;
    let mut v_n: Py = Py::None;
    v_b = py::getitem(&c__BASES()?, &Py::str(&py::to_str(&v_base.clone())))?;
    v_n = py::int_base(&f__clean_digits(v_value.clone(), v_b.clone())?, &v_b.clone())?;
    if ({ let t31 = v_n.clone(); Py::Bool({ let t32 = py::pow(&Py::Int(2i64), &Py::Int(53i64))?; if py::ge(&t31, &t32)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("values up to 2^53 - 1 are supported"))));
    }
    return Ok(Py::dict(vec![(Py::str("decimal"), py::float(&v_n.clone())?), (Py::str("binary"), Py::str(&py::format(&v_n.clone(), &(Py::str("b")).as_str()?)?)), (Py::str("octal"), Py::str(&py::format(&v_n.clone(), &(Py::str("o")).as_str()?)?)), (Py::str("hexadecimal"), Py::str(&py::format(&v_n.clone(), &(Py::str("X")).as_str()?)?))]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_bjt_divider_bias(mut v_VCC: Py, mut v_R1: Py, mut v_R2: Py, mut v_RC: Py, mut v_RE: Py, mut v_beta: Py, mut v_VBE: Py) -> R {
    let mut v_ib: Py = Py::None;
    let mut v_ic: Py = Py::None;
    let mut v_ie: Py = Py::None;
    let mut v_rth: Py = Py::None;
    let mut v_vc: Py = Py::None;
    let mut v_vce: Py = Py::None;
    let mut v_ve: Py = Py::None;
    let mut v_vth: Py = Py::None;
    let _ = f__check_positive(Py::dict(vec![(Py::str("VCC"), v_VCC.clone()), (Py::str("R1"), v_R1.clone()), (Py::str("R2"), v_R2.clone()), (Py::str("RC"), v_RC.clone()), (Py::str("RE"), v_RE.clone()), (Py::str("beta"), v_beta.clone()), (Py::str("VBE"), v_VBE.clone())]))?;
    v_vth = py::truediv(&py::mul(&v_VCC.clone(), &v_R2.clone())?, &py::add(&v_R1.clone(), &v_R2.clone())?)?;
    v_rth = py::truediv(&py::mul(&v_R1.clone(), &v_R2.clone())?, &py::add(&v_R1.clone(), &v_R2.clone())?)?;
    v_ib = py::truediv(&py::sub(&v_vth.clone(), &v_VBE.clone())?, &py::add(&v_rth.clone(), &py::mul(&py::add(&v_beta.clone(), &Py::Int(1i64))?, &v_RE.clone())?)?)?;
    if ({ let t44 = v_ib.clone(); Py::Bool({ let t45 = Py::Int(0i64); if py::le(&t44, &t45)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the base-emitter junction is not forward biased: the transistor is cut off"))));
    }
    let t46 = (Py::tuple(vec![py::mul(&v_beta.clone(), &v_ib.clone())?, py::mul(&py::add(&v_beta.clone(), &Py::Int(1i64))?, &v_ib.clone())?])).items()?;
    if t46.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_ic = t46[0].clone();
    v_ie = t46[1].clone();
    v_ve = py::mul(&v_ie.clone(), &v_RE.clone())?;
    v_vc = py::sub(&v_VCC.clone(), &py::mul(&v_ic.clone(), &v_RC.clone())?)?;
    v_vce = py::sub(&v_vc.clone(), &v_ve.clone())?;
    if ({ let t47 = v_vce.clone(); Py::Bool({ let t48 = Py::Float(0.2); if py::lt(&t47, &t48)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("VCE would be below 0.2 V: the transistor is saturated and the active-region model does not apply"))));
    }
    return Ok(Py::dict(vec![(Py::str("IC"), v_ic.clone()), (Py::str("IC_mA"), py::mul(&v_ic.clone(), &Py::Float(1000.0))?), (Py::str("IB"), v_ib.clone()), (Py::str("IE"), v_ie.clone()), (Py::str("VB"), py::add(&v_ve.clone(), &v_VBE.clone())?), (Py::str("VE"), v_ve.clone()), (Py::str("VC"), v_vc.clone()), (Py::str("VCE"), v_vce.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_bode_point(mut v_num: Py, mut v_den: Py, mut v_w: Py) -> R {
    let mut v_g: Py = Py::None;
    let t59 = (Py::tuple(vec![f__ptrim(v_num.clone())?, f__ptrim(v_den.clone())?])).items()?;
    if t59.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_num = t59[0].clone();
    v_den = t59[1].clone();
    if (Py::Bool(({ let mut out = vec![]; for t60 in py::iter(&(v_den.clone()))? { let mut v_x: Py; 
v_x = t60;
out.push({ let t61 = v_x.clone(); Py::Bool({ let t62 = Py::Int(0i64); if py::eq(&t61, &t62) { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy()))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the denominator must not be zero"))));
    }
    if (py::not(&{ let t65 = { let t63 = v_w.clone(); Py::Bool({ let t64 = Py::Int(0i64); if py::gt(&t63, &t64)? { true } else { false } }) }; if !t65.truthy() { t65 } else { py::math1("isfinite", &v_w.clone())? } })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the frequency must be positive"))));
    }
    v_g = f__G(v_num.clone(), v_den.clone(), v_w.clone())?;
    if ({ let t69 = v_g.clone(); Py::Bool({ let t70 = Py::Int(0i64); if py::eq(&t69, &t70) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("G(jw) = 0 at this frequency: the magnitude in decibels is minus infinity"))));
    }
    return Ok(Py::dict(vec![(Py::str("magnitude"), py::abs(&v_g.clone())?), (Py::str("magnitude_db"), py::mul(&Py::Int(20i64), &py::math1("log10", &py::abs(&v_g.clone())?)?)?), (Py::str("phase_deg"), py::math1("degrees", &py::cmath("phase", &[v_g.clone()])?)?), (Py::str("real"), py::real(&v_g.clone())?), (Py::str("imag"), py::imag(&v_g.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_boolean_equivalence(mut v_expression1: Py, mut v_expression2: Py) -> R {
    let mut v_env: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_names: Py = Py::None;
    let mut v_t1: Py = Py::None;
    let mut v_t2: Py = Py::None;
    let t102 = (Py::tuple(vec![f__bool_parse(v_expression1.clone())?, f__bool_parse(v_expression2.clone())?])).items()?;
    if t102.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_t1 = t102[0].clone();
    v_t2 = t102[1].clone();
    v_names = py::sorted(&py::bitor(&f__bool_vars(v_t1.clone(), Py::None)?, &f__bool_vars(v_t2.clone(), Py::None)?)?, (Py::Bool(false)).truthy())?;
    if ({ let t112 = py::len(&v_names.clone())?; Py::Bool({ let t113 = Py::Int(12i64); if py::gt(&t112, &t113)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("at most 12 variables"))));
    }
    v_n = py::len(&v_names.clone())?;
    for t114 in py::range_iter(&[(py::pow(&Py::Int(2i64), &v_n.clone())?).i()?]) {
        v_m = t114;
        v_env = { let mut out = vec![]; for t115 in py::iter(&(py::enumerate(&v_names.clone(), 0)?))? { let mut v_i: Py; let mut v_nm: Py; 
let t116 = (t115).items()?;
if t116.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_i = t116[0].clone();
v_nm = t116[1].clone();
out.push((v_nm.clone(), py::bitand(&py::rshift(&v_m.clone(), &py::sub(&py::sub(&v_n.clone(), &Py::Int(1i64))?, &v_i.clone())?)?, &Py::Int(1i64))?)); } Py::dict(out) };
        if ({ let t117 = f__bool_eval(v_t1.clone(), v_env.clone())?; Py::Bool({ let t118 = f__bool_eval(v_t2.clone(), v_env.clone())?; if py::ne(&t117, &t118) { true } else { false } }) }).truthy() {
            return Ok(Py::dict(vec![(Py::str("equivalent"), Py::Bool(false)), (Py::str("counterexample"), py::method(&Py::str(", "), "join", &[{ let mut out = vec![]; for t130 in py::iter(&(py::method(&v_env.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t131 = (t130).items()?;
if t131.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t131[0].clone();
v_v = t131[1].clone();
out.push(Py::str(&[py::format(&v_k.clone(), "")?, "=".to_string(), py::format(&v_v.clone(), "")?].concat())); } Py::list(out) }])?)]));
        }
    }
    return Ok(Py::dict(vec![(Py::str("equivalent"), Py::Bool(true)), (Py::str("counterexample"), Py::str("none"))]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_boolean_evaluate(mut v_expression: Py, mut v_values: Py) -> R {
    let mut v_env: Py = Py::None;
    let mut v_missing: Py = Py::None;
    let mut v_tree: Py = Py::None;
    v_tree = f__bool_parse(v_expression.clone())?;
    v_env = f__assignments(v_values.clone())?;
    v_missing = py::sorted(&py::sub(&f__bool_vars(v_tree.clone(), Py::None)?, &py::set(&v_env.clone())?)?, (Py::Bool(false)).truthy())?;
    if (v_missing.clone()).truthy() {
        return Err(py::value_error(py::to_str(&py::add(&Py::str("give a value for "), &py::method(&Py::str(", "), "join", &[v_missing.clone()])?)?)));
    }
    return Ok(Py::dict(vec![(Py::str("value"), py::float(&f__bool_eval(v_tree.clone(), v_env.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_closed_loop_poles(mut v_num: Py, mut v_den: Py, mut v_K: Py) -> R {
    let mut v_char: Py = Py::None;
    let mut v_mr: Py = Py::None;
    let mut v_poles: Py = Py::None;
    let t211 = (f__check_tf(v_num.clone(), v_den.clone())?).items()?;
    if t211.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_num = t211[0].clone();
    v_den = t211[1].clone();
    if (py::not(&py::math1("isfinite", &v_K.clone())?)).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the gain K must be finite"))));
    }
    v_char = f__padd(v_den.clone(), { let mut out = vec![]; for t215 in py::iter(&(v_num.clone()))? { let mut v_x: Py; 
v_x = t215;
out.push(py::mul(&v_K.clone(), &v_x.clone())?); } Py::list(out) })?;
    v_char = f__ptrim(v_char.clone())?;
    if ({ let t216 = py::len(&v_char.clone())?; Py::Bool({ let t217 = Py::Int(2i64); if py::lt(&t216, &t217)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the closed-loop characteristic polynomial has no roots"))));
    }
    v_poles = f__canonical_roots(f__roots(v_char.clone())?, Py::Float(1e-09))?;
    v_mr = py::minmax(&[{ let mut out = vec![]; for t270 in py::iter(&(v_poles.clone()))? { let mut v_p: Py; 
v_p = t270;
out.push(py::real(&py::complex(&[v_p.clone()])?)?); } Py::list(out) }], true)?;
    return Ok(Py::dict(vec![(Py::str("poles"), v_poles.clone()), (Py::str("max_real_part"), v_mr.clone()), (Py::str("stable"), { let t271 = v_mr.clone(); Py::Bool({ let t272 = Py::Int(0i64); if py::lt(&t271, &t272)? { true } else { false } }) }), (Py::str("characteristic"), v_char.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_complex_power(mut v_V: Py, mut v_V_phase: Py, mut v_I: Py, mut v_I_phase: Py) -> R {
    let mut v_S: Py = Py::None;
    let mut v_nature: Py = Py::None;
    let mut v_pf: Py = Py::None;
    if (py::not(&{ let t291 = { let t287 = v_V.clone(); Py::Bool({ let t288 = Py::Int(0i64); if py::gt(&t287, &t288)? { true } else { false } }) }; if !t291.truthy() { t291 } else { { let t289 = v_I.clone(); Py::Bool({ let t290 = Py::Int(0i64); if py::gt(&t289, &t290)? { true } else { false } }) } } })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the rms voltage and current must be positive"))));
    }
    v_S = py::mul(&py::cmath("rect", &[v_V.clone(), py::math1("radians", &v_V_phase.clone())?])?, &py::method(&py::cmath("rect", &[v_I.clone(), py::math1("radians", &v_I_phase.clone())?])?, "conjugate", &[])?)?;
    v_pf = py::truediv(&py::real(&v_S.clone())?, &py::abs(&v_S.clone())?)?;
    v_nature = (if ({ let t292 = py::abs(&py::imag(&v_S.clone())?)?; Py::Bool({ let t293 = py::mul(&Py::Float(1e-12), &py::abs(&v_S.clone())?)?; if py::le(&t292, &t293)? { true } else { false } }) }).truthy() { Py::str("unity") } else { (if ({ let t294 = py::imag(&v_S.clone())?; Py::Bool({ let t295 = Py::Int(0i64); if py::gt(&t294, &t295)? { true } else { false } }) }).truthy() { Py::str("lagging") } else { Py::str("leading") }) });
    return Ok(Py::dict(vec![(Py::str("P"), py::real(&v_S.clone())?), (Py::str("Q"), py::imag(&v_S.clone())?), (Py::str("S"), py::abs(&v_S.clone())?), (Py::str("power_factor"), v_pf.clone()), (Py::str("nature"), v_nature.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_controllability_observability(mut v_A: Py, mut v_B: Py, mut v_C: Py) -> R {
    let mut v_Wc: Py = Py::None;
    let mut v_Wo: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_rc: Py = Py::None;
    let mut v_ro: Py = Py::None;
    v_n = py::len(&v_A.clone())?;
    if ({ let t305 = Py::Bool(({ let mut out = vec![]; for t297 in py::iter(&(v_A.clone()))? { let mut v_r: Py; 
v_r = t297;
out.push({ let t298 = py::len(&v_r.clone())?; Py::Bool({ let t299 = v_n.clone(); if py::ne(&t298, &t299) { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())); if t305.truthy() { t305 } else { { let t304 = { let t300 = py::len(&v_B.clone())?; Py::Bool({ let t301 = v_n.clone(); if py::ne(&t300, &t301) { true } else { false } }) }; if t304.truthy() { t304 } else { { let t302 = py::len(&v_C.clone())?; Py::Bool({ let t303 = v_n.clone(); if py::ne(&t302, &t303) { true } else { false } }) } } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("A must be n x n and B, C must have n entries"))));
    }
    let t315 = (f__ctrb_obsv(v_A.clone(), v_B.clone(), v_C.clone())?).items()?;
    if t315.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_Wc = t315[0].clone();
    v_Wo = t315[1].clone();
    let t334 = (Py::tuple(vec![f__rank(v_Wc.clone(), Py::Float(1e-09))?, f__rank(v_Wo.clone(), Py::Float(1e-09))?])).items()?;
    if t334.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_rc = t334[0].clone();
    v_ro = t334[1].clone();
    return Ok(Py::dict(vec![(Py::str("controllability_rank"), py::float(&v_rc.clone())?), (Py::str("observability_rank"), py::float(&v_ro.clone())?), (Py::str("controllable"), { let t335 = v_rc.clone(); Py::Bool({ let t336 = v_n.clone(); if py::eq(&t335, &t336) { true } else { false } }) }), (Py::str("observable"), { let t337 = v_ro.clone(); Py::Bool({ let t338 = v_n.clone(); if py::eq(&t337, &t338) { true } else { false } }) })]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_current_divider(mut v_I: Py, mut v_R1: Py, mut v_R2: Py) -> R {
    let _ = f__check_positive(Py::dict(vec![(Py::str("R1"), v_R1.clone()), (Py::str("R2"), v_R2.clone())]))?;
    return Ok(Py::dict(vec![(Py::str("I1"), py::truediv(&py::mul(&v_I.clone(), &v_R2.clone())?, &py::add(&v_R1.clone(), &v_R2.clone())?)?), (Py::str("I2"), py::truediv(&py::mul(&v_I.clone(), &v_R1.clone())?, &py::add(&v_R1.clone(), &v_R2.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_dc_motor(mut v_V: Py, mut v_R: Py, mut v_Kt: Py, mut v_Ke: Py, mut v_T: Py, mut v_b: Py) -> R {
    let mut v_i: Py = Py::None;
    let mut v_w: Py = Py::None;
    let _ = f__check_positive(Py::dict(vec![(Py::str("R"), v_R.clone()), (Py::str("Kt"), v_Kt.clone()), (Py::str("Ke"), v_Ke.clone())]))?;
    if ({ let t362 = v_b.clone(); Py::Bool({ let t363 = Py::Int(0i64); if py::lt(&t362, &t363)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the friction coefficient must not be negative"))));
    }
    v_w = py::truediv(&py::sub(&py::mul(&v_Kt.clone(), &v_V.clone())?, &py::mul(&v_R.clone(), &v_T.clone())?)?, &py::add(&py::mul(&v_Kt.clone(), &v_Ke.clone())?, &py::mul(&v_R.clone(), &v_b.clone())?)?)?;
    v_i = py::truediv(&py::add(&v_T.clone(), &py::mul(&v_b.clone(), &v_w.clone())?)?, &v_Kt.clone())?;
    return Ok(Py::dict(vec![(Py::str("speed"), v_w.clone()), (Py::str("rpm"), py::truediv(&py::mul(&v_w.clone(), &Py::Int(60i64))?, &py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?)?), (Py::str("current"), v_i.clone()), (Py::str("input_power"), py::mul(&v_V.clone(), &v_i.clone())?), (Py::str("output_power"), py::mul(&v_T.clone(), &v_w.clone())?), (Py::str("efficiency"), (if ({ let t364 = py::mul(&v_V.clone(), &v_i.clone())?; Py::Bool({ let t365 = Py::Int(0i64); if py::gt(&t364, &t365)? { true } else { false } }) }).truthy() { py::truediv(&py::mul(&v_T.clone(), &v_w.clone())?, &py::mul(&v_V.clone(), &v_i.clone())?)? } else { Py::Float(0.0) }))]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_dc_node_voltages(mut v_netlist: Py) -> R {
    let mut v_cur: Py = Py::None;
    let mut v_els: Py = Py::None;
    let mut v_volts: Py = Py::None;
    v_els = f__netlist(v_netlist.clone())?;
    let t475 = (f__mna(v_els.clone())?).items()?;
    if t475.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_volts = t475[0].clone();
    v_cur = t475[1].clone();
    return Ok(Py::dict(vec![(Py::str("node_voltages"), { let mut out = vec![]; for t476 in py::iter(&(py::method(&v_volts.clone(), "items", &[])?))? { let mut v_n: Py; let mut v_v: Py; 
let t477 = (t476).items()?;
if t477.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_n = t477[0].clone();
v_v = t477[1].clone();
if ({ let t478 = v_n.clone(); Py::Bool({ let t479 = Py::str("0"); if py::ne(&t478, &t479) { true } else { false } }) }).truthy() { out.push((v_n.clone(), v_v.clone())); }} Py::dict(out) }), (Py::str("source_currents"), v_cur.clone()), (Py::str("power_delivered"), f__power_delivered(v_els.clone(), v_volts.clone(), v_cur.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_delta_to_wye(mut v_Rab: Py, mut v_Rbc: Py, mut v_Rca: Py) -> R {
    let mut v_n: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_v: Py = Py::None;
    for t514 in py::iter(&(Py::tuple(vec![Py::tuple(vec![Py::str("Rab"), v_Rab.clone()]), Py::tuple(vec![Py::str("Rbc"), v_Rbc.clone()]), Py::tuple(vec![Py::str("Rca"), v_Rca.clone()])])))? {
        let t518 = (t514).items()?;
        if t518.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_n = t518[0].clone();
        v_v = t518[1].clone();
        if (py::not(&{ let t517 = { let t515 = v_v.clone(); Py::Bool({ let t516 = Py::Int(0i64); if py::gt(&t515, &t516)? { true } else { false } }) }; if !t517.truthy() { t517 } else { py::math1("isfinite", &v_v.clone())? } })).truthy() {
            return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_n.clone(), "")?, " must be a positive resistance".to_string()].concat()))));
        }
    }
    v_s = py::add(&py::add(&v_Rab.clone(), &v_Rbc.clone())?, &v_Rca.clone())?;
    return Ok(Py::dict(vec![(Py::str("Ra"), py::truediv(&py::mul(&v_Rab.clone(), &v_Rca.clone())?, &v_s.clone())?), (Py::str("Rb"), py::truediv(&py::mul(&v_Rab.clone(), &v_Rbc.clone())?, &v_s.clone())?), (Py::str("Rc"), py::truediv(&py::mul(&v_Rbc.clone(), &v_Rca.clone())?, &v_s.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_difference_amplifier(mut v_V1: Py, mut v_V2: Py, mut v_R1: Py, mut v_R2: Py, mut v_R3: Py, mut v_R4: Py) -> R {
    let mut v_vout: Py = Py::None;
    let _ = f__check_positive(Py::dict(vec![(Py::str("R1"), v_R1.clone()), (Py::str("R2"), v_R2.clone()), (Py::str("R3"), v_R3.clone()), (Py::str("R4"), v_R4.clone())]))?;
    v_vout = py::sub(&py::mul(&py::mul(&py::truediv(&v_R4.clone(), &py::add(&v_R3.clone(), &v_R4.clone())?)?, &py::add(&Py::Int(1i64), &py::truediv(&v_R2.clone(), &v_R1.clone())?)?)?, &v_V2.clone())?, &py::mul(&py::truediv(&v_R2.clone(), &v_R1.clone())?, &v_V1.clone())?)?;
    return Ok(Py::dict(vec![(Py::str("Vout"), v_vout.clone()), (Py::str("gain_inverting"), py::truediv(&py::neg(&v_R2.clone())?, &v_R1.clone())?), (Py::str("gain_noninverting"), py::mul(&py::truediv(&v_R4.clone(), &py::add(&v_R3.clone(), &v_R4.clone())?)?, &py::add(&Py::Int(1i64), &py::truediv(&v_R2.clone(), &v_R1.clone())?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_diode_circuit(mut v_Vs: Py, mut v_R: Py, mut v_Is: Py, mut v_n: Py, mut v_VT: Py) -> R {
    let mut v__: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_hi: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_lo: Py = Py::None;
    let mut v_mid: Py = Py::None;
    let _ = f__check_positive(Py::dict(vec![(Py::str("Vs"), v_Vs.clone()), (Py::str("R"), v_R.clone()), (Py::str("Is"), v_Is.clone()), (Py::str("n"), v_n.clone()), (Py::str("VT"), v_VT.clone())]))?;
    v_f = { let v_Is = v_Is.clone(); let v_R = v_R.clone(); let v_VT = v_VT.clone(); let v_Vs = v_Vs.clone(); let v_n = v_n.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_i: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::sub(&py::sub(&v_Vs.clone(), &py::mul(&v_i.clone(), &v_R.clone())?)?, &py::mul(&py::mul(&v_n.clone(), &v_VT.clone())?, &py::math1("log1p", &py::truediv(&v_i.clone(), &v_Is.clone())?)?)?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    let t537 = (Py::tuple(vec![Py::Float(0.0), py::truediv(&v_Vs.clone(), &v_R.clone())?])).items()?;
    if t537.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lo = t537[0].clone();
    v_hi = t537[1].clone();
    for t538 in py::range_iter(&[(Py::Int(400i64)).i()?]) {
        v__ = t538;
        v_mid = py::mul(&Py::Float(0.5), &py::add(&v_lo.clone(), &v_hi.clone())?)?;
        if ({ let t539 = v_mid.clone(); Py::Bool({ let t540 = Py::tuple(vec![v_lo.clone(), v_hi.clone()]); if py::contains(&t540, &t539)? { true } else { false } }) }).truthy() {
            break;
        }
        let t543 = ((if ({ let t541 = py::call(&v_f.clone(), &[v_mid.clone()])?; Py::Bool({ let t542 = Py::Int(0i64); if py::gt(&t541, &t542)? { true } else { false } }) }).truthy() { Py::tuple(vec![v_mid.clone(), v_hi.clone()]) } else { Py::tuple(vec![v_lo.clone(), v_mid.clone()]) })).items()?;
        if t543.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_lo = t543[0].clone();
        v_hi = t543[1].clone();
    }
    v_i = py::mul(&Py::Float(0.5), &py::add(&v_lo.clone(), &v_hi.clone())?)?;
    for t544 in py::range_iter(&[(Py::Int(3i64)).i()?]) {
        v__ = t544;
        v_d = py::sub(&py::neg(&v_R.clone())?, &py::truediv(&py::mul(&v_n.clone(), &v_VT.clone())?, &py::add(&v_Is.clone(), &v_i.clone())?)?)?;
        v_i = py::sub(&v_i, &py::truediv(&py::call(&v_f.clone(), &[v_i.clone()])?, &v_d.clone())?)?;
    }
    return Ok(Py::dict(vec![(Py::str("I"), v_i.clone()), (Py::str("I_mA"), py::mul(&v_i.clone(), &Py::Float(1000.0))?), (Py::str("VD"), py::mul(&py::mul(&v_n.clone(), &v_VT.clone())?, &py::math1("log1p", &py::truediv(&v_i.clone(), &v_Is.clone())?)?)?), (Py::str("VR"), py::mul(&v_i.clone(), &v_R.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_first_order_time_to_value(mut v_V0: Py, mut v_Vf: Py, mut v_tau: Py, mut v_v: Py) -> R {
    let mut v_frac: Py = Py::None;
    let _ = f__check_positive(Py::dict(vec![(Py::str("tau"), v_tau.clone())]))?;
    if ({ let t551 = v_V0.clone(); Py::Bool({ let t552 = v_Vf.clone(); if py::eq(&t551, &t552) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the initial and final values are equal: nothing changes"))));
    }
    v_frac = py::truediv(&py::sub(&v_v.clone(), &v_V0.clone())?, &py::sub(&v_Vf.clone(), &v_V0.clone())?)?;
    if (py::not(&{ let t553 = Py::Int(0i64); Py::Bool({ let t554 = v_frac.clone(); if py::le(&t553, &t554)? { { let t555 = Py::Int(1i64); if py::lt(&t554, &t555)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the value is never reached: it must lie between V0 and Vf (Vf itself only as t goes to infinity)"))));
    }
    return Ok(Py::dict(vec![(Py::str("t"), py::mul(&py::neg(&v_tau.clone())?, &py::math1("log1p", &py::neg(&v_frac.clone())?)?)?), (Py::str("time_constants"), py::neg(&py::math1("log1p", &py::neg(&v_frac.clone())?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_first_order_transient(mut v_V0: Py, mut v_Vf: Py, mut v_tau: Py, mut v_t: Py) -> R {
    let mut v_e: Py = Py::None;
    let _ = f__check_positive(Py::dict(vec![(Py::str("tau"), v_tau.clone())]))?;
    if ({ let t562 = v_t.clone(); Py::Bool({ let t563 = Py::Int(0i64); if py::lt(&t562, &t563)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the time must be zero or positive"))));
    }
    v_e = py::math1("exp", &py::truediv(&py::neg(&v_t.clone())?, &v_tau.clone())?)?;
    return Ok(Py::dict(vec![(Py::str("v"), py::add(&v_Vf.clone(), &py::mul(&py::sub(&v_V0.clone(), &v_Vf.clone())?, &v_e.clone())?)?), (Py::str("fraction_complete"), py::sub(&Py::Int(1i64), &v_e.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_gain_margin(mut v_num: Py, mut v_den: Py) -> R {
    let mut v_gm: Py = Py::None;
    let mut v_wp: Py = Py::None;
    let t581 = (f__check_tf(v_num.clone(), v_den.clone())?).items()?;
    if t581.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_num = t581[0].clone();
    v_den = t581[1].clone();
    v_wp = f__phase_crossover(v_num.clone(), v_den.clone())?;
    if ({ let t654 = py::len(&v_wp.clone())?; Py::Bool({ let t655 = Py::Int(1i64); if py::ne(&t654, &t655) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&py::add(&(if (py::not(&v_wp.clone())).truthy() { Py::str("the phase never reaches -180 degrees, so the gain margin is infinite") } else { Py::str(&["the phase crosses -180 degrees ".to_string(), py::format(&py::len(&v_wp.clone())?, "")?, " times".to_string()].concat()) }), &Py::str(": this capability needs exactly one phase crossover"))?)));
    }
    v_gm = py::truediv(&Py::Float(1.0), &py::abs(&f__G(v_num.clone(), v_den.clone(), py::getitem(&v_wp.clone(), &Py::Int(0i64))?)?)?)?;
    return Ok(Py::dict(vec![(Py::str("gain_margin"), v_gm.clone()), (Py::str("gain_margin_db"), py::mul(&Py::Int(20i64), &py::math1("log10", &v_gm.clone())?)?), (Py::str("phase_crossover"), py::getitem(&v_wp.clone(), &Py::Int(0i64))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_gray_code(mut v_value: Py, mut v_direction: Py) -> R {
    let mut v_n: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_x: Py = Py::None;
    v_v = py::method(&v_value.clone(), "strip", &[])?;
    if ({ let t662 = py::not(&v_v.clone()); if t662.truthy() { t662 } else { { let t661 = py::sub(&py::set(&v_v.clone())?, &py::set(&Py::list(vec![Py::str("0"), Py::str("1")]))?)?; if t661.truthy() { t661 } else { { let t659 = py::len(&v_v.clone())?; Py::Bool({ let t660 = Py::Int(32i64); if py::gt(&t659, &t660)? { true } else { false } }) } } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the value must be 1 to 32 binary digits"))));
    }
    v_n = py::len(&v_v.clone())?;
    v_x = py::int_base(&v_v.clone(), &Py::Int(2i64))?;
    if ({ let t663 = v_direction.clone(); Py::Bool({ let t664 = Py::str("to_gray"); if py::eq(&t663, &t664) { true } else { false } }) }).truthy() {
        v_r = py::bitxor(&v_x.clone(), &py::rshift(&v_x.clone(), &Py::Int(1i64))?)?;
    } else {
        let t665 = (Py::tuple(vec![v_x.clone(), py::rshift(&v_x.clone(), &Py::Int(1i64))?])).items()?;
        if t665.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_r = t665[0].clone();
        v_s = t665[1].clone();
        while (v_s.clone()).truthy() {
            v_r = py::bitxor(&v_r, &v_s.clone())?;
            v_s = py::rshift(&v_s, &Py::Int(1i64))?;
        }
    }
    return Ok(Py::dict(vec![(Py::str("result"), Py::str(&py::format(&v_r.clone(), &(Py::str(&["0".to_string(), py::format(&v_n.clone(), "")?, "b".to_string()].concat())).as_str()?)?)), (Py::str("decimal"), py::float(&v_r.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_inductor_energy(mut v_L: Py, mut v_I: Py) -> R {
    let _ = f__check_positive(Py::dict(vec![(Py::str("L"), v_L.clone())]))?;
    return Ok(Py::dict(vec![(Py::str("energy"), py::mul(&py::mul(&py::mul(&Py::Float(0.5), &v_L.clone())?, &v_I.clone())?, &v_I.clone())?), (Py::str("flux_linkage"), py::mul(&v_L.clone(), &v_I.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_max_power_transfer(mut v_Vth: Py, mut v_Rth: Py) -> R {
    let _ = f__check_positive(Py::dict(vec![(Py::str("Rth"), v_Rth.clone())]))?;
    return Ok(Py::dict(vec![(Py::str("RL"), v_Rth.clone()), (Py::str("Pmax"), py::truediv(&py::mul(&v_Vth.clone(), &v_Vth.clone())?, &py::mul(&Py::Int(4i64), &v_Rth.clone())?)?), (Py::str("efficiency"), Py::Float(0.5))]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_minterms_to_sop(mut v_minterms: Py, mut v_variables: Py, mut v_dont_cares: Py) -> R {
    let mut v_dc: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_names: Py = Py::None;
    let mut v_on: Py = Py::None;
    let mut v_out: Py = Py::None;
    v_n = py::int(&v_variables.clone())?;
    if (py::not(&{ let t747 = Py::Int(1i64); Py::Bool({ let t748 = v_n.clone(); if py::le(&t747, &t748)? { { let t749 = Py::Int(4i64); if py::le(&t748, &t749)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("minimization is offered for 1 to 4 variables"))));
    }
    v_on = py::sorted(&py::set(&{ let mut out = vec![]; for t750 in py::iter(&(v_minterms.clone()))? { let mut v_m: Py; 
v_m = t750;
out.push(py::int(&v_m.clone())?); } Py::list(out) })?, (Py::Bool(false)).truthy())?;
    v_dc = py::sorted(&py::set(&{ let mut out = vec![]; for t752 in py::iter(&({ let t751 = v_dont_cares.clone(); if t751.truthy() { t751 } else { Py::list(vec![]) } }))? { let mut v_m: Py; 
v_m = t752;
out.push(py::int(&v_m.clone())?); } Py::list(out) })?, (Py::Bool(false)).truthy())?;
    for t754 in py::iter(&(py::add(&Py::list((v_minterms.clone()).items()?), &Py::list(({ let t753 = v_dont_cares.clone(); if t753.truthy() { t753 } else { Py::list(vec![]) } }).items()?))?))? {
        v_m = t754;
        if ({ let t760 = { let t755 = v_m.clone(); Py::Bool({ let t756 = py::int(&v_m.clone())?; if py::ne(&t755, &t756) { true } else { false } }) }; if t760.truthy() { t760 } else { py::not(&{ let t757 = Py::Int(0i64); Py::Bool({ let t758 = v_m.clone(); if py::le(&t757, &t758)? { { let t759 = py::pow(&Py::Int(2i64), &v_n.clone())?; if py::lt(&t758, &t759)? { true } else { false } } } else { false } }) }) } }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str(&["minterm numbers must be whole numbers from 0 to ".to_string(), py::format(&py::sub(&py::pow(&Py::Int(2i64), &v_n.clone())?, &Py::Int(1i64))?, "")?].concat()))));
        }
    }
    if (py::bitand(&py::set(&v_on.clone())?, &py::set(&v_dc.clone())?)?).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a row cannot be both a minterm and a don't-care"))));
    }
    v_names = py::slice(&Py::str("ABCD"), None, Some((v_n.clone()).i()?), None)?;
    v_out = f__sop_out(v_n.clone(), v_names.clone(), v_on.clone(), v_dc.clone())?;
    py::setitem(&v_out.clone(), &Py::str("variables"), py::method(&Py::str(", "), "join", &[v_names.clone()])?)?;
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_mosfet_saturation(mut v_kn: Py, mut v_WL: Py, mut v_VGS: Py, mut v_VTH: Py, mut v_VDS: Py) -> R {
    let mut v_idd: Py = Py::None;
    let mut v_vov: Py = Py::None;
    let _ = f__check_positive(Py::dict(vec![(Py::str("kn"), v_kn.clone()), (Py::str("WL"), v_WL.clone())]))?;
    v_vov = py::sub(&v_VGS.clone(), &v_VTH.clone())?;
    if ({ let t881 = v_vov.clone(); Py::Bool({ let t882 = Py::Int(0i64); if py::le(&t881, &t882)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("VGS does not exceed the threshold voltage: the transistor is off"))));
    }
    if ({ let t883 = v_VDS.clone(); Py::Bool({ let t884 = v_vov.clone(); if py::lt(&t883, &t884)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("VDS < VGS - VTH: the transistor is in the triode region, not saturation"))));
    }
    v_idd = py::mul(&py::mul(&py::mul(&py::mul(&Py::Float(0.5), &v_kn.clone())?, &v_WL.clone())?, &v_vov.clone())?, &v_vov.clone())?;
    return Ok(Py::dict(vec![(Py::str("ID"), v_idd.clone()), (Py::str("ID_mA"), py::mul(&v_idd.clone(), &Py::Float(1000.0))?), (Py::str("VOV"), v_vov.clone()), (Py::str("gm"), py::truediv(&py::mul(&Py::Int(2i64), &v_idd.clone())?, &v_vov.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_norton_equivalent(mut v_netlist: Py, mut v_node_a: Py, mut v_node_b: Py) -> R {
    let mut v_rth: Py = Py::None;
    let mut v_vth: Py = Py::None;
    let t1070 = (f__thevenin(v_netlist.clone(), v_node_a.clone(), v_node_b.clone())?).items()?;
    if t1070.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_vth = t1070[0].clone();
    v_rth = t1070[1].clone();
    return Ok(Py::dict(vec![(Py::str("In"), py::truediv(&v_vth.clone(), &v_rth.clone())?), (Py::str("Rn"), v_rth.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_opamp_integrator(mut v_R: Py, mut v_C: Py, mut v_Vin: Py, mut v_t: Py, mut v_V0: Py) -> R {
    let _ = f__check_positive(Py::dict(vec![(Py::str("R"), v_R.clone()), (Py::str("C"), v_C.clone())]))?;
    if ({ let t1081 = v_t.clone(); Py::Bool({ let t1082 = Py::Int(0i64); if py::lt(&t1081, &t1082)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the time must be zero or positive"))));
    }
    return Ok(Py::dict(vec![(Py::str("Vout"), py::sub(&v_V0.clone(), &py::truediv(&py::mul(&v_Vin.clone(), &v_t.clone())?, &py::mul(&v_R.clone(), &v_C.clone())?)?)?), (Py::str("slope"), py::truediv(&py::neg(&v_Vin.clone())?, &py::mul(&v_R.clone(), &v_C.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_parallel_rlc_impedance(mut v_R: Py, mut v_L: Py, mut v_C: Py, mut v_f: Py) -> R {
    let mut v_B: Py = Py::None;
    let mut v_Y: Py = Py::None;
    let mut v_w: Py = Py::None;
    let _ = f__check_positive(Py::dict(vec![(Py::str("R"), v_R.clone()), (Py::str("L"), v_L.clone()), (Py::str("C"), v_C.clone()), (Py::str("f"), v_f.clone())]))?;
    v_w = py::mul(&py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?, &v_f.clone())?;
    v_B = py::sub(&py::mul(&v_w.clone(), &v_C.clone())?, &py::truediv(&Py::Float(1.0), &py::mul(&v_w.clone(), &v_L.clone())?)?)?;
    if ({ let t1131 = py::abs(&v_B.clone())?; Py::Bool({ let t1132 = py::mul(&Py::Float(1e-12), &py::minmax(&[py::mul(&v_w.clone(), &v_C.clone())?, py::truediv(&Py::Float(1.0), &py::mul(&v_w.clone(), &v_L.clone())?)?], true)?)?; if py::le(&t1131, &t1132)? { true } else { false } }) }).truthy() {
        v_B = Py::Float(0.0);
    }
    v_Y = py::complex(&[py::truediv(&Py::Float(1.0), &v_R.clone())?, v_B.clone()])?;
    return Ok(f__impedance_out(py::truediv(&Py::Float(1.0), &v_Y.clone())?, v_w.clone(), v_L.clone(), v_C.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_phase_margin(mut v_num: Py, mut v_den: Py) -> R {
    let mut v_g: Py = Py::None;
    let mut v_pm: Py = Py::None;
    let mut v_wc: Py = Py::None;
    let t1160 = (f__check_tf(v_num.clone(), v_den.clone())?).items()?;
    if t1160.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_num = t1160[0].clone();
    v_den = t1160[1].clone();
    v_wc = f__gain_crossover(v_num.clone(), v_den.clone())?;
    if ({ let t1234 = py::len(&v_wc.clone())?; Py::Bool({ let t1235 = Py::Int(1i64); if py::ne(&t1234, &t1235) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&py::add(&py::add(&Py::str("|G(jw)| crosses 1 "), &(if (py::not(&v_wc.clone())).truthy() { Py::str("nowhere") } else { Py::str(&[py::format(&py::len(&v_wc.clone())?, "")?, " times".to_string()].concat()) }))?, &Py::str(": this capability needs exactly one gain crossover"))?)));
    }
    v_g = f__G(v_num.clone(), v_den.clone(), py::getitem(&v_wc.clone(), &Py::Int(0i64))?)?;
    v_pm = py::sub(&py::modulo(&py::math1("degrees", &py::cmath("phase", &[v_g.clone()])?)?, &Py::Float(360.0))?, &Py::Float(180.0))?;
    return Ok(Py::dict(vec![(Py::str("phase_margin"), v_pm.clone()), (Py::str("gain_crossover"), py::getitem(&v_wc.clone(), &Py::Int(0i64))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_phasor_sum(mut v_amplitudes: Py, mut v_phases_deg: Py) -> R {
    let mut v_z: Py = Py::None;
    if ({ let t1237 = py::len(&v_amplitudes.clone())?; Py::Bool({ let t1238 = py::len(&v_phases_deg.clone())?; if py::ne(&t1237, &t1238) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("give one phase for every amplitude"))));
    }
    if ({ let t1239 = py::len(&v_amplitudes.clone())?; Py::Bool({ let t1240 = Py::Int(2i64); if py::lt(&t1239, &t1240)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("give at least two phasors"))));
    }
    if (Py::Bool(({ let mut out = vec![]; for t1241 in py::iter(&(v_amplitudes.clone()))? { let mut v_a: Py; 
v_a = t1241;
out.push({ let t1242 = v_a.clone(); Py::Bool({ let t1243 = Py::Int(0i64); if py::lt(&t1242, &t1243)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("amplitudes must not be negative"))));
    }
    v_z = py::sum(&{ let mut out = vec![]; for t1244 in py::iter(&(py::zip(&[v_amplitudes.clone(), v_phases_deg.clone()])?))? { let mut v_a: Py; let mut v_p: Py; 
let t1245 = (t1244).items()?;
if t1245.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t1245[0].clone();
v_p = t1245[1].clone();
out.push(py::mul(&v_a.clone(), &py::cmath("exp", &[py::mul(&Py::Complex(0.0, 1.0), &py::math1("radians", &v_p.clone())?)?])?)?); } Py::list(out) }, None)?;
    if ({ let t1246 = py::abs(&v_z.clone())?; Py::Bool({ let t1247 = py::mul(&Py::Float(1e-12), &py::minmax(&[v_amplitudes.clone()], true)?)?; if py::le(&t1246, &t1247)? { true } else { false } }) }).truthy() {
        return Ok(Py::dict(vec![(Py::str("amplitude"), Py::Float(0.0)), (Py::str("phase_deg"), Py::Float(0.0)), (Py::str("real"), Py::Float(0.0)), (Py::str("imag"), Py::Float(0.0))]));
    }
    return Ok(Py::dict(vec![(Py::str("amplitude"), py::abs(&v_z.clone())?), (Py::str("phase_deg"), py::math1("degrees", &py::cmath("phase", &[v_z.clone()])?)?), (Py::str("real"), py::real(&v_z.clone())?), (Py::str("imag"), py::imag(&v_z.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_polar_to_rectangular(mut v_magnitude: Py, mut v_angle_deg: Py) -> R {
    let mut v_a: Py = Py::None;
    if (py::not(&{ let t1253 = py::math1("isfinite", &v_magnitude.clone())?; if !t1253.truthy() { t1253 } else { py::math1("isfinite", &v_angle_deg.clone())? } })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("magnitude and angle must be finite"))));
    }
    if ({ let t1254 = v_magnitude.clone(); Py::Bool({ let t1255 = Py::Int(0i64); if py::lt(&t1254, &t1255)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the magnitude must not be negative"))));
    }
    v_a = py::math1("radians", &v_angle_deg.clone())?;
    return Ok(Py::dict(vec![(Py::str("real"), py::mul(&v_magnitude.clone(), &py::math1("cos", &v_a.clone())?)?), (Py::str("imag"), py::mul(&v_magnitude.clone(), &py::math1("sin", &v_a.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_power_factor_correction(mut v_P: Py, mut v_V: Py, mut v_f: Py, mut v_pf_initial: Py, mut v_pf_target: Py) -> R {
    let mut v_C: Py = Py::None;
    let mut v_q1: Py = Py::None;
    let mut v_q2: Py = Py::None;
    let mut v_qc: Py = Py::None;
    let _ = f__check_positive(Py::dict(vec![(Py::str("P"), v_P.clone()), (Py::str("V"), v_V.clone()), (Py::str("f"), v_f.clone())]))?;
    if (py::not(&{ let t1272 = { let t1266 = Py::Int(0i64); Py::Bool({ let t1267 = v_pf_initial.clone(); if py::lt(&t1266, &t1267)? { { let t1268 = Py::Int(1i64); if py::le(&t1267, &t1268)? { true } else { false } } } else { false } }) }; if !t1272.truthy() { t1272 } else { { let t1269 = Py::Int(0i64); Py::Bool({ let t1270 = v_pf_target.clone(); if py::lt(&t1269, &t1270)? { { let t1271 = Py::Int(1i64); if py::le(&t1270, &t1271)? { true } else { false } } } else { false } }) } } })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("power factors must be between 0 and 1"))));
    }
    if ({ let t1273 = v_pf_target.clone(); Py::Bool({ let t1274 = v_pf_initial.clone(); if py::le(&t1273, &t1274)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the target power factor must be higher than the present one"))));
    }
    v_q1 = py::mul(&v_P.clone(), &py::math1("tan", &py::math1("acos", &v_pf_initial.clone())?)?)?;
    v_q2 = py::mul(&v_P.clone(), &py::math1("tan", &py::math1("acos", &v_pf_target.clone())?)?)?;
    v_qc = py::sub(&v_q1.clone(), &v_q2.clone())?;
    v_C = py::truediv(&v_qc.clone(), &py::mul(&py::mul(&py::mul(&py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?, &v_f.clone())?, &v_V.clone())?, &v_V.clone())?)?;
    return Ok(Py::dict(vec![(Py::str("C"), v_C.clone()), (Py::str("C_uF"), py::mul(&v_C.clone(), &Py::Float(1000000.0))?), (Py::str("Qc"), v_qc.clone()), (Py::str("Q_initial"), v_q1.clone()), (Py::str("Q_final"), v_q2.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_rc_filter(mut v_R: Py, mut v_C: Py, mut v_f: Py, mut v_filter: Py) -> R {
    let mut v_fc: Py = Py::None;
    let mut v_h: Py = Py::None;
    let mut v_x: Py = Py::None;
    let _ = f__check_positive(Py::dict(vec![(Py::str("R"), v_R.clone()), (Py::str("C"), v_C.clone()), (Py::str("f"), v_f.clone())]))?;
    v_fc = py::truediv(&Py::Int(1i64), &py::mul(&py::mul(&py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?, &v_R.clone())?, &v_C.clone())?)?;
    v_x = py::truediv(&v_f.clone(), &v_fc.clone())?;
    v_h = (if ({ let t1281 = v_filter.clone(); Py::Bool({ let t1282 = Py::str("lowpass"); if py::eq(&t1281, &t1282) { true } else { false } }) }).truthy() { py::truediv(&Py::Int(1i64), &py::complex(&[Py::Int(1i64), v_x.clone()])?)? } else { py::truediv(&py::complex(&[Py::Int(0i64), v_x.clone()])?, &py::complex(&[Py::Int(1i64), v_x.clone()])?)? });
    return Ok(Py::dict(vec![(Py::str("fc"), v_fc.clone()), (Py::str("gain"), py::abs(&v_h.clone())?), (Py::str("gain_db"), py::mul(&Py::Int(20i64), &py::math1("log10", &py::abs(&v_h.clone())?)?)?), (Py::str("phase_deg"), py::math1("degrees", &py::cmath("phase", &[v_h.clone()])?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_rectangular_to_polar(mut v_real: Py, mut v_imag: Py) -> R {
    if ({ let t1290 = { let t1286 = v_real.clone(); Py::Bool({ let t1287 = Py::Int(0i64); if py::eq(&t1286, &t1287) { true } else { false } }) }; if !t1290.truthy() { t1290 } else { { let t1288 = v_imag.clone(); Py::Bool({ let t1289 = Py::Int(0i64); if py::eq(&t1288, &t1289) { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the phasor 0 has no angle"))));
    }
    return Ok(Py::dict(vec![(Py::str("magnitude"), py::hypot(&[v_real.clone(), v_imag.clone()])?), (Py::str("angle_deg"), py::math1("degrees", &py::atan2(&v_imag.clone(), &v_real.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_resistor_network(mut v_network: Py) -> R {
    let mut v_tree: Py = Py::None;
    v_tree = f__net_tree(v_network.clone())?;
    if (Py::Bool(matches!(v_tree.clone(), Py::Float(_)))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the expression has a single resistor: combine at least two with + or ||"))));
    }
    return Ok(Py::dict(vec![(Py::str("equivalent"), f__net_value(v_tree.clone())?), (Py::str("resistors"), py::float(&f__net_count(v_tree.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_rms_waveform(mut v_shape: Py, mut v_amplitude: Py, mut v_offset: Py) -> R {
    let mut v_rms: Py = Py::None;
    if ({ let t1334 = v_amplitude.clone(); Py::Bool({ let t1335 = Py::Int(0i64); if py::lt(&t1334, &t1335)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the amplitude must not be negative"))));
    }
    v_rms = py::math1("sqrt", &py::add(&py::mul(&v_offset.clone(), &v_offset.clone())?, &py::mul(&py::mul(&py::getitem(&c__SHAPES()?, &v_shape.clone())?, &v_amplitude.clone())?, &v_amplitude.clone())?)?)?;
    return Ok(Py::dict(vec![(Py::str("rms"), v_rms.clone()), (Py::str("average"), v_offset.clone()), (Py::str("ac_rms"), py::mul(&v_amplitude.clone(), &py::math1("sqrt", &py::getitem(&c__SHAPES()?, &v_shape.clone())?)?)?), (Py::str("crest_factor"), (if (v_rms.clone()).truthy() { py::truediv(&py::add(&py::abs(&v_offset.clone())?, &v_amplitude.clone())?, &v_rms.clone())? } else { Py::Float(0.0) }))]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_routh_hurwitz(mut v_coefficients: Py) -> R {
    let mut v__: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_changes: Py = Py::None;
    let mut v_first: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_new: Py = Py::None;
    let mut v_r0: Py = Py::None;
    let mut v_r1: Py = Py::None;
    let mut v_rows: Py = Py::None;
    let mut v_scale: Py = Py::None;
    let mut v_width: Py = Py::None;
    v_c = f__ptrim(v_coefficients.clone())?;
    if ({ let t1343 = py::len(&v_c.clone())?; Py::Bool({ let t1344 = Py::Int(2i64); if py::lt(&t1343, &t1344)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("give a polynomial of degree 1 or more"))));
    }
    if (Py::Bool(({ let mut out = vec![]; for t1345 in py::iter(&(v_c.clone()))? { let mut v_x: Py; 
v_x = t1345;
out.push(py::not(&py::math1("isfinite", &v_x.clone())?)); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("coefficients must be finite"))));
    }
    v_n = py::sub(&py::len(&v_c.clone())?, &Py::Int(1i64))?;
    v_width = py::add(&py::floordiv(&v_n.clone(), &Py::Int(2i64))?, &Py::Int(1i64))?;
    v_r0 = py::add(&py::slice(&v_c.clone(), Some((Py::Int(0i64)).i()?), None, Some((Py::Int(2i64)).i()?))?, &py::mul(&Py::list(vec![Py::Float(0.0)]), &py::sub(&v_width.clone(), &py::len(&py::slice(&v_c.clone(), Some((Py::Int(0i64)).i()?), None, Some((Py::Int(2i64)).i()?))?)?)?)?)?;
    v_r1 = py::add(&py::slice(&v_c.clone(), Some((Py::Int(1i64)).i()?), None, Some((Py::Int(2i64)).i()?))?, &py::mul(&Py::list(vec![Py::Float(0.0)]), &py::sub(&v_width.clone(), &py::len(&py::slice(&v_c.clone(), Some((Py::Int(1i64)).i()?), None, Some((Py::Int(2i64)).i()?))?)?)?)?)?;
    v_rows = Py::list(vec![v_r0.clone(), v_r1.clone()]);
    v_scale = py::minmax(&[{ let mut out = vec![]; for t1346 in py::iter(&(v_c.clone()))? { let mut v_x: Py; 
v_x = t1346;
out.push(py::abs(&v_x.clone())?); } Py::list(out) }], true)?;
    for t1347 in py::range_iter(&[(py::sub(&v_n.clone(), &Py::Int(1i64))?).i()?]) {
        v__ = t1347;
        let t1348 = (Py::tuple(vec![py::getitem(&v_rows.clone(), &py::neg(&Py::Int(2i64))?)?, py::getitem(&v_rows.clone(), &py::neg(&Py::Int(1i64))?)?])).items()?;
        if t1348.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_a = t1348[0].clone();
        v_b = t1348[1].clone();
        if ({ let t1349 = py::abs(&py::getitem(&v_b.clone(), &Py::Int(0i64))?)?; Py::Bool({ let t1350 = py::mul(&Py::Float(1e-12), &v_scale.clone())?; if py::le(&t1349, &t1350)? { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("a zero appears in the first column of the Routh array (a special case: roots on or symmetric about the imaginary axis); this capability does not handle it"))));
        }
        v_new = py::add(&{ let mut out = vec![]; for t1351 in py::range_iter(&[(py::sub(&v_width.clone(), &Py::Int(1i64))?).i()?]) { let mut v_i: Py; 
v_i = t1351;
out.push(py::truediv(&py::sub(&py::mul(&py::getitem(&v_b.clone(), &Py::Int(0i64))?, &py::getitem(&v_a.clone(), &py::add(&v_i.clone(), &Py::Int(1i64))?)?)?, &py::mul(&py::getitem(&v_a.clone(), &Py::Int(0i64))?, &py::getitem(&v_b.clone(), &py::add(&v_i.clone(), &Py::Int(1i64))?)?)?)?, &py::getitem(&v_b.clone(), &Py::Int(0i64))?)?); } Py::list(out) }, &Py::list(vec![Py::Float(0.0)]))?;
        let _ = py::method(&v_rows.clone(), "append", &[v_new.clone()])?;
        v_scale = py::minmax(&[v_scale.clone(), py::minmax(&[{ let mut out = vec![]; for t1352 in py::iter(&(v_new.clone()))? { let mut v_x: Py; 
v_x = t1352;
out.push(py::abs(&v_x.clone())?); } Py::list(out) }], true)?], true)?;
    }
    v_first = { let mut out = vec![]; for t1353 in py::iter(&(v_rows.clone()))? { let mut v_r: Py; 
v_r = t1353;
out.push(py::getitem(&v_r.clone(), &Py::Int(0i64))?); } Py::list(out) };
    if (Py::Bool(({ let mut out = vec![]; for t1354 in py::iter(&(v_first.clone()))? { let mut v_x: Py; 
v_x = t1354;
out.push({ let t1355 = py::abs(&v_x.clone())?; Py::Bool({ let t1356 = py::mul(&Py::Float(1e-12), &v_scale.clone())?; if py::le(&t1355, &t1356)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a zero appears in the first column of the Routh array (a special case); this capability does not handle it"))));
    }
    v_changes = py::sum(&{ let mut out = vec![]; for t1357 in py::iter(&(py::zip(&[v_first.clone(), py::slice(&v_first.clone(), Some((Py::Int(1i64)).i()?), None, None)?])?))? { let mut v_x: Py; let mut v_y: Py; 
let t1358 = (t1357).items()?;
if t1358.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t1358[0].clone();
v_y = t1358[1].clone();
if ({ let t1359 = { let t1361 = v_x.clone(); Py::Bool({ let t1362 = Py::Int(0i64); if py::gt(&t1361, &t1362)? { true } else { false } }) }; Py::Bool({ let t1360 = { let t1363 = v_y.clone(); Py::Bool({ let t1364 = Py::Int(0i64); if py::gt(&t1363, &t1364)? { true } else { false } }) }; if py::ne(&t1359, &t1360) { true } else { false } }) }).truthy() { out.push(Py::Int(1i64)); }} Py::list(out) }, None)?;
    return Ok(Py::dict(vec![(Py::str("rhp_roots"), py::float(&v_changes.clone())?), (Py::str("stable"), { let t1365 = v_changes.clone(); Py::Bool({ let t1366 = Py::Int(0i64); if py::eq(&t1365, &t1366) { true } else { false } }) }), (Py::str("first_column"), v_first.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_second_order_step(mut v_zeta: Py, mut v_wn: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_mp: Py = Py::None;
    let mut v_q: Py = Py::None;
    let mut v_sg: Py = Py::None;
    let mut v_t10: Py = Py::None;
    let mut v_t90: Py = Py::None;
    let mut v_tk: Py = Py::None;
    let mut v_tp: Py = Py::None;
    let mut v_ts: Py = Py::None;
    let mut v_tz: Py = Py::None;
    let mut v_wd: Py = Py::None;
    let mut v_y: Py = Py::None;
    if (py::not(&{ let t1422 = Py::Int(0i64); Py::Bool({ let t1423 = v_zeta.clone(); if py::lt(&t1422, &t1423)? { { let t1424 = Py::Int(1i64); if py::lt(&t1423, &t1424)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the damping ratio must be between 0 and 1 (an underdamped system)"))));
    }
    if (py::not(&{ let t1427 = { let t1425 = v_wn.clone(); Py::Bool({ let t1426 = Py::Int(0i64); if py::gt(&t1425, &t1426)? { true } else { false } }) }; if !t1427.truthy() { t1427 } else { py::math1("isfinite", &v_wn.clone())? } })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the natural frequency must be positive"))));
    }
    v_q = py::math1("sqrt", &py::sub(&Py::Int(1i64), &py::mul(&v_zeta.clone(), &v_zeta.clone())?)?)?;
    let t1428 = (Py::tuple(vec![py::mul(&v_wn.clone(), &v_q.clone())?, py::mul(&v_zeta.clone(), &v_wn.clone())?])).items()?;
    if t1428.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_wd = t1428[0].clone();
    v_sg = t1428[1].clone();
    v_y = { let v_q = v_q.clone(); let v_sg = v_sg.clone(); let v_wd = v_wd.clone(); let v_wn = v_wn.clone(); let v_y = v_y.clone(); let v_zeta = v_zeta.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_t: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::sub(&Py::Int(1i64), &py::mul(&py::math1("exp", &py::mul(&py::neg(&v_sg.clone())?, &v_t.clone())?)?, &py::add(&py::math1("cos", &py::mul(&v_wd.clone(), &v_t.clone())?)?, &py::mul(&py::truediv(&v_zeta.clone(), &v_q.clone())?, &py::math1("sin", &py::mul(&v_wd.clone(), &v_t.clone())?)?)?)?)?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_tp = py::truediv(&Py::Float(std::f64::consts::PI), &v_wd.clone())?;
    v_mp = py::math1("exp", &py::truediv(&py::mul(&py::neg(&v_zeta.clone())?, &Py::Float(std::f64::consts::PI))?, &v_q.clone())?)?;
    v_t10 = f__bisect({ let v_mp = v_mp.clone(); let v_q = v_q.clone(); let v_sg = v_sg.clone(); let v_tp = v_tp.clone(); let v_wd = v_wd.clone(); let v_wn = v_wn.clone(); let v_y = v_y.clone(); let v_zeta = v_zeta.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_t: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::sub(&py::call(&v_y.clone(), &[v_t.clone()])?, &Py::Float(0.1))?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, Py::Float(0.0), v_tp.clone())?;
    v_t90 = f__bisect({ let v_mp = v_mp.clone(); let v_q = v_q.clone(); let v_sg = v_sg.clone(); let v_t10 = v_t10.clone(); let v_tp = v_tp.clone(); let v_wd = v_wd.clone(); let v_wn = v_wn.clone(); let v_y = v_y.clone(); let v_zeta = v_zeta.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_t: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::sub(&py::call(&v_y.clone(), &[v_t.clone()])?, &Py::Float(0.9))?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, Py::Float(0.0), v_tp.clone())?;
    v_k = py::int(&py::math1("floor", &py::truediv(&py::mul(&py::math1("log", &py::truediv(&Py::Int(1i64), &Py::Float(0.02))?)?, &v_wd.clone())?, &py::mul(&v_sg.clone(), &Py::Float(std::f64::consts::PI))?)?)?)?;
    while ({ let t1443 = { let t1439 = v_k.clone(); Py::Bool({ let t1440 = Py::Int(0i64); if py::gt(&t1439, &t1440)? { true } else { false } }) }; if !t1443.truthy() { t1443 } else { { let t1441 = py::math1("exp", &py::truediv(&py::mul(&py::mul(&py::neg(&v_sg.clone())?, &v_k.clone())?, &Py::Float(std::f64::consts::PI))?, &v_wd.clone())?)?; Py::Bool({ let t1442 = Py::Float(0.02); if py::le(&t1441, &t1442)? { true } else { false } }) } } }).truthy() {
        v_k = py::sub(&v_k, &Py::Int(1i64))?;
    }
    v_tk = py::truediv(&py::mul(&v_k.clone(), &Py::Float(std::f64::consts::PI))?, &v_wd.clone())?;
    v_tz = py::truediv(&py::add(&py::add(&py::truediv(&Py::Float(std::f64::consts::PI), &Py::Int(2i64))?, &py::math1("asin", &v_zeta.clone())?)?, &py::mul(&v_k.clone(), &Py::Float(std::f64::consts::PI))?)?, &v_wd.clone())?;
    v_ts = f__bisect({ let v_k = v_k.clone(); let v_mp = v_mp.clone(); let v_q = v_q.clone(); let v_sg = v_sg.clone(); let v_t10 = v_t10.clone(); let v_t90 = v_t90.clone(); let v_tk = v_tk.clone(); let v_tp = v_tp.clone(); let v_tz = v_tz.clone(); let v_wd = v_wd.clone(); let v_wn = v_wn.clone(); let v_y = v_y.clone(); let v_zeta = v_zeta.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_t: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::sub(&py::abs(&py::sub(&py::call(&v_y.clone(), &[v_t.clone()])?, &Py::Int(1i64))?)?, &Py::Float(0.02))?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, v_tk.clone(), v_tz.clone())?;
    return Ok(Py::dict(vec![(Py::str("percent_overshoot"), py::mul(&Py::Int(100i64), &v_mp.clone())?), (Py::str("overshoot"), v_mp.clone()), (Py::str("peak_time"), v_tp.clone()), (Py::str("rise_time"), py::sub(&v_t90.clone(), &v_t10.clone())?), (Py::str("t10"), v_t10.clone()), (Py::str("t90"), v_t90.clone()), (Py::str("settling_time"), v_ts.clone()), (Py::str("damped_frequency"), v_wd.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_series_rlc_impedance(mut v_R: Py, mut v_L: Py, mut v_C: Py, mut v_f: Py) -> R {
    let mut v_X: Py = Py::None;
    let mut v_w: Py = Py::None;
    if (py::not(&{ let t1491 = py::math1("isfinite", &v_R.clone())?; if !t1491.truthy() { t1491 } else { { let t1489 = v_R.clone(); Py::Bool({ let t1490 = Py::Int(0i64); if py::ge(&t1489, &t1490)? { true } else { false } }) } } })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("R must be zero or positive"))));
    }
    let _ = f__check_positive(Py::dict(vec![(Py::str("L"), v_L.clone()), (Py::str("C"), v_C.clone()), (Py::str("f"), v_f.clone())]))?;
    v_w = py::mul(&py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?, &v_f.clone())?;
    v_X = py::sub(&py::mul(&v_w.clone(), &v_L.clone())?, &py::truediv(&Py::Float(1.0), &py::mul(&v_w.clone(), &v_C.clone())?)?)?;
    if ({ let t1497 = py::abs(&v_X.clone())?; Py::Bool({ let t1498 = py::mul(&Py::Float(1e-12), &py::minmax(&[py::mul(&v_w.clone(), &v_L.clone())?, py::truediv(&Py::Float(1.0), &py::mul(&v_w.clone(), &v_C.clone())?)?], true)?)?; if py::le(&t1497, &t1498)? { true } else { false } }) }).truthy() {
        v_X = Py::Float(0.0);
    }
    return Ok(f__impedance_out(py::complex(&[v_R.clone(), v_X.clone()])?, v_w.clone(), v_L.clone(), v_C.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_series_rlc_step(mut v_R: Py, mut v_L: Py, mut v_C: Py, mut v_V: Py, mut v_t: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_e: Py = Py::None;
    let mut v_e1: Py = Py::None;
    let mut v_e2: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_kind: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_s1: Py = Py::None;
    let mut v_s2: Py = Py::None;
    let mut v_vc: Py = Py::None;
    let mut v_w0: Py = Py::None;
    let mut v_wd: Py = Py::None;
    let mut v_z: Py = Py::None;
    let _ = f__check_positive(Py::dict(vec![(Py::str("R"), v_R.clone()), (Py::str("L"), v_L.clone()), (Py::str("C"), v_C.clone())]))?;
    if (py::not(&{ let t1518 = py::math1("isfinite", &v_V.clone())?; if !t1518.truthy() { t1518 } else { { let t1517 = py::math1("isfinite", &v_t.clone())?; if !t1517.truthy() { t1517 } else { { let t1515 = v_t.clone(); Py::Bool({ let t1516 = Py::Int(0i64); if py::ge(&t1515, &t1516)? { true } else { false } }) } } } } })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the step voltage must be finite and the time zero or positive"))));
    }
    let t1519 = (Py::tuple(vec![py::truediv(&v_R.clone(), &py::mul(&Py::Int(2i64), &v_L.clone())?)?, py::truediv(&Py::Int(1i64), &py::math1("sqrt", &py::mul(&v_L.clone(), &v_C.clone())?)?)?])).items()?;
    if t1519.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t1519[0].clone();
    v_w0 = t1519[1].clone();
    v_z = py::truediv(&v_a.clone(), &v_w0.clone())?;
    if ({ let t1520 = py::abs(&py::sub(&v_z.clone(), &Py::Int(1i64))?)?; Py::Bool({ let t1521 = Py::Float(1e-09); if py::le(&t1520, &t1521)? { true } else { false } }) }).truthy() {
        v_vc = py::mul(&v_V.clone(), &py::sub(&Py::Int(1i64), &py::mul(&py::add(&Py::Int(1i64), &py::mul(&v_a.clone(), &v_t.clone())?)?, &py::math1("exp", &py::mul(&py::neg(&v_a.clone())?, &v_t.clone())?)?)?)?)?;
        v_i = py::mul(&py::mul(&py::truediv(&v_V.clone(), &v_L.clone())?, &v_t.clone())?, &py::math1("exp", &py::mul(&py::neg(&v_a.clone())?, &v_t.clone())?)?)?;
        v_kind = Py::str("critically damped");
    } else {
        if ({ let t1522 = v_z.clone(); Py::Bool({ let t1523 = Py::Int(1i64); if py::lt(&t1522, &t1523)? { true } else { false } }) }).truthy() {
            v_wd = py::math1("sqrt", &py::sub(&py::mul(&v_w0.clone(), &v_w0.clone())?, &py::mul(&v_a.clone(), &v_a.clone())?)?)?;
            v_e = py::math1("exp", &py::mul(&py::neg(&v_a.clone())?, &v_t.clone())?)?;
            v_vc = py::mul(&v_V.clone(), &py::sub(&Py::Int(1i64), &py::mul(&v_e.clone(), &py::add(&py::math1("cos", &py::mul(&v_wd.clone(), &v_t.clone())?)?, &py::mul(&py::truediv(&v_a.clone(), &v_wd.clone())?, &py::math1("sin", &py::mul(&v_wd.clone(), &v_t.clone())?)?)?)?)?)?)?;
            v_i = py::mul(&py::mul(&py::truediv(&v_V.clone(), &py::mul(&v_wd.clone(), &v_L.clone())?)?, &v_e.clone())?, &py::math1("sin", &py::mul(&v_wd.clone(), &v_t.clone())?)?)?;
            v_kind = Py::str("underdamped");
        } else {
            v_r = py::math1("sqrt", &py::sub(&py::mul(&v_a.clone(), &v_a.clone())?, &py::mul(&v_w0.clone(), &v_w0.clone())?)?)?;
            let t1524 = (Py::tuple(vec![py::add(&py::neg(&v_a.clone())?, &v_r.clone())?, py::sub(&py::neg(&v_a.clone())?, &v_r.clone())?])).items()?;
            if t1524.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_s1 = t1524[0].clone();
            v_s2 = t1524[1].clone();
            let t1525 = (Py::tuple(vec![py::math1("exp", &py::mul(&v_s1.clone(), &v_t.clone())?)?, py::math1("exp", &py::mul(&v_s2.clone(), &v_t.clone())?)?])).items()?;
            if t1525.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_e1 = t1525[0].clone();
            v_e2 = t1525[1].clone();
            v_vc = py::mul(&v_V.clone(), &py::add(&Py::Int(1i64), &py::truediv(&py::sub(&py::mul(&v_s2.clone(), &v_e1.clone())?, &py::mul(&v_s1.clone(), &v_e2.clone())?)?, &py::sub(&v_s1.clone(), &v_s2.clone())?)?)?)?;
            v_i = py::mul(&py::truediv(&v_V.clone(), &py::mul(&v_L.clone(), &py::sub(&v_s1.clone(), &v_s2.clone())?)?)?, &py::sub(&v_e1.clone(), &v_e2.clone())?)?;
            v_kind = Py::str("overdamped");
        }
    }
    return Ok(Py::dict(vec![(Py::str("vC"), v_vc.clone()), (Py::str("i"), v_i.clone()), (Py::str("i_mA"), py::mul(&v_i.clone(), &Py::Float(1000.0))?), (Py::str("alpha"), v_a.clone()), (Py::str("omega0"), v_w0.clone()), (Py::str("zeta"), v_z.clone()), (Py::str("response"), v_kind.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_state_space_to_tf(mut v_A: Py, mut v_B: Py, mut v_C: Py, mut v_D: Py) -> R {
    let mut v_N: Py = Py::None;
    let mut v_Ns: Py = Py::None;
    let mut v_den: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_num: Py = Py::None;
    v_n = py::len(&v_A.clone())?;
    if ({ let t1547 = Py::Bool(({ let mut out = vec![]; for t1539 in py::iter(&(v_A.clone()))? { let mut v_r: Py; 
v_r = t1539;
out.push({ let t1540 = py::len(&v_r.clone())?; Py::Bool({ let t1541 = v_n.clone(); if py::ne(&t1540, &t1541) { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())); if t1547.truthy() { t1547 } else { { let t1546 = { let t1542 = py::len(&v_B.clone())?; Py::Bool({ let t1543 = v_n.clone(); if py::ne(&t1542, &t1543) { true } else { false } }) }; if t1546.truthy() { t1546 } else { { let t1544 = py::len(&v_C.clone())?; Py::Bool({ let t1545 = v_n.clone(); if py::ne(&t1544, &t1545) { true } else { false } }) } } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("A must be n x n and B, C must have n entries"))));
    }
    let t1561 = (f__charpoly_adj(v_A.clone())?).items()?;
    if t1561.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_den = t1561[0].clone();
    v_Ns = t1561[1].clone();
    v_num = { let mut out = vec![]; for t1562 in py::iter(&(v_den.clone()))? { let mut v_d: Py; 
v_d = t1562;
out.push(py::mul(&v_D.clone(), &v_d.clone())?); } Py::list(out) };
    for t1563 in py::iter(&(py::enumerate(&v_Ns.clone(), 0)?))? {
        let t1568 = (t1563).items()?;
        if t1568.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1568[0].clone();
        v_N = t1568[1].clone();
        let t1564 = v_num.clone(); let t1565 = py::add(&v_k.clone(), &Py::Int(1i64))?;
        py::setitem(&t1564, &t1565, py::iadd(&py::getitem(&t1564, &t1565)?, &py::sum(&{ let mut out = vec![]; for t1566 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t1566;
for t1567 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t1567;
out.push(py::mul(&py::mul(&py::getitem(&v_C.clone(), &v_i.clone())?, &py::getitem(&py::getitem(&v_N.clone(), &v_i.clone())?, &v_j.clone())?)?, &py::getitem(&v_B.clone(), &v_j.clone())?)?); }} Py::list(out) }, None)?)?)?;
    }
    return Ok(Py::dict(vec![(Py::str("num"), f__snap(v_num.clone())?), (Py::str("den"), f__snap(v_den.clone())?), (Py::str("order"), py::float(&v_n.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_steady_state_error(mut v_num: Py, mut v_den: Py, mut v_input: Py) -> R {
    let mut v_K: Py = Py::None;
    let mut v_char: Py = Py::None;
    let mut v_dprime: Py = Py::None;
    let mut v_ess: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_nprime: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_zd: Py = Py::None;
    let mut v_zn: Py = Py::None;
    let t1629 = (f__check_tf(v_num.clone(), v_den.clone())?).items()?;
    if t1629.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_num = t1629[0].clone();
    v_den = t1629[1].clone();
    v_char = f__ptrim(f__padd(v_den.clone(), v_num.clone())?)?;
    if ({ let t1633 = py::minmax(&[{ let mut out = vec![]; for t1671 in py::iter(&(f__roots(v_char.clone())?))? { let mut v_r: Py; 
v_r = t1671;
out.push(py::real(&py::complex(&[v_r.clone()])?)?); } Py::list(out) }], true)?; Py::Bool({ let t1634 = py::mul(&py::neg(&Py::Float(1e-09))?, &py::minmax(&[Py::Float(1.0), py::minmax(&[{ let mut out = vec![]; for t1672 in py::iter(&(f__roots(v_char.clone())?))? { let mut v_r: Py; 
v_r = t1672;
out.push(py::abs(&py::complex(&[v_r.clone()])?)?); } Py::list(out) }], true)?], true)?)?; if py::ge(&t1633, &t1634)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the closed loop is not stable, so there is no steady-state error"))));
    }
    v_k = py::getitem(&c__INPUT_ORDER()?, &v_input.clone())?;
    v_zd = Py::Int(0i64);
    while ({ let t1677 = { let t1673 = v_zd.clone(); Py::Bool({ let t1674 = py::len(&v_den.clone())?; if py::lt(&t1673, &t1674)? { true } else { false } }) }; if !t1677.truthy() { t1677 } else { { let t1675 = py::getitem(&v_den.clone(), &py::sub(&py::sub(&py::len(&v_den.clone())?, &Py::Int(1i64))?, &v_zd.clone())?)?; Py::Bool({ let t1676 = Py::Int(0i64); if py::eq(&t1675, &t1676) { true } else { false } }) } } }).truthy() {
        v_zd = py::iadd(&v_zd, &Py::Int(1i64))?;
    }
    v_zn = Py::Int(0i64);
    while ({ let t1682 = { let t1678 = v_zn.clone(); Py::Bool({ let t1679 = py::len(&v_num.clone())?; if py::lt(&t1678, &t1679)? { true } else { false } }) }; if !t1682.truthy() { t1682 } else { { let t1680 = py::getitem(&v_num.clone(), &py::sub(&py::sub(&py::len(&v_num.clone())?, &Py::Int(1i64))?, &v_zn.clone())?)?; Py::Bool({ let t1681 = Py::Int(0i64); if py::eq(&t1680, &t1681) { true } else { false } }) } } }).truthy() {
        v_zn = py::iadd(&v_zn, &Py::Int(1i64))?;
    }
    v_r = py::sub(&v_zd.clone(), &v_zn.clone())?;
    let t1683 = (Py::tuple(vec![py::slice(&v_num.clone(), None, Some((py::sub(&py::len(&v_num.clone())?, &v_zn.clone())?).i()?), None)?, py::slice(&v_den.clone(), None, Some((py::sub(&py::len(&v_den.clone())?, &v_zd.clone())?).i()?), None)?])).items()?;
    if t1683.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_nprime = t1683[0].clone();
    v_dprime = t1683[1].clone();
    if ({ let t1684 = v_k.clone(); Py::Bool({ let t1685 = v_r.clone(); if py::gt(&t1684, &t1685)? { true } else { false } }) }).truthy() {
        v_K = Py::Float(0.0);
    } else {
        if ({ let t1686 = v_k.clone(); Py::Bool({ let t1687 = v_r.clone(); if py::eq(&t1686, &t1687) { true } else { false } }) }).truthy() {
            v_K = py::truediv(&py::getitem(&v_nprime.clone(), &py::neg(&Py::Int(1i64))?)?, &py::getitem(&v_dprime.clone(), &py::neg(&Py::Int(1i64))?)?)?;
        } else {
            v_K = Py::Float(f64::INFINITY);
        }
    }
    if ({ let t1688 = v_k.clone(); Py::Bool({ let t1689 = Py::Int(0i64); if py::eq(&t1688, &t1689) { true } else { false } }) }).truthy() {
        v_ess = (if ({ let t1690 = v_K.clone(); Py::Bool({ let t1691 = Py::Float(f64::INFINITY); if py::eq(&t1690, &t1691) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { py::truediv(&Py::Float(1.0), &py::add(&Py::Float(1.0), &v_K.clone())?)? });
    } else {
        v_ess = (if ({ let t1692 = v_K.clone(); Py::Bool({ let t1693 = Py::Float(f64::INFINITY); if py::eq(&t1692, &t1693) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { (if ({ let t1694 = v_K.clone(); Py::Bool({ let t1695 = Py::Int(0i64); if py::eq(&t1694, &t1695) { true } else { false } }) }).truthy() { Py::str("infinite") } else { py::truediv(&Py::Float(1.0), &v_K.clone())? }) });
    }
    return Ok(Py::dict(vec![(Py::str("steady_state_error"), v_ess.clone()), (Py::str("error_constant"), (if ({ let t1696 = v_K.clone(); Py::Bool({ let t1697 = Py::Float(f64::INFINITY); if py::eq(&t1696, &t1697) { true } else { false } }) }).truthy() { Py::str("infinite") } else { v_K.clone() })), (Py::str("system_type"), py::float(&v_r.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_summing_amplifier(mut v_V: Py, mut v_R: Py, mut v_Rf: Py) -> R {
    let mut v_terms: Py = Py::None;
    if ({ let t1724 = py::len(&v_V.clone())?; Py::Bool({ let t1725 = py::len(&v_R.clone())?; if py::ne(&t1724, &t1725) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("give one input resistor for every input voltage"))));
    }
    if ({ let t1731 = Py::Bool(({ let mut out = vec![]; for t1726 in py::iter(&(v_R.clone()))? { let mut v_r: Py; 
v_r = t1726;
out.push({ let t1727 = v_r.clone(); Py::Bool({ let t1728 = Py::Int(0i64); if py::le(&t1727, &t1728)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())); if t1731.truthy() { t1731 } else { py::not(&{ let t1729 = v_Rf.clone(); Py::Bool({ let t1730 = Py::Int(0i64); if py::gt(&t1729, &t1730)? { true } else { false } }) }) } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("resistances must be positive"))));
    }
    v_terms = { let mut out = vec![]; for t1732 in py::iter(&(py::zip(&[v_V.clone(), v_R.clone()])?))? { let mut v_r: Py; let mut v_v: Py; 
let t1733 = (t1732).items()?;
if t1733.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_v = t1733[0].clone();
v_r = t1733[1].clone();
out.push(py::mul(&py::truediv(&v_Rf.clone(), &v_r.clone())?, &v_v.clone())?); } Py::list(out) };
    return Ok(Py::dict(vec![(Py::str("Vout"), py::neg(&py::fsum(&v_terms.clone())?)?), (Py::str("gains"), { let mut out = vec![]; for t1734 in py::iter(&(v_R.clone()))? { let mut v_r: Py; 
v_r = t1734;
out.push(py::truediv(&py::neg(&v_Rf.clone())?, &v_r.clone())?); } Py::list(out) })]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_thevenin_equivalent(mut v_netlist: Py, mut v_node_a: Py, mut v_node_b: Py) -> R {
    let mut v_rth: Py = Py::None;
    let mut v_vth: Py = Py::None;
    let t1874 = (f__thevenin(v_netlist.clone(), v_node_a.clone(), v_node_b.clone())?).items()?;
    if t1874.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_vth = t1874[0].clone();
    v_rth = t1874[1].clone();
    return Ok(Py::dict(vec![(Py::str("Vth"), v_vth.clone()), (Py::str("Rth"), v_rth.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_three_phase_load(mut v_VL: Py, mut v_R: Py, mut v_X: Py, mut v_connection: Py) -> R {
    let mut v_P: Py = Py::None;
    let mut v_Q: Py = Py::None;
    let mut v_Z: Py = Py::None;
    let mut v_il: Py = Py::None;
    let mut v_iph: Py = Py::None;
    let mut v_vph: Py = Py::None;
    if (py::not(&{ let t1882 = { let t1880 = v_VL.clone(); Py::Bool({ let t1881 = Py::Int(0i64); if py::gt(&t1880, &t1881)? { true } else { false } }) }; if !t1882.truthy() { t1882 } else { py::math1("isfinite", &v_VL.clone())? } })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the line voltage must be positive"))));
    }
    if ({ let t1890 = { let t1883 = v_R.clone(); Py::Bool({ let t1884 = Py::Int(0i64); if py::lt(&t1883, &t1884)? { true } else { false } }) }; if t1890.truthy() { t1890 } else { { let t1889 = { let t1885 = v_R.clone(); Py::Bool({ let t1886 = Py::Int(0i64); if py::eq(&t1885, &t1886) { true } else { false } }) }; if !t1889.truthy() { t1889 } else { { let t1887 = v_X.clone(); Py::Bool({ let t1888 = Py::Int(0i64); if py::eq(&t1887, &t1888) { true } else { false } }) } } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the phase impedance must have R >= 0 and not be zero"))));
    }
    v_Z = py::complex(&[v_R.clone(), v_X.clone()])?;
    if ({ let t1891 = v_connection.clone(); Py::Bool({ let t1892 = Py::str("wye"); if py::eq(&t1891, &t1892) { true } else { false } }) }).truthy() {
        v_vph = py::truediv(&v_VL.clone(), &py::math1("sqrt", &Py::Int(3i64))?)?;
        v_iph = py::truediv(&v_vph.clone(), &py::abs(&v_Z.clone())?)?;
        v_il = v_iph.clone();
    } else {
        v_vph = v_VL.clone();
        v_iph = py::truediv(&v_vph.clone(), &py::abs(&v_Z.clone())?)?;
        v_il = py::mul(&py::math1("sqrt", &Py::Int(3i64))?, &v_iph.clone())?;
    }
    let t1893 = (Py::tuple(vec![py::mul(&py::mul(&py::mul(&Py::Int(3i64), &v_iph.clone())?, &v_iph.clone())?, &v_R.clone())?, py::mul(&py::mul(&py::mul(&Py::Int(3i64), &v_iph.clone())?, &v_iph.clone())?, &v_X.clone())?])).items()?;
    if t1893.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_P = t1893[0].clone();
    v_Q = t1893[1].clone();
    return Ok(Py::dict(vec![(Py::str("phase_voltage"), v_vph.clone()), (Py::str("phase_current"), v_iph.clone()), (Py::str("line_current"), v_il.clone()), (Py::str("P"), v_P.clone()), (Py::str("Q"), v_Q.clone()), (Py::str("S"), py::mul(&py::mul(&Py::Int(3i64), &v_vph.clone())?, &v_iph.clone())?), (Py::str("power_factor"), py::truediv(&v_R.clone(), &py::abs(&v_Z.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_transformer(mut v_Np: Py, mut v_Ns: Py, mut v_Vp: Py, mut v_RL: Py) -> R {
    let mut v_i_s: Py = Py::None;
    let mut v_vs: Py = Py::None;
    let _ = f__check_positive(Py::dict(vec![(Py::str("Np"), v_Np.clone()), (Py::str("Ns"), v_Ns.clone()), (Py::str("RL"), v_RL.clone())]))?;
    v_vs = py::truediv(&py::mul(&v_Vp.clone(), &v_Ns.clone())?, &v_Np.clone())?;
    v_i_s = py::truediv(&v_vs.clone(), &v_RL.clone())?;
    return Ok(Py::dict(vec![(Py::str("Vs"), v_vs.clone()), (Py::str("Is"), v_i_s.clone()), (Py::str("Ip"), py::truediv(&py::mul(&v_i_s.clone(), &v_Ns.clone())?, &v_Np.clone())?), (Py::str("Rin"), py::mul(&v_RL.clone(), &py::pow(&py::truediv(&v_Np.clone(), &v_Ns.clone())?, &Py::Int(2i64))?)?), (Py::str("turns_ratio"), py::truediv(&v_Np.clone(), &v_Ns.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_truth_table_sop(mut v_expression: Py) -> R {
    let mut v_n: Py = Py::None;
    let mut v_names: Py = Py::None;
    let mut v_on: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_tree: Py = Py::None;
    v_tree = f__bool_parse(v_expression.clone())?;
    v_names = py::sorted(&f__bool_vars(v_tree.clone(), Py::None)?, (Py::Bool(false)).truthy())?;
    v_n = py::len(&v_names.clone())?;
    if ({ let t1942 = v_n.clone(); Py::Bool({ let t1943 = Py::Int(0i64); if py::eq(&t1942, &t1943) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the expression has no variables"))));
    }
    if ({ let t1944 = v_n.clone(); Py::Bool({ let t1945 = Py::Int(4i64); if py::gt(&t1944, &t1945)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("minimization is offered for up to 4 variables"))));
    }
    v_on = { let mut out = vec![]; for t1946 in py::range_iter(&[(py::pow(&Py::Int(2i64), &v_n.clone())?).i()?]) { let mut v_m: Py; 
v_m = t1946;
if (f__bool_eval(v_tree.clone(), { let mut out = vec![]; for t1958 in py::iter(&(py::enumerate(&v_names.clone(), 0)?))? { let mut v_i: Py; let mut v_nm: Py; 
let t1959 = (t1958).items()?;
if t1959.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_i = t1959[0].clone();
v_nm = t1959[1].clone();
out.push((v_nm.clone(), py::bitand(&py::rshift(&v_m.clone(), &py::sub(&py::sub(&v_n.clone(), &Py::Int(1i64))?, &v_i.clone())?)?, &Py::Int(1i64))?)); } Py::dict(out) })?).truthy() { out.push(v_m.clone()); }} Py::list(out) };
    v_out = Py::dict(vec![(Py::str("variables"), py::method(&Py::str(", "), "join", &[v_names.clone()])?), (Py::str("minterms"), { let mut out = vec![]; for t1960 in py::iter(&(v_on.clone()))? { let mut v_m: Py; 
v_m = t1960;
out.push(py::float(&v_m.clone())?); } Py::list(out) }), (Py::str("maxterms"), { let mut out = vec![]; for t1961 in py::range_iter(&[(py::pow(&Py::Int(2i64), &v_n.clone())?).i()?]) { let mut v_m: Py; 
v_m = t1961;
if ({ let t1962 = v_m.clone(); Py::Bool({ let t1963 = v_on.clone(); if !py::contains(&t1963, &t1962)? { true } else { false } }) }).truthy() { out.push(py::float(&v_m.clone())?); }} Py::list(out) }), (Py::str("canonical_sop"), f__canonical_sop(v_n.clone(), v_names.clone(), v_on.clone())?)]);
    let _ = py::method(&v_out.clone(), "update", &[f__sop_out(v_n.clone(), v_names.clone(), v_on.clone(), Py::tuple(vec![]))?])?;
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_twos_complement_decode(mut v_pattern: Py) -> R {
    let mut v_n: Py = Py::None;
    let mut v_p: Py = Py::None;
    v_p = py::method(&v_pattern.clone(), "strip", &[])?;
    if ({ let t2048 = py::not(&v_p.clone()); if t2048.truthy() { t2048 } else { { let t2047 = py::sub(&py::set(&v_p.clone())?, &py::set(&Py::list(vec![Py::str("0"), Py::str("1")]))?)?; if t2047.truthy() { t2047 } else { { let t2045 = py::len(&v_p.clone())?; Py::Bool({ let t2046 = Py::Int(64i64); if py::gt(&t2045, &t2046)? { true } else { false } }) } } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the pattern must be 1 to 64 binary digits"))));
    }
    v_n = py::len(&v_p.clone())?;
    return Ok(Py::dict(vec![(Py::str("value"), py::float(&py::sub(&py::int_base(&v_p.clone(), &Py::Int(2i64))?, &(if ({ let t2049 = py::getitem(&v_p.clone(), &Py::Int(0i64))?; Py::Bool({ let t2050 = Py::str("1"); if py::eq(&t2049, &t2050) { true } else { false } }) }).truthy() { py::pow(&Py::Int(2i64), &v_n.clone())? } else { Py::Int(0i64) }))?)?), (Py::str("bits"), py::float(&v_n.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_twos_complement_encode(mut v_value: Py, mut v_bits: Py) -> R {
    let mut v_n: Py = Py::None;
    let mut v_u: Py = Py::None;
    let mut v_v: Py = Py::None;
    let t2052 = (Py::tuple(vec![py::int(&v_value.clone())?, py::int(&v_bits.clone())?])).items()?;
    if t2052.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_v = t2052[0].clone();
    v_n = t2052[1].clone();
    if (py::not(&{ let t2053 = Py::Int(2i64); Py::Bool({ let t2054 = v_n.clone(); if py::le(&t2053, &t2054)? { { let t2055 = Py::Int(64i64); if py::le(&t2054, &t2055)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the width must be 2 to 64 bits"))));
    }
    if (py::not(&{ let t2056 = py::neg(&py::pow(&Py::Int(2i64), &py::sub(&v_n.clone(), &Py::Int(1i64))?)?)?; Py::Bool({ let t2057 = v_v.clone(); if py::le(&t2056, &t2057)? { { let t2058 = py::pow(&Py::Int(2i64), &py::sub(&v_n.clone(), &Py::Int(1i64))?)?; if py::lt(&t2057, &t2058)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_v.clone(), "")?, " does not fit in ".to_string(), py::format(&v_n.clone(), "")?, "-bit two's complement (range ".to_string(), py::format(&py::neg(&py::pow(&Py::Int(2i64), &py::sub(&v_n.clone(), &Py::Int(1i64))?)?)?, "")?, " to ".to_string(), py::format(&py::sub(&py::pow(&Py::Int(2i64), &py::sub(&v_n.clone(), &Py::Int(1i64))?)?, &Py::Int(1i64))?, "")?, ")".to_string()].concat()))));
    }
    v_u = py::modulo(&v_v.clone(), &py::pow(&Py::Int(2i64), &v_n.clone())?)?;
    return Ok(Py::dict(vec![(Py::str("pattern"), Py::str(&py::format(&v_u.clone(), &(Py::str(&["0".to_string(), py::format(&v_n.clone(), "")?, "b".to_string()].concat())).as_str()?)?)), (Py::str("hexadecimal"), Py::str(&py::format(&v_u.clone(), &(Py::str(&["0".to_string(), py::format(&py::floordiv(&py::add(&v_n.clone(), &Py::Int(3i64))?, &Py::Int(4i64))?, "")?, "X".to_string()].concat())).as_str()?)?)), (Py::str("unsigned"), py::float(&v_u.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_voltage_divider(mut v_Vin: Py, mut v_R1: Py, mut v_R2: Py) -> R {
    let _ = f__check_positive(Py::dict(vec![(Py::str("R1"), v_R1.clone()), (Py::str("R2"), v_R2.clone())]))?;
    return Ok(Py::dict(vec![(Py::str("Vout"), py::truediv(&py::mul(&v_Vin.clone(), &v_R2.clone())?, &py::add(&v_R1.clone(), &v_R2.clone())?)?), (Py::str("I"), py::truediv(&v_Vin.clone(), &py::add(&v_R1.clone(), &v_R2.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_wheatstone_bridge(mut v_Vs: Py, mut v_R1: Py, mut v_R2: Py, mut v_R3: Py, mut v_R4: Py) -> R {
    let mut v_vout: Py = Py::None;
    let _ = f__check_positive(Py::dict(vec![(Py::str("R1"), v_R1.clone()), (Py::str("R2"), v_R2.clone()), (Py::str("R3"), v_R3.clone()), (Py::str("R4"), v_R4.clone())]))?;
    v_vout = py::mul(&v_Vs.clone(), &py::sub(&py::truediv(&v_R2.clone(), &py::add(&v_R1.clone(), &v_R2.clone())?)?, &py::truediv(&v_R4.clone(), &py::add(&v_R3.clone(), &v_R4.clone())?)?)?)?;
    return Ok(Py::dict(vec![(Py::str("Vout"), v_vout.clone()), (Py::str("balanced"), { let t2080 = py::abs(&py::sub(&py::mul(&v_R1.clone(), &v_R4.clone())?, &py::mul(&v_R2.clone(), &v_R3.clone())?)?)?; Py::Bool({ let t2081 = py::mul(&py::mul(&Py::Float(1e-12), &v_R1.clone())?, &v_R4.clone())?; if py::le(&t2080, &t2081)? { true } else { false } }) }), (Py::str("R4_balance"), py::truediv(&py::mul(&v_R2.clone(), &v_R3.clone())?, &v_R1.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_wye_to_delta(mut v_Ra: Py, mut v_Rb: Py, mut v_Rc: Py) -> R {
    let mut v_n: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_v: Py = Py::None;
    for t2083 in py::iter(&(Py::tuple(vec![Py::tuple(vec![Py::str("Ra"), v_Ra.clone()]), Py::tuple(vec![Py::str("Rb"), v_Rb.clone()]), Py::tuple(vec![Py::str("Rc"), v_Rc.clone()])])))? {
        let t2087 = (t2083).items()?;
        if t2087.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_n = t2087[0].clone();
        v_v = t2087[1].clone();
        if (py::not(&{ let t2086 = { let t2084 = v_v.clone(); Py::Bool({ let t2085 = Py::Int(0i64); if py::gt(&t2084, &t2085)? { true } else { false } }) }; if !t2086.truthy() { t2086 } else { py::math1("isfinite", &v_v.clone())? } })).truthy() {
            return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_n.clone(), "")?, " must be a positive resistance".to_string()].concat()))));
        }
    }
    v_p = py::add(&py::add(&py::mul(&v_Ra.clone(), &v_Rb.clone())?, &py::mul(&v_Rb.clone(), &v_Rc.clone())?)?, &py::mul(&v_Rc.clone(), &v_Ra.clone())?)?;
    return Ok(Py::dict(vec![(Py::str("Rab"), py::truediv(&v_p.clone(), &v_Rc.clone())?), (Py::str("Rbc"), py::truediv(&v_p.clone(), &v_Ra.clone())?), (Py::str("Rca"), py::truediv(&v_p.clone(), &v_Rb.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_zener_regulator(mut v_Vin: Py, mut v_R: Py, mut v_Vz: Py, mut v_RL: Py) -> R {
    let mut v_il: Py = Py::None;
    let mut v_ir: Py = Py::None;
    let mut v_iz: Py = Py::None;
    let _ = f__check_positive(Py::dict(vec![(Py::str("Vin"), v_Vin.clone()), (Py::str("R"), v_R.clone()), (Py::str("Vz"), v_Vz.clone()), (Py::str("RL"), v_RL.clone())]))?;
    if ({ let t2097 = py::truediv(&py::mul(&v_Vin.clone(), &v_RL.clone())?, &py::add(&v_R.clone(), &v_RL.clone())?)?; Py::Bool({ let t2098 = v_Vz.clone(); if py::le(&t2097, &t2098)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("without the Zener the load voltage would not reach Vz: the diode is not in breakdown (no regulation)"))));
    }
    let t2099 = (Py::tuple(vec![py::truediv(&py::sub(&v_Vin.clone(), &v_Vz.clone())?, &v_R.clone())?, py::truediv(&v_Vz.clone(), &v_RL.clone())?])).items()?;
    if t2099.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_ir = t2099[0].clone();
    v_il = t2099[1].clone();
    v_iz = py::sub(&v_ir.clone(), &v_il.clone())?;
    return Ok(Py::dict(vec![(Py::str("VL"), v_Vz.clone()), (Py::str("IR"), v_ir.clone()), (Py::str("IL"), v_il.clone()), (Py::str("IZ"), v_iz.clone()), (Py::str("IZ_mA"), py::mul(&v_iz.clone(), &Py::Float(1000.0))?), (Py::str("PZ"), py::mul(&v_Vz.clone(), &v_iz.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1083(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::add(&py::truediv(&py::abs(&py::sub(&py::sub(&py::getitem(&v_o.clone(), &Py::str("Vout"))?, &py::getitem(&v_i.clone(), &Py::str("V0"))?)?, &py::mul(&py::getitem(&v_o.clone(), &Py::str("slope"))?, &py::getitem(&v_i.clone(), &Py::str("t"))?)?)?)?, &py::minmax(&[py::abs(&py::getitem(&v_i.clone(), &Py::str("V0"))?)?, py::abs(&py::getitem(&v_o.clone(), &Py::str("Vout"))?)?, Py::Float(1e-300)], true)?)?, &f__rel(py::mul(&py::mul(&py::getitem(&v_o.clone(), &Py::str("slope"))?, &py::getitem(&v_i.clone(), &Py::str("R"))?)?, &py::getitem(&v_i.clone(), &Py::str("C"))?)?, py::neg(&py::getitem(&v_i.clone(), &Py::str("Vin"))?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1084(mut v_kw: Py) -> R {
    let mut v_out: Py = Py::None;
    let mut v_total: Py = Py::None;
    let mut v_vals: Py = Py::None;
    v_vals = py::getitem(&v_kw.clone(), &Py::str("C"))?;
    let _ = f__positive_values(v_vals.clone(), Py::str("capacitors"))?;
    v_total = (if (Py::Bool(true)).truthy() { py::sum(&v_vals.clone(), None)? } else { py::truediv(&Py::Float(1.0), &py::sum(&{ let mut out = vec![]; for t1091 in py::iter(&(v_vals.clone()))? { let mut v_v: Py; 
v_v = t1091;
out.push(py::truediv(&Py::Float(1.0), &v_v.clone())?); } Py::list(out) }, None)?)? });
    v_out = Py::dict(vec![(Py::str("equivalent"), v_total.clone()), (Py::str("count"), py::float(&py::len(&v_vals.clone())?)?)]);
    if ({ let t1092 = Py::str("capacitance"); Py::Bool({ let t1093 = Py::str("resistance"); if py::ne(&t1092, &t1093) { true } else { false } }) }).truthy() {
        py::setitem(&v_out.clone(), &Py::str("unit"), { let t1094 = py::method(&v_kw.clone(), "get", &[Py::str("unit")])?; if t1094.truthy() { t1094 } else { Py::str("the unit of the inputs") } })?;
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1095(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_eq: Py = Py::None;
    let mut v_vals: Py = Py::None;
    let t1096 = (Py::tuple(vec![py::getitem(&v_inputs.clone(), &Py::str("C"))?, py::getitem(&v_outputs.clone(), &Py::str("equivalent"))?])).items()?;
    if t1096.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_vals = t1096[0].clone();
    v_eq = t1096[1].clone();
    if (Py::Bool(true)).truthy() {
        return Ok(py::add(&py::truediv(&py::abs(&py::sub(&v_eq.clone(), &py::fsum(&v_vals.clone())?)?)?, &py::fsum(&v_vals.clone())?)?, &py::abs(&py::sub(&py::method(&v_outputs.clone(), "get", &[Py::str("count"), py::len(&v_vals.clone())?])?, &py::len(&v_vals.clone())?)?)?)?);
    }
    return Ok(py::add(&py::abs(&py::sub(&py::fsum(&{ let mut out = vec![]; for t1097 in py::iter(&(v_vals.clone()))? { let mut v_v: Py; 
v_v = t1097;
out.push(py::truediv(&v_eq.clone(), &v_v.clone())?); } Py::list(out) })?, &Py::Float(1.0))?)?, &py::abs(&py::sub(&py::method(&v_outputs.clone(), "get", &[Py::str("count"), py::len(&v_vals.clone())?])?, &py::len(&v_vals.clone())?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1098(mut v_kw: Py) -> R {
    let mut v_out: Py = Py::None;
    let mut v_total: Py = Py::None;
    let mut v_vals: Py = Py::None;
    v_vals = py::getitem(&v_kw.clone(), &Py::str("L"))?;
    let _ = f__positive_values(v_vals.clone(), Py::str("inductors"))?;
    v_total = (if (Py::Bool(false)).truthy() { py::sum(&v_vals.clone(), None)? } else { py::truediv(&Py::Float(1.0), &py::sum(&{ let mut out = vec![]; for t1105 in py::iter(&(v_vals.clone()))? { let mut v_v: Py; 
v_v = t1105;
out.push(py::truediv(&Py::Float(1.0), &v_v.clone())?); } Py::list(out) }, None)?)? });
    v_out = Py::dict(vec![(Py::str("equivalent"), v_total.clone()), (Py::str("count"), py::float(&py::len(&v_vals.clone())?)?)]);
    if ({ let t1106 = Py::str("inductance"); Py::Bool({ let t1107 = Py::str("resistance"); if py::ne(&t1106, &t1107) { true } else { false } }) }).truthy() {
        py::setitem(&v_out.clone(), &Py::str("unit"), { let t1108 = py::method(&v_kw.clone(), "get", &[Py::str("unit")])?; if t1108.truthy() { t1108 } else { Py::str("the unit of the inputs") } })?;
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1109(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_eq: Py = Py::None;
    let mut v_vals: Py = Py::None;
    let t1110 = (Py::tuple(vec![py::getitem(&v_inputs.clone(), &Py::str("L"))?, py::getitem(&v_outputs.clone(), &Py::str("equivalent"))?])).items()?;
    if t1110.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_vals = t1110[0].clone();
    v_eq = t1110[1].clone();
    if (Py::Bool(false)).truthy() {
        return Ok(py::add(&py::truediv(&py::abs(&py::sub(&v_eq.clone(), &py::fsum(&v_vals.clone())?)?)?, &py::fsum(&v_vals.clone())?)?, &py::abs(&py::sub(&py::method(&v_outputs.clone(), "get", &[Py::str("count"), py::len(&v_vals.clone())?])?, &py::len(&v_vals.clone())?)?)?)?);
    }
    return Ok(py::add(&py::abs(&py::sub(&py::fsum(&{ let mut out = vec![]; for t1111 in py::iter(&(v_vals.clone()))? { let mut v_v: Py; 
v_v = t1111;
out.push(py::truediv(&v_eq.clone(), &v_v.clone())?); } Py::list(out) })?, &Py::Float(1.0))?)?, &py::abs(&py::sub(&py::method(&v_outputs.clone(), "get", &[Py::str("count"), py::len(&v_vals.clone())?])?, &py::len(&v_vals.clone())?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1112(mut v_kw: Py) -> R {
    let mut v_out: Py = Py::None;
    let mut v_total: Py = Py::None;
    let mut v_vals: Py = Py::None;
    v_vals = py::getitem(&v_kw.clone(), &Py::str("R"))?;
    let _ = f__positive_values(v_vals.clone(), Py::str("resistors"))?;
    v_total = (if (Py::Bool(false)).truthy() { py::sum(&v_vals.clone(), None)? } else { py::truediv(&Py::Float(1.0), &py::sum(&{ let mut out = vec![]; for t1119 in py::iter(&(v_vals.clone()))? { let mut v_v: Py; 
v_v = t1119;
out.push(py::truediv(&Py::Float(1.0), &v_v.clone())?); } Py::list(out) }, None)?)? });
    v_out = Py::dict(vec![(Py::str("equivalent"), v_total.clone()), (Py::str("count"), py::float(&py::len(&v_vals.clone())?)?)]);
    if ({ let t1120 = Py::str("resistance"); Py::Bool({ let t1121 = Py::str("resistance"); if py::ne(&t1120, &t1121) { true } else { false } }) }).truthy() {
        py::setitem(&v_out.clone(), &Py::str("unit"), { let t1122 = py::method(&v_kw.clone(), "get", &[Py::str("unit")])?; if t1122.truthy() { t1122 } else { Py::str("the unit of the inputs") } })?;
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1123(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_eq: Py = Py::None;
    let mut v_vals: Py = Py::None;
    let t1124 = (Py::tuple(vec![py::getitem(&v_inputs.clone(), &Py::str("R"))?, py::getitem(&v_outputs.clone(), &Py::str("equivalent"))?])).items()?;
    if t1124.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_vals = t1124[0].clone();
    v_eq = t1124[1].clone();
    if (Py::Bool(false)).truthy() {
        return Ok(py::add(&py::truediv(&py::abs(&py::sub(&v_eq.clone(), &py::fsum(&v_vals.clone())?)?)?, &py::fsum(&v_vals.clone())?)?, &py::abs(&py::sub(&py::method(&v_outputs.clone(), "get", &[Py::str("count"), py::len(&v_vals.clone())?])?, &py::len(&v_vals.clone())?)?)?)?);
    }
    return Ok(py::add(&py::abs(&py::sub(&py::fsum(&{ let mut out = vec![]; for t1125 in py::iter(&(v_vals.clone()))? { let mut v_v: Py; 
v_v = t1125;
out.push(py::truediv(&v_eq.clone(), &v_v.clone())?); } Py::list(out) })?, &Py::Float(1.0))?)?, &py::abs(&py::sub(&py::method(&v_outputs.clone(), "get", &[Py::str("count"), py::len(&v_vals.clone())?])?, &py::len(&v_vals.clone())?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1133(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_C: Py = Py::None;
    let mut v_L: Py = Py::None;
    let mut v_R: Py = Py::None;
    let mut v_Z: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_res: Py = Py::None;
    let mut v_w: Py = Py::None;
    let t1134 = (Py::tuple(vec![py::getitem(&v_inputs.clone(), &Py::str("R"))?, py::getitem(&v_inputs.clone(), &Py::str("L"))?, py::getitem(&v_inputs.clone(), &Py::str("C"))?, py::getitem(&v_inputs.clone(), &Py::str("f"))?])).items()?;
    if t1134.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_R = t1134[0].clone();
    v_L = t1134[1].clone();
    v_C = t1134[2].clone();
    v_f = t1134[3].clone();
    v_w = py::mul(&py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?, &v_f.clone())?;
    v_Z = py::complex(&[py::getitem(&v_outputs.clone(), &Py::str("real"))?, py::getitem(&v_outputs.clone(), &Py::str("imag"))?])?;
    v_res = (if (Py::Bool(false)).truthy() { py::truediv(&py::abs(&py::sub(&v_Z.clone(), &py::add(&py::add(&v_R.clone(), &py::mul(&py::mul(&Py::Complex(0.0, 1.0), &v_w.clone())?, &v_L.clone())?)?, &py::truediv(&Py::Int(1i64), &py::mul(&py::mul(&Py::Complex(0.0, 1.0), &v_w.clone())?, &v_C.clone())?)?)?)?)?, &py::abs(&v_Z.clone())?)? } else { py::abs(&py::sub(&py::mul(&v_Z.clone(), &py::add(&py::add(&py::truediv(&Py::Int(1i64), &v_R.clone())?, &py::truediv(&Py::Int(1i64), &py::mul(&py::mul(&Py::Complex(0.0, 1.0), &v_w.clone())?, &v_L.clone())?)?)?, &py::mul(&py::mul(&Py::Complex(0.0, 1.0), &v_w.clone())?, &v_C.clone())?)?)?, &Py::Int(1i64))?)? });
    return Ok(py::add(&py::add(&py::add(&py::add(&v_res.clone(), &f__rel(py::getitem(&v_outputs.clone(), &Py::str("impedance"))?, py::abs(&v_Z.clone())?)?)?, &py::truediv(&py::abs(&py::sub(&py::getitem(&v_outputs.clone(), &Py::str("phase_deg"))?, &py::math1("degrees", &py::cmath("phase", &[v_Z.clone()])?)?)?)?, &Py::Int(180i64))?)?, &f__rel(py::getitem(&v_outputs.clone(), &Py::str("XL"))?, py::mul(&v_w.clone(), &v_L.clone())?)?)?, &f__rel(py::getitem(&v_outputs.clone(), &Py::str("XC"))?, py::truediv(&Py::Int(1i64), &py::mul(&v_w.clone(), &v_C.clone())?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1135(mut v_R: Py, mut v_L: Py, mut v_C: Py, mut v_s: Py) -> R {
    return Ok(f__resonance(v_R.clone(), v_L.clone(), v_C.clone(), v_s.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1142(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_C: Py = Py::None;
    let mut v_L: Py = Py::None;
    let mut v_R: Py = Py::None;
    let mut v_Y: Py = Py::None;
    let mut v_peak: Py = Py::None;
    let t1143 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("R"))?, py::getitem(&v_i.clone(), &Py::str("L"))?, py::getitem(&v_i.clone(), &Py::str("C"))?])).items()?;
    if t1143.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_R = t1143[0].clone();
    v_L = t1143[1].clone();
    v_C = t1143[2].clone();
    v_Y = { let v_C = v_C.clone(); let v_L = v_L.clone(); let v_R = v_R.clone(); let v_Y = v_Y.clone(); let v_i = v_i.clone(); let v_o = v_o.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_f: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_w: Py = Py::None;
            v_w = py::mul(&py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?, &v_f.clone())?;
            return Ok((if (Py::Bool(false)).truthy() { py::truediv(&Py::Int(1i64), &py::complex(&[v_R.clone(), py::sub(&py::mul(&v_w.clone(), &v_L.clone())?, &py::truediv(&Py::Int(1i64), &py::mul(&v_w.clone(), &v_C.clone())?)?)?])?)? } else { py::truediv(&Py::Int(1i64), &py::complex(&[py::truediv(&Py::Int(1i64), &v_R.clone())?, py::sub(&py::mul(&v_w.clone(), &v_C.clone())?, &py::truediv(&Py::Int(1i64), &py::mul(&v_w.clone(), &v_L.clone())?)?)?])?)? }));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_peak = py::abs(&py::call(&v_Y.clone(), &[py::getitem(&v_o.clone(), &Py::str("f0"))?])?)?;
    return Ok(py::add(&py::add(&py::add(&py::add(&py::truediv(&py::abs(&py::imag(&py::call(&v_Y.clone(), &[py::getitem(&v_o.clone(), &Py::str("f0"))?])?)?)?, &v_peak.clone())?, &py::abs(&py::sub(&py::truediv(&py::abs(&py::call(&v_Y.clone(), &[py::getitem(&v_o.clone(), &Py::str("f_low"))?])?)?, &v_peak.clone())?, &py::pow(&Py::Int(2i64), &py::neg(&Py::Float(0.5))?)?)?)?)?, &py::abs(&py::sub(&py::truediv(&py::abs(&py::call(&v_Y.clone(), &[py::getitem(&v_o.clone(), &Py::str("f_high"))?])?)?, &v_peak.clone())?, &py::pow(&Py::Int(2i64), &py::neg(&Py::Float(0.5))?)?)?)?)?, &f__rel(py::getitem(&v_o.clone(), &Py::str("bandwidth"))?, py::sub(&py::getitem(&v_o.clone(), &Py::str("f_high"))?, &py::getitem(&v_o.clone(), &Py::str("f_low"))?)?)?)?, &f__rel(py::getitem(&v_o.clone(), &Py::str("Q"))?, py::truediv(&py::getitem(&v_o.clone(), &Py::str("f0"))?, &py::getitem(&v_o.clone(), &Py::str("bandwidth"))?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_12(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::add(&py::add(&py::add(&py::float(&py::not(&{ let t20 = { let t13 = py::mul(&py::getitem(&v_o.clone(), &Py::str("code"))?, &py::getitem(&v_o.clone(), &Py::str("lsb"))?)?; Py::Bool({ let t14 = py::mul(&py::getitem(&v_i.clone(), &Py::str("Vin"))?, &py::add(&Py::Int(1i64), &Py::Float(1e-12))?)?; if py::le(&t13, &t14)? { true } else { false } }) }; if !t20.truthy() { t20 } else { { let t19 = { let t15 = py::getitem(&v_i.clone(), &Py::str("Vin"))?; Py::Bool({ let t16 = py::mul(&py::add(&py::getitem(&v_o.clone(), &Py::str("code"))?, &Py::Int(1i64))?, &py::getitem(&v_o.clone(), &Py::str("lsb"))?)?; if py::lt(&t15, &t16)? { true } else { false } }) }; if t19.truthy() { t19 } else { { let t17 = py::getitem(&v_o.clone(), &Py::str("code"))?; Py::Bool({ let t18 = py::sub(&py::pow(&Py::Int(2i64), &py::int(&py::getitem(&v_i.clone(), &Py::str("bits"))?)?)?, &Py::Int(1i64))?; if py::eq(&t17, &t18) { true } else { false } }) } } } } }))?, &f__rel(py::mul(&py::getitem(&v_o.clone(), &Py::str("lsb"))?, &py::pow(&Py::Int(2i64), &py::int(&py::getitem(&v_i.clone(), &Py::str("bits"))?)?)?)?, py::getitem(&v_i.clone(), &Py::str("Vref"))?)?)?, &py::float(&{ let t21 = py::int_base(&py::getitem(&v_o.clone(), &Py::str("binary"))?, &Py::Int(2i64))?; Py::Bool({ let t22 = py::getitem(&v_o.clone(), &Py::str("code"))?; if py::ne(&t21, &t22) { true } else { false } }) })?)?, &f__rel(py::getitem(&v_o.clone(), &Py::str("lsb_mV"))?, py::mul(&py::getitem(&v_o.clone(), &Py::str("lsb"))?, &Py::Float(1000.0))?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1283(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::call(&{ let v_i = v_i.clone(); let v_o = v_o.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_w: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_rc: Py = a.get(1).cloned().unwrap_or(Py::None);
            return Ok(py::call(&{ let v_i = v_i.clone(); let v_o = v_o.clone(); let v_rc = v_rc.clone(); let v_w = v_w.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_h: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::add(&py::add(&py::add(&f__rel(py::getitem(&v_o.clone(), &Py::str("gain"))?, py::abs(&v_h.clone())?)?, &py::truediv(&py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("phase_deg"))?, &py::math1("degrees", &py::cmath("phase", &[v_h.clone()])?)?)?)?, &Py::Int(180i64))?)?, &f__rel(py::mul(&py::mul(&py::mul(&py::getitem(&v_o.clone(), &Py::str("fc"))?, &Py::Int(2i64))?, &Py::Float(std::f64::consts::PI))?, &v_rc.clone())?, Py::Float(1.0))?)?, &py::truediv(&py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("gain_db"))?, &py::mul(&Py::Int(20i64), &py::math1("log10", &py::abs(&v_h.clone())?)?)?)?)?, &py::minmax(&[Py::Float(1.0), py::abs(&py::getitem(&v_o.clone(), &Py::str("gain_db"))?)?], true)?)?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, &[(if ({ let t1284 = py::getitem(&v_i.clone(), &Py::str("filter"))?; Py::Bool({ let t1285 = Py::str("lowpass"); if py::eq(&t1284, &t1285) { true } else { false } }) }).truthy() { py::truediv(&Py::Int(1i64), &py::add(&Py::Int(1i64), &py::mul(&py::mul(&Py::Complex(0.0, 1.0), &v_w.clone())?, &v_rc.clone())?)?)? } else { py::truediv(&py::mul(&py::mul(&Py::Complex(0.0, 1.0), &v_w.clone())?, &v_rc.clone())?, &py::add(&Py::Int(1i64), &py::mul(&py::mul(&Py::Complex(0.0, 1.0), &v_w.clone())?, &v_rc.clone())?)?)? })])?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, &[py::mul(&py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?, &py::getitem(&v_i.clone(), &Py::str("f"))?)?, py::mul(&py::getitem(&v_i.clone(), &Py::str("R"))?, &py::getitem(&v_i.clone(), &Py::str("C"))?)?])?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1336(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::call(&{ let v_i = v_i.clone(); let v_o = v_o.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_off: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::add(&py::add(&f__rel(py::pow(&py::getitem(&v_o.clone(), &Py::str("rms"))?, &Py::Int(2i64))?, py::add(&py::pow(&v_off.clone(), &Py::Int(2i64))?, &py::mul(&py::getitem(&c__SHAPES()?, &py::getitem(&v_i.clone(), &Py::str("shape"))?)?, &py::pow(&py::getitem(&v_i.clone(), &Py::str("amplitude"))?, &Py::Int(2i64))?)?)?)?, &py::truediv(&py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("average"))?, &v_off.clone())?)?, &py::minmax(&[py::getitem(&v_o.clone(), &Py::str("rms"))?, Py::Float(1e-300)], true)?)?)?, &f__rel(py::add(&py::pow(&py::getitem(&v_o.clone(), &Py::str("ac_rms"))?, &Py::Int(2i64))?, &py::pow(&v_off.clone(), &Py::Int(2i64))?)?, py::pow(&py::getitem(&v_o.clone(), &Py::str("rms"))?, &Py::Int(2i64))?)?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, &[py::method(&v_i.clone(), "get", &[Py::str("offset"), Py::Float(0.0)])?])?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1447(mut v_kw: Py) -> R {
    let mut v_out: Py = Py::None;
    let mut v_total: Py = Py::None;
    let mut v_vals: Py = Py::None;
    v_vals = py::getitem(&v_kw.clone(), &Py::str("C"))?;
    let _ = f__positive_values(v_vals.clone(), Py::str("capacitors"))?;
    v_total = (if (Py::Bool(false)).truthy() { py::sum(&v_vals.clone(), None)? } else { py::truediv(&Py::Float(1.0), &py::sum(&{ let mut out = vec![]; for t1454 in py::iter(&(v_vals.clone()))? { let mut v_v: Py; 
v_v = t1454;
out.push(py::truediv(&Py::Float(1.0), &v_v.clone())?); } Py::list(out) }, None)?)? });
    v_out = Py::dict(vec![(Py::str("equivalent"), v_total.clone()), (Py::str("count"), py::float(&py::len(&v_vals.clone())?)?)]);
    if ({ let t1455 = Py::str("capacitance"); Py::Bool({ let t1456 = Py::str("resistance"); if py::ne(&t1455, &t1456) { true } else { false } }) }).truthy() {
        py::setitem(&v_out.clone(), &Py::str("unit"), { let t1457 = py::method(&v_kw.clone(), "get", &[Py::str("unit")])?; if t1457.truthy() { t1457 } else { Py::str("the unit of the inputs") } })?;
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1458(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_eq: Py = Py::None;
    let mut v_vals: Py = Py::None;
    let t1459 = (Py::tuple(vec![py::getitem(&v_inputs.clone(), &Py::str("C"))?, py::getitem(&v_outputs.clone(), &Py::str("equivalent"))?])).items()?;
    if t1459.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_vals = t1459[0].clone();
    v_eq = t1459[1].clone();
    if (Py::Bool(false)).truthy() {
        return Ok(py::add(&py::truediv(&py::abs(&py::sub(&v_eq.clone(), &py::fsum(&v_vals.clone())?)?)?, &py::fsum(&v_vals.clone())?)?, &py::abs(&py::sub(&py::method(&v_outputs.clone(), "get", &[Py::str("count"), py::len(&v_vals.clone())?])?, &py::len(&v_vals.clone())?)?)?)?);
    }
    return Ok(py::add(&py::abs(&py::sub(&py::fsum(&{ let mut out = vec![]; for t1460 in py::iter(&(v_vals.clone()))? { let mut v_v: Py; 
v_v = t1460;
out.push(py::truediv(&v_eq.clone(), &v_v.clone())?); } Py::list(out) })?, &Py::Float(1.0))?)?, &py::abs(&py::sub(&py::method(&v_outputs.clone(), "get", &[Py::str("count"), py::len(&v_vals.clone())?])?, &py::len(&v_vals.clone())?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1461(mut v_kw: Py) -> R {
    let mut v_out: Py = Py::None;
    let mut v_total: Py = Py::None;
    let mut v_vals: Py = Py::None;
    v_vals = py::getitem(&v_kw.clone(), &Py::str("L"))?;
    let _ = f__positive_values(v_vals.clone(), Py::str("inductors"))?;
    v_total = (if (Py::Bool(true)).truthy() { py::sum(&v_vals.clone(), None)? } else { py::truediv(&Py::Float(1.0), &py::sum(&{ let mut out = vec![]; for t1468 in py::iter(&(v_vals.clone()))? { let mut v_v: Py; 
v_v = t1468;
out.push(py::truediv(&Py::Float(1.0), &v_v.clone())?); } Py::list(out) }, None)?)? });
    v_out = Py::dict(vec![(Py::str("equivalent"), v_total.clone()), (Py::str("count"), py::float(&py::len(&v_vals.clone())?)?)]);
    if ({ let t1469 = Py::str("inductance"); Py::Bool({ let t1470 = Py::str("resistance"); if py::ne(&t1469, &t1470) { true } else { false } }) }).truthy() {
        py::setitem(&v_out.clone(), &Py::str("unit"), { let t1471 = py::method(&v_kw.clone(), "get", &[Py::str("unit")])?; if t1471.truthy() { t1471 } else { Py::str("the unit of the inputs") } })?;
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1472(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_eq: Py = Py::None;
    let mut v_vals: Py = Py::None;
    let t1473 = (Py::tuple(vec![py::getitem(&v_inputs.clone(), &Py::str("L"))?, py::getitem(&v_outputs.clone(), &Py::str("equivalent"))?])).items()?;
    if t1473.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_vals = t1473[0].clone();
    v_eq = t1473[1].clone();
    if (Py::Bool(true)).truthy() {
        return Ok(py::add(&py::truediv(&py::abs(&py::sub(&v_eq.clone(), &py::fsum(&v_vals.clone())?)?)?, &py::fsum(&v_vals.clone())?)?, &py::abs(&py::sub(&py::method(&v_outputs.clone(), "get", &[Py::str("count"), py::len(&v_vals.clone())?])?, &py::len(&v_vals.clone())?)?)?)?);
    }
    return Ok(py::add(&py::abs(&py::sub(&py::fsum(&{ let mut out = vec![]; for t1474 in py::iter(&(v_vals.clone()))? { let mut v_v: Py; 
v_v = t1474;
out.push(py::truediv(&v_eq.clone(), &v_v.clone())?); } Py::list(out) })?, &Py::Float(1.0))?)?, &py::abs(&py::sub(&py::method(&v_outputs.clone(), "get", &[Py::str("count"), py::len(&v_vals.clone())?])?, &py::len(&v_vals.clone())?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1475(mut v_kw: Py) -> R {
    let mut v_out: Py = Py::None;
    let mut v_total: Py = Py::None;
    let mut v_vals: Py = Py::None;
    v_vals = py::getitem(&v_kw.clone(), &Py::str("R"))?;
    let _ = f__positive_values(v_vals.clone(), Py::str("resistors"))?;
    v_total = (if (Py::Bool(true)).truthy() { py::sum(&v_vals.clone(), None)? } else { py::truediv(&Py::Float(1.0), &py::sum(&{ let mut out = vec![]; for t1482 in py::iter(&(v_vals.clone()))? { let mut v_v: Py; 
v_v = t1482;
out.push(py::truediv(&Py::Float(1.0), &v_v.clone())?); } Py::list(out) }, None)?)? });
    v_out = Py::dict(vec![(Py::str("equivalent"), v_total.clone()), (Py::str("count"), py::float(&py::len(&v_vals.clone())?)?)]);
    if ({ let t1483 = Py::str("resistance"); Py::Bool({ let t1484 = Py::str("resistance"); if py::ne(&t1483, &t1484) { true } else { false } }) }).truthy() {
        py::setitem(&v_out.clone(), &Py::str("unit"), { let t1485 = py::method(&v_kw.clone(), "get", &[Py::str("unit")])?; if t1485.truthy() { t1485 } else { Py::str("the unit of the inputs") } })?;
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1486(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_eq: Py = Py::None;
    let mut v_vals: Py = Py::None;
    let t1487 = (Py::tuple(vec![py::getitem(&v_inputs.clone(), &Py::str("R"))?, py::getitem(&v_outputs.clone(), &Py::str("equivalent"))?])).items()?;
    if t1487.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_vals = t1487[0].clone();
    v_eq = t1487[1].clone();
    if (Py::Bool(true)).truthy() {
        return Ok(py::add(&py::truediv(&py::abs(&py::sub(&v_eq.clone(), &py::fsum(&v_vals.clone())?)?)?, &py::fsum(&v_vals.clone())?)?, &py::abs(&py::sub(&py::method(&v_outputs.clone(), "get", &[Py::str("count"), py::len(&v_vals.clone())?])?, &py::len(&v_vals.clone())?)?)?)?);
    }
    return Ok(py::add(&py::abs(&py::sub(&py::fsum(&{ let mut out = vec![]; for t1488 in py::iter(&(v_vals.clone()))? { let mut v_v: Py; 
v_v = t1488;
out.push(py::truediv(&v_eq.clone(), &v_v.clone())?); } Py::list(out) })?, &Py::Float(1.0))?)?, &py::abs(&py::sub(&py::method(&v_outputs.clone(), "get", &[Py::str("count"), py::len(&v_vals.clone())?])?, &py::len(&v_vals.clone())?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1499(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_C: Py = Py::None;
    let mut v_L: Py = Py::None;
    let mut v_R: Py = Py::None;
    let mut v_Z: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_res: Py = Py::None;
    let mut v_w: Py = Py::None;
    let t1500 = (Py::tuple(vec![py::getitem(&v_inputs.clone(), &Py::str("R"))?, py::getitem(&v_inputs.clone(), &Py::str("L"))?, py::getitem(&v_inputs.clone(), &Py::str("C"))?, py::getitem(&v_inputs.clone(), &Py::str("f"))?])).items()?;
    if t1500.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_R = t1500[0].clone();
    v_L = t1500[1].clone();
    v_C = t1500[2].clone();
    v_f = t1500[3].clone();
    v_w = py::mul(&py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?, &v_f.clone())?;
    v_Z = py::complex(&[py::getitem(&v_outputs.clone(), &Py::str("real"))?, py::getitem(&v_outputs.clone(), &Py::str("imag"))?])?;
    v_res = (if (Py::Bool(true)).truthy() { py::truediv(&py::abs(&py::sub(&v_Z.clone(), &py::add(&py::add(&v_R.clone(), &py::mul(&py::mul(&Py::Complex(0.0, 1.0), &v_w.clone())?, &v_L.clone())?)?, &py::truediv(&Py::Int(1i64), &py::mul(&py::mul(&Py::Complex(0.0, 1.0), &v_w.clone())?, &v_C.clone())?)?)?)?)?, &py::abs(&v_Z.clone())?)? } else { py::abs(&py::sub(&py::mul(&v_Z.clone(), &py::add(&py::add(&py::truediv(&Py::Int(1i64), &v_R.clone())?, &py::truediv(&Py::Int(1i64), &py::mul(&py::mul(&Py::Complex(0.0, 1.0), &v_w.clone())?, &v_L.clone())?)?)?, &py::mul(&py::mul(&Py::Complex(0.0, 1.0), &v_w.clone())?, &v_C.clone())?)?)?, &Py::Int(1i64))?)? });
    return Ok(py::add(&py::add(&py::add(&py::add(&v_res.clone(), &f__rel(py::getitem(&v_outputs.clone(), &Py::str("impedance"))?, py::abs(&v_Z.clone())?)?)?, &py::truediv(&py::abs(&py::sub(&py::getitem(&v_outputs.clone(), &Py::str("phase_deg"))?, &py::math1("degrees", &py::cmath("phase", &[v_Z.clone()])?)?)?)?, &Py::Int(180i64))?)?, &f__rel(py::getitem(&v_outputs.clone(), &Py::str("XL"))?, py::mul(&v_w.clone(), &v_L.clone())?)?)?, &f__rel(py::getitem(&v_outputs.clone(), &Py::str("XC"))?, py::truediv(&Py::Int(1i64), &py::mul(&v_w.clone(), &v_C.clone())?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1501(mut v_R: Py, mut v_L: Py, mut v_C: Py, mut v_s: Py) -> R {
    return Ok(f__resonance(v_R.clone(), v_L.clone(), v_C.clone(), v_s.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1508(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_C: Py = Py::None;
    let mut v_L: Py = Py::None;
    let mut v_R: Py = Py::None;
    let mut v_Y: Py = Py::None;
    let mut v_peak: Py = Py::None;
    let t1509 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("R"))?, py::getitem(&v_i.clone(), &Py::str("L"))?, py::getitem(&v_i.clone(), &Py::str("C"))?])).items()?;
    if t1509.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_R = t1509[0].clone();
    v_L = t1509[1].clone();
    v_C = t1509[2].clone();
    v_Y = { let v_C = v_C.clone(); let v_L = v_L.clone(); let v_R = v_R.clone(); let v_Y = v_Y.clone(); let v_i = v_i.clone(); let v_o = v_o.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_f: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_w: Py = Py::None;
            v_w = py::mul(&py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?, &v_f.clone())?;
            return Ok((if (Py::Bool(true)).truthy() { py::truediv(&Py::Int(1i64), &py::complex(&[v_R.clone(), py::sub(&py::mul(&v_w.clone(), &v_L.clone())?, &py::truediv(&Py::Int(1i64), &py::mul(&v_w.clone(), &v_C.clone())?)?)?])?)? } else { py::truediv(&Py::Int(1i64), &py::complex(&[py::truediv(&Py::Int(1i64), &v_R.clone())?, py::sub(&py::mul(&v_w.clone(), &v_C.clone())?, &py::truediv(&Py::Int(1i64), &py::mul(&v_w.clone(), &v_L.clone())?)?)?])?)? }));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_peak = py::abs(&py::call(&v_Y.clone(), &[py::getitem(&v_o.clone(), &Py::str("f0"))?])?)?;
    return Ok(py::add(&py::add(&py::add(&py::add(&py::truediv(&py::abs(&py::imag(&py::call(&v_Y.clone(), &[py::getitem(&v_o.clone(), &Py::str("f0"))?])?)?)?, &v_peak.clone())?, &py::abs(&py::sub(&py::truediv(&py::abs(&py::call(&v_Y.clone(), &[py::getitem(&v_o.clone(), &Py::str("f_low"))?])?)?, &v_peak.clone())?, &py::pow(&Py::Int(2i64), &py::neg(&Py::Float(0.5))?)?)?)?)?, &py::abs(&py::sub(&py::truediv(&py::abs(&py::call(&v_Y.clone(), &[py::getitem(&v_o.clone(), &Py::str("f_high"))?])?)?, &v_peak.clone())?, &py::pow(&Py::Int(2i64), &py::neg(&Py::Float(0.5))?)?)?)?)?, &f__rel(py::getitem(&v_o.clone(), &Py::str("bandwidth"))?, py::sub(&py::getitem(&v_o.clone(), &Py::str("f_high"))?, &py::getitem(&v_o.clone(), &Py::str("f_low"))?)?)?)?, &f__rel(py::getitem(&v_o.clone(), &Py::str("Q"))?, py::truediv(&py::getitem(&v_o.clone(), &Py::str("f0"))?, &py::getitem(&v_o.clone(), &Py::str("bandwidth"))?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1902(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::add(&py::add(&py::add(&py::truediv(&py::abs(&py::sub(&py::mul(&py::getitem(&v_o.clone(), &Py::str("Vs"))?, &py::getitem(&v_i.clone(), &Py::str("Np"))?)?, &py::mul(&py::getitem(&v_i.clone(), &Py::str("Vp"))?, &py::getitem(&v_i.clone(), &Py::str("Ns"))?)?)?)?, &py::minmax(&[py::abs(&py::mul(&py::getitem(&v_i.clone(), &Py::str("Vp"))?, &py::getitem(&v_i.clone(), &Py::str("Ns"))?)?)?, Py::Float(1e-300)], true)?)?, &py::truediv(&py::abs(&py::sub(&py::mul(&py::getitem(&v_o.clone(), &Py::str("Is"))?, &py::getitem(&v_i.clone(), &Py::str("RL"))?)?, &py::getitem(&v_o.clone(), &Py::str("Vs"))?)?)?, &py::minmax(&[py::abs(&py::getitem(&v_o.clone(), &Py::str("Vs"))?)?, Py::Float(1e-300)], true)?)?)?, &py::truediv(&py::abs(&py::sub(&py::mul(&py::getitem(&v_i.clone(), &Py::str("Vp"))?, &py::getitem(&v_o.clone(), &Py::str("Ip"))?)?, &py::mul(&py::getitem(&v_o.clone(), &Py::str("Vs"))?, &py::getitem(&v_o.clone(), &Py::str("Is"))?)?)?)?, &py::minmax(&[py::abs(&py::mul(&py::getitem(&v_o.clone(), &Py::str("Vs"))?, &py::getitem(&v_o.clone(), &Py::str("Is"))?)?)?, Py::Float(1e-300)], true)?)?)?, &f__rel(py::mul(&py::getitem(&v_o.clone(), &Py::str("Rin"))?, &py::getitem(&v_o.clone(), &Py::str("Ip"))?)?, py::getitem(&v_i.clone(), &Py::str("Vp"))?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_194(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("value"))?, &py::getitem(&f_boolean_evaluate(py::getitem(&v_i.clone(), &Py::str("expression"))?, py::getitem(&v_i.clone(), &Py::str("values"))?)?, &Py::str("value"))?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2051(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::truediv(&py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("value"))?, &py::getitem(&f_twos_complement_decode(py::getitem(&v_i.clone(), &Py::str("pattern"))?)?, &Py::str("value"))?)?)?, &py::minmax(&[Py::Float(1.0), py::abs(&py::getitem(&v_o.clone(), &Py::str("value"))?)?], true)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2074(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::add(&py::truediv(&py::abs(&py::sub(&py::add(&py::mul(&py::getitem(&v_o.clone(), &Py::str("I"))?, &py::getitem(&v_i.clone(), &Py::str("R1"))?)?, &py::getitem(&v_o.clone(), &Py::str("Vout"))?)?, &py::getitem(&v_i.clone(), &Py::str("Vin"))?)?)?, &py::minmax(&[py::abs(&py::getitem(&v_i.clone(), &Py::str("Vin"))?)?, Py::Float(1e-300)], true)?)?, &py::truediv(&py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("Vout"))?, &py::mul(&py::getitem(&v_o.clone(), &Py::str("I"))?, &py::getitem(&v_i.clone(), &Py::str("R2"))?)?)?)?, &py::minmax(&[py::abs(&py::getitem(&v_i.clone(), &Py::str("Vin"))?)?, Py::Float(1e-300)], true)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2082(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::add(&py::truediv(&py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("Vout"))?, &py::mul(&py::getitem(&v_i.clone(), &Py::str("Vs"))?, &py::sub(&py::truediv(&py::getitem(&v_i.clone(), &Py::str("R2"))?, &py::add(&py::getitem(&v_i.clone(), &Py::str("R1"))?, &py::getitem(&v_i.clone(), &Py::str("R2"))?)?)?, &py::truediv(&py::getitem(&v_i.clone(), &Py::str("R4"))?, &py::add(&py::getitem(&v_i.clone(), &Py::str("R3"))?, &py::getitem(&v_i.clone(), &Py::str("R4"))?)?)?)?)?)?)?, &py::minmax(&[py::abs(&py::getitem(&v_i.clone(), &Py::str("Vs"))?)?, Py::Float(1e-300)], true)?)?, &f__rel(py::mul(&py::getitem(&v_o.clone(), &Py::str("R4_balance"))?, &py::getitem(&v_i.clone(), &Py::str("R1"))?)?, py::mul(&py::getitem(&v_i.clone(), &Py::str("R2"))?, &py::getitem(&v_i.clone(), &Py::str("R3"))?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2088(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__verify_dw(v_i.clone(), v_o.clone(), Py::Bool(false))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_356(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::add(&py::truediv(&py::abs(&py::sub(&py::add(&py::getitem(&v_o.clone(), &Py::str("I1"))?, &py::getitem(&v_o.clone(), &Py::str("I2"))?)?, &py::getitem(&v_i.clone(), &Py::str("I"))?)?)?, &py::minmax(&[py::abs(&py::getitem(&v_i.clone(), &Py::str("I"))?)?, Py::Float(1e-300)], true)?)?, &py::truediv(&py::abs(&py::sub(&py::mul(&py::getitem(&v_o.clone(), &Py::str("I1"))?, &py::getitem(&v_i.clone(), &Py::str("R1"))?)?, &py::mul(&py::getitem(&v_o.clone(), &Py::str("I2"))?, &py::getitem(&v_i.clone(), &Py::str("R2"))?)?)?)?, &py::minmax(&[py::abs(&py::mul(&py::getitem(&v_i.clone(), &Py::str("I"))?, &py::getitem(&v_i.clone(), &Py::str("R1"))?)?)?, Py::Float(1e-300)], true)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_366(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::add(&py::add(&py::add(&py::truediv(&py::abs(&py::sub(&py::sub(&py::getitem(&v_i.clone(), &Py::str("V"))?, &py::mul(&py::getitem(&v_i.clone(), &Py::str("R"))?, &py::getitem(&v_o.clone(), &Py::str("current"))?)?)?, &py::mul(&py::getitem(&v_i.clone(), &Py::str("Ke"))?, &py::getitem(&v_o.clone(), &Py::str("speed"))?)?)?)?, &py::minmax(&[py::abs(&py::getitem(&v_i.clone(), &Py::str("V"))?)?, Py::Float(1e-300)], true)?)?, &py::truediv(&py::abs(&py::sub(&py::sub(&py::mul(&py::getitem(&v_i.clone(), &Py::str("Kt"))?, &py::getitem(&v_o.clone(), &Py::str("current"))?)?, &py::mul(&py::getitem(&v_i.clone(), &Py::str("b"))?, &py::getitem(&v_o.clone(), &Py::str("speed"))?)?)?, &py::getitem(&v_i.clone(), &Py::str("T"))?)?)?, &py::minmax(&[py::abs(&py::mul(&py::getitem(&v_i.clone(), &Py::str("Kt"))?, &py::getitem(&v_o.clone(), &Py::str("current"))?)?)?, py::abs(&py::getitem(&v_i.clone(), &Py::str("T"))?)?, Py::Float(1e-300)], true)?)?)?, &f__rel(py::truediv(&py::mul(&py::mul(&py::getitem(&v_o.clone(), &Py::str("rpm"))?, &Py::Int(2i64))?, &Py::Float(std::f64::consts::PI))?, &Py::Int(60i64))?, py::getitem(&v_o.clone(), &Py::str("speed"))?)?)?, &py::truediv(&py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("input_power"))?, &py::mul(&py::getitem(&v_i.clone(), &Py::str("V"))?, &py::getitem(&v_o.clone(), &Py::str("current"))?)?)?)?, &py::minmax(&[py::abs(&py::mul(&py::getitem(&v_i.clone(), &Py::str("V"))?, &py::getitem(&v_o.clone(), &Py::str("current"))?)?)?, Py::Float(1e-300)], true)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_519(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__verify_dw(v_i.clone(), v_o.clone(), Py::Bool(true))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_545(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::add(&py::add(&py::truediv(&py::abs(&py::sub(&py::sub(&py::getitem(&v_i.clone(), &Py::str("Vs"))?, &py::mul(&py::getitem(&v_o.clone(), &Py::str("I"))?, &py::getitem(&v_i.clone(), &Py::str("R"))?)?)?, &py::getitem(&v_o.clone(), &Py::str("VD"))?)?)?, &py::getitem(&v_i.clone(), &Py::str("Vs"))?)?, &py::truediv(&py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("I"))?, &py::mul(&py::getitem(&v_i.clone(), &Py::str("Is"))?, &py::math1("expm1", &py::truediv(&py::getitem(&v_o.clone(), &Py::str("VD"))?, &py::mul(&py::getitem(&v_i.clone(), &Py::str("n"))?, &py::getitem(&v_i.clone(), &Py::str("VT"))?)?)?)?)?)?)?, &py::minmax(&[py::getitem(&v_o.clone(), &Py::str("I"))?, Py::Float(1e-300)], true)?)?)?, &f__rel(py::getitem(&v_o.clone(), &Py::str("I_mA"))?, py::mul(&py::getitem(&v_o.clone(), &Py::str("I"))?, &Py::Float(1000.0))?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_556(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::add(&py::truediv(&py::abs(&py::sub(&py::add(&py::getitem(&v_i.clone(), &Py::str("Vf"))?, &py::mul(&py::sub(&py::getitem(&v_i.clone(), &Py::str("V0"))?, &py::getitem(&v_i.clone(), &Py::str("Vf"))?)?, &py::math1("exp", &py::truediv(&py::neg(&py::getitem(&v_o.clone(), &Py::str("t"))?)?, &py::getitem(&v_i.clone(), &Py::str("tau"))?)?)?)?)?, &py::getitem(&v_i.clone(), &Py::str("v"))?)?)?, &py::minmax(&[py::abs(&py::getitem(&v_i.clone(), &Py::str("V0"))?)?, py::abs(&py::getitem(&v_i.clone(), &Py::str("Vf"))?)?, Py::Float(1e-300)], true)?)?, &f__rel(py::mul(&py::getitem(&v_o.clone(), &Py::str("time_constants"))?, &py::getitem(&v_i.clone(), &Py::str("tau"))?)?, py::getitem(&v_o.clone(), &Py::str("t"))?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_564(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::add(&py::truediv(&py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("v"))?, &py::add(&py::getitem(&v_i.clone(), &Py::str("Vf"))?, &py::mul(&py::sub(&py::getitem(&v_i.clone(), &Py::str("V0"))?, &py::getitem(&v_i.clone(), &Py::str("Vf"))?)?, &py::sub(&Py::Int(1i64), &py::getitem(&v_o.clone(), &Py::str("fraction_complete"))?)?)?)?)?)?, &py::minmax(&[py::abs(&py::getitem(&v_i.clone(), &Py::str("V0"))?)?, py::abs(&py::getitem(&v_i.clone(), &Py::str("Vf"))?)?, Py::Float(1e-300)], true)?)?, &py::truediv(&py::abs(&py::add(&py::math1("log", &py::minmax(&[Py::Float(1e-300), py::sub(&Py::Int(1i64), &py::getitem(&v_o.clone(), &Py::str("fraction_complete"))?)?], true)?)?, &py::truediv(&py::getitem(&v_i.clone(), &Py::str("t"))?, &py::getitem(&v_i.clone(), &Py::str("tau"))?)?)?)?, &py::minmax(&[Py::Float(1.0), py::truediv(&py::getitem(&v_i.clone(), &Py::str("t"))?, &py::getitem(&v_i.clone(), &Py::str("tau"))?)?], true)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_680(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::add(&f__rel(py::getitem(&v_o.clone(), &Py::str("energy"))?, py::mul(&py::mul(&Py::Float(0.5), &py::getitem(&v_o.clone(), &Py::str("flux_linkage"))?)?, &py::getitem(&v_i.clone(), &Py::str("I"))?)?)?, &f__rel(py::getitem(&v_o.clone(), &Py::str("flux_linkage"))?, py::mul(&py::getitem(&v_i.clone(), &Py::str("L"))?, &py::getitem(&v_i.clone(), &Py::str("I"))?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_681(mut v_Rin: Py, mut v_Rf: Py, mut v_Vin: Py) -> R {
    return Ok(py::getitem(&Py::tuple(vec![f__check_positive(Py::dict(vec![(Py::str("Rin"), v_Rin.clone()), (Py::str("Rf"), v_Rf.clone())]))?, Py::dict(vec![(Py::str("gain"), py::truediv(&py::neg(&v_Rf.clone())?, &v_Rin.clone())?), (Py::str("Vout"), py::mul(&py::truediv(&py::neg(&v_Rf.clone())?, &v_Rin.clone())?, &v_Vin.clone())?)])]), &Py::Int(1i64))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_687(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::add(&py::truediv(&py::abs(&py::add(&py::truediv(&py::getitem(&v_i.clone(), &Py::str("Vin"))?, &py::getitem(&v_i.clone(), &Py::str("Rin"))?)?, &py::truediv(&py::getitem(&v_o.clone(), &Py::str("Vout"))?, &py::getitem(&v_i.clone(), &Py::str("Rf"))?)?)?)?, &py::minmax(&[py::abs(&py::truediv(&py::getitem(&v_i.clone(), &Py::str("Vin"))?, &py::getitem(&v_i.clone(), &Py::str("Rin"))?)?)?, Py::Float(1e-300)], true)?)?, &f__rel(py::getitem(&v_o.clone(), &Py::str("gain"))?, py::truediv(&py::neg(&py::getitem(&v_i.clone(), &Py::str("Rf"))?)?, &py::getitem(&v_i.clone(), &Py::str("Rin"))?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_695(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::add(&py::add(&py::truediv(&py::abs(&py::sub(&py::math1("degrees", &py::sub(&py::math1("atan", &py::truediv(&py::getitem(&v_o.clone(), &Py::str("w_max"))?, &py::getitem(&v_i.clone(), &Py::str("z"))?)?)?, &py::math1("atan", &py::truediv(&py::getitem(&v_o.clone(), &Py::str("w_max"))?, &py::getitem(&v_i.clone(), &Py::str("p"))?)?)?)?)?, &py::getitem(&v_o.clone(), &Py::str("phase_max_deg"))?)?)?, &Py::Int(90i64))?, &py::mul(&py::abs(&py::sub(&py::truediv(&py::getitem(&v_i.clone(), &Py::str("z"))?, &py::add(&py::pow(&py::getitem(&v_i.clone(), &Py::str("z"))?, &Py::Int(2i64))?, &py::pow(&py::getitem(&v_o.clone(), &Py::str("w_max"))?, &Py::Int(2i64))?)?)?, &py::truediv(&py::getitem(&v_i.clone(), &Py::str("p"))?, &py::add(&py::pow(&py::getitem(&v_i.clone(), &Py::str("p"))?, &Py::Int(2i64))?, &py::pow(&py::getitem(&v_o.clone(), &Py::str("w_max"))?, &Py::Int(2i64))?)?)?)?)?, &py::getitem(&v_o.clone(), &Py::str("w_max"))?)?)?, &py::truediv(&py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("gain_at_w_max_db"))?, &py::mul(&Py::Int(10i64), &py::math1("log10", &py::truediv(&py::add(&py::pow(&py::getitem(&v_i.clone(), &Py::str("z"))?, &Py::Int(2i64))?, &py::pow(&py::getitem(&v_o.clone(), &Py::str("w_max"))?, &Py::Int(2i64))?)?, &py::add(&py::pow(&py::getitem(&v_i.clone(), &Py::str("p"))?, &Py::Int(2i64))?, &py::pow(&py::getitem(&v_o.clone(), &Py::str("w_max"))?, &Py::Int(2i64))?)?)?)?)?)?)?, &py::minmax(&[Py::Float(1.0), py::abs(&py::getitem(&v_o.clone(), &Py::str("gain_at_w_max_db"))?)?], true)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_701(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::add(&py::add(&f__rel(py::getitem(&v_o.clone(), &Py::str("RL"))?, py::getitem(&v_i.clone(), &Py::str("Rth"))?)?, &f__rel(py::getitem(&v_o.clone(), &Py::str("Pmax"))?, py::truediv(&py::mul(&py::pow(&py::getitem(&v_i.clone(), &Py::str("Vth"))?, &Py::Int(2i64))?, &py::getitem(&v_o.clone(), &Py::str("RL"))?)?, &py::pow(&py::add(&py::getitem(&v_i.clone(), &Py::str("Rth"))?, &py::getitem(&v_o.clone(), &Py::str("RL"))?)?, &Py::Int(2i64))?)?)?)?, &py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("efficiency"))?, &Py::Float(0.5))?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_702(mut v_A: Py, mut v_b: Py) -> R {
    let mut v_n: Py = Py::None;
    let mut v_x: Py = Py::None;
    v_n = py::len(&v_A.clone())?;
    if (Py::Bool(({ let mut out = vec![]; for t703 in py::iter(&(v_A.clone()))? { let mut v_r: Py; 
v_r = t703;
out.push({ let t704 = py::len(&v_r.clone())?; Py::Bool({ let t705 = v_n.clone(); if py::ne(&t704, &t705) { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the matrix must be square"))));
    }
    if ({ let t706 = py::len(&v_b.clone())?; Py::Bool({ let t707 = v_n.clone(); if py::ne(&t706, &t707) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&["the vector needs ".to_string(), py::format(&v_n.clone(), "")?, " entries, one per row of the matrix".to_string()].concat()))));
    }
    v_x = f__gauss(v_A.clone(), v_b.clone())?;
    return Ok(Py::dict(vec![(Py::str("currents"), v_x.clone()), (Py::str("first"), py::getitem(&v_x.clone(), &Py::Int(0i64))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_735(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_bi: Py = Py::None;
    let mut v_row: Py = Py::None;
    let mut v_scale: Py = Py::None;
    let mut v_terms: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let mut v_x: Py = Py::None;
    let t736 = (Py::tuple(vec![py::getitem(&v_inputs.clone(), &Py::str("A"))?, py::getitem(&v_inputs.clone(), &Py::str("b"))?, py::getitem(&v_outputs.clone(), &Py::str("currents"))?])).items()?;
    if t736.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_A = t736[0].clone();
    v_b = t736[1].clone();
    v_x = t736[2].clone();
    if ({ let t737 = py::len(&v_x.clone())?; Py::Bool({ let t738 = py::len(&v_b.clone())?; if py::ne(&t737, &t738) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_worst = Py::Float(0.0);
    for t739 in py::iter(&(py::zip(&[v_A.clone(), v_b.clone()])?))? {
        let t744 = (t739).items()?;
        if t744.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_row = t744[0].clone();
        v_bi = t744[1].clone();
        v_terms = { let mut out = vec![]; for t740 in py::iter(&(py::zip(&[v_row.clone(), v_x.clone()])?))? { let mut v_a: Py; let mut v_xi: Py; 
let t741 = (t740).items()?;
if t741.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t741[0].clone();
v_xi = t741[1].clone();
out.push(py::mul(&v_a.clone(), &v_xi.clone())?); } Py::list(out) };
        v_scale = { let t743 = py::add(&py::fsum(&{ let mut out = vec![]; for t742 in py::iter(&(v_terms.clone()))? { let mut v_t: Py; 
v_t = t742;
out.push(py::abs(&v_t.clone())?); } Py::list(out) })?, &py::abs(&v_bi.clone())?)?; if t743.truthy() { t743 } else { Py::Float(1.0) } };
        v_worst = py::minmax(&[v_worst.clone(), py::truediv(&py::abs(&py::sub(&py::fsum(&v_terms.clone())?, &v_bi.clone())?)?, &v_scale.clone())?], true)?;
    }
    return Ok((if ({ let t745 = py::getitem(&v_x.clone(), &Py::Int(0i64))?; Py::Bool({ let t746 = py::getitem(&v_outputs.clone(), &Py::str("first"))?; if py::ne(&t745, &t746) { true } else { false } }) }).truthy() { py::add(&v_worst.clone(), &f__rel(py::getitem(&v_outputs.clone(), &Py::str("first"))?, py::getitem(&v_x.clone(), &Py::Int(0i64))?)?)? } else { v_worst.clone() }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_885(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::add(&py::add(&py::add(&f__rel(py::getitem(&v_o.clone(), &Py::str("ID"))?, py::mul(&py::mul(&py::mul(&Py::Float(0.5), &py::getitem(&v_i.clone(), &Py::str("kn"))?)?, &py::getitem(&v_i.clone(), &Py::str("WL"))?)?, &py::pow(&py::getitem(&v_o.clone(), &Py::str("VOV"))?, &Py::Int(2i64))?)?)?, &py::truediv(&py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("VOV"))?, &py::sub(&py::getitem(&v_i.clone(), &Py::str("VGS"))?, &py::getitem(&v_i.clone(), &Py::str("VTH"))?)?)?)?, &py::abs(&py::getitem(&v_i.clone(), &Py::str("VGS"))?)?)?)?, &f__rel(py::mul(&py::getitem(&v_o.clone(), &Py::str("gm"))?, &py::getitem(&v_o.clone(), &Py::str("VOV"))?)?, py::mul(&Py::Int(2i64), &py::getitem(&v_o.clone(), &Py::str("ID"))?)?)?)?, &f__rel(py::getitem(&v_o.clone(), &Py::str("ID_mA"))?, py::mul(&py::getitem(&v_o.clone(), &Py::str("ID"))?, &Py::Float(1000.0))?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_886(mut v_A: Py, mut v_b: Py) -> R {
    let mut v_n: Py = Py::None;
    let mut v_x: Py = Py::None;
    v_n = py::len(&v_A.clone())?;
    if (Py::Bool(({ let mut out = vec![]; for t887 in py::iter(&(v_A.clone()))? { let mut v_r: Py; 
v_r = t887;
out.push({ let t888 = py::len(&v_r.clone())?; Py::Bool({ let t889 = v_n.clone(); if py::ne(&t888, &t889) { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the matrix must be square"))));
    }
    if ({ let t890 = py::len(&v_b.clone())?; Py::Bool({ let t891 = v_n.clone(); if py::ne(&t890, &t891) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&["the vector needs ".to_string(), py::format(&v_n.clone(), "")?, " entries, one per row of the matrix".to_string()].concat()))));
    }
    v_x = f__gauss(v_A.clone(), v_b.clone())?;
    return Ok(Py::dict(vec![(Py::str("voltages"), v_x.clone()), (Py::str("first"), py::getitem(&v_x.clone(), &Py::Int(0i64))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_919(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_bi: Py = Py::None;
    let mut v_row: Py = Py::None;
    let mut v_scale: Py = Py::None;
    let mut v_terms: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let mut v_x: Py = Py::None;
    let t920 = (Py::tuple(vec![py::getitem(&v_inputs.clone(), &Py::str("A"))?, py::getitem(&v_inputs.clone(), &Py::str("b"))?, py::getitem(&v_outputs.clone(), &Py::str("voltages"))?])).items()?;
    if t920.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_A = t920[0].clone();
    v_b = t920[1].clone();
    v_x = t920[2].clone();
    if ({ let t921 = py::len(&v_x.clone())?; Py::Bool({ let t922 = py::len(&v_b.clone())?; if py::ne(&t921, &t922) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_worst = Py::Float(0.0);
    for t923 in py::iter(&(py::zip(&[v_A.clone(), v_b.clone()])?))? {
        let t928 = (t923).items()?;
        if t928.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_row = t928[0].clone();
        v_bi = t928[1].clone();
        v_terms = { let mut out = vec![]; for t924 in py::iter(&(py::zip(&[v_row.clone(), v_x.clone()])?))? { let mut v_a: Py; let mut v_xi: Py; 
let t925 = (t924).items()?;
if t925.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t925[0].clone();
v_xi = t925[1].clone();
out.push(py::mul(&v_a.clone(), &v_xi.clone())?); } Py::list(out) };
        v_scale = { let t927 = py::add(&py::fsum(&{ let mut out = vec![]; for t926 in py::iter(&(v_terms.clone()))? { let mut v_t: Py; 
v_t = t926;
out.push(py::abs(&v_t.clone())?); } Py::list(out) })?, &py::abs(&v_bi.clone())?)?; if t927.truthy() { t927 } else { Py::Float(1.0) } };
        v_worst = py::minmax(&[v_worst.clone(), py::truediv(&py::abs(&py::sub(&py::fsum(&v_terms.clone())?, &v_bi.clone())?)?, &v_scale.clone())?], true)?;
    }
    return Ok((if ({ let t929 = py::getitem(&v_x.clone(), &Py::Int(0i64))?; Py::Bool({ let t930 = py::getitem(&v_outputs.clone(), &Py::str("first"))?; if py::ne(&t929, &t930) { true } else { false } }) }).truthy() { py::add(&v_worst.clone(), &f__rel(py::getitem(&v_outputs.clone(), &Py::str("first"))?, py::getitem(&v_x.clone(), &Py::Int(0i64))?)?)? } else { v_worst.clone() }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_931(mut v_Rg: Py, mut v_Rf: Py, mut v_Vin: Py) -> R {
    return Ok(py::getitem(&Py::tuple(vec![f__check_positive(Py::dict(vec![(Py::str("Rg"), v_Rg.clone()), (Py::str("Rf"), v_Rf.clone())]))?, Py::dict(vec![(Py::str("gain"), py::add(&Py::Int(1i64), &py::truediv(&v_Rf.clone(), &v_Rg.clone())?)?), (Py::str("Vout"), py::mul(&py::add(&Py::Int(1i64), &py::truediv(&v_Rf.clone(), &v_Rg.clone())?)?, &v_Vin.clone())?)])]), &Py::Int(1i64))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_937(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::add(&py::truediv(&py::abs(&py::sub(&py::truediv(&py::mul(&py::getitem(&v_o.clone(), &Py::str("Vout"))?, &py::getitem(&v_i.clone(), &Py::str("Rg"))?)?, &py::add(&py::getitem(&v_i.clone(), &Py::str("Rg"))?, &py::getitem(&v_i.clone(), &Py::str("Rf"))?)?)?, &py::getitem(&v_i.clone(), &Py::str("Vin"))?)?)?, &py::minmax(&[py::abs(&py::getitem(&v_i.clone(), &Py::str("Vin"))?)?, Py::Float(1e-300)], true)?)?, &f__rel(py::getitem(&v_o.clone(), &Py::str("gain"))?, py::add(&Py::Int(1i64), &py::truediv(&py::getitem(&v_i.clone(), &Py::str("Rf"))?, &py::getitem(&v_i.clone(), &Py::str("Rg"))?)?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}
