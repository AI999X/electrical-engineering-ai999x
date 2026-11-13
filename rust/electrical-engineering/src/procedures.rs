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
    "ac_mesh_currents",
    "ac_node_voltages",
    "ac_norton_equivalent",
    "ac_thevenin_equivalent",
    "adc_conversion",
    "ammeter_shunt",
    "base_conversion",
    "bjt_divider_bias",
    "bode_point",
    "boolean_equivalence",
    "boolean_evaluate",
    "cell_battery",
    "closed_loop_poles",
    "coil_capacitor_parallel",
    "coil_inductance",
    "complex_power",
    "controllability_observability",
    "current_divider",
    "dc_machine_emf",
    "dc_motor",
    "dc_motor_speed_change",
    "dc_node_voltages",
    "dc_shunt_generator",
    "dc_shunt_motor",
    "delta_to_wye",
    "difference_amplifier",
    "diode_circuit",
    "electrical_energy_cost",
    "faraday_emf",
    "first_order_time_to_value",
    "first_order_transient",
    "gain_margin",
    "gray_code",
    "inductor_energy",
    "inverting_amplifier",
    "lead_compensator",
    "magnetic_circuit",
    "max_power_transfer",
    "mesh_currents",
    "minterms_to_sop",
    "mosfet_saturation",
    "mutual_inductance",
    "nodal_voltages",
    "noninverting_amplifier",
    "norton_equivalent",
    "opamp_integrator",
    "parallel_ac_supply",
    "parallel_capacitance",
    "parallel_inductance",
    "parallel_plate_capacitor",
    "parallel_resistance",
    "parallel_rlc_impedance",
    "parallel_rlc_resonance",
    "phase_margin",
    "phasor_sum",
    "plane_wave_lossless",
    "plane_wave_lossy",
    "polar_to_rectangular",
    "power_factor_correction",
    "rc_filter",
    "rectangular_to_polar",
    "resistance_at_temperature",
    "resistor_network",
    "rms_waveform",
    "routh_hurwitz",
    "second_order_step",
    "self_inductance",
    "series_ac_supply",
    "series_capacitance",
    "series_inductance",
    "series_resistance",
    "series_rlc_impedance",
    "series_rlc_resonance",
    "series_rlc_step",
    "sinusoid_time_to_value",
    "sinusoid_values",
    "state_space_to_tf",
    "steady_state_error",
    "summing_amplifier",
    "temperature_coefficient_from_resistances",
    "thevenin_equivalent",
    "three_phase_load",
    "transformer",
    "truth_table_sop",
    "two_wattmeter",
    "twos_complement_decode",
    "twos_complement_encode",
    "voltage_divider",
    "voltmeter_multiplier",
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
        "ac_mesh_currents" => {
            let r = f_ac_mesh_currents(match v.get("R") { Some(x) => py::from_val(x), None => { return None } }, match v.get("X") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V_mag") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V_deg") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_ac_mesh(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ac_node_voltages" => {
            let r = f_ac_node_voltages(match v.get("netlist") { Some(x) => py::from_val(x), None => { return None } }, match v.get("f") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_ac_nodes(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ac_norton_equivalent" => {
            let r = f_ac_norton_equivalent(match v.get("netlist") { Some(x) => py::from_val(x), None => { return None } }, match v.get("node_a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("node_b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("f") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_ac_norton(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ac_thevenin_equivalent" => {
            let r = f_ac_thevenin_equivalent(match v.get("netlist") { Some(x) => py::from_val(x), None => { return None } }, match v.get("node_a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("node_b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("f") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_ac_thevenin(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "adc_conversion" => {
            let r = f_adc_conversion(match v.get("Vin") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Vref") { Some(x) => py::from_val(x), None => { return None } }, match v.get("bits") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_637(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ammeter_shunt" => {
            let r = f_ammeter_shunt(match v.get("Ifs") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Rm") { Some(x) => py::from_val(x), None => { return None } }, match v.get("I") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_ammeter(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
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
            finish(r, |i: &Py, o: &Py| -> f64 { match s_827(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cell_battery" => {
            let r = f_cell_battery(match v.get("E") { Some(x) => py::from_val(x), None => { return None } }, match v.get("r") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("connection") { Some(x) => py::from_val(x), None => { return None } }, match v.get("RL") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_cells(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "closed_loop_poles" => {
            let r = f_closed_loop_poles(match v.get("num") { Some(x) => py::from_val(x), None => { return None } }, match v.get("den") { Some(x) => py::from_val(x), None => { return None } }, match v.get("K") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_poles(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "coil_capacitor_parallel" => {
            let r = f_coil_capacitor_parallel(match v.get("V") { Some(x) => py::from_val(x), None => { return None } }, match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_coil_cap(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "coil_inductance" => {
            let r = f_coil_inductance(match v.get("N") { Some(x) => py::from_val(x), None => { return None } }, match v.get("length") { Some(x) => py::from_val(x), None => { return None } }, match v.get("area") { Some(x) => py::from_val(x), None => { return None } }, match v.get("mu_r") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_coil_l(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
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
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1029(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "dc_machine_emf" => {
            let r = f_dc_machine_emf(match v.get("poles") { Some(x) => py::from_val(x), None => { return None } }, match v.get("flux") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Z") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("winding") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_dc_emf(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "dc_motor" => {
            let r = f_dc_motor(match v.get("V") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Kt") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Ke") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1059(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "dc_motor_speed_change" => {
            let r = f_dc_motor_speed_change(match v.get("V") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Ra") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Ia1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Ia2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("flux_ratio") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_dc_speed(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "dc_node_voltages" => {
            let r = f_dc_node_voltages(match v.get("netlist") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_nodes(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "dc_shunt_generator" => {
            let r = f_dc_shunt_generator(match v.get("V") { Some(x) => py::from_val(x), None => { return None } }, match v.get("IL") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Ra") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Rf") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_dc_gen(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "dc_shunt_motor" => {
            let r = f_dc_shunt_motor(match v.get("V") { Some(x) => py::from_val(x), None => { return None } }, match v.get("I") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Ra") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Rf") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("losses") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_dc_motor(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "delta_to_wye" => {
            let r = f_delta_to_wye(match v.get("Rab") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Rbc") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Rca") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1254(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "difference_amplifier" => {
            let r = f_difference_amplifier(match v.get("V1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R3") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R4") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_diffamp(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "diode_circuit" => {
            let r = f_diode_circuit(match v.get("Vs") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Is") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("VT") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1280(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "electrical_energy_cost" => {
            let r = f_electrical_energy_cost(match v.get("P") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } }, match v.get("tariff") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_energy_cost(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "faraday_emf" => {
            let r = f_faraday_emf(match v.get("N") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dPhi") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dt") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_faraday(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "first_order_time_to_value" => {
            let r = f_first_order_time_to_value(match v.get("V0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Vf") { Some(x) => py::from_val(x), None => { return None } }, match v.get("tau") { Some(x) => py::from_val(x), None => { return None } }, match v.get("v") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1305(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "first_order_transient" => {
            let r = f_first_order_transient(match v.get("V0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Vf") { Some(x) => py::from_val(x), None => { return None } }, match v.get("tau") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1313(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
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
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1429(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "inverting_amplifier" => {
            let r = s_1430(match v.get("Rin") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Rf") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Vin") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1436(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "lead_compensator" => {
            let r = f__lead(match v.get("z") { Some(x) => py::from_val(x), None => { return None } }, match v.get("p") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1444(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "magnetic_circuit" => {
            let r = f_magnetic_circuit(match v.get("N") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lengths") { Some(x) => py::from_val(x), None => { return None } }, match v.get("areas") { Some(x) => py::from_val(x), None => { return None } }, match v.get("mu_r") { Some(x) => py::from_val(x), None => { return None } }, match v.get("flux") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } }, match v.get("current") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_magnetic(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "max_power_transfer" => {
            let r = f_max_power_transfer(match v.get("Vth") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Rth") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1499(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "mesh_currents" => {
            let r = s_1500(match v.get("A") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1533(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "minterms_to_sop" => {
            let r = f_minterms_to_sop(match v.get("minterms") { Some(x) => py::from_val(x), None => { return None } }, match v.get("variables") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dont_cares") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_m2s(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "mosfet_saturation" => {
            let r = f_mosfet_saturation(match v.get("kn") { Some(x) => py::from_val(x), None => { return None } }, match v.get("WL") { Some(x) => py::from_val(x), None => { return None } }, match v.get("VGS") { Some(x) => py::from_val(x), None => { return None } }, match v.get("VTH") { Some(x) => py::from_val(x), None => { return None } }, match v.get("VDS") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1683(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "mutual_inductance" => {
            let r = f_mutual_inductance(match v.get("L1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dI1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dt") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_mutual(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "nodal_voltages" => {
            let r = s_1694(match v.get("A") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1727(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "noninverting_amplifier" => {
            let r = s_1739(match v.get("Rg") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Rf") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Vin") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1745(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "norton_equivalent" => {
            let r = f_norton_equivalent(match v.get("netlist") { Some(x) => py::from_val(x), None => { return None } }, match v.get("node_a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("node_b") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_norton(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "opamp_integrator" => {
            let r = f_opamp_integrator(match v.get("R") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Vin") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V0") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1891(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "parallel_ac_supply" => {
            let r = f_parallel_ac_supply(match v.get("V") { Some(x) => py::from_val(x), None => { return None } }, match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R") { Some(x) => py::from_val(x), None => { return None } }, match v.get("circuit") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } }, match v.get("C") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_parallel_ac(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "parallel_capacitance" => {
            let r = s_1949({ let mut d = vec![]; for k in ["C", "unit"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1960(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "parallel_inductance" => {
            let r = s_1963({ let mut d = vec![]; for k in ["L", "unit"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1974(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "parallel_plate_capacitor" => {
            let r = f_parallel_plate_capacitor(match v.get("V") { Some(x) => py::from_val(x), None => { return None } }, match v.get("d") { Some(x) => py::from_val(x), None => { return None } }, match v.get("A") { Some(x) => py::from_val(x), None => { return None } }, match v.get("eps_r") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(2.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_plate_cap(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "parallel_resistance" => {
            let r = s_1992({ let mut d = vec![]; for k in ["R"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2003(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "parallel_rlc_impedance" => {
            let r = f_parallel_rlc_impedance(match v.get("R") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C") { Some(x) => py::from_val(x), None => { return None } }, match v.get("f") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2013(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "parallel_rlc_resonance" => {
            let r = s_2015(match v.get("R") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C") { Some(x) => py::from_val(x), None => { return None } }, (|| -> R { Ok(Py::Bool(false)) })().ok()?);
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2022(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "phase_margin" => {
            let r = f_phase_margin(match v.get("num") { Some(x) => py::from_val(x), None => { return None } }, match v.get("den") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_pm(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "phasor_sum" => {
            let r = f_phasor_sum(match v.get("amplitudes") { Some(x) => py::from_val(x), None => { return None } }, match v.get("phases_deg") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_phasor_sum(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "plane_wave_lossless" => {
            let r = f_plane_wave_lossless(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("eps_r") { Some(x) => py::from_val(x), None => { return None } }, match v.get("mu_r") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_wave(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "plane_wave_lossy" => {
            let r = f_plane_wave_lossy(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("eps_r") { Some(x) => py::from_val(x), None => { return None } }, match v.get("mu_r") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sigma") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_lossy(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
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
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2190(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "rectangular_to_polar" => {
            let r = f_rectangular_to_polar(match v.get("real") { Some(x) => py::from_val(x), None => { return None } }, match v.get("imag") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_r2p(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "resistance_at_temperature" => {
            let r = f_resistance_at_temperature(match v.get("R1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("alpha1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_r_temp(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "resistor_network" => {
            let r = f_resistor_network(match v.get("network") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_network(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "rms_waveform" => {
            let r = f_rms_waveform(match v.get("shape") { Some(x) => py::from_val(x), None => { return None } }, match v.get("amplitude") { Some(x) => py::from_val(x), None => { return None } }, match v.get("offset") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2251(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "routh_hurwitz" => {
            let r = f_routh_hurwitz(match v.get("coefficients") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_routh(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "second_order_step" => {
            let r = f_second_order_step(match v.get("zeta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("wn") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_step2(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "self_inductance" => {
            let r = f_self_inductance(match v.get("N") { Some(x) => py::from_val(x), None => { return None } }, match v.get("flux") { Some(x) => py::from_val(x), None => { return None } }, match v.get("I") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_self_l(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "series_ac_supply" => {
            let r = f_series_ac_supply(match v.get("V") { Some(x) => py::from_val(x), None => { return None } }, match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R") { Some(x) => py::from_val(x), None => { return None } }, match v.get("circuit") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } }, match v.get("C") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_series_ac(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "series_capacitance" => {
            let r = s_2424({ let mut d = vec![]; for k in ["C", "unit"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2435(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "series_inductance" => {
            let r = s_2438({ let mut d = vec![]; for k in ["L", "unit"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2449(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "series_resistance" => {
            let r = s_2452({ let mut d = vec![]; for k in ["R"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2463(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "series_rlc_impedance" => {
            let r = f_series_rlc_impedance(match v.get("R") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C") { Some(x) => py::from_val(x), None => { return None } }, match v.get("f") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2476(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "series_rlc_resonance" => {
            let r = s_2478(match v.get("R") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C") { Some(x) => py::from_val(x), None => { return None } }, (|| -> R { Ok(Py::Bool(true)) })().ok()?);
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2485(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "series_rlc_step" => {
            let r = f_series_rlc_step(match v.get("R") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_rlc_step(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "sinusoid_time_to_value" => {
            let r = f_sinusoid_time_to_value(match v.get("Vm") { Some(x) => py::from_val(x), None => { return None } }, match v.get("w") { Some(x) => py::from_val(x), None => { return None } }, match v.get("phi") { Some(x) => py::from_val(x), None => { return None } }, match v.get("v") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_sin_time(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "sinusoid_values" => {
            let r = f_sinusoid_values(match v.get("Vm") { Some(x) => py::from_val(x), None => { return None } }, match v.get("w") { Some(x) => py::from_val(x), None => { return None } }, match v.get("phi") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_sinusoid(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
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
        "temperature_coefficient_from_resistances" => {
            let r = f_temperature_coefficient_from_resistances(match v.get("R1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_tc_from(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
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
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2958(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "truth_table_sop" => {
            let r = f_truth_table_sop(match v.get("expression") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_tt(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "two_wattmeter" => {
            let r = f_two_wattmeter(match v.get("W1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("W2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_two_watt(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "twos_complement_decode" => {
            let r = f_twos_complement_decode(match v.get("pattern") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3111(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "twos_complement_encode" => {
            let r = f_twos_complement_encode(match v.get("value") { Some(x) => py::from_val(x), None => { return None } }, match v.get("bits") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_tc_enc(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "voltage_divider" => {
            let r = f_voltage_divider(match v.get("Vin") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3134(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "voltmeter_multiplier" => {
            let r = f_voltmeter_multiplier(match v.get("Ifs") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Rm") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_voltmeter(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "wheatstone_bridge" => {
            let r = f_wheatstone_bridge(match v.get("Vs") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R3") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R4") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3150(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "wye_to_delta" => {
            let r = f_wye_to_delta(match v.get("Ra") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Rb") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Rc") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3156(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
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
fn c__C0() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(299792458.0)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__EPS0() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(8.854187817620389e-12)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__INPUT_ORDER() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::dict(vec![(Py::str("step"), Py::Int(0i64)), (Py::str("ramp"), Py::Int(1i64)), (Py::str("parabola"), Py::Int(2i64))])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__MU0() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(1.2566370614359173e-06)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__SHAPES() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::dict(vec![(Py::str("sine"), Py::Float(0.5)), (Py::str("square"), Py::Float(1.0)), (Py::str("triangle"), Py::Float(0.3333333333333333)), (Py::str("sawtooth"), Py::Float(0.3333333333333333))])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__TWO_PI() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(6.283185307179586)) })());
    V.with(|v| v.clone())
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__G(mut v_num: Py, mut v_den: Py, mut v_w: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_s: Py = Py::None;
    v_s = py::mul(&Py::Complex(0.0, 1.0), &v_w.clone())?;
    v_d = f__pval(v_den.clone(), v_s.clone())?;
    if ({ let t2112 = v_d.clone(); Py::Bool({ let t2113 = Py::Int(0i64); if py::eq(&t2112, &t2113) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("G(s) has a pole on the imaginary axis at this frequency"))));
    }
    return Ok(py::truediv(&f__pval(v_num.clone(), v_s.clone())?, &v_d.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__ac_absorbed_power(mut v_els: Py, mut v_volts: Py, mut v_w: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_dv: Py = Py::None;
    let mut v_name: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_ph: Py = Py::None;
    let mut v_q: Py = Py::None;
    let mut v_t: Py = Py::None;
    let mut v_v2: Py = Py::None;
    let mut v_val: Py = Py::None;
    let t271 = (Py::tuple(vec![Py::Float(0.0), Py::Float(0.0)])).items()?;
    if t271.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_p = t271[0].clone();
    v_q = t271[1].clone();
    for t272 in py::iter(&(v_els.clone()))? {
        let t279 = (t272).items()?;
        if t279.len() != 6 { return Err(PyErr::Value("unpack".into())); }
        v_t = t279[0].clone();
        v_name = t279[1].clone();
        v_a = t279[2].clone();
        v_b = t279[3].clone();
        v_val = t279[4].clone();
        v_ph = t279[5].clone();
        if ({ let t273 = v_t.clone(); Py::Bool({ let t274 = Py::str("RLC"); if py::contains(&t274, &t273)? { true } else { false } }) }).truthy() {
            v_dv = py::sub(&py::getitem(&v_volts.clone(), &v_a.clone())?, &py::getitem(&v_volts.clone(), &v_b.clone())?)?;
            v_v2 = py::add(&py::mul(&py::real(&v_dv.clone())?, &py::real(&v_dv.clone())?)?, &py::mul(&py::imag(&v_dv.clone())?, &py::imag(&v_dv.clone())?)?)?;
            if ({ let t275 = v_t.clone(); Py::Bool({ let t276 = Py::str("R"); if py::eq(&t275, &t276) { true } else { false } }) }).truthy() {
                v_p = py::iadd(&v_p, &py::truediv(&v_v2.clone(), &v_val.clone())?)?;
            } else {
                if ({ let t277 = v_t.clone(); Py::Bool({ let t278 = Py::str("L"); if py::eq(&t277, &t278) { true } else { false } }) }).truthy() {
                    v_q = py::iadd(&v_q, &py::truediv(&v_v2.clone(), &py::mul(&v_w.clone(), &v_val.clone())?)?)?;
                } else {
                    v_q = py::sub(&v_q, &py::mul(&py::mul(&v_v2.clone(), &v_w.clone())?, &v_val.clone())?)?;
                }
            }
        }
    }
    return Ok(py::complex(&[v_p.clone(), v_q.clone()])?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__ac_admittance(mut v_t: Py, mut v_val: Py, mut v_w: Py) -> R {
    if ({ let t519 = v_t.clone(); Py::Bool({ let t520 = Py::str("R"); if py::eq(&t519, &t520) { true } else { false } }) }).truthy() {
        return Ok(py::complex(&[py::truediv(&Py::Float(1.0), &v_val.clone())?, Py::Float(0.0)])?);
    }
    if ({ let t521 = v_t.clone(); Py::Bool({ let t522 = Py::str("L"); if py::eq(&t521, &t522) { true } else { false } }) }).truthy() {
        return Ok(py::complex(&[Py::Float(0.0), py::truediv(&py::neg(&Py::Float(1.0))?, &py::mul(&v_w.clone(), &v_val.clone())?)?])?);
    }
    return Ok(py::complex(&[Py::Float(0.0), py::mul(&v_w.clone(), &v_val.clone())?])?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__ac_check(mut v_V: Py, mut v_f: Py, mut v_R: Py, mut v_circuit: Py, mut v_L: Py, mut v_C: Py) -> R {
    let _ = f__check_positive(Py::dict(vec![(Py::str("V"), v_V.clone()), (Py::str("f"), v_f.clone()), (Py::str("R"), v_R.clone())]))?;
    if ({ let t2378 = py::not(&{ let t2372 = py::math1("isfinite", &v_L.clone())?; if !t2372.truthy() { t2372 } else { py::math1("isfinite", &v_C.clone())? } }); if t2378.truthy() { t2378 } else { { let t2377 = { let t2373 = v_L.clone(); Py::Bool({ let t2374 = Py::Int(0i64); if py::lt(&t2373, &t2374)? { true } else { false } }) }; if t2377.truthy() { t2377 } else { { let t2375 = v_C.clone(); Py::Bool({ let t2376 = Py::Int(0i64); if py::lt(&t2375, &t2376)? { true } else { false } }) } } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("L and C must not be negative"))));
    }
    if ({ let t2386 = { let t2379 = v_circuit.clone(); Py::Bool({ let t2380 = Py::str("RL"); if py::eq(&t2379, &t2380) { true } else { false } }) }; if !t2386.truthy() { t2386 } else { py::not(&{ let t2385 = { let t2381 = v_L.clone(); Py::Bool({ let t2382 = Py::Int(0i64); if py::gt(&t2381, &t2382)? { true } else { false } }) }; if !t2385.truthy() { t2385 } else { { let t2383 = v_C.clone(); Py::Bool({ let t2384 = Py::Int(0i64); if py::eq(&t2383, &t2384) { true } else { false } }) } } }) } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("an R-L circuit needs an inductance L > 0 and no capacitor"))));
    }
    if ({ let t2394 = { let t2387 = v_circuit.clone(); Py::Bool({ let t2388 = Py::str("RC"); if py::eq(&t2387, &t2388) { true } else { false } }) }; if !t2394.truthy() { t2394 } else { py::not(&{ let t2393 = { let t2389 = v_C.clone(); Py::Bool({ let t2390 = Py::Int(0i64); if py::gt(&t2389, &t2390)? { true } else { false } }) }; if !t2393.truthy() { t2393 } else { { let t2391 = v_L.clone(); Py::Bool({ let t2392 = Py::Int(0i64); if py::eq(&t2391, &t2392) { true } else { false } }) } } }) } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("an R-C circuit needs a capacitance C > 0 and no inductor"))));
    }
    if ({ let t2402 = { let t2395 = v_circuit.clone(); Py::Bool({ let t2396 = Py::str("RLC"); if py::eq(&t2395, &t2396) { true } else { false } }) }; if !t2402.truthy() { t2402 } else { py::not(&{ let t2401 = { let t2397 = v_L.clone(); Py::Bool({ let t2398 = Py::Int(0i64); if py::gt(&t2397, &t2398)? { true } else { false } }) }; if !t2401.truthy() { t2401 } else { { let t2399 = v_C.clone(); Py::Bool({ let t2400 = Py::Int(0i64); if py::gt(&t2399, &t2400)? { true } else { false } }) } } }) } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("an R-L-C circuit needs both L > 0 and C > 0"))));
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__ac_mesh_out(mut v_I: Py) -> R {
    let mut v_deg: Py = Py::None;
    let mut v_im: Py = Py::None;
    let mut v_mag: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_re_: Py = Py::None;
    let mut v_scale: Py = Py::None;
    let mut v_z: Py = Py::None;
    v_scale = Py::Float(0.0);
    for t34 in py::iter(&(v_I.clone()))? {
        v_z = t34;
        v_scale = py::minmax(&[v_scale.clone(), f__cabs(v_z.clone())?], true)?;
    }
    let t35 = (Py::tuple(vec![Py::list(vec![]), Py::list(vec![]), Py::list(vec![]), Py::list(vec![])])).items()?;
    if t35.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_re_ = t35[0].clone();
    v_im = t35[1].clone();
    v_mag = t35[2].clone();
    v_deg = t35[3].clone();
    for t36 in py::iter(&(v_I.clone()))? {
        v_z = t36;
        v_p = f__phasor_parts(v_z.clone(), v_scale.clone())?;
        let _ = py::method(&v_re_.clone(), "append", &[py::getitem(&v_p.clone(), &Py::Int(0i64))?])?;
        let _ = py::method(&v_im.clone(), "append", &[py::getitem(&v_p.clone(), &Py::Int(1i64))?])?;
        let _ = py::method(&v_mag.clone(), "append", &[py::getitem(&v_p.clone(), &Py::Int(2i64))?])?;
        let _ = py::method(&v_deg.clone(), "append", &[py::getitem(&v_p.clone(), &Py::Int(3i64))?])?;
    }
    return Ok(Py::dict(vec![(Py::str("currents_real"), v_re_.clone()), (Py::str("currents_imag"), v_im.clone()), (Py::str("currents_mag"), v_mag.clone()), (Py::str("currents_deg"), v_deg.clone()), (Py::str("first_mag"), py::getitem(&v_mag.clone(), &Py::Int(0i64))?), (Py::str("first_deg"), py::getitem(&v_deg.clone(), &Py::Int(0i64))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__ac_mna(mut v_els: Py, mut v_w: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_A0: Py = Py::None;
    let mut v_N: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_cur: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_e: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_ia: Py = Py::None;
    let mut v_ib: Py = Py::None;
    let mut v_idx: Py = Py::None;
    let mut v_imax: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_name: Py = Py::None;
    let mut v_nodes: Py = Py::None;
    let mut v_ph: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_res: Py = Py::None;
    let mut v_row: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_src: Py = Py::None;
    let mut v_t: Py = Py::None;
    let mut v_val: Py = Py::None;
    let mut v_vmax: Py = Py::None;
    let mut v_volts: Py = Py::None;
    let mut v_vs: Py = Py::None;
    let mut v_x: Py = Py::None;
    let mut v_y: Py = Py::None;
    let mut v_z: Py = Py::None;
    v_nodes = py::sorted_key(&py::set(&{ let mut out = vec![]; for t505 in py::iter(&(v_els.clone()))? { let mut v_e: Py; 
v_e = t505;
for t506 in py::iter(&(Py::tuple(vec![py::getitem(&v_e.clone(), &Py::Int(2i64))?, py::getitem(&v_e.clone(), &Py::Int(3i64))?])))? { let mut v_n: Py; 
v_n = t506;
if ({ let t507 = v_n.clone(); Py::Bool({ let t508 = Py::str("0"); if py::ne(&t507, &t508) { true } else { false } }) }).truthy() { out.push(v_n.clone()); }}} Py::list(out) })?, &py::func(|a: &[Py]| -> R { f__node_key(a.get(0).cloned().unwrap_or(Py::None)) }), (Py::Bool(false)).truthy())?;
    v_idx = { let mut out = vec![]; for t509 in py::iter(&(py::enumerate(&v_nodes.clone(), 0)?))? { let mut v_i: Py; let mut v_n: Py; 
let t510 = (t509).items()?;
if t510.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_i = t510[0].clone();
v_n = t510[1].clone();
out.push((v_n.clone(), v_i.clone())); } Py::dict(out) };
    v_vs = { let mut out = vec![]; for t511 in py::iter(&(v_els.clone()))? { let mut v_e: Py; 
v_e = t511;
if ({ let t512 = py::getitem(&v_e.clone(), &Py::Int(0i64))?; Py::Bool({ let t513 = Py::str("V"); if py::eq(&t512, &t513) { true } else { false } }) }).truthy() { out.push(v_e.clone()); }} Py::list(out) };
    v_N = py::add(&py::len(&v_nodes.clone())?, &py::len(&v_vs.clone())?)?;
    v_A = { let mut out = vec![]; for t514 in py::range_iter(&[(v_N.clone()).i()?]) { let mut v__: Py; 
v__ = t514;
out.push(py::mul(&Py::list(vec![py::complex(&[Py::Float(0.0), Py::Float(0.0)])?]), &v_N.clone())?); } Py::list(out) };
    v_z = py::mul(&Py::list(vec![py::complex(&[Py::Float(0.0), Py::Float(0.0)])?]), &v_N.clone())?;
    for t515 in py::iter(&(v_els.clone()))? {
        let t544 = (t515).items()?;
        if t544.len() != 6 { return Err(PyErr::Value("unpack".into())); }
        v_t = t544[0].clone();
        v_name = t544[1].clone();
        v_a = t544[2].clone();
        v_b = t544[3].clone();
        v_val = t544[4].clone();
        v_ph = t544[5].clone();
        let t516 = (Py::tuple(vec![py::method(&v_idx.clone(), "get", &[v_a.clone()])?, py::method(&v_idx.clone(), "get", &[v_b.clone()])?])).items()?;
        if t516.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_ia = t516[0].clone();
        v_ib = t516[1].clone();
        if ({ let t517 = v_t.clone(); Py::Bool({ let t518 = Py::str("RLC"); if py::contains(&t518, &t517)? { true } else { false } }) }).truthy() {
            v_y = f__ac_admittance(v_t.clone(), v_val.clone(), v_w.clone())?;
            for t523 in py::iter(&(Py::tuple(vec![Py::tuple(vec![v_ia.clone(), v_ia.clone(), v_y.clone()]), Py::tuple(vec![v_ib.clone(), v_ib.clone(), v_y.clone()]), Py::tuple(vec![v_ia.clone(), v_ib.clone(), py::neg(&v_y.clone())?]), Py::tuple(vec![v_ib.clone(), v_ia.clone(), py::neg(&v_y.clone())?])])))? {
                let t529 = (t523).items()?;
                if t529.len() != 3 { return Err(PyErr::Value("unpack".into())); }
                v_i = t529[0].clone();
                v_j = t529[1].clone();
                v_s = t529[2].clone();
                if ({ let t528 = { let t524 = v_i.clone(); Py::Bool({ let t525 = Py::None; if !py::is(&t524, &t525) { true } else { false } }) }; if !t528.truthy() { t528 } else { { let t526 = v_j.clone(); Py::Bool({ let t527 = Py::None; if !py::is(&t526, &t527) { true } else { false } }) } } }).truthy() {
                    py::setitem(&py::getitem(&v_A.clone(), &v_i.clone())?, &v_j.clone(), py::add(&py::getitem(&py::getitem(&v_A.clone(), &v_i.clone())?, &v_j.clone())?, &v_s.clone())?)?;
                }
            }
        } else {
            if ({ let t530 = v_t.clone(); Py::Bool({ let t531 = Py::str("I"); if py::eq(&t530, &t531) { true } else { false } }) }).truthy() {
                v_src = f__rect_deg(v_val.clone(), v_ph.clone())?;
                if ({ let t540 = v_ia.clone(); Py::Bool({ let t541 = Py::None; if !py::is(&t540, &t541) { true } else { false } }) }).truthy() {
                    py::setitem(&v_z.clone(), &v_ia.clone(), py::sub(&py::getitem(&v_z.clone(), &v_ia.clone())?, &v_src.clone())?)?;
                }
                if ({ let t542 = v_ib.clone(); Py::Bool({ let t543 = Py::None; if !py::is(&t542, &t543) { true } else { false } }) }).truthy() {
                    py::setitem(&v_z.clone(), &v_ib.clone(), py::add(&py::getitem(&v_z.clone(), &v_ib.clone())?, &v_src.clone())?)?;
                }
            }
        }
    }
    for t545 in py::iter(&(py::enumerate(&v_vs.clone(), 0)?))? {
        let t551 = (t545).items()?;
        if t551.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t551[0].clone();
        let t552 = (t551[1].clone()).items()?;
        if t552.len() != 6 { return Err(PyErr::Value("unpack".into())); }
        v_t = t552[0].clone();
        v_name = t552[1].clone();
        v_a = t552[2].clone();
        v_b = t552[3].clone();
        v_val = t552[4].clone();
        v_ph = t552[5].clone();
        v_r = py::add(&py::len(&v_nodes.clone())?, &v_k.clone())?;
        let t546 = (Py::tuple(vec![py::method(&v_idx.clone(), "get", &[v_a.clone()])?, py::method(&v_idx.clone(), "get", &[v_b.clone()])?])).items()?;
        if t546.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_ia = t546[0].clone();
        v_ib = t546[1].clone();
        if ({ let t547 = v_ia.clone(); Py::Bool({ let t548 = Py::None; if !py::is(&t547, &t548) { true } else { false } }) }).truthy() {
            py::setitem(&py::getitem(&v_A.clone(), &v_ia.clone())?, &v_r.clone(), py::add(&py::getitem(&py::getitem(&v_A.clone(), &v_ia.clone())?, &v_r.clone())?, &Py::Float(1.0))?)?;
            py::setitem(&py::getitem(&v_A.clone(), &v_r.clone())?, &v_ia.clone(), py::add(&py::getitem(&py::getitem(&v_A.clone(), &v_r.clone())?, &v_ia.clone())?, &Py::Float(1.0))?)?;
        }
        if ({ let t549 = v_ib.clone(); Py::Bool({ let t550 = Py::None; if !py::is(&t549, &t550) { true } else { false } }) }).truthy() {
            py::setitem(&py::getitem(&v_A.clone(), &v_ib.clone())?, &v_r.clone(), py::sub(&py::getitem(&py::getitem(&v_A.clone(), &v_ib.clone())?, &v_r.clone())?, &Py::Float(1.0))?)?;
            py::setitem(&py::getitem(&v_A.clone(), &v_r.clone())?, &v_ib.clone(), py::sub(&py::getitem(&py::getitem(&v_A.clone(), &v_r.clone())?, &v_ib.clone())?, &Py::Float(1.0))?)?;
        }
        py::setitem(&v_z.clone(), &v_r.clone(), f__rect_deg(v_val.clone(), v_ph.clone())?)?;
    }
    v_A0 = Py::list(vec![]);
    for t553 in py::iter(&(v_A.clone()))? {
        v_row = t553;
        let _ = py::method(&v_A0.clone(), "append", &[Py::list((v_row.clone()).items()?)])?;
    }
    v_x = f__cgauss(v_A0.clone(), v_z.clone())?;
    v_res = Py::list(vec![]);
    for t577 in py::range_iter(&[(v_N.clone()).i()?]) {
        v_i = t577;
        v_s = py::getitem(&v_z.clone(), &v_i.clone())?;
        for t578 in py::range_iter(&[(v_N.clone()).i()?]) {
            v_j = t578;
            v_s = py::sub(&v_s.clone(), &py::mul(&py::getitem(&py::getitem(&v_A0.clone(), &v_i.clone())?, &v_j.clone())?, &py::getitem(&v_x.clone(), &v_j.clone())?)?)?;
        }
        let _ = py::method(&v_res.clone(), "append", &[v_s.clone()])?;
    }
    v_d = f__cgauss(v_A0.clone(), v_res.clone())?;
    for t579 in py::range_iter(&[(v_N.clone()).i()?]) {
        v_i = t579;
        py::setitem(&v_x.clone(), &v_i.clone(), py::add(&py::getitem(&v_x.clone(), &v_i.clone())?, &py::getitem(&v_d.clone(), &v_i.clone())?)?)?;
    }
    v_vmax = { let t = { let mut out = vec![]; for t580 in py::iter(&(v_nodes.clone()))? { let mut v_n: Py; 
v_n = t580;
out.push(f__cabs(py::getitem(&v_x.clone(), &py::getitem(&v_idx.clone(), &v_n.clone())?)?)?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } };
    v_imax = { let t = { let mut out = vec![]; for t581 in py::range_iter(&[(py::len(&v_vs.clone())?).i()?]) { let mut v_k: Py; 
v_k = t581;
out.push(f__cabs(py::getitem(&v_x.clone(), &py::add(&py::len(&v_nodes.clone())?, &v_k.clone())?)?)?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } };
    v_volts = Py::dict(vec![(Py::str("0"), py::complex(&[Py::Float(0.0), Py::Float(0.0)])?)]);
    for t582 in py::iter(&(v_nodes.clone()))? {
        v_n = t582;
        py::setitem(&v_volts.clone(), &v_n.clone(), f__csnap(py::getitem(&v_x.clone(), &py::getitem(&v_idx.clone(), &v_n.clone())?)?, v_vmax.clone())?)?;
    }
    v_cur = Py::dict(vec![]);
    for t588 in py::iter(&(py::enumerate(&v_vs.clone(), 0)?))? {
        let t589 = (t588).items()?;
        if t589.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t589[0].clone();
        v_e = t589[1].clone();
        py::setitem(&v_cur.clone(), &py::getitem(&v_e.clone(), &Py::Int(1i64))?, f__csnap(py::getitem(&v_x.clone(), &py::add(&py::len(&v_nodes.clone())?, &v_k.clone())?)?, v_imax.clone())?)?;
    }
    return Ok(Py::tuple(vec![v_volts.clone(), v_cur.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__ac_netlist(mut v_text: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_els: Py = Py::None;
    let mut v_name: Py = Py::None;
    let mut v_names: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_part: Py = Py::None;
    let mut v_ph: Py = Py::None;
    let mut v_t: Py = Py::None;
    let mut v_toks: Py = Py::None;
    let mut v_val: Py = Py::None;
    let t458 = (Py::tuple(vec![Py::list(vec![]), py::set(&Py::list(vec![]))?])).items()?;
    if t458.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_els = t458[0].clone();
    v_names = t458[1].clone();
    for t459 in py::iter(&(ax_engine::pyre::module_call("split", &Py::str("[;\\n]+"), &[v_text.clone(), Py::Int(0)], 0)?))? {
        v_part = t459;
        v_p = py::method(&py::method(&py::method(&v_part.clone(), "strip", &[])?, "rstrip", &[Py::str(".")])?, "strip", &[])?;
        if (py::not(&v_p.clone())).truthy() {
            continue;
        }
        v_toks = py::method(&v_p.clone(), "split", &[])?;
        v_t = py::method(&py::getitem(&py::getitem(&v_toks.clone(), &Py::Int(0i64))?, &Py::Int(0i64))?, "upper", &[])?;
        if ({ let t460 = v_t.clone(); Py::Bool({ let t461 = Py::str("RLCVI"); if !py::contains(&t461, &t460)? { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str(&["element '".to_string(), py::format(&py::getitem(&v_toks.clone(), &Py::Int(0i64))?, "")?, "': only R, L, C, voltage sources (V) and current sources (I)".to_string()].concat()))));
        }
        if ({ let t472 = { let t466 = { let t462 = v_t.clone(); Py::Bool({ let t463 = Py::str("RLC"); if py::contains(&t463, &t462)? { true } else { false } }) }; if !t466.truthy() { t466 } else { { let t464 = py::len(&v_toks.clone())?; Py::Bool({ let t465 = Py::Int(4i64); if py::ne(&t464, &t465) { true } else { false } }) } } }; if t472.truthy() { t472 } else { { let t471 = { let t467 = v_t.clone(); Py::Bool({ let t468 = Py::str("VI"); if py::contains(&t468, &t467)? { true } else { false } }) }; if !t471.truthy() { t471 } else { { let t469 = py::len(&v_toks.clone())?; Py::Bool({ let t470 = Py::tuple(vec![Py::Int(4i64), Py::Int(5i64)]); if !py::contains(&t470, &t469)? { true } else { false } }) } } } } }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str(&["cannot read the element '".to_string(), py::format(&v_p.clone(), "")?, "': write <name> <node> <node> <value> (sources: [<angle in degrees>])".to_string()].concat()))));
        }
        let t473 = (Py::tuple(vec![py::getitem(&v_toks.clone(), &Py::Int(0i64))?, py::getitem(&v_toks.clone(), &Py::Int(1i64))?, py::getitem(&v_toks.clone(), &Py::Int(2i64))?])).items()?;
        if t473.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v_name = t473[0].clone();
        v_a = t473[1].clone();
        v_b = t473[2].clone();
        if ({ let t474 = py::method(&v_name.clone(), "upper", &[])?; Py::Bool({ let t475 = v_names.clone(); if py::contains(&t475, &t474)? { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str(&["element name '".to_string(), py::format(&v_name.clone(), "")?, "' is used twice".to_string()].concat()))));
        }
        let _ = py::method(&v_names.clone(), "add", &[py::method(&v_name.clone(), "upper", &[])?])?;
        let t478: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                v_val = py::float(&py::getitem(&v_toks.clone(), &Py::Int(3i64))?)?;
                v_ph = (if ({ let t476 = py::len(&v_toks.clone())?; Py::Bool({ let t477 = Py::Int(5i64); if py::eq(&t476, &t477) { true } else { false } }) }).truthy() { py::float(&py::getitem(&v_toks.clone(), &Py::Int(4i64))?)? } else { Py::Float(0.0) });
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t478 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (matches!(e, PyErr::Value(_))) => {
                return Err(py::value_error(py::to_str(&Py::str(&["element '".to_string(), py::format(&v_name.clone(), "")?, "': its value is not a number".to_string()].concat()))));
            }
            Err(e) => return Err(e),
        }
        if (py::not(&{ let t479 = py::math1("isfinite", &v_val.clone())?; if !t479.truthy() { t479 } else { py::math1("isfinite", &v_ph.clone())? } })).truthy() {
            return Err(py::value_error(py::to_str(&Py::str(&["element '".to_string(), py::format(&v_name.clone(), "")?, "': the value must be finite".to_string()].concat()))));
        }
        if ({ let t484 = { let t480 = v_t.clone(); Py::Bool({ let t481 = Py::str("RLC"); if py::contains(&t481, &t480)? { true } else { false } }) }; if !t484.truthy() { t484 } else { { let t482 = v_val.clone(); Py::Bool({ let t483 = Py::Int(0i64); if py::le(&t482, &t483)? { true } else { false } }) } } }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str(&["element '".to_string(), py::format(&v_name.clone(), "")?, "' must have a positive value".to_string()].concat()))));
        }
        if ({ let t489 = { let t485 = v_t.clone(); Py::Bool({ let t486 = Py::str("VI"); if py::contains(&t486, &t485)? { true } else { false } }) }; if !t489.truthy() { t489 } else { { let t487 = v_val.clone(); Py::Bool({ let t488 = Py::Int(0i64); if py::lt(&t487, &t488)? { true } else { false } }) } } }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str(&["source '".to_string(), py::format(&v_name.clone(), "")?, "': give a non-negative magnitude (a sign is an angle of 180 degrees)".to_string()].concat()))));
        }
        if ({ let t490 = v_a.clone(); Py::Bool({ let t491 = v_b.clone(); if py::eq(&t490, &t491) { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str(&["element '".to_string(), py::format(&v_name.clone(), "")?, "' connects node ".to_string(), py::format(&v_a.clone(), "")?, " to itself".to_string()].concat()))));
        }
        let _ = py::method(&v_els.clone(), "append", &[Py::tuple(vec![v_t.clone(), v_name.clone(), v_a.clone(), v_b.clone(), v_val.clone(), v_ph.clone()])])?;
    }
    if (py::not(&Py::Bool(({ let mut out = vec![]; for t492 in py::iter(&(v_els.clone()))? { let mut v_e: Py; 
v_e = t492;
out.push({ let t493 = py::getitem(&v_e.clone(), &Py::Int(0i64))?; Py::Bool({ let t494 = Py::str("RLC"); if py::contains(&t494, &t493)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the circuit needs at least one resistor, inductor or capacitor"))));
    }
    if (py::not(&Py::Bool(({ let mut out = vec![]; for t495 in py::iter(&(v_els.clone()))? { let mut v_e: Py; 
v_e = t495;
out.push({ let t496 = Py::str("0"); Py::Bool({ let t497 = Py::tuple(vec![py::getitem(&v_e.clone(), &Py::Int(2i64))?, py::getitem(&v_e.clone(), &Py::Int(3i64))?]); if py::contains(&t497, &t496)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the circuit needs a ground node named 0"))));
    }
    return Ok(v_els.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__ac_nodes_out(mut v_volts: Py, mut v_cur: Py, mut v_S: Py) -> R {
    let mut v_P: Py = Py::None;
    let mut v_Q: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_ci: Py = Py::None;
    let mut v_cr: Py = Py::None;
    let mut v_imax: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_sc: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_vd: Py = Py::None;
    let mut v_vi: Py = Py::None;
    let mut v_vm: Py = Py::None;
    let mut v_vmax: Py = Py::None;
    let mut v_vr: Py = Py::None;
    v_vmax = Py::Float(0.0);
    for t220 in py::iter(&(py::method(&v_volts.clone(), "values", &[])?))? {
        v_v = t220;
        v_vmax = py::minmax(&[v_vmax.clone(), f__cabs(v_v.clone())?], true)?;
    }
    v_imax = Py::Float(0.0);
    for t221 in py::iter(&(py::method(&v_cur.clone(), "values", &[])?))? {
        v_c = t221;
        v_imax = py::minmax(&[v_imax.clone(), f__cabs(v_c.clone())?], true)?;
    }
    let t222 = (Py::tuple(vec![Py::dict(vec![]), Py::dict(vec![]), Py::dict(vec![]), Py::dict(vec![]), Py::dict(vec![]), Py::dict(vec![])])).items()?;
    if t222.len() != 6 { return Err(PyErr::Value("unpack".into())); }
    v_vr = t222[0].clone();
    v_vi = t222[1].clone();
    v_vm = t222[2].clone();
    v_vd = t222[3].clone();
    v_cr = t222[4].clone();
    v_ci = t222[5].clone();
    for t223 in py::iter(&(py::method(&v_volts.clone(), "items", &[])?))? {
        let t236 = (t223).items()?;
        if t236.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_n = t236[0].clone();
        v_v = t236[1].clone();
        if ({ let t224 = v_n.clone(); Py::Bool({ let t225 = Py::str("0"); if py::ne(&t224, &t225) { true } else { false } }) }).truthy() {
            v_p = f__phasor_parts(v_v.clone(), v_vmax.clone())?;
            let t235 = (Py::tuple(vec![py::getitem(&v_p.clone(), &Py::Int(0i64))?, py::getitem(&v_p.clone(), &Py::Int(1i64))?, py::getitem(&v_p.clone(), &Py::Int(2i64))?, py::getitem(&v_p.clone(), &Py::Int(3i64))?])).items()?;
            if t235.len() != 4 { return Err(PyErr::Value("unpack".into())); }
            py::setitem(&v_vr.clone(), &v_n.clone(), t235[0].clone())?;
            py::setitem(&v_vi.clone(), &v_n.clone(), t235[1].clone())?;
            py::setitem(&v_vm.clone(), &v_n.clone(), t235[2].clone())?;
            py::setitem(&v_vd.clone(), &v_n.clone(), t235[3].clone())?;
        }
    }
    for t237 in py::iter(&(py::method(&v_cur.clone(), "items", &[])?))? {
        let t239 = (t237).items()?;
        if t239.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t239[0].clone();
        v_c = t239[1].clone();
        v_p = f__phasor_parts(v_c.clone(), v_imax.clone())?;
        let t238 = (Py::tuple(vec![py::getitem(&v_p.clone(), &Py::Int(0i64))?, py::getitem(&v_p.clone(), &Py::Int(1i64))?])).items()?;
        if t238.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        py::setitem(&v_cr.clone(), &v_k.clone(), t238[0].clone())?;
        py::setitem(&v_ci.clone(), &v_k.clone(), t238[1].clone())?;
    }
    v_sc = py::minmax(&[py::abs(&py::real(&v_S.clone())?)?, py::abs(&py::imag(&v_S.clone())?)?], true)?;
    v_P = (if ({ let t240 = py::abs(&py::real(&v_S.clone())?)?; Py::Bool({ let t241 = py::mul(&Py::Float(1e-12), &v_sc.clone())?; if py::le(&t240, &t241)? { true } else { false } }) }).truthy() { Py::Float(0.0) } else { py::real(&v_S.clone())? });
    v_Q = (if ({ let t242 = py::abs(&py::imag(&v_S.clone())?)?; Py::Bool({ let t243 = py::mul(&Py::Float(1e-12), &v_sc.clone())?; if py::le(&t242, &t243)? { true } else { false } }) }).truthy() { Py::Float(0.0) } else { py::imag(&v_S.clone())? });
    return Ok(Py::dict(vec![(Py::str("node_voltages_mag"), v_vm.clone()), (Py::str("node_voltages_deg"), v_vd.clone()), (Py::str("node_voltages_real"), v_vr.clone()), (Py::str("node_voltages_imag"), v_vi.clone()), (Py::str("source_currents_real"), v_cr.clone()), (Py::str("source_currents_imag"), v_ci.clone()), (Py::str("P_delivered"), f__r12(v_P.clone())?), (Py::str("Q_delivered"), f__r12(v_Q.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__ac_port_out(mut v_prefix_v: Py, mut v_v: Py, mut v_prefix_z: Py, mut v_z: Py) -> R {
    let mut v_pv: Py = Py::None;
    let mut v_pz: Py = Py::None;
    v_pv = f__phasor_parts(v_v.clone(), f__cabs(v_v.clone())?)?;
    v_pz = f__phasor_parts(v_z.clone(), f__cabs(v_z.clone())?)?;
    return Ok(Py::dict(vec![(py::add(&v_prefix_v.clone(), &Py::str("_real"))?, py::getitem(&v_pv.clone(), &Py::Int(0i64))?), (py::add(&v_prefix_v.clone(), &Py::str("_imag"))?, py::getitem(&v_pv.clone(), &Py::Int(1i64))?), (py::add(&v_prefix_v.clone(), &Py::str("_mag"))?, py::getitem(&v_pv.clone(), &Py::Int(2i64))?), (py::add(&v_prefix_v.clone(), &Py::str("_deg"))?, py::getitem(&v_pv.clone(), &Py::Int(3i64))?), (py::add(&v_prefix_z.clone(), &Py::str("_real"))?, py::getitem(&v_pz.clone(), &Py::Int(0i64))?), (py::add(&v_prefix_z.clone(), &Py::str("_imag"))?, py::getitem(&v_pz.clone(), &Py::Int(1i64))?), (py::add(&v_prefix_z.clone(), &Py::str("_mag"))?, py::getitem(&v_pz.clone(), &Py::Int(2i64))?), (py::add(&v_prefix_z.clone(), &Py::str("_deg"))?, py::getitem(&v_pz.clone(), &Py::Int(3i64))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__ac_port_residual(mut v_inputs: Py, mut v_vth: Py, mut v_zth: Py) -> R {
    let mut v__: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_dz: Py = Py::None;
    let mut v_els: Py = Py::None;
    let mut v_o1: Py = Py::None;
    let mut v_v1: Py = Py::None;
    let mut v_v2: Py = Py::None;
    let mut v_vmax: Py = Py::None;
    let mut v_w: Py = Py::None;
    let t620 = (Py::tuple(vec![py::getitem(&v_inputs.clone(), &Py::str("node_a"))?, py::getitem(&v_inputs.clone(), &Py::str("node_b"))?])).items()?;
    if t620.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t620[0].clone();
    v_b = t620[1].clone();
    v_els = f__ac_netlist(py::getitem(&v_inputs.clone(), &Py::str("netlist"))?)?;
    v_w = py::mul(&c__TWO_PI()?, &py::getitem(&v_inputs.clone(), &Py::str("f"))?)?;
    let t621 = (f__ac_mna(v_els.clone(), v_w.clone())?).items()?;
    if t621.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_v1 = t621[0].clone();
    v__ = t621[1].clone();
    let t622 = (f__ac_mna(py::add(&v_els.clone(), &Py::list(vec![Py::tuple(vec![Py::str("I"), Py::str("Itest_check"), v_b.clone(), v_a.clone(), Py::Float(1.0), Py::Float(0.0)])]))?, v_w.clone())?).items()?;
    if t622.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_v2 = t622[0].clone();
    v__ = t622[1].clone();
    v_vmax = py::minmax(&[{ let mut out = vec![]; for t623 in py::iter(&(py::method(&v_v1.clone(), "values", &[])?))? { let mut v_v: Py; 
v_v = t623;
out.push(f__cabs(v_v.clone())?); } Py::list(out) }], true)?;
    v_o1 = f__csnap(py::sub(&py::getitem(&v_v1.clone(), &v_a.clone())?, &py::getitem(&v_v1.clone(), &v_b.clone())?)?, v_vmax.clone())?;
    v_dz = py::sub(&py::sub(&py::getitem(&v_v2.clone(), &v_a.clone())?, &py::getitem(&v_v2.clone(), &v_b.clone())?)?, &py::sub(&py::getitem(&v_v1.clone(), &v_a.clone())?, &py::getitem(&v_v1.clone(), &v_b.clone())?)?)?;
    return Ok(py::add(&py::truediv(&f__cabs(py::sub(&v_o1.clone(), &v_vth.clone())?)?, &py::minmax(&[f__cabs(v_o1.clone())?, f__cabs(v_vth.clone())?, Py::Float(1e-300)], true)?)?, &py::truediv(&f__cabs(py::sub(&v_dz.clone(), &v_zth.clone())?)?, &py::minmax(&[f__cabs(v_dz.clone())?, f__cabs(v_zth.clone())?, Py::Float(1e-300)], true)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__ac_source_power(mut v_els: Py, mut v_volts: Py, mut v_cur: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_name: Py = Py::None;
    let mut v_ph: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_t: Py = Py::None;
    let mut v_val: Py = Py::None;
    v_s = py::complex(&[Py::Float(0.0), Py::Float(0.0)])?;
    for t244 in py::iter(&(v_els.clone()))? {
        let t249 = (t244).items()?;
        if t249.len() != 6 { return Err(PyErr::Value("unpack".into())); }
        v_t = t249[0].clone();
        v_name = t249[1].clone();
        v_a = t249[2].clone();
        v_b = t249[3].clone();
        v_val = t249[4].clone();
        v_ph = t249[5].clone();
        if ({ let t245 = v_t.clone(); Py::Bool({ let t246 = Py::str("V"); if py::eq(&t245, &t246) { true } else { false } }) }).truthy() {
            v_s = py::sub(&v_s.clone(), &py::mul(&f__rect_deg(v_val.clone(), v_ph.clone())?, &py::method(&py::getitem(&v_cur.clone(), &v_name.clone())?, "conjugate", &[])?)?)?;
        } else {
            if ({ let t247 = v_t.clone(); Py::Bool({ let t248 = Py::str("I"); if py::eq(&t247, &t248) { true } else { false } }) }).truthy() {
                v_s = py::add(&v_s.clone(), &py::mul(&py::sub(&py::getitem(&v_volts.clone(), &v_b.clone())?, &py::getitem(&v_volts.clone(), &v_a.clone())?)?, &py::method(&f__rect_deg(v_val.clone(), v_ph.clone())?, "conjugate", &[])?)?)?;
            }
        }
    }
    return Ok(v_s.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__ac_thevenin(mut v_netlist: Py, mut v_node_a: Py, mut v_node_b: Py, mut v_f: Py) -> R {
    let mut v__: Py = Py::None;
    let mut v_dead: Py = Py::None;
    let mut v_e: Py = Py::None;
    let mut v_els: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_nodes: Py = Py::None;
    let mut v_scale: Py = Py::None;
    let mut v_v2: Py = Py::None;
    let mut v_vmax: Py = Py::None;
    let mut v_volts: Py = Py::None;
    let mut v_vth: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_zmax: Py = Py::None;
    let mut v_zth: Py = Py::None;
    let _ = f__check_positive(Py::dict(vec![(Py::str("f"), v_f.clone())]))?;
    v_els = f__ac_netlist(v_netlist.clone())?;
    v_nodes = py::set(&{ let mut out = vec![]; for t498 in py::iter(&(v_els.clone()))? { let mut v_e: Py; 
v_e = t498;
for t499 in py::iter(&(Py::tuple(vec![py::getitem(&v_e.clone(), &Py::Int(2i64))?, py::getitem(&v_e.clone(), &Py::Int(3i64))?])))? { let mut v_n: Py; 
v_n = t499;
out.push(v_n.clone()); }} Py::list(out) })?;
    for t500 in py::iter(&(Py::tuple(vec![v_node_a.clone(), v_node_b.clone()])))? {
        v_n = t500;
        if ({ let t501 = v_n.clone(); Py::Bool({ let t502 = v_nodes.clone(); if !py::contains(&t502, &t501)? { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str(&["node ".to_string(), py::format(&v_n.clone(), "")?, " is not in the circuit".to_string()].concat()))));
        }
    }
    if ({ let t503 = v_node_a.clone(); Py::Bool({ let t504 = v_node_b.clone(); if py::eq(&t503, &t504) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the two terminals must be different nodes"))));
    }
    v_w = py::mul(&c__TWO_PI()?, &v_f.clone())?;
    let t590 = (f__ac_mna(v_els.clone(), v_w.clone())?).items()?;
    if t590.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_volts = t590[0].clone();
    v__ = t590[1].clone();
    v_vmax = py::minmax(&[{ let mut out = vec![]; for t591 in py::iter(&(py::method(&v_volts.clone(), "values", &[])?))? { let mut v_v: Py; 
v_v = t591;
out.push(f__cabs(v_v.clone())?); } Py::list(out) }], true)?;
    v_vth = f__csnap(py::sub(&py::getitem(&v_volts.clone(), &v_node_a.clone())?, &py::getitem(&v_volts.clone(), &v_node_b.clone())?)?, v_vmax.clone())?;
    v_dead = py::add(&{ let mut out = vec![]; for t592 in py::iter(&(v_els.clone()))? { let mut v_a: Py; let mut v_b: Py; let mut v_n: Py; let mut v_ph: Py; let mut v_t: Py; let mut v_v: Py; 
let t593 = (t592).items()?;
if t593.len() != 6 { return Err(PyErr::Value("unpack".into())); }
v_t = t593[0].clone();
v_n = t593[1].clone();
v_a = t593[2].clone();
v_b = t593[3].clone();
v_v = t593[4].clone();
v_ph = t593[5].clone();
if ({ let t594 = v_t.clone(); Py::Bool({ let t595 = Py::str("V"); if py::eq(&t594, &t595) { true } else { false } }) }).truthy() { out.push(Py::tuple(vec![v_t.clone(), v_n.clone(), v_a.clone(), v_b.clone(), Py::Float(0.0), Py::Float(0.0)])); }} Py::list(out) }, &{ let mut out = vec![]; for t596 in py::iter(&(v_els.clone()))? { let mut v_e: Py; 
v_e = t596;
if ({ let t597 = py::getitem(&v_e.clone(), &Py::Int(0i64))?; Py::Bool({ let t598 = Py::str("RLC"); if py::contains(&t598, &t597)? { true } else { false } }) }).truthy() { out.push(v_e.clone()); }} Py::list(out) })?;
    let _ = py::method(&v_dead.clone(), "append", &[Py::tuple(vec![Py::str("I"), Py::str("Itest"), v_node_b.clone(), v_node_a.clone(), Py::Float(1.0), Py::Float(0.0)])])?;
    let t599 = (f__ac_mna(v_dead.clone(), v_w.clone())?).items()?;
    if t599.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_v2 = t599[0].clone();
    v__ = t599[1].clone();
    v_zmax = py::minmax(&[{ let mut out = vec![]; for t600 in py::iter(&(py::method(&v_v2.clone(), "values", &[])?))? { let mut v_v: Py; 
v_v = t600;
out.push(f__cabs(v_v.clone())?); } Py::list(out) }], true)?;
    v_zth = f__csnap(py::sub(&py::getitem(&v_v2.clone(), &v_node_a.clone())?, &py::getitem(&v_v2.clone(), &v_node_b.clone())?)?, v_zmax.clone())?;
    v_scale = Py::Float(0.0);
    for t601 in py::iter(&(v_els.clone()))? {
        v_e = t601;
        if ({ let t602 = py::getitem(&v_e.clone(), &Py::Int(0i64))?; Py::Bool({ let t603 = Py::str("RLC"); if py::contains(&t603, &t602)? { true } else { false } }) }).truthy() {
            v_scale = py::minmax(&[v_scale.clone(), f__cabs(f__cdiv(py::complex(&[Py::Float(1.0), Py::Float(0.0)])?, f__ac_admittance(py::getitem(&v_e.clone(), &Py::Int(0i64))?, py::getitem(&v_e.clone(), &Py::Int(4i64))?, v_w.clone())?)?)?], true)?;
        }
    }
    if ({ let t604 = f__cabs(v_zth.clone())?; Py::Bool({ let t605 = py::mul(&Py::Float(1e-12), &v_scale.clone())?; if py::le(&t604, &t605)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("an ideal voltage source (or a short at this frequency) is across the terminals: the Thevenin impedance is 0"))));
    }
    return Ok(Py::tuple(vec![v_vth.clone(), v_zth.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__assignments(mut v_text: Py) -> R {
    let mut v_name: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_out = Py::dict(vec![]);
    for t803 in py::iter(&(ax_engine::pyre::module_call("findall", &Py::str("([A-Za-z_][A-Za-z0-9_]*)\\s*=\\s*([^,;\\s]+)"), &[v_text.clone()], 0)?))? {
        let t806 = (t803).items()?;
        if t806.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_name = t806[0].clone();
        v_v = t806[1].clone();
        if ({ let t804 = v_v.clone(); Py::Bool({ let t805 = Py::tuple(vec![Py::str("0"), Py::str("1")]); if !py::contains(&t805, &t804)? { true } else { false } }) }).truthy() {
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
    for t2344 in py::range_iter(&[(Py::Int(300i64)).i()?]) {
        v__ = t2344;
        v_m = py::mul(&Py::Float(0.5), &py::add(&v_a.clone(), &v_b.clone())?)?;
        if ({ let t2345 = v_m.clone(); Py::Bool({ let t2346 = Py::tuple(vec![v_a.clone(), v_b.clone()]); if py::contains(&t2346, &t2345)? { true } else { false } }) }).truthy() {
            break;
        }
        v_fm = py::call(&v_f.clone(), &[v_m.clone()])?;
        if ({ let t2347 = { let t2349 = v_fm.clone(); Py::Bool({ let t2350 = Py::Int(0i64); if py::gt(&t2349, &t2350)? { true } else { false } }) }; Py::Bool({ let t2348 = { let t2351 = v_fa.clone(); Py::Bool({ let t2352 = Py::Int(0i64); if py::gt(&t2351, &t2352)? { true } else { false } }) }; if py::eq(&t2347, &t2348) { true } else { false } }) }).truthy() {
            let t2353 = (Py::tuple(vec![v_m.clone(), v_fm.clone()])).items()?;
            if t2353.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_a = t2353[0].clone();
            v_fa = t2353[1].clone();
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
    if ({ let t3003 = v_t.clone(); Py::Bool({ let t3004 = Py::str("var"); if py::eq(&t3003, &t3004) { true } else { false } }) }).truthy() {
        return Ok(py::getitem(&v_env.clone(), &py::getitem(&v_tree.clone(), &Py::Int(1i64))?)?);
    }
    if ({ let t3005 = v_t.clone(); Py::Bool({ let t3006 = Py::str("const"); if py::eq(&t3005, &t3006) { true } else { false } }) }).truthy() {
        return Ok(py::getitem(&v_tree.clone(), &Py::Int(1i64))?);
    }
    if ({ let t3007 = v_t.clone(); Py::Bool({ let t3008 = Py::str("not"); if py::eq(&t3007, &t3008) { true } else { false } }) }).truthy() {
        return Ok(py::sub(&Py::Int(1i64), &f__bool_eval(py::getitem(&v_tree.clone(), &Py::Int(1i64))?, v_env.clone())?)?);
    }
    v_vals = { let mut out = vec![]; for t3009 in py::iter(&(py::getitem(&v_tree.clone(), &Py::Int(1i64))?))? { let mut v_x: Py; 
v_x = t3009;
out.push(f__bool_eval(v_x.clone(), v_env.clone())?); } Py::list(out) };
    if ({ let t3010 = v_t.clone(); Py::Bool({ let t3011 = Py::str("and"); if py::eq(&t3010, &t3011) { true } else { false } }) }).truthy() {
        return Ok(py::int(&Py::Bool((v_vals.clone()).items()?.iter().all(|x| x.truthy())))?);
    }
    if ({ let t3012 = v_t.clone(); Py::Bool({ let t3013 = Py::str("or"); if py::eq(&t3012, &t3013) { true } else { false } }) }).truthy() {
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
            return Ok((if ({ let t2966 = py::getitem(&v_pos.clone(), &Py::Int(0i64))?; Py::Bool({ let t2967 = py::len(&v_toks.clone())?; if py::lt(&t2966, &t2967)? { true } else { false } }) }).truthy() { py::getitem(&v_toks.clone(), &py::getitem(&v_pos.clone(), &Py::Int(0i64))?)? } else { Py::None }));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_take = { let v_peek = v_peek.clone(); let v_pos = v_pos.clone(); let v_take = v_take.clone(); let v_text = v_text.clone(); let v_toks = v_toks.clone(); let c_orx = c_orx.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            if ({ let t2968 = py::getitem(&v_pos.clone(), &Py::Int(0i64))?; Py::Bool({ let t2969 = py::len(&v_toks.clone())?; if py::ge(&t2968, &t2969)? { true } else { false } }) }).truthy() {
                return Err(py::value_error(py::to_str(&Py::str("the Boolean expression ends too early"))));
            }
            let t2970 = v_pos.clone(); let t2971 = Py::Int(0i64);
            py::setitem(&t2970, &t2971, py::iadd(&py::getitem(&t2970, &t2971)?, &Py::Int(1i64))?)?;
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
            while ({ let t2972 = py::call(&v_peek.clone(), &[])?; Py::Bool({ let t2973 = v_op.clone(); if py::eq(&t2972, &t2973) { true } else { false } }) }).truthy() {
                let _ = py::call(&v_take.clone(), &[])?;
                let _ = py::method(&v_items.clone(), "append", &[py::call(&v_sub.clone(), &[])?])?;
            }
            return Ok((if ({ let t2974 = py::len(&v_items.clone())?; Py::Bool({ let t2975 = Py::Int(1i64); if py::eq(&t2974, &t2975) { true } else { false } }) }).truthy() { py::getitem(&v_items.clone(), &Py::Int(0i64))? } else { Py::tuple(vec![py::method(&v_op.clone(), "lower", &[])?, v_items.clone()]) }));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
            return Ok(v_f.clone());
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    let rec2976 = py::rec_cell();
    v_unary = { let v_binary = v_binary.clone(); let v_peek = v_peek.clone(); let v_pos = v_pos.clone(); let v_take = v_take.clone(); let v_text = v_text.clone(); let v_toks = v_toks.clone(); let c_orx = c_orx.clone(); let rec2976 = rec2976.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let v_unary: Py = py::rec_get(&rec2976)?;
            let mut v_node: Py = Py::None;
            let mut v_t: Py = Py::None;
            if ({ let t2977 = py::call(&v_peek.clone(), &[])?; Py::Bool({ let t2978 = Py::str("NOT"); if py::eq(&t2977, &t2978) { true } else { false } }) }).truthy() {
                let _ = py::call(&v_take.clone(), &[])?;
                return Ok(Py::tuple(vec![Py::str("not"), py::call(&v_unary.clone(), &[])?]));
            }
            v_t = py::call(&v_take.clone(), &[])?;
            if ({ let t2979 = v_t.clone(); Py::Bool({ let t2980 = Py::str("("); if py::eq(&t2979, &t2980) { true } else { false } }) }).truthy() {
                v_node = py::call(&py::cell_get(&c_orx)?, &[])?;
                if ({ let t2981 = py::call(&v_take.clone(), &[])?; Py::Bool({ let t2982 = Py::str(")"); if py::ne(&t2981, &t2982) { true } else { false } }) }).truthy() {
                    return Err(py::value_error(py::to_str(&Py::str("unbalanced parentheses"))));
                }
            } else {
                if (Py::Bool(matches!(v_t.clone(), Py::Tuple(_)))).truthy() {
                    v_node = (if ({ let t2983 = py::getitem(&v_t.clone(), &Py::Int(0i64))?; Py::Bool({ let t2984 = Py::str("v"); if py::eq(&t2983, &t2984) { true } else { false } }) }).truthy() { Py::tuple(vec![Py::str("var"), py::getitem(&v_t.clone(), &Py::Int(1i64))?]) } else { Py::tuple(vec![Py::str("const"), py::getitem(&v_t.clone(), &Py::Int(1i64))?]) });
                } else {
                    return Err(py::value_error(py::to_str(&Py::str(&["unexpected ".to_string(), py::format(&Py::str(&py::repr(&v_t.clone())), "")?, " in the Boolean expression".to_string()].concat()))));
                }
            }
            while ({ let t2985 = py::call(&v_peek.clone(), &[])?; Py::Bool({ let t2986 = Py::str("'"); if py::eq(&t2985, &t2986) { true } else { false } }) }).truthy() {
                let _ = py::call(&v_take.clone(), &[])?;
                v_node = Py::tuple(vec![Py::str("not"), v_node.clone()]);
            }
            return Ok(v_node.clone());
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    py::rec_set(&rec2976, &v_unary);
    v_andx = py::call(&v_binary.clone(), &[Py::str("AND"), v_unary.clone()])?;
    v_xorx = py::call(&v_binary.clone(), &[Py::str("XOR"), v_andx.clone()])?;
    py::cell_set(&c_orx, py::call(&v_binary.clone(), &[Py::str("OR"), v_xorx.clone()])?);
    v_tree = py::call(&py::cell_get(&c_orx)?, &[])?;
    if ({ let t2987 = py::getitem(&v_pos.clone(), &Py::Int(0i64))?; Py::Bool({ let t2988 = py::len(&v_toks.clone())?; if py::ne(&t2987, &t2988) { true } else { false } }) }).truthy() {
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
    let t2959 = (Py::tuple(vec![Py::Int(0i64), Py::list(vec![])])).items()?;
    if t2959.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_pos = t2959[0].clone();
    v_out = t2959[1].clone();
    v_text = py::method(&v_text.clone(), "strip", &[])?;
    while ({ let t2964 = v_pos.clone(); Py::Bool({ let t2965 = py::len(&v_text.clone())?; if py::lt(&t2964, &t2965)? { true } else { false } }) }).truthy() {
        v_m = py::method(&c__BOOL_RE()?, "match", &[v_text.clone(), v_pos.clone()])?;
        if ({ let t2962 = py::not(&v_m.clone()); if t2962.truthy() { t2962 } else { { let t2960 = py::method(&v_m.clone(), "end", &[])?; Py::Bool({ let t2961 = v_pos.clone(); if py::eq(&t2960, &t2961) { true } else { false } }) } } }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str(&["cannot read the Boolean expression at '".to_string(), py::format(&py::slice(&v_text.clone(), Some((v_pos.clone()).i()?), Some((py::add(&v_pos.clone(), &Py::Int(12i64))?).i()?), None)?, "")?, "'".to_string()].concat()))));
        }
        let t2963 = (py::method(&v_m.clone(), "groups", &[])?).items()?;
        if t2963.len() != 4 { return Err(PyErr::Value("unpack".into())); }
        v_kw = t2963[0].clone();
        v_var = t2963[1].clone();
        v_const = t2963[2].clone();
        v_op = t2963[3].clone();
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
    v_acc = (if ({ let t2989 = v_acc.clone(); Py::Bool({ let t2990 = Py::None; if py::is(&t2989, &t2990) { true } else { false } }) }).truthy() { py::set(&Py::list(vec![]))? } else { v_acc.clone() });
    if ({ let t2991 = py::getitem(&v_tree.clone(), &Py::Int(0i64))?; Py::Bool({ let t2992 = Py::str("var"); if py::eq(&t2991, &t2992) { true } else { false } }) }).truthy() {
        let _ = py::method(&v_acc.clone(), "add", &[py::getitem(&v_tree.clone(), &Py::Int(1i64))?])?;
    } else {
        if ({ let t2993 = py::getitem(&v_tree.clone(), &Py::Int(0i64))?; Py::Bool({ let t2994 = Py::str("not"); if py::eq(&t2993, &t2994) { true } else { false } }) }).truthy() {
            let _ = f__bool_vars(py::getitem(&v_tree.clone(), &Py::Int(1i64))?, v_acc.clone())?;
        } else {
            if ({ let t2995 = py::getitem(&v_tree.clone(), &Py::Int(0i64))?; Py::Bool({ let t2996 = Py::tuple(vec![Py::str("and"), Py::str("or"), Py::str("xor")]); if py::contains(&t2996, &t2995)? { true } else { false } }) }).truthy() {
                for t2997 in py::iter(&(py::getitem(&v_tree.clone(), &Py::Int(1i64))?))? {
                    v_x = t2997;
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
fn f__cabs(mut v_z: Py) -> R {
    return Ok(py::hypot(&[py::real(&v_z.clone())?, py::imag(&v_z.clone())?])?);
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
    let t872 = (Py::tuple(vec![Py::list(vec![]), Py::list(vec![])])).items()?;
    if t872.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_real = t872[0].clone();
    v_cplx = t872[1].clone();
    for t873 in py::iter(&(v_roots.clone()))? {
        v_r = t873;
        v_r = py::complex(&[v_r.clone()])?;
        let _ = py::method(&(if ({ let t874 = py::abs(&py::imag(&v_r.clone())?)?; Py::Bool({ let t875 = py::mul(&v_tol.clone(), &py::minmax(&[Py::Float(1.0), py::abs(&v_r.clone())?], true)?)?; if py::le(&t874, &t875)? { true } else { false } }) }).truthy() { v_real.clone() } else { v_cplx.clone() }), "append", &[v_r.clone()])?;
    }
    v_out = { let mut out = vec![]; for t876 in py::iter(&(v_real.clone()))? { let mut v_x: Py; 
v_x = t876;
out.push(py::complex(&[py::real(&v_x.clone())?, Py::Float(0.0)])?); } Py::list(out) };
    v_ups = py::sorted_key(&{ let mut out = vec![]; for t877 in py::iter(&(v_cplx.clone()))? { let mut v_x: Py; 
v_x = t877;
if ({ let t878 = py::imag(&v_x.clone())?; Py::Bool({ let t879 = Py::Int(0i64); if py::gt(&t878, &t879)? { true } else { false } }) }).truthy() { out.push(v_x.clone()); }} Py::list(out) }, &{ let v_cplx = v_cplx.clone(); let v_out = v_out.clone(); let v_r = v_r.clone(); let v_real = v_real.clone(); let v_roots = v_roots.clone(); let v_tol = v_tol.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_x: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(Py::tuple(vec![py::real(&v_x.clone())?, py::imag(&v_x.clone())?]));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, (Py::Bool(false)).truthy())?;
    v_downs = { let mut out = vec![]; for t880 in py::iter(&(v_cplx.clone()))? { let mut v_x: Py; 
v_x = t880;
if ({ let t881 = py::imag(&v_x.clone())?; Py::Bool({ let t882 = Py::Int(0i64); if py::lt(&t881, &t882)? { true } else { false } }) }).truthy() { out.push(v_x.clone()); }} Py::list(out) };
    for t883 in py::iter(&(v_ups.clone()))? {
        v_u = t883;
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
        let t884 = (Py::tuple(vec![py::truediv(&py::add(&py::real(&v_u.clone())?, &py::real(&v_dn.clone())?)?, &Py::Int(2i64))?, py::truediv(&py::sub(&py::imag(&v_u.clone())?, &py::imag(&v_dn.clone())?)?, &Py::Int(2i64))?])).items()?;
        if t884.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_re_ = t884[0].clone();
        v_im = t884[1].clone();
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
    return Ok({ let mut out = vec![]; for t885 in py::iter(&(v_out.clone()))? { let mut v_x: Py; 
v_x = t885;
out.push((if ({ let t886 = py::imag(&v_x.clone())?; Py::Bool({ let t887 = Py::Int(0i64); if py::eq(&t886, &t887) { true } else { false } }) }).truthy() { py::real(&v_x.clone())? } else { v_x.clone() })); } Py::list(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__canonical_sop(mut v_n: Py, mut v_names: Py, mut v_on: Py) -> R {
    return Ok({ let t3023 = py::method(&Py::str(" + "), "join", &[{ let mut out = vec![]; for t3020 in py::iter(&(v_on.clone()))? { let mut v_m: Py; 
v_m = t3020;
out.push(f__term_text(Py::tuple(vec![v_m.clone(), Py::Int(0i64)]), v_names.clone())?); } Py::list(out) }])?; if t3023.truthy() { t3023 } else { Py::str("0") } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__cdiv(mut v_a: Py, mut v_b: Py) -> R {
    let mut v_d: Py = Py::None;
    v_d = py::add(&py::mul(&py::real(&v_b.clone())?, &py::real(&v_b.clone())?)?, &py::mul(&py::imag(&v_b.clone())?, &py::imag(&v_b.clone())?)?)?;
    if ({ let t570 = v_d.clone(); Py::Bool({ let t571 = Py::Int(0i64); if py::eq(&t570, &t571) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("division by a zero impedance"))));
    }
    return Ok(py::complex(&[py::truediv(&py::add(&py::mul(&py::real(&v_a.clone())?, &py::real(&v_b.clone())?)?, &py::mul(&py::imag(&v_a.clone())?, &py::imag(&v_b.clone())?)?)?, &v_d.clone())?, py::truediv(&py::sub(&py::mul(&py::imag(&v_a.clone())?, &py::real(&v_b.clone())?)?, &py::mul(&py::real(&v_a.clone())?, &py::imag(&v_b.clone())?)?)?, &v_d.clone())?])?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__cgauss(mut v_A: Py, mut v_b: Py) -> R {
    let mut v_M: Py = Py::None;
    let mut v_best: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_m2: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_row: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_scale: Py = Py::None;
    let mut v_x: Py = Py::None;
    v_n = py::len(&v_A.clone())?;
    v_M = Py::list(vec![]);
    for t554 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_i = t554;
        v_row = Py::list(vec![]);
        for t555 in py::range_iter(&[(v_n.clone()).i()?]) {
            v_j = t555;
            let _ = py::method(&v_row.clone(), "append", &[py::complex(&[py::getitem(&py::getitem(&v_A.clone(), &v_i.clone())?, &v_j.clone())?])?])?;
        }
        let _ = py::method(&v_row.clone(), "append", &[py::complex(&[py::getitem(&v_b.clone(), &v_i.clone())?])?])?;
        let _ = py::method(&v_M.clone(), "append", &[v_row.clone()])?;
    }
    v_scale = Py::Float(0.0);
    for t556 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_i = t556;
        for t557 in py::range_iter(&[(v_n.clone()).i()?]) {
            v_j = t557;
            v_scale = py::minmax(&[v_scale.clone(), py::add(&py::mul(&py::real(&py::getitem(&py::getitem(&v_M.clone(), &v_i.clone())?, &v_j.clone())?)?, &py::real(&py::getitem(&py::getitem(&v_M.clone(), &v_i.clone())?, &v_j.clone())?)?)?, &py::mul(&py::imag(&py::getitem(&py::getitem(&v_M.clone(), &v_i.clone())?, &v_j.clone())?)?, &py::imag(&py::getitem(&py::getitem(&v_M.clone(), &v_i.clone())?, &v_j.clone())?)?)?)?], true)?;
        }
    }
    if ({ let t558 = v_scale.clone(); Py::Bool({ let t559 = Py::Int(0i64); if py::eq(&t558, &t559) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the system is singular: the equations do not determine every unknown"))));
    }
    for t560 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_c = t560;
        v_p = v_c.clone();
        v_best = py::add(&py::mul(&py::real(&py::getitem(&py::getitem(&v_M.clone(), &v_c.clone())?, &v_c.clone())?)?, &py::real(&py::getitem(&py::getitem(&v_M.clone(), &v_c.clone())?, &v_c.clone())?)?)?, &py::mul(&py::imag(&py::getitem(&py::getitem(&v_M.clone(), &v_c.clone())?, &v_c.clone())?)?, &py::imag(&py::getitem(&py::getitem(&v_M.clone(), &v_c.clone())?, &v_c.clone())?)?)?)?;
        for t561 in py::range_iter(&[(py::add(&v_c.clone(), &Py::Int(1i64))?).i()?, (v_n.clone()).i()?]) {
            v_r = t561;
            v_m2 = py::add(&py::mul(&py::real(&py::getitem(&py::getitem(&v_M.clone(), &v_r.clone())?, &v_c.clone())?)?, &py::real(&py::getitem(&py::getitem(&v_M.clone(), &v_r.clone())?, &v_c.clone())?)?)?, &py::mul(&py::imag(&py::getitem(&py::getitem(&v_M.clone(), &v_r.clone())?, &v_c.clone())?)?, &py::imag(&py::getitem(&py::getitem(&v_M.clone(), &v_r.clone())?, &v_c.clone())?)?)?)?;
            if ({ let t562 = v_m2.clone(); Py::Bool({ let t563 = v_best.clone(); if py::gt(&t562, &t563)? { true } else { false } }) }).truthy() {
                v_p = v_r.clone();
                v_best = v_m2.clone();
            }
        }
        if ({ let t564 = v_best.clone(); Py::Bool({ let t565 = py::mul(&Py::Float(1e-24), &v_scale.clone())?; if py::le(&t564, &t565)? { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("the system is singular: the equations do not determine every unknown"))));
        }
        if ({ let t566 = v_p.clone(); Py::Bool({ let t567 = v_c.clone(); if py::ne(&t566, &t567) { true } else { false } }) }).truthy() {
            let t568 = (Py::tuple(vec![py::getitem(&v_M.clone(), &v_p.clone())?, py::getitem(&v_M.clone(), &v_c.clone())?])).items()?;
            if t568.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            py::setitem(&v_M.clone(), &v_c.clone(), t568[0].clone())?;
            py::setitem(&v_M.clone(), &v_p.clone(), t568[1].clone())?;
        }
        for t569 in py::range_iter(&[(py::add(&v_c.clone(), &Py::Int(1i64))?).i()?, (v_n.clone()).i()?]) {
            v_r = t569;
            v_f = f__cdiv(py::getitem(&py::getitem(&v_M.clone(), &v_r.clone())?, &v_c.clone())?, py::getitem(&py::getitem(&v_M.clone(), &v_c.clone())?, &v_c.clone())?)?;
            if ({ let t572 = v_f.clone(); Py::Bool({ let t573 = Py::Int(0i64); if py::ne(&t572, &t573) { true } else { false } }) }).truthy() {
                for t574 in py::range_iter(&[(v_c.clone()).i()?, (py::add(&v_n.clone(), &Py::Int(1i64))?).i()?]) {
                    v_k = t574;
                    py::setitem(&py::getitem(&v_M.clone(), &v_r.clone())?, &v_k.clone(), py::sub(&py::getitem(&py::getitem(&v_M.clone(), &v_r.clone())?, &v_k.clone())?, &py::mul(&v_f.clone(), &py::getitem(&py::getitem(&v_M.clone(), &v_c.clone())?, &v_k.clone())?)?)?)?;
                }
            }
        }
    }
    v_x = py::mul(&Py::list(vec![py::complex(&[Py::Float(0.0), Py::Float(0.0)])?]), &v_n.clone())?;
    for t575 in py::range_iter(&[(py::sub(&v_n.clone(), &Py::Int(1i64))?).i()?, (py::neg(&Py::Int(1i64))?).i()?, (py::neg(&Py::Int(1i64))?).i()?]) {
        v_r = t575;
        v_s = py::getitem(&py::getitem(&v_M.clone(), &v_r.clone())?, &v_n.clone())?;
        for t576 in py::range_iter(&[(py::add(&v_r.clone(), &Py::Int(1i64))?).i()?, (v_n.clone()).i()?]) {
            v_k = t576;
            v_s = py::sub(&v_s.clone(), &py::mul(&py::getitem(&py::getitem(&v_M.clone(), &v_r.clone())?, &v_k.clone())?, &py::getitem(&v_x.clone(), &v_k.clone())?)?)?;
        }
        py::setitem(&v_x.clone(), &v_r.clone(), f__cdiv(v_s.clone(), py::getitem(&py::getitem(&v_M.clone(), &v_r.clone())?, &v_r.clone())?)?)?;
    }
    return Ok(v_x.clone());
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
    v_I = { let mut out = vec![]; for t2596 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t2596;
out.push({ let mut out = vec![]; for t2597 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t2597;
out.push(py::float(&{ let t2598 = v_i.clone(); Py::Bool({ let t2599 = v_j.clone(); if py::eq(&t2598, &t2599) { true } else { false } }) })?); } Py::list(out) }); } Py::list(out) };
    v_M = { let mut out = vec![]; for t2600 in py::iter(&(v_I.clone()))? { let mut v_row: Py; 
v_row = t2600;
out.push(py::slice(&v_row.clone(), None, None, None)?); } Py::list(out) };
    let t2601 = (Py::tuple(vec![Py::list(vec![Py::Float(1.0)]), Py::list(vec![])])).items()?;
    if t2601.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_coeffs = t2601[0].clone();
    v_Ns = t2601[1].clone();
    for t2602 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&v_n.clone(), &Py::Int(1i64))?).i()?]) {
        v_k = t2602;
        let _ = py::method(&v_Ns.clone(), "append", &[v_M.clone()])?;
        v_AM = { let mut out = vec![]; for t2603 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t2603;
out.push({ let mut out = vec![]; for t2604 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t2604;
out.push(py::sum(&{ let mut out = vec![]; for t2605 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_t: Py; 
v_t = t2605;
out.push(py::mul(&py::getitem(&py::getitem(&v_A.clone(), &v_i.clone())?, &v_t.clone())?, &py::getitem(&py::getitem(&v_M.clone(), &v_t.clone())?, &v_j.clone())?)?); } Py::list(out) }, None)?); } Py::list(out) }); } Py::list(out) };
        v_c = py::truediv(&py::neg(&py::sum(&{ let mut out = vec![]; for t2606 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t2606;
out.push(py::getitem(&py::getitem(&v_AM.clone(), &v_i.clone())?, &v_i.clone())?); } Py::list(out) }, None)?)?, &v_k.clone())?;
        let _ = py::method(&v_coeffs.clone(), "append", &[v_c.clone()])?;
        v_M = { let mut out = vec![]; for t2607 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t2607;
out.push({ let mut out = vec![]; for t2608 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t2608;
out.push(py::add(&py::getitem(&py::getitem(&v_AM.clone(), &v_i.clone())?, &v_j.clone())?, &py::mul(&v_c.clone(), &py::getitem(&py::getitem(&v_I.clone(), &v_i.clone())?, &v_j.clone())?)?)?); } Py::list(out) }); } Py::list(out) };
    }
    return Ok(Py::tuple(vec![v_coeffs.clone(), v_Ns.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__check_phase(mut v_phi: Py) -> R {
    if (py::not(&{ let t2575 = py::math1("isfinite", &v_phi.clone())?; if !t2575.truthy() { t2575 } else { { let t2573 = py::abs(&v_phi.clone())?; Py::Bool({ let t2574 = py::mul(&Py::Float(2.0), &Py::Float(std::f64::consts::PI))?; if py::le(&t2573, &t2574)? { true } else { false } }) } } })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the phase angle must be between -2 pi and 2 pi rad"))));
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__check_positive(mut v_kw: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    for t3160 in py::iter(&(py::method(&v_kw.clone(), "items", &[])?))? {
        let t3164 = (t3160).items()?;
        if t3164.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3164[0].clone();
        v_v = t3164[1].clone();
        if (py::not(&{ let t3163 = py::math1("isfinite", &v_v.clone())?; if !t3163.truthy() { t3163 } else { { let t3161 = v_v.clone(); Py::Bool({ let t3162 = Py::Int(0i64); if py::gt(&t3161, &t3162)? { true } else { false } }) } } })).truthy() {
            return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_k.clone(), "")?, " must be positive".to_string()].concat()))));
        }
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__check_tf(mut v_num: Py, mut v_den: Py) -> R {
    let mut v_x: Py = Py::None;
    let t2667 = (Py::tuple(vec![f__ptrim(v_num.clone())?, f__ptrim(v_den.clone())?])).items()?;
    if t2667.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_num = t2667[0].clone();
    v_den = t2667[1].clone();
    if (Py::Bool(({ let mut out = vec![]; for t2668 in py::iter(&(v_den.clone()))? { let mut v_x: Py; 
v_x = t2668;
out.push({ let t2669 = v_x.clone(); Py::Bool({ let t2670 = Py::Int(0i64); if py::eq(&t2669, &t2670) { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy()))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the denominator must not be zero"))));
    }
    if (Py::Bool(({ let mut out = vec![]; for t2671 in py::iter(&(v_num.clone()))? { let mut v_x: Py; 
v_x = t2671;
out.push({ let t2672 = v_x.clone(); Py::Bool({ let t2673 = Py::Int(0i64); if py::eq(&t2672, &t2673) { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy()))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the numerator must not be zero"))));
    }
    if ({ let t2674 = py::len(&v_num.clone())?; Py::Bool({ let t2675 = py::len(&v_den.clone())?; if py::gt(&t2674, &t2675)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the transfer function must be proper (numerator degree <= denominator degree)"))));
    }
    for t2676 in py::iter(&(py::add(&v_num.clone(), &v_den.clone())?))? {
        v_x = t2676;
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
    if ({ let t656 = py::method(&py::slice(&v_v.clone(), None, Some((Py::Int(2i64)).i()?), None)?, "lower", &[])?; Py::Bool({ let t657 = v_pre.clone(); if py::contains(&t657, &t656)? { true } else { false } }) }).truthy() {
        if ({ let t658 = py::getitem(&v_pre.clone(), &py::method(&py::slice(&v_v.clone(), None, Some((Py::Int(2i64)).i()?), None)?, "lower", &[])?)?; Py::Bool({ let t659 = v_base.clone(); if py::ne(&t658, &t659) { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str(&["the prefix ".to_string(), py::format(&py::slice(&v_v.clone(), None, Some((Py::Int(2i64)).i()?), None)?, "")?, " does not match base ".to_string(), py::format(&v_base.clone(), "")?].concat()))));
        }
        v_v = py::slice(&v_v.clone(), Some((Py::Int(2i64)).i()?), None, None)?;
    }
    if ({ let t663 = py::not(&v_v.clone()); if t663.truthy() { t663 } else { Py::Bool(({ let mut out = vec![]; for t660 in py::iter(&(py::method(&v_v.clone(), "lower", &[])?))? { let mut v_c: Py; 
v_c = t660;
out.push({ let t661 = v_c.clone(); Py::Bool({ let t662 = py::slice(&Py::str("0123456789abcdef"), None, Some((v_base.clone()).i()?), None)?; if !py::contains(&t662, &t661)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())) } }).truthy() {
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
    let t3040 = (v_imp.clone()).items()?;
    if t3040.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_v = t3040[0].clone();
    v_mask = t3040[1].clone();
    return Ok({ let t3041 = py::bitand(&v_m.clone(), &py::invert(&v_mask.clone())?)?; Py::Bool({ let t3042 = v_v.clone(); if py::eq(&t3041, &t3042) { true } else { false } }) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__csnap(mut v_z: Py, mut v_scale: Py) -> R {
    let mut v_im: Py = Py::None;
    let mut v_re_: Py = Py::None;
    let t583 = (Py::tuple(vec![py::real(&v_z.clone())?, py::imag(&v_z.clone())?])).items()?;
    if t583.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_re_ = t583[0].clone();
    v_im = t583[1].clone();
    if ({ let t584 = py::abs(&v_re_.clone())?; Py::Bool({ let t585 = py::mul(&Py::Float(1e-12), &v_scale.clone())?; if py::le(&t584, &t585)? { true } else { false } }) }).truthy() {
        v_re_ = Py::Float(0.0);
    }
    if ({ let t586 = py::abs(&v_im.clone())?; Py::Bool({ let t587 = py::mul(&Py::Float(1e-12), &v_scale.clone())?; if py::le(&t586, &t587)? { true } else { false } }) }).truthy() {
        v_im = Py::Float(0.0);
    }
    return Ok(py::complex(&[py::add(&v_re_.clone(), &Py::Float(0.0))?, py::add(&v_im.clone(), &Py::Float(0.0))?])?);
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
    let t979 = (Py::tuple(vec![Py::list(vec![]), Py::list((v_B.clone()).items()?)])).items()?;
    if t979.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_cols = t979[0].clone();
    v_v = t979[1].clone();
    for t980 in py::range_iter(&[(v_n.clone()).i()?]) {
        v__ = t980;
        let _ = py::method(&v_cols.clone(), "append", &[v_v.clone()])?;
        v_v = { let mut out = vec![]; for t981 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t981;
out.push(py::sum(&{ let mut out = vec![]; for t982 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t982;
out.push(py::mul(&py::getitem(&py::getitem(&v_A.clone(), &v_i.clone())?, &v_j.clone())?, &py::getitem(&v_v.clone(), &v_j.clone())?)?); } Py::list(out) }, None)?); } Py::list(out) };
    }
    let t983 = (Py::tuple(vec![Py::list(vec![]), Py::list((v_C.clone()).items()?)])).items()?;
    if t983.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_rows = t983[0].clone();
    v_w = t983[1].clone();
    for t984 in py::range_iter(&[(v_n.clone()).i()?]) {
        v__ = t984;
        let _ = py::method(&v_rows.clone(), "append", &[v_w.clone()])?;
        v_w = { let mut out = vec![]; for t985 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t985;
out.push(py::sum(&{ let mut out = vec![]; for t986 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t986;
out.push(py::mul(&py::getitem(&v_w.clone(), &v_i.clone())?, &py::getitem(&py::getitem(&v_A.clone(), &v_i.clone())?, &v_j.clone())?)?); } Py::list(out) }, None)?); } Py::list(out) };
    }
    return Ok(Py::tuple(vec![{ let mut out = vec![]; for t987 in py::iter(&(py::zip(&{ let mut v: Vec<Py> = vec![]; v.extend((v_cols.clone()).items()?); v })?))? { let mut v_x: Py; 
v_x = t987;
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
    let t2043 = (Py::tuple(vec![f__iw(v_num.clone())?, f__iw(v_den.clone())?])).items()?;
    if t2043.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_n_iw = t2043[0].clone();
    v_d_iw = t2043[1].clone();
    v_nn = { let mut out = vec![]; for t2051 in py::iter(&(f__pmul(v_n_iw.clone(), { let mut out = vec![]; for t2050 in py::iter(&(v_n_iw.clone()))? { let mut v_y: Py; 
v_y = t2050;
out.push(py::method(&v_y.clone(), "conjugate", &[])?); } Py::list(out) })?))? { let mut v_x: Py; 
v_x = t2051;
out.push(py::real(&v_x.clone())?); } Py::list(out) };
    v_dd = { let mut out = vec![]; for t2053 in py::iter(&(f__pmul(v_d_iw.clone(), { let mut out = vec![]; for t2052 in py::iter(&(v_d_iw.clone()))? { let mut v_y: Py; 
v_y = t2052;
out.push(py::method(&v_y.clone(), "conjugate", &[])?); } Py::list(out) })?))? { let mut v_x: Py; 
v_x = t2053;
out.push(py::real(&v_x.clone())?); } Py::list(out) };
    v_P = f__ptrim(f__padd(v_nn.clone(), { let mut out = vec![]; for t2057 in py::iter(&(v_dd.clone()))? { let mut v_x: Py; 
v_x = t2057;
out.push(py::neg(&v_x.clone())?); } Py::list(out) })?)?;
    v_ws = f__positive_real_roots(v_P.clone(), Py::Float(1.0))?;
    return Ok({ let mut out = vec![]; for t2104 in py::iter(&(v_ws.clone()))? { let mut v_w: Py; 
v_w = t2104;
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
    if ({ let t2886 = { let t2878 = v_n.clone(); Py::Bool({ let t2879 = Py::Int(0i64); if py::eq(&t2878, &t2879) { true } else { false } }) }; if t2886.truthy() { t2886 } else { { let t2885 = Py::Bool(({ let mut out = vec![]; for t2880 in py::iter(&(v_A.clone()))? { let mut v_r: Py; 
v_r = t2880;
out.push({ let t2881 = py::len(&v_r.clone())?; Py::Bool({ let t2882 = v_n.clone(); if py::ne(&t2881, &t2882) { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())); if t2885.truthy() { t2885 } else { { let t2883 = py::len(&v_b.clone())?; Py::Bool({ let t2884 = v_n.clone(); if py::ne(&t2883, &t2884) { true } else { false } }) } } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the matrix must be square and the vector must have one entry per row"))));
    }
    v_M = { let mut out = vec![]; for t2887 in py::iter(&(py::enumerate(&v_A.clone(), 0)?))? { let mut v_i: Py; let mut v_row: Py; 
let t2888 = (t2887).items()?;
if t2888.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_i = t2888[0].clone();
v_row = t2888[1].clone();
out.push(py::add(&{ let mut out = vec![]; for t2889 in py::iter(&(v_row.clone()))? { let mut v_x: Py; 
v_x = t2889;
out.push((if (Py::Bool(matches!(v_x.clone(), Py::Complex(..)))).truthy() { py::complex(&[v_x.clone()])? } else { py::float(&v_x.clone())? })); } Py::list(out) }, &Py::list(vec![py::getitem(&v_b.clone(), &v_i.clone())?]))?); } Py::list(out) };
    v_scale = { let t2892 = py::minmax(&[{ let mut out = vec![]; for t2890 in py::iter(&(v_A.clone()))? { let mut v_row: Py; 
v_row = t2890;
for t2891 in py::iter(&(v_row.clone()))? { let mut v_x: Py; 
v_x = t2891;
out.push(py::abs(&v_x.clone())?); }} Py::list(out) }], true)?; if t2892.truthy() { t2892 } else { Py::Float(1.0) } };
    for t2893 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_c = t2893;
        v_p = py::minmax_key(&[Py::list((py::range_iter(&[(v_c.clone()).i()?, (v_n.clone()).i()?])).into_iter().collect::<Vec<Py>>())], &{ let v_A = v_A.clone(); let v_M = v_M.clone(); let v_b = v_b.clone(); let v_c = v_c.clone(); let v_n = v_n.clone(); let v_scale = v_scale.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_r: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::abs(&py::getitem(&py::getitem(&v_M.clone(), &v_r.clone())?, &v_c.clone())?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, true)?;
        if ({ let t2894 = py::abs(&py::getitem(&py::getitem(&v_M.clone(), &v_p.clone())?, &v_c.clone())?)?; Py::Bool({ let t2895 = py::mul(&Py::Float(1e-12), &v_scale.clone())?; if py::le(&t2894, &t2895)? { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("the system is singular: the equations do not determine every unknown"))));
        }
        let t2896 = (Py::tuple(vec![py::getitem(&v_M.clone(), &v_p.clone())?, py::getitem(&v_M.clone(), &v_c.clone())?])).items()?;
        if t2896.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        py::setitem(&v_M.clone(), &v_c.clone(), t2896[0].clone())?;
        py::setitem(&v_M.clone(), &v_p.clone(), t2896[1].clone())?;
        for t2897 in py::range_iter(&[(py::add(&v_c.clone(), &Py::Int(1i64))?).i()?, (v_n.clone()).i()?]) {
            v_r = t2897;
            v_f = py::truediv(&py::getitem(&py::getitem(&v_M.clone(), &v_r.clone())?, &v_c.clone())?, &py::getitem(&py::getitem(&v_M.clone(), &v_c.clone())?, &v_c.clone())?)?;
            if ({ let t2898 = v_f.clone(); Py::Bool({ let t2899 = Py::Int(0i64); if py::ne(&t2898, &t2899) { true } else { false } }) }).truthy() {
                for t2900 in py::range_iter(&[(v_c.clone()).i()?, (py::add(&v_n.clone(), &Py::Int(1i64))?).i()?]) {
                    v_k = t2900;
                    let t2901 = py::getitem(&v_M.clone(), &v_r.clone())?; let t2902 = v_k.clone();
                    py::setitem(&t2901, &t2902, py::sub(&py::getitem(&t2901, &t2902)?, &py::mul(&v_f.clone(), &py::getitem(&py::getitem(&v_M.clone(), &v_c.clone())?, &v_k.clone())?)?)?)?;
                }
            }
        }
    }
    v_x = py::mul(&Py::list(vec![Py::Float(0.0)]), &v_n.clone())?;
    for t2903 in py::iter(&(py::reversed(&Py::list((py::range_iter(&[(v_n.clone()).i()?])).into_iter().collect::<Vec<Py>>()))?))? {
        v_r = t2903;
        py::setitem(&v_x.clone(), &v_r.clone(), py::truediv(&py::sub(&py::getitem(&py::getitem(&v_M.clone(), &v_r.clone())?, &v_n.clone())?, &py::sum(&{ let mut out = vec![]; for t2904 in py::range_iter(&[(py::add(&v_r.clone(), &Py::Int(1i64))?).i()?, (v_n.clone()).i()?]) { let mut v_k: Py; 
v_k = t2904;
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
    return Ok({ let mut out = vec![]; for t2041 in py::iter(&(py::enumerate(&v_c.clone(), 0)?))? { let mut v_i: Py; let mut v_x: Py; 
let t2042 = (t2041).items()?;
if t2042.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_i = t2042[0].clone();
v_x = t2042[1].clone();
out.push(py::mul(&v_x.clone(), &py::pow(&Py::Complex(0.0, 1.0), &py::sub(&v_n.clone(), &v_i.clone())?)?)?); } Py::list(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__lead(mut v_z: Py, mut v_p: Py) -> R {
    let mut v_phi: Py = Py::None;
    let mut v_wm: Py = Py::None;
    let _ = f__check_positive(Py::dict(vec![(Py::str("z"), v_z.clone()), (Py::str("p"), v_p.clone())]))?;
    if (py::not(&{ let t1442 = v_p.clone(); Py::Bool({ let t1443 = v_z.clone(); if py::gt(&t1442, &t1443)? { true } else { false } }) })).truthy() {
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
    let t3024 = (Py::tuple(vec![py::sorted(&py::set(&v_on.clone())?, (Py::Bool(false)).truthy())?, py::sorted(&py::sub(&py::set(&v_dc.clone())?, &py::set(&v_on.clone())?)?, (Py::Bool(false)).truthy())?])).items()?;
    if t3024.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_on = t3024[0].clone();
    v_dc = t3024[1].clone();
    if (py::not(&v_on.clone())).truthy() {
        return Ok(Py::list(vec![]));
    }
    v_primes = f__prime_implicants(v_n.clone(), py::bitor(&py::set(&v_on.clone())?, &py::set(&v_dc.clone())?)?)?;
    v_essential = Py::list(vec![]);
    for t3038 in py::iter(&(v_on.clone()))? {
        v_m = t3038;
        v_cov = { let mut out = vec![]; for t3039 in py::iter(&(v_primes.clone()))? { let mut v_p: Py; 
v_p = t3039;
if (f__covers(v_p.clone(), v_m.clone())?).truthy() { out.push(v_p.clone()); }} Py::list(out) };
        if ({ let t3047 = { let t3043 = py::len(&v_cov.clone())?; Py::Bool({ let t3044 = Py::Int(1i64); if py::eq(&t3043, &t3044) { true } else { false } }) }; if !t3047.truthy() { t3047 } else { { let t3045 = py::getitem(&v_cov.clone(), &Py::Int(0i64))?; Py::Bool({ let t3046 = v_essential.clone(); if !py::contains(&t3046, &t3045)? { true } else { false } }) } } }).truthy() {
            let _ = py::method(&v_essential.clone(), "append", &[py::getitem(&v_cov.clone(), &Py::Int(0i64))?])?;
        }
    }
    v_left = { let mut out = vec![]; for t3048 in py::iter(&(v_on.clone()))? { let mut v_m: Py; 
v_m = t3048;
if (py::not(&Py::Bool(({ let mut out = vec![]; for t3049 in py::iter(&(v_essential.clone()))? { let mut v_p: Py; 
v_p = t3049;
out.push(f__covers(v_p.clone(), v_m.clone())?); } Py::list(out) }).items()?.iter().any(|x| x.truthy())))).truthy() { out.push(v_m.clone()); }} Py::list(out) };
    v_rest = { let mut out = vec![]; for t3050 in py::iter(&(v_primes.clone()))? { let mut v_p: Py; 
v_p = t3050;
if ({ let t3051 = v_p.clone(); Py::Bool({ let t3052 = v_essential.clone(); if !py::contains(&t3052, &t3051)? { true } else { false } }) }).truthy() { out.push(v_p.clone()); }} Py::list(out) };
    v_best = Py::None;
    if (v_left.clone()).truthy() {
        for t3053 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&py::len(&v_rest.clone())?, &Py::Int(1i64))?).i()?]) {
            v_k = t3053;
            for t3054 in py::iter(&(py::itertools("combinations", &[v_rest.clone(), v_k.clone()])?))? {
                v_combo = t3054;
                if (Py::Bool(({ let mut out = vec![]; for t3055 in py::iter(&(v_left.clone()))? { let mut v_m: Py; 
v_m = t3055;
out.push(Py::Bool(({ let mut out = vec![]; for t3056 in py::iter(&(v_combo.clone()))? { let mut v_p: Py; 
v_p = t3056;
out.push(f__covers(v_p.clone(), v_m.clone())?); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))); } Py::list(out) }).items()?.iter().all(|x| x.truthy()))).truthy() {
                    v_key = Py::tuple(vec![v_k.clone(), py::sum(&{ let mut out = vec![]; for t3057 in py::iter(&(v_combo.clone()))? { let mut v_p: Py; 
v_p = t3057;
out.push(f__literals(v_p.clone(), v_n.clone())?); } Py::list(out) }, None)?, py::sorted(&v_combo.clone(), (Py::Bool(false)).truthy())?]);
                    if ({ let t3062 = { let t3058 = v_best.clone(); Py::Bool({ let t3059 = Py::None; if py::is(&t3058, &t3059) { true } else { false } }) }; if t3062.truthy() { t3062 } else { { let t3060 = v_key.clone(); Py::Bool({ let t3061 = v_best.clone(); if py::lt(&t3060, &t3061)? { true } else { false } }) } } }).truthy() {
                        v_best = v_key.clone();
                    }
                }
            }
            if ({ let t3063 = v_best.clone(); Py::Bool({ let t3064 = Py::None; if !py::is(&t3063, &t3064) { true } else { false } }) }).truthy() {
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
    v_nodes = py::sorted_key(&py::set(&{ let mut out = vec![]; for t2828 in py::iter(&(v_els.clone()))? { let mut v_e: Py; 
v_e = t2828;
for t2829 in py::iter(&(Py::tuple(vec![py::getitem(&v_e.clone(), &Py::Int(2i64))?, py::getitem(&v_e.clone(), &Py::Int(3i64))?])))? { let mut v_n: Py; 
v_n = t2829;
if ({ let t2830 = v_n.clone(); Py::Bool({ let t2831 = Py::str("0"); if py::ne(&t2830, &t2831) { true } else { false } }) }).truthy() { out.push(v_n.clone()); }}} Py::list(out) })?, &py::func(|a: &[Py]| -> R { f__node_key(a.get(0).cloned().unwrap_or(Py::None)) }), (Py::Bool(false)).truthy())?;
    v_idx = { let mut out = vec![]; for t2832 in py::iter(&(py::enumerate(&v_nodes.clone(), 0)?))? { let mut v_i: Py; let mut v_n: Py; 
let t2833 = (t2832).items()?;
if t2833.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_i = t2833[0].clone();
v_n = t2833[1].clone();
out.push((v_n.clone(), v_i.clone())); } Py::dict(out) };
    v_vs = { let mut out = vec![]; for t2834 in py::iter(&(v_els.clone()))? { let mut v_e: Py; 
v_e = t2834;
if ({ let t2835 = py::getitem(&v_e.clone(), &Py::Int(0i64))?; Py::Bool({ let t2836 = Py::str("V"); if py::eq(&t2835, &t2836) { true } else { false } }) }).truthy() { out.push(v_e.clone()); }} Py::list(out) };
    v_N = py::add(&py::len(&v_nodes.clone())?, &py::len(&v_vs.clone())?)?;
    v_A = { let mut out = vec![]; for t2837 in py::range_iter(&[(v_N.clone()).i()?]) { let mut v__: Py; 
v__ = t2837;
out.push(py::mul(&Py::list(vec![Py::Float(0.0)]), &v_N.clone())?); } Py::list(out) };
    v_z = py::mul(&Py::list(vec![Py::Float(0.0)]), &v_N.clone())?;
    for t2838 in py::iter(&(v_els.clone()))? {
        let t2861 = (t2838).items()?;
        if t2861.len() != 5 { return Err(PyErr::Value("unpack".into())); }
        v_t = t2861[0].clone();
        v_name = t2861[1].clone();
        v_a = t2861[2].clone();
        v_b = t2861[3].clone();
        v_val = t2861[4].clone();
        let t2839 = (Py::tuple(vec![py::method(&v_idx.clone(), "get", &[v_a.clone()])?, py::method(&v_idx.clone(), "get", &[v_b.clone()])?])).items()?;
        if t2839.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_ia = t2839[0].clone();
        v_ib = t2839[1].clone();
        if ({ let t2840 = v_t.clone(); Py::Bool({ let t2841 = Py::str("R"); if py::eq(&t2840, &t2841) { true } else { false } }) }).truthy() {
            v_g = py::truediv(&Py::Float(1.0), &v_val.clone())?;
            for t2842 in py::iter(&(Py::tuple(vec![Py::tuple(vec![v_ia.clone(), v_ia.clone(), v_g.clone()]), Py::tuple(vec![v_ib.clone(), v_ib.clone(), v_g.clone()]), Py::tuple(vec![v_ia.clone(), v_ib.clone(), py::neg(&v_g.clone())?]), Py::tuple(vec![v_ib.clone(), v_ia.clone(), py::neg(&v_g.clone())?])])))? {
                let t2850 = (t2842).items()?;
                if t2850.len() != 3 { return Err(PyErr::Value("unpack".into())); }
                v_i = t2850[0].clone();
                v_j = t2850[1].clone();
                v_s = t2850[2].clone();
                if ({ let t2847 = { let t2843 = v_i.clone(); Py::Bool({ let t2844 = Py::None; if !py::is(&t2843, &t2844) { true } else { false } }) }; if !t2847.truthy() { t2847 } else { { let t2845 = v_j.clone(); Py::Bool({ let t2846 = Py::None; if !py::is(&t2845, &t2846) { true } else { false } }) } } }).truthy() {
                    let t2848 = py::getitem(&v_A.clone(), &v_i.clone())?; let t2849 = v_j.clone();
                    py::setitem(&t2848, &t2849, py::iadd(&py::getitem(&t2848, &t2849)?, &v_s.clone())?)?;
                }
            }
        } else {
            if ({ let t2851 = v_t.clone(); Py::Bool({ let t2852 = Py::str("I"); if py::eq(&t2851, &t2852) { true } else { false } }) }).truthy() {
                if ({ let t2853 = v_ia.clone(); Py::Bool({ let t2854 = Py::None; if !py::is(&t2853, &t2854) { true } else { false } }) }).truthy() {
                    let t2855 = v_z.clone(); let t2856 = v_ia.clone();
                    py::setitem(&t2855, &t2856, py::sub(&py::getitem(&t2855, &t2856)?, &v_val.clone())?)?;
                }
                if ({ let t2857 = v_ib.clone(); Py::Bool({ let t2858 = Py::None; if !py::is(&t2857, &t2858) { true } else { false } }) }).truthy() {
                    let t2859 = v_z.clone(); let t2860 = v_ib.clone();
                    py::setitem(&t2859, &t2860, py::iadd(&py::getitem(&t2859, &t2860)?, &v_val.clone())?)?;
                }
            }
        }
    }
    for t2862 in py::iter(&(py::enumerate(&v_vs.clone(), 0)?))? {
        let t2876 = (t2862).items()?;
        if t2876.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2876[0].clone();
        let t2877 = (t2876[1].clone()).items()?;
        if t2877.len() != 5 { return Err(PyErr::Value("unpack".into())); }
        v_t = t2877[0].clone();
        v_name = t2877[1].clone();
        v_a = t2877[2].clone();
        v_b = t2877[3].clone();
        v_val = t2877[4].clone();
        v_r = py::add(&py::len(&v_nodes.clone())?, &v_k.clone())?;
        let t2863 = (Py::tuple(vec![py::method(&v_idx.clone(), "get", &[v_a.clone()])?, py::method(&v_idx.clone(), "get", &[v_b.clone()])?])).items()?;
        if t2863.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_ia = t2863[0].clone();
        v_ib = t2863[1].clone();
        if ({ let t2864 = v_ia.clone(); Py::Bool({ let t2865 = Py::None; if !py::is(&t2864, &t2865) { true } else { false } }) }).truthy() {
            let t2866 = py::getitem(&v_A.clone(), &v_ia.clone())?; let t2867 = v_r.clone();
            py::setitem(&t2866, &t2867, py::iadd(&py::getitem(&t2866, &t2867)?, &Py::Float(1.0))?)?;
            let t2868 = py::getitem(&v_A.clone(), &v_r.clone())?; let t2869 = v_ia.clone();
            py::setitem(&t2868, &t2869, py::iadd(&py::getitem(&t2868, &t2869)?, &Py::Float(1.0))?)?;
        }
        if ({ let t2870 = v_ib.clone(); Py::Bool({ let t2871 = Py::None; if !py::is(&t2870, &t2871) { true } else { false } }) }).truthy() {
            let t2872 = py::getitem(&v_A.clone(), &v_ib.clone())?; let t2873 = v_r.clone();
            py::setitem(&t2872, &t2873, py::sub(&py::getitem(&t2872, &t2873)?, &Py::Float(1.0))?)?;
            let t2874 = py::getitem(&v_A.clone(), &v_r.clone())?; let t2875 = v_ib.clone();
            py::setitem(&t2874, &t2875, py::sub(&py::getitem(&t2874, &t2875)?, &Py::Float(1.0))?)?;
        }
        py::setitem(&v_z.clone(), &v_r.clone(), v_val.clone())?;
    }
    v_x = f__gauss(v_A.clone(), v_z.clone())?;
    v_vmax = { let t = { let mut out = vec![]; for t2905 in py::iter(&(v_nodes.clone()))? { let mut v_n: Py; 
v_n = t2905;
out.push(py::abs(&py::getitem(&v_x.clone(), &py::getitem(&v_idx.clone(), &v_n.clone())?)?)?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } };
    v_imax = { let t = { let mut out = vec![]; for t2906 in py::range_iter(&[(py::len(&v_vs.clone())?).i()?]) { let mut v_k: Py; 
v_k = t2906;
out.push(py::abs(&py::getitem(&v_x.clone(), &py::add(&py::len(&v_nodes.clone())?, &v_k.clone())?)?)?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } };
    v_snap = { let v_A = v_A.clone(); let v_N = v_N.clone(); let v_a = v_a.clone(); let v_b = v_b.clone(); let v_els = v_els.clone(); let v_g = v_g.clone(); let v_i = v_i.clone(); let v_ia = v_ia.clone(); let v_ib = v_ib.clone(); let v_idx = v_idx.clone(); let v_imax = v_imax.clone(); let v_j = v_j.clone(); let v_k = v_k.clone(); let v_name = v_name.clone(); let v_nodes = v_nodes.clone(); let v_r = v_r.clone(); let v_t = v_t.clone(); let v_val = v_val.clone(); let v_vmax = v_vmax.clone(); let v_vs = v_vs.clone(); let v_x = v_x.clone(); let v_z = v_z.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_v: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_s: Py = a.get(1).cloned().unwrap_or(Py::None);
            return Ok((if ({ let t2907 = py::abs(&v_v.clone())?; Py::Bool({ let t2908 = py::mul(&Py::Float(1e-12), &v_s.clone())?; if py::le(&t2907, &t2908)? { true } else { false } }) }).truthy() { Py::Float(0.0) } else { v_v.clone() }));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_volts = { let t2909 = Py::dict(vec![]); py::setitem(&t2909, &Py::str("0"), Py::Float(0.0))?; py::kw_merge(&t2909, &{ let mut out = vec![]; for t2910 in py::iter(&(v_nodes.clone()))? { let mut v_n: Py; 
v_n = t2910;
out.push((v_n.clone(), py::call(&v_snap.clone(), &[py::getitem(&v_x.clone(), &py::getitem(&v_idx.clone(), &v_n.clone())?)?, v_vmax.clone()])?)); } Py::dict(out) })?; t2909 };
    v_cur = { let mut out = vec![]; for t2911 in py::iter(&(py::enumerate(&v_vs.clone(), 0)?))? { let mut v_e: Py; let mut v_k: Py; 
let t2912 = (t2911).items()?;
if t2912.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t2912[0].clone();
v_e = t2912[1].clone();
out.push((py::getitem(&v_e.clone(), &Py::Int(1i64))?, py::call(&v_snap.clone(), &[py::getitem(&v_x.clone(), &py::add(&py::len(&v_nodes.clone())?, &v_k.clone())?)?, v_imax.clone()])?)); } Py::dict(out) };
    return Ok(Py::tuple(vec![v_volts.clone(), v_cur.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__net_count(mut v_t: Py) -> R {
    return Ok((if (Py::Bool(matches!(v_t.clone(), Py::Float(_)))).truthy() { Py::Int(1i64) } else { py::sum(&{ let mut out = vec![]; for t2242 in py::iter(&(py::getitem(&v_t.clone(), &Py::Int(1i64))?))? { let mut v_x: Py; 
v_x = t2242;
out.push(f__net_count(v_x.clone())?); } Py::list(out) }, None)? }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__net_tokens(mut v_text: Py) -> R {
    let mut v_toks: Py = Py::None;
    v_toks = ax_engine::pyre::module_call("findall", &Py::str("\\|\\||\\+|\\(|\\)|(?:\\d+(?:\\.\\d+)?|\\.\\d+)(?:[eE][-+]?\\d+)?|\\S"), &[v_text.clone()], 0)?;
    if ({ let t2210 = py::method(&Py::str(""), "join", &[v_toks.clone()])?; Py::Bool({ let t2211 = ax_engine::pyre::module_call("sub", &Py::str("\\s+"), &[Py::str(""), v_text.clone(), Py::Int(0)], 0)?; if py::ne(&t2210, &t2211) { true } else { false } }) }).truthy() {
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
            return Ok((if ({ let t2212 = py::getitem(&v_pos.clone(), &Py::Int(0i64))?; Py::Bool({ let t2213 = py::len(&v_toks.clone())?; if py::lt(&t2212, &t2213)? { true } else { false } }) }).truthy() { py::getitem(&v_toks.clone(), &py::getitem(&v_pos.clone(), &Py::Int(0i64))?)? } else { Py::None }));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_take = { let v_peek = v_peek.clone(); let v_pos = v_pos.clone(); let v_take = v_take.clone(); let v_text = v_text.clone(); let v_toks = v_toks.clone(); let c_expr = c_expr.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            if ({ let t2214 = py::getitem(&v_pos.clone(), &Py::Int(0i64))?; Py::Bool({ let t2215 = py::len(&v_toks.clone())?; if py::ge(&t2214, &t2215)? { true } else { false } }) }).truthy() {
                return Err(py::value_error(py::to_str(&Py::str("the network expression ends too early (unbalanced parentheses?)"))));
            }
            let t2216 = v_pos.clone(); let t2217 = Py::Int(0i64);
            py::setitem(&t2216, &t2217, py::iadd(&py::getitem(&t2216, &t2217)?, &Py::Int(1i64))?)?;
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
            if ({ let t2218 = v_t.clone(); Py::Bool({ let t2219 = Py::str("("); if py::eq(&t2218, &t2219) { true } else { false } }) }).truthy() {
                let _ = py::call(&v_take.clone(), &[])?;
                v_node = py::call(&py::cell_get(&c_expr)?, &[])?;
                if ({ let t2220 = py::call(&v_take.clone(), &[])?; Py::Bool({ let t2221 = Py::str(")"); if py::ne(&t2220, &t2221) { true } else { false } }) }).truthy() {
                    return Err(py::value_error(py::to_str(&Py::str("unbalanced parentheses in the network expression"))));
                }
                return Ok(v_node.clone());
            }
            if ({ let t2224 = { let t2222 = v_t.clone(); Py::Bool({ let t2223 = Py::None; if py::is(&t2222, &t2223) { true } else { false } }) }; if t2224.truthy() { t2224 } else { py::not(&ax_engine::pyre::module_call("fullmatch", &Py::str("(?:\\d+(?:\\.\\d+)?|\\.\\d+)(?:[eE][-+]?\\d+)?"), &[v_t.clone()], 0)?) } }).truthy() {
                return Err(py::value_error(py::to_str(&Py::str(&["expected a resistance value, found ".to_string(), py::format(&Py::str(&py::repr(&v_t.clone())), "")?].concat()))));
            }
            let _ = py::call(&v_take.clone(), &[])?;
            v_v = py::float(&v_t.clone())?;
            if (py::not(&{ let t2227 = { let t2225 = v_v.clone(); Py::Bool({ let t2226 = Py::Int(0i64); if py::gt(&t2225, &t2226)? { true } else { false } }) }; if !t2227.truthy() { t2227 } else { py::math1("isfinite", &v_v.clone())? } })).truthy() {
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
            while ({ let t2228 = py::call(&v_peek.clone(), &[])?; Py::Bool({ let t2229 = Py::str("||"); if py::eq(&t2228, &t2229) { true } else { false } }) }).truthy() {
                let _ = py::call(&v_take.clone(), &[])?;
                let _ = py::method(&v_items.clone(), "append", &[py::call(&v_factor.clone(), &[])?])?;
            }
            return Ok((if ({ let t2230 = py::len(&v_items.clone())?; Py::Bool({ let t2231 = Py::Int(1i64); if py::eq(&t2230, &t2231) { true } else { false } }) }).truthy() { py::getitem(&v_items.clone(), &Py::Int(0i64))? } else { Py::tuple(vec![Py::str("p"), v_items.clone()]) }));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    py::cell_set(&c_expr, { let v_factor = v_factor.clone(); let v_peek = v_peek.clone(); let v_pos = v_pos.clone(); let v_take = v_take.clone(); let v_term = v_term.clone(); let v_text = v_text.clone(); let v_toks = v_toks.clone(); let c_expr = c_expr.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_items: Py = Py::None;
            v_items = Py::list(vec![py::call(&v_term.clone(), &[])?]);
            while ({ let t2232 = py::call(&v_peek.clone(), &[])?; Py::Bool({ let t2233 = Py::str("+"); if py::eq(&t2232, &t2233) { true } else { false } }) }).truthy() {
                let _ = py::call(&v_take.clone(), &[])?;
                let _ = py::method(&v_items.clone(), "append", &[py::call(&v_term.clone(), &[])?])?;
            }
            return Ok((if ({ let t2234 = py::len(&v_items.clone())?; Py::Bool({ let t2235 = Py::Int(1i64); if py::eq(&t2234, &t2235) { true } else { false } }) }).truthy() { py::getitem(&v_items.clone(), &Py::Int(0i64))? } else { Py::tuple(vec![Py::str("s"), v_items.clone()]) }));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) });
    v_tree = py::call(&py::cell_get(&c_expr)?, &[])?;
    if ({ let t2236 = py::getitem(&v_pos.clone(), &Py::Int(0i64))?; Py::Bool({ let t2237 = py::len(&v_toks.clone())?; if py::ne(&t2236, &t2237) { true } else { false } }) }).truthy() {
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
    v_vals = { let mut out = vec![]; for t2238 in py::iter(&(py::getitem(&v_t.clone(), &Py::Int(1i64))?))? { let mut v_x: Py; 
v_x = t2238;
out.push(f__net_value(v_x.clone())?); } Py::list(out) };
    return Ok((if ({ let t2239 = py::getitem(&v_t.clone(), &Py::Int(0i64))?; Py::Bool({ let t2240 = Py::str("s"); if py::eq(&t2239, &t2240) { true } else { false } }) }).truthy() { py::fsum(&v_vals.clone())? } else { py::truediv(&Py::Float(1.0), &py::fsum(&{ let mut out = vec![]; for t2241 in py::iter(&(v_vals.clone()))? { let mut v_v: Py; 
v_v = t2241;
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
    let t2798 = (Py::tuple(vec![Py::list(vec![]), py::set(&Py::list(vec![]))?])).items()?;
    if t2798.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_els = t2798[0].clone();
    v_names = t2798[1].clone();
    for t2799 in py::iter(&(ax_engine::pyre::module_call("split", &Py::str("[;\\n]+"), &[v_text.clone(), Py::Int(0)], 0)?))? {
        v_part = t2799;
        v_p = py::method(&py::method(&py::method(&v_part.clone(), "strip", &[])?, "rstrip", &[Py::str(".")])?, "strip", &[])?;
        if (py::not(&v_p.clone())).truthy() {
            continue;
        }
        v_toks = py::method(&v_p.clone(), "split", &[])?;
        if ({ let t2800 = py::len(&v_toks.clone())?; Py::Bool({ let t2801 = Py::Int(4i64); if py::ne(&t2800, &t2801) { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str(&["cannot read the element '".to_string(), py::format(&v_p.clone(), "")?, "': write <name> <node> <node> <value>".to_string()].concat()))));
        }
        let t2802 = (v_toks.clone()).items()?;
        if t2802.len() != 4 { return Err(PyErr::Value("unpack".into())); }
        v_name = t2802[0].clone();
        v_a = t2802[1].clone();
        v_b = t2802[2].clone();
        v_v = t2802[3].clone();
        v_t = py::method(&py::getitem(&v_name.clone(), &Py::Int(0i64))?, "upper", &[])?;
        if ({ let t2803 = v_t.clone(); Py::Bool({ let t2804 = Py::str("RVI"); if !py::contains(&t2804, &t2803)? { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str(&["element '".to_string(), py::format(&v_name.clone(), "")?, "': only resistors (R), voltage sources (V) and current sources (I)".to_string()].concat()))));
        }
        if ({ let t2805 = py::method(&v_name.clone(), "upper", &[])?; Py::Bool({ let t2806 = v_names.clone(); if py::contains(&t2806, &t2805)? { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str(&["element name '".to_string(), py::format(&v_name.clone(), "")?, "' is used twice".to_string()].concat()))));
        }
        let _ = py::method(&v_names.clone(), "add", &[py::method(&v_name.clone(), "upper", &[])?])?;
        let t2807: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                v_val = py::float(&v_v.clone())?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t2807 {
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
        if ({ let t2812 = { let t2808 = v_t.clone(); Py::Bool({ let t2809 = Py::str("R"); if py::eq(&t2808, &t2809) { true } else { false } }) }; if !t2812.truthy() { t2812 } else { { let t2810 = v_val.clone(); Py::Bool({ let t2811 = Py::Int(0i64); if py::le(&t2810, &t2811)? { true } else { false } }) } } }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str(&["resistor '".to_string(), py::format(&v_name.clone(), "")?, "' must have a positive resistance".to_string()].concat()))));
        }
        if ({ let t2813 = v_a.clone(); Py::Bool({ let t2814 = v_b.clone(); if py::eq(&t2813, &t2814) { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str(&["element '".to_string(), py::format(&v_name.clone(), "")?, "' connects node ".to_string(), py::format(&v_a.clone(), "")?, " to itself".to_string()].concat()))));
        }
        let _ = py::method(&v_els.clone(), "append", &[Py::tuple(vec![v_t.clone(), v_name.clone(), v_a.clone(), v_b.clone(), v_val.clone()])])?;
    }
    if (py::not(&Py::Bool(({ let mut out = vec![]; for t2815 in py::iter(&(v_els.clone()))? { let mut v_e: Py; 
v_e = t2815;
out.push({ let t2816 = py::getitem(&v_e.clone(), &Py::Int(0i64))?; Py::Bool({ let t2817 = Py::str("R"); if py::eq(&t2816, &t2817) { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the circuit needs at least one resistor"))));
    }
    if (py::not(&Py::Bool(({ let mut out = vec![]; for t2818 in py::iter(&(v_els.clone()))? { let mut v_e: Py; 
v_e = t2818;
out.push({ let t2819 = Py::str("0"); Py::Bool({ let t2820 = Py::tuple(vec![py::getitem(&v_e.clone(), &Py::Int(2i64))?, py::getitem(&v_e.clone(), &Py::Int(3i64))?]); if py::contains(&t2820, &t2819)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())))).truthy() {
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
    let t2678 = (Py::tuple(vec![py::add(&py::mul(&Py::list(vec![Py::Float(0.0)]), &py::sub(&v_n.clone(), &py::len(&v_a.clone())?)?)?, &Py::list((v_a.clone()).items()?))?, py::add(&py::mul(&Py::list(vec![Py::Float(0.0)]), &py::sub(&v_n.clone(), &py::len(&v_b.clone())?)?)?, &Py::list((v_b.clone()).items()?))?])).items()?;
    if t2678.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t2678[0].clone();
    v_b = t2678[1].clone();
    return Ok({ let mut out = vec![]; for t2679 in py::iter(&(py::zip(&[v_a.clone(), v_b.clone()])?))? { let mut v_x: Py; let mut v_y: Py; 
let t2680 = (t2679).items()?;
if t2680.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t2680[0].clone();
v_y = t2680[1].clone();
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
    return Ok({ let t2695 = { let mut out = vec![]; for t2693 in py::iter(&(py::enumerate(&py::slice(&v_c.clone(), None, Some((py::neg(&Py::Int(1i64))?).i()?), None)?, 0)?))? { let mut v_i: Py; let mut v_x: Py; 
let t2694 = (t2693).items()?;
if t2694.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_i = t2694[0].clone();
v_x = t2694[1].clone();
out.push(py::mul(&v_x.clone(), &py::sub(&v_n.clone(), &v_i.clone())?)?); } Py::list(out) }; if t2695.truthy() { t2695 } else { Py::list(vec![Py::Float(0.0)]) } });
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
    let t1333 = (Py::tuple(vec![f__iw(v_num.clone())?, f__iw(v_den.clone())?])).items()?;
    if t1333.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_n_iw = t1333[0].clone();
    v_d_iw = t1333[1].clone();
    v_nd = f__pmul(v_n_iw.clone(), { let mut out = vec![]; for t1340 in py::iter(&(v_d_iw.clone()))? { let mut v_y: Py; 
v_y = t1340;
out.push(py::method(&v_y.clone(), "conjugate", &[])?); } Py::list(out) })?;
    v_im = f__ptrim({ let mut out = vec![]; for t1341 in py::iter(&(v_nd.clone()))? { let mut v_x: Py; 
v_x = t1341;
out.push(py::imag(&v_x.clone())?); } Py::list(out) })?;
    if ({ let t1342 = py::len(&v_im.clone())?; Py::Bool({ let t1343 = Py::Int(2i64); if py::lt(&t1342, &t1343)? { true } else { false } }) }).truthy() {
        return Ok(Py::list(vec![]));
    }
    v_ws = { let mut out = vec![]; for t1390 in py::iter(&(f__positive_real_roots(v_im.clone(), Py::Float(1.0))?))? { let mut v_w: Py; 
v_w = t1390;
if ({ let t1391 = py::real(&f__G(v_num.clone(), v_den.clone(), v_w.clone())?)?; Py::Bool({ let t1392 = Py::Int(0i64); if py::lt(&t1391, &t1392)? { true } else { false } }) }).truthy() { out.push(v_w.clone()); }} Py::list(out) };
    return Ok({ let mut out = vec![]; for t1395 in py::iter(&(v_ws.clone()))? { let mut v_w: Py; 
v_w = t1395;
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
fn f__phasor_parts(mut v_z: Py, mut v_scale: Py) -> R {
    let mut v_ang: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_s: Py = Py::None;
    v_s = f__csnap(v_z.clone(), v_scale.clone())?;
    v_m = py::hypot(&[py::real(&v_s.clone())?, py::imag(&v_s.clone())?])?;
    v_ang = (if ({ let t607 = v_m.clone(); Py::Bool({ let t608 = Py::Int(0i64); if py::gt(&t607, &t608)? { true } else { false } }) }).truthy() { py::math1("degrees", &py::atan2(&py::imag(&v_s.clone())?, &py::real(&v_s.clone())?)?)? } else { Py::Float(0.0) });
    if ({ let t609 = v_ang.clone(); Py::Bool({ let t610 = py::neg(&Py::Float(180.0))?; if py::eq(&t609, &t610) { true } else { false } }) }).truthy() {
        v_ang = Py::Float(180.0);
    }
    return Ok(Py::tuple(vec![f__r12(py::real(&v_s.clone())?)?, f__r12(py::imag(&v_s.clone())?)?, f__r12(v_m.clone())?, f__r12(v_ang.clone())?]));
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
    for t2044 in py::iter(&(py::enumerate(&v_a.clone(), 0)?))? {
        let t2049 = (t2044).items()?;
        if t2049.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_i = t2049[0].clone();
        v_x = t2049[1].clone();
        for t2045 in py::iter(&(py::enumerate(&v_b.clone(), 0)?))? {
            let t2048 = (t2045).items()?;
            if t2048.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_j = t2048[0].clone();
            v_y = t2048[1].clone();
            let t2046 = v_out.clone(); let t2047 = py::add(&v_i.clone(), &v_j.clone())?;
            py::setitem(&t2046, &t2047, py::iadd(&py::getitem(&t2046, &t2047)?, &py::mul(&v_x.clone(), &v_y.clone())?)?)?;
        }
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__polar_err(mut v_o: Py, mut v_p: Py) -> R {
    let mut v_e: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_z: Py = Py::None;
    v_z = py::complex(&[py::getitem(&v_o.clone(), &py::add(&v_p.clone(), &Py::str("_real"))?)?, py::getitem(&v_o.clone(), &py::add(&v_p.clone(), &Py::str("_imag"))?)?])?;
    v_m = f__cabs(v_z.clone())?;
    v_e = py::truediv(&py::abs(&py::sub(&py::getitem(&v_o.clone(), &py::add(&v_p.clone(), &Py::str("_mag"))?)?, &v_m.clone())?)?, &py::minmax(&[Py::Float(1e-300), py::abs(&py::getitem(&v_o.clone(), &py::add(&v_p.clone(), &Py::str("_mag"))?)?)?, v_m.clone()], true)?)?;
    if ({ let t624 = v_m.clone(); Py::Bool({ let t625 = Py::Int(0i64); if py::gt(&t624, &t625)? { true } else { false } }) }).truthy() {
        v_e = py::iadd(&v_e, &py::truediv(&py::abs(&py::sub(&py::modulo(&py::add(&py::sub(&py::getitem(&v_o.clone(), &py::add(&v_p.clone(), &Py::str("_deg"))?)?, &py::math1("degrees", &py::atan2(&py::imag(&v_z.clone())?, &py::real(&v_z.clone())?)?)?)?, &Py::Float(180.0))?, &Py::Float(360.0))?, &Py::Float(180.0))?)?, &Py::Float(180.0))?)?;
    } else {
        v_e = py::iadd(&v_e, &py::truediv(&py::abs(&py::getitem(&v_o.clone(), &py::add(&v_p.clone(), &Py::str("_deg"))?)?)?, &Py::Float(180.0))?)?;
    }
    return Ok(v_e.clone());
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
    for t2105 in py::range_iter(&[(Py::Int(30i64)).i()?]) {
        v__ = t2105;
        v_h = py::mul(&Py::Float(1e-07), &v_w.clone())?;
        v_fw = py::call(&v_f.clone(), &[v_w.clone()])?;
        v_d = py::truediv(&py::sub(&py::call(&v_f.clone(), &[py::add(&v_w.clone(), &v_h.clone())?])?, &py::call(&v_f.clone(), &[py::sub(&v_w.clone(), &v_h.clone())?])?)?, &py::mul(&Py::Int(2i64), &v_h.clone())?)?;
        if ({ let t2106 = v_d.clone(); Py::Bool({ let t2107 = Py::Int(0i64); if py::eq(&t2106, &t2107) { true } else { false } }) }).truthy() {
            break;
        }
        v_step = py::truediv(&v_fw.clone(), &v_d.clone())?;
        v_w2 = py::sub(&v_w.clone(), &v_step.clone())?;
        if ({ let t2108 = v_w2.clone(); Py::Bool({ let t2109 = Py::Int(0i64); if py::le(&t2108, &t2109)? { true } else { false } }) }).truthy() {
            break;
        }
        if ({ let t2110 = py::abs(&v_step.clone())?; Py::Bool({ let t2111 = py::mul(&Py::Float(1e-16), &v_w.clone())?; if py::le(&t2110, &t2111)? { true } else { false } }) }).truthy() {
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
    for t2094 in py::iter(&(f__roots(v_c.clone())?))? {
        v_r = t2094;
        v_r = py::complex(&[v_r.clone()])?;
        if ({ let t2099 = { let t2095 = py::real(&v_r.clone())?; Py::Bool({ let t2096 = py::mul(&Py::Float(1e-09), &v_scale_w.clone())?; if py::gt(&t2095, &t2096)? { true } else { false } }) }; if !t2099.truthy() { t2099 } else { { let t2097 = py::abs(&py::imag(&v_r.clone())?)?; Py::Bool({ let t2098 = py::mul(&Py::Float(1e-07), &py::minmax(&[py::abs(&v_r.clone())?, Py::Float(1e-300)], true)?)?; if py::le(&t2097, &t2098)? { true } else { false } }) } } }).truthy() {
            let _ = py::method(&v_out.clone(), "append", &[py::real(&v_r.clone())?])?;
        }
    }
    let _ = py::method_sort(&v_out.clone(), None, false)?;
    v_merged = Py::list(vec![]);
    for t2100 in py::iter(&(v_out.clone()))? {
        v_x = t2100;
        if ({ let t2103 = py::not(&v_merged.clone()); if t2103.truthy() { t2103 } else { { let t2101 = py::abs(&py::sub(&v_x.clone(), &py::getitem(&v_merged.clone(), &py::neg(&Py::Int(1i64))?)?)?)?; Py::Bool({ let t2102 = py::mul(&Py::Float(1e-07), &v_x.clone())?; if py::gt(&t2101, &t2102)? { true } else { false } }) } } }).truthy() {
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
    if ({ let t2453 = py::len(&v_vals.clone())?; Py::Bool({ let t2454 = Py::Int(2i64); if py::lt(&t2453, &t2454)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&["give at least two ".to_string(), py::format(&v_what.clone(), "")?].concat()))));
    }
    for t2455 in py::iter(&(v_vals.clone()))? {
        v_v = t2455;
        if (py::not(&{ let t2458 = py::math1("isfinite", &v_v.clone())?; if !t2458.truthy() { t2458 } else { { let t2456 = v_v.clone(); Py::Bool({ let t2457 = Py::Int(0i64); if py::gt(&t2456, &t2457)? { true } else { false } }) } } })).truthy() {
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
    for t1191 in py::iter(&(v_els.clone()))? {
        let t1196 = (t1191).items()?;
        if t1196.len() != 5 { return Err(PyErr::Value("unpack".into())); }
        v_t = t1196[0].clone();
        v_name = t1196[1].clone();
        v_a = t1196[2].clone();
        v_b = t1196[3].clone();
        v_val = t1196[4].clone();
        if ({ let t1192 = v_t.clone(); Py::Bool({ let t1193 = Py::str("V"); if py::eq(&t1192, &t1193) { true } else { false } }) }).truthy() {
            v_p = py::iadd(&v_p, &py::mul(&py::neg(&v_val.clone())?, &py::getitem(&v_cur.clone(), &v_name.clone())?)?)?;
        } else {
            if ({ let t1194 = v_t.clone(); Py::Bool({ let t1195 = Py::str("I"); if py::eq(&t1194, &t1195) { true } else { false } }) }).truthy() {
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
    v_current = py::set(&{ let mut out = vec![]; for t3025 in py::iter(&(v_ones.clone()))? { let mut v_m: Py; 
v_m = t3025;
out.push(Py::tuple(vec![v_m.clone(), Py::Int(0i64)])); } Py::list(out) })?;
    v_primes = py::set(&Py::list(vec![]))?;
    while (v_current.clone()).truthy() {
        let t3026 = (Py::tuple(vec![py::set(&Py::list(vec![]))?, py::set(&Py::list(vec![]))?])).items()?;
        if t3026.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_nxt = t3026[0].clone();
        v_used = t3026[1].clone();
        v_lst = py::sorted(&v_current.clone(), (Py::Bool(false)).truthy())?;
        for t3027 in py::range_iter(&[(py::len(&v_lst.clone())?).i()?]) {
            v_i = t3027;
            for t3028 in py::range_iter(&[(py::add(&v_i.clone(), &Py::Int(1i64))?).i()?, (py::len(&v_lst.clone())?).i()?]) {
                v_j = t3028;
                let t3029 = (Py::tuple(vec![py::getitem(&v_lst.clone(), &v_i.clone())?, py::getitem(&v_lst.clone(), &v_j.clone())?])).items()?;
                if t3029.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                let t3030 = (t3029[0].clone()).items()?;
                if t3030.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                v_v1 = t3030[0].clone();
                v_m1 = t3030[1].clone();
                let t3031 = (t3029[1].clone()).items()?;
                if t3031.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                v_v2 = t3031[0].clone();
                v_m2 = t3031[1].clone();
                v_d = py::bitxor(&v_v1.clone(), &v_v2.clone())?;
                if ({ let t3037 = { let t3032 = v_m1.clone(); Py::Bool({ let t3033 = v_m2.clone(); if py::eq(&t3032, &t3033) { true } else { false } }) }; if !t3037.truthy() { t3037 } else { { let t3036 = v_d.clone(); if !t3036.truthy() { t3036 } else { { let t3034 = py::bitand(&v_d.clone(), &py::sub(&v_d.clone(), &Py::Int(1i64))?)?; Py::Bool({ let t3035 = Py::Int(0i64); if py::eq(&t3034, &t3035) { true } else { false } }) } } } } }).truthy() {
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
    v_c = { let mut out = vec![]; for t2661 in py::iter(&(v_c.clone()))? { let mut v_x: Py; 
v_x = t2661;
out.push(py::float(&v_x.clone())?); } Py::list(out) };
    while ({ let t2666 = { let t2662 = py::len(&v_c.clone())?; Py::Bool({ let t2663 = Py::Int(1i64); if py::gt(&t2662, &t2663)? { true } else { false } }) }; if !t2666.truthy() { t2666 } else { { let t2664 = py::getitem(&v_c.clone(), &Py::Int(0i64))?; Py::Bool({ let t2665 = Py::Int(0i64); if py::eq(&t2664, &t2665) { true } else { false } }) } } }).truthy() {
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
    for t2700 in py::iter(&(v_c.clone()))? {
        v_x = t2700;
        v_acc = py::add(&py::mul(&v_acc.clone(), &v_z.clone())?, &v_x.clone())?;
    }
    return Ok(v_acc.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__r12(mut v_x: Py) -> R {
    if ({ let t613 = { let t611 = v_x.clone(); Py::Bool({ let t612 = Py::Int(0i64); if py::eq(&t611, &t612) { true } else { false } }) }; if t613.truthy() { t613 } else { py::not(&py::math1("isfinite", &v_x.clone())?) } }).truthy() {
        return Ok((if ({ let t614 = v_x.clone(); Py::Bool({ let t615 = Py::Int(0i64); if py::eq(&t614, &t615) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { v_x.clone() }));
    }
    return Ok(py::add(&py::float(&Py::str(&[py::format(&v_x.clone(), ".12g")?].concat()))?, &Py::Float(0.0))?);
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
    v_R = { let mut out = vec![]; for t989 in py::iter(&(v_M.clone()))? { let mut v_row: Py; 
v_row = t989;
out.push(py::slice(&v_row.clone(), None, None, None)?); } Py::list(out) };
    let t990 = (Py::tuple(vec![py::len(&v_R.clone())?, py::len(&py::getitem(&v_R.clone(), &Py::Int(0i64))?)?])).items()?;
    if t990.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_rows = t990[0].clone();
    v_cols = t990[1].clone();
    v_scale = { let t993 = py::minmax(&[{ let mut out = vec![]; for t991 in py::iter(&(v_R.clone()))? { let mut v_r: Py; 
v_r = t991;
for t992 in py::iter(&(v_r.clone()))? { let mut v_x: Py; 
v_x = t992;
out.push(py::abs(&v_x.clone())?); }} Py::list(out) }], true)?; if t993.truthy() { t993 } else { Py::Float(1.0) } };
    let t994 = (Py::tuple(vec![Py::Int(0i64), Py::Int(0i64)])).items()?;
    if t994.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_rank = t994[0].clone();
    v_r = t994[1].clone();
    for t995 in py::range_iter(&[(v_cols.clone()).i()?]) {
        v_c = t995;
        v_p = py::minmax_key(&[Py::list((py::range_iter(&[(v_r.clone()).i()?, (v_rows.clone()).i()?])).into_iter().collect::<Vec<Py>>())], &{ let v_M = v_M.clone(); let v_R = v_R.clone(); let v_c = v_c.clone(); let v_cols = v_cols.clone(); let v_r = v_r.clone(); let v_rank = v_rank.clone(); let v_rows = v_rows.clone(); let v_scale = v_scale.clone(); let v_tol = v_tol.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_i: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::abs(&py::getitem(&py::getitem(&v_R.clone(), &v_i.clone())?, &v_c.clone())?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, true)?;
        if ({ let t1000 = { let t996 = v_p.clone(); Py::Bool({ let t997 = Py::None; if py::is(&t996, &t997) { true } else { false } }) }; if t1000.truthy() { t1000 } else { { let t998 = py::abs(&py::getitem(&py::getitem(&v_R.clone(), &v_p.clone())?, &v_c.clone())?)?; Py::Bool({ let t999 = py::mul(&v_tol.clone(), &v_scale.clone())?; if py::le(&t998, &t999)? { true } else { false } }) } } }).truthy() {
            continue;
        }
        let t1001 = (Py::tuple(vec![py::getitem(&v_R.clone(), &v_p.clone())?, py::getitem(&v_R.clone(), &v_r.clone())?])).items()?;
        if t1001.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        py::setitem(&v_R.clone(), &v_r.clone(), t1001[0].clone())?;
        py::setitem(&v_R.clone(), &v_p.clone(), t1001[1].clone())?;
        for t1002 in py::range_iter(&[(py::add(&v_r.clone(), &Py::Int(1i64))?).i()?, (v_rows.clone()).i()?]) {
            v_i = t1002;
            v_f = py::truediv(&py::getitem(&py::getitem(&v_R.clone(), &v_i.clone())?, &v_c.clone())?, &py::getitem(&py::getitem(&v_R.clone(), &v_r.clone())?, &v_c.clone())?)?;
            py::setitem(&v_R.clone(), &v_i.clone(), { let mut out = vec![]; for t1003 in py::iter(&(py::zip(&[py::getitem(&v_R.clone(), &v_i.clone())?, py::getitem(&v_R.clone(), &v_r.clone())?])?))? { let mut v_x: Py; let mut v_y: Py; 
let t1004 = (t1003).items()?;
if t1004.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t1004[0].clone();
v_y = t1004[1].clone();
out.push(py::sub(&v_x.clone(), &py::mul(&v_f.clone(), &v_y.clone())?)?); } Py::list(out) })?;
        }
        v_r = py::iadd(&v_r, &Py::Int(1i64))?;
        v_rank = py::iadd(&v_rank, &Py::Int(1i64))?;
        if ({ let t1005 = v_r.clone(); Py::Bool({ let t1006 = v_rows.clone(); if py::eq(&t1005, &t1006) { true } else { false } }) }).truthy() {
            break;
        }
    }
    return Ok(v_rank.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__rect_deg(mut v_m: Py, mut v_deg: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_t: Py = Py::None;
    v_a = py::modulo(&v_deg.clone(), &Py::Float(360.0))?;
    if ({ let t532 = v_a.clone(); Py::Bool({ let t533 = Py::Int(0i64); if py::eq(&t532, &t533) { true } else { false } }) }).truthy() {
        return Ok(py::complex(&[v_m.clone(), Py::Float(0.0)])?);
    }
    if ({ let t534 = v_a.clone(); Py::Bool({ let t535 = Py::Int(90i64); if py::eq(&t534, &t535) { true } else { false } }) }).truthy() {
        return Ok(py::complex(&[Py::Float(0.0), v_m.clone()])?);
    }
    if ({ let t536 = v_a.clone(); Py::Bool({ let t537 = Py::Int(180i64); if py::eq(&t536, &t537) { true } else { false } }) }).truthy() {
        return Ok(py::complex(&[py::neg(&v_m.clone())?, Py::Float(0.0)])?);
    }
    if ({ let t538 = v_a.clone(); Py::Bool({ let t539 = Py::Int(270i64); if py::eq(&t538, &t539) { true } else { false } }) }).truthy() {
        return Ok(py::complex(&[Py::Float(0.0), py::neg(&v_m.clone())?])?);
    }
    v_t = py::math1("radians", &v_a.clone())?;
    return Ok(py::complex(&[py::mul(&v_m.clone(), &py::math1("cos", &v_t.clone())?)?, py::mul(&v_m.clone(), &py::math1("sin", &v_t.clone())?)?])?);
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
    let t2484 = (Py::tuple(vec![py::add(&py::neg(&v_a.clone())?, &py::math1("sqrt", &py::add(&py::mul(&v_a.clone(), &v_a.clone())?, &py::mul(&v_w0.clone(), &v_w0.clone())?)?)?)?, py::add(&v_a.clone(), &py::math1("sqrt", &py::add(&py::mul(&v_a.clone(), &v_a.clone())?, &py::mul(&v_w0.clone(), &v_w0.clone())?)?)?)?])).items()?;
    if t2484.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_w1 = t2484[0].clone();
    v_w2 = t2484[1].clone();
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
    if ({ let t2683 = py::len(&v_c.clone())?; Py::Bool({ let t2684 = Py::Int(2i64); if py::lt(&t2683, &t2684)? { true } else { false } }) }).truthy() {
        return Ok(Py::list(vec![]));
    }
    v_zeros = Py::Int(0i64);
    while ({ let t2689 = { let t2685 = py::len(&v_c.clone())?; Py::Bool({ let t2686 = Py::Int(1i64); if py::gt(&t2685, &t2686)? { true } else { false } }) }; if !t2689.truthy() { t2689 } else { { let t2687 = py::getitem(&v_c.clone(), &py::neg(&Py::Int(1i64))?)?; Py::Bool({ let t2688 = Py::Int(0i64); if py::eq(&t2687, &t2688) { true } else { false } }) } } }).truthy() {
        let _ = py::method(&v_c.clone(), "pop", &[])?;
        v_zeros = py::iadd(&v_zeros, &Py::Int(1i64))?;
    }
    v_n = py::sub(&py::len(&v_c.clone())?, &Py::Int(1i64))?;
    if ({ let t2690 = v_n.clone(); Py::Bool({ let t2691 = Py::Int(0i64); if py::eq(&t2690, &t2691) { true } else { false } }) }).truthy() {
        return Ok(py::mul(&Py::list(vec![Py::Float(0.0)]), &v_zeros.clone())?);
    }
    v_a = { let mut out = vec![]; for t2692 in py::iter(&(v_c.clone()))? { let mut v_x: Py; 
v_x = t2692;
out.push(py::truediv(&v_x.clone(), &py::getitem(&v_c.clone(), &Py::Int(0i64))?)?); } Py::list(out) };
    v_d = f__pder(v_a.clone())?;
    v_r = { let t2696 = py::pow(&py::abs(&py::getitem(&v_a.clone(), &py::neg(&Py::Int(1i64))?)?)?, &py::truediv(&Py::Float(1.0), &v_n.clone())?)?; if t2696.truthy() { t2696 } else { Py::Float(1.0) } };
    v_z = { let mut out = vec![]; for t2697 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_k: Py; 
v_k = t2697;
out.push(py::mul(&v_r.clone(), &py::cmath("exp", &[py::mul(&Py::Complex(0.0, 1.0), &py::add(&py::truediv(&py::mul(&py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?, &v_k.clone())?, &v_n.clone())?, &Py::Float(0.4))?)?])?)?); } Py::list(out) };
    for t2698 in py::range_iter(&[(Py::Int(800i64)).i()?]) {
        v__ = t2698;
        v_biggest = Py::Float(0.0);
        for t2699 in py::range_iter(&[(v_n.clone()).i()?]) {
            v_i = t2699;
            v_p = f__pval(v_a.clone(), py::getitem(&v_z.clone(), &v_i.clone())?)?;
            if ({ let t2701 = v_p.clone(); Py::Bool({ let t2702 = Py::Int(0i64); if py::eq(&t2701, &t2702) { true } else { false } }) }).truthy() {
                continue;
            }
            v_dp = f__pval(v_d.clone(), py::getitem(&v_z.clone(), &v_i.clone())?)?;
            v_ratio = (if ({ let t2703 = v_dp.clone(); Py::Bool({ let t2704 = Py::Int(0i64); if py::ne(&t2703, &t2704) { true } else { false } }) }).truthy() { py::truediv(&v_p.clone(), &v_dp.clone())? } else { py::mul(&v_p.clone(), &Py::Float(0.001))? });
            v_s = py::sum(&{ let mut out = vec![]; for t2705 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t2705;
if ({ let t2710 = { let t2706 = v_j.clone(); Py::Bool({ let t2707 = v_i.clone(); if py::ne(&t2706, &t2707) { true } else { false } }) }; if !t2710.truthy() { t2710 } else { { let t2708 = py::getitem(&v_z.clone(), &v_i.clone())?; Py::Bool({ let t2709 = py::getitem(&v_z.clone(), &v_j.clone())?; if py::ne(&t2708, &t2709) { true } else { false } }) } } }).truthy() { out.push(py::truediv(&Py::Float(1.0), &py::sub(&py::getitem(&v_z.clone(), &v_i.clone())?, &py::getitem(&v_z.clone(), &v_j.clone())?)?)?); }} Py::list(out) }, None)?;
            v_w = py::truediv(&v_ratio.clone(), &py::sub(&Py::Int(1i64), &py::mul(&v_ratio.clone(), &v_s.clone())?)?)?;
            let t2711 = v_z.clone(); let t2712 = v_i.clone();
            py::setitem(&t2711, &t2712, py::sub(&py::getitem(&t2711, &t2712)?, &v_w.clone())?)?;
            v_biggest = py::minmax(&[v_biggest.clone(), py::truediv(&py::abs(&v_w.clone())?, &py::minmax(&[Py::Float(1.0), py::abs(&py::getitem(&v_z.clone(), &v_i.clone())?)?], true)?)?], true)?;
        }
        if ({ let t2713 = v_biggest.clone(); Py::Bool({ let t2714 = Py::Float(1e-16); if py::lt(&t2713, &t2714)? { true } else { false } }) }).truthy() {
            break;
        }
    }
    v_out = Py::list(vec![]);
    for t2715 in py::iter(&(v_z.clone()))? {
        v_x = t2715;
        for t2716 in py::range_iter(&[(Py::Int(2i64)).i()?]) {
            v__ = t2716;
            v_dp = f__pval(v_d.clone(), v_x.clone())?;
            if ({ let t2717 = v_dp.clone(); Py::Bool({ let t2718 = Py::Int(0i64); if py::ne(&t2717, &t2718) { true } else { false } }) }).truthy() {
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
    v_s = { let t = { let mut out = vec![]; for t2617 in py::iter(&(v_cs.clone()))? { let mut v_c: Py; 
v_c = t2617;
out.push(py::abs(&v_c.clone())?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } };
    return Ok({ let mut out = vec![]; for t2618 in py::iter(&(v_cs.clone()))? { let mut v_c: Py; 
v_c = t2618;
out.push((if ({ let t2619 = py::abs(&v_c.clone())?; Py::Bool({ let t2620 = py::mul(&Py::Float(1e-12), &v_s.clone())?; if py::le(&t2619, &t2620)? { true } else { false } }) }).truthy() { Py::Float(0.0) } else { v_c.clone() })); } Py::list(out) });
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
    for t3080 in py::range_iter(&[(py::pow(&Py::Int(2i64), &v_n.clone())?).i()?]) {
        v_m = t3080;
        if ({ let t3081 = v_m.clone(); Py::Bool({ let t3082 = v_dc.clone(); if py::contains(&t3082, &t3081)? { true } else { false } }) }).truthy() {
            continue;
        }
        v_env = { let mut out = vec![]; for t3083 in py::iter(&(py::enumerate(&v_names.clone(), 0)?))? { let mut v_i: Py; let mut v_nm: Py; 
let t3084 = (t3083).items()?;
if t3084.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_i = t3084[0].clone();
v_nm = t3084[1].clone();
out.push((v_nm.clone(), py::bitand(&py::rshift(&v_m.clone(), &py::sub(&py::sub(&v_n.clone(), &Py::Int(1i64))?, &v_i.clone())?)?, &Py::Int(1i64))?)); } Py::dict(out) };
        v_bad = py::iadd(&v_bad, &{ let t3085 = f__bool_eval(v_tree.clone(), v_env.clone())?; Py::Bool({ let t3086 = { let t3087 = v_m.clone(); Py::Bool({ let t3088 = v_on.clone(); if py::contains(&t3088, &t3087)? { true } else { false } }) }; if py::ne(&t3085, &t3086) { true } else { false } }) })?;
    }
    if ({ let t3089 = v_text.clone(); Py::Bool({ let t3090 = Py::tuple(vec![Py::str("0"), Py::str("1")]); if py::contains(&t3090, &t3089)? { true } else { false } }) }).truthy() {
        return Ok(Py::tuple(vec![v_bad.clone(), (if ({ let t3091 = v_text.clone(); Py::Bool({ let t3092 = Py::str("0"); if py::eq(&t3091, &t3092) { true } else { false } }) }).truthy() { Py::Int(0i64) } else { Py::Int(1i64) }), Py::Int(0i64)]));
    }
    v_terms = py::method(&v_text.clone(), "split", &[Py::str(" + ")])?;
    return Ok(Py::tuple(vec![v_bad.clone(), py::len(&v_terms.clone())?, py::sum(&{ let mut out = vec![]; for t3093 in py::iter(&(v_terms.clone()))? { let mut v_t: Py; 
v_t = t3093;
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
        if (Py::Bool(({ let mut out = vec![]; for t3065 in py::iter(&(v_cover.clone()))? { let mut v_p: Py; 
v_p = t3065;
out.push({ let t3066 = py::getitem(&v_p.clone(), &Py::Int(1i64))?; Py::Bool({ let t3067 = py::sub(&py::lshift(&Py::Int(1i64), &v_n.clone())?, &Py::Int(1i64))?; if py::eq(&t3066, &t3067) { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
            let t3068 = (Py::tuple(vec![Py::str("1"), Py::list(vec![Py::tuple(vec![Py::Int(0i64), py::sub(&py::lshift(&Py::Int(1i64), &v_n.clone())?, &Py::Int(1i64))?])])])).items()?;
            if t3068.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_text = t3068[0].clone();
            v_cover = t3068[1].clone();
        } else {
            v_text = py::method(&Py::str(" + "), "join", &[{ let mut out = vec![]; for t3069 in py::iter(&(v_cover.clone()))? { let mut v_p: Py; 
v_p = t3069;
out.push(f__term_text(v_p.clone(), v_names.clone())?); } Py::list(out) }])?;
        }
    }
    v_lits = py::sum(&{ let mut out = vec![]; for t3070 in py::iter(&(v_cover.clone()))? { let mut v_p: Py; 
v_p = t3070;
out.push(f__literals(v_p.clone(), v_n.clone())?); } Py::list(out) }, None)?;
    if ({ let t3071 = v_text.clone(); Py::Bool({ let t3072 = Py::tuple(vec![Py::str("0"), Py::str("1")]); if !py::contains(&t3072, &t3071)? { true } else { false } }) }).truthy() {
        v_text = py::method(&Py::str(" + "), "join", &[py::sorted(&py::method(&v_text.clone(), "split", &[Py::str(" + ")])?, (Py::Bool(false)).truthy())?])?;
    }
    return Ok(Py::dict(vec![(Py::str("minimal_sop"), v_text.clone()), (Py::str("terms"), py::float(&(if ({ let t3073 = v_text.clone(); Py::Bool({ let t3074 = Py::str("0"); if py::eq(&t3073, &t3074) { true } else { false } }) }).truthy() { Py::Int(0i64) } else { py::len(&v_cover.clone())? }))?), (Py::str("literals"), py::float(&(if ({ let t3075 = v_text.clone(); Py::Bool({ let t3076 = Py::str("1"); if py::eq(&t3075, &t3076) { true } else { false } }) }).truthy() { Py::Int(0i64) } else { v_lits.clone() }))?)]));
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
    let t3021 = (v_imp.clone()).items()?;
    if t3021.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_v = t3021[0].clone();
    v_mask = t3021[1].clone();
    v_lits = { let mut out = vec![]; for t3022 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t3022;
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
    v_nodes = py::set(&{ let mut out = vec![]; for t2821 in py::iter(&(v_els.clone()))? { let mut v_e: Py; 
v_e = t2821;
for t2822 in py::iter(&(Py::tuple(vec![py::getitem(&v_e.clone(), &Py::Int(2i64))?, py::getitem(&v_e.clone(), &Py::Int(3i64))?])))? { let mut v_n: Py; 
v_n = t2822;
out.push(v_n.clone()); }} Py::list(out) })?;
    for t2823 in py::iter(&(Py::tuple(vec![v_node_a.clone(), v_node_b.clone()])))? {
        v_n = t2823;
        if ({ let t2824 = v_n.clone(); Py::Bool({ let t2825 = v_nodes.clone(); if !py::contains(&t2825, &t2824)? { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str(&["node ".to_string(), py::format(&v_n.clone(), "")?, " is not in the circuit".to_string()].concat()))));
        }
    }
    if ({ let t2826 = v_node_a.clone(); Py::Bool({ let t2827 = v_node_b.clone(); if py::eq(&t2826, &t2827) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the two terminals must be different nodes"))));
    }
    let t2913 = (f__mna(v_els.clone())?).items()?;
    if t2913.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_volts = t2913[0].clone();
    v__ = t2913[1].clone();
    v_vth = py::sub(&py::getitem(&v_volts.clone(), &v_node_a.clone())?, &py::getitem(&v_volts.clone(), &v_node_b.clone())?)?;
    if ({ let t2914 = py::abs(&v_vth.clone())?; Py::Bool({ let t2915 = py::mul(&Py::Float(1e-12), &py::minmax(&[{ let mut out = vec![]; for t2916 in py::iter(&(py::method(&v_volts.clone(), "values", &[])?))? { let mut v_v: Py; 
v_v = t2916;
out.push(py::abs(&v_v.clone())?); } Py::list(out) }], true)?)?; if py::le(&t2914, &t2915)? { true } else { false } }) }).truthy() {
        v_vth = Py::Float(0.0);
    }
    v_dead = py::add(&{ let mut out = vec![]; for t2917 in py::iter(&(v_els.clone()))? { let mut v_a: Py; let mut v_b: Py; let mut v_n: Py; let mut v_t: Py; let mut v_v: Py; 
let t2918 = (t2917).items()?;
if t2918.len() != 5 { return Err(PyErr::Value("unpack".into())); }
v_t = t2918[0].clone();
v_n = t2918[1].clone();
v_a = t2918[2].clone();
v_b = t2918[3].clone();
v_v = t2918[4].clone();
if ({ let t2919 = v_t.clone(); Py::Bool({ let t2920 = Py::str("V"); if py::eq(&t2919, &t2920) { true } else { false } }) }).truthy() { out.push(Py::tuple(vec![v_t.clone(), v_n.clone(), v_a.clone(), v_b.clone(), Py::Float(0.0)])); }} Py::list(out) }, &{ let mut out = vec![]; for t2921 in py::iter(&(v_els.clone()))? { let mut v_e: Py; 
v_e = t2921;
if ({ let t2922 = py::getitem(&v_e.clone(), &Py::Int(0i64))?; Py::Bool({ let t2923 = Py::str("R"); if py::eq(&t2922, &t2923) { true } else { false } }) }).truthy() { out.push(v_e.clone()); }} Py::list(out) })?;
    let _ = py::method(&v_dead.clone(), "append", &[Py::tuple(vec![Py::str("I"), Py::str("Itest"), v_node_b.clone(), v_node_a.clone(), Py::Float(1.0)])])?;
    let t2924 = (f__mna(v_dead.clone())?).items()?;
    if t2924.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_v2 = t2924[0].clone();
    v__ = t2924[1].clone();
    v_rth = py::sub(&py::getitem(&v_v2.clone(), &v_node_a.clone())?, &py::getitem(&v_v2.clone(), &v_node_b.clone())?)?;
    v_scale = py::minmax(&[{ let mut out = vec![]; for t2925 in py::iter(&(v_els.clone()))? { let mut v_e: Py; 
v_e = t2925;
if ({ let t2926 = py::getitem(&v_e.clone(), &Py::Int(0i64))?; Py::Bool({ let t2927 = Py::str("R"); if py::eq(&t2926, &t2927) { true } else { false } }) }).truthy() { out.push(py::getitem(&v_e.clone(), &Py::Int(4i64))?); }} Py::list(out) }], true)?;
    if ({ let t2928 = py::abs(&v_rth.clone())?; Py::Bool({ let t2929 = py::mul(&Py::Float(1e-12), &v_scale.clone())?; if py::le(&t2928, &t2929)? { true } else { false } }) }).truthy() {
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
    let t2950 = (Py::tuple(vec![py::getitem(&v_inputs.clone(), &Py::str("VL"))?, py::getitem(&v_inputs.clone(), &Py::str("R"))?, py::getitem(&v_inputs.clone(), &Py::str("X"))?])).items()?;
    if t2950.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_VL = t2950[0].clone();
    v_R = t2950[1].clone();
    v_X = t2950[2].clone();
    v_Z = py::abs(&py::complex(&[v_R.clone(), v_X.clone()])?)?;
    v_S = py::mul(&py::mul(&py::math1("sqrt", &Py::Int(3i64))?, &v_VL.clone())?, &py::getitem(&v_outputs.clone(), &Py::str("line_current"))?)?;
    v_want_vph = (if ({ let t2951 = py::getitem(&v_inputs.clone(), &Py::str("connection"))?; Py::Bool({ let t2952 = Py::str("wye"); if py::eq(&t2951, &t2952) { true } else { false } }) }).truthy() { py::truediv(&v_VL.clone(), &py::math1("sqrt", &Py::Int(3i64))?)? } else { v_VL.clone() });
    return Ok(py::add(&py::add(&py::add(&py::add(&py::add(&f__rel(py::getitem(&v_outputs.clone(), &Py::str("phase_voltage"))?, v_want_vph.clone())?, &f__rel(py::mul(&py::getitem(&v_outputs.clone(), &Py::str("phase_current"))?, &v_Z.clone())?, py::getitem(&v_outputs.clone(), &Py::str("phase_voltage"))?)?)?, &f__rel(py::getitem(&v_outputs.clone(), &Py::str("S"))?, v_S.clone())?)?, &py::truediv(&py::abs(&py::sub(&py::getitem(&v_outputs.clone(), &Py::str("P"))?, &py::truediv(&py::mul(&v_S.clone(), &v_R.clone())?, &v_Z.clone())?)?)?, &v_S.clone())?)?, &py::truediv(&py::abs(&py::sub(&py::getitem(&v_outputs.clone(), &Py::str("Q"))?, &py::truediv(&py::mul(&v_S.clone(), &v_X.clone())?, &v_Z.clone())?)?)?, &v_S.clone())?)?, &py::abs(&py::sub(&py::getitem(&v_outputs.clone(), &Py::str("power_factor"))?, &py::truediv(&v_R.clone(), &v_Z.clone())?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_ac_mesh(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_I: Py = Py::None;
    let mut v_R: Py = Py::None;
    let mut v_Vd: Py = Py::None;
    let mut v_Vm: Py = Py::None;
    let mut v_X: Py = Py::None;
    let mut v_acc: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_o: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_sc: Py = Py::None;
    let mut v_t: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t74 = (Py::tuple(vec![py::getitem(&v_inputs.clone(), &Py::str("R"))?, py::getitem(&v_inputs.clone(), &Py::str("X"))?, py::getitem(&v_inputs.clone(), &Py::str("V_mag"))?, py::getitem(&v_inputs.clone(), &Py::str("V_deg"))?])).items()?;
    if t74.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_R = t74[0].clone();
    v_X = t74[1].clone();
    v_Vm = t74[2].clone();
    v_Vd = t74[3].clone();
    v_n = py::len(&v_R.clone())?;
    for t75 in py::iter(&(Py::tuple(vec![Py::str("currents_real"), Py::str("currents_imag"), Py::str("currents_mag"), Py::str("currents_deg")])))? {
        v_k = t75;
        if ({ let t80 = { let t76 = v_k.clone(); Py::Bool({ let t77 = v_outputs.clone(); if !py::contains(&t77, &t76)? { true } else { false } }) }; if t80.truthy() { t80 } else { { let t78 = py::len(&py::getitem(&v_outputs.clone(), &v_k.clone())?)?; Py::Bool({ let t79 = v_n.clone(); if py::ne(&t78, &t79) { true } else { false } }) } } }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
    }
    v_I = Py::list(vec![]);
    for t81 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_k = t81;
        let _ = py::method(&v_I.clone(), "append", &[py::complex(&[py::getitem(&py::getitem(&v_outputs.clone(), &Py::str("currents_real"))?, &v_k.clone())?, py::getitem(&py::getitem(&v_outputs.clone(), &Py::str("currents_imag"))?, &v_k.clone())?])?])?;
    }
    v_worst = Py::Float(0.0);
    for t82 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_r = t82;
        v_acc = py::complex(&[Py::Float(0.0), Py::Float(0.0)])?;
        v_sc = Py::Float(0.0);
        for t83 in py::range_iter(&[(v_n.clone()).i()?]) {
            v_c = t83;
            v_t = py::mul(&py::complex(&[py::getitem(&py::getitem(&v_R.clone(), &v_r.clone())?, &v_c.clone())?, py::getitem(&py::getitem(&v_X.clone(), &v_r.clone())?, &v_c.clone())?])?, &py::getitem(&v_I.clone(), &v_c.clone())?)?;
            v_acc = py::add(&v_acc.clone(), &v_t.clone())?;
            v_sc = py::iadd(&v_sc, &f__cabs(v_t.clone())?)?;
        }
        v_v = f__rect_deg(py::getitem(&v_Vm.clone(), &v_r.clone())?, py::getitem(&v_Vd.clone(), &v_r.clone())?)?;
        v_sc = py::iadd(&v_sc, &f__cabs(v_v.clone())?)?;
        if ({ let t84 = v_sc.clone(); Py::Bool({ let t85 = Py::Int(0i64); if py::gt(&t84, &t85)? { true } else { false } }) }).truthy() {
            v_worst = py::minmax(&[v_worst.clone(), py::truediv(&f__cabs(py::sub(&v_acc.clone(), &v_v.clone())?)?, &v_sc.clone())?], true)?;
        }
    }
    for t86 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_k = t86;
        v_o = Py::dict(vec![(Py::str("c_real"), py::getitem(&py::getitem(&v_outputs.clone(), &Py::str("currents_real"))?, &v_k.clone())?), (Py::str("c_imag"), py::getitem(&py::getitem(&v_outputs.clone(), &Py::str("currents_imag"))?, &v_k.clone())?), (Py::str("c_mag"), py::getitem(&py::getitem(&v_outputs.clone(), &Py::str("currents_mag"))?, &v_k.clone())?), (Py::str("c_deg"), py::getitem(&py::getitem(&v_outputs.clone(), &Py::str("currents_deg"))?, &v_k.clone())?)]);
        v_worst = py::minmax(&[v_worst.clone(), f__polar_err(v_o.clone(), Py::str("c"))?], true)?;
    }
    return Ok(py::add(&py::add(&v_worst.clone(), &f__rel(py::getitem(&v_outputs.clone(), &Py::str("first_mag"))?, py::getitem(&py::getitem(&v_outputs.clone(), &Py::str("currents_mag"))?, &Py::Int(0i64))?)?)?, &py::truediv(&py::abs(&py::sub(&py::getitem(&v_outputs.clone(), &Py::str("first_deg"))?, &py::getitem(&py::getitem(&v_outputs.clone(), &Py::str("currents_deg"))?, &Py::Int(0i64))?)?)?, &Py::Float(180.0))?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_ac_nodes(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_S: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_cur: Py = Py::None;
    let mut v_els: Py = Py::None;
    let mut v_imax: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_nodes: Py = Py::None;
    let mut v_o: Py = Py::None;
    let mut v_sc: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_vmax: Py = Py::None;
    let mut v_volts: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_els = f__ac_netlist(py::getitem(&v_inputs.clone(), &Py::str("netlist"))?)?;
    v_w = py::mul(&c__TWO_PI()?, &py::getitem(&v_inputs.clone(), &Py::str("f"))?)?;
    let t250 = (f__ac_mna(v_els.clone(), v_w.clone())?).items()?;
    if t250.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_volts = t250[0].clone();
    v_cur = t250[1].clone();
    for t251 in py::iter(&(Py::tuple(vec![Py::str("node_voltages_mag"), Py::str("node_voltages_deg"), Py::str("node_voltages_real"), Py::str("node_voltages_imag"), Py::str("source_currents_real"), Py::str("source_currents_imag"), Py::str("P_delivered"), Py::str("Q_delivered")])))? {
        v_k = t251;
        if ({ let t252 = v_k.clone(); Py::Bool({ let t253 = v_outputs.clone(); if !py::contains(&t253, &t252)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
    }
    v_nodes = { let mut out = vec![]; for t254 in py::iter(&(v_volts.clone()))? { let mut v_n: Py; 
v_n = t254;
if ({ let t255 = v_n.clone(); Py::Bool({ let t256 = Py::str("0"); if py::ne(&t255, &t256) { true } else { false } }) }).truthy() { out.push(v_n.clone()); }} Py::list(out) };
    if ({ let t261 = { let t257 = py::sorted(&py::getitem(&v_outputs.clone(), &Py::str("node_voltages_real"))?, (Py::Bool(false)).truthy())?; Py::Bool({ let t258 = py::sorted(&v_nodes.clone(), (Py::Bool(false)).truthy())?; if py::ne(&t257, &t258) { true } else { false } }) }; if t261.truthy() { t261 } else { { let t259 = py::sorted(&py::getitem(&v_outputs.clone(), &Py::str("source_currents_real"))?, (Py::Bool(false)).truthy())?; Py::Bool({ let t260 = py::sorted(&v_cur.clone(), (Py::Bool(false)).truthy())?; if py::ne(&t259, &t260) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_vmax = py::minmax(&[py::add(&{ let mut out = vec![]; for t262 in py::iter(&(py::method(&v_volts.clone(), "values", &[])?))? { let mut v_v: Py; 
v_v = t262;
out.push(f__cabs(v_v.clone())?); } Py::list(out) }, &Py::list(vec![Py::Float(1e-300)]))?], true)?;
    v_imax = py::minmax(&[py::add(&{ let mut out = vec![]; for t263 in py::iter(&(py::method(&v_cur.clone(), "values", &[])?))? { let mut v_c: Py; 
v_c = t263;
out.push(f__cabs(v_c.clone())?); } Py::list(out) }, &Py::list(vec![Py::Float(1e-300)]))?], true)?;
    v_worst = Py::Float(0.0);
    for t264 in py::iter(&(v_nodes.clone()))? {
        v_n = t264;
        v_v = py::getitem(&v_volts.clone(), &v_n.clone())?;
        v_o = Py::dict(vec![(Py::str("v_real"), py::getitem(&py::getitem(&v_outputs.clone(), &Py::str("node_voltages_real"))?, &v_n.clone())?), (Py::str("v_imag"), py::getitem(&py::getitem(&v_outputs.clone(), &Py::str("node_voltages_imag"))?, &v_n.clone())?), (Py::str("v_mag"), py::getitem(&py::getitem(&v_outputs.clone(), &Py::str("node_voltages_mag"))?, &v_n.clone())?), (Py::str("v_deg"), py::getitem(&py::getitem(&v_outputs.clone(), &Py::str("node_voltages_deg"))?, &v_n.clone())?)]);
        v_worst = py::minmax(&[v_worst.clone(), py::truediv(&py::add(&py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("v_real"))?, &py::real(&v_v.clone())?)?)?, &py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("v_imag"))?, &py::imag(&v_v.clone())?)?)?)?, &v_vmax.clone())?], true)?;
        if ({ let t265 = f__cabs(v_v.clone())?; Py::Bool({ let t266 = py::mul(&Py::Float(1e-09), &v_vmax.clone())?; if py::gt(&t265, &t266)? { true } else { false } }) }).truthy() {
            v_worst = py::minmax(&[v_worst.clone(), f__polar_err(v_o.clone(), Py::str("v"))?], true)?;
        }
    }
    for t269 in py::iter(&(py::method(&v_cur.clone(), "items", &[])?))? {
        let t270 = (t269).items()?;
        if t270.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t270[0].clone();
        v_c = t270[1].clone();
        v_worst = py::minmax(&[v_worst.clone(), py::truediv(&py::add(&py::abs(&py::sub(&py::getitem(&py::getitem(&v_outputs.clone(), &Py::str("source_currents_real"))?, &v_k.clone())?, &py::real(&v_c.clone())?)?)?, &py::abs(&py::sub(&py::getitem(&py::getitem(&v_outputs.clone(), &Py::str("source_currents_imag"))?, &v_k.clone())?, &py::imag(&v_c.clone())?)?)?)?, &v_imax.clone())?], true)?;
    }
    v_S = f__ac_absorbed_power(v_els.clone(), v_volts.clone(), v_w.clone())?;
    v_sc = py::minmax(&[py::abs(&py::real(&v_S.clone())?)?, py::abs(&py::imag(&v_S.clone())?)?, py::abs(&py::getitem(&v_outputs.clone(), &Py::str("P_delivered"))?)?, py::abs(&py::getitem(&v_outputs.clone(), &Py::str("Q_delivered"))?)?, Py::Float(1e-300)], true)?;
    return Ok(py::add(&v_worst.clone(), &py::truediv(&py::add(&py::abs(&py::sub(&py::getitem(&v_outputs.clone(), &Py::str("P_delivered"))?, &py::real(&v_S.clone())?)?)?, &py::abs(&py::sub(&py::getitem(&v_outputs.clone(), &Py::str("Q_delivered"))?, &py::imag(&v_S.clone())?)?)?)?, &v_sc.clone())?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_ac_norton(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_vth: Py = Py::None;
    let mut v_zn: Py = Py::None;
    for t443 in py::iter(&(Py::tuple(vec![Py::str("In"), Py::str("Zn")])))? {
        v_k = t443;
        for t444 in py::iter(&(Py::tuple(vec![Py::str("_real"), Py::str("_imag"), Py::str("_mag"), Py::str("_deg")])))? {
            v_s = t444;
            if ({ let t445 = py::add(&v_k.clone(), &v_s.clone())?; Py::Bool({ let t446 = v_outputs.clone(); if !py::contains(&t446, &t445)? { true } else { false } }) }).truthy() {
                return Ok(Py::Float(f64::INFINITY));
            }
        }
    }
    v_zn = py::complex(&[py::getitem(&v_outputs.clone(), &Py::str("Zn_real"))?, py::getitem(&v_outputs.clone(), &Py::str("Zn_imag"))?])?;
    v_vth = py::mul(&py::complex(&[py::getitem(&v_outputs.clone(), &Py::str("In_real"))?, py::getitem(&v_outputs.clone(), &Py::str("In_imag"))?])?, &v_zn.clone())?;
    return Ok(py::add(&py::add(&f__ac_port_residual(v_inputs.clone(), v_vth.clone(), v_zn.clone())?, &f__polar_err(v_outputs.clone(), Py::str("In"))?)?, &f__polar_err(v_outputs.clone(), Py::str("Zn"))?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_ac_thevenin(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_vth: Py = Py::None;
    let mut v_zth: Py = Py::None;
    for t616 in py::iter(&(Py::tuple(vec![Py::str("Vth"), Py::str("Zth")])))? {
        v_k = t616;
        for t617 in py::iter(&(Py::tuple(vec![Py::str("_real"), Py::str("_imag"), Py::str("_mag"), Py::str("_deg")])))? {
            v_s = t617;
            if ({ let t618 = py::add(&v_k.clone(), &v_s.clone())?; Py::Bool({ let t619 = v_outputs.clone(); if !py::contains(&t619, &t618)? { true } else { false } }) }).truthy() {
                return Ok(Py::Float(f64::INFINITY));
            }
        }
    }
    v_vth = py::complex(&[py::getitem(&v_outputs.clone(), &Py::str("Vth_real"))?, py::getitem(&v_outputs.clone(), &Py::str("Vth_imag"))?])?;
    v_zth = py::complex(&[py::getitem(&v_outputs.clone(), &Py::str("Zth_real"))?, py::getitem(&v_outputs.clone(), &Py::str("Zth_imag"))?])?;
    return Ok(py::add(&py::add(&f__ac_port_residual(v_inputs.clone(), v_vth.clone(), v_zth.clone())?, &f__polar_err(v_outputs.clone(), Py::str("Vth"))?)?, &f__polar_err(v_outputs.clone(), Py::str("Zth"))?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_ammeter(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_I: Py = Py::None;
    let mut v_Ifs: Py = Py::None;
    let mut v_Rm: Py = Py::None;
    let t655 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("Ifs"))?, py::getitem(&v_i.clone(), &Py::str("Rm"))?, py::getitem(&v_i.clone(), &Py::str("I"))?])).items()?;
    if t655.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_Ifs = t655[0].clone();
    v_Rm = t655[1].clone();
    v_I = t655[2].clone();
    return Ok(py::add(&py::add(&py::add(&f__rel(py::mul(&v_Ifs.clone(), &v_Rm.clone())?, py::mul(&py::sub(&v_I.clone(), &v_Ifs.clone())?, &py::getitem(&v_o.clone(), &Py::str("Rsh"))?)?)?, &f__rel(py::mul(&py::getitem(&v_o.clone(), &Py::str("multiplying_factor"))?, &v_Ifs.clone())?, v_I.clone())?)?, &f__rel(py::getitem(&v_o.clone(), &Py::str("V_fs"))?, py::mul(&v_Ifs.clone(), &v_Rm.clone())?)?)?, &f__rel(py::mul(&py::getitem(&v_o.clone(), &Py::str("R_total"))?, &py::add(&v_Rm.clone(), &py::getitem(&v_o.clone(), &Py::str("Rsh"))?)?)?, py::mul(&v_Rm.clone(), &py::getitem(&v_o.clone(), &Py::str("Rsh"))?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_base(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_b: Py = Py::None;
    let mut v_n: Py = Py::None;
    v_b = py::getitem(&c__BASES()?, &Py::str(&py::to_str(&py::getitem(&v_inputs.clone(), &Py::str("base"))?)))?;
    v_n = py::int_base(&f__clean_digits(py::getitem(&v_inputs.clone(), &Py::str("value"))?, v_b.clone())?, &v_b.clone())?;
    return Ok(py::add(&py::add(&py::add(&py::truediv(&py::abs(&py::sub(&py::getitem(&v_outputs.clone(), &Py::str("decimal"))?, &v_n.clone())?)?, &py::minmax(&[Py::Int(1i64), v_n.clone()], true)?)?, &{ let t666 = py::int_base(&py::getitem(&v_outputs.clone(), &Py::str("binary"))?, &Py::Int(2i64))?; Py::Bool({ let t667 = v_n.clone(); if py::ne(&t666, &t667) { true } else { false } }) })?, &{ let t668 = py::int_base(&py::getitem(&v_outputs.clone(), &Py::str("octal"))?, &Py::Int(8i64))?; Py::Bool({ let t669 = v_n.clone(); if py::ne(&t668, &t669) { true } else { false } }) })?, &{ let t670 = py::int_base(&py::getitem(&v_outputs.clone(), &Py::str("hexadecimal"))?, &Py::Int(16i64))?; Py::Bool({ let t671 = v_n.clone(); if py::ne(&t670, &t671) { true } else { false } }) })?);
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
    let t683 = ({ let mut out = vec![]; for t682 in py::iter(&(Py::tuple(vec![Py::str("VCC"), Py::str("R1"), Py::str("R2"), Py::str("RC"), Py::str("RE"), Py::str("beta"), Py::str("VBE")])))? { let mut v_k: Py; 
v_k = t682;
out.push(py::getitem(&v_inputs.clone(), &v_k.clone())?); } Py::list(out) }).items()?;
    if t683.len() != 7 { return Err(PyErr::Value("unpack".into())); }
    v_VCC = t683[0].clone();
    v_R1 = t683[1].clone();
    v_R2 = t683[2].clone();
    v_RC = t683[3].clone();
    v_RE = t683[4].clone();
    v_beta = t683[5].clone();
    v_VBE = t683[6].clone();
    let t684 = (Py::tuple(vec![py::getitem(&v_outputs.clone(), &Py::str("IB"))?, py::getitem(&v_outputs.clone(), &Py::str("VB"))?, py::getitem(&v_outputs.clone(), &Py::str("VE"))?, py::getitem(&v_outputs.clone(), &Py::str("VC"))?])).items()?;
    if t684.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_ib = t684[0].clone();
    v_vb = t684[1].clone();
    v_ve = t684[2].clone();
    v_vc = t684[3].clone();
    let t685 = (Py::tuple(vec![py::truediv(&py::sub(&v_VCC.clone(), &v_vb.clone())?, &v_R1.clone())?, py::truediv(&v_vb.clone(), &v_R2.clone())?])).items()?;
    if t685.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_i1 = t685[0].clone();
    v_i2 = t685[1].clone();
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
    let t704 = (Py::tuple(vec![f__ptrim(py::getitem(&v_inputs.clone(), &Py::str("num"))?)?, f__ptrim(py::getitem(&v_inputs.clone(), &Py::str("den"))?)?])).items()?;
    if t704.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_num = t704[0].clone();
    v_den = t704[1].clone();
    v_g = f__G(v_num.clone(), v_den.clone(), py::getitem(&v_inputs.clone(), &Py::str("w"))?)?;
    v_d = py::sub(&py::math1("degrees", &py::cmath("phase", &[v_g.clone()])?)?, &py::getitem(&v_outputs.clone(), &Py::str("phase_deg"))?)?;
    v_d = py::sub(&py::modulo(&py::add(&v_d.clone(), &Py::Int(180i64))?, &Py::Int(360i64))?, &Py::Int(180i64))?;
    return Ok(py::add(&py::add(&py::add(&f__rel(py::getitem(&v_outputs.clone(), &Py::str("magnitude"))?, py::abs(&v_g.clone())?)?, &py::truediv(&py::abs(&py::sub(&py::getitem(&v_outputs.clone(), &Py::str("magnitude_db"))?, &py::mul(&Py::Int(20i64), &py::math1("log10", &py::abs(&v_g.clone())?)?)?)?)?, &py::minmax(&[Py::Float(1.0), py::abs(&py::getitem(&v_outputs.clone(), &Py::str("magnitude_db"))?)?], true)?)?)?, &py::truediv(&py::abs(&v_d.clone())?, &Py::Int(180i64))?)?, &py::truediv(&py::abs(&py::sub(&py::complex(&[py::method(&v_outputs.clone(), "get", &[Py::str("real"), py::real(&v_g.clone())?])?, py::method(&v_outputs.clone(), "get", &[Py::str("imag"), py::imag(&v_g.clone())?])?])?, &v_g.clone())?)?, &py::abs(&v_g.clone())?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_cells(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_E: Py = Py::None;
    let mut v_RL: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_want_e: Py = Py::None;
    let mut v_want_r: Py = Py::None;
    let t844 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("E"))?, py::getitem(&v_i.clone(), &Py::str("r"))?, py::getitem(&v_i.clone(), &Py::str("n"))?, py::getitem(&v_i.clone(), &Py::str("RL"))?])).items()?;
    if t844.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_E = t844[0].clone();
    v_r = t844[1].clone();
    v_n = t844[2].clone();
    v_RL = t844[3].clone();
    v_want_e = (if ({ let t845 = py::getitem(&v_i.clone(), &Py::str("connection"))?; Py::Bool({ let t846 = Py::str("series"); if py::eq(&t845, &t846) { true } else { false } }) }).truthy() { py::mul(&v_n.clone(), &v_E.clone())? } else { v_E.clone() });
    v_want_r = (if ({ let t847 = py::getitem(&v_i.clone(), &Py::str("connection"))?; Py::Bool({ let t848 = Py::str("series"); if py::eq(&t847, &t848) { true } else { false } }) }).truthy() { py::mul(&v_n.clone(), &v_r.clone())? } else { py::truediv(&v_r.clone(), &v_n.clone())? });
    return Ok(py::add(&py::add(&py::add(&py::add(&py::add(&f__rel(py::getitem(&v_o.clone(), &Py::str("emf_total"))?, v_want_e.clone())?, &f__rel(py::getitem(&v_o.clone(), &Py::str("r_total"))?, v_want_r.clone())?)?, &py::truediv(&py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("emf_total"))?, &py::mul(&py::getitem(&v_o.clone(), &Py::str("I"))?, &py::add(&py::getitem(&v_o.clone(), &Py::str("r_total"))?, &v_RL.clone())?)?)?)?, &py::getitem(&v_o.clone(), &Py::str("emf_total"))?)?)?, &f__rel(py::getitem(&v_o.clone(), &Py::str("V_terminal"))?, py::mul(&py::getitem(&v_o.clone(), &Py::str("I"))?, &v_RL.clone())?)?)?, &py::truediv(&py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("lost_volts"))?, &py::mul(&py::getitem(&v_o.clone(), &Py::str("I"))?, &py::getitem(&v_o.clone(), &Py::str("r_total"))?)?)?)?, &py::getitem(&v_o.clone(), &Py::str("emf_total"))?)?)?, &f__rel(py::getitem(&v_o.clone(), &Py::str("P_load"))?, py::mul(&py::getitem(&v_o.clone(), &Py::str("I"))?, &py::getitem(&v_o.clone(), &Py::str("V_terminal"))?)?)?)?);
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
    let t1012 = (f__ctrb_obsv(py::getitem(&v_inputs.clone(), &Py::str("A"))?, py::getitem(&v_inputs.clone(), &Py::str("B"))?, py::getitem(&v_inputs.clone(), &Py::str("C"))?)?).items()?;
    if t1012.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_Wc = t1012[0].clone();
    v_Wo = t1012[1].clone();
    v_n = py::len(&py::getitem(&v_inputs.clone(), &Py::str("A"))?)?;
    let t1015 = (Py::tuple(vec![f__rank({ let mut out = vec![]; for t1013 in py::iter(&(py::zip(&{ let mut v: Vec<Py> = vec![]; v.extend((v_Wc.clone()).items()?); v })?))? { let mut v_x: Py; 
v_x = t1013;
out.push(Py::list((v_x.clone()).items()?)); } Py::list(out) }, Py::Float(1e-09))?, f__rank({ let mut out = vec![]; for t1014 in py::iter(&(py::zip(&{ let mut v: Vec<Py> = vec![]; v.extend((v_Wo.clone()).items()?); v })?))? { let mut v_x: Py; 
v_x = t1014;
out.push(Py::list((v_x.clone()).items()?)); } Py::list(out) }, Py::Float(1e-09))?])).items()?;
    if t1015.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_rc = t1015[0].clone();
    v_ro = t1015[1].clone();
    return Ok(py::add(&py::add(&py::add(&py::abs(&py::sub(&py::getitem(&v_outputs.clone(), &Py::str("controllability_rank"))?, &v_rc.clone())?)?, &py::abs(&py::sub(&py::getitem(&v_outputs.clone(), &Py::str("observability_rank"))?, &v_ro.clone())?)?)?, &{ let t1016 = py::getitem(&v_outputs.clone(), &Py::str("controllable"))?; Py::Bool({ let t1017 = { let t1018 = v_rc.clone(); Py::Bool({ let t1019 = v_n.clone(); if py::eq(&t1018, &t1019) { true } else { false } }) }; if py::ne(&t1016, &t1017) { true } else { false } }) })?, &{ let t1020 = py::getitem(&v_outputs.clone(), &Py::str("observable"))?; Py::Bool({ let t1021 = { let t1022 = v_ro.clone(); Py::Bool({ let t1023 = v_n.clone(); if py::eq(&t1022, &t1023) { true } else { false } }) }; if py::ne(&t1020, &t1021) { true } else { false } }) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_coil_cap(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_C: Py = Py::None;
    let mut v_L: Py = Py::None;
    let mut v_R: Py = Py::None;
    let mut v_V: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_dr: Py = Py::None;
    let mut v_im_: Py = Py::None;
    let mut v_re_: Py = Py::None;
    let mut v_res: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_wr: Py = Py::None;
    let t950 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("V"))?, py::getitem(&v_i.clone(), &Py::str("R"))?, py::getitem(&v_i.clone(), &Py::str("L"))?, py::getitem(&v_i.clone(), &Py::str("C"))?])).items()?;
    if t950.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_V = t950[0].clone();
    v_R = t950[1].clone();
    v_L = t950[2].clone();
    v_C = t950[3].clone();
    v_w = py::mul(&c__TWO_PI()?, &py::getitem(&v_i.clone(), &Py::str("f"))?)?;
    v_d = py::add(&py::mul(&v_R.clone(), &v_R.clone())?, &py::pow(&py::mul(&v_w.clone(), &v_L.clone())?, &Py::Int(2i64))?)?;
    let t951 = (Py::tuple(vec![py::truediv(&py::mul(&v_V.clone(), &v_R.clone())?, &v_d.clone())?, py::add(&py::truediv(&py::mul(&py::mul(&py::neg(&v_V.clone())?, &v_w.clone())?, &v_L.clone())?, &v_d.clone())?, &py::mul(&py::mul(&v_V.clone(), &v_w.clone())?, &v_C.clone())?)?])).items()?;
    if t951.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_re_ = t951[0].clone();
    v_im_ = t951[1].clone();
    v_res = py::add(&py::add(&py::add(&py::add(&py::add(&f__rel(py::getitem(&v_o.clone(), &Py::str("I_coil"))?, py::truediv(&v_V.clone(), &py::math1("sqrt", &v_d.clone())?)?)?, &f__rel(py::getitem(&v_o.clone(), &Py::str("I_C"))?, py::mul(&py::mul(&v_V.clone(), &v_w.clone())?, &v_C.clone())?)?)?, &f__rel(py::getitem(&v_o.clone(), &Py::str("I"))?, py::hypot(&[v_re_.clone(), v_im_.clone()])?)?)?, &f__rel(py::mul(&py::getitem(&v_o.clone(), &Py::str("Z"))?, &py::getitem(&v_o.clone(), &Py::str("I"))?)?, v_V.clone())?)?, &py::truediv(&py::abs(&py::sub(&py::math1("radians", &py::getitem(&v_o.clone(), &Py::str("phase_deg"))?)?, &py::atan2(&py::neg(&v_im_.clone())?, &v_re_.clone())?)?)?, &Py::Float(std::f64::consts::PI))?)?, &f__rel(py::getitem(&v_o.clone(), &Py::str("P"))?, py::mul(&v_V.clone(), &v_re_.clone())?)?)?;
    v_wr = py::mul(&c__TWO_PI()?, &py::getitem(&v_o.clone(), &Py::str("f_r"))?)?;
    v_dr = py::add(&py::mul(&v_R.clone(), &v_R.clone())?, &py::pow(&py::mul(&v_wr.clone(), &v_L.clone())?, &Py::Int(2i64))?)?;
    return Ok(py::add(&py::add(&py::add(&py::add(&v_res.clone(), &py::truediv(&py::abs(&py::sub(&py::mul(&v_wr.clone(), &v_C.clone())?, &py::truediv(&py::mul(&v_wr.clone(), &v_L.clone())?, &v_dr.clone())?)?)?, &py::mul(&v_wr.clone(), &v_C.clone())?)?)?, &f__rel(py::getitem(&v_o.clone(), &Py::str("R_dynamic"))?, py::truediv(&v_dr.clone(), &v_R.clone())?)?)?, &f__rel(py::mul(&py::getitem(&v_o.clone(), &Py::str("I_resonance"))?, &py::getitem(&v_o.clone(), &Py::str("R_dynamic"))?)?, v_V.clone())?)?, &f__rel(py::mul(&py::getitem(&v_o.clone(), &Py::str("Q"))?, &v_R.clone())?, py::mul(&v_wr.clone(), &v_L.clone())?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_coil_l(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::add(&py::add(&f__rel(py::mul(&py::getitem(&v_o.clone(), &Py::str("L"))?, &py::getitem(&v_o.clone(), &Py::str("reluctance"))?)?, py::mul(&py::getitem(&v_i.clone(), &Py::str("N"))?, &py::getitem(&v_i.clone(), &Py::str("N"))?)?)?, &f__rel(py::mul(&py::mul(&py::mul(&py::getitem(&v_o.clone(), &Py::str("reluctance"))?, &c__MU0()?)?, &py::getitem(&v_i.clone(), &Py::str("mu_r"))?)?, &py::getitem(&v_i.clone(), &Py::str("area"))?)?, py::getitem(&v_i.clone(), &Py::str("length"))?)?)?, &f__rel(py::mul(&py::getitem(&v_o.clone(), &Py::str("flux_per_ampere"))?, &py::getitem(&v_i.clone(), &Py::str("N"))?)?, py::getitem(&v_o.clone(), &Py::str("L"))?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_cpower(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_I: Py = Py::None;
    let mut v_S: Py = Py::None;
    let mut v_V: Py = Py::None;
    let mut v_th: Py = Py::None;
    let t969 = (Py::tuple(vec![py::getitem(&v_inputs.clone(), &Py::str("V"))?, py::getitem(&v_inputs.clone(), &Py::str("I"))?])).items()?;
    if t969.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_V = t969[0].clone();
    v_I = t969[1].clone();
    v_th = py::math1("radians", &py::sub(&py::getitem(&v_inputs.clone(), &Py::str("V_phase"))?, &py::getitem(&v_inputs.clone(), &Py::str("I_phase"))?)?)?;
    v_S = py::mul(&v_V.clone(), &v_I.clone())?;
    return Ok(py::add(&py::add(&py::add(&py::add(&py::truediv(&py::abs(&py::sub(&py::getitem(&v_outputs.clone(), &Py::str("P"))?, &py::mul(&v_S.clone(), &py::math1("cos", &v_th.clone())?)?)?)?, &v_S.clone())?, &py::truediv(&py::abs(&py::sub(&py::getitem(&v_outputs.clone(), &Py::str("Q"))?, &py::mul(&v_S.clone(), &py::math1("sin", &v_th.clone())?)?)?)?, &v_S.clone())?)?, &f__rel(py::getitem(&v_outputs.clone(), &Py::str("S"))?, v_S.clone())?)?, &py::abs(&py::sub(&py::getitem(&v_outputs.clone(), &Py::str("power_factor"))?, &py::truediv(&py::getitem(&v_outputs.clone(), &Py::str("P"))?, &py::getitem(&v_outputs.clone(), &Py::str("S"))?)?)?)?)?, &py::truediv(&py::abs(&py::sub(&py::add(&py::pow(&py::getitem(&v_outputs.clone(), &Py::str("P"))?, &Py::Int(2i64))?, &py::pow(&py::getitem(&v_outputs.clone(), &Py::str("Q"))?, &Py::Int(2i64))?)?, &py::pow(&py::getitem(&v_outputs.clone(), &Py::str("S"))?, &Py::Int(2i64))?)?)?, &py::pow(&v_S.clone(), &Py::Int(2i64))?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_dc_emf(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_poles: Py = Py::None;
    let mut v_want: Py = Py::None;
    v_poles = py::getitem(&v_i.clone(), &Py::str("poles"))?;
    v_want = (if ({ let t1048 = py::getitem(&v_i.clone(), &Py::str("winding"))?; Py::Bool({ let t1049 = Py::str("lap"); if py::eq(&t1048, &t1049) { true } else { false } }) }).truthy() { v_poles.clone() } else { Py::Float(2.0) });
    return Ok(py::add(&py::add(&f__rel(py::getitem(&v_o.clone(), &Py::str("conductor_emf"))?, py::mul(&py::mul(&v_poles.clone(), &py::getitem(&v_i.clone(), &Py::str("flux"))?)?, &py::getitem(&v_i.clone(), &Py::str("n"))?)?)?, &f__rel(py::mul(&py::getitem(&v_o.clone(), &Py::str("E"))?, &py::getitem(&v_o.clone(), &Py::str("paths"))?)?, py::mul(&py::getitem(&v_o.clone(), &Py::str("conductor_emf"))?, &py::getitem(&v_i.clone(), &Py::str("Z"))?)?)?)?, &py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("paths"))?, &v_want.clone())?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_dc_gen(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_IL: Py = Py::None;
    let mut v_Ra: Py = Py::None;
    let mut v_Rf: Py = Py::None;
    let mut v_V: Py = Py::None;
    let mut v_pg: Py = Py::None;
    let t1233 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("V"))?, py::getitem(&v_i.clone(), &Py::str("IL"))?, py::getitem(&v_i.clone(), &Py::str("Ra"))?, py::getitem(&v_i.clone(), &Py::str("Rf"))?])).items()?;
    if t1233.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_V = t1233[0].clone();
    v_IL = t1233[1].clone();
    v_Ra = t1233[2].clone();
    v_Rf = t1233[3].clone();
    v_pg = py::getitem(&v_o.clone(), &Py::str("P_generated"))?;
    return Ok(py::add(&py::add(&py::add(&py::add(&py::add(&py::add(&f__rel(py::mul(&py::getitem(&v_o.clone(), &Py::str("If"))?, &v_Rf.clone())?, v_V.clone())?, &f__rel(py::getitem(&v_o.clone(), &Py::str("Ia"))?, py::add(&v_IL.clone(), &py::getitem(&v_o.clone(), &Py::str("If"))?)?)?)?, &f__rel(py::getitem(&v_o.clone(), &Py::str("E"))?, py::add(&v_V.clone(), &py::mul(&py::getitem(&v_o.clone(), &Py::str("Ia"))?, &v_Ra.clone())?)?)?)?, &f__rel(v_pg.clone(), py::mul(&py::getitem(&v_o.clone(), &Py::str("E"))?, &py::getitem(&v_o.clone(), &Py::str("Ia"))?)?)?)?, &f__rel(v_pg.clone(), py::add(&py::getitem(&v_o.clone(), &Py::str("P_out"))?, &py::getitem(&v_o.clone(), &Py::str("copper_loss"))?)?)?)?, &py::truediv(&py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("P_out"))?, &py::mul(&v_V.clone(), &v_IL.clone())?)?)?, &v_pg.clone())?)?, &py::truediv(&py::abs(&py::sub(&py::sub(&py::getitem(&v_o.clone(), &Py::str("copper_loss"))?, &py::mul(&py::pow(&py::getitem(&v_o.clone(), &Py::str("Ia"))?, &Py::Int(2i64))?, &v_Ra.clone())?)?, &py::mul(&py::pow(&py::getitem(&v_o.clone(), &Py::str("If"))?, &Py::Int(2i64))?, &v_Rf.clone())?)?)?, &v_pg.clone())?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_dc_motor(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_I: Py = Py::None;
    let mut v_Ra: Py = Py::None;
    let mut v_Rf: Py = Py::None;
    let mut v_V: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_pc: Py = Py::None;
    let t1248 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("V"))?, py::getitem(&v_i.clone(), &Py::str("I"))?, py::getitem(&v_i.clone(), &Py::str("Ra"))?, py::getitem(&v_i.clone(), &Py::str("Rf"))?, py::getitem(&v_i.clone(), &Py::str("n"))?])).items()?;
    if t1248.len() != 5 { return Err(PyErr::Value("unpack".into())); }
    v_V = t1248[0].clone();
    v_I = t1248[1].clone();
    v_Ra = t1248[2].clone();
    v_Rf = t1248[3].clone();
    v_n = t1248[4].clone();
    v_pc = py::getitem(&v_o.clone(), &Py::str("P_converted"))?;
    return Ok(py::add(&py::add(&py::add(&py::add(&py::add(&py::add(&f__rel(py::mul(&py::getitem(&v_o.clone(), &Py::str("If"))?, &v_Rf.clone())?, v_V.clone())?, &f__rel(py::add(&py::getitem(&v_o.clone(), &Py::str("If"))?, &py::getitem(&v_o.clone(), &Py::str("Ia"))?)?, v_I.clone())?)?, &f__rel(py::add(&py::getitem(&v_o.clone(), &Py::str("E"))?, &py::mul(&py::getitem(&v_o.clone(), &Py::str("Ia"))?, &v_Ra.clone())?)?, v_V.clone())?)?, &f__rel(v_pc.clone(), py::mul(&py::getitem(&v_o.clone(), &Py::str("E"))?, &py::getitem(&v_o.clone(), &Py::str("Ia"))?)?)?)?, &f__rel(py::mul(&py::mul(&py::getitem(&v_o.clone(), &Py::str("torque"))?, &c__TWO_PI()?)?, &v_n.clone())?, v_pc.clone())?)?, &py::truediv(&py::abs(&py::add(&py::sub(&py::getitem(&v_o.clone(), &Py::str("P_out"))?, &v_pc.clone())?, &py::method(&v_i.clone(), "get", &[Py::str("losses"), Py::Float(0.0)])?)?)?, &v_pc.clone())?)?, &py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("efficiency"))?, &py::truediv(&py::getitem(&v_o.clone(), &Py::str("P_out"))?, &py::mul(&v_V.clone(), &v_I.clone())?)?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_dc_speed(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_Ra: Py = Py::None;
    let mut v_V: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_n1: Py = Py::None;
    let t1077 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("V"))?, py::getitem(&v_i.clone(), &Py::str("Ra"))?, py::getitem(&v_i.clone(), &Py::str("n1"))?])).items()?;
    if t1077.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_V = t1077[0].clone();
    v_Ra = t1077[1].clone();
    v_n1 = t1077[2].clone();
    v_k = py::method(&v_i.clone(), "get", &[Py::str("flux_ratio"), Py::Float(1.0)])?;
    return Ok(py::add(&py::add(&py::add(&py::add(&f__rel(py::add(&py::getitem(&v_o.clone(), &Py::str("E1"))?, &py::mul(&py::getitem(&v_i.clone(), &Py::str("Ia1"))?, &v_Ra.clone())?)?, v_V.clone())?, &f__rel(py::add(&py::getitem(&v_o.clone(), &Py::str("E2"))?, &py::mul(&py::getitem(&v_i.clone(), &Py::str("Ia2"))?, &v_Ra.clone())?)?, v_V.clone())?)?, &f__rel(py::mul(&py::getitem(&v_o.clone(), &Py::str("E2"))?, &v_n1.clone())?, py::mul(&py::mul(&py::getitem(&v_o.clone(), &Py::str("E1"))?, &v_k.clone())?, &py::getitem(&v_o.clone(), &Py::str("n2"))?)?)?)?, &f__rel(py::getitem(&v_o.clone(), &Py::str("n2_rpm"))?, py::mul(&Py::Float(60.0), &py::getitem(&v_o.clone(), &Py::str("n2"))?)?)?)?, &py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("speed_change"))?, &py::sub(&py::truediv(&py::getitem(&v_o.clone(), &Py::str("n2"))?, &v_n1.clone())?, &Py::Float(1.0))?)?)?)?);
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
    let t1264 = ({ let mut out = vec![]; for t1263 in py::iter(&(Py::tuple(vec![Py::str("V1"), Py::str("V2"), Py::str("R1"), Py::str("R2"), Py::str("R3"), Py::str("R4")])))? { let mut v_k: Py; 
v_k = t1263;
out.push(py::getitem(&v_inputs.clone(), &v_k.clone())?); } Py::list(out) }).items()?;
    if t1264.len() != 6 { return Err(PyErr::Value("unpack".into())); }
    v_V1 = t1264[0].clone();
    v_V2 = t1264[1].clone();
    v_R1 = t1264[2].clone();
    v_R2 = t1264[3].clone();
    v_R3 = t1264[4].clone();
    v_R4 = t1264[5].clone();
    v_vo = py::getitem(&v_outputs.clone(), &Py::str("Vout"))?;
    v_vp = py::truediv(&py::mul(&v_V2.clone(), &v_R4.clone())?, &py::add(&v_R3.clone(), &v_R4.clone())?)?;
    v_terms = Py::list(vec![py::truediv(&py::sub(&v_V1.clone(), &v_vp.clone())?, &v_R1.clone())?, py::truediv(&py::sub(&v_vo.clone(), &v_vp.clone())?, &v_R2.clone())?]);
    return Ok(py::add(&py::truediv(&py::abs(&py::fsum(&v_terms.clone())?)?, &{ let t1266 = py::fsum(&{ let mut out = vec![]; for t1265 in py::iter(&(v_terms.clone()))? { let mut v_t: Py; 
v_t = t1265;
out.push(py::abs(&v_t.clone())?); } Py::list(out) })?; if t1266.truthy() { t1266 } else { Py::Float(1.0) } })?, &f__rel(py::getitem(&v_outputs.clone(), &Py::str("gain_inverting"))?, py::truediv(&py::neg(&v_R2.clone())?, &v_R1.clone())?)?)?);
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
        let t3157 = (Py::tuple(vec![Py::tuple(vec![py::getitem(&v_inputs.clone(), &Py::str("Rab"))?, py::getitem(&v_inputs.clone(), &Py::str("Rbc"))?, py::getitem(&v_inputs.clone(), &Py::str("Rca"))?]), Py::tuple(vec![py::getitem(&v_outputs.clone(), &Py::str("Ra"))?, py::getitem(&v_outputs.clone(), &Py::str("Rb"))?, py::getitem(&v_outputs.clone(), &Py::str("Rc"))?])])).items()?;
        if t3157.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_d = t3157[0].clone();
        v_y = t3157[1].clone();
    } else {
        let t3158 = (Py::tuple(vec![Py::tuple(vec![py::getitem(&v_inputs.clone(), &Py::str("Ra"))?, py::getitem(&v_inputs.clone(), &Py::str("Rb"))?, py::getitem(&v_inputs.clone(), &Py::str("Rc"))?]), Py::tuple(vec![py::getitem(&v_outputs.clone(), &Py::str("Rab"))?, py::getitem(&v_outputs.clone(), &Py::str("Rbc"))?, py::getitem(&v_outputs.clone(), &Py::str("Rca"))?])])).items()?;
        if t3158.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_y = t3158[0].clone();
        v_d = t3158[1].clone();
    }
    v_pd = { let mut v: Vec<Py> = vec![]; v.extend((v_d.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![];  let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "Rab") { Some(x) => x, None => { return Err(PyErr::Type("_pair_resistances_delta() missing argument Rab".into())) } } }; let a1 = if 1 < v.len() { v[1].clone() } else { match py::kw_take(&mut d, "Rbc") { Some(x) => x, None => { return Err(PyErr::Type("_pair_resistances_delta() missing argument Rbc".into())) } } }; let a2 = if 2 < v.len() { v[2].clone() } else { match py::kw_take(&mut d, "Rca") { Some(x) => x, None => { return Err(PyErr::Type("_pair_resistances_delta() missing argument Rca".into())) } } }; if v.len() > 3 { return Err(PyErr::Type("_pair_resistances_delta() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("_pair_resistances_delta() got an unexpected keyword argument".into())) } f__pair_resistances_delta(a0, a1, a2)? };
    v_py = Py::dict(vec![(Py::str("ab"), py::add(&py::getitem(&v_y.clone(), &Py::Int(0i64))?, &py::getitem(&v_y.clone(), &Py::Int(1i64))?)?), (Py::str("bc"), py::add(&py::getitem(&v_y.clone(), &Py::Int(1i64))?, &py::getitem(&v_y.clone(), &Py::Int(2i64))?)?), (Py::str("ca"), py::add(&py::getitem(&v_y.clone(), &Py::Int(2i64))?, &py::getitem(&v_y.clone(), &Py::Int(0i64))?)?)]);
    return Ok(py::minmax(&[{ let mut out = vec![]; for t3159 in py::iter(&(v_pd.clone()))? { let mut v_k: Py; 
v_k = t3159;
out.push(f__rel(py::getitem(&v_pd.clone(), &v_k.clone())?, py::getitem(&v_py.clone(), &v_k.clone())?)?); } Py::list(out) }], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_energy_cost(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::add(&py::add(&f__rel(py::getitem(&v_o.clone(), &Py::str("energy"))?, py::mul(&py::getitem(&v_i.clone(), &Py::str("P"))?, &py::getitem(&v_i.clone(), &Py::str("t"))?)?)?, &f__rel(py::mul(&py::getitem(&v_o.clone(), &Py::str("energy_kWh"))?, &Py::Float(3600000.0))?, py::getitem(&v_o.clone(), &Py::str("energy"))?)?)?, &py::truediv(&py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("cost"))?, &py::mul(&py::getitem(&v_o.clone(), &Py::str("energy_kWh"))?, &py::getitem(&v_i.clone(), &Py::str("tariff"))?)?)?)?, &py::minmax(&[py::abs(&py::getitem(&v_o.clone(), &Py::str("cost"))?)?, py::mul(&py::getitem(&v_o.clone(), &Py::str("energy_kWh"))?, &py::getitem(&v_i.clone(), &Py::str("tariff"))?)?, Py::Float(1e-300)], true)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_equiv(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_env: Py = Py::None;
    let mut v_t1: Py = Py::None;
    let mut v_t2: Py = Py::None;
    let t765 = (Py::tuple(vec![f__bool_parse(py::getitem(&v_inputs.clone(), &Py::str("expression1"))?)?, f__bool_parse(py::getitem(&v_inputs.clone(), &Py::str("expression2"))?)?])).items()?;
    if t765.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_t1 = t765[0].clone();
    v_t2 = t765[1].clone();
    if (py::not(&Py::Bool(matches!(py::getitem(&v_outputs.clone(), &Py::str("equivalent"))?, Py::Bool(_))))).truthy() {
        return Ok(Py::Float(1.0));
    }
    if (py::getitem(&v_outputs.clone(), &Py::str("equivalent"))?).truthy() {
        return Ok(py::float(&py::not(&py::getitem(&f_boolean_equivalence(py::getitem(&v_inputs.clone(), &Py::str("expression1"))?, py::getitem(&v_inputs.clone(), &Py::str("expression2"))?)?, &Py::str("equivalent"))?))?);
    }
    v_env = f__assignments(py::getitem(&v_outputs.clone(), &Py::str("counterexample"))?)?;
    let t772: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            return Ok(Some(py::float(&{ let t770 = f__bool_eval(v_t1.clone(), v_env.clone())?; Py::Bool({ let t771 = f__bool_eval(v_t2.clone(), v_env.clone())?; if py::eq(&t770, &t771) { true } else { false } }) })?));
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t772 {
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
fn f__verify_faraday(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_N: Py = Py::None;
    let mut v_big: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_t: Py = Py::None;
    let t1294 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("N"))?, py::getitem(&v_i.clone(), &Py::str("dPhi"))?, py::getitem(&v_i.clone(), &Py::str("dt"))?])).items()?;
    if t1294.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_N = t1294[0].clone();
    v_d = t1294[1].clone();
    v_t = t1294[2].clone();
    v_big = py::minmax(&[py::mul(&v_N.clone(), &py::abs(&v_d.clone())?)?, Py::Float(1e-300)], true)?;
    return Ok(py::add(&py::add(&py::add(&py::truediv(&py::abs(&py::sub(&py::mul(&py::getitem(&v_o.clone(), &Py::str("emf"))?, &v_t.clone())?, &py::mul(&v_N.clone(), &py::abs(&v_d.clone())?)?)?)?, &v_big.clone())?, &py::truediv(&py::abs(&py::add(&py::mul(&py::getitem(&v_o.clone(), &Py::str("emf_signed"))?, &v_t.clone())?, &py::mul(&v_N.clone(), &v_d.clone())?)?)?, &v_big.clone())?)?, &py::truediv(&py::abs(&py::sub(&py::mul(&py::getitem(&v_o.clone(), &Py::str("flux_rate"))?, &v_t.clone())?, &v_d.clone())?)?, &py::minmax(&[py::abs(&v_d.clone())?, Py::Float(1e-300)], true)?)?)?, &py::truediv(&py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("linkage_change"))?, &py::mul(&v_N.clone(), &v_d.clone())?)?)?, &v_big.clone())?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_gm(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_den: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_num: Py = Py::None;
    let mut v_w: Py = Py::None;
    let t1405 = (Py::tuple(vec![f__ptrim(py::getitem(&v_inputs.clone(), &Py::str("num"))?)?, f__ptrim(py::getitem(&v_inputs.clone(), &Py::str("den"))?)?])).items()?;
    if t1405.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_num = t1405[0].clone();
    v_den = t1405[1].clone();
    v_w = py::getitem(&v_outputs.clone(), &Py::str("phase_crossover"))?;
    v_g = f__G(v_num.clone(), v_den.clone(), v_w.clone())?;
    return Ok(py::add(&py::add(&py::add(&py::truediv(&py::abs(&py::imag(&v_g.clone())?)?, &py::abs(&v_g.clone())?)?, &{ let t1406 = py::real(&v_g.clone())?; Py::Bool({ let t1407 = Py::Int(0i64); if py::ge(&t1406, &t1407)? { true } else { false } }) })?, &f__rel(py::getitem(&v_outputs.clone(), &Py::str("gain_margin"))?, py::truediv(&Py::Float(1.0), &py::abs(&v_g.clone())?)?)?)?, &py::truediv(&py::abs(&py::sub(&py::getitem(&v_outputs.clone(), &Py::str("gain_margin_db"))?, &py::mul(&Py::Int(20i64), &py::math1("log10", &py::getitem(&v_outputs.clone(), &Py::str("gain_margin"))?)?)?)?)?, &py::minmax(&[Py::Float(1.0), py::abs(&py::getitem(&v_outputs.clone(), &Py::str("gain_margin_db"))?)?], true)?)?)?);
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
    let t1415 = (Py::tuple(vec![py::method(&py::getitem(&v_inputs.clone(), &Py::str("value"))?, "strip", &[])?, py::getitem(&v_outputs.clone(), &Py::str("result"))?])).items()?;
    if t1415.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_v = t1415[0].clone();
    v_r = t1415[1].clone();
    if ({ let t1416 = py::len(&v_r.clone())?; Py::Bool({ let t1417 = py::len(&v_v.clone())?; if py::ne(&t1416, &t1417) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(1.0));
    }
    let t1420 = ((if ({ let t1418 = py::getitem(&v_inputs.clone(), &Py::str("direction"))?; Py::Bool({ let t1419 = Py::str("to_gray"); if py::eq(&t1418, &t1419) { true } else { false } }) }).truthy() { Py::tuple(vec![v_r.clone(), v_v.clone()]) } else { Py::tuple(vec![v_v.clone(), v_r.clone()]) })).items()?;
    if t1420.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_g = t1420[0].clone();
    v_b = t1420[1].clone();
    let t1421 = (Py::tuple(vec![py::int_base(&v_g.clone(), &Py::Int(2i64))?, py::int_base(&v_b.clone(), &Py::Int(2i64))?])).items()?;
    if t1421.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_gi = t1421[0].clone();
    v_bi = t1421[1].clone();
    return Ok(py::add(&py::float(&{ let t1422 = v_gi.clone(); Py::Bool({ let t1423 = py::bitxor(&v_bi.clone(), &py::rshift(&v_bi.clone(), &Py::Int(1i64))?)?; if py::ne(&t1422, &t1423) { true } else { false } }) })?, &py::abs(&py::sub(&py::getitem(&v_outputs.clone(), &Py::str("decimal"))?, &py::int_base(&v_r.clone(), &Py::Int(2i64))?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_lossy(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_eps: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_mu: Py = Py::None;
    let mut v_sg: Py = Py::None;
    let mut v_w: Py = Py::None;
    let t2158 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("f"))?, py::getitem(&v_i.clone(), &Py::str("sigma"))?])).items()?;
    if t2158.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_f = t2158[0].clone();
    v_sg = t2158[1].clone();
    v_w = py::mul(&c__TWO_PI()?, &v_f.clone())?;
    v_eps = py::mul(&c__EPS0()?, &py::getitem(&v_i.clone(), &Py::str("eps_r"))?)?;
    v_mu = py::mul(&c__MU0()?, &py::getitem(&v_i.clone(), &Py::str("mu_r"))?)?;
    let t2159 = (Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("alpha"))?, py::getitem(&v_o.clone(), &Py::str("beta"))?])).items()?;
    if t2159.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t2159[0].clone();
    v_b = t2159[1].clone();
    return Ok(py::add(&py::add(&py::add(&py::add(&py::add(&py::add(&py::add(&py::truediv(&py::abs(&py::add(&py::sub(&py::mul(&v_a.clone(), &v_a.clone())?, &py::mul(&v_b.clone(), &v_b.clone())?)?, &py::mul(&py::mul(&py::mul(&v_w.clone(), &v_w.clone())?, &v_mu.clone())?, &v_eps.clone())?)?)?, &py::mul(&v_b.clone(), &v_b.clone())?)?, &py::truediv(&py::abs(&py::sub(&py::mul(&py::mul(&Py::Int(2i64), &v_a.clone())?, &v_b.clone())?, &py::mul(&py::mul(&v_w.clone(), &v_mu.clone())?, &v_sg.clone())?)?)?, &py::mul(&py::mul(&v_w.clone(), &v_mu.clone())?, &v_sg.clone())?)?)?, &f__rel(py::mul(&py::getitem(&v_o.clone(), &Py::str("skin_depth"))?, &v_a.clone())?, Py::Float(1.0))?)?, &f__rel(py::mul(&py::getitem(&v_o.clone(), &Py::str("v"))?, &v_b.clone())?, v_w.clone())?)?, &f__rel(py::mul(&py::getitem(&v_o.clone(), &Py::str("wavelength"))?, &v_b.clone())?, c__TWO_PI()?)?)?, &f__rel(py::mul(&py::pow(&py::getitem(&v_o.clone(), &Py::str("eta"))?, &Py::Int(4i64))?, &py::add(&py::mul(&v_sg.clone(), &v_sg.clone())?, &py::pow(&py::mul(&v_w.clone(), &v_eps.clone())?, &Py::Int(2i64))?)?)?, py::pow(&py::mul(&v_w.clone(), &v_mu.clone())?, &Py::Int(2i64))?)?)?, &py::truediv(&py::abs(&py::sub(&py::mul(&Py::Int(2i64), &py::math1("radians", &py::getitem(&v_o.clone(), &Py::str("eta_angle_deg"))?)?)?, &py::atan2(&v_sg.clone(), &py::mul(&v_w.clone(), &v_eps.clone())?)?)?)?, &Py::Float(std::f64::consts::PI))?)?, &f__rel(py::mul(&py::mul(&py::getitem(&v_o.clone(), &Py::str("loss_tangent"))?, &v_w.clone())?, &v_eps.clone())?, v_sg.clone())?)?);
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
    v_dc = py::set(&{ let mut out = vec![]; for t1615 in py::iter(&({ let t1614 = py::method(&v_inputs.clone(), "get", &[Py::str("dont_cares")])?; if t1614.truthy() { t1614 } else { Py::list(vec![]) } }))? { let mut v_m: Py; 
v_m = t1615;
out.push(py::int(&v_m.clone())?); } Py::list(out) })?;
    let t1672 = (f__sop_check(py::getitem(&v_outputs.clone(), &Py::str("minimal_sop"))?, v_n.clone(), py::slice(&Py::str("ABCD"), None, Some((v_n.clone()).i()?), None)?, py::set(&{ let mut out = vec![]; for t1671 in py::iter(&(py::getitem(&v_inputs.clone(), &Py::str("minterms"))?))? { let mut v_m: Py; 
v_m = t1671;
out.push(py::int(&v_m.clone())?); } Py::list(out) })?, v_dc.clone())?).items()?;
    if t1672.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_bad = t1672[0].clone();
    v_t = t1672[1].clone();
    v_lit = t1672[2].clone();
    v_best = f__sop_out(v_n.clone(), py::slice(&Py::str("ABCD"), None, Some((v_n.clone()).i()?), None)?, py::sorted(&py::set(&{ let mut out = vec![]; for t1673 in py::iter(&(py::getitem(&v_inputs.clone(), &Py::str("minterms"))?))? { let mut v_m: Py; 
v_m = t1673;
out.push(py::int(&v_m.clone())?); } Py::list(out) })?, (Py::Bool(false)).truthy())?, py::sorted(&v_dc.clone(), (Py::Bool(false)).truthy())?)?;
    return Ok(py::add(&py::add(&py::add(&py::add(&v_bad.clone(), &py::abs(&py::sub(&py::getitem(&v_outputs.clone(), &Py::str("terms"))?, &v_t.clone())?)?)?, &py::abs(&py::sub(&py::getitem(&v_outputs.clone(), &Py::str("literals"))?, &v_lit.clone())?)?)?, &py::abs(&py::sub(&py::getitem(&v_outputs.clone(), &Py::str("terms"))?, &py::getitem(&v_best.clone(), &Py::str("terms"))?)?)?)?, &py::abs(&py::sub(&py::getitem(&v_outputs.clone(), &Py::str("literals"))?, &py::getitem(&v_best.clone(), &Py::str("literals"))?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_magnetic(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_amp: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_res: Py = Py::None;
    v_k = py::len(&py::getitem(&v_i.clone(), &Py::str("lengths"))?)?;
    if ({ let t1490 = { let t1486 = py::len(&py::getitem(&v_o.clone(), &Py::str("B"))?)?; Py::Bool({ let t1487 = v_k.clone(); if py::ne(&t1486, &t1487) { true } else { false } }) }; if t1490.truthy() { t1490 } else { { let t1488 = py::len(&py::getitem(&v_o.clone(), &Py::str("H"))?)?; Py::Bool({ let t1489 = v_k.clone(); if py::ne(&t1488, &t1489) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_res = Py::Float(0.0);
    v_amp = Py::Float(0.0);
    for t1491 in py::range_iter(&[(v_k.clone()).i()?]) {
        v_j = t1491;
        v_res = py::iadd(&v_res, &py::add(&f__rel(py::getitem(&py::getitem(&v_o.clone(), &Py::str("B"))?, &v_j.clone())?, py::truediv(&py::getitem(&v_o.clone(), &Py::str("flux"))?, &py::getitem(&py::getitem(&v_i.clone(), &Py::str("areas"))?, &v_j.clone())?)?)?, &f__rel(py::mul(&py::mul(&py::getitem(&py::getitem(&v_o.clone(), &Py::str("H"))?, &v_j.clone())?, &c__MU0()?)?, &py::getitem(&py::getitem(&v_i.clone(), &Py::str("mu_r"))?, &v_j.clone())?)?, py::getitem(&py::getitem(&v_o.clone(), &Py::str("B"))?, &v_j.clone())?)?)?)?;
        v_amp = py::iadd(&v_amp, &py::mul(&py::getitem(&py::getitem(&v_o.clone(), &Py::str("H"))?, &v_j.clone())?, &py::getitem(&py::getitem(&v_i.clone(), &Py::str("lengths"))?, &v_j.clone())?)?)?;
    }
    v_res = py::iadd(&v_res, &py::add(&py::add(&f__rel(v_amp.clone(), py::mul(&py::getitem(&v_i.clone(), &Py::str("N"))?, &py::getitem(&v_o.clone(), &Py::str("current"))?)?)?, &f__rel(py::getitem(&v_o.clone(), &Py::str("mmf"))?, py::mul(&py::getitem(&v_i.clone(), &Py::str("N"))?, &py::getitem(&v_o.clone(), &Py::str("current"))?)?)?)?, &f__rel(py::mul(&py::getitem(&v_o.clone(), &Py::str("reluctance"))?, &py::getitem(&v_o.clone(), &Py::str("flux"))?)?, py::getitem(&v_o.clone(), &Py::str("mmf"))?)?)?)?;
    if ({ let t1492 = py::method(&v_i.clone(), "get", &[Py::str("flux"), Py::Float(0.0)])?; Py::Bool({ let t1493 = Py::Int(0i64); if py::gt(&t1492, &t1493)? { true } else { false } }) }).truthy() {
        v_res = py::iadd(&v_res, &f__rel(py::getitem(&v_o.clone(), &Py::str("flux"))?, py::getitem(&v_i.clone(), &Py::str("flux"))?)?)?;
    } else {
        v_res = py::iadd(&v_res, &f__rel(py::getitem(&v_o.clone(), &Py::str("current"))?, py::method(&v_i.clone(), "get", &[Py::str("current"), Py::Float(0.0)])?)?)?;
    }
    return Ok(v_res.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_mutual(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_L1: Py = Py::None;
    let mut v_L2: Py = Py::None;
    let mut v_big: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_rate: Py = Py::None;
    let t1693 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("L1"))?, py::getitem(&v_i.clone(), &Py::str("L2"))?, py::getitem(&v_i.clone(), &Py::str("k"))?])).items()?;
    if t1693.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_L1 = t1693[0].clone();
    v_L2 = t1693[1].clone();
    v_k = t1693[2].clone();
    v_rate = py::truediv(&py::abs(&py::getitem(&v_i.clone(), &Py::str("dI1"))?)?, &py::getitem(&v_i.clone(), &Py::str("dt"))?)?;
    v_big = py::minmax(&[py::mul(&v_L1.clone(), &v_rate.clone())?, Py::Float(1e-300)], true)?;
    return Ok(py::add(&py::add(&py::add(&py::add(&py::truediv(&py::abs(&py::sub(&py::pow(&py::getitem(&v_o.clone(), &Py::str("M"))?, &Py::Int(2i64))?, &py::mul(&py::mul(&py::mul(&v_k.clone(), &v_k.clone())?, &v_L1.clone())?, &v_L2.clone())?)?)?, &py::mul(&v_L1.clone(), &v_L2.clone())?)?, &py::truediv(&py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("E1"))?, &py::mul(&v_L1.clone(), &v_rate.clone())?)?)?, &v_big.clone())?)?, &py::truediv(&py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("E2"))?, &py::mul(&py::getitem(&v_o.clone(), &Py::str("M"))?, &v_rate.clone())?)?)?, &v_big.clone())?)?, &py::truediv(&py::abs(&py::sub(&py::add(&py::getitem(&v_o.clone(), &Py::str("L_aiding"))?, &py::getitem(&v_o.clone(), &Py::str("L_opposing"))?)?, &py::mul(&Py::Float(2.0), &py::add(&v_L1.clone(), &v_L2.clone())?)?)?)?, &py::add(&v_L1.clone(), &v_L2.clone())?)?)?, &py::truediv(&py::abs(&py::sub(&py::sub(&py::getitem(&v_o.clone(), &Py::str("L_aiding"))?, &py::getitem(&v_o.clone(), &Py::str("L_opposing"))?)?, &py::mul(&Py::Float(4.0), &py::getitem(&v_o.clone(), &Py::str("M"))?)?)?)?, &py::add(&v_L1.clone(), &v_L2.clone())?)?)?);
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
    let rec2243 = py::rec_cell();
    v_walk = { let v_inputs = v_inputs.clone(); let v_outputs = v_outputs.clone(); let v_tree = v_tree.clone(); let v_worst = v_worst.clone(); let rec2243 = rec2243.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_t: Py = a.get(0).cloned().unwrap_or(Py::None);
            let v_walk: Py = py::rec_get(&rec2243)?;
            let mut v_eq: Py = Py::None;
            let mut v_vals: Py = Py::None;
            if (Py::Bool(matches!(v_t.clone(), Py::Float(_)))).truthy() {
                return Ok(v_t.clone());
            }
            v_vals = { let mut out = vec![]; for t2244 in py::iter(&(py::getitem(&v_t.clone(), &Py::Int(1i64))?))? { let mut v_x: Py; 
v_x = t2244;
out.push(py::call(&v_walk.clone(), &[v_x.clone()])?); } Py::list(out) };
            if ({ let t2245 = py::getitem(&v_t.clone(), &Py::Int(0i64))?; Py::Bool({ let t2246 = Py::str("s"); if py::eq(&t2245, &t2246) { true } else { false } }) }).truthy() {
                return Ok(py::fsum(&v_vals.clone())?);
            }
            v_eq = py::truediv(&Py::Float(1.0), &py::fsum(&{ let mut out = vec![]; for t2247 in py::iter(&(v_vals.clone()))? { let mut v_v: Py; 
v_v = t2247;
out.push(py::truediv(&Py::Float(1.0), &v_v.clone())?); } Py::list(out) })?)?;
            py::setitem(&v_worst.clone(), &Py::Int(0i64), py::minmax(&[py::getitem(&v_worst.clone(), &Py::Int(0i64))?, py::abs(&py::sub(&py::fsum(&{ let mut out = vec![]; for t2248 in py::iter(&(v_vals.clone()))? { let mut v_v: Py; 
v_v = t2248;
out.push(py::truediv(&v_eq.clone(), &v_v.clone())?); } Py::list(out) })?, &Py::Float(1.0))?)?], true)?)?;
            return Ok(v_eq.clone());
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    py::rec_set(&rec2243, &v_walk);
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
    v_V = { let t1197 = Py::dict(vec![]); py::setitem(&t1197, &Py::str("0"), Py::Float(0.0))?; py::kw_merge(&t1197, &py::getitem(&v_outputs.clone(), &Py::str("node_voltages"))?)?; t1197 };
    v_cur = py::getitem(&v_outputs.clone(), &Py::str("source_currents"))?;
    v_nodes = py::set(&{ let mut out = vec![]; for t1198 in py::iter(&(v_els.clone()))? { let mut v_e: Py; 
v_e = t1198;
for t1199 in py::iter(&(Py::tuple(vec![py::getitem(&v_e.clone(), &Py::Int(2i64))?, py::getitem(&v_e.clone(), &Py::Int(3i64))?])))? { let mut v_n: Py; 
v_n = t1199;
out.push(v_n.clone()); }} Py::list(out) })?;
    if ({ let t1207 = { let t1200 = py::set(&v_V.clone())?; Py::Bool({ let t1201 = v_nodes.clone(); if py::ne(&t1200, &t1201) { true } else { false } }) }; if t1207.truthy() { t1207 } else { { let t1202 = py::set(&v_cur.clone())?; Py::Bool({ let t1203 = py::set(&{ let mut out = vec![]; for t1204 in py::iter(&(v_els.clone()))? { let mut v_e: Py; 
v_e = t1204;
if ({ let t1205 = py::getitem(&v_e.clone(), &Py::Int(0i64))?; Py::Bool({ let t1206 = Py::str("V"); if py::eq(&t1205, &t1206) { true } else { false } }) }).truthy() { out.push(py::getitem(&v_e.clone(), &Py::Int(1i64))?); }} Py::list(out) })?; if py::ne(&t1202, &t1203) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_kcl = { let mut out = vec![]; for t1208 in py::iter(&(v_nodes.clone()))? { let mut v_n: Py; 
v_n = t1208;
out.push((v_n.clone(), Py::list(vec![]))); } Py::dict(out) };
    let t1209 = (Py::tuple(vec![Py::Float(0.0), Py::Float(0.0), Py::list(vec![])])).items()?;
    if t1209.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_worst = t1209[0].clone();
    v_diss = t1209[1].clone();
    v_terms = t1209[2].clone();
    for t1210 in py::iter(&(v_els.clone()))? {
        let t1215 = (t1210).items()?;
        if t1215.len() != 5 { return Err(PyErr::Value("unpack".into())); }
        v_t = t1215[0].clone();
        v_name = t1215[1].clone();
        v_a = t1215[2].clone();
        v_b = t1215[3].clone();
        v_val = t1215[4].clone();
        if ({ let t1211 = v_t.clone(); Py::Bool({ let t1212 = Py::str("R"); if py::eq(&t1211, &t1212) { true } else { false } }) }).truthy() {
            v_i = py::truediv(&py::sub(&py::getitem(&v_V.clone(), &v_a.clone())?, &py::getitem(&v_V.clone(), &v_b.clone())?)?, &v_val.clone())?;
            v_diss = py::iadd(&v_diss, &py::mul(&py::mul(&v_i.clone(), &v_i.clone())?, &v_val.clone())?)?;
        } else {
            if ({ let t1213 = v_t.clone(); Py::Bool({ let t1214 = Py::str("V"); if py::eq(&t1213, &t1214) { true } else { false } }) }).truthy() {
                v_i = py::getitem(&v_cur.clone(), &v_name.clone())?;
                v_worst = py::minmax(&[v_worst.clone(), py::truediv(&py::abs(&py::sub(&py::sub(&py::getitem(&v_V.clone(), &v_a.clone())?, &py::getitem(&v_V.clone(), &v_b.clone())?)?, &v_val.clone())?)?, &py::minmax(&[py::abs(&v_val.clone())?, py::abs(&py::getitem(&v_V.clone(), &v_a.clone())?)?, py::abs(&py::getitem(&v_V.clone(), &v_b.clone())?)?, Py::Float(1e-12)], true)?)?], true)?;
            } else {
                v_i = v_val.clone();
            }
        }
        let _ = py::method(&py::getitem(&v_kcl.clone(), &v_a.clone())?, "append", &[v_i.clone()])?;
        let _ = py::method(&py::getitem(&v_kcl.clone(), &v_b.clone())?, "append", &[py::neg(&v_i.clone())?])?;
    }
    v_big = { let t1218 = { let t = { let mut out = vec![]; for t1216 in py::iter(&(py::method(&v_kcl.clone(), "values", &[])?))? { let mut v_ts: Py; 
v_ts = t1216;
for t1217 in py::iter(&(v_ts.clone()))? { let mut v_x: Py; 
v_x = t1217;
out.push(py::abs(&v_x.clone())?); }} Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } }; if t1218.truthy() { t1218 } else { Py::Float(1.0) } };
    for t1219 in py::iter(&(py::method(&v_kcl.clone(), "items", &[])?))? {
        let t1223 = (t1219).items()?;
        if t1223.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_n = t1223[0].clone();
        v_terms = t1223[1].clone();
        if ({ let t1220 = v_n.clone(); Py::Bool({ let t1221 = Py::str("0"); if py::eq(&t1220, &t1221) { true } else { false } }) }).truthy() {
            continue;
        }
        v_scale = py::minmax(&[py::fsum(&{ let mut out = vec![]; for t1222 in py::iter(&(v_terms.clone()))? { let mut v_x: Py; 
v_x = t1222;
out.push(py::abs(&v_x.clone())?); } Py::list(out) })?, v_big.clone()], true)?;
        v_worst = py::minmax(&[v_worst.clone(), py::truediv(&py::abs(&py::fsum(&v_terms.clone())?)?, &v_scale.clone())?], true)?;
    }
    v_p = py::getitem(&v_outputs.clone(), &Py::str("power_delivered"))?;
    v_worst = py::minmax(&[v_worst.clone(), (if ({ let t1224 = v_p.clone(); if t1224.truthy() { t1224 } else { v_diss.clone() } }).truthy() { py::truediv(&py::abs(&py::sub(&v_p.clone(), &v_diss.clone())?)?, &py::minmax(&[py::abs(&v_p.clone())?, py::abs(&v_diss.clone())?, Py::Float(1e-300)], true)?)? } else { Py::Float(0.0) })], true)?;
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
    let t2163 = (Py::tuple(vec![py::getitem(&v_inputs.clone(), &Py::str("magnitude"))?, py::getitem(&v_inputs.clone(), &Py::str("angle_deg"))?])).items()?;
    if t2163.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_m = t2163[0].clone();
    v_a = t2163[1].clone();
    v_z = py::complex(&[py::getitem(&v_outputs.clone(), &Py::str("real"))?, py::getitem(&v_outputs.clone(), &Py::str("imag"))?])?;
    v_d = (if ({ let t2164 = py::abs(&v_z.clone())?; Py::Bool({ let t2165 = Py::Int(0i64); if py::gt(&t2164, &t2165)? { true } else { false } }) }).truthy() { py::sub(&py::modulo(&py::add(&py::sub(&py::math1("degrees", &py::cmath("phase", &[v_z.clone()])?)?, &v_a.clone())?, &Py::Int(180i64))?, &Py::Int(360i64))?, &Py::Int(180i64))? } else { Py::Float(0.0) });
    return Ok(py::add(&py::truediv(&py::abs(&py::sub(&py::abs(&v_z.clone())?, &v_m.clone())?)?, &py::minmax(&[Py::Float(1.0), v_m.clone()], true)?)?, &(if ({ let t2166 = v_m.clone(); Py::Bool({ let t2167 = Py::Int(0i64); if py::gt(&t2166, &t2167)? { true } else { false } }) }).truthy() { py::truediv(&py::abs(&v_d.clone())?, &Py::Int(180i64))? } else { Py::Float(0.0) }))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_parallel_ac(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_C: Py = Py::None;
    let mut v_L: Py = Py::None;
    let mut v_R: Py = Py::None;
    let mut v_S: Py = Py::None;
    let mut v_V: Py = Py::None;
    let mut v_cur: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_il: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let t1938 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("V"))?, py::getitem(&v_i.clone(), &Py::str("R"))?])).items()?;
    if t1938.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_V = t1938[0].clone();
    v_R = t1938[1].clone();
    let t1939 = (Py::tuple(vec![py::method(&v_i.clone(), "get", &[Py::str("L"), Py::Float(0.0)])?, py::method(&v_i.clone(), "get", &[Py::str("C"), Py::Float(0.0)])?])).items()?;
    if t1939.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_L = t1939[0].clone();
    v_C = t1939[1].clone();
    v_w = py::mul(&c__TWO_PI()?, &py::getitem(&v_i.clone(), &Py::str("f"))?)?;
    let t1940 = (Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("I"))?, py::getitem(&v_o.clone(), &Py::str("S"))?])).items()?;
    if t1940.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_cur = t1940[0].clone();
    v_S = t1940[1].clone();
    v_il = (if ({ let t1941 = v_L.clone(); Py::Bool({ let t1942 = Py::Int(0i64); if py::gt(&t1941, &t1942)? { true } else { false } }) }).truthy() { py::truediv(&v_V.clone(), &py::mul(&v_w.clone(), &v_L.clone())?)? } else { Py::Float(0.0) });
    v_d = py::sub(&py::getitem(&v_o.clone(), &Py::str("IL"))?, &py::getitem(&v_o.clone(), &Py::str("IC"))?)?;
    v_want = (if ({ let t1943 = v_d.clone(); Py::Bool({ let t1944 = py::mul(&Py::Float(1e-12), &v_cur.clone())?; if py::gt(&t1943, &t1944)? { true } else { false } }) }).truthy() { Py::str("lagging") } else { (if ({ let t1945 = v_d.clone(); Py::Bool({ let t1946 = py::mul(&py::neg(&Py::Float(1e-12))?, &v_cur.clone())?; if py::lt(&t1945, &t1946)? { true } else { false } }) }).truthy() { Py::str("leading") } else { Py::str("unity") }) });
    return Ok(py::add(&py::add(&py::add(&py::add(&py::add(&py::add(&py::add(&py::add(&py::add(&py::add(&py::truediv(&py::abs(&py::sub(&py::hypot(&[py::getitem(&v_o.clone(), &Py::str("IR"))?, v_d.clone()])?, &v_cur.clone())?)?, &v_cur.clone())?, &f__rel(py::mul(&py::getitem(&v_o.clone(), &Py::str("IR"))?, &v_R.clone())?, v_V.clone())?)?, &py::truediv(&py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("IL"))?, &v_il.clone())?)?, &v_cur.clone())?)?, &py::truediv(&py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("IC"))?, &py::mul(&py::mul(&v_V.clone(), &v_w.clone())?, &v_C.clone())?)?)?, &v_cur.clone())?)?, &f__rel(py::mul(&py::getitem(&v_o.clone(), &Py::str("Z"))?, &v_cur.clone())?, v_V.clone())?)?, &f__rel(v_S.clone(), py::mul(&v_V.clone(), &v_cur.clone())?)?)?, &py::truediv(&py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("P"))?, &py::mul(&py::mul(&v_V.clone(), &v_cur.clone())?, &py::getitem(&v_o.clone(), &Py::str("power_factor"))?)?)?)?, &v_S.clone())?)?, &py::truediv(&py::abs(&py::sub(&py::add(&py::pow(&py::getitem(&v_o.clone(), &Py::str("P"))?, &Py::Int(2i64))?, &py::pow(&py::getitem(&v_o.clone(), &Py::str("Q"))?, &Py::Int(2i64))?)?, &py::mul(&v_S.clone(), &v_S.clone())?)?)?, &py::mul(&v_S.clone(), &v_S.clone())?)?)?, &py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("power_factor"))?, &py::math1("cos", &py::math1("radians", &py::getitem(&v_o.clone(), &Py::str("phase_deg"))?)?)?)?)?)?, &py::truediv(&py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("Q"))?, &py::mul(&v_V.clone(), &v_d.clone())?)?)?, &v_S.clone())?)?, &py::float(&{ let t1947 = py::getitem(&v_o.clone(), &Py::str("nature"))?; Py::Bool({ let t1948 = v_want.clone(); if py::ne(&t1947, &t1948) { true } else { false } }) })?)?);
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
    let t2182 = (Py::tuple(vec![py::getitem(&v_inputs.clone(), &Py::str("P"))?, py::getitem(&v_inputs.clone(), &Py::str("V"))?, py::getitem(&v_inputs.clone(), &Py::str("f"))?])).items()?;
    if t2182.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_P = t2182[0].clone();
    v_V = t2182[1].clone();
    v_f = t2182[2].clone();
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
    let t2128 = (Py::tuple(vec![py::getitem(&v_outputs.clone(), &Py::str("amplitude"))?, py::math1("radians", &py::getitem(&v_outputs.clone(), &Py::str("phase_deg"))?)?])).items()?;
    if t2128.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_A = t2128[0].clone();
    v_p = t2128[1].clone();
    v_scale = { let t2129 = py::sum(&py::getitem(&v_inputs.clone(), &Py::str("amplitudes"))?, None)?; if t2129.truthy() { t2129 } else { Py::Float(1.0) } };
    v_worst = Py::Float(0.0);
    for t2130 in py::range_iter(&[(Py::Int(8i64)).i()?]) {
        v_k = t2130;
        v_th = py::add(&py::truediv(&py::mul(&py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?, &v_k.clone())?, &Py::Int(8i64))?, &Py::Float(0.1))?;
        v_lhs = py::fsum(&{ let mut out = vec![]; for t2131 in py::iter(&(py::zip(&[py::getitem(&v_inputs.clone(), &Py::str("amplitudes"))?, py::getitem(&v_inputs.clone(), &Py::str("phases_deg"))?])?))? { let mut v_a: Py; let mut v_q: Py; 
let t2132 = (t2131).items()?;
if t2132.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t2132[0].clone();
v_q = t2132[1].clone();
out.push(py::mul(&v_a.clone(), &py::math1("cos", &py::add(&v_th.clone(), &py::math1("radians", &v_q.clone())?)?)?)?); } Py::list(out) })?;
        v_worst = py::minmax(&[v_worst.clone(), py::truediv(&py::abs(&py::sub(&v_lhs.clone(), &py::mul(&v_A.clone(), &py::math1("cos", &py::add(&v_th.clone(), &v_p.clone())?)?)?)?)?, &v_scale.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_plate_cap(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_gaps: Py = Py::None;
    v_gaps = py::sub(&py::method(&v_i.clone(), "get", &[Py::str("n"), Py::Float(2.0)])?, &Py::Int(1i64))?;
    return Ok(py::add(&py::add(&py::add(&py::add(&f__rel(py::mul(&py::getitem(&v_o.clone(), &Py::str("E"))?, &py::getitem(&v_i.clone(), &Py::str("d"))?)?, py::getitem(&v_i.clone(), &Py::str("V"))?)?, &f__rel(py::getitem(&v_o.clone(), &Py::str("D"))?, py::mul(&py::mul(&c__EPS0()?, &py::getitem(&v_i.clone(), &Py::str("eps_r"))?)?, &py::getitem(&v_o.clone(), &Py::str("E"))?)?)?)?, &f__rel(py::mul(&py::mul(&py::getitem(&v_o.clone(), &Py::str("D"))?, &v_gaps.clone())?, &py::getitem(&v_i.clone(), &Py::str("A"))?)?, py::getitem(&v_o.clone(), &Py::str("Q"))?)?)?, &f__rel(py::getitem(&v_o.clone(), &Py::str("Q"))?, py::mul(&py::getitem(&v_o.clone(), &Py::str("C"))?, &py::getitem(&v_i.clone(), &Py::str("V"))?)?)?)?, &f__rel(py::getitem(&v_o.clone(), &Py::str("energy"))?, py::mul(&py::mul(&Py::Float(0.5), &py::getitem(&v_o.clone(), &Py::str("Q"))?)?, &py::getitem(&v_i.clone(), &Py::str("V"))?)?)?)?);
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
    let t2116 = (Py::tuple(vec![f__ptrim(py::getitem(&v_inputs.clone(), &Py::str("num"))?)?, f__ptrim(py::getitem(&v_inputs.clone(), &Py::str("den"))?)?])).items()?;
    if t2116.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_num = t2116[0].clone();
    v_den = t2116[1].clone();
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
    let t927 = (Py::tuple(vec![f__ptrim(py::getitem(&v_inputs.clone(), &Py::str("num"))?)?, f__ptrim(py::getitem(&v_inputs.clone(), &Py::str("den"))?)?])).items()?;
    if t927.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_num = t927[0].clone();
    v_den = t927[1].clone();
    v_char = f__ptrim(f__padd(v_den.clone(), { let mut out = vec![]; for t928 in py::iter(&(v_num.clone()))? { let mut v_x: Py; 
v_x = t928;
out.push(py::mul(&py::method(&v_inputs.clone(), "get", &[Py::str("K"), Py::Float(1.0)])?, &v_x.clone())?); } Py::list(out) })?)?;
    v_poles = py::getitem(&v_outputs.clone(), &Py::str("poles"))?;
    if ({ let t929 = py::len(&v_poles.clone())?; Py::Bool({ let t930 = py::sub(&py::len(&v_char.clone())?, &Py::Int(1i64))?; if py::ne(&t929, &t930) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_worst = Py::Float(0.0);
    for t931 in py::iter(&(v_poles.clone()))? {
        v_p = t931;
        v_p = py::complex(&[v_p.clone()])?;
        v_terms = { let mut out = vec![]; for t932 in py::iter(&(py::enumerate(&v_char.clone(), 0)?))? { let mut v_c: Py; let mut v_i: Py; 
let t933 = (t932).items()?;
if t933.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_i = t933[0].clone();
v_c = t933[1].clone();
out.push(py::mul(&v_c.clone(), &py::pow(&v_p.clone(), &py::sub(&py::sub(&py::len(&v_char.clone())?, &Py::Int(1i64))?, &v_i.clone())?)?)?); } Py::list(out) };
        v_worst = py::minmax(&[v_worst.clone(), py::truediv(&py::abs(&py::sum(&v_terms.clone(), None)?)?, &{ let t935 = py::sum(&{ let mut out = vec![]; for t934 in py::iter(&(v_terms.clone()))? { let mut v_t: Py; 
v_t = t934;
out.push(py::abs(&v_t.clone())?); } Py::list(out) }, None)?; if t935.truthy() { t935 } else { Py::Float(1.0) } })?], true)?;
    }
    v_mr = py::minmax(&[{ let mut out = vec![]; for t936 in py::iter(&(v_poles.clone()))? { let mut v_p: Py; 
v_p = t936;
out.push(py::real(&py::complex(&[v_p.clone()])?)?); } Py::list(out) }], true)?;
    return Ok(py::add(&py::add(&v_worst.clone(), &py::abs(&py::sub(&py::getitem(&v_outputs.clone(), &Py::str("max_real_part"))?, &v_mr.clone())?)?)?, &{ let t937 = py::getitem(&v_outputs.clone(), &Py::str("stable"))?; Py::Bool({ let t938 = { let t939 = v_mr.clone(); Py::Bool({ let t940 = Py::Int(0i64); if py::lt(&t939, &t940)? { true } else { false } }) }; if py::ne(&t937, &t938) { true } else { false } }) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_r2p(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_z: Py = Py::None;
    let t2198 = (Py::tuple(vec![py::getitem(&v_outputs.clone(), &Py::str("magnitude"))?, py::math1("radians", &py::getitem(&v_outputs.clone(), &Py::str("angle_deg"))?)?])).items()?;
    if t2198.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_m = t2198[0].clone();
    v_a = t2198[1].clone();
    v_z = py::complex(&[py::getitem(&v_inputs.clone(), &Py::str("real"))?, py::getitem(&v_inputs.clone(), &Py::str("imag"))?])?;
    return Ok(py::add(&py::truediv(&py::abs(&py::sub(&py::complex(&[py::mul(&v_m.clone(), &py::math1("cos", &v_a.clone())?)?, py::mul(&v_m.clone(), &py::math1("sin", &v_a.clone())?)?])?, &v_z.clone())?)?, &py::minmax(&[Py::Float(1.0), py::abs(&v_z.clone())?], true)?)?, &py::not(&{ let t2199 = py::neg(&Py::Int(180i64))?; Py::Bool({ let t2200 = py::getitem(&v_outputs.clone(), &Py::str("angle_deg"))?; if py::lt(&t2199, &t2200)? { { let t2201 = Py::Int(180i64); if py::le(&t2200, &t2201)? { true } else { false } } } else { false } }) }))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_r_temp(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_R1: Py = Py::None;
    let mut v_a1: Py = Py::None;
    let t2209 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("R1"))?, py::getitem(&v_i.clone(), &Py::str("alpha1"))?])).items()?;
    if t2209.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_R1 = t2209[0].clone();
    v_a1 = t2209[1].clone();
    return Ok(py::add(&py::add(&f__rel(py::getitem(&v_o.clone(), &Py::str("R2"))?, py::mul(&v_R1.clone(), &py::add(&Py::Float(1.0), &py::mul(&v_a1.clone(), &py::sub(&py::getitem(&v_i.clone(), &Py::str("T2"))?, &py::getitem(&v_i.clone(), &Py::str("T1"))?)?)?)?)?)?, &py::truediv(&py::abs(&py::sub(&py::mul(&py::getitem(&v_o.clone(), &Py::str("alpha2"))?, &py::getitem(&v_o.clone(), &Py::str("R2"))?)?, &py::mul(&v_a1.clone(), &v_R1.clone())?)?)?, &py::minmax(&[py::abs(&py::mul(&v_a1.clone(), &v_R1.clone())?)?, Py::Float(1e-300)], true)?)?)?, &py::truediv(&py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("change"))?, &py::sub(&py::getitem(&v_o.clone(), &Py::str("R2"))?, &v_R1.clone())?)?)?, &v_R1.clone())?)?);
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
    let t2504 = ({ let mut out = vec![]; for t2503 in py::iter(&(Py::tuple(vec![Py::str("R"), Py::str("L"), Py::str("C"), Py::str("V"), Py::str("t")])))? { let mut v_k: Py; 
v_k = t2503;
out.push(py::getitem(&v_inputs.clone(), &v_k.clone())?); } Py::list(out) }).items()?;
    if t2504.len() != 5 { return Err(PyErr::Value("unpack".into())); }
    v_R = t2504[0].clone();
    v_L = t2504[1].clone();
    v_C = t2504[2].clone();
    v_V = t2504[3].clone();
    v_t = t2504[4].clone();
    let t2505 = (Py::tuple(vec![py::truediv(&v_R.clone(), &py::mul(&Py::Int(2i64), &v_L.clone())?)?, py::truediv(&Py::Int(1i64), &py::math1("sqrt", &py::mul(&v_L.clone(), &v_C.clone())?)?)?])).items()?;
    if t2505.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t2505[0].clone();
    v_w0 = t2505[1].clone();
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
    let t2506 = (Py::tuple(vec![py::call(&v_f.clone(), &[py::sub(&v_t.clone(), &v_h.clone())?])?, py::call(&v_f.clone(), &[py::add(&v_t.clone(), &v_h.clone())?])?])).items()?;
    if t2506.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lo = t2506[0].clone();
    v_hi = t2506[1].clone();
    v_didt = (if ({ let t2507 = v_t.clone(); Py::Bool({ let t2508 = v_h.clone(); if py::ge(&t2507, &t2508)? { true } else { false } }) }).truthy() { py::truediv(&py::sub(&py::getitem(&v_hi.clone(), &Py::str("i"))?, &py::getitem(&v_lo.clone(), &Py::str("i"))?)?, &py::mul(&Py::Int(2i64), &v_h.clone())?)? } else { py::truediv(&py::sub(&py::getitem(&v_hi.clone(), &Py::str("i"))?, &py::getitem(&v_outputs.clone(), &Py::str("i"))?)?, &v_h.clone())? });
    v_dvdt = (if ({ let t2509 = v_t.clone(); Py::Bool({ let t2510 = v_h.clone(); if py::ge(&t2509, &t2510)? { true } else { false } }) }).truthy() { py::truediv(&py::sub(&py::getitem(&v_hi.clone(), &Py::str("vC"))?, &py::getitem(&v_lo.clone(), &Py::str("vC"))?)?, &py::mul(&Py::Int(2i64), &v_h.clone())?)? } else { py::truediv(&py::sub(&py::getitem(&v_hi.clone(), &Py::str("vC"))?, &py::getitem(&v_outputs.clone(), &Py::str("vC"))?)?, &v_h.clone())? });
    v_scale = { let t2511 = py::abs(&v_V.clone())?; if t2511.truthy() { t2511 } else { Py::Float(1.0) } };
    v_kvl = py::truediv(&py::abs(&py::sub(&py::sub(&py::sub(&v_V.clone(), &py::mul(&v_R.clone(), &py::getitem(&v_outputs.clone(), &Py::str("i"))?)?)?, &py::mul(&v_L.clone(), &v_didt.clone())?)?, &py::getitem(&v_outputs.clone(), &Py::str("vC"))?)?)?, &v_scale.clone())?;
    v_cap = py::truediv(&py::mul(&py::abs(&py::sub(&py::mul(&v_C.clone(), &v_dvdt.clone())?, &py::getitem(&v_outputs.clone(), &Py::str("i"))?)?)?, &v_R.clone())?, &v_scale.clone())?;
    v_here = py::call(&v_f.clone(), &[v_t.clone()])?;
    v_same = py::add(&py::truediv(&py::abs(&py::sub(&py::getitem(&v_here.clone(), &Py::str("vC"))?, &py::getitem(&v_outputs.clone(), &Py::str("vC"))?)?)?, &v_scale.clone())?, &py::truediv(&py::mul(&py::abs(&py::sub(&py::getitem(&v_here.clone(), &Py::str("i"))?, &py::getitem(&v_outputs.clone(), &Py::str("i"))?)?)?, &v_R.clone())?, &v_scale.clone())?)?;
    return Ok(py::add(&py::add(&py::add(&py::add(&py::add(&py::add(&v_same.clone(), &(if ({ let t2512 = v_kvl.clone(); Py::Bool({ let t2513 = Py::Float(1e-05); if py::gt(&t2512, &t2513)? { true } else { false } }) }).truthy() { v_kvl.clone() } else { Py::Float(0.0) }))?, &(if ({ let t2514 = v_cap.clone(); Py::Bool({ let t2515 = Py::Float(1e-05); if py::gt(&t2514, &t2515)? { true } else { false } }) }).truthy() { v_cap.clone() } else { Py::Float(0.0) }))?, &f__rel(py::getitem(&v_outputs.clone(), &Py::str("alpha"))?, v_a.clone())?)?, &f__rel(py::getitem(&v_outputs.clone(), &Py::str("omega0"))?, v_w0.clone())?)?, &f__rel(py::getitem(&v_outputs.clone(), &Py::str("zeta"))?, py::truediv(&v_a.clone(), &v_w0.clone())?)?)?, &f__rel(py::getitem(&v_outputs.clone(), &Py::str("i_mA"))?, py::mul(&py::getitem(&v_outputs.clone(), &Py::str("i"))?, &Py::Float(1000.0))?)?)?);
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
    v_rhp = py::sum(&{ let mut out = vec![]; for t2318 in py::iter(&(v_roots.clone()))? { let mut v_r: Py; 
v_r = t2318;
if ({ let t2319 = py::real(&py::complex(&[v_r.clone()])?)?; Py::Bool({ let t2320 = Py::Int(0i64); if py::gt(&t2319, &t2320)? { true } else { false } }) }).truthy() { out.push(Py::Int(1i64)); }} Py::list(out) }, None)?;
    v_fc = py::method(&v_outputs.clone(), "get", &[Py::str("first_column"), Py::list(vec![])])?;
    v_ch = (if (v_fc.clone()).truthy() { py::sum(&{ let mut out = vec![]; for t2321 in py::iter(&(py::zip(&[v_fc.clone(), py::slice(&v_fc.clone(), Some((Py::Int(1i64)).i()?), None, None)?])?))? { let mut v_x: Py; let mut v_y: Py; 
let t2322 = (t2321).items()?;
if t2322.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t2322[0].clone();
v_y = t2322[1].clone();
if ({ let t2323 = { let t2325 = v_x.clone(); Py::Bool({ let t2326 = Py::Int(0i64); if py::gt(&t2325, &t2326)? { true } else { false } }) }; Py::Bool({ let t2324 = { let t2327 = v_y.clone(); Py::Bool({ let t2328 = Py::Int(0i64); if py::gt(&t2327, &t2328)? { true } else { false } }) }; if py::ne(&t2323, &t2324) { true } else { false } }) }).truthy() { out.push(Py::Int(1i64)); }} Py::list(out) }, None)? } else { v_rhp.clone() });
    v_col = Py::Float(0.0);
    if (v_fc.clone()).truthy() {
        v_want = py::getitem(&f_routh_hurwitz(py::getitem(&v_inputs.clone(), &Py::str("coefficients"))?)?, &Py::str("first_column"))?;
        v_col = (if ({ let t2329 = py::len(&v_want.clone())?; Py::Bool({ let t2330 = py::len(&v_fc.clone())?; if py::ne(&t2329, &t2330) { true } else { false } }) }).truthy() { Py::Float(1.0) } else { py::minmax(&[{ let mut out = vec![]; for t2331 in py::iter(&(py::zip(&[v_fc.clone(), v_want.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t2332 = (t2331).items()?;
if t2332.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t2332[0].clone();
v_b = t2332[1].clone();
out.push(f__rel(v_a.clone(), v_b.clone())?); } Py::list(out) }], true)? });
    }
    return Ok(py::add(&py::add(&py::add(&py::abs(&py::sub(&py::getitem(&v_outputs.clone(), &Py::str("rhp_roots"))?, &v_rhp.clone())?)?, &py::abs(&py::sub(&v_ch.clone(), &v_rhp.clone())?)?)?, &{ let t2333 = py::getitem(&v_outputs.clone(), &Py::str("stable"))?; Py::Bool({ let t2334 = { let t2335 = v_rhp.clone(); Py::Bool({ let t2336 = Py::Int(0i64); if py::eq(&t2335, &t2336) { true } else { false } }) }; if py::ne(&t2333, &t2334) { true } else { false } }) })?, &v_col.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_self_l(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::add(&f__rel(py::mul(&py::getitem(&v_o.clone(), &Py::str("L"))?, &py::getitem(&v_i.clone(), &Py::str("I"))?)?, py::getitem(&v_o.clone(), &Py::str("flux_linkage"))?)?, &f__rel(py::getitem(&v_o.clone(), &Py::str("flux_linkage"))?, py::mul(&py::getitem(&v_i.clone(), &Py::str("N"))?, &py::getitem(&v_i.clone(), &Py::str("flux"))?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_series_ac(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_C: Py = Py::None;
    let mut v_L: Py = Py::None;
    let mut v_R: Py = Py::None;
    let mut v_S: Py = Py::None;
    let mut v_V: Py = Py::None;
    let mut v_cur: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_xc: Py = Py::None;
    let t2413 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("V"))?, py::getitem(&v_i.clone(), &Py::str("R"))?])).items()?;
    if t2413.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_V = t2413[0].clone();
    v_R = t2413[1].clone();
    let t2414 = (Py::tuple(vec![py::method(&v_i.clone(), "get", &[Py::str("L"), Py::Float(0.0)])?, py::method(&v_i.clone(), "get", &[Py::str("C"), Py::Float(0.0)])?])).items()?;
    if t2414.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_L = t2414[0].clone();
    v_C = t2414[1].clone();
    v_w = py::mul(&c__TWO_PI()?, &py::getitem(&v_i.clone(), &Py::str("f"))?)?;
    let t2415 = (Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("I"))?, py::getitem(&v_o.clone(), &Py::str("S"))?])).items()?;
    if t2415.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_cur = t2415[0].clone();
    v_S = t2415[1].clone();
    v_xc = (if ({ let t2416 = v_C.clone(); Py::Bool({ let t2417 = Py::Int(0i64); if py::gt(&t2416, &t2417)? { true } else { false } }) }).truthy() { py::truediv(&Py::Float(1.0), &py::mul(&v_w.clone(), &v_C.clone())?)? } else { Py::Float(0.0) });
    v_d = py::sub(&py::getitem(&v_o.clone(), &Py::str("VL"))?, &py::getitem(&v_o.clone(), &Py::str("VC"))?)?;
    v_want = (if ({ let t2418 = v_d.clone(); Py::Bool({ let t2419 = py::mul(&Py::Float(1e-12), &v_V.clone())?; if py::gt(&t2418, &t2419)? { true } else { false } }) }).truthy() { Py::str("lagging") } else { (if ({ let t2420 = v_d.clone(); Py::Bool({ let t2421 = py::mul(&py::neg(&Py::Float(1e-12))?, &v_V.clone())?; if py::lt(&t2420, &t2421)? { true } else { false } }) }).truthy() { Py::str("leading") } else { Py::str("unity") }) });
    return Ok(py::add(&py::add(&py::add(&py::add(&py::add(&py::add(&py::add(&py::add(&py::add(&py::add(&py::add(&py::add(&py::truediv(&py::abs(&py::sub(&py::hypot(&[py::getitem(&v_o.clone(), &Py::str("VR"))?, v_d.clone()])?, &v_V.clone())?)?, &v_V.clone())?, &f__rel(py::getitem(&v_o.clone(), &Py::str("VR"))?, py::mul(&v_cur.clone(), &v_R.clone())?)?)?, &py::truediv(&py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("VL"))?, &py::mul(&py::mul(&v_cur.clone(), &v_w.clone())?, &v_L.clone())?)?)?, &v_V.clone())?)?, &py::truediv(&py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("VC"))?, &py::mul(&v_cur.clone(), &v_xc.clone())?)?)?, &v_V.clone())?)?, &f__rel(py::mul(&py::getitem(&v_o.clone(), &Py::str("Z"))?, &v_cur.clone())?, v_V.clone())?)?, &py::truediv(&py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("XL"))?, &py::mul(&v_w.clone(), &v_L.clone())?)?)?, &py::getitem(&v_o.clone(), &Py::str("Z"))?)?)?, &py::truediv(&py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("XC"))?, &v_xc.clone())?)?, &py::getitem(&v_o.clone(), &Py::str("Z"))?)?)?, &f__rel(v_S.clone(), py::mul(&v_V.clone(), &v_cur.clone())?)?)?, &py::truediv(&py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("P"))?, &py::mul(&py::mul(&v_V.clone(), &v_cur.clone())?, &py::getitem(&v_o.clone(), &Py::str("power_factor"))?)?)?)?, &v_S.clone())?)?, &py::truediv(&py::abs(&py::sub(&py::add(&py::pow(&py::getitem(&v_o.clone(), &Py::str("P"))?, &Py::Int(2i64))?, &py::pow(&py::getitem(&v_o.clone(), &Py::str("Q"))?, &Py::Int(2i64))?)?, &py::mul(&v_S.clone(), &v_S.clone())?)?)?, &py::mul(&v_S.clone(), &v_S.clone())?)?)?, &py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("power_factor"))?, &py::math1("cos", &py::math1("radians", &py::getitem(&v_o.clone(), &Py::str("phase_deg"))?)?)?)?)?)?, &py::truediv(&py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("Q"))?, &py::mul(&v_cur.clone(), &v_d.clone())?)?)?, &v_S.clone())?)?, &py::float(&{ let t2422 = py::getitem(&v_o.clone(), &Py::str("nature"))?; Py::Bool({ let t2423 = v_want.clone(); if py::ne(&t2422, &t2423) { true } else { false } }) })?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_sin_time(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_T: Py = Py::None;
    let mut v_Vm: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_first: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_phi: Py = Py::None;
    let mut v_res: Py = Py::None;
    let mut v_t: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let t2547 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("Vm"))?, py::getitem(&v_i.clone(), &Py::str("w"))?, py::getitem(&v_i.clone(), &Py::str("phi"))?, py::getitem(&v_i.clone(), &Py::str("v"))?])).items()?;
    if t2547.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_Vm = t2547[0].clone();
    v_w = t2547[1].clone();
    v_phi = t2547[2].clone();
    v_v = t2547[3].clone();
    v_t = py::getitem(&v_o.clone(), &Py::str("t"))?;
    v_T = py::truediv(&c__TWO_PI()?, &v_w.clone())?;
    v_res = py::add(&py::add(&py::truediv(&py::abs(&py::sub(&py::mul(&v_Vm.clone(), &py::math1("sin", &py::add(&py::mul(&v_w.clone(), &v_t.clone())?, &v_phi.clone())?)?)?, &v_v.clone())?)?, &v_Vm.clone())?, &f__rel(py::getitem(&v_o.clone(), &Py::str("T"))?, v_T.clone())?)?, &py::float(&{ let t2552 = { let t2548 = v_t.clone(); Py::Bool({ let t2549 = Py::Int(0i64); if py::lt(&t2548, &t2549)? { true } else { false } }) }; if t2552.truthy() { t2552 } else { { let t2550 = v_t.clone(); Py::Bool({ let t2551 = v_T.clone(); if py::ge(&t2550, &t2551)? { true } else { false } }) } } })?)?;
    v_first = py::sub(&py::mul(&v_Vm.clone(), &py::math1("sin", &v_phi.clone())?)?, &v_v.clone())?;
    for t2553 in py::range_iter(&[(Py::Int(1i64)).i()?, (Py::Int(200i64)).i()?]) {
        v_k = t2553;
        v_g = py::sub(&py::mul(&v_Vm.clone(), &py::math1("sin", &py::add(&py::truediv(&py::mul(&py::mul(&v_w.clone(), &v_t.clone())?, &v_k.clone())?, &Py::Float(200.0))?, &v_phi.clone())?)?)?, &v_v.clone())?;
        if ({ let t2561 = { let t2554 = py::mul(&v_g.clone(), &v_first.clone())?; Py::Bool({ let t2555 = Py::Int(0i64); if py::lt(&t2554, &t2555)? { true } else { false } }) }; if !t2561.truthy() { t2561 } else { { let t2560 = { let t2556 = py::abs(&v_g.clone())?; Py::Bool({ let t2557 = py::mul(&Py::Float(1e-09), &v_Vm.clone())?; if py::gt(&t2556, &t2557)? { true } else { false } }) }; if !t2560.truthy() { t2560 } else { { let t2558 = py::abs(&v_first.clone())?; Py::Bool({ let t2559 = py::mul(&Py::Float(1e-09), &v_Vm.clone())?; if py::gt(&t2558, &t2559)? { true } else { false } }) } } } } }).truthy() {
            v_res = py::iadd(&v_res, &Py::Float(1.0))?;
            break;
        }
    }
    v_c = py::math1("cos", &py::add(&py::mul(&v_w.clone(), &v_t.clone())?, &v_phi.clone())?)?;
    v_want = (if ({ let t2562 = v_c.clone(); Py::Bool({ let t2563 = Py::Float(1e-09); if py::gt(&t2562, &t2563)? { true } else { false } }) }).truthy() { Py::str("rising") } else { (if ({ let t2564 = v_c.clone(); Py::Bool({ let t2565 = py::neg(&Py::Float(1e-09))?; if py::lt(&t2564, &t2565)? { true } else { false } }) }).truthy() { Py::str("falling") } else { Py::str("peak") }) });
    return Ok(py::add(&v_res.clone(), &py::float(&{ let t2566 = py::getitem(&v_o.clone(), &Py::str("direction"))?; Py::Bool({ let t2567 = v_want.clone(); if py::ne(&t2566, &t2567) { true } else { false } }) })?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_sinusoid(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_Vm: Py = Py::None;
    let mut v_phi: Py = Py::None;
    let mut v_t: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let t2580 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("Vm"))?, py::getitem(&v_i.clone(), &Py::str("w"))?, py::getitem(&v_i.clone(), &Py::str("phi"))?, py::getitem(&v_i.clone(), &Py::str("t"))?])).items()?;
    if t2580.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_Vm = t2580[0].clone();
    v_w = t2580[1].clone();
    v_phi = t2580[2].clone();
    v_t = t2580[3].clone();
    v_want = (if ({ let t2581 = v_phi.clone(); Py::Bool({ let t2582 = Py::Int(0i64); if py::eq(&t2581, &t2582) { true } else { false } }) }).truthy() { Py::str("in phase") } else { (if ({ let t2583 = v_phi.clone(); Py::Bool({ let t2584 = Py::Int(0i64); if py::gt(&t2583, &t2584)? { true } else { false } }) }).truthy() { Py::str("leads") } else { Py::str("lags") }) });
    return Ok(py::add(&py::add(&py::add(&py::add(&py::add(&py::add(&py::add(&py::add(&py::add(&py::truediv(&py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("v"))?, &py::mul(&v_Vm.clone(), &py::math1("sin", &py::add(&py::mul(&v_w.clone(), &v_t.clone())?, &v_phi.clone())?)?)?)?)?, &v_Vm.clone())?, &f__rel(py::mul(&py::getitem(&v_o.clone(), &Py::str("f"))?, &c__TWO_PI()?)?, v_w.clone())?)?, &f__rel(py::mul(&py::getitem(&v_o.clone(), &Py::str("T"))?, &py::getitem(&v_o.clone(), &Py::str("f"))?)?, Py::Float(1.0))?)?, &py::truediv(&py::abs(&py::sub(&py::math1("radians", &py::getitem(&v_o.clone(), &Py::str("phase_deg"))?)?, &v_phi.clone())?)?, &py::minmax(&[Py::Float(1.0), py::abs(&v_phi.clone())?], true)?)?)?, &py::truediv(&py::abs(&py::sub(&py::mul(&py::getitem(&v_o.clone(), &Py::str("time_shift"))?, &v_w.clone())?, &v_phi.clone())?)?, &py::minmax(&[Py::Float(1.0), py::abs(&v_phi.clone())?], true)?)?)?, &f__rel(py::mul(&py::getitem(&v_o.clone(), &Py::str("rms"))?, &py::getitem(&v_o.clone(), &Py::str("peak_factor"))?)?, v_Vm.clone())?)?, &f__rel(py::pow(&py::getitem(&v_o.clone(), &Py::str("peak_factor"))?, &Py::Int(2i64))?, Py::Float(2.0))?)?, &f__rel(py::mul(&py::getitem(&v_o.clone(), &Py::str("average"))?, &Py::Float(std::f64::consts::PI))?, py::mul(&Py::Float(2.0), &v_Vm.clone())?)?)?, &f__rel(py::mul(&py::getitem(&v_o.clone(), &Py::str("form_factor"))?, &py::getitem(&v_o.clone(), &Py::str("average"))?)?, py::getitem(&v_o.clone(), &Py::str("rms"))?)?)?, &py::float(&{ let t2585 = py::getitem(&v_o.clone(), &Py::str("relation"))?; Py::Bool({ let t2586 = v_want.clone(); if py::ne(&t2585, &t2586) { true } else { false } }) })?)?);
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
    let t2621 = (Py::tuple(vec![py::getitem(&v_inputs.clone(), &Py::str("A"))?, py::getitem(&v_inputs.clone(), &Py::str("B"))?, py::getitem(&v_inputs.clone(), &Py::str("C"))?, py::getitem(&v_inputs.clone(), &Py::str("D"))?])).items()?;
    if t2621.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_A = t2621[0].clone();
    v_B = t2621[1].clone();
    v_C = t2621[2].clone();
    v_D = t2621[3].clone();
    v_n = py::len(&v_A.clone())?;
    v_worst = Py::Float(0.0);
    for t2622 in py::iter(&(Py::tuple(vec![py::complex(&[Py::Float(0.37), Py::Float(1.3)])?, py::complex(&[py::neg(&Py::Float(2.1))?, Py::Float(0.7)])?])))? {
        v_s = t2622;
        v_M = { let mut out = vec![]; for t2623 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t2623;
out.push({ let mut out = vec![]; for t2624 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t2624;
out.push(py::sub(&(if ({ let t2625 = v_i.clone(); Py::Bool({ let t2626 = v_j.clone(); if py::eq(&t2625, &t2626) { true } else { false } }) }).truthy() { v_s.clone() } else { Py::Int(0i64) }), &py::getitem(&py::getitem(&v_A.clone(), &v_i.clone())?, &v_j.clone())?)?); } Py::list(out) }); } Py::list(out) };
        let t2655: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                v_x = f__gauss(v_M.clone(), { let mut out = vec![]; for t2654 in py::iter(&(v_B.clone()))? { let mut v_b: Py; 
v_b = t2654;
out.push(py::complex(&[v_b.clone()])?); } Py::list(out) })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t2655 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (matches!(e, PyErr::Value(_))) => {
                continue;
            }
            Err(e) => return Err(e),
        }
        v_g = py::add(&py::sum(&{ let mut out = vec![]; for t2656 in py::iter(&(py::zip(&[v_C.clone(), v_x.clone()])?))? { let mut v_c: Py; let mut v_xi: Py; 
let t2657 = (t2656).items()?;
if t2657.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_c = t2657[0].clone();
v_xi = t2657[1].clone();
out.push(py::mul(&v_c.clone(), &v_xi.clone())?); } Py::list(out) }, None)?, &v_D.clone())?;
        v_h = py::truediv(&f__pval(py::getitem(&v_outputs.clone(), &Py::str("num"))?, v_s.clone())?, &f__pval(py::getitem(&v_outputs.clone(), &Py::str("den"))?, v_s.clone())?)?;
        v_worst = py::minmax(&[v_worst.clone(), py::truediv(&py::abs(&py::sub(&v_g.clone(), &v_h.clone())?)?, &py::minmax(&[py::abs(&v_g.clone())?, Py::Float(1e-12)], true)?)?], true)?;
    }
    return Ok(py::add(&py::add(&py::add(&v_worst.clone(), &py::abs(&py::sub(&py::getitem(&py::getitem(&v_outputs.clone(), &Py::str("den"))?, &Py::Int(0i64))?, &Py::Float(1.0))?)?)?, &py::abs(&py::sub(&py::getitem(&v_outputs.clone(), &Py::str("order"))?, &v_n.clone())?)?)?, &py::float(&{ let t2659 = py::len(&py::getitem(&v_outputs.clone(), &Py::str("num"))?)?; Py::Bool({ let t2660 = py::add(&v_n.clone(), &Py::Int(1i64))?; if py::ne(&t2659, &t2660) { true } else { false } }) })?)?);
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
    v_num = { let mut out = vec![]; for t2746 in py::iter(&(f__ptrim(py::getitem(&v_inputs.clone(), &Py::str("num"))?)?))? { let mut v_x: Py; 
v_x = t2746;
out.push(py::fraction(&[v_x.clone()])?); } Py::list(out) };
    v_den = { let mut out = vec![]; for t2747 in py::iter(&(f__ptrim(py::getitem(&v_inputs.clone(), &Py::str("den"))?)?))? { let mut v_x: Py; 
v_x = t2747;
out.push(py::fraction(&[v_x.clone()])?); } Py::list(out) };
    v_k = py::getitem(&c__INPUT_ORDER()?, &py::getitem(&v_inputs.clone(), &Py::str("input"))?)?;
    v_top = py::slice(&v_den.clone(), None, None, Some((py::neg(&Py::Int(1i64))?).i()?))?;
    let t2748 = (Py::tuple(vec![py::slice(&v_num.clone(), None, None, Some((py::neg(&Py::Int(1i64))?).i()?))?, py::slice(&v_den.clone(), None, None, Some((py::neg(&Py::Int(1i64))?).i()?))?])).items()?;
    if t2748.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_n_ = t2748[0].clone();
    v_d_ = t2748[1].clone();
    v_bot = py::add(&py::mul(&Py::list(vec![py::fraction(&[Py::Int(0i64)])?]), &v_k.clone())?, &{ let mut out = vec![]; for t2749 in py::range_iter(&[(py::minmax(&[py::len(&v_d_.clone())?, py::len(&v_n_.clone())?], true)?).i()?]) { let mut v_i: Py; 
v_i = t2749;
out.push(py::add(&(if ({ let t2750 = v_i.clone(); Py::Bool({ let t2751 = py::len(&v_d_.clone())?; if py::lt(&t2750, &t2751)? { true } else { false } }) }).truthy() { py::getitem(&v_d_.clone(), &v_i.clone())? } else { Py::Int(0i64) }), &(if ({ let t2752 = v_i.clone(); Py::Bool({ let t2753 = py::len(&v_n_.clone())?; if py::lt(&t2752, &t2753)? { true } else { false } }) }).truthy() { py::getitem(&v_n_.clone(), &v_i.clone())? } else { Py::Int(0i64) }))?); } Py::list(out) })?;
    v_a = (match ('g2754: { let mut out = vec![]; for t2755 in py::iter(&(py::enumerate(&v_top.clone(), 0)?))? { let mut v_i: Py; let mut v_x: Py; 
let t2756 = (t2755).items()?;
if t2756.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_i = t2756[0].clone();
v_x = t2756[1].clone();
if ({ let t2757 = v_x.clone(); Py::Bool({ let t2758 = Py::Int(0i64); if py::ne(&t2757, &t2758) { true } else { false } }) }).truthy() { out.push(v_i.clone()); break 'g2754 Py::list(out); }} Py::list(out) }).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) });
    v_b = (match ('g2759: { let mut out = vec![]; for t2760 in py::iter(&(py::enumerate(&v_bot.clone(), 0)?))? { let mut v_i: Py; let mut v_x: Py; 
let t2761 = (t2760).items()?;
if t2761.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_i = t2761[0].clone();
v_x = t2761[1].clone();
if ({ let t2762 = v_x.clone(); Py::Bool({ let t2763 = Py::Int(0i64); if py::ne(&t2762, &t2763) { true } else { false } }) }).truthy() { out.push(v_i.clone()); break 'g2759 Py::list(out); }} Py::list(out) }).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) });
    v_want = (if ({ let t2764 = v_a.clone(); Py::Bool({ let t2765 = v_b.clone(); if py::gt(&t2764, &t2765)? { true } else { false } }) }).truthy() { py::fraction(&[Py::Int(0i64)])? } else { (if ({ let t2766 = v_a.clone(); Py::Bool({ let t2767 = v_b.clone(); if py::eq(&t2766, &t2767) { true } else { false } }) }).truthy() { py::truediv(&py::getitem(&v_top.clone(), &v_a.clone())?, &py::getitem(&v_bot.clone(), &v_b.clone())?)? } else { Py::None }) });
    v_ess = py::getitem(&v_outputs.clone(), &Py::str("steady_state_error"))?;
    if ({ let t2768 = v_want.clone(); Py::Bool({ let t2769 = Py::None; if py::is(&t2768, &t2769) { true } else { false } }) }).truthy() {
        return Ok((if ({ let t2770 = v_ess.clone(); Py::Bool({ let t2771 = Py::str("infinite"); if py::eq(&t2770, &t2771) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { Py::Float(1.0) }));
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
    let t2359 = (Py::tuple(vec![py::getitem(&v_inputs.clone(), &Py::str("zeta"))?, py::getitem(&v_inputs.clone(), &Py::str("wn"))?])).items()?;
    if t2359.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_z = t2359[0].clone();
    v_wn = t2359[1].clone();
    v_q = py::math1("sqrt", &py::sub(&Py::Int(1i64), &py::mul(&v_z.clone(), &v_z.clone())?)?)?;
    let t2360 = (Py::tuple(vec![py::mul(&v_wn.clone(), &v_q.clone())?, py::mul(&v_z.clone(), &v_wn.clone())?])).items()?;
    if t2360.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_wd = t2360[0].clone();
    v_sg = t2360[1].clone();
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
    let t2361 = (Py::tuple(vec![py::getitem(&v_outputs.clone(), &Py::str("peak_time"))?, py::getitem(&v_outputs.clone(), &Py::str("settling_time"))?])).items()?;
    if t2361.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_tp = t2361[0].clone();
    v_ts = t2361[1].clone();
    v_res = Py::list(vec![py::truediv(&py::abs(&py::sub(&py::mul(&v_wd.clone(), &v_tp.clone())?, &Py::Float(std::f64::consts::PI))?)?, &Py::Float(std::f64::consts::PI))?, py::abs(&py::sub(&py::sub(&py::call(&v_y.clone(), &[v_tp.clone()])?, &Py::Int(1i64))?, &py::getitem(&v_outputs.clone(), &Py::str("overshoot"))?)?)?, py::truediv(&py::abs(&py::sub(&py::getitem(&v_outputs.clone(), &Py::str("percent_overshoot"))?, &py::mul(&Py::Int(100i64), &py::getitem(&v_outputs.clone(), &Py::str("overshoot"))?)?)?)?, &Py::Int(100i64))?, py::abs(&py::sub(&py::call(&v_y.clone(), &[py::getitem(&v_outputs.clone(), &Py::str("t10"))?])?, &Py::Float(0.1))?)?, py::abs(&py::sub(&py::call(&v_y.clone(), &[py::getitem(&v_outputs.clone(), &Py::str("t90"))?])?, &Py::Float(0.9))?)?, py::mul(&py::abs(&py::sub(&py::getitem(&v_outputs.clone(), &Py::str("rise_time"))?, &py::sub(&py::getitem(&v_outputs.clone(), &Py::str("t90"))?, &py::getitem(&v_outputs.clone(), &Py::str("t10"))?)?)?)?, &v_wn.clone())?, py::abs(&py::sub(&py::abs(&py::sub(&py::call(&v_y.clone(), &[v_ts.clone()])?, &Py::Int(1i64))?)?, &Py::Float(0.02))?)?, py::minmax(&[Py::Float(0.0), py::sub(&py::math1("exp", &py::truediv(&py::mul(&py::mul(&py::neg(&v_sg.clone())?, &py::add(&py::math1("floor", &py::truediv(&py::mul(&v_ts.clone(), &v_wd.clone())?, &Py::Float(std::f64::consts::PI))?)?, &Py::Int(1i64))?)?, &Py::Float(std::f64::consts::PI))?, &v_wd.clone())?)?, &Py::Float(0.02))?], true)?]);
    return Ok(py::minmax(&[v_res.clone()], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_summing(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_scale: Py = Py::None;
    let mut v_terms: Py = Py::None;
    v_terms = py::add(&{ let mut out = vec![]; for t2783 in py::iter(&(py::zip(&[py::getitem(&v_inputs.clone(), &Py::str("V"))?, py::getitem(&v_inputs.clone(), &Py::str("R"))?])?))? { let mut v_r: Py; let mut v_v: Py; 
let t2784 = (t2783).items()?;
if t2784.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_v = t2784[0].clone();
v_r = t2784[1].clone();
out.push(py::truediv(&v_v.clone(), &v_r.clone())?); } Py::list(out) }, &Py::list(vec![py::truediv(&py::getitem(&v_outputs.clone(), &Py::str("Vout"))?, &py::getitem(&v_inputs.clone(), &Py::str("Rf"))?)?]))?;
    v_scale = { let t2786 = py::fsum(&{ let mut out = vec![]; for t2785 in py::iter(&(v_terms.clone()))? { let mut v_t: Py; 
v_t = t2785;
out.push(py::abs(&v_t.clone())?); } Py::list(out) })?; if t2786.truthy() { t2786 } else { Py::Float(1.0) } };
    return Ok(py::add(&py::truediv(&py::abs(&py::fsum(&v_terms.clone())?)?, &v_scale.clone())?, &py::minmax(&[{ let mut out = vec![]; for t2788 in py::iter(&(py::zip(&[py::method(&v_outputs.clone(), "get", &[Py::str("gains"), { let mut out = vec![]; for t2787 in py::iter(&(py::getitem(&v_inputs.clone(), &Py::str("R"))?))? { let mut v_r: Py; 
v_r = t2787;
out.push(py::truediv(&py::neg(&py::getitem(&v_inputs.clone(), &Py::str("Rf"))?)?, &v_r.clone())?); } Py::list(out) }])?, py::getitem(&v_inputs.clone(), &Py::str("R"))?])?))? { let mut v_g: Py; let mut v_r: Py; 
let t2789 = (t2788).items()?;
if t2789.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_g = t2789[0].clone();
v_r = t2789[1].clone();
out.push(py::abs(&py::add(&v_g.clone(), &py::truediv(&py::getitem(&v_inputs.clone(), &Py::str("Rf"))?, &v_r.clone())?)?)?); } Py::list(out) }], true)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_tc_enc(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_n: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_val: Py = Py::None;
    let t3119 = (Py::tuple(vec![py::getitem(&v_outputs.clone(), &Py::str("pattern"))?, py::int(&py::getitem(&v_inputs.clone(), &Py::str("bits"))?)?])).items()?;
    if t3119.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_p = t3119[0].clone();
    v_n = t3119[1].clone();
    if ({ let t3122 = { let t3120 = py::len(&v_p.clone())?; Py::Bool({ let t3121 = v_n.clone(); if py::ne(&t3120, &t3121) { true } else { false } }) }; if t3122.truthy() { t3122 } else { py::sub(&py::set(&v_p.clone())?, &py::set(&Py::list(vec![Py::str("0"), Py::str("1")]))?)? } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_val = py::sub(&py::int_base(&v_p.clone(), &Py::Int(2i64))?, &(if ({ let t3123 = py::getitem(&v_p.clone(), &Py::Int(0i64))?; Py::Bool({ let t3124 = Py::str("1"); if py::eq(&t3123, &t3124) { true } else { false } }) }).truthy() { py::pow(&Py::Int(2i64), &v_n.clone())? } else { Py::Int(0i64) }))?;
    return Ok(py::add(&py::add(&py::float(&{ let t3125 = v_val.clone(); Py::Bool({ let t3126 = py::int(&py::getitem(&v_inputs.clone(), &Py::str("value"))?)?; if py::ne(&t3125, &t3126) { true } else { false } }) })?, &py::float(&{ let t3127 = py::int_base(&py::getitem(&v_outputs.clone(), &Py::str("hexadecimal"))?, &Py::Int(16i64))?; Py::Bool({ let t3128 = py::int_base(&v_p.clone(), &Py::Int(2i64))?; if py::ne(&t3127, &t3128) { true } else { false } }) })?)?, &py::abs(&py::sub(&py::getitem(&v_outputs.clone(), &Py::str("unsigned"))?, &py::int_base(&v_p.clone(), &Py::Int(2i64))?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_tc_from(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_R1: Py = Py::None;
    let mut v_R2: Py = Py::None;
    let mut v_s: Py = Py::None;
    let t2797 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("R1"))?, py::getitem(&v_i.clone(), &Py::str("R2"))?])).items()?;
    if t2797.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_R1 = t2797[0].clone();
    v_R2 = t2797[1].clone();
    v_s = py::getitem(&v_o.clone(), &Py::str("slope"))?;
    return Ok(py::add(&py::add(&py::truediv(&py::abs(&py::sub(&py::mul(&v_s.clone(), &py::sub(&py::getitem(&v_i.clone(), &Py::str("T2"))?, &py::getitem(&v_i.clone(), &Py::str("T1"))?)?)?, &py::sub(&v_R2.clone(), &v_R1.clone())?)?)?, &py::minmax(&[py::abs(&py::sub(&v_R2.clone(), &v_R1.clone())?)?, py::mul(&Py::Float(1e-12), &v_R1.clone())?], true)?)?, &py::truediv(&py::abs(&py::sub(&py::mul(&py::getitem(&v_o.clone(), &Py::str("alpha1"))?, &v_R1.clone())?, &v_s.clone())?)?, &py::minmax(&[py::abs(&v_s.clone())?, Py::Float(1e-300)], true)?)?)?, &py::truediv(&py::abs(&py::sub(&py::mul(&py::getitem(&v_o.clone(), &Py::str("alpha2"))?, &v_R2.clone())?, &v_s.clone())?)?, &py::minmax(&[py::abs(&v_s.clone())?, Py::Float(1e-300)], true)?)?)?);
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
    let t2931 = (Py::tuple(vec![py::getitem(&v_outputs.clone(), &Py::str("Vth"))?, py::getitem(&v_outputs.clone(), &Py::str("Rth"))?])).items()?;
    if t2931.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_vth = t2931[0].clone();
    v_rth = t2931[1].clone();
    let t2932 = (Py::tuple(vec![py::getitem(&v_inputs.clone(), &Py::str("node_a"))?, py::getitem(&v_inputs.clone(), &Py::str("node_b"))?])).items()?;
    if t2932.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t2932[0].clone();
    v_b = t2932[1].clone();
    v_els = f__netlist(py::getitem(&v_inputs.clone(), &Py::str("netlist"))?)?;
    let t2933 = (f__mna(v_els.clone())?).items()?;
    if t2933.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_v1 = t2933[0].clone();
    v__ = t2933[1].clone();
    let t2934 = (f__mna(py::add(&v_els.clone(), &Py::list(vec![Py::tuple(vec![Py::str("I"), Py::str("Itest_check"), v_b.clone(), v_a.clone(), Py::Float(1.0)])]))?)?).items()?;
    if t2934.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_v2 = t2934[0].clone();
    v__ = t2934[1].clone();
    let t2935 = (Py::tuple(vec![py::sub(&py::getitem(&v_v1.clone(), &v_a.clone())?, &py::getitem(&v_v1.clone(), &v_b.clone())?)?, py::sub(&py::getitem(&v_v2.clone(), &v_a.clone())?, &py::getitem(&v_v2.clone(), &v_b.clone())?)?])).items()?;
    if t2935.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_o1 = t2935[0].clone();
    v_o2 = t2935[1].clone();
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
    v_on = { let mut out = vec![]; for t3077 in py::range_iter(&[(py::pow(&Py::Int(2i64), &v_n.clone())?).i()?]) { let mut v_m: Py; 
v_m = t3077;
if (f__bool_eval(v_tree.clone(), { let mut out = vec![]; for t3078 in py::iter(&(py::enumerate(&v_names.clone(), 0)?))? { let mut v_i: Py; let mut v_nm: Py; 
let t3079 = (t3078).items()?;
if t3079.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_i = t3079[0].clone();
v_nm = t3079[1].clone();
out.push((v_nm.clone(), py::bitand(&py::rshift(&v_m.clone(), &py::sub(&py::sub(&v_n.clone(), &Py::Int(1i64))?, &v_i.clone())?)?, &Py::Int(1i64))?)); } Py::dict(out) })?).truthy() { out.push(v_m.clone()); }} Py::list(out) };
    let t3094 = (f__sop_check(py::getitem(&v_outputs.clone(), &Py::str("minimal_sop"))?, v_n.clone(), v_names.clone(), py::set(&v_on.clone())?, Py::tuple(vec![]))?).items()?;
    if t3094.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_bad = t3094[0].clone();
    v_t = t3094[1].clone();
    v_lit = t3094[2].clone();
    let t3095 = (f__sop_check(py::getitem(&v_outputs.clone(), &Py::str("canonical_sop"))?, v_n.clone(), v_names.clone(), py::set(&v_on.clone())?, Py::tuple(vec![]))?).items()?;
    if t3095.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_bad2 = t3095[0].clone();
    v__ = t3095[1].clone();
    v__ = t3095[2].clone();
    v_best = f__sop_out(v_n.clone(), v_names.clone(), v_on.clone(), Py::tuple(vec![]))?;
    return Ok(py::add(&py::add(&py::add(&py::add(&py::add(&py::add(&py::add(&v_bad.clone(), &v_bad2.clone())?, &py::abs(&py::sub(&py::getitem(&v_outputs.clone(), &Py::str("terms"))?, &v_t.clone())?)?)?, &py::abs(&py::sub(&py::getitem(&v_outputs.clone(), &Py::str("literals"))?, &v_lit.clone())?)?)?, &py::abs(&py::sub(&py::getitem(&v_outputs.clone(), &Py::str("terms"))?, &py::getitem(&v_best.clone(), &Py::str("terms"))?)?)?)?, &py::abs(&py::sub(&py::getitem(&v_outputs.clone(), &Py::str("literals"))?, &py::getitem(&v_best.clone(), &Py::str("literals"))?)?)?)?, &{ let t3096 = py::getitem(&v_outputs.clone(), &Py::str("minterms"))?; Py::Bool({ let t3097 = { let mut out = vec![]; for t3098 in py::iter(&(v_on.clone()))? { let mut v_m: Py; 
v_m = t3098;
out.push(py::float(&v_m.clone())?); } Py::list(out) }; if py::ne(&t3096, &t3097) { true } else { false } }) })?, &{ let t3099 = py::getitem(&v_outputs.clone(), &Py::str("variables"))?; Py::Bool({ let t3100 = py::method(&Py::str(", "), "join", &[v_names.clone()])?; if py::ne(&t3099, &t3100) { true } else { false } }) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_two_watt(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_W1: Py = Py::None;
    let mut v_W2: Py = Py::None;
    let mut v_s: Py = Py::None;
    let t3104 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("W1"))?, py::getitem(&v_i.clone(), &Py::str("W2"))?])).items()?;
    if t3104.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_W1 = t3104[0].clone();
    v_W2 = t3104[1].clone();
    v_s = py::getitem(&v_o.clone(), &Py::str("S"))?;
    return Ok(py::add(&py::add(&py::add(&py::add(&f__rel(py::getitem(&v_o.clone(), &Py::str("P"))?, py::add(&v_W1.clone(), &v_W2.clone())?)?, &py::truediv(&py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("Q"))?, &py::mul(&py::math1("sqrt", &Py::Float(3.0))?, &py::abs(&py::sub(&v_W1.clone(), &v_W2.clone())?)?)?)?)?, &v_s.clone())?)?, &py::truediv(&py::abs(&py::sub(&py::sub(&py::mul(&v_s.clone(), &v_s.clone())?, &py::pow(&py::getitem(&v_o.clone(), &Py::str("P"))?, &Py::Int(2i64))?)?, &py::pow(&py::getitem(&v_o.clone(), &Py::str("Q"))?, &Py::Int(2i64))?)?)?, &py::mul(&v_s.clone(), &v_s.clone())?)?)?, &py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("power_factor"))?, &py::truediv(&py::getitem(&v_o.clone(), &Py::str("P"))?, &v_s.clone())?)?)?)?, &py::truediv(&py::abs(&py::sub(&py::mul(&py::math1("tan", &py::math1("radians", &py::getitem(&v_o.clone(), &Py::str("phase_deg"))?)?)?, &py::getitem(&v_o.clone(), &Py::str("P"))?)?, &py::getitem(&v_o.clone(), &Py::str("Q"))?)?)?, &v_s.clone())?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_voltmeter(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_Ifs: Py = Py::None;
    let mut v_Rm: Py = Py::None;
    let mut v_V: Py = Py::None;
    let t3142 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("Ifs"))?, py::getitem(&v_i.clone(), &Py::str("Rm"))?, py::getitem(&v_i.clone(), &Py::str("V"))?])).items()?;
    if t3142.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_Ifs = t3142[0].clone();
    v_Rm = t3142[1].clone();
    v_V = t3142[2].clone();
    return Ok(py::add(&py::add(&py::add(&f__rel(py::mul(&v_Ifs.clone(), &py::add(&v_Rm.clone(), &py::getitem(&v_o.clone(), &Py::str("Rmult"))?)?)?, v_V.clone())?, &f__rel(py::getitem(&v_o.clone(), &Py::str("R_total"))?, py::add(&v_Rm.clone(), &py::getitem(&v_o.clone(), &Py::str("Rmult"))?)?)?)?, &f__rel(py::mul(&py::getitem(&v_o.clone(), &Py::str("sensitivity"))?, &v_Ifs.clone())?, Py::Float(1.0))?)?, &f__rel(py::getitem(&v_o.clone(), &Py::str("V_fs"))?, py::mul(&v_Ifs.clone(), &v_Rm.clone())?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_wave(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_er: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_mr: Py = Py::None;
    let t2145 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("eps_r"))?, py::getitem(&v_i.clone(), &Py::str("mu_r"))?, py::getitem(&v_i.clone(), &Py::str("f"))?])).items()?;
    if t2145.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_er = t2145[0].clone();
    v_mr = t2145[1].clone();
    v_f = t2145[2].clone();
    return Ok(py::add(&py::add(&py::add(&py::add(&f__rel(py::mul(&py::mul(&py::mul(&py::mul(&py::pow(&py::getitem(&v_o.clone(), &Py::str("v"))?, &Py::Int(2i64))?, &v_er.clone())?, &v_mr.clone())?, &c__EPS0()?)?, &c__MU0()?)?, Py::Float(1.0))?, &f__rel(py::mul(&py::getitem(&v_o.clone(), &Py::str("wavelength"))?, &v_f.clone())?, py::getitem(&v_o.clone(), &Py::str("v"))?)?)?, &f__rel(py::mul(&py::mul(&py::pow(&py::getitem(&v_o.clone(), &Py::str("eta"))?, &Py::Int(2i64))?, &c__EPS0()?)?, &v_er.clone())?, py::mul(&c__MU0()?, &v_mr.clone())?)?)?, &f__rel(py::mul(&py::getitem(&v_o.clone(), &Py::str("beta"))?, &py::getitem(&v_o.clone(), &Py::str("wavelength"))?)?, c__TWO_PI()?)?)?, &f__rel(py::mul(&py::getitem(&v_o.clone(), &Py::str("n"))?, &py::getitem(&v_o.clone(), &Py::str("v"))?)?, c__C0()?)?)?);
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
    let t3168 = (Py::tuple(vec![py::getitem(&v_inputs.clone(), &Py::str("Vin"))?, py::getitem(&v_inputs.clone(), &Py::str("R"))?, py::getitem(&v_inputs.clone(), &Py::str("Vz"))?, py::getitem(&v_inputs.clone(), &Py::str("RL"))?])).items()?;
    if t3168.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_Vin = t3168[0].clone();
    v_R = t3168[1].clone();
    v_Vz = t3168[2].clone();
    v_RL = t3168[3].clone();
    let t3169 = (Py::tuple(vec![py::getitem(&v_outputs.clone(), &Py::str("IR"))?, py::getitem(&v_outputs.clone(), &Py::str("IL"))?, py::getitem(&v_outputs.clone(), &Py::str("IZ"))?])).items()?;
    if t3169.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_ir = t3169[0].clone();
    v_il = t3169[1].clone();
    v_iz = t3169[2].clone();
    return Ok(py::add(&py::add(&py::add(&py::add(&py::add(&py::add(&py::truediv(&py::abs(&py::sub(&py::sub(&v_Vin.clone(), &py::mul(&v_ir.clone(), &v_R.clone())?)?, &py::getitem(&v_outputs.clone(), &Py::str("VL"))?)?)?, &v_Vin.clone())?, &py::truediv(&py::abs(&py::sub(&py::getitem(&v_outputs.clone(), &Py::str("VL"))?, &v_Vz.clone())?)?, &v_Vz.clone())?)?, &py::truediv(&py::abs(&py::sub(&py::sub(&v_ir.clone(), &v_il.clone())?, &v_iz.clone())?)?, &v_ir.clone())?)?, &py::truediv(&py::abs(&py::sub(&py::mul(&v_il.clone(), &v_RL.clone())?, &py::getitem(&v_outputs.clone(), &Py::str("VL"))?)?)?, &v_Vz.clone())?)?, &f__rel(py::getitem(&v_outputs.clone(), &Py::str("IZ_mA"))?, py::mul(&v_iz.clone(), &Py::Float(1000.0))?)?)?, &py::truediv(&py::abs(&py::sub(&py::getitem(&v_outputs.clone(), &Py::str("PZ"))?, &py::mul(&v_Vz.clone(), &v_iz.clone())?)?)?, &py::mul(&v_Vz.clone(), &v_ir.clone())?)?)?, &{ let t3170 = v_iz.clone(); Py::Bool({ let t3171 = Py::Int(0i64); if py::le(&t3170, &t3171)? { true } else { false } }) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_ac_mesh_currents(mut v_R: Py, mut v_X: Py, mut v_V_mag: Py, mut v_V_deg: Py) -> R {
    let mut v_V: Py = Py::None;
    let mut v_Z: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_row: Py = Py::None;
    v_n = py::len(&v_R.clone())?;
    if ({ let t1 = v_n.clone(); Py::Bool({ let t2 = Py::Int(0i64); if py::eq(&t1, &t2) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("give at least one mesh"))));
    }
    for t3 in py::iter(&(v_R.clone()))? {
        v_row = t3;
        if ({ let t4 = py::len(&v_row.clone())?; Py::Bool({ let t5 = v_n.clone(); if py::ne(&t4, &t5) { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("the resistance matrix must be square"))));
        }
    }
    if ({ let t6 = py::len(&v_X.clone())?; Py::Bool({ let t7 = v_n.clone(); if py::ne(&t6, &t7) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the reactance matrix must have the shape of the resistance matrix"))));
    }
    for t8 in py::iter(&(v_X.clone()))? {
        v_row = t8;
        if ({ let t9 = py::len(&v_row.clone())?; Py::Bool({ let t10 = v_n.clone(); if py::ne(&t9, &t10) { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("the reactance matrix must have the shape of the resistance matrix"))));
        }
    }
    if ({ let t15 = { let t11 = py::len(&v_V_mag.clone())?; Py::Bool({ let t12 = v_n.clone(); if py::ne(&t11, &t12) { true } else { false } }) }; if t15.truthy() { t15 } else { { let t13 = py::len(&v_V_deg.clone())?; Py::Bool({ let t14 = v_n.clone(); if py::ne(&t13, &t14) { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&["give ".to_string(), py::format(&v_n.clone(), "")?, " source magnitudes and ".to_string(), py::format(&v_n.clone(), "")?, " source angles, one per mesh".to_string()].concat()))));
    }
    for t16 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_k = t16;
        if (py::not(&{ let t20 = py::math1("isfinite", &py::getitem(&v_V_mag.clone(), &v_k.clone())?)?; if !t20.truthy() { t20 } else { { let t19 = py::math1("isfinite", &py::getitem(&v_V_deg.clone(), &v_k.clone())?)?; if !t19.truthy() { t19 } else { { let t17 = py::getitem(&v_V_mag.clone(), &v_k.clone())?; Py::Bool({ let t18 = Py::Int(0i64); if py::ge(&t17, &t18)? { true } else { false } }) } } } } })).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("source magnitudes must be zero or positive and the angles finite (write a sign as 180 degrees)"))));
        }
        for t21 in py::range_iter(&[(v_n.clone()).i()?]) {
            v_j = t21;
            if (py::not(&{ let t22 = py::math1("isfinite", &py::getitem(&py::getitem(&v_R.clone(), &v_k.clone())?, &v_j.clone())?)?; if !t22.truthy() { t22 } else { py::math1("isfinite", &py::getitem(&py::getitem(&v_X.clone(), &v_k.clone())?, &v_j.clone())?)? } })).truthy() {
                return Err(py::value_error(py::to_str(&Py::str("the matrices must hold finite numbers"))));
            }
        }
    }
    v_Z = Py::list(vec![]);
    for t23 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_i = t23;
        v_row = Py::list(vec![]);
        for t24 in py::range_iter(&[(v_n.clone()).i()?]) {
            v_j = t24;
            let _ = py::method(&v_row.clone(), "append", &[py::complex(&[py::getitem(&py::getitem(&v_R.clone(), &v_i.clone())?, &v_j.clone())?, py::getitem(&py::getitem(&v_X.clone(), &v_i.clone())?, &v_j.clone())?])?])?;
        }
        let _ = py::method(&v_Z.clone(), "append", &[v_row.clone()])?;
    }
    v_V = Py::list(vec![]);
    for t25 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_k = t25;
        let _ = py::method(&v_V.clone(), "append", &[f__rect_deg(py::getitem(&v_V_mag.clone(), &v_k.clone())?, py::getitem(&v_V_deg.clone(), &v_k.clone())?)?])?;
    }
    return Ok(f__ac_mesh_out(f__cgauss(v_Z.clone(), v_V.clone())?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_ac_node_voltages(mut v_netlist: Py, mut v_f: Py) -> R {
    let mut v_cur: Py = Py::None;
    let mut v_els: Py = Py::None;
    let mut v_volts: Py = Py::None;
    let _ = f__check_positive(Py::dict(vec![(Py::str("f"), v_f.clone())]))?;
    v_els = f__ac_netlist(v_netlist.clone())?;
    let t219 = (f__ac_mna(v_els.clone(), py::mul(&c__TWO_PI()?, &v_f.clone())?)?).items()?;
    if t219.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_volts = t219[0].clone();
    v_cur = t219[1].clone();
    return Ok(f__ac_nodes_out(v_volts.clone(), v_cur.clone(), f__ac_source_power(v_els.clone(), v_volts.clone(), v_cur.clone())?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_ac_norton_equivalent(mut v_netlist: Py, mut v_node_a: Py, mut v_node_b: Py, mut v_f: Py) -> R {
    let mut v_vth: Py = Py::None;
    let mut v_zth: Py = Py::None;
    let t433 = (f__ac_thevenin(v_netlist.clone(), v_node_a.clone(), v_node_b.clone(), v_f.clone())?).items()?;
    if t433.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_vth = t433[0].clone();
    v_zth = t433[1].clone();
    return Ok(f__ac_port_out(Py::str("In"), f__cdiv(v_vth.clone(), v_zth.clone())?, Py::str("Zn"), v_zth.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_ac_thevenin_equivalent(mut v_netlist: Py, mut v_node_a: Py, mut v_node_b: Py, mut v_f: Py) -> R {
    let mut v_vth: Py = Py::None;
    let mut v_zth: Py = Py::None;
    let t606 = (f__ac_thevenin(v_netlist.clone(), v_node_a.clone(), v_node_b.clone(), v_f.clone())?).items()?;
    if t606.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_vth = t606[0].clone();
    v_zth = t606[1].clone();
    return Ok(f__ac_port_out(Py::str("Vth"), v_vth.clone(), Py::str("Zth"), v_zth.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_adc_conversion(mut v_Vin: Py, mut v_Vref: Py, mut v_bits: Py) -> R {
    let mut v_code: Py = Py::None;
    let mut v_lsb: Py = Py::None;
    let mut v_n: Py = Py::None;
    v_n = py::int(&v_bits.clone())?;
    if (py::not(&{ let t626 = Py::Int(1i64); Py::Bool({ let t627 = v_n.clone(); if py::le(&t626, &t627)? { { let t628 = Py::Int(32i64); if py::le(&t627, &t628)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the resolution must be 1 to 32 bits"))));
    }
    let _ = f__check_positive(Py::dict(vec![(Py::str("Vref"), v_Vref.clone())]))?;
    if (py::not(&{ let t634 = Py::Int(0i64); Py::Bool({ let t635 = v_Vin.clone(); if py::le(&t634, &t635)? { { let t636 = v_Vref.clone(); if py::le(&t635, &t636)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the input must be between 0 and Vref"))));
    }
    v_lsb = py::truediv(&v_Vref.clone(), &py::pow(&Py::Int(2i64), &v_n.clone())?)?;
    v_code = py::minmax(&[py::int(&py::math1("floor", &py::truediv(&v_Vin.clone(), &v_lsb.clone())?)?)?, py::sub(&py::pow(&Py::Int(2i64), &v_n.clone())?, &Py::Int(1i64))?], false)?;
    return Ok(Py::dict(vec![(Py::str("code"), py::float(&v_code.clone())?), (Py::str("binary"), Py::str(&py::format(&v_code.clone(), &(Py::str(&["0".to_string(), py::format(&v_n.clone(), "")?, "b".to_string()].concat())).as_str()?)?)), (Py::str("lsb"), v_lsb.clone()), (Py::str("lsb_mV"), py::mul(&v_lsb.clone(), &Py::Float(1000.0))?), (Py::str("quantized"), py::mul(&v_code.clone(), &v_lsb.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_ammeter_shunt(mut v_Ifs: Py, mut v_Rm: Py, mut v_I: Py) -> R {
    let mut v_rsh: Py = Py::None;
    let mut v_vfs: Py = Py::None;
    let _ = f__check_positive(Py::dict(vec![(Py::str("Ifs"), v_Ifs.clone()), (Py::str("Rm"), v_Rm.clone()), (Py::str("I"), v_I.clone())]))?;
    if ({ let t653 = v_I.clone(); Py::Bool({ let t654 = v_Ifs.clone(); if py::le(&t653, &t654)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the new range must be larger than the meter's full-scale current"))));
    }
    v_vfs = py::mul(&v_Ifs.clone(), &v_Rm.clone())?;
    v_rsh = py::truediv(&v_vfs.clone(), &py::sub(&v_I.clone(), &v_Ifs.clone())?)?;
    return Ok(Py::dict(vec![(Py::str("Rsh"), v_rsh.clone()), (Py::str("multiplying_factor"), py::truediv(&v_I.clone(), &v_Ifs.clone())?), (Py::str("V_fs"), v_vfs.clone()), (Py::str("R_total"), py::truediv(&v_vfs.clone(), &v_I.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_base_conversion(mut v_value: Py, mut v_base: Py) -> R {
    let mut v_b: Py = Py::None;
    let mut v_n: Py = Py::None;
    v_b = py::getitem(&c__BASES()?, &Py::str(&py::to_str(&v_base.clone())))?;
    v_n = py::int_base(&f__clean_digits(v_value.clone(), v_b.clone())?, &v_b.clone())?;
    if ({ let t664 = v_n.clone(); Py::Bool({ let t665 = py::pow(&Py::Int(2i64), &Py::Int(53i64))?; if py::ge(&t664, &t665)? { true } else { false } }) }).truthy() {
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
    if ({ let t677 = v_ib.clone(); Py::Bool({ let t678 = Py::Int(0i64); if py::le(&t677, &t678)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the base-emitter junction is not forward biased: the transistor is cut off"))));
    }
    let t679 = (Py::tuple(vec![py::mul(&v_beta.clone(), &v_ib.clone())?, py::mul(&py::add(&v_beta.clone(), &Py::Int(1i64))?, &v_ib.clone())?])).items()?;
    if t679.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_ic = t679[0].clone();
    v_ie = t679[1].clone();
    v_ve = py::mul(&v_ie.clone(), &v_RE.clone())?;
    v_vc = py::sub(&v_VCC.clone(), &py::mul(&v_ic.clone(), &v_RC.clone())?)?;
    v_vce = py::sub(&v_vc.clone(), &v_ve.clone())?;
    if ({ let t680 = v_vce.clone(); Py::Bool({ let t681 = Py::Float(0.2); if py::lt(&t680, &t681)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("VCE would be below 0.2 V: the transistor is saturated and the active-region model does not apply"))));
    }
    return Ok(Py::dict(vec![(Py::str("IC"), v_ic.clone()), (Py::str("IC_mA"), py::mul(&v_ic.clone(), &Py::Float(1000.0))?), (Py::str("IB"), v_ib.clone()), (Py::str("IE"), v_ie.clone()), (Py::str("VB"), py::add(&v_ve.clone(), &v_VBE.clone())?), (Py::str("VE"), v_ve.clone()), (Py::str("VC"), v_vc.clone()), (Py::str("VCE"), v_vce.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_bode_point(mut v_num: Py, mut v_den: Py, mut v_w: Py) -> R {
    let mut v_g: Py = Py::None;
    let t692 = (Py::tuple(vec![f__ptrim(v_num.clone())?, f__ptrim(v_den.clone())?])).items()?;
    if t692.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_num = t692[0].clone();
    v_den = t692[1].clone();
    if (Py::Bool(({ let mut out = vec![]; for t693 in py::iter(&(v_den.clone()))? { let mut v_x: Py; 
v_x = t693;
out.push({ let t694 = v_x.clone(); Py::Bool({ let t695 = Py::Int(0i64); if py::eq(&t694, &t695) { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy()))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the denominator must not be zero"))));
    }
    if (py::not(&{ let t698 = { let t696 = v_w.clone(); Py::Bool({ let t697 = Py::Int(0i64); if py::gt(&t696, &t697)? { true } else { false } }) }; if !t698.truthy() { t698 } else { py::math1("isfinite", &v_w.clone())? } })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the frequency must be positive"))));
    }
    v_g = f__G(v_num.clone(), v_den.clone(), v_w.clone())?;
    if ({ let t702 = v_g.clone(); Py::Bool({ let t703 = Py::Int(0i64); if py::eq(&t702, &t703) { true } else { false } }) }).truthy() {
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
    let t735 = (Py::tuple(vec![f__bool_parse(v_expression1.clone())?, f__bool_parse(v_expression2.clone())?])).items()?;
    if t735.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_t1 = t735[0].clone();
    v_t2 = t735[1].clone();
    v_names = py::sorted(&py::bitor(&f__bool_vars(v_t1.clone(), Py::None)?, &f__bool_vars(v_t2.clone(), Py::None)?)?, (Py::Bool(false)).truthy())?;
    if ({ let t745 = py::len(&v_names.clone())?; Py::Bool({ let t746 = Py::Int(12i64); if py::gt(&t745, &t746)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("at most 12 variables"))));
    }
    v_n = py::len(&v_names.clone())?;
    for t747 in py::range_iter(&[(py::pow(&Py::Int(2i64), &v_n.clone())?).i()?]) {
        v_m = t747;
        v_env = { let mut out = vec![]; for t748 in py::iter(&(py::enumerate(&v_names.clone(), 0)?))? { let mut v_i: Py; let mut v_nm: Py; 
let t749 = (t748).items()?;
if t749.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_i = t749[0].clone();
v_nm = t749[1].clone();
out.push((v_nm.clone(), py::bitand(&py::rshift(&v_m.clone(), &py::sub(&py::sub(&v_n.clone(), &Py::Int(1i64))?, &v_i.clone())?)?, &Py::Int(1i64))?)); } Py::dict(out) };
        if ({ let t750 = f__bool_eval(v_t1.clone(), v_env.clone())?; Py::Bool({ let t751 = f__bool_eval(v_t2.clone(), v_env.clone())?; if py::ne(&t750, &t751) { true } else { false } }) }).truthy() {
            return Ok(Py::dict(vec![(Py::str("equivalent"), Py::Bool(false)), (Py::str("counterexample"), py::method(&Py::str(", "), "join", &[{ let mut out = vec![]; for t763 in py::iter(&(py::method(&v_env.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t764 = (t763).items()?;
if t764.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t764[0].clone();
v_v = t764[1].clone();
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
fn f_cell_battery(mut v_E: Py, mut v_r: Py, mut v_n: Py, mut v_connection: Py, mut v_RL: Py) -> R {
    let mut v_cur: Py = Py::None;
    let mut v_emf: Py = Py::None;
    let mut v_rint: Py = Py::None;
    let mut v_vt: Py = Py::None;
    let _ = f__check_positive(Py::dict(vec![(Py::str("E"), v_E.clone()), (Py::str("r"), v_r.clone()), (Py::str("RL"), v_RL.clone())]))?;
    if (py::not(&{ let t839 = py::math1("isfinite", &v_n.clone())?; if !t839.truthy() { t839 } else { { let t838 = { let t833 = v_n.clone(); Py::Bool({ let t834 = py::int(&v_n.clone())?; if py::eq(&t833, &t834) { true } else { false } }) }; if !t838.truthy() { t838 } else { { let t835 = Py::Int(1i64); Py::Bool({ let t836 = v_n.clone(); if py::le(&t835, &t836)? { { let t837 = Py::Int(1000i64); if py::le(&t836, &t837)? { true } else { false } } } else { false } }) } } } } })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the number of cells must be a whole number from 1 to 1000"))));
    }
    if ({ let t840 = v_connection.clone(); Py::Bool({ let t841 = Py::str("series"); if py::eq(&t840, &t841) { true } else { false } }) }).truthy() {
        let t842 = (Py::tuple(vec![py::mul(&v_n.clone(), &v_E.clone())?, py::mul(&v_n.clone(), &v_r.clone())?])).items()?;
        if t842.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_emf = t842[0].clone();
        v_rint = t842[1].clone();
    } else {
        let t843 = (Py::tuple(vec![v_E.clone(), py::truediv(&v_r.clone(), &v_n.clone())?])).items()?;
        if t843.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_emf = t843[0].clone();
        v_rint = t843[1].clone();
    }
    v_cur = py::truediv(&v_emf.clone(), &py::add(&v_rint.clone(), &v_RL.clone())?)?;
    v_vt = py::mul(&v_cur.clone(), &v_RL.clone())?;
    return Ok(Py::dict(vec![(Py::str("emf_total"), v_emf.clone()), (Py::str("r_total"), v_rint.clone()), (Py::str("I"), v_cur.clone()), (Py::str("V_terminal"), v_vt.clone()), (Py::str("lost_volts"), py::mul(&v_cur.clone(), &v_rint.clone())?), (Py::str("P_load"), py::mul(&v_cur.clone(), &v_vt.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_closed_loop_poles(mut v_num: Py, mut v_den: Py, mut v_K: Py) -> R {
    let mut v_char: Py = Py::None;
    let mut v_mr: Py = Py::None;
    let mut v_poles: Py = Py::None;
    let t865 = (f__check_tf(v_num.clone(), v_den.clone())?).items()?;
    if t865.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_num = t865[0].clone();
    v_den = t865[1].clone();
    if (py::not(&py::math1("isfinite", &v_K.clone())?)).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the gain K must be finite"))));
    }
    v_char = f__padd(v_den.clone(), { let mut out = vec![]; for t869 in py::iter(&(v_num.clone()))? { let mut v_x: Py; 
v_x = t869;
out.push(py::mul(&v_K.clone(), &v_x.clone())?); } Py::list(out) })?;
    v_char = f__ptrim(v_char.clone())?;
    if ({ let t870 = py::len(&v_char.clone())?; Py::Bool({ let t871 = Py::Int(2i64); if py::lt(&t870, &t871)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the closed-loop characteristic polynomial has no roots"))));
    }
    v_poles = f__canonical_roots(f__roots(v_char.clone())?, Py::Float(1e-09))?;
    v_mr = py::minmax(&[{ let mut out = vec![]; for t924 in py::iter(&(v_poles.clone()))? { let mut v_p: Py; 
v_p = t924;
out.push(py::real(&py::complex(&[v_p.clone()])?)?); } Py::list(out) }], true)?;
    return Ok(Py::dict(vec![(Py::str("poles"), v_poles.clone()), (Py::str("max_real_part"), v_mr.clone()), (Py::str("stable"), { let t925 = v_mr.clone(); Py::Bool({ let t926 = Py::Int(0i64); if py::lt(&t925, &t926)? { true } else { false } }) }), (Py::str("characteristic"), v_char.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_coil_capacitor_parallel(mut v_V: Py, mut v_f: Py, mut v_R: Py, mut v_L: Py, mut v_C: Py) -> R {
    let mut v_b: Py = Py::None;
    let mut v_bc: Py = Py::None;
    let mut v_bl: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_rd: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_wr: Py = Py::None;
    let mut v_xl: Py = Py::None;
    let mut v_y: Py = Py::None;
    let _ = f__check_positive(Py::dict(vec![(Py::str("V"), v_V.clone()), (Py::str("f"), v_f.clone()), (Py::str("R"), v_R.clone()), (Py::str("L"), v_L.clone()), (Py::str("C"), v_C.clone())]))?;
    if ({ let t946 = py::mul(&v_R.clone(), &v_R.clone())?; Py::Bool({ let t947 = py::truediv(&v_L.clone(), &v_C.clone())?; if py::ge(&t946, &t947)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the coil's resistance must be below sqrt(L/C), or the circuit has no resonance"))));
    }
    v_w = py::mul(&c__TWO_PI()?, &v_f.clone())?;
    v_xl = py::mul(&v_w.clone(), &v_L.clone())?;
    v_d = py::add(&py::mul(&v_R.clone(), &v_R.clone())?, &py::mul(&v_xl.clone(), &v_xl.clone())?)?;
    v_g = py::truediv(&v_R.clone(), &v_d.clone())?;
    v_bl = py::truediv(&v_xl.clone(), &v_d.clone())?;
    v_bc = py::mul(&v_w.clone(), &v_C.clone())?;
    v_b = py::sub(&v_bc.clone(), &v_bl.clone())?;
    if ({ let t948 = py::abs(&v_b.clone())?; Py::Bool({ let t949 = py::mul(&Py::Float(1e-12), &py::minmax(&[v_bc.clone(), v_bl.clone()], true)?)?; if py::le(&t948, &t949)? { true } else { false } }) }).truthy() {
        v_b = Py::Float(0.0);
    }
    v_y = py::hypot(&[v_g.clone(), v_b.clone()])?;
    v_wr = py::math1("sqrt", &py::sub(&py::truediv(&Py::Float(1.0), &py::mul(&v_L.clone(), &v_C.clone())?)?, &py::truediv(&py::mul(&v_R.clone(), &v_R.clone())?, &py::mul(&v_L.clone(), &v_L.clone())?)?)?)?;
    v_rd = py::truediv(&v_L.clone(), &py::mul(&v_C.clone(), &v_R.clone())?)?;
    return Ok(Py::dict(vec![(Py::str("I_coil"), py::truediv(&v_V.clone(), &py::math1("sqrt", &v_d.clone())?)?), (Py::str("I_C"), py::mul(&v_V.clone(), &v_bc.clone())?), (Py::str("I"), py::mul(&v_V.clone(), &v_y.clone())?), (Py::str("Z"), py::truediv(&Py::Float(1.0), &v_y.clone())?), (Py::str("phase_deg"), py::add(&py::math1("degrees", &py::atan2(&py::neg(&v_b.clone())?, &v_g.clone())?)?, &Py::Float(0.0))?), (Py::str("P"), py::mul(&py::mul(&v_V.clone(), &v_V.clone())?, &v_g.clone())?), (Py::str("f_r"), py::truediv(&v_wr.clone(), &c__TWO_PI()?)?), (Py::str("R_dynamic"), v_rd.clone()), (Py::str("I_resonance"), py::truediv(&v_V.clone(), &v_rd.clone())?), (Py::str("Q"), py::truediv(&py::mul(&v_wr.clone(), &v_L.clone())?, &v_R.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_coil_inductance(mut v_N: Py, mut v_length: Py, mut v_area: Py, mut v_mu_r: Py) -> R {
    let mut v_rel: Py = Py::None;
    let _ = f__check_positive(Py::dict(vec![(Py::str("N"), v_N.clone()), (Py::str("length"), v_length.clone()), (Py::str("area"), v_area.clone())]))?;
    if (py::not(&{ let t959 = py::math1("isfinite", &v_mu_r.clone())?; if !t959.truthy() { t959 } else { { let t957 = v_mu_r.clone(); Py::Bool({ let t958 = Py::Int(1i64); if py::ge(&t957, &t958)? { true } else { false } }) } } })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the relative permeability must be at least 1"))));
    }
    v_rel = py::truediv(&v_length.clone(), &py::mul(&py::mul(&c__MU0()?, &v_mu_r.clone())?, &v_area.clone())?)?;
    return Ok(Py::dict(vec![(Py::str("L"), py::truediv(&py::mul(&v_N.clone(), &v_N.clone())?, &v_rel.clone())?), (Py::str("reluctance"), v_rel.clone()), (Py::str("flux_per_ampere"), py::truediv(&v_N.clone(), &v_rel.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_complex_power(mut v_V: Py, mut v_V_phase: Py, mut v_I: Py, mut v_I_phase: Py) -> R {
    let mut v_S: Py = Py::None;
    let mut v_nature: Py = Py::None;
    let mut v_pf: Py = Py::None;
    if (py::not(&{ let t964 = { let t960 = v_V.clone(); Py::Bool({ let t961 = Py::Int(0i64); if py::gt(&t960, &t961)? { true } else { false } }) }; if !t964.truthy() { t964 } else { { let t962 = v_I.clone(); Py::Bool({ let t963 = Py::Int(0i64); if py::gt(&t962, &t963)? { true } else { false } }) } } })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the rms voltage and current must be positive"))));
    }
    v_S = py::mul(&py::cmath("rect", &[v_V.clone(), py::math1("radians", &v_V_phase.clone())?])?, &py::method(&py::cmath("rect", &[v_I.clone(), py::math1("radians", &v_I_phase.clone())?])?, "conjugate", &[])?)?;
    v_pf = py::truediv(&py::real(&v_S.clone())?, &py::abs(&v_S.clone())?)?;
    v_nature = (if ({ let t965 = py::abs(&py::imag(&v_S.clone())?)?; Py::Bool({ let t966 = py::mul(&Py::Float(1e-12), &py::abs(&v_S.clone())?)?; if py::le(&t965, &t966)? { true } else { false } }) }).truthy() { Py::str("unity") } else { (if ({ let t967 = py::imag(&v_S.clone())?; Py::Bool({ let t968 = Py::Int(0i64); if py::gt(&t967, &t968)? { true } else { false } }) }).truthy() { Py::str("lagging") } else { Py::str("leading") }) });
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
    if ({ let t978 = Py::Bool(({ let mut out = vec![]; for t970 in py::iter(&(v_A.clone()))? { let mut v_r: Py; 
v_r = t970;
out.push({ let t971 = py::len(&v_r.clone())?; Py::Bool({ let t972 = v_n.clone(); if py::ne(&t971, &t972) { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())); if t978.truthy() { t978 } else { { let t977 = { let t973 = py::len(&v_B.clone())?; Py::Bool({ let t974 = v_n.clone(); if py::ne(&t973, &t974) { true } else { false } }) }; if t977.truthy() { t977 } else { { let t975 = py::len(&v_C.clone())?; Py::Bool({ let t976 = v_n.clone(); if py::ne(&t975, &t976) { true } else { false } }) } } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("A must be n x n and B, C must have n entries"))));
    }
    let t988 = (f__ctrb_obsv(v_A.clone(), v_B.clone(), v_C.clone())?).items()?;
    if t988.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_Wc = t988[0].clone();
    v_Wo = t988[1].clone();
    let t1007 = (Py::tuple(vec![f__rank(v_Wc.clone(), Py::Float(1e-09))?, f__rank(v_Wo.clone(), Py::Float(1e-09))?])).items()?;
    if t1007.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_rc = t1007[0].clone();
    v_ro = t1007[1].clone();
    return Ok(Py::dict(vec![(Py::str("controllability_rank"), py::float(&v_rc.clone())?), (Py::str("observability_rank"), py::float(&v_ro.clone())?), (Py::str("controllable"), { let t1008 = v_rc.clone(); Py::Bool({ let t1009 = v_n.clone(); if py::eq(&t1008, &t1009) { true } else { false } }) }), (Py::str("observable"), { let t1010 = v_ro.clone(); Py::Bool({ let t1011 = v_n.clone(); if py::eq(&t1010, &t1011) { true } else { false } }) })]));
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
fn f_dc_machine_emf(mut v_poles: Py, mut v_flux: Py, mut v_Z: Py, mut v_n: Py, mut v_winding: Py) -> R {
    let mut v_paths: Py = Py::None;
    let mut v_per: Py = Py::None;
    let _ = f__check_positive(Py::dict(vec![(Py::str("flux"), v_flux.clone()), (Py::str("Z"), v_Z.clone()), (Py::str("n"), v_n.clone())]))?;
    if (py::not(&{ let t1043 = py::math1("isfinite", &v_poles.clone())?; if !t1043.truthy() { t1043 } else { { let t1042 = { let t1035 = v_poles.clone(); Py::Bool({ let t1036 = py::int(&v_poles.clone())?; if py::eq(&t1035, &t1036) { true } else { false } }) }; if !t1042.truthy() { t1042 } else { { let t1041 = { let t1037 = v_poles.clone(); Py::Bool({ let t1038 = Py::Int(2i64); if py::ge(&t1037, &t1038)? { true } else { false } }) }; if !t1041.truthy() { t1041 } else { { let t1039 = py::modulo(&py::int(&v_poles.clone())?, &Py::Int(2i64))?; Py::Bool({ let t1040 = Py::Int(0i64); if py::eq(&t1039, &t1040) { true } else { false } }) } } } } } } })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the number of poles must be an even whole number of at least 2"))));
    }
    if ({ let t1044 = v_Z.clone(); Py::Bool({ let t1045 = py::int(&v_Z.clone())?; if py::ne(&t1044, &t1045) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the number of armature conductors must be a whole number"))));
    }
    v_paths = (if ({ let t1046 = v_winding.clone(); Py::Bool({ let t1047 = Py::str("lap"); if py::eq(&t1046, &t1047) { true } else { false } }) }).truthy() { v_poles.clone() } else { Py::Float(2.0) });
    v_per = py::mul(&py::mul(&v_poles.clone(), &v_flux.clone())?, &v_n.clone())?;
    return Ok(Py::dict(vec![(Py::str("E"), py::truediv(&py::mul(&v_per.clone(), &v_Z.clone())?, &v_paths.clone())?), (Py::str("paths"), py::float(&v_paths.clone())?), (Py::str("conductor_emf"), v_per.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_dc_motor(mut v_V: Py, mut v_R: Py, mut v_Kt: Py, mut v_Ke: Py, mut v_T: Py, mut v_b: Py) -> R {
    let mut v_i: Py = Py::None;
    let mut v_w: Py = Py::None;
    let _ = f__check_positive(Py::dict(vec![(Py::str("R"), v_R.clone()), (Py::str("Kt"), v_Kt.clone()), (Py::str("Ke"), v_Ke.clone())]))?;
    if ({ let t1055 = v_b.clone(); Py::Bool({ let t1056 = Py::Int(0i64); if py::lt(&t1055, &t1056)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the friction coefficient must not be negative"))));
    }
    v_w = py::truediv(&py::sub(&py::mul(&v_Kt.clone(), &v_V.clone())?, &py::mul(&v_R.clone(), &v_T.clone())?)?, &py::add(&py::mul(&v_Kt.clone(), &v_Ke.clone())?, &py::mul(&v_R.clone(), &v_b.clone())?)?)?;
    v_i = py::truediv(&py::add(&v_T.clone(), &py::mul(&v_b.clone(), &v_w.clone())?)?, &v_Kt.clone())?;
    return Ok(Py::dict(vec![(Py::str("speed"), v_w.clone()), (Py::str("rpm"), py::truediv(&py::mul(&v_w.clone(), &Py::Int(60i64))?, &py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?)?), (Py::str("current"), v_i.clone()), (Py::str("input_power"), py::mul(&v_V.clone(), &v_i.clone())?), (Py::str("output_power"), py::mul(&v_T.clone(), &v_w.clone())?), (Py::str("efficiency"), (if ({ let t1057 = py::mul(&v_V.clone(), &v_i.clone())?; Py::Bool({ let t1058 = Py::Int(0i64); if py::gt(&t1057, &t1058)? { true } else { false } }) }).truthy() { py::truediv(&py::mul(&v_T.clone(), &v_w.clone())?, &py::mul(&v_V.clone(), &v_i.clone())?)? } else { Py::Float(0.0) }))]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_dc_motor_speed_change(mut v_V: Py, mut v_Ra: Py, mut v_Ia1: Py, mut v_n1: Py, mut v_Ia2: Py, mut v_flux_ratio: Py) -> R {
    let mut v_e1: Py = Py::None;
    let mut v_e2: Py = Py::None;
    let mut v_n2: Py = Py::None;
    let _ = f__check_positive(Py::dict(vec![(Py::str("V"), v_V.clone()), (Py::str("Ra"), v_Ra.clone()), (Py::str("n1"), v_n1.clone()), (Py::str("flux_ratio"), v_flux_ratio.clone())]))?;
    if (py::not(&{ let t1071 = py::math1("isfinite", &v_Ia1.clone())?; if !t1071.truthy() { t1071 } else { { let t1070 = { let t1065 = v_Ia1.clone(); Py::Bool({ let t1066 = Py::Int(0i64); if py::ge(&t1065, &t1066)? { true } else { false } }) }; if !t1070.truthy() { t1070 } else { { let t1069 = py::math1("isfinite", &v_Ia2.clone())?; if !t1069.truthy() { t1069 } else { { let t1067 = v_Ia2.clone(); Py::Bool({ let t1068 = Py::Int(0i64); if py::ge(&t1067, &t1068)? { true } else { false } }) } } } } } } })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the armature currents must be zero or positive"))));
    }
    v_e1 = py::sub(&v_V.clone(), &py::mul(&v_Ia1.clone(), &v_Ra.clone())?)?;
    v_e2 = py::sub(&v_V.clone(), &py::mul(&v_Ia2.clone(), &v_Ra.clone())?)?;
    if ({ let t1076 = { let t1072 = v_e1.clone(); Py::Bool({ let t1073 = Py::Int(0i64); if py::le(&t1072, &t1073)? { true } else { false } }) }; if t1076.truthy() { t1076 } else { { let t1074 = v_e2.clone(); Py::Bool({ let t1075 = Py::Int(0i64); if py::le(&t1074, &t1075)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the armature drop Ia Ra reaches the supply voltage: there is no positive back emf"))));
    }
    v_n2 = py::truediv(&py::mul(&v_n1.clone(), &py::truediv(&v_e2.clone(), &v_e1.clone())?)?, &v_flux_ratio.clone())?;
    return Ok(Py::dict(vec![(Py::str("E1"), v_e1.clone()), (Py::str("E2"), v_e2.clone()), (Py::str("n2"), v_n2.clone()), (Py::str("n2_rpm"), py::mul(&Py::Float(60.0), &v_n2.clone())?), (Py::str("speed_change"), py::sub(&py::truediv(&v_n2.clone(), &v_n1.clone())?, &Py::Float(1.0))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_dc_node_voltages(mut v_netlist: Py) -> R {
    let mut v_cur: Py = Py::None;
    let mut v_els: Py = Py::None;
    let mut v_volts: Py = Py::None;
    v_els = f__netlist(v_netlist.clone())?;
    let t1186 = (f__mna(v_els.clone())?).items()?;
    if t1186.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_volts = t1186[0].clone();
    v_cur = t1186[1].clone();
    return Ok(Py::dict(vec![(Py::str("node_voltages"), { let mut out = vec![]; for t1187 in py::iter(&(py::method(&v_volts.clone(), "items", &[])?))? { let mut v_n: Py; let mut v_v: Py; 
let t1188 = (t1187).items()?;
if t1188.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_n = t1188[0].clone();
v_v = t1188[1].clone();
if ({ let t1189 = v_n.clone(); Py::Bool({ let t1190 = Py::str("0"); if py::ne(&t1189, &t1190) { true } else { false } }) }).truthy() { out.push((v_n.clone(), v_v.clone())); }} Py::dict(out) }), (Py::str("source_currents"), v_cur.clone()), (Py::str("power_delivered"), f__power_delivered(v_els.clone(), v_volts.clone(), v_cur.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_dc_shunt_generator(mut v_V: Py, mut v_IL: Py, mut v_Ra: Py, mut v_Rf: Py) -> R {
    let mut v_e: Py = Py::None;
    let mut v_i_f: Py = Py::None;
    let mut v_ia: Py = Py::None;
    let _ = f__check_positive(Py::dict(vec![(Py::str("V"), v_V.clone()), (Py::str("Ra"), v_Ra.clone()), (Py::str("Rf"), v_Rf.clone())]))?;
    if (py::not(&{ let t1232 = py::math1("isfinite", &v_IL.clone())?; if !t1232.truthy() { t1232 } else { { let t1230 = v_IL.clone(); Py::Bool({ let t1231 = Py::Int(0i64); if py::ge(&t1230, &t1231)? { true } else { false } }) } } })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the load current must be zero or positive"))));
    }
    v_i_f = py::truediv(&v_V.clone(), &v_Rf.clone())?;
    v_ia = py::add(&v_IL.clone(), &v_i_f.clone())?;
    v_e = py::add(&v_V.clone(), &py::mul(&v_ia.clone(), &v_Ra.clone())?)?;
    return Ok(Py::dict(vec![(Py::str("If"), v_i_f.clone()), (Py::str("Ia"), v_ia.clone()), (Py::str("E"), v_e.clone()), (Py::str("P_out"), py::mul(&v_V.clone(), &v_IL.clone())?), (Py::str("P_generated"), py::mul(&v_e.clone(), &v_ia.clone())?), (Py::str("copper_loss"), py::add(&py::mul(&py::mul(&v_ia.clone(), &v_ia.clone())?, &v_Ra.clone())?, &py::mul(&v_V.clone(), &v_i_f.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_dc_shunt_motor(mut v_V: Py, mut v_I: Py, mut v_Ra: Py, mut v_Rf: Py, mut v_n: Py, mut v_losses: Py) -> R {
    let mut v_e: Py = Py::None;
    let mut v_i_f: Py = Py::None;
    let mut v_ia: Py = Py::None;
    let mut v_pc: Py = Py::None;
    let mut v_pout: Py = Py::None;
    let _ = f__check_positive(Py::dict(vec![(Py::str("V"), v_V.clone()), (Py::str("I"), v_I.clone()), (Py::str("Ra"), v_Ra.clone()), (Py::str("Rf"), v_Rf.clone()), (Py::str("n"), v_n.clone())]))?;
    if (py::not(&{ let t1241 = py::math1("isfinite", &v_losses.clone())?; if !t1241.truthy() { t1241 } else { { let t1239 = v_losses.clone(); Py::Bool({ let t1240 = Py::Int(0i64); if py::ge(&t1239, &t1240)? { true } else { false } }) } } })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the losses must be zero or positive"))));
    }
    v_i_f = py::truediv(&v_V.clone(), &v_Rf.clone())?;
    v_ia = py::sub(&v_I.clone(), &v_i_f.clone())?;
    if ({ let t1242 = v_ia.clone(); Py::Bool({ let t1243 = Py::Int(0i64); if py::le(&t1242, &t1243)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the supply current must exceed the field current V/Rf"))));
    }
    v_e = py::sub(&v_V.clone(), &py::mul(&v_ia.clone(), &v_Ra.clone())?)?;
    if ({ let t1244 = v_e.clone(); Py::Bool({ let t1245 = Py::Int(0i64); if py::le(&t1244, &t1245)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the armature drop Ia Ra reaches the supply voltage: there is no positive back emf"))));
    }
    v_pc = py::mul(&v_e.clone(), &v_ia.clone())?;
    v_pout = py::sub(&v_pc.clone(), &v_losses.clone())?;
    if ({ let t1246 = v_pout.clone(); Py::Bool({ let t1247 = Py::Int(0i64); if py::lt(&t1246, &t1247)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the stated losses exceed the power converted"))));
    }
    return Ok(Py::dict(vec![(Py::str("If"), v_i_f.clone()), (Py::str("Ia"), v_ia.clone()), (Py::str("E"), v_e.clone()), (Py::str("P_converted"), v_pc.clone()), (Py::str("torque"), py::truediv(&v_pc.clone(), &py::mul(&c__TWO_PI()?, &v_n.clone())?)?), (Py::str("P_out"), v_pout.clone()), (Py::str("efficiency"), py::truediv(&v_pout.clone(), &py::mul(&v_V.clone(), &v_I.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_delta_to_wye(mut v_Rab: Py, mut v_Rbc: Py, mut v_Rca: Py) -> R {
    let mut v_n: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_v: Py = Py::None;
    for t1249 in py::iter(&(Py::tuple(vec![Py::tuple(vec![Py::str("Rab"), v_Rab.clone()]), Py::tuple(vec![Py::str("Rbc"), v_Rbc.clone()]), Py::tuple(vec![Py::str("Rca"), v_Rca.clone()])])))? {
        let t1253 = (t1249).items()?;
        if t1253.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_n = t1253[0].clone();
        v_v = t1253[1].clone();
        if (py::not(&{ let t1252 = { let t1250 = v_v.clone(); Py::Bool({ let t1251 = Py::Int(0i64); if py::gt(&t1250, &t1251)? { true } else { false } }) }; if !t1252.truthy() { t1252 } else { py::math1("isfinite", &v_v.clone())? } })).truthy() {
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
    let t1272 = (Py::tuple(vec![Py::Float(0.0), py::truediv(&v_Vs.clone(), &v_R.clone())?])).items()?;
    if t1272.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lo = t1272[0].clone();
    v_hi = t1272[1].clone();
    for t1273 in py::range_iter(&[(Py::Int(400i64)).i()?]) {
        v__ = t1273;
        v_mid = py::mul(&Py::Float(0.5), &py::add(&v_lo.clone(), &v_hi.clone())?)?;
        if ({ let t1274 = v_mid.clone(); Py::Bool({ let t1275 = Py::tuple(vec![v_lo.clone(), v_hi.clone()]); if py::contains(&t1275, &t1274)? { true } else { false } }) }).truthy() {
            break;
        }
        let t1278 = ((if ({ let t1276 = py::call(&v_f.clone(), &[v_mid.clone()])?; Py::Bool({ let t1277 = Py::Int(0i64); if py::gt(&t1276, &t1277)? { true } else { false } }) }).truthy() { Py::tuple(vec![v_mid.clone(), v_hi.clone()]) } else { Py::tuple(vec![v_lo.clone(), v_mid.clone()]) })).items()?;
        if t1278.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_lo = t1278[0].clone();
        v_hi = t1278[1].clone();
    }
    v_i = py::mul(&Py::Float(0.5), &py::add(&v_lo.clone(), &v_hi.clone())?)?;
    for t1279 in py::range_iter(&[(Py::Int(3i64)).i()?]) {
        v__ = t1279;
        v_d = py::sub(&py::neg(&v_R.clone())?, &py::truediv(&py::mul(&v_n.clone(), &v_VT.clone())?, &py::add(&v_Is.clone(), &v_i.clone())?)?)?;
        v_i = py::sub(&v_i, &py::truediv(&py::call(&v_f.clone(), &[v_i.clone()])?, &v_d.clone())?)?;
    }
    return Ok(Py::dict(vec![(Py::str("I"), v_i.clone()), (Py::str("I_mA"), py::mul(&v_i.clone(), &Py::Float(1000.0))?), (Py::str("VD"), py::mul(&py::mul(&v_n.clone(), &v_VT.clone())?, &py::math1("log1p", &py::truediv(&v_i.clone(), &v_Is.clone())?)?)?), (Py::str("VR"), py::mul(&v_i.clone(), &v_R.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_electrical_energy_cost(mut v_P: Py, mut v_t: Py, mut v_tariff: Py) -> R {
    let mut v_energy: Py = Py::None;
    let mut v_kwh: Py = Py::None;
    let _ = f__check_positive(Py::dict(vec![(Py::str("P"), v_P.clone()), (Py::str("t"), v_t.clone())]))?;
    if (py::not(&{ let t1288 = py::math1("isfinite", &v_tariff.clone())?; if !t1288.truthy() { t1288 } else { { let t1286 = v_tariff.clone(); Py::Bool({ let t1287 = Py::Int(0i64); if py::ge(&t1286, &t1287)? { true } else { false } }) } } })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the tariff must be zero or positive"))));
    }
    v_energy = py::mul(&v_P.clone(), &v_t.clone())?;
    v_kwh = py::truediv(&v_energy.clone(), &Py::Float(3600000.0))?;
    return Ok(Py::dict(vec![(Py::str("energy"), v_energy.clone()), (Py::str("energy_kWh"), v_kwh.clone()), (Py::str("cost"), py::mul(&v_kwh.clone(), &v_tariff.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_faraday_emf(mut v_N: Py, mut v_dPhi: Py, mut v_dt: Py) -> R {
    let mut v_rate: Py = Py::None;
    let _ = f__check_positive(Py::dict(vec![(Py::str("N"), v_N.clone()), (Py::str("dt"), v_dt.clone())]))?;
    if (py::not(&py::math1("isfinite", &v_dPhi.clone())?)).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the change of flux must be a finite number"))));
    }
    v_rate = py::truediv(&v_dPhi.clone(), &v_dt.clone())?;
    return Ok(Py::dict(vec![(Py::str("emf"), py::mul(&v_N.clone(), &py::abs(&v_rate.clone())?)?), (Py::str("emf_signed"), py::add(&py::mul(&py::neg(&v_N.clone())?, &v_rate.clone())?, &Py::Float(0.0))?), (Py::str("flux_rate"), v_rate.clone()), (Py::str("linkage_change"), py::mul(&v_N.clone(), &v_dPhi.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_first_order_time_to_value(mut v_V0: Py, mut v_Vf: Py, mut v_tau: Py, mut v_v: Py) -> R {
    let mut v_frac: Py = Py::None;
    let _ = f__check_positive(Py::dict(vec![(Py::str("tau"), v_tau.clone())]))?;
    if ({ let t1300 = v_V0.clone(); Py::Bool({ let t1301 = v_Vf.clone(); if py::eq(&t1300, &t1301) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the initial and final values are equal: nothing changes"))));
    }
    v_frac = py::truediv(&py::sub(&v_v.clone(), &v_V0.clone())?, &py::sub(&v_Vf.clone(), &v_V0.clone())?)?;
    if (py::not(&{ let t1302 = Py::Int(0i64); Py::Bool({ let t1303 = v_frac.clone(); if py::le(&t1302, &t1303)? { { let t1304 = Py::Int(1i64); if py::lt(&t1303, &t1304)? { true } else { false } } } else { false } }) })).truthy() {
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
    if ({ let t1311 = v_t.clone(); Py::Bool({ let t1312 = Py::Int(0i64); if py::lt(&t1311, &t1312)? { true } else { false } }) }).truthy() {
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
    let t1330 = (f__check_tf(v_num.clone(), v_den.clone())?).items()?;
    if t1330.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_num = t1330[0].clone();
    v_den = t1330[1].clone();
    v_wp = f__phase_crossover(v_num.clone(), v_den.clone())?;
    if ({ let t1403 = py::len(&v_wp.clone())?; Py::Bool({ let t1404 = Py::Int(1i64); if py::ne(&t1403, &t1404) { true } else { false } }) }).truthy() {
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
    if ({ let t1411 = py::not(&v_v.clone()); if t1411.truthy() { t1411 } else { { let t1410 = py::sub(&py::set(&v_v.clone())?, &py::set(&Py::list(vec![Py::str("0"), Py::str("1")]))?)?; if t1410.truthy() { t1410 } else { { let t1408 = py::len(&v_v.clone())?; Py::Bool({ let t1409 = Py::Int(32i64); if py::gt(&t1408, &t1409)? { true } else { false } }) } } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the value must be 1 to 32 binary digits"))));
    }
    v_n = py::len(&v_v.clone())?;
    v_x = py::int_base(&v_v.clone(), &Py::Int(2i64))?;
    if ({ let t1412 = v_direction.clone(); Py::Bool({ let t1413 = Py::str("to_gray"); if py::eq(&t1412, &t1413) { true } else { false } }) }).truthy() {
        v_r = py::bitxor(&v_x.clone(), &py::rshift(&v_x.clone(), &Py::Int(1i64))?)?;
    } else {
        let t1414 = (Py::tuple(vec![v_x.clone(), py::rshift(&v_x.clone(), &Py::Int(1i64))?])).items()?;
        if t1414.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_r = t1414[0].clone();
        v_s = t1414[1].clone();
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
fn f_magnetic_circuit(mut v_N: Py, mut v_lengths: Py, mut v_areas: Py, mut v_mu_r: Py, mut v_flux: Py, mut v_current: Py) -> R {
    let mut v_B: Py = Py::None;
    let mut v_H: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_cur: Py = Py::None;
    let mut v_given: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_mmf: Py = Py::None;
    let mut v_phi: Py = Py::None;
    let mut v_rel: Py = Py::None;
    let _ = f__check_positive(Py::dict(vec![(Py::str("N"), v_N.clone())]))?;
    v_k = py::len(&v_lengths.clone())?;
    if ({ let t1457 = { let t1450 = v_k.clone(); Py::Bool({ let t1451 = Py::Int(0i64); if py::eq(&t1450, &t1451) { true } else { false } }) }; if t1457.truthy() { t1457 } else { { let t1456 = { let t1452 = py::len(&v_areas.clone())?; Py::Bool({ let t1453 = v_k.clone(); if py::ne(&t1452, &t1453) { true } else { false } }) }; if t1456.truthy() { t1456 } else { { let t1454 = py::len(&v_mu_r.clone())?; Py::Bool({ let t1455 = v_k.clone(); if py::ne(&t1454, &t1455) { true } else { false } }) } } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("give one length, one area and one relative permeability for every section"))));
    }
    for t1458 in py::range_iter(&[(v_k.clone()).i()?]) {
        v_j = t1458;
        if (py::not(&{ let t1465 = py::math1("isfinite", &py::getitem(&v_lengths.clone(), &v_j.clone())?)?; if !t1465.truthy() { t1465 } else { { let t1464 = { let t1459 = py::getitem(&v_lengths.clone(), &v_j.clone())?; Py::Bool({ let t1460 = Py::Int(0i64); if py::gt(&t1459, &t1460)? { true } else { false } }) }; if !t1464.truthy() { t1464 } else { { let t1463 = py::math1("isfinite", &py::getitem(&v_areas.clone(), &v_j.clone())?)?; if !t1463.truthy() { t1463 } else { { let t1461 = py::getitem(&v_areas.clone(), &v_j.clone())?; Py::Bool({ let t1462 = Py::Int(0i64); if py::gt(&t1461, &t1462)? { true } else { false } }) } } } } } } })).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("every length and every area must be positive"))));
        }
        if (py::not(&{ let t1468 = py::math1("isfinite", &py::getitem(&v_mu_r.clone(), &v_j.clone())?)?; if !t1468.truthy() { t1468 } else { { let t1466 = py::getitem(&v_mu_r.clone(), &v_j.clone())?; Py::Bool({ let t1467 = Py::Int(1i64); if py::ge(&t1466, &t1467)? { true } else { false } }) } } })).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("every relative permeability must be at least 1"))));
        }
    }
    if ({ let t1475 = py::not(&{ let t1469 = py::math1("isfinite", &v_flux.clone())?; if !t1469.truthy() { t1469 } else { py::math1("isfinite", &v_current.clone())? } }); if t1475.truthy() { t1475 } else { { let t1474 = { let t1470 = v_flux.clone(); Py::Bool({ let t1471 = Py::Int(0i64); if py::lt(&t1470, &t1471)? { true } else { false } }) }; if t1474.truthy() { t1474 } else { { let t1472 = v_current.clone(); Py::Bool({ let t1473 = Py::Int(0i64); if py::lt(&t1472, &t1473)? { true } else { false } }) } } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the flux and the current must not be negative"))));
    }
    v_given = Py::Int(0i64);
    if ({ let t1476 = v_flux.clone(); Py::Bool({ let t1477 = Py::Int(0i64); if py::gt(&t1476, &t1477)? { true } else { false } }) }).truthy() {
        v_given = py::iadd(&v_given, &Py::Int(1i64))?;
    }
    if ({ let t1478 = v_current.clone(); Py::Bool({ let t1479 = Py::Int(0i64); if py::gt(&t1478, &t1479)? { true } else { false } }) }).truthy() {
        v_given = py::iadd(&v_given, &Py::Int(1i64))?;
    }
    if ({ let t1480 = v_given.clone(); Py::Bool({ let t1481 = Py::Int(1i64); if py::ne(&t1480, &t1481) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("give exactly one of the flux and the current"))));
    }
    v_rel = Py::Float(0.0);
    for t1482 in py::range_iter(&[(v_k.clone()).i()?]) {
        v_j = t1482;
        v_rel = py::iadd(&v_rel, &py::truediv(&py::getitem(&v_lengths.clone(), &v_j.clone())?, &py::mul(&py::mul(&c__MU0()?, &py::getitem(&v_mu_r.clone(), &v_j.clone())?)?, &py::getitem(&v_areas.clone(), &v_j.clone())?)?)?)?;
    }
    if ({ let t1483 = v_flux.clone(); Py::Bool({ let t1484 = Py::Int(0i64); if py::gt(&t1483, &t1484)? { true } else { false } }) }).truthy() {
        v_phi = v_flux.clone();
        v_mmf = py::mul(&v_phi.clone(), &v_rel.clone())?;
        v_cur = py::truediv(&v_mmf.clone(), &v_N.clone())?;
    } else {
        v_cur = v_current.clone();
        v_mmf = py::mul(&v_N.clone(), &v_cur.clone())?;
        v_phi = py::truediv(&v_mmf.clone(), &v_rel.clone())?;
    }
    v_B = Py::list(vec![]);
    v_H = Py::list(vec![]);
    for t1485 in py::range_iter(&[(v_k.clone()).i()?]) {
        v_j = t1485;
        v_b = py::truediv(&v_phi.clone(), &py::getitem(&v_areas.clone(), &v_j.clone())?)?;
        let _ = py::method(&v_B.clone(), "append", &[v_b.clone()])?;
        let _ = py::method(&v_H.clone(), "append", &[py::truediv(&v_b.clone(), &py::mul(&c__MU0()?, &py::getitem(&v_mu_r.clone(), &v_j.clone())?)?)?])?;
    }
    return Ok(Py::dict(vec![(Py::str("reluctance"), v_rel.clone()), (Py::str("mmf"), v_mmf.clone()), (Py::str("current"), v_cur.clone()), (Py::str("flux"), v_phi.clone()), (Py::str("B"), v_B.clone()), (Py::str("H"), v_H.clone())]));
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
    if (py::not(&{ let t1545 = Py::Int(1i64); Py::Bool({ let t1546 = v_n.clone(); if py::le(&t1545, &t1546)? { { let t1547 = Py::Int(4i64); if py::le(&t1546, &t1547)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("minimization is offered for 1 to 4 variables"))));
    }
    v_on = py::sorted(&py::set(&{ let mut out = vec![]; for t1548 in py::iter(&(v_minterms.clone()))? { let mut v_m: Py; 
v_m = t1548;
out.push(py::int(&v_m.clone())?); } Py::list(out) })?, (Py::Bool(false)).truthy())?;
    v_dc = py::sorted(&py::set(&{ let mut out = vec![]; for t1550 in py::iter(&({ let t1549 = v_dont_cares.clone(); if t1549.truthy() { t1549 } else { Py::list(vec![]) } }))? { let mut v_m: Py; 
v_m = t1550;
out.push(py::int(&v_m.clone())?); } Py::list(out) })?, (Py::Bool(false)).truthy())?;
    for t1552 in py::iter(&(py::add(&Py::list((v_minterms.clone()).items()?), &Py::list(({ let t1551 = v_dont_cares.clone(); if t1551.truthy() { t1551 } else { Py::list(vec![]) } }).items()?))?))? {
        v_m = t1552;
        if ({ let t1558 = { let t1553 = v_m.clone(); Py::Bool({ let t1554 = py::int(&v_m.clone())?; if py::ne(&t1553, &t1554) { true } else { false } }) }; if t1558.truthy() { t1558 } else { py::not(&{ let t1555 = Py::Int(0i64); Py::Bool({ let t1556 = v_m.clone(); if py::le(&t1555, &t1556)? { { let t1557 = py::pow(&Py::Int(2i64), &v_n.clone())?; if py::lt(&t1556, &t1557)? { true } else { false } } } else { false } }) }) } }).truthy() {
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
    if ({ let t1679 = v_vov.clone(); Py::Bool({ let t1680 = Py::Int(0i64); if py::le(&t1679, &t1680)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("VGS does not exceed the threshold voltage: the transistor is off"))));
    }
    if ({ let t1681 = v_VDS.clone(); Py::Bool({ let t1682 = v_vov.clone(); if py::lt(&t1681, &t1682)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("VDS < VGS - VTH: the transistor is in the triode region, not saturation"))));
    }
    v_idd = py::mul(&py::mul(&py::mul(&py::mul(&Py::Float(0.5), &v_kn.clone())?, &v_WL.clone())?, &v_vov.clone())?, &v_vov.clone())?;
    return Ok(Py::dict(vec![(Py::str("ID"), v_idd.clone()), (Py::str("ID_mA"), py::mul(&v_idd.clone(), &Py::Float(1000.0))?), (Py::str("VOV"), v_vov.clone()), (Py::str("gm"), py::truediv(&py::mul(&Py::Int(2i64), &v_idd.clone())?, &v_vov.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_mutual_inductance(mut v_L1: Py, mut v_L2: Py, mut v_k: Py, mut v_dI1: Py, mut v_dt: Py) -> R {
    let mut v_m: Py = Py::None;
    let mut v_rate: Py = Py::None;
    let _ = f__check_positive(Py::dict(vec![(Py::str("L1"), v_L1.clone()), (Py::str("L2"), v_L2.clone()), (Py::str("dt"), v_dt.clone())]))?;
    if (py::not(&{ let t1692 = py::math1("isfinite", &v_k.clone())?; if !t1692.truthy() { t1692 } else { { let t1689 = Py::Int(0i64); Py::Bool({ let t1690 = v_k.clone(); if py::le(&t1689, &t1690)? { { let t1691 = Py::Int(1i64); if py::le(&t1690, &t1691)? { true } else { false } } } else { false } }) } } })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the coupling coefficient must be between 0 and 1"))));
    }
    if (py::not(&py::math1("isfinite", &v_dI1.clone())?)).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the change of current must be a finite number"))));
    }
    v_m = py::mul(&v_k.clone(), &py::math1("sqrt", &py::mul(&v_L1.clone(), &v_L2.clone())?)?)?;
    v_rate = py::truediv(&py::abs(&v_dI1.clone())?, &v_dt.clone())?;
    return Ok(Py::dict(vec![(Py::str("M"), v_m.clone()), (Py::str("E1"), py::mul(&v_L1.clone(), &v_rate.clone())?), (Py::str("E2"), py::mul(&v_m.clone(), &v_rate.clone())?), (Py::str("L_aiding"), py::add(&py::add(&v_L1.clone(), &v_L2.clone())?, &py::mul(&Py::Float(2.0), &v_m.clone())?)?), (Py::str("L_opposing"), py::sub(&py::add(&v_L1.clone(), &v_L2.clone())?, &py::mul(&Py::Float(2.0), &v_m.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_norton_equivalent(mut v_netlist: Py, mut v_node_a: Py, mut v_node_b: Py) -> R {
    let mut v_rth: Py = Py::None;
    let mut v_vth: Py = Py::None;
    let t1878 = (f__thevenin(v_netlist.clone(), v_node_a.clone(), v_node_b.clone())?).items()?;
    if t1878.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_vth = t1878[0].clone();
    v_rth = t1878[1].clone();
    return Ok(Py::dict(vec![(Py::str("In"), py::truediv(&v_vth.clone(), &v_rth.clone())?), (Py::str("Rn"), v_rth.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_opamp_integrator(mut v_R: Py, mut v_C: Py, mut v_Vin: Py, mut v_t: Py, mut v_V0: Py) -> R {
    let _ = f__check_positive(Py::dict(vec![(Py::str("R"), v_R.clone()), (Py::str("C"), v_C.clone())]))?;
    if ({ let t1889 = v_t.clone(); Py::Bool({ let t1890 = Py::Int(0i64); if py::lt(&t1889, &t1890)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the time must be zero or positive"))));
    }
    return Ok(Py::dict(vec![(Py::str("Vout"), py::sub(&v_V0.clone(), &py::truediv(&py::mul(&v_Vin.clone(), &v_t.clone())?, &py::mul(&v_R.clone(), &v_C.clone())?)?)?), (Py::str("slope"), py::truediv(&py::neg(&v_Vin.clone())?, &py::mul(&v_R.clone(), &v_C.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_parallel_ac_supply(mut v_V: Py, mut v_f: Py, mut v_R: Py, mut v_circuit: Py, mut v_L: Py, mut v_C: Py) -> R {
    let mut v_b: Py = Py::None;
    let mut v_cur: Py = Py::None;
    let mut v_ic: Py = Py::None;
    let mut v_il: Py = Py::None;
    let mut v_ir: Py = Py::None;
    let mut v_nature: Py = Py::None;
    let mut v_w: Py = Py::None;
    let _ = f__ac_check(v_V.clone(), v_f.clone(), v_R.clone(), v_circuit.clone(), v_L.clone(), v_C.clone())?;
    v_w = py::mul(&c__TWO_PI()?, &v_f.clone())?;
    v_ir = py::truediv(&v_V.clone(), &v_R.clone())?;
    v_il = (if ({ let t1928 = v_L.clone(); Py::Bool({ let t1929 = Py::Int(0i64); if py::gt(&t1928, &t1929)? { true } else { false } }) }).truthy() { py::truediv(&v_V.clone(), &py::mul(&v_w.clone(), &v_L.clone())?)? } else { Py::Float(0.0) });
    v_ic = (if ({ let t1930 = v_C.clone(); Py::Bool({ let t1931 = Py::Int(0i64); if py::gt(&t1930, &t1931)? { true } else { false } }) }).truthy() { py::mul(&py::mul(&v_V.clone(), &v_w.clone())?, &v_C.clone())? } else { Py::Float(0.0) });
    v_b = py::sub(&v_il.clone(), &v_ic.clone())?;
    if ({ let t1932 = py::abs(&v_b.clone())?; Py::Bool({ let t1933 = py::mul(&Py::Float(1e-12), &py::minmax(&[v_il.clone(), v_ic.clone()], true)?)?; if py::le(&t1932, &t1933)? { true } else { false } }) }).truthy() {
        v_b = Py::Float(0.0);
    }
    v_cur = py::hypot(&[v_ir.clone(), v_b.clone()])?;
    v_nature = (if ({ let t1934 = v_b.clone(); Py::Bool({ let t1935 = Py::Int(0i64); if py::eq(&t1934, &t1935) { true } else { false } }) }).truthy() { Py::str("unity") } else { (if ({ let t1936 = v_b.clone(); Py::Bool({ let t1937 = Py::Int(0i64); if py::gt(&t1936, &t1937)? { true } else { false } }) }).truthy() { Py::str("lagging") } else { Py::str("leading") }) });
    return Ok(Py::dict(vec![(Py::str("IR"), v_ir.clone()), (Py::str("IL"), v_il.clone()), (Py::str("IC"), v_ic.clone()), (Py::str("I"), v_cur.clone()), (Py::str("Z"), py::truediv(&v_V.clone(), &v_cur.clone())?), (Py::str("phase_deg"), py::add(&py::math1("degrees", &py::atan2(&v_b.clone(), &v_ir.clone())?)?, &Py::Float(0.0))?), (Py::str("power_factor"), py::truediv(&v_ir.clone(), &v_cur.clone())?), (Py::str("P"), py::mul(&v_V.clone(), &v_ir.clone())?), (Py::str("Q"), py::add(&py::mul(&v_V.clone(), &v_b.clone())?, &Py::Float(0.0))?), (Py::str("S"), py::mul(&v_V.clone(), &v_cur.clone())?), (Py::str("nature"), v_nature.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_parallel_plate_capacitor(mut v_V: Py, mut v_d: Py, mut v_A: Py, mut v_eps_r: Py, mut v_n: Py) -> R {
    let mut v_cap: Py = Py::None;
    let mut v_field: Py = Py::None;
    let _ = f__check_positive(Py::dict(vec![(Py::str("V"), v_V.clone()), (Py::str("d"), v_d.clone()), (Py::str("A"), v_A.clone())]))?;
    if (py::not(&{ let t1984 = py::math1("isfinite", &v_eps_r.clone())?; if !t1984.truthy() { t1984 } else { { let t1982 = v_eps_r.clone(); Py::Bool({ let t1983 = Py::Int(1i64); if py::ge(&t1982, &t1983)? { true } else { false } }) } } })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the relative permittivity must be at least 1"))));
    }
    if (py::not(&{ let t1991 = py::math1("isfinite", &v_n.clone())?; if !t1991.truthy() { t1991 } else { { let t1990 = { let t1985 = v_n.clone(); Py::Bool({ let t1986 = py::int(&v_n.clone())?; if py::eq(&t1985, &t1986) { true } else { false } }) }; if !t1990.truthy() { t1990 } else { { let t1987 = Py::Int(2i64); Py::Bool({ let t1988 = v_n.clone(); if py::le(&t1987, &t1988)? { { let t1989 = Py::Int(1000i64); if py::le(&t1988, &t1989)? { true } else { false } } } else { false } }) } } } } })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the number of plates must be a whole number from 2 to 1000"))));
    }
    v_cap = py::truediv(&py::mul(&py::mul(&py::mul(&c__EPS0()?, &v_eps_r.clone())?, &v_A.clone())?, &py::sub(&v_n.clone(), &Py::Int(1i64))?)?, &v_d.clone())?;
    v_field = py::truediv(&v_V.clone(), &v_d.clone())?;
    return Ok(Py::dict(vec![(Py::str("C"), v_cap.clone()), (Py::str("Q"), py::mul(&v_cap.clone(), &v_V.clone())?), (Py::str("E"), v_field.clone()), (Py::str("D"), py::mul(&py::mul(&c__EPS0()?, &v_eps_r.clone())?, &v_field.clone())?), (Py::str("energy"), py::mul(&py::mul(&py::mul(&Py::Float(0.5), &v_cap.clone())?, &v_V.clone())?, &v_V.clone())?)]));
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
    if ({ let t2011 = py::abs(&v_B.clone())?; Py::Bool({ let t2012 = py::mul(&Py::Float(1e-12), &py::minmax(&[py::mul(&v_w.clone(), &v_C.clone())?, py::truediv(&Py::Float(1.0), &py::mul(&v_w.clone(), &v_L.clone())?)?], true)?)?; if py::le(&t2011, &t2012)? { true } else { false } }) }).truthy() {
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
    let t2040 = (f__check_tf(v_num.clone(), v_den.clone())?).items()?;
    if t2040.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_num = t2040[0].clone();
    v_den = t2040[1].clone();
    v_wc = f__gain_crossover(v_num.clone(), v_den.clone())?;
    if ({ let t2114 = py::len(&v_wc.clone())?; Py::Bool({ let t2115 = Py::Int(1i64); if py::ne(&t2114, &t2115) { true } else { false } }) }).truthy() {
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
    if ({ let t2117 = py::len(&v_amplitudes.clone())?; Py::Bool({ let t2118 = py::len(&v_phases_deg.clone())?; if py::ne(&t2117, &t2118) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("give one phase for every amplitude"))));
    }
    if ({ let t2119 = py::len(&v_amplitudes.clone())?; Py::Bool({ let t2120 = Py::Int(2i64); if py::lt(&t2119, &t2120)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("give at least two phasors"))));
    }
    if (Py::Bool(({ let mut out = vec![]; for t2121 in py::iter(&(v_amplitudes.clone()))? { let mut v_a: Py; 
v_a = t2121;
out.push({ let t2122 = v_a.clone(); Py::Bool({ let t2123 = Py::Int(0i64); if py::lt(&t2122, &t2123)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("amplitudes must not be negative"))));
    }
    v_z = py::sum(&{ let mut out = vec![]; for t2124 in py::iter(&(py::zip(&[v_amplitudes.clone(), v_phases_deg.clone()])?))? { let mut v_a: Py; let mut v_p: Py; 
let t2125 = (t2124).items()?;
if t2125.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t2125[0].clone();
v_p = t2125[1].clone();
out.push(py::mul(&v_a.clone(), &py::cmath("exp", &[py::mul(&Py::Complex(0.0, 1.0), &py::math1("radians", &v_p.clone())?)?])?)?); } Py::list(out) }, None)?;
    if ({ let t2126 = py::abs(&v_z.clone())?; Py::Bool({ let t2127 = py::mul(&Py::Float(1e-12), &py::minmax(&[v_amplitudes.clone()], true)?)?; if py::le(&t2126, &t2127)? { true } else { false } }) }).truthy() {
        return Ok(Py::dict(vec![(Py::str("amplitude"), Py::Float(0.0)), (Py::str("phase_deg"), Py::Float(0.0)), (Py::str("real"), Py::Float(0.0)), (Py::str("imag"), Py::Float(0.0))]));
    }
    return Ok(Py::dict(vec![(Py::str("amplitude"), py::abs(&v_z.clone())?), (Py::str("phase_deg"), py::math1("degrees", &py::cmath("phase", &[v_z.clone()])?)?), (Py::str("real"), py::real(&v_z.clone())?), (Py::str("imag"), py::imag(&v_z.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_plane_wave_lossless(mut v_f: Py, mut v_eps_r: Py, mut v_mu_r: Py) -> R {
    let mut v_idx: Py = Py::None;
    let mut v_lam: Py = Py::None;
    let mut v_v: Py = Py::None;
    let _ = f__check_positive(Py::dict(vec![(Py::str("f"), v_f.clone())]))?;
    if (py::not(&{ let t2144 = py::math1("isfinite", &v_eps_r.clone())?; if !t2144.truthy() { t2144 } else { { let t2143 = { let t2138 = v_eps_r.clone(); Py::Bool({ let t2139 = Py::Int(1i64); if py::ge(&t2138, &t2139)? { true } else { false } }) }; if !t2143.truthy() { t2143 } else { { let t2142 = py::math1("isfinite", &v_mu_r.clone())?; if !t2142.truthy() { t2142 } else { { let t2140 = v_mu_r.clone(); Py::Bool({ let t2141 = Py::Int(1i64); if py::ge(&t2140, &t2141)? { true } else { false } }) } } } } } } })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the relative permittivity and permeability must be at least 1"))));
    }
    v_idx = py::math1("sqrt", &py::mul(&v_eps_r.clone(), &v_mu_r.clone())?)?;
    v_v = py::truediv(&c__C0()?, &v_idx.clone())?;
    v_lam = py::truediv(&v_v.clone(), &v_f.clone())?;
    return Ok(Py::dict(vec![(Py::str("v"), v_v.clone()), (Py::str("wavelength"), v_lam.clone()), (Py::str("eta"), py::mul(&py::mul(&c__MU0()?, &c__C0()?)?, &py::math1("sqrt", &py::truediv(&v_mu_r.clone(), &v_eps_r.clone())?)?)?), (Py::str("beta"), py::truediv(&c__TWO_PI()?, &v_lam.clone())?), (Py::str("n"), v_idx.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_plane_wave_lossy(mut v_f: Py, mut v_eps_r: Py, mut v_mu_r: Py, mut v_sigma: Py) -> R {
    let mut v_a0: Py = Py::None;
    let mut v_alpha: Py = Py::None;
    let mut v_beta: Py = Py::None;
    let mut v_eps: Py = Py::None;
    let mut v_mu: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_x: Py = Py::None;
    let _ = f__check_positive(Py::dict(vec![(Py::str("f"), v_f.clone()), (Py::str("sigma"), v_sigma.clone())]))?;
    if (py::not(&{ let t2157 = py::math1("isfinite", &v_eps_r.clone())?; if !t2157.truthy() { t2157 } else { { let t2156 = { let t2151 = v_eps_r.clone(); Py::Bool({ let t2152 = Py::Int(1i64); if py::ge(&t2151, &t2152)? { true } else { false } }) }; if !t2156.truthy() { t2156 } else { { let t2155 = py::math1("isfinite", &v_mu_r.clone())?; if !t2155.truthy() { t2155 } else { { let t2153 = v_mu_r.clone(); Py::Bool({ let t2154 = Py::Int(1i64); if py::ge(&t2153, &t2154)? { true } else { false } }) } } } } } } })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the relative permittivity and permeability must be at least 1"))));
    }
    v_w = py::mul(&c__TWO_PI()?, &v_f.clone())?;
    v_eps = py::mul(&c__EPS0()?, &v_eps_r.clone())?;
    v_mu = py::mul(&c__MU0()?, &v_mu_r.clone())?;
    v_x = py::truediv(&v_sigma.clone(), &py::mul(&v_w.clone(), &v_eps.clone())?)?;
    v_s = py::hypot(&[Py::Float(1.0), v_x.clone()])?;
    v_a0 = py::mul(&v_w.clone(), &py::math1("sqrt", &py::truediv(&py::mul(&v_mu.clone(), &v_eps.clone())?, &Py::Float(2.0))?)?)?;
    v_alpha = py::truediv(&py::mul(&v_a0.clone(), &v_x.clone())?, &py::math1("sqrt", &py::add(&v_s.clone(), &Py::Float(1.0))?)?)?;
    v_beta = py::mul(&v_a0.clone(), &py::math1("sqrt", &py::add(&v_s.clone(), &Py::Float(1.0))?)?)?;
    return Ok(Py::dict(vec![(Py::str("alpha"), v_alpha.clone()), (Py::str("beta"), v_beta.clone()), (Py::str("skin_depth"), py::truediv(&Py::Float(1.0), &v_alpha.clone())?), (Py::str("v"), py::truediv(&v_w.clone(), &v_beta.clone())?), (Py::str("wavelength"), py::truediv(&c__TWO_PI()?, &v_beta.clone())?), (Py::str("eta"), py::math1("sqrt", &py::truediv(&py::mul(&v_w.clone(), &v_mu.clone())?, &py::hypot(&[v_sigma.clone(), py::mul(&v_w.clone(), &v_eps.clone())?])?)?)?), (Py::str("eta_angle_deg"), py::mul(&Py::Float(0.5), &py::math1("degrees", &py::atan2(&v_sigma.clone(), &py::mul(&v_w.clone(), &v_eps.clone())?)?)?)?), (Py::str("loss_tangent"), v_x.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_polar_to_rectangular(mut v_magnitude: Py, mut v_angle_deg: Py) -> R {
    let mut v_a: Py = Py::None;
    if (py::not(&{ let t2160 = py::math1("isfinite", &v_magnitude.clone())?; if !t2160.truthy() { t2160 } else { py::math1("isfinite", &v_angle_deg.clone())? } })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("magnitude and angle must be finite"))));
    }
    if ({ let t2161 = v_magnitude.clone(); Py::Bool({ let t2162 = Py::Int(0i64); if py::lt(&t2161, &t2162)? { true } else { false } }) }).truthy() {
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
    if (py::not(&{ let t2179 = { let t2173 = Py::Int(0i64); Py::Bool({ let t2174 = v_pf_initial.clone(); if py::lt(&t2173, &t2174)? { { let t2175 = Py::Int(1i64); if py::le(&t2174, &t2175)? { true } else { false } } } else { false } }) }; if !t2179.truthy() { t2179 } else { { let t2176 = Py::Int(0i64); Py::Bool({ let t2177 = v_pf_target.clone(); if py::lt(&t2176, &t2177)? { { let t2178 = Py::Int(1i64); if py::le(&t2177, &t2178)? { true } else { false } } } else { false } }) } } })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("power factors must be between 0 and 1"))));
    }
    if ({ let t2180 = v_pf_target.clone(); Py::Bool({ let t2181 = v_pf_initial.clone(); if py::le(&t2180, &t2181)? { true } else { false } }) }).truthy() {
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
    v_h = (if ({ let t2188 = v_filter.clone(); Py::Bool({ let t2189 = Py::str("lowpass"); if py::eq(&t2188, &t2189) { true } else { false } }) }).truthy() { py::truediv(&Py::Int(1i64), &py::complex(&[Py::Int(1i64), v_x.clone()])?)? } else { py::truediv(&py::complex(&[Py::Int(0i64), v_x.clone()])?, &py::complex(&[Py::Int(1i64), v_x.clone()])?)? });
    return Ok(Py::dict(vec![(Py::str("fc"), v_fc.clone()), (Py::str("gain"), py::abs(&v_h.clone())?), (Py::str("gain_db"), py::mul(&Py::Int(20i64), &py::math1("log10", &py::abs(&v_h.clone())?)?)?), (Py::str("phase_deg"), py::math1("degrees", &py::cmath("phase", &[v_h.clone()])?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_rectangular_to_polar(mut v_real: Py, mut v_imag: Py) -> R {
    if ({ let t2197 = { let t2193 = v_real.clone(); Py::Bool({ let t2194 = Py::Int(0i64); if py::eq(&t2193, &t2194) { true } else { false } }) }; if !t2197.truthy() { t2197 } else { { let t2195 = v_imag.clone(); Py::Bool({ let t2196 = Py::Int(0i64); if py::eq(&t2195, &t2196) { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the phasor 0 has no angle"))));
    }
    return Ok(Py::dict(vec![(Py::str("magnitude"), py::hypot(&[v_real.clone(), v_imag.clone()])?), (Py::str("angle_deg"), py::math1("degrees", &py::atan2(&v_imag.clone(), &v_real.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_resistance_at_temperature(mut v_R1: Py, mut v_alpha1: Py, mut v_T1: Py, mut v_T2: Py) -> R {
    let mut v_g: Py = Py::None;
    let _ = f__check_positive(Py::dict(vec![(Py::str("R1"), v_R1.clone()), (Py::str("T1"), v_T1.clone()), (Py::str("T2"), v_T2.clone())]))?;
    if (py::not(&py::math1("isfinite", &v_alpha1.clone())?)).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("alpha1 must be a finite number"))));
    }
    v_g = py::add(&Py::Float(1.0), &py::mul(&v_alpha1.clone(), &py::sub(&v_T2.clone(), &v_T1.clone())?)?)?;
    if ({ let t2207 = v_g.clone(); Py::Bool({ let t2208 = Py::Int(0i64); if py::le(&t2207, &t2208)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the linear model gives no positive resistance at T2: the change of temperature is too large for this coefficient"))));
    }
    return Ok(Py::dict(vec![(Py::str("R2"), py::mul(&v_R1.clone(), &v_g.clone())?), (Py::str("alpha2"), py::truediv(&v_alpha1.clone(), &v_g.clone())?), (Py::str("change"), py::mul(&py::mul(&v_R1.clone(), &v_alpha1.clone())?, &py::sub(&v_T2.clone(), &v_T1.clone())?)?)]));
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
    if ({ let t2249 = v_amplitude.clone(); Py::Bool({ let t2250 = Py::Int(0i64); if py::lt(&t2249, &t2250)? { true } else { false } }) }).truthy() {
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
    if ({ let t2258 = py::len(&v_c.clone())?; Py::Bool({ let t2259 = Py::Int(2i64); if py::lt(&t2258, &t2259)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("give a polynomial of degree 1 or more"))));
    }
    if (Py::Bool(({ let mut out = vec![]; for t2260 in py::iter(&(v_c.clone()))? { let mut v_x: Py; 
v_x = t2260;
out.push(py::not(&py::math1("isfinite", &v_x.clone())?)); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("coefficients must be finite"))));
    }
    v_n = py::sub(&py::len(&v_c.clone())?, &Py::Int(1i64))?;
    v_width = py::add(&py::floordiv(&v_n.clone(), &Py::Int(2i64))?, &Py::Int(1i64))?;
    v_r0 = py::add(&py::slice(&v_c.clone(), Some((Py::Int(0i64)).i()?), None, Some((Py::Int(2i64)).i()?))?, &py::mul(&Py::list(vec![Py::Float(0.0)]), &py::sub(&v_width.clone(), &py::len(&py::slice(&v_c.clone(), Some((Py::Int(0i64)).i()?), None, Some((Py::Int(2i64)).i()?))?)?)?)?)?;
    v_r1 = py::add(&py::slice(&v_c.clone(), Some((Py::Int(1i64)).i()?), None, Some((Py::Int(2i64)).i()?))?, &py::mul(&Py::list(vec![Py::Float(0.0)]), &py::sub(&v_width.clone(), &py::len(&py::slice(&v_c.clone(), Some((Py::Int(1i64)).i()?), None, Some((Py::Int(2i64)).i()?))?)?)?)?)?;
    v_rows = Py::list(vec![v_r0.clone(), v_r1.clone()]);
    v_scale = py::minmax(&[{ let mut out = vec![]; for t2261 in py::iter(&(v_c.clone()))? { let mut v_x: Py; 
v_x = t2261;
out.push(py::abs(&v_x.clone())?); } Py::list(out) }], true)?;
    for t2262 in py::range_iter(&[(py::sub(&v_n.clone(), &Py::Int(1i64))?).i()?]) {
        v__ = t2262;
        let t2263 = (Py::tuple(vec![py::getitem(&v_rows.clone(), &py::neg(&Py::Int(2i64))?)?, py::getitem(&v_rows.clone(), &py::neg(&Py::Int(1i64))?)?])).items()?;
        if t2263.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_a = t2263[0].clone();
        v_b = t2263[1].clone();
        if ({ let t2264 = py::abs(&py::getitem(&v_b.clone(), &Py::Int(0i64))?)?; Py::Bool({ let t2265 = py::mul(&Py::Float(1e-12), &v_scale.clone())?; if py::le(&t2264, &t2265)? { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("a zero appears in the first column of the Routh array (a special case: roots on or symmetric about the imaginary axis); this capability does not handle it"))));
        }
        v_new = py::add(&{ let mut out = vec![]; for t2266 in py::range_iter(&[(py::sub(&v_width.clone(), &Py::Int(1i64))?).i()?]) { let mut v_i: Py; 
v_i = t2266;
out.push(py::truediv(&py::sub(&py::mul(&py::getitem(&v_b.clone(), &Py::Int(0i64))?, &py::getitem(&v_a.clone(), &py::add(&v_i.clone(), &Py::Int(1i64))?)?)?, &py::mul(&py::getitem(&v_a.clone(), &Py::Int(0i64))?, &py::getitem(&v_b.clone(), &py::add(&v_i.clone(), &Py::Int(1i64))?)?)?)?, &py::getitem(&v_b.clone(), &Py::Int(0i64))?)?); } Py::list(out) }, &Py::list(vec![Py::Float(0.0)]))?;
        let _ = py::method(&v_rows.clone(), "append", &[v_new.clone()])?;
        v_scale = py::minmax(&[v_scale.clone(), py::minmax(&[{ let mut out = vec![]; for t2267 in py::iter(&(v_new.clone()))? { let mut v_x: Py; 
v_x = t2267;
out.push(py::abs(&v_x.clone())?); } Py::list(out) }], true)?], true)?;
    }
    v_first = { let mut out = vec![]; for t2268 in py::iter(&(v_rows.clone()))? { let mut v_r: Py; 
v_r = t2268;
out.push(py::getitem(&v_r.clone(), &Py::Int(0i64))?); } Py::list(out) };
    if (Py::Bool(({ let mut out = vec![]; for t2269 in py::iter(&(v_first.clone()))? { let mut v_x: Py; 
v_x = t2269;
out.push({ let t2270 = py::abs(&v_x.clone())?; Py::Bool({ let t2271 = py::mul(&Py::Float(1e-12), &v_scale.clone())?; if py::le(&t2270, &t2271)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a zero appears in the first column of the Routh array (a special case); this capability does not handle it"))));
    }
    v_changes = py::sum(&{ let mut out = vec![]; for t2272 in py::iter(&(py::zip(&[v_first.clone(), py::slice(&v_first.clone(), Some((Py::Int(1i64)).i()?), None, None)?])?))? { let mut v_x: Py; let mut v_y: Py; 
let t2273 = (t2272).items()?;
if t2273.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t2273[0].clone();
v_y = t2273[1].clone();
if ({ let t2274 = { let t2276 = v_x.clone(); Py::Bool({ let t2277 = Py::Int(0i64); if py::gt(&t2276, &t2277)? { true } else { false } }) }; Py::Bool({ let t2275 = { let t2278 = v_y.clone(); Py::Bool({ let t2279 = Py::Int(0i64); if py::gt(&t2278, &t2279)? { true } else { false } }) }; if py::ne(&t2274, &t2275) { true } else { false } }) }).truthy() { out.push(Py::Int(1i64)); }} Py::list(out) }, None)?;
    return Ok(Py::dict(vec![(Py::str("rhp_roots"), py::float(&v_changes.clone())?), (Py::str("stable"), { let t2280 = v_changes.clone(); Py::Bool({ let t2281 = Py::Int(0i64); if py::eq(&t2280, &t2281) { true } else { false } }) }), (Py::str("first_column"), v_first.clone())]));
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
    if (py::not(&{ let t2337 = Py::Int(0i64); Py::Bool({ let t2338 = v_zeta.clone(); if py::lt(&t2337, &t2338)? { { let t2339 = Py::Int(1i64); if py::lt(&t2338, &t2339)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the damping ratio must be between 0 and 1 (an underdamped system)"))));
    }
    if (py::not(&{ let t2342 = { let t2340 = v_wn.clone(); Py::Bool({ let t2341 = Py::Int(0i64); if py::gt(&t2340, &t2341)? { true } else { false } }) }; if !t2342.truthy() { t2342 } else { py::math1("isfinite", &v_wn.clone())? } })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the natural frequency must be positive"))));
    }
    v_q = py::math1("sqrt", &py::sub(&Py::Int(1i64), &py::mul(&v_zeta.clone(), &v_zeta.clone())?)?)?;
    let t2343 = (Py::tuple(vec![py::mul(&v_wn.clone(), &v_q.clone())?, py::mul(&v_zeta.clone(), &v_wn.clone())?])).items()?;
    if t2343.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_wd = t2343[0].clone();
    v_sg = t2343[1].clone();
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
    while ({ let t2358 = { let t2354 = v_k.clone(); Py::Bool({ let t2355 = Py::Int(0i64); if py::gt(&t2354, &t2355)? { true } else { false } }) }; if !t2358.truthy() { t2358 } else { { let t2356 = py::math1("exp", &py::truediv(&py::mul(&py::mul(&py::neg(&v_sg.clone())?, &v_k.clone())?, &Py::Float(std::f64::consts::PI))?, &v_wd.clone())?)?; Py::Bool({ let t2357 = Py::Float(0.02); if py::le(&t2356, &t2357)? { true } else { false } }) } } }).truthy() {
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
fn f_self_inductance(mut v_N: Py, mut v_flux: Py, mut v_I: Py) -> R {
    let _ = f__check_positive(Py::dict(vec![(Py::str("N"), v_N.clone()), (Py::str("flux"), v_flux.clone()), (Py::str("I"), v_I.clone())]))?;
    return Ok(Py::dict(vec![(Py::str("L"), py::truediv(&py::mul(&v_N.clone(), &v_flux.clone())?, &v_I.clone())?), (Py::str("flux_linkage"), py::mul(&v_N.clone(), &v_flux.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_series_ac_supply(mut v_V: Py, mut v_f: Py, mut v_R: Py, mut v_circuit: Py, mut v_L: Py, mut v_C: Py) -> R {
    let mut v_cur: Py = Py::None;
    let mut v_nature: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_x: Py = Py::None;
    let mut v_xc: Py = Py::None;
    let mut v_xl: Py = Py::None;
    let mut v_z: Py = Py::None;
    let _ = f__ac_check(v_V.clone(), v_f.clone(), v_R.clone(), v_circuit.clone(), v_L.clone(), v_C.clone())?;
    v_w = py::mul(&c__TWO_PI()?, &v_f.clone())?;
    v_xl = (if ({ let t2403 = v_L.clone(); Py::Bool({ let t2404 = Py::Int(0i64); if py::gt(&t2403, &t2404)? { true } else { false } }) }).truthy() { py::mul(&v_w.clone(), &v_L.clone())? } else { Py::Float(0.0) });
    v_xc = (if ({ let t2405 = v_C.clone(); Py::Bool({ let t2406 = Py::Int(0i64); if py::gt(&t2405, &t2406)? { true } else { false } }) }).truthy() { py::truediv(&Py::Float(1.0), &py::mul(&v_w.clone(), &v_C.clone())?)? } else { Py::Float(0.0) });
    v_x = py::sub(&v_xl.clone(), &v_xc.clone())?;
    if ({ let t2407 = py::abs(&v_x.clone())?; Py::Bool({ let t2408 = py::mul(&Py::Float(1e-12), &py::minmax(&[v_xl.clone(), v_xc.clone()], true)?)?; if py::le(&t2407, &t2408)? { true } else { false } }) }).truthy() {
        v_x = Py::Float(0.0);
    }
    v_z = py::hypot(&[v_R.clone(), v_x.clone()])?;
    v_cur = py::truediv(&v_V.clone(), &v_z.clone())?;
    v_nature = (if ({ let t2409 = v_x.clone(); Py::Bool({ let t2410 = Py::Int(0i64); if py::eq(&t2409, &t2410) { true } else { false } }) }).truthy() { Py::str("unity") } else { (if ({ let t2411 = v_x.clone(); Py::Bool({ let t2412 = Py::Int(0i64); if py::gt(&t2411, &t2412)? { true } else { false } }) }).truthy() { Py::str("lagging") } else { Py::str("leading") }) });
    return Ok(Py::dict(vec![(Py::str("Z"), v_z.clone()), (Py::str("I"), v_cur.clone()), (Py::str("VR"), py::mul(&v_cur.clone(), &v_R.clone())?), (Py::str("VL"), py::mul(&v_cur.clone(), &v_xl.clone())?), (Py::str("VC"), py::mul(&v_cur.clone(), &v_xc.clone())?), (Py::str("XL"), v_xl.clone()), (Py::str("XC"), v_xc.clone()), (Py::str("phase_deg"), py::add(&py::math1("degrees", &py::atan2(&v_x.clone(), &v_R.clone())?)?, &Py::Float(0.0))?), (Py::str("power_factor"), py::truediv(&v_R.clone(), &v_z.clone())?), (Py::str("P"), py::mul(&py::mul(&v_cur.clone(), &v_cur.clone())?, &v_R.clone())?), (Py::str("Q"), py::add(&py::mul(&py::mul(&v_cur.clone(), &v_cur.clone())?, &v_x.clone())?, &Py::Float(0.0))?), (Py::str("S"), py::mul(&v_V.clone(), &v_cur.clone())?), (Py::str("nature"), v_nature.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_series_rlc_impedance(mut v_R: Py, mut v_L: Py, mut v_C: Py, mut v_f: Py) -> R {
    let mut v_X: Py = Py::None;
    let mut v_w: Py = Py::None;
    if (py::not(&{ let t2468 = py::math1("isfinite", &v_R.clone())?; if !t2468.truthy() { t2468 } else { { let t2466 = v_R.clone(); Py::Bool({ let t2467 = Py::Int(0i64); if py::ge(&t2466, &t2467)? { true } else { false } }) } } })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("R must be zero or positive"))));
    }
    let _ = f__check_positive(Py::dict(vec![(Py::str("L"), v_L.clone()), (Py::str("C"), v_C.clone()), (Py::str("f"), v_f.clone())]))?;
    v_w = py::mul(&py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?, &v_f.clone())?;
    v_X = py::sub(&py::mul(&v_w.clone(), &v_L.clone())?, &py::truediv(&Py::Float(1.0), &py::mul(&v_w.clone(), &v_C.clone())?)?)?;
    if ({ let t2474 = py::abs(&v_X.clone())?; Py::Bool({ let t2475 = py::mul(&Py::Float(1e-12), &py::minmax(&[py::mul(&v_w.clone(), &v_L.clone())?, py::truediv(&Py::Float(1.0), &py::mul(&v_w.clone(), &v_C.clone())?)?], true)?)?; if py::le(&t2474, &t2475)? { true } else { false } }) }).truthy() {
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
    if (py::not(&{ let t2495 = py::math1("isfinite", &v_V.clone())?; if !t2495.truthy() { t2495 } else { { let t2494 = py::math1("isfinite", &v_t.clone())?; if !t2494.truthy() { t2494 } else { { let t2492 = v_t.clone(); Py::Bool({ let t2493 = Py::Int(0i64); if py::ge(&t2492, &t2493)? { true } else { false } }) } } } } })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the step voltage must be finite and the time zero or positive"))));
    }
    let t2496 = (Py::tuple(vec![py::truediv(&v_R.clone(), &py::mul(&Py::Int(2i64), &v_L.clone())?)?, py::truediv(&Py::Int(1i64), &py::math1("sqrt", &py::mul(&v_L.clone(), &v_C.clone())?)?)?])).items()?;
    if t2496.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t2496[0].clone();
    v_w0 = t2496[1].clone();
    v_z = py::truediv(&v_a.clone(), &v_w0.clone())?;
    if ({ let t2497 = py::abs(&py::sub(&v_z.clone(), &Py::Int(1i64))?)?; Py::Bool({ let t2498 = Py::Float(1e-09); if py::le(&t2497, &t2498)? { true } else { false } }) }).truthy() {
        v_vc = py::mul(&v_V.clone(), &py::sub(&Py::Int(1i64), &py::mul(&py::add(&Py::Int(1i64), &py::mul(&v_a.clone(), &v_t.clone())?)?, &py::math1("exp", &py::mul(&py::neg(&v_a.clone())?, &v_t.clone())?)?)?)?)?;
        v_i = py::mul(&py::mul(&py::truediv(&v_V.clone(), &v_L.clone())?, &v_t.clone())?, &py::math1("exp", &py::mul(&py::neg(&v_a.clone())?, &v_t.clone())?)?)?;
        v_kind = Py::str("critically damped");
    } else {
        if ({ let t2499 = v_z.clone(); Py::Bool({ let t2500 = Py::Int(1i64); if py::lt(&t2499, &t2500)? { true } else { false } }) }).truthy() {
            v_wd = py::math1("sqrt", &py::sub(&py::mul(&v_w0.clone(), &v_w0.clone())?, &py::mul(&v_a.clone(), &v_a.clone())?)?)?;
            v_e = py::math1("exp", &py::mul(&py::neg(&v_a.clone())?, &v_t.clone())?)?;
            v_vc = py::mul(&v_V.clone(), &py::sub(&Py::Int(1i64), &py::mul(&v_e.clone(), &py::add(&py::math1("cos", &py::mul(&v_wd.clone(), &v_t.clone())?)?, &py::mul(&py::truediv(&v_a.clone(), &v_wd.clone())?, &py::math1("sin", &py::mul(&v_wd.clone(), &v_t.clone())?)?)?)?)?)?)?;
            v_i = py::mul(&py::mul(&py::truediv(&v_V.clone(), &py::mul(&v_wd.clone(), &v_L.clone())?)?, &v_e.clone())?, &py::math1("sin", &py::mul(&v_wd.clone(), &v_t.clone())?)?)?;
            v_kind = Py::str("underdamped");
        } else {
            v_r = py::math1("sqrt", &py::sub(&py::mul(&v_a.clone(), &v_a.clone())?, &py::mul(&v_w0.clone(), &v_w0.clone())?)?)?;
            let t2501 = (Py::tuple(vec![py::add(&py::neg(&v_a.clone())?, &v_r.clone())?, py::sub(&py::neg(&v_a.clone())?, &v_r.clone())?])).items()?;
            if t2501.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_s1 = t2501[0].clone();
            v_s2 = t2501[1].clone();
            let t2502 = (Py::tuple(vec![py::math1("exp", &py::mul(&v_s1.clone(), &v_t.clone())?)?, py::math1("exp", &py::mul(&v_s2.clone(), &v_t.clone())?)?])).items()?;
            if t2502.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_e1 = t2502[0].clone();
            v_e2 = t2502[1].clone();
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
fn f_sinusoid_time_to_value(mut v_Vm: Py, mut v_w: Py, mut v_phi: Py, mut v_v: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_best: Py = Py::None;
    let mut v_direction: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_slope: Py = Py::None;
    let mut v_th: Py = Py::None;
    let mut v_x: Py = Py::None;
    let _ = f__check_positive(Py::dict(vec![(Py::str("Vm"), v_Vm.clone()), (Py::str("w"), v_w.clone())]))?;
    let _ = f__check_phase(v_phi.clone())?;
    if (py::not(&{ let t2526 = py::math1("isfinite", &v_v.clone())?; if !t2526.truthy() { t2526 } else { { let t2524 = py::abs(&v_v.clone())?; Py::Bool({ let t2525 = v_Vm.clone(); if py::le(&t2524, &t2525)? { true } else { false } }) } } })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a sinusoid never reaches a value beyond its amplitude"))));
    }
    v_r = py::truediv(&v_v.clone(), &v_Vm.clone())?;
    if ({ let t2527 = v_r.clone(); Py::Bool({ let t2528 = Py::Float(1.0); if py::gt(&t2527, &t2528)? { true } else { false } }) }).truthy() {
        v_r = Py::Float(1.0);
    }
    if ({ let t2529 = v_r.clone(); Py::Bool({ let t2530 = py::neg(&Py::Float(1.0))?; if py::lt(&t2529, &t2530)? { true } else { false } }) }).truthy() {
        v_r = py::neg(&Py::Float(1.0))?;
    }
    v_a = py::math1("asin", &v_r.clone())?;
    v_best = Py::Float(10.0);
    for t2531 in py::iter(&(Py::list(vec![v_a.clone(), py::sub(&Py::Float(std::f64::consts::PI), &v_a.clone())?])))? {
        v_th = t2531;
        v_x = py::sub(&v_th.clone(), &v_phi.clone())?;
        while ({ let t2532 = v_x.clone(); Py::Bool({ let t2533 = Py::Int(0i64); if py::lt(&t2532, &t2533)? { true } else { false } }) }).truthy() {
            v_x = py::iadd(&v_x, &c__TWO_PI()?)?;
        }
        while ({ let t2534 = v_x.clone(); Py::Bool({ let t2535 = c__TWO_PI()?; if py::ge(&t2534, &t2535)? { true } else { false } }) }).truthy() {
            v_x = py::sub(&v_x, &c__TWO_PI()?)?;
        }
        if ({ let t2540 = { let t2536 = v_x.clone(); Py::Bool({ let t2537 = Py::Float(1e-12); if py::le(&t2536, &t2537)? { true } else { false } }) }; if t2540.truthy() { t2540 } else { { let t2538 = py::sub(&c__TWO_PI()?, &v_x.clone())?; Py::Bool({ let t2539 = Py::Float(1e-12); if py::le(&t2538, &t2539)? { true } else { false } }) } } }).truthy() {
            v_x = Py::Float(0.0);
        }
        if ({ let t2541 = v_x.clone(); Py::Bool({ let t2542 = v_best.clone(); if py::lt(&t2541, &t2542)? { true } else { false } }) }).truthy() {
            v_best = v_x.clone();
        }
    }
    v_slope = py::math1("cos", &py::add(&v_best.clone(), &v_phi.clone())?)?;
    v_direction = (if ({ let t2543 = v_slope.clone(); Py::Bool({ let t2544 = Py::Float(1e-09); if py::gt(&t2543, &t2544)? { true } else { false } }) }).truthy() { Py::str("rising") } else { (if ({ let t2545 = v_slope.clone(); Py::Bool({ let t2546 = py::neg(&Py::Float(1e-09))?; if py::lt(&t2545, &t2546)? { true } else { false } }) }).truthy() { Py::str("falling") } else { Py::str("peak") }) });
    return Ok(Py::dict(vec![(Py::str("t"), py::truediv(&v_best.clone(), &v_w.clone())?), (Py::str("T"), py::truediv(&c__TWO_PI()?, &v_w.clone())?), (Py::str("direction"), v_direction.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_sinusoid_values(mut v_Vm: Py, mut v_w: Py, mut v_phi: Py, mut v_t: Py) -> R {
    let mut v_rel: Py = Py::None;
    let _ = f__check_positive(Py::dict(vec![(Py::str("Vm"), v_Vm.clone()), (Py::str("w"), v_w.clone())]))?;
    let _ = f__check_phase(v_phi.clone())?;
    if (py::not(&py::math1("isfinite", &v_t.clone())?)).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the time must be a finite number"))));
    }
    v_rel = (if ({ let t2576 = v_phi.clone(); Py::Bool({ let t2577 = Py::Int(0i64); if py::eq(&t2576, &t2577) { true } else { false } }) }).truthy() { Py::str("in phase") } else { (if ({ let t2578 = v_phi.clone(); Py::Bool({ let t2579 = Py::Int(0i64); if py::gt(&t2578, &t2579)? { true } else { false } }) }).truthy() { Py::str("leads") } else { Py::str("lags") }) });
    return Ok(Py::dict(vec![(Py::str("v"), py::mul(&v_Vm.clone(), &py::math1("sin", &py::add(&py::mul(&v_w.clone(), &v_t.clone())?, &v_phi.clone())?)?)?), (Py::str("f"), py::truediv(&v_w.clone(), &c__TWO_PI()?)?), (Py::str("T"), py::truediv(&c__TWO_PI()?, &v_w.clone())?), (Py::str("phase_deg"), py::add(&py::math1("degrees", &v_phi.clone())?, &Py::Float(0.0))?), (Py::str("relation"), v_rel.clone()), (Py::str("time_shift"), py::add(&py::truediv(&v_phi.clone(), &v_w.clone())?, &Py::Float(0.0))?), (Py::str("rms"), py::truediv(&v_Vm.clone(), &py::math1("sqrt", &Py::Float(2.0))?)?), (Py::str("average"), py::truediv(&py::mul(&Py::Float(2.0), &v_Vm.clone())?, &Py::Float(std::f64::consts::PI))?), (Py::str("form_factor"), py::truediv(&Py::Float(std::f64::consts::PI), &py::mul(&Py::Float(2.0), &py::math1("sqrt", &Py::Float(2.0))?)?)?), (Py::str("peak_factor"), py::math1("sqrt", &Py::Float(2.0))?)]));
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
    if ({ let t2595 = Py::Bool(({ let mut out = vec![]; for t2587 in py::iter(&(v_A.clone()))? { let mut v_r: Py; 
v_r = t2587;
out.push({ let t2588 = py::len(&v_r.clone())?; Py::Bool({ let t2589 = v_n.clone(); if py::ne(&t2588, &t2589) { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())); if t2595.truthy() { t2595 } else { { let t2594 = { let t2590 = py::len(&v_B.clone())?; Py::Bool({ let t2591 = v_n.clone(); if py::ne(&t2590, &t2591) { true } else { false } }) }; if t2594.truthy() { t2594 } else { { let t2592 = py::len(&v_C.clone())?; Py::Bool({ let t2593 = v_n.clone(); if py::ne(&t2592, &t2593) { true } else { false } }) } } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("A must be n x n and B, C must have n entries"))));
    }
    let t2609 = (f__charpoly_adj(v_A.clone())?).items()?;
    if t2609.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_den = t2609[0].clone();
    v_Ns = t2609[1].clone();
    v_num = { let mut out = vec![]; for t2610 in py::iter(&(v_den.clone()))? { let mut v_d: Py; 
v_d = t2610;
out.push(py::mul(&v_D.clone(), &v_d.clone())?); } Py::list(out) };
    for t2611 in py::iter(&(py::enumerate(&v_Ns.clone(), 0)?))? {
        let t2616 = (t2611).items()?;
        if t2616.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2616[0].clone();
        v_N = t2616[1].clone();
        let t2612 = v_num.clone(); let t2613 = py::add(&v_k.clone(), &Py::Int(1i64))?;
        py::setitem(&t2612, &t2613, py::iadd(&py::getitem(&t2612, &t2613)?, &py::sum(&{ let mut out = vec![]; for t2614 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t2614;
for t2615 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t2615;
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
    let t2677 = (f__check_tf(v_num.clone(), v_den.clone())?).items()?;
    if t2677.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_num = t2677[0].clone();
    v_den = t2677[1].clone();
    v_char = f__ptrim(f__padd(v_den.clone(), v_num.clone())?)?;
    if ({ let t2681 = py::minmax(&[{ let mut out = vec![]; for t2719 in py::iter(&(f__roots(v_char.clone())?))? { let mut v_r: Py; 
v_r = t2719;
out.push(py::real(&py::complex(&[v_r.clone()])?)?); } Py::list(out) }], true)?; Py::Bool({ let t2682 = py::mul(&py::neg(&Py::Float(1e-09))?, &py::minmax(&[Py::Float(1.0), py::minmax(&[{ let mut out = vec![]; for t2720 in py::iter(&(f__roots(v_char.clone())?))? { let mut v_r: Py; 
v_r = t2720;
out.push(py::abs(&py::complex(&[v_r.clone()])?)?); } Py::list(out) }], true)?], true)?)?; if py::ge(&t2681, &t2682)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the closed loop is not stable, so there is no steady-state error"))));
    }
    v_k = py::getitem(&c__INPUT_ORDER()?, &v_input.clone())?;
    v_zd = Py::Int(0i64);
    while ({ let t2725 = { let t2721 = v_zd.clone(); Py::Bool({ let t2722 = py::len(&v_den.clone())?; if py::lt(&t2721, &t2722)? { true } else { false } }) }; if !t2725.truthy() { t2725 } else { { let t2723 = py::getitem(&v_den.clone(), &py::sub(&py::sub(&py::len(&v_den.clone())?, &Py::Int(1i64))?, &v_zd.clone())?)?; Py::Bool({ let t2724 = Py::Int(0i64); if py::eq(&t2723, &t2724) { true } else { false } }) } } }).truthy() {
        v_zd = py::iadd(&v_zd, &Py::Int(1i64))?;
    }
    v_zn = Py::Int(0i64);
    while ({ let t2730 = { let t2726 = v_zn.clone(); Py::Bool({ let t2727 = py::len(&v_num.clone())?; if py::lt(&t2726, &t2727)? { true } else { false } }) }; if !t2730.truthy() { t2730 } else { { let t2728 = py::getitem(&v_num.clone(), &py::sub(&py::sub(&py::len(&v_num.clone())?, &Py::Int(1i64))?, &v_zn.clone())?)?; Py::Bool({ let t2729 = Py::Int(0i64); if py::eq(&t2728, &t2729) { true } else { false } }) } } }).truthy() {
        v_zn = py::iadd(&v_zn, &Py::Int(1i64))?;
    }
    v_r = py::sub(&v_zd.clone(), &v_zn.clone())?;
    let t2731 = (Py::tuple(vec![py::slice(&v_num.clone(), None, Some((py::sub(&py::len(&v_num.clone())?, &v_zn.clone())?).i()?), None)?, py::slice(&v_den.clone(), None, Some((py::sub(&py::len(&v_den.clone())?, &v_zd.clone())?).i()?), None)?])).items()?;
    if t2731.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_nprime = t2731[0].clone();
    v_dprime = t2731[1].clone();
    if ({ let t2732 = v_k.clone(); Py::Bool({ let t2733 = v_r.clone(); if py::gt(&t2732, &t2733)? { true } else { false } }) }).truthy() {
        v_K = Py::Float(0.0);
    } else {
        if ({ let t2734 = v_k.clone(); Py::Bool({ let t2735 = v_r.clone(); if py::eq(&t2734, &t2735) { true } else { false } }) }).truthy() {
            v_K = py::truediv(&py::getitem(&v_nprime.clone(), &py::neg(&Py::Int(1i64))?)?, &py::getitem(&v_dprime.clone(), &py::neg(&Py::Int(1i64))?)?)?;
        } else {
            v_K = Py::Float(f64::INFINITY);
        }
    }
    if ({ let t2736 = v_k.clone(); Py::Bool({ let t2737 = Py::Int(0i64); if py::eq(&t2736, &t2737) { true } else { false } }) }).truthy() {
        v_ess = (if ({ let t2738 = v_K.clone(); Py::Bool({ let t2739 = Py::Float(f64::INFINITY); if py::eq(&t2738, &t2739) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { py::truediv(&Py::Float(1.0), &py::add(&Py::Float(1.0), &v_K.clone())?)? });
    } else {
        v_ess = (if ({ let t2740 = v_K.clone(); Py::Bool({ let t2741 = Py::Float(f64::INFINITY); if py::eq(&t2740, &t2741) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { (if ({ let t2742 = v_K.clone(); Py::Bool({ let t2743 = Py::Int(0i64); if py::eq(&t2742, &t2743) { true } else { false } }) }).truthy() { Py::str("infinite") } else { py::truediv(&Py::Float(1.0), &v_K.clone())? }) });
    }
    return Ok(Py::dict(vec![(Py::str("steady_state_error"), v_ess.clone()), (Py::str("error_constant"), (if ({ let t2744 = v_K.clone(); Py::Bool({ let t2745 = Py::Float(f64::INFINITY); if py::eq(&t2744, &t2745) { true } else { false } }) }).truthy() { Py::str("infinite") } else { v_K.clone() })), (Py::str("system_type"), py::float(&v_r.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_summing_amplifier(mut v_V: Py, mut v_R: Py, mut v_Rf: Py) -> R {
    let mut v_terms: Py = Py::None;
    if ({ let t2772 = py::len(&v_V.clone())?; Py::Bool({ let t2773 = py::len(&v_R.clone())?; if py::ne(&t2772, &t2773) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("give one input resistor for every input voltage"))));
    }
    if ({ let t2779 = Py::Bool(({ let mut out = vec![]; for t2774 in py::iter(&(v_R.clone()))? { let mut v_r: Py; 
v_r = t2774;
out.push({ let t2775 = v_r.clone(); Py::Bool({ let t2776 = Py::Int(0i64); if py::le(&t2775, &t2776)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())); if t2779.truthy() { t2779 } else { py::not(&{ let t2777 = v_Rf.clone(); Py::Bool({ let t2778 = Py::Int(0i64); if py::gt(&t2777, &t2778)? { true } else { false } }) }) } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("resistances must be positive"))));
    }
    v_terms = { let mut out = vec![]; for t2780 in py::iter(&(py::zip(&[v_V.clone(), v_R.clone()])?))? { let mut v_r: Py; let mut v_v: Py; 
let t2781 = (t2780).items()?;
if t2781.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_v = t2781[0].clone();
v_r = t2781[1].clone();
out.push(py::mul(&py::truediv(&v_Rf.clone(), &v_r.clone())?, &v_v.clone())?); } Py::list(out) };
    return Ok(Py::dict(vec![(Py::str("Vout"), py::neg(&py::fsum(&v_terms.clone())?)?), (Py::str("gains"), { let mut out = vec![]; for t2782 in py::iter(&(v_R.clone()))? { let mut v_r: Py; 
v_r = t2782;
out.push(py::truediv(&py::neg(&v_Rf.clone())?, &v_r.clone())?); } Py::list(out) })]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_temperature_coefficient_from_resistances(mut v_R1: Py, mut v_T1: Py, mut v_R2: Py, mut v_T2: Py) -> R {
    let mut v_slope: Py = Py::None;
    let _ = f__check_positive(Py::dict(vec![(Py::str("R1"), v_R1.clone()), (Py::str("T1"), v_T1.clone()), (Py::str("R2"), v_R2.clone()), (Py::str("T2"), v_T2.clone())]))?;
    if ({ let t2795 = v_T1.clone(); Py::Bool({ let t2796 = v_T2.clone(); if py::eq(&t2795, &t2796) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the two temperatures must differ"))));
    }
    v_slope = py::truediv(&py::sub(&v_R2.clone(), &v_R1.clone())?, &py::sub(&v_T2.clone(), &v_T1.clone())?)?;
    return Ok(Py::dict(vec![(Py::str("alpha1"), py::truediv(&v_slope.clone(), &v_R1.clone())?), (Py::str("alpha2"), py::truediv(&v_slope.clone(), &v_R2.clone())?), (Py::str("slope"), v_slope.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_thevenin_equivalent(mut v_netlist: Py, mut v_node_a: Py, mut v_node_b: Py) -> R {
    let mut v_rth: Py = Py::None;
    let mut v_vth: Py = Py::None;
    let t2930 = (f__thevenin(v_netlist.clone(), v_node_a.clone(), v_node_b.clone())?).items()?;
    if t2930.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_vth = t2930[0].clone();
    v_rth = t2930[1].clone();
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
    if (py::not(&{ let t2938 = { let t2936 = v_VL.clone(); Py::Bool({ let t2937 = Py::Int(0i64); if py::gt(&t2936, &t2937)? { true } else { false } }) }; if !t2938.truthy() { t2938 } else { py::math1("isfinite", &v_VL.clone())? } })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the line voltage must be positive"))));
    }
    if ({ let t2946 = { let t2939 = v_R.clone(); Py::Bool({ let t2940 = Py::Int(0i64); if py::lt(&t2939, &t2940)? { true } else { false } }) }; if t2946.truthy() { t2946 } else { { let t2945 = { let t2941 = v_R.clone(); Py::Bool({ let t2942 = Py::Int(0i64); if py::eq(&t2941, &t2942) { true } else { false } }) }; if !t2945.truthy() { t2945 } else { { let t2943 = v_X.clone(); Py::Bool({ let t2944 = Py::Int(0i64); if py::eq(&t2943, &t2944) { true } else { false } }) } } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the phase impedance must have R >= 0 and not be zero"))));
    }
    v_Z = py::complex(&[v_R.clone(), v_X.clone()])?;
    if ({ let t2947 = v_connection.clone(); Py::Bool({ let t2948 = Py::str("wye"); if py::eq(&t2947, &t2948) { true } else { false } }) }).truthy() {
        v_vph = py::truediv(&v_VL.clone(), &py::math1("sqrt", &Py::Int(3i64))?)?;
        v_iph = py::truediv(&v_vph.clone(), &py::abs(&v_Z.clone())?)?;
        v_il = v_iph.clone();
    } else {
        v_vph = v_VL.clone();
        v_iph = py::truediv(&v_vph.clone(), &py::abs(&v_Z.clone())?)?;
        v_il = py::mul(&py::math1("sqrt", &Py::Int(3i64))?, &v_iph.clone())?;
    }
    let t2949 = (Py::tuple(vec![py::mul(&py::mul(&py::mul(&Py::Int(3i64), &v_iph.clone())?, &v_iph.clone())?, &v_R.clone())?, py::mul(&py::mul(&py::mul(&Py::Int(3i64), &v_iph.clone())?, &v_iph.clone())?, &v_X.clone())?])).items()?;
    if t2949.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_P = t2949[0].clone();
    v_Q = t2949[1].clone();
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
    if ({ let t2998 = v_n.clone(); Py::Bool({ let t2999 = Py::Int(0i64); if py::eq(&t2998, &t2999) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the expression has no variables"))));
    }
    if ({ let t3000 = v_n.clone(); Py::Bool({ let t3001 = Py::Int(4i64); if py::gt(&t3000, &t3001)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("minimization is offered for up to 4 variables"))));
    }
    v_on = { let mut out = vec![]; for t3002 in py::range_iter(&[(py::pow(&Py::Int(2i64), &v_n.clone())?).i()?]) { let mut v_m: Py; 
v_m = t3002;
if (f__bool_eval(v_tree.clone(), { let mut out = vec![]; for t3014 in py::iter(&(py::enumerate(&v_names.clone(), 0)?))? { let mut v_i: Py; let mut v_nm: Py; 
let t3015 = (t3014).items()?;
if t3015.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_i = t3015[0].clone();
v_nm = t3015[1].clone();
out.push((v_nm.clone(), py::bitand(&py::rshift(&v_m.clone(), &py::sub(&py::sub(&v_n.clone(), &Py::Int(1i64))?, &v_i.clone())?)?, &Py::Int(1i64))?)); } Py::dict(out) })?).truthy() { out.push(v_m.clone()); }} Py::list(out) };
    v_out = Py::dict(vec![(Py::str("variables"), py::method(&Py::str(", "), "join", &[v_names.clone()])?), (Py::str("minterms"), { let mut out = vec![]; for t3016 in py::iter(&(v_on.clone()))? { let mut v_m: Py; 
v_m = t3016;
out.push(py::float(&v_m.clone())?); } Py::list(out) }), (Py::str("maxterms"), { let mut out = vec![]; for t3017 in py::range_iter(&[(py::pow(&Py::Int(2i64), &v_n.clone())?).i()?]) { let mut v_m: Py; 
v_m = t3017;
if ({ let t3018 = v_m.clone(); Py::Bool({ let t3019 = v_on.clone(); if !py::contains(&t3019, &t3018)? { true } else { false } }) }).truthy() { out.push(py::float(&v_m.clone())?); }} Py::list(out) }), (Py::str("canonical_sop"), f__canonical_sop(v_n.clone(), v_names.clone(), v_on.clone())?)]);
    let _ = py::method(&v_out.clone(), "update", &[f__sop_out(v_n.clone(), v_names.clone(), v_on.clone(), Py::tuple(vec![]))?])?;
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_two_wattmeter(mut v_W1: Py, mut v_W2: Py) -> R {
    let mut v_p: Py = Py::None;
    let mut v_q: Py = Py::None;
    let mut v_s: Py = Py::None;
    if (py::not(&{ let t3101 = py::math1("isfinite", &v_W1.clone())?; if !t3101.truthy() { t3101 } else { py::math1("isfinite", &v_W2.clone())? } })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the readings must be finite numbers"))));
    }
    v_p = py::add(&v_W1.clone(), &v_W2.clone())?;
    if ({ let t3102 = v_p.clone(); Py::Bool({ let t3103 = Py::Int(0i64); if py::le(&t3102, &t3103)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the sum of the two readings (the total power) must be positive"))));
    }
    v_q = py::mul(&py::math1("sqrt", &Py::Float(3.0))?, &py::abs(&py::sub(&v_W1.clone(), &v_W2.clone())?)?)?;
    v_s = py::hypot(&[v_p.clone(), v_q.clone()])?;
    return Ok(Py::dict(vec![(Py::str("P"), v_p.clone()), (Py::str("Q"), v_q.clone()), (Py::str("S"), v_s.clone()), (Py::str("power_factor"), py::truediv(&v_p.clone(), &v_s.clone())?), (Py::str("phase_deg"), py::math1("degrees", &py::atan2(&v_q.clone(), &v_p.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_twos_complement_decode(mut v_pattern: Py) -> R {
    let mut v_n: Py = Py::None;
    let mut v_p: Py = Py::None;
    v_p = py::method(&v_pattern.clone(), "strip", &[])?;
    if ({ let t3108 = py::not(&v_p.clone()); if t3108.truthy() { t3108 } else { { let t3107 = py::sub(&py::set(&v_p.clone())?, &py::set(&Py::list(vec![Py::str("0"), Py::str("1")]))?)?; if t3107.truthy() { t3107 } else { { let t3105 = py::len(&v_p.clone())?; Py::Bool({ let t3106 = Py::Int(64i64); if py::gt(&t3105, &t3106)? { true } else { false } }) } } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the pattern must be 1 to 64 binary digits"))));
    }
    v_n = py::len(&v_p.clone())?;
    return Ok(Py::dict(vec![(Py::str("value"), py::float(&py::sub(&py::int_base(&v_p.clone(), &Py::Int(2i64))?, &(if ({ let t3109 = py::getitem(&v_p.clone(), &Py::Int(0i64))?; Py::Bool({ let t3110 = Py::str("1"); if py::eq(&t3109, &t3110) { true } else { false } }) }).truthy() { py::pow(&Py::Int(2i64), &v_n.clone())? } else { Py::Int(0i64) }))?)?), (Py::str("bits"), py::float(&v_n.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_twos_complement_encode(mut v_value: Py, mut v_bits: Py) -> R {
    let mut v_n: Py = Py::None;
    let mut v_u: Py = Py::None;
    let mut v_v: Py = Py::None;
    let t3112 = (Py::tuple(vec![py::int(&v_value.clone())?, py::int(&v_bits.clone())?])).items()?;
    if t3112.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_v = t3112[0].clone();
    v_n = t3112[1].clone();
    if (py::not(&{ let t3113 = Py::Int(2i64); Py::Bool({ let t3114 = v_n.clone(); if py::le(&t3113, &t3114)? { { let t3115 = Py::Int(64i64); if py::le(&t3114, &t3115)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the width must be 2 to 64 bits"))));
    }
    if (py::not(&{ let t3116 = py::neg(&py::pow(&Py::Int(2i64), &py::sub(&v_n.clone(), &Py::Int(1i64))?)?)?; Py::Bool({ let t3117 = v_v.clone(); if py::le(&t3116, &t3117)? { { let t3118 = py::pow(&Py::Int(2i64), &py::sub(&v_n.clone(), &Py::Int(1i64))?)?; if py::lt(&t3117, &t3118)? { true } else { false } } } else { false } }) })).truthy() {
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
fn f_voltmeter_multiplier(mut v_Ifs: Py, mut v_Rm: Py, mut v_V: Py) -> R {
    let mut v_total: Py = Py::None;
    let mut v_vfs: Py = Py::None;
    let _ = f__check_positive(Py::dict(vec![(Py::str("Ifs"), v_Ifs.clone()), (Py::str("Rm"), v_Rm.clone()), (Py::str("V"), v_V.clone())]))?;
    v_vfs = py::mul(&v_Ifs.clone(), &v_Rm.clone())?;
    if ({ let t3140 = v_V.clone(); Py::Bool({ let t3141 = v_vfs.clone(); if py::le(&t3140, &t3141)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the new range must be larger than the meter's full-scale voltage Ifs Rm"))));
    }
    v_total = py::truediv(&v_V.clone(), &v_Ifs.clone())?;
    return Ok(Py::dict(vec![(Py::str("Rmult"), py::sub(&v_total.clone(), &v_Rm.clone())?), (Py::str("R_total"), v_total.clone()), (Py::str("sensitivity"), py::truediv(&Py::Float(1.0), &v_Ifs.clone())?), (Py::str("V_fs"), v_vfs.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_wheatstone_bridge(mut v_Vs: Py, mut v_R1: Py, mut v_R2: Py, mut v_R3: Py, mut v_R4: Py) -> R {
    let mut v_vout: Py = Py::None;
    let _ = f__check_positive(Py::dict(vec![(Py::str("R1"), v_R1.clone()), (Py::str("R2"), v_R2.clone()), (Py::str("R3"), v_R3.clone()), (Py::str("R4"), v_R4.clone())]))?;
    v_vout = py::mul(&v_Vs.clone(), &py::sub(&py::truediv(&v_R2.clone(), &py::add(&v_R1.clone(), &v_R2.clone())?)?, &py::truediv(&v_R4.clone(), &py::add(&v_R3.clone(), &v_R4.clone())?)?)?)?;
    return Ok(Py::dict(vec![(Py::str("Vout"), v_vout.clone()), (Py::str("balanced"), { let t3148 = py::abs(&py::sub(&py::mul(&v_R1.clone(), &v_R4.clone())?, &py::mul(&v_R2.clone(), &v_R3.clone())?)?)?; Py::Bool({ let t3149 = py::mul(&py::mul(&Py::Float(1e-12), &v_R1.clone())?, &v_R4.clone())?; if py::le(&t3148, &t3149)? { true } else { false } }) }), (Py::str("R4_balance"), py::truediv(&py::mul(&v_R2.clone(), &v_R3.clone())?, &v_R1.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_wye_to_delta(mut v_Ra: Py, mut v_Rb: Py, mut v_Rc: Py) -> R {
    let mut v_n: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_v: Py = Py::None;
    for t3151 in py::iter(&(Py::tuple(vec![Py::tuple(vec![Py::str("Ra"), v_Ra.clone()]), Py::tuple(vec![Py::str("Rb"), v_Rb.clone()]), Py::tuple(vec![Py::str("Rc"), v_Rc.clone()])])))? {
        let t3155 = (t3151).items()?;
        if t3155.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_n = t3155[0].clone();
        v_v = t3155[1].clone();
        if (py::not(&{ let t3154 = { let t3152 = v_v.clone(); Py::Bool({ let t3153 = Py::Int(0i64); if py::gt(&t3152, &t3153)? { true } else { false } }) }; if !t3154.truthy() { t3154 } else { py::math1("isfinite", &v_v.clone())? } })).truthy() {
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
    if ({ let t3165 = py::truediv(&py::mul(&v_Vin.clone(), &v_RL.clone())?, &py::add(&v_R.clone(), &v_RL.clone())?)?; Py::Bool({ let t3166 = v_Vz.clone(); if py::le(&t3165, &t3166)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("without the Zener the load voltage would not reach Vz: the diode is not in breakdown (no regulation)"))));
    }
    let t3167 = (Py::tuple(vec![py::truediv(&py::sub(&v_Vin.clone(), &v_Vz.clone())?, &v_R.clone())?, py::truediv(&v_Vz.clone(), &v_RL.clone())?])).items()?;
    if t3167.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_ir = t3167[0].clone();
    v_il = t3167[1].clone();
    v_iz = py::sub(&v_ir.clone(), &v_il.clone())?;
    return Ok(Py::dict(vec![(Py::str("VL"), v_Vz.clone()), (Py::str("IR"), v_ir.clone()), (Py::str("IL"), v_il.clone()), (Py::str("IZ"), v_iz.clone()), (Py::str("IZ_mA"), py::mul(&v_iz.clone(), &Py::Float(1000.0))?), (Py::str("PZ"), py::mul(&v_Vz.clone(), &v_iz.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1029(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::add(&py::truediv(&py::abs(&py::sub(&py::add(&py::getitem(&v_o.clone(), &Py::str("I1"))?, &py::getitem(&v_o.clone(), &Py::str("I2"))?)?, &py::getitem(&v_i.clone(), &Py::str("I"))?)?)?, &py::minmax(&[py::abs(&py::getitem(&v_i.clone(), &Py::str("I"))?)?, Py::Float(1e-300)], true)?)?, &py::truediv(&py::abs(&py::sub(&py::mul(&py::getitem(&v_o.clone(), &Py::str("I1"))?, &py::getitem(&v_i.clone(), &Py::str("R1"))?)?, &py::mul(&py::getitem(&v_o.clone(), &Py::str("I2"))?, &py::getitem(&v_i.clone(), &Py::str("R2"))?)?)?)?, &py::minmax(&[py::abs(&py::mul(&py::getitem(&v_i.clone(), &Py::str("I"))?, &py::getitem(&v_i.clone(), &Py::str("R1"))?)?)?, Py::Float(1e-300)], true)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1059(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::add(&py::add(&py::add(&py::truediv(&py::abs(&py::sub(&py::sub(&py::getitem(&v_i.clone(), &Py::str("V"))?, &py::mul(&py::getitem(&v_i.clone(), &Py::str("R"))?, &py::getitem(&v_o.clone(), &Py::str("current"))?)?)?, &py::mul(&py::getitem(&v_i.clone(), &Py::str("Ke"))?, &py::getitem(&v_o.clone(), &Py::str("speed"))?)?)?)?, &py::minmax(&[py::abs(&py::getitem(&v_i.clone(), &Py::str("V"))?)?, Py::Float(1e-300)], true)?)?, &py::truediv(&py::abs(&py::sub(&py::sub(&py::mul(&py::getitem(&v_i.clone(), &Py::str("Kt"))?, &py::getitem(&v_o.clone(), &Py::str("current"))?)?, &py::mul(&py::getitem(&v_i.clone(), &Py::str("b"))?, &py::getitem(&v_o.clone(), &Py::str("speed"))?)?)?, &py::getitem(&v_i.clone(), &Py::str("T"))?)?)?, &py::minmax(&[py::abs(&py::mul(&py::getitem(&v_i.clone(), &Py::str("Kt"))?, &py::getitem(&v_o.clone(), &Py::str("current"))?)?)?, py::abs(&py::getitem(&v_i.clone(), &Py::str("T"))?)?, Py::Float(1e-300)], true)?)?)?, &f__rel(py::truediv(&py::mul(&py::mul(&py::getitem(&v_o.clone(), &Py::str("rpm"))?, &Py::Int(2i64))?, &Py::Float(std::f64::consts::PI))?, &Py::Int(60i64))?, py::getitem(&v_o.clone(), &Py::str("speed"))?)?)?, &py::truediv(&py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("input_power"))?, &py::mul(&py::getitem(&v_i.clone(), &Py::str("V"))?, &py::getitem(&v_o.clone(), &Py::str("current"))?)?)?)?, &py::minmax(&[py::abs(&py::mul(&py::getitem(&v_i.clone(), &Py::str("V"))?, &py::getitem(&v_o.clone(), &Py::str("current"))?)?)?, Py::Float(1e-300)], true)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1254(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__verify_dw(v_i.clone(), v_o.clone(), Py::Bool(true))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1280(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::add(&py::add(&py::truediv(&py::abs(&py::sub(&py::sub(&py::getitem(&v_i.clone(), &Py::str("Vs"))?, &py::mul(&py::getitem(&v_o.clone(), &Py::str("I"))?, &py::getitem(&v_i.clone(), &Py::str("R"))?)?)?, &py::getitem(&v_o.clone(), &Py::str("VD"))?)?)?, &py::getitem(&v_i.clone(), &Py::str("Vs"))?)?, &py::truediv(&py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("I"))?, &py::mul(&py::getitem(&v_i.clone(), &Py::str("Is"))?, &py::math1("expm1", &py::truediv(&py::getitem(&v_o.clone(), &Py::str("VD"))?, &py::mul(&py::getitem(&v_i.clone(), &Py::str("n"))?, &py::getitem(&v_i.clone(), &Py::str("VT"))?)?)?)?)?)?)?, &py::minmax(&[py::getitem(&v_o.clone(), &Py::str("I"))?, Py::Float(1e-300)], true)?)?)?, &f__rel(py::getitem(&v_o.clone(), &Py::str("I_mA"))?, py::mul(&py::getitem(&v_o.clone(), &Py::str("I"))?, &Py::Float(1000.0))?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1305(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::add(&py::truediv(&py::abs(&py::sub(&py::add(&py::getitem(&v_i.clone(), &Py::str("Vf"))?, &py::mul(&py::sub(&py::getitem(&v_i.clone(), &Py::str("V0"))?, &py::getitem(&v_i.clone(), &Py::str("Vf"))?)?, &py::math1("exp", &py::truediv(&py::neg(&py::getitem(&v_o.clone(), &Py::str("t"))?)?, &py::getitem(&v_i.clone(), &Py::str("tau"))?)?)?)?)?, &py::getitem(&v_i.clone(), &Py::str("v"))?)?)?, &py::minmax(&[py::abs(&py::getitem(&v_i.clone(), &Py::str("V0"))?)?, py::abs(&py::getitem(&v_i.clone(), &Py::str("Vf"))?)?, Py::Float(1e-300)], true)?)?, &f__rel(py::mul(&py::getitem(&v_o.clone(), &Py::str("time_constants"))?, &py::getitem(&v_i.clone(), &Py::str("tau"))?)?, py::getitem(&v_o.clone(), &Py::str("t"))?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1313(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::add(&py::truediv(&py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("v"))?, &py::add(&py::getitem(&v_i.clone(), &Py::str("Vf"))?, &py::mul(&py::sub(&py::getitem(&v_i.clone(), &Py::str("V0"))?, &py::getitem(&v_i.clone(), &Py::str("Vf"))?)?, &py::sub(&Py::Int(1i64), &py::getitem(&v_o.clone(), &Py::str("fraction_complete"))?)?)?)?)?)?, &py::minmax(&[py::abs(&py::getitem(&v_i.clone(), &Py::str("V0"))?)?, py::abs(&py::getitem(&v_i.clone(), &Py::str("Vf"))?)?, Py::Float(1e-300)], true)?)?, &py::truediv(&py::abs(&py::add(&py::math1("log", &py::minmax(&[Py::Float(1e-300), py::sub(&Py::Int(1i64), &py::getitem(&v_o.clone(), &Py::str("fraction_complete"))?)?], true)?)?, &py::truediv(&py::getitem(&v_i.clone(), &Py::str("t"))?, &py::getitem(&v_i.clone(), &Py::str("tau"))?)?)?)?, &py::minmax(&[Py::Float(1.0), py::truediv(&py::getitem(&v_i.clone(), &Py::str("t"))?, &py::getitem(&v_i.clone(), &Py::str("tau"))?)?], true)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1429(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::add(&f__rel(py::getitem(&v_o.clone(), &Py::str("energy"))?, py::mul(&py::mul(&Py::Float(0.5), &py::getitem(&v_o.clone(), &Py::str("flux_linkage"))?)?, &py::getitem(&v_i.clone(), &Py::str("I"))?)?)?, &f__rel(py::getitem(&v_o.clone(), &Py::str("flux_linkage"))?, py::mul(&py::getitem(&v_i.clone(), &Py::str("L"))?, &py::getitem(&v_i.clone(), &Py::str("I"))?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1430(mut v_Rin: Py, mut v_Rf: Py, mut v_Vin: Py) -> R {
    return Ok(py::getitem(&Py::tuple(vec![f__check_positive(Py::dict(vec![(Py::str("Rin"), v_Rin.clone()), (Py::str("Rf"), v_Rf.clone())]))?, Py::dict(vec![(Py::str("gain"), py::truediv(&py::neg(&v_Rf.clone())?, &v_Rin.clone())?), (Py::str("Vout"), py::mul(&py::truediv(&py::neg(&v_Rf.clone())?, &v_Rin.clone())?, &v_Vin.clone())?)])]), &Py::Int(1i64))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1436(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::add(&py::truediv(&py::abs(&py::add(&py::truediv(&py::getitem(&v_i.clone(), &Py::str("Vin"))?, &py::getitem(&v_i.clone(), &Py::str("Rin"))?)?, &py::truediv(&py::getitem(&v_o.clone(), &Py::str("Vout"))?, &py::getitem(&v_i.clone(), &Py::str("Rf"))?)?)?)?, &py::minmax(&[py::abs(&py::truediv(&py::getitem(&v_i.clone(), &Py::str("Vin"))?, &py::getitem(&v_i.clone(), &Py::str("Rin"))?)?)?, Py::Float(1e-300)], true)?)?, &f__rel(py::getitem(&v_o.clone(), &Py::str("gain"))?, py::truediv(&py::neg(&py::getitem(&v_i.clone(), &Py::str("Rf"))?)?, &py::getitem(&v_i.clone(), &Py::str("Rin"))?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1444(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::add(&py::add(&py::truediv(&py::abs(&py::sub(&py::math1("degrees", &py::sub(&py::math1("atan", &py::truediv(&py::getitem(&v_o.clone(), &Py::str("w_max"))?, &py::getitem(&v_i.clone(), &Py::str("z"))?)?)?, &py::math1("atan", &py::truediv(&py::getitem(&v_o.clone(), &Py::str("w_max"))?, &py::getitem(&v_i.clone(), &Py::str("p"))?)?)?)?)?, &py::getitem(&v_o.clone(), &Py::str("phase_max_deg"))?)?)?, &Py::Int(90i64))?, &py::mul(&py::abs(&py::sub(&py::truediv(&py::getitem(&v_i.clone(), &Py::str("z"))?, &py::add(&py::pow(&py::getitem(&v_i.clone(), &Py::str("z"))?, &Py::Int(2i64))?, &py::pow(&py::getitem(&v_o.clone(), &Py::str("w_max"))?, &Py::Int(2i64))?)?)?, &py::truediv(&py::getitem(&v_i.clone(), &Py::str("p"))?, &py::add(&py::pow(&py::getitem(&v_i.clone(), &Py::str("p"))?, &Py::Int(2i64))?, &py::pow(&py::getitem(&v_o.clone(), &Py::str("w_max"))?, &Py::Int(2i64))?)?)?)?)?, &py::getitem(&v_o.clone(), &Py::str("w_max"))?)?)?, &py::truediv(&py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("gain_at_w_max_db"))?, &py::mul(&Py::Int(10i64), &py::math1("log10", &py::truediv(&py::add(&py::pow(&py::getitem(&v_i.clone(), &Py::str("z"))?, &Py::Int(2i64))?, &py::pow(&py::getitem(&v_o.clone(), &Py::str("w_max"))?, &Py::Int(2i64))?)?, &py::add(&py::pow(&py::getitem(&v_i.clone(), &Py::str("p"))?, &Py::Int(2i64))?, &py::pow(&py::getitem(&v_o.clone(), &Py::str("w_max"))?, &Py::Int(2i64))?)?)?)?)?)?)?, &py::minmax(&[Py::Float(1.0), py::abs(&py::getitem(&v_o.clone(), &Py::str("gain_at_w_max_db"))?)?], true)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1499(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::add(&py::add(&f__rel(py::getitem(&v_o.clone(), &Py::str("RL"))?, py::getitem(&v_i.clone(), &Py::str("Rth"))?)?, &f__rel(py::getitem(&v_o.clone(), &Py::str("Pmax"))?, py::truediv(&py::mul(&py::pow(&py::getitem(&v_i.clone(), &Py::str("Vth"))?, &Py::Int(2i64))?, &py::getitem(&v_o.clone(), &Py::str("RL"))?)?, &py::pow(&py::add(&py::getitem(&v_i.clone(), &Py::str("Rth"))?, &py::getitem(&v_o.clone(), &Py::str("RL"))?)?, &Py::Int(2i64))?)?)?)?, &py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("efficiency"))?, &Py::Float(0.5))?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1500(mut v_A: Py, mut v_b: Py) -> R {
    let mut v_n: Py = Py::None;
    let mut v_x: Py = Py::None;
    v_n = py::len(&v_A.clone())?;
    if (Py::Bool(({ let mut out = vec![]; for t1501 in py::iter(&(v_A.clone()))? { let mut v_r: Py; 
v_r = t1501;
out.push({ let t1502 = py::len(&v_r.clone())?; Py::Bool({ let t1503 = v_n.clone(); if py::ne(&t1502, &t1503) { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the matrix must be square"))));
    }
    if ({ let t1504 = py::len(&v_b.clone())?; Py::Bool({ let t1505 = v_n.clone(); if py::ne(&t1504, &t1505) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&["the vector needs ".to_string(), py::format(&v_n.clone(), "")?, " entries, one per row of the matrix".to_string()].concat()))));
    }
    v_x = f__gauss(v_A.clone(), v_b.clone())?;
    return Ok(Py::dict(vec![(Py::str("currents"), v_x.clone()), (Py::str("first"), py::getitem(&v_x.clone(), &Py::Int(0i64))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1533(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_bi: Py = Py::None;
    let mut v_row: Py = Py::None;
    let mut v_scale: Py = Py::None;
    let mut v_terms: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let mut v_x: Py = Py::None;
    let t1534 = (Py::tuple(vec![py::getitem(&v_inputs.clone(), &Py::str("A"))?, py::getitem(&v_inputs.clone(), &Py::str("b"))?, py::getitem(&v_outputs.clone(), &Py::str("currents"))?])).items()?;
    if t1534.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_A = t1534[0].clone();
    v_b = t1534[1].clone();
    v_x = t1534[2].clone();
    if ({ let t1535 = py::len(&v_x.clone())?; Py::Bool({ let t1536 = py::len(&v_b.clone())?; if py::ne(&t1535, &t1536) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_worst = Py::Float(0.0);
    for t1537 in py::iter(&(py::zip(&[v_A.clone(), v_b.clone()])?))? {
        let t1542 = (t1537).items()?;
        if t1542.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_row = t1542[0].clone();
        v_bi = t1542[1].clone();
        v_terms = { let mut out = vec![]; for t1538 in py::iter(&(py::zip(&[v_row.clone(), v_x.clone()])?))? { let mut v_a: Py; let mut v_xi: Py; 
let t1539 = (t1538).items()?;
if t1539.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t1539[0].clone();
v_xi = t1539[1].clone();
out.push(py::mul(&v_a.clone(), &v_xi.clone())?); } Py::list(out) };
        v_scale = { let t1541 = py::add(&py::fsum(&{ let mut out = vec![]; for t1540 in py::iter(&(v_terms.clone()))? { let mut v_t: Py; 
v_t = t1540;
out.push(py::abs(&v_t.clone())?); } Py::list(out) })?, &py::abs(&v_bi.clone())?)?; if t1541.truthy() { t1541 } else { Py::Float(1.0) } };
        v_worst = py::minmax(&[v_worst.clone(), py::truediv(&py::abs(&py::sub(&py::fsum(&v_terms.clone())?, &v_bi.clone())?)?, &v_scale.clone())?], true)?;
    }
    return Ok((if ({ let t1543 = py::getitem(&v_x.clone(), &Py::Int(0i64))?; Py::Bool({ let t1544 = py::getitem(&v_outputs.clone(), &Py::str("first"))?; if py::ne(&t1543, &t1544) { true } else { false } }) }).truthy() { py::add(&v_worst.clone(), &f__rel(py::getitem(&v_outputs.clone(), &Py::str("first"))?, py::getitem(&v_x.clone(), &Py::Int(0i64))?)?)? } else { v_worst.clone() }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1683(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::add(&py::add(&py::add(&f__rel(py::getitem(&v_o.clone(), &Py::str("ID"))?, py::mul(&py::mul(&py::mul(&Py::Float(0.5), &py::getitem(&v_i.clone(), &Py::str("kn"))?)?, &py::getitem(&v_i.clone(), &Py::str("WL"))?)?, &py::pow(&py::getitem(&v_o.clone(), &Py::str("VOV"))?, &Py::Int(2i64))?)?)?, &py::truediv(&py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("VOV"))?, &py::sub(&py::getitem(&v_i.clone(), &Py::str("VGS"))?, &py::getitem(&v_i.clone(), &Py::str("VTH"))?)?)?)?, &py::abs(&py::getitem(&v_i.clone(), &Py::str("VGS"))?)?)?)?, &f__rel(py::mul(&py::getitem(&v_o.clone(), &Py::str("gm"))?, &py::getitem(&v_o.clone(), &Py::str("VOV"))?)?, py::mul(&Py::Int(2i64), &py::getitem(&v_o.clone(), &Py::str("ID"))?)?)?)?, &f__rel(py::getitem(&v_o.clone(), &Py::str("ID_mA"))?, py::mul(&py::getitem(&v_o.clone(), &Py::str("ID"))?, &Py::Float(1000.0))?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1694(mut v_A: Py, mut v_b: Py) -> R {
    let mut v_n: Py = Py::None;
    let mut v_x: Py = Py::None;
    v_n = py::len(&v_A.clone())?;
    if (Py::Bool(({ let mut out = vec![]; for t1695 in py::iter(&(v_A.clone()))? { let mut v_r: Py; 
v_r = t1695;
out.push({ let t1696 = py::len(&v_r.clone())?; Py::Bool({ let t1697 = v_n.clone(); if py::ne(&t1696, &t1697) { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the matrix must be square"))));
    }
    if ({ let t1698 = py::len(&v_b.clone())?; Py::Bool({ let t1699 = v_n.clone(); if py::ne(&t1698, &t1699) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&["the vector needs ".to_string(), py::format(&v_n.clone(), "")?, " entries, one per row of the matrix".to_string()].concat()))));
    }
    v_x = f__gauss(v_A.clone(), v_b.clone())?;
    return Ok(Py::dict(vec![(Py::str("voltages"), v_x.clone()), (Py::str("first"), py::getitem(&v_x.clone(), &Py::Int(0i64))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1727(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_bi: Py = Py::None;
    let mut v_row: Py = Py::None;
    let mut v_scale: Py = Py::None;
    let mut v_terms: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let mut v_x: Py = Py::None;
    let t1728 = (Py::tuple(vec![py::getitem(&v_inputs.clone(), &Py::str("A"))?, py::getitem(&v_inputs.clone(), &Py::str("b"))?, py::getitem(&v_outputs.clone(), &Py::str("voltages"))?])).items()?;
    if t1728.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_A = t1728[0].clone();
    v_b = t1728[1].clone();
    v_x = t1728[2].clone();
    if ({ let t1729 = py::len(&v_x.clone())?; Py::Bool({ let t1730 = py::len(&v_b.clone())?; if py::ne(&t1729, &t1730) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_worst = Py::Float(0.0);
    for t1731 in py::iter(&(py::zip(&[v_A.clone(), v_b.clone()])?))? {
        let t1736 = (t1731).items()?;
        if t1736.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_row = t1736[0].clone();
        v_bi = t1736[1].clone();
        v_terms = { let mut out = vec![]; for t1732 in py::iter(&(py::zip(&[v_row.clone(), v_x.clone()])?))? { let mut v_a: Py; let mut v_xi: Py; 
let t1733 = (t1732).items()?;
if t1733.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t1733[0].clone();
v_xi = t1733[1].clone();
out.push(py::mul(&v_a.clone(), &v_xi.clone())?); } Py::list(out) };
        v_scale = { let t1735 = py::add(&py::fsum(&{ let mut out = vec![]; for t1734 in py::iter(&(v_terms.clone()))? { let mut v_t: Py; 
v_t = t1734;
out.push(py::abs(&v_t.clone())?); } Py::list(out) })?, &py::abs(&v_bi.clone())?)?; if t1735.truthy() { t1735 } else { Py::Float(1.0) } };
        v_worst = py::minmax(&[v_worst.clone(), py::truediv(&py::abs(&py::sub(&py::fsum(&v_terms.clone())?, &v_bi.clone())?)?, &v_scale.clone())?], true)?;
    }
    return Ok((if ({ let t1737 = py::getitem(&v_x.clone(), &Py::Int(0i64))?; Py::Bool({ let t1738 = py::getitem(&v_outputs.clone(), &Py::str("first"))?; if py::ne(&t1737, &t1738) { true } else { false } }) }).truthy() { py::add(&v_worst.clone(), &f__rel(py::getitem(&v_outputs.clone(), &Py::str("first"))?, py::getitem(&v_x.clone(), &Py::Int(0i64))?)?)? } else { v_worst.clone() }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1739(mut v_Rg: Py, mut v_Rf: Py, mut v_Vin: Py) -> R {
    return Ok(py::getitem(&Py::tuple(vec![f__check_positive(Py::dict(vec![(Py::str("Rg"), v_Rg.clone()), (Py::str("Rf"), v_Rf.clone())]))?, Py::dict(vec![(Py::str("gain"), py::add(&Py::Int(1i64), &py::truediv(&v_Rf.clone(), &v_Rg.clone())?)?), (Py::str("Vout"), py::mul(&py::add(&Py::Int(1i64), &py::truediv(&v_Rf.clone(), &v_Rg.clone())?)?, &v_Vin.clone())?)])]), &Py::Int(1i64))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1745(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::add(&py::truediv(&py::abs(&py::sub(&py::truediv(&py::mul(&py::getitem(&v_o.clone(), &Py::str("Vout"))?, &py::getitem(&v_i.clone(), &Py::str("Rg"))?)?, &py::add(&py::getitem(&v_i.clone(), &Py::str("Rg"))?, &py::getitem(&v_i.clone(), &Py::str("Rf"))?)?)?, &py::getitem(&v_i.clone(), &Py::str("Vin"))?)?)?, &py::minmax(&[py::abs(&py::getitem(&v_i.clone(), &Py::str("Vin"))?)?, Py::Float(1e-300)], true)?)?, &f__rel(py::getitem(&v_o.clone(), &Py::str("gain"))?, py::add(&Py::Int(1i64), &py::truediv(&py::getitem(&v_i.clone(), &Py::str("Rf"))?, &py::getitem(&v_i.clone(), &Py::str("Rg"))?)?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1891(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::add(&py::truediv(&py::abs(&py::sub(&py::sub(&py::getitem(&v_o.clone(), &Py::str("Vout"))?, &py::getitem(&v_i.clone(), &Py::str("V0"))?)?, &py::mul(&py::getitem(&v_o.clone(), &Py::str("slope"))?, &py::getitem(&v_i.clone(), &Py::str("t"))?)?)?)?, &py::minmax(&[py::abs(&py::getitem(&v_i.clone(), &Py::str("V0"))?)?, py::abs(&py::getitem(&v_o.clone(), &Py::str("Vout"))?)?, Py::Float(1e-300)], true)?)?, &f__rel(py::mul(&py::mul(&py::getitem(&v_o.clone(), &Py::str("slope"))?, &py::getitem(&v_i.clone(), &Py::str("R"))?)?, &py::getitem(&v_i.clone(), &Py::str("C"))?)?, py::neg(&py::getitem(&v_i.clone(), &Py::str("Vin"))?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1949(mut v_kw: Py) -> R {
    let mut v_out: Py = Py::None;
    let mut v_total: Py = Py::None;
    let mut v_vals: Py = Py::None;
    v_vals = py::getitem(&v_kw.clone(), &Py::str("C"))?;
    let _ = f__positive_values(v_vals.clone(), Py::str("capacitors"))?;
    v_total = (if (Py::Bool(true)).truthy() { py::sum(&v_vals.clone(), None)? } else { py::truediv(&Py::Float(1.0), &py::sum(&{ let mut out = vec![]; for t1956 in py::iter(&(v_vals.clone()))? { let mut v_v: Py; 
v_v = t1956;
out.push(py::truediv(&Py::Float(1.0), &v_v.clone())?); } Py::list(out) }, None)?)? });
    v_out = Py::dict(vec![(Py::str("equivalent"), v_total.clone()), (Py::str("count"), py::float(&py::len(&v_vals.clone())?)?)]);
    if ({ let t1957 = Py::str("capacitance"); Py::Bool({ let t1958 = Py::str("resistance"); if py::ne(&t1957, &t1958) { true } else { false } }) }).truthy() {
        py::setitem(&v_out.clone(), &Py::str("unit"), { let t1959 = py::method(&v_kw.clone(), "get", &[Py::str("unit")])?; if t1959.truthy() { t1959 } else { Py::str("the unit of the inputs") } })?;
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1960(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_eq: Py = Py::None;
    let mut v_vals: Py = Py::None;
    let t1961 = (Py::tuple(vec![py::getitem(&v_inputs.clone(), &Py::str("C"))?, py::getitem(&v_outputs.clone(), &Py::str("equivalent"))?])).items()?;
    if t1961.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_vals = t1961[0].clone();
    v_eq = t1961[1].clone();
    if (Py::Bool(true)).truthy() {
        return Ok(py::add(&py::truediv(&py::abs(&py::sub(&v_eq.clone(), &py::fsum(&v_vals.clone())?)?)?, &py::fsum(&v_vals.clone())?)?, &py::abs(&py::sub(&py::method(&v_outputs.clone(), "get", &[Py::str("count"), py::len(&v_vals.clone())?])?, &py::len(&v_vals.clone())?)?)?)?);
    }
    return Ok(py::add(&py::abs(&py::sub(&py::fsum(&{ let mut out = vec![]; for t1962 in py::iter(&(v_vals.clone()))? { let mut v_v: Py; 
v_v = t1962;
out.push(py::truediv(&v_eq.clone(), &v_v.clone())?); } Py::list(out) })?, &Py::Float(1.0))?)?, &py::abs(&py::sub(&py::method(&v_outputs.clone(), "get", &[Py::str("count"), py::len(&v_vals.clone())?])?, &py::len(&v_vals.clone())?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1963(mut v_kw: Py) -> R {
    let mut v_out: Py = Py::None;
    let mut v_total: Py = Py::None;
    let mut v_vals: Py = Py::None;
    v_vals = py::getitem(&v_kw.clone(), &Py::str("L"))?;
    let _ = f__positive_values(v_vals.clone(), Py::str("inductors"))?;
    v_total = (if (Py::Bool(false)).truthy() { py::sum(&v_vals.clone(), None)? } else { py::truediv(&Py::Float(1.0), &py::sum(&{ let mut out = vec![]; for t1970 in py::iter(&(v_vals.clone()))? { let mut v_v: Py; 
v_v = t1970;
out.push(py::truediv(&Py::Float(1.0), &v_v.clone())?); } Py::list(out) }, None)?)? });
    v_out = Py::dict(vec![(Py::str("equivalent"), v_total.clone()), (Py::str("count"), py::float(&py::len(&v_vals.clone())?)?)]);
    if ({ let t1971 = Py::str("inductance"); Py::Bool({ let t1972 = Py::str("resistance"); if py::ne(&t1971, &t1972) { true } else { false } }) }).truthy() {
        py::setitem(&v_out.clone(), &Py::str("unit"), { let t1973 = py::method(&v_kw.clone(), "get", &[Py::str("unit")])?; if t1973.truthy() { t1973 } else { Py::str("the unit of the inputs") } })?;
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1974(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_eq: Py = Py::None;
    let mut v_vals: Py = Py::None;
    let t1975 = (Py::tuple(vec![py::getitem(&v_inputs.clone(), &Py::str("L"))?, py::getitem(&v_outputs.clone(), &Py::str("equivalent"))?])).items()?;
    if t1975.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_vals = t1975[0].clone();
    v_eq = t1975[1].clone();
    if (Py::Bool(false)).truthy() {
        return Ok(py::add(&py::truediv(&py::abs(&py::sub(&v_eq.clone(), &py::fsum(&v_vals.clone())?)?)?, &py::fsum(&v_vals.clone())?)?, &py::abs(&py::sub(&py::method(&v_outputs.clone(), "get", &[Py::str("count"), py::len(&v_vals.clone())?])?, &py::len(&v_vals.clone())?)?)?)?);
    }
    return Ok(py::add(&py::abs(&py::sub(&py::fsum(&{ let mut out = vec![]; for t1976 in py::iter(&(v_vals.clone()))? { let mut v_v: Py; 
v_v = t1976;
out.push(py::truediv(&v_eq.clone(), &v_v.clone())?); } Py::list(out) })?, &Py::Float(1.0))?)?, &py::abs(&py::sub(&py::method(&v_outputs.clone(), "get", &[Py::str("count"), py::len(&v_vals.clone())?])?, &py::len(&v_vals.clone())?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1992(mut v_kw: Py) -> R {
    let mut v_out: Py = Py::None;
    let mut v_total: Py = Py::None;
    let mut v_vals: Py = Py::None;
    v_vals = py::getitem(&v_kw.clone(), &Py::str("R"))?;
    let _ = f__positive_values(v_vals.clone(), Py::str("resistors"))?;
    v_total = (if (Py::Bool(false)).truthy() { py::sum(&v_vals.clone(), None)? } else { py::truediv(&Py::Float(1.0), &py::sum(&{ let mut out = vec![]; for t1999 in py::iter(&(v_vals.clone()))? { let mut v_v: Py; 
v_v = t1999;
out.push(py::truediv(&Py::Float(1.0), &v_v.clone())?); } Py::list(out) }, None)?)? });
    v_out = Py::dict(vec![(Py::str("equivalent"), v_total.clone()), (Py::str("count"), py::float(&py::len(&v_vals.clone())?)?)]);
    if ({ let t2000 = Py::str("resistance"); Py::Bool({ let t2001 = Py::str("resistance"); if py::ne(&t2000, &t2001) { true } else { false } }) }).truthy() {
        py::setitem(&v_out.clone(), &Py::str("unit"), { let t2002 = py::method(&v_kw.clone(), "get", &[Py::str("unit")])?; if t2002.truthy() { t2002 } else { Py::str("the unit of the inputs") } })?;
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2003(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_eq: Py = Py::None;
    let mut v_vals: Py = Py::None;
    let t2004 = (Py::tuple(vec![py::getitem(&v_inputs.clone(), &Py::str("R"))?, py::getitem(&v_outputs.clone(), &Py::str("equivalent"))?])).items()?;
    if t2004.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_vals = t2004[0].clone();
    v_eq = t2004[1].clone();
    if (Py::Bool(false)).truthy() {
        return Ok(py::add(&py::truediv(&py::abs(&py::sub(&v_eq.clone(), &py::fsum(&v_vals.clone())?)?)?, &py::fsum(&v_vals.clone())?)?, &py::abs(&py::sub(&py::method(&v_outputs.clone(), "get", &[Py::str("count"), py::len(&v_vals.clone())?])?, &py::len(&v_vals.clone())?)?)?)?);
    }
    return Ok(py::add(&py::abs(&py::sub(&py::fsum(&{ let mut out = vec![]; for t2005 in py::iter(&(v_vals.clone()))? { let mut v_v: Py; 
v_v = t2005;
out.push(py::truediv(&v_eq.clone(), &v_v.clone())?); } Py::list(out) })?, &Py::Float(1.0))?)?, &py::abs(&py::sub(&py::method(&v_outputs.clone(), "get", &[Py::str("count"), py::len(&v_vals.clone())?])?, &py::len(&v_vals.clone())?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2013(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_C: Py = Py::None;
    let mut v_L: Py = Py::None;
    let mut v_R: Py = Py::None;
    let mut v_Z: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_res: Py = Py::None;
    let mut v_w: Py = Py::None;
    let t2014 = (Py::tuple(vec![py::getitem(&v_inputs.clone(), &Py::str("R"))?, py::getitem(&v_inputs.clone(), &Py::str("L"))?, py::getitem(&v_inputs.clone(), &Py::str("C"))?, py::getitem(&v_inputs.clone(), &Py::str("f"))?])).items()?;
    if t2014.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_R = t2014[0].clone();
    v_L = t2014[1].clone();
    v_C = t2014[2].clone();
    v_f = t2014[3].clone();
    v_w = py::mul(&py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?, &v_f.clone())?;
    v_Z = py::complex(&[py::getitem(&v_outputs.clone(), &Py::str("real"))?, py::getitem(&v_outputs.clone(), &Py::str("imag"))?])?;
    v_res = (if (Py::Bool(false)).truthy() { py::truediv(&py::abs(&py::sub(&v_Z.clone(), &py::add(&py::add(&v_R.clone(), &py::mul(&py::mul(&Py::Complex(0.0, 1.0), &v_w.clone())?, &v_L.clone())?)?, &py::truediv(&Py::Int(1i64), &py::mul(&py::mul(&Py::Complex(0.0, 1.0), &v_w.clone())?, &v_C.clone())?)?)?)?)?, &py::abs(&v_Z.clone())?)? } else { py::abs(&py::sub(&py::mul(&v_Z.clone(), &py::add(&py::add(&py::truediv(&Py::Int(1i64), &v_R.clone())?, &py::truediv(&Py::Int(1i64), &py::mul(&py::mul(&Py::Complex(0.0, 1.0), &v_w.clone())?, &v_L.clone())?)?)?, &py::mul(&py::mul(&Py::Complex(0.0, 1.0), &v_w.clone())?, &v_C.clone())?)?)?, &Py::Int(1i64))?)? });
    return Ok(py::add(&py::add(&py::add(&py::add(&v_res.clone(), &f__rel(py::getitem(&v_outputs.clone(), &Py::str("impedance"))?, py::abs(&v_Z.clone())?)?)?, &py::truediv(&py::abs(&py::sub(&py::getitem(&v_outputs.clone(), &Py::str("phase_deg"))?, &py::math1("degrees", &py::cmath("phase", &[v_Z.clone()])?)?)?)?, &Py::Int(180i64))?)?, &f__rel(py::getitem(&v_outputs.clone(), &Py::str("XL"))?, py::mul(&v_w.clone(), &v_L.clone())?)?)?, &f__rel(py::getitem(&v_outputs.clone(), &Py::str("XC"))?, py::truediv(&Py::Int(1i64), &py::mul(&v_w.clone(), &v_C.clone())?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2015(mut v_R: Py, mut v_L: Py, mut v_C: Py, mut v_s: Py) -> R {
    return Ok(f__resonance(v_R.clone(), v_L.clone(), v_C.clone(), v_s.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2022(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_C: Py = Py::None;
    let mut v_L: Py = Py::None;
    let mut v_R: Py = Py::None;
    let mut v_Y: Py = Py::None;
    let mut v_peak: Py = Py::None;
    let t2023 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("R"))?, py::getitem(&v_i.clone(), &Py::str("L"))?, py::getitem(&v_i.clone(), &Py::str("C"))?])).items()?;
    if t2023.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_R = t2023[0].clone();
    v_L = t2023[1].clone();
    v_C = t2023[2].clone();
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
fn s_2190(mut v_i: Py, mut v_o: Py) -> R {
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
        }) }, &[(if ({ let t2191 = py::getitem(&v_i.clone(), &Py::str("filter"))?; Py::Bool({ let t2192 = Py::str("lowpass"); if py::eq(&t2191, &t2192) { true } else { false } }) }).truthy() { py::truediv(&Py::Int(1i64), &py::add(&Py::Int(1i64), &py::mul(&py::mul(&Py::Complex(0.0, 1.0), &v_w.clone())?, &v_rc.clone())?)?)? } else { py::truediv(&py::mul(&py::mul(&Py::Complex(0.0, 1.0), &v_w.clone())?, &v_rc.clone())?, &py::add(&Py::Int(1i64), &py::mul(&py::mul(&Py::Complex(0.0, 1.0), &v_w.clone())?, &v_rc.clone())?)?)? })])?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, &[py::mul(&py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?, &py::getitem(&v_i.clone(), &Py::str("f"))?)?, py::mul(&py::getitem(&v_i.clone(), &Py::str("R"))?, &py::getitem(&v_i.clone(), &Py::str("C"))?)?])?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2251(mut v_i: Py, mut v_o: Py) -> R {
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
fn s_2424(mut v_kw: Py) -> R {
    let mut v_out: Py = Py::None;
    let mut v_total: Py = Py::None;
    let mut v_vals: Py = Py::None;
    v_vals = py::getitem(&v_kw.clone(), &Py::str("C"))?;
    let _ = f__positive_values(v_vals.clone(), Py::str("capacitors"))?;
    v_total = (if (Py::Bool(false)).truthy() { py::sum(&v_vals.clone(), None)? } else { py::truediv(&Py::Float(1.0), &py::sum(&{ let mut out = vec![]; for t2431 in py::iter(&(v_vals.clone()))? { let mut v_v: Py; 
v_v = t2431;
out.push(py::truediv(&Py::Float(1.0), &v_v.clone())?); } Py::list(out) }, None)?)? });
    v_out = Py::dict(vec![(Py::str("equivalent"), v_total.clone()), (Py::str("count"), py::float(&py::len(&v_vals.clone())?)?)]);
    if ({ let t2432 = Py::str("capacitance"); Py::Bool({ let t2433 = Py::str("resistance"); if py::ne(&t2432, &t2433) { true } else { false } }) }).truthy() {
        py::setitem(&v_out.clone(), &Py::str("unit"), { let t2434 = py::method(&v_kw.clone(), "get", &[Py::str("unit")])?; if t2434.truthy() { t2434 } else { Py::str("the unit of the inputs") } })?;
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2435(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_eq: Py = Py::None;
    let mut v_vals: Py = Py::None;
    let t2436 = (Py::tuple(vec![py::getitem(&v_inputs.clone(), &Py::str("C"))?, py::getitem(&v_outputs.clone(), &Py::str("equivalent"))?])).items()?;
    if t2436.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_vals = t2436[0].clone();
    v_eq = t2436[1].clone();
    if (Py::Bool(false)).truthy() {
        return Ok(py::add(&py::truediv(&py::abs(&py::sub(&v_eq.clone(), &py::fsum(&v_vals.clone())?)?)?, &py::fsum(&v_vals.clone())?)?, &py::abs(&py::sub(&py::method(&v_outputs.clone(), "get", &[Py::str("count"), py::len(&v_vals.clone())?])?, &py::len(&v_vals.clone())?)?)?)?);
    }
    return Ok(py::add(&py::abs(&py::sub(&py::fsum(&{ let mut out = vec![]; for t2437 in py::iter(&(v_vals.clone()))? { let mut v_v: Py; 
v_v = t2437;
out.push(py::truediv(&v_eq.clone(), &v_v.clone())?); } Py::list(out) })?, &Py::Float(1.0))?)?, &py::abs(&py::sub(&py::method(&v_outputs.clone(), "get", &[Py::str("count"), py::len(&v_vals.clone())?])?, &py::len(&v_vals.clone())?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2438(mut v_kw: Py) -> R {
    let mut v_out: Py = Py::None;
    let mut v_total: Py = Py::None;
    let mut v_vals: Py = Py::None;
    v_vals = py::getitem(&v_kw.clone(), &Py::str("L"))?;
    let _ = f__positive_values(v_vals.clone(), Py::str("inductors"))?;
    v_total = (if (Py::Bool(true)).truthy() { py::sum(&v_vals.clone(), None)? } else { py::truediv(&Py::Float(1.0), &py::sum(&{ let mut out = vec![]; for t2445 in py::iter(&(v_vals.clone()))? { let mut v_v: Py; 
v_v = t2445;
out.push(py::truediv(&Py::Float(1.0), &v_v.clone())?); } Py::list(out) }, None)?)? });
    v_out = Py::dict(vec![(Py::str("equivalent"), v_total.clone()), (Py::str("count"), py::float(&py::len(&v_vals.clone())?)?)]);
    if ({ let t2446 = Py::str("inductance"); Py::Bool({ let t2447 = Py::str("resistance"); if py::ne(&t2446, &t2447) { true } else { false } }) }).truthy() {
        py::setitem(&v_out.clone(), &Py::str("unit"), { let t2448 = py::method(&v_kw.clone(), "get", &[Py::str("unit")])?; if t2448.truthy() { t2448 } else { Py::str("the unit of the inputs") } })?;
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2449(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_eq: Py = Py::None;
    let mut v_vals: Py = Py::None;
    let t2450 = (Py::tuple(vec![py::getitem(&v_inputs.clone(), &Py::str("L"))?, py::getitem(&v_outputs.clone(), &Py::str("equivalent"))?])).items()?;
    if t2450.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_vals = t2450[0].clone();
    v_eq = t2450[1].clone();
    if (Py::Bool(true)).truthy() {
        return Ok(py::add(&py::truediv(&py::abs(&py::sub(&v_eq.clone(), &py::fsum(&v_vals.clone())?)?)?, &py::fsum(&v_vals.clone())?)?, &py::abs(&py::sub(&py::method(&v_outputs.clone(), "get", &[Py::str("count"), py::len(&v_vals.clone())?])?, &py::len(&v_vals.clone())?)?)?)?);
    }
    return Ok(py::add(&py::abs(&py::sub(&py::fsum(&{ let mut out = vec![]; for t2451 in py::iter(&(v_vals.clone()))? { let mut v_v: Py; 
v_v = t2451;
out.push(py::truediv(&v_eq.clone(), &v_v.clone())?); } Py::list(out) })?, &Py::Float(1.0))?)?, &py::abs(&py::sub(&py::method(&v_outputs.clone(), "get", &[Py::str("count"), py::len(&v_vals.clone())?])?, &py::len(&v_vals.clone())?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2452(mut v_kw: Py) -> R {
    let mut v_out: Py = Py::None;
    let mut v_total: Py = Py::None;
    let mut v_vals: Py = Py::None;
    v_vals = py::getitem(&v_kw.clone(), &Py::str("R"))?;
    let _ = f__positive_values(v_vals.clone(), Py::str("resistors"))?;
    v_total = (if (Py::Bool(true)).truthy() { py::sum(&v_vals.clone(), None)? } else { py::truediv(&Py::Float(1.0), &py::sum(&{ let mut out = vec![]; for t2459 in py::iter(&(v_vals.clone()))? { let mut v_v: Py; 
v_v = t2459;
out.push(py::truediv(&Py::Float(1.0), &v_v.clone())?); } Py::list(out) }, None)?)? });
    v_out = Py::dict(vec![(Py::str("equivalent"), v_total.clone()), (Py::str("count"), py::float(&py::len(&v_vals.clone())?)?)]);
    if ({ let t2460 = Py::str("resistance"); Py::Bool({ let t2461 = Py::str("resistance"); if py::ne(&t2460, &t2461) { true } else { false } }) }).truthy() {
        py::setitem(&v_out.clone(), &Py::str("unit"), { let t2462 = py::method(&v_kw.clone(), "get", &[Py::str("unit")])?; if t2462.truthy() { t2462 } else { Py::str("the unit of the inputs") } })?;
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2463(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_eq: Py = Py::None;
    let mut v_vals: Py = Py::None;
    let t2464 = (Py::tuple(vec![py::getitem(&v_inputs.clone(), &Py::str("R"))?, py::getitem(&v_outputs.clone(), &Py::str("equivalent"))?])).items()?;
    if t2464.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_vals = t2464[0].clone();
    v_eq = t2464[1].clone();
    if (Py::Bool(true)).truthy() {
        return Ok(py::add(&py::truediv(&py::abs(&py::sub(&v_eq.clone(), &py::fsum(&v_vals.clone())?)?)?, &py::fsum(&v_vals.clone())?)?, &py::abs(&py::sub(&py::method(&v_outputs.clone(), "get", &[Py::str("count"), py::len(&v_vals.clone())?])?, &py::len(&v_vals.clone())?)?)?)?);
    }
    return Ok(py::add(&py::abs(&py::sub(&py::fsum(&{ let mut out = vec![]; for t2465 in py::iter(&(v_vals.clone()))? { let mut v_v: Py; 
v_v = t2465;
out.push(py::truediv(&v_eq.clone(), &v_v.clone())?); } Py::list(out) })?, &Py::Float(1.0))?)?, &py::abs(&py::sub(&py::method(&v_outputs.clone(), "get", &[Py::str("count"), py::len(&v_vals.clone())?])?, &py::len(&v_vals.clone())?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2476(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_C: Py = Py::None;
    let mut v_L: Py = Py::None;
    let mut v_R: Py = Py::None;
    let mut v_Z: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_res: Py = Py::None;
    let mut v_w: Py = Py::None;
    let t2477 = (Py::tuple(vec![py::getitem(&v_inputs.clone(), &Py::str("R"))?, py::getitem(&v_inputs.clone(), &Py::str("L"))?, py::getitem(&v_inputs.clone(), &Py::str("C"))?, py::getitem(&v_inputs.clone(), &Py::str("f"))?])).items()?;
    if t2477.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_R = t2477[0].clone();
    v_L = t2477[1].clone();
    v_C = t2477[2].clone();
    v_f = t2477[3].clone();
    v_w = py::mul(&py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?, &v_f.clone())?;
    v_Z = py::complex(&[py::getitem(&v_outputs.clone(), &Py::str("real"))?, py::getitem(&v_outputs.clone(), &Py::str("imag"))?])?;
    v_res = (if (Py::Bool(true)).truthy() { py::truediv(&py::abs(&py::sub(&v_Z.clone(), &py::add(&py::add(&v_R.clone(), &py::mul(&py::mul(&Py::Complex(0.0, 1.0), &v_w.clone())?, &v_L.clone())?)?, &py::truediv(&Py::Int(1i64), &py::mul(&py::mul(&Py::Complex(0.0, 1.0), &v_w.clone())?, &v_C.clone())?)?)?)?)?, &py::abs(&v_Z.clone())?)? } else { py::abs(&py::sub(&py::mul(&v_Z.clone(), &py::add(&py::add(&py::truediv(&Py::Int(1i64), &v_R.clone())?, &py::truediv(&Py::Int(1i64), &py::mul(&py::mul(&Py::Complex(0.0, 1.0), &v_w.clone())?, &v_L.clone())?)?)?, &py::mul(&py::mul(&Py::Complex(0.0, 1.0), &v_w.clone())?, &v_C.clone())?)?)?, &Py::Int(1i64))?)? });
    return Ok(py::add(&py::add(&py::add(&py::add(&v_res.clone(), &f__rel(py::getitem(&v_outputs.clone(), &Py::str("impedance"))?, py::abs(&v_Z.clone())?)?)?, &py::truediv(&py::abs(&py::sub(&py::getitem(&v_outputs.clone(), &Py::str("phase_deg"))?, &py::math1("degrees", &py::cmath("phase", &[v_Z.clone()])?)?)?)?, &Py::Int(180i64))?)?, &f__rel(py::getitem(&v_outputs.clone(), &Py::str("XL"))?, py::mul(&v_w.clone(), &v_L.clone())?)?)?, &f__rel(py::getitem(&v_outputs.clone(), &Py::str("XC"))?, py::truediv(&Py::Int(1i64), &py::mul(&v_w.clone(), &v_C.clone())?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2478(mut v_R: Py, mut v_L: Py, mut v_C: Py, mut v_s: Py) -> R {
    return Ok(f__resonance(v_R.clone(), v_L.clone(), v_C.clone(), v_s.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2485(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_C: Py = Py::None;
    let mut v_L: Py = Py::None;
    let mut v_R: Py = Py::None;
    let mut v_Y: Py = Py::None;
    let mut v_peak: Py = Py::None;
    let t2486 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("R"))?, py::getitem(&v_i.clone(), &Py::str("L"))?, py::getitem(&v_i.clone(), &Py::str("C"))?])).items()?;
    if t2486.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_R = t2486[0].clone();
    v_L = t2486[1].clone();
    v_C = t2486[2].clone();
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
fn s_2958(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::add(&py::add(&py::add(&py::truediv(&py::abs(&py::sub(&py::mul(&py::getitem(&v_o.clone(), &Py::str("Vs"))?, &py::getitem(&v_i.clone(), &Py::str("Np"))?)?, &py::mul(&py::getitem(&v_i.clone(), &Py::str("Vp"))?, &py::getitem(&v_i.clone(), &Py::str("Ns"))?)?)?)?, &py::minmax(&[py::abs(&py::mul(&py::getitem(&v_i.clone(), &Py::str("Vp"))?, &py::getitem(&v_i.clone(), &Py::str("Ns"))?)?)?, Py::Float(1e-300)], true)?)?, &py::truediv(&py::abs(&py::sub(&py::mul(&py::getitem(&v_o.clone(), &Py::str("Is"))?, &py::getitem(&v_i.clone(), &Py::str("RL"))?)?, &py::getitem(&v_o.clone(), &Py::str("Vs"))?)?)?, &py::minmax(&[py::abs(&py::getitem(&v_o.clone(), &Py::str("Vs"))?)?, Py::Float(1e-300)], true)?)?)?, &py::truediv(&py::abs(&py::sub(&py::mul(&py::getitem(&v_i.clone(), &Py::str("Vp"))?, &py::getitem(&v_o.clone(), &Py::str("Ip"))?)?, &py::mul(&py::getitem(&v_o.clone(), &Py::str("Vs"))?, &py::getitem(&v_o.clone(), &Py::str("Is"))?)?)?)?, &py::minmax(&[py::abs(&py::mul(&py::getitem(&v_o.clone(), &Py::str("Vs"))?, &py::getitem(&v_o.clone(), &Py::str("Is"))?)?)?, Py::Float(1e-300)], true)?)?)?, &f__rel(py::mul(&py::getitem(&v_o.clone(), &Py::str("Rin"))?, &py::getitem(&v_o.clone(), &Py::str("Ip"))?)?, py::getitem(&v_i.clone(), &Py::str("Vp"))?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3111(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::truediv(&py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("value"))?, &py::getitem(&f_twos_complement_decode(py::getitem(&v_i.clone(), &Py::str("pattern"))?)?, &Py::str("value"))?)?)?, &py::minmax(&[Py::Float(1.0), py::abs(&py::getitem(&v_o.clone(), &Py::str("value"))?)?], true)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3134(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::add(&py::truediv(&py::abs(&py::sub(&py::add(&py::mul(&py::getitem(&v_o.clone(), &Py::str("I"))?, &py::getitem(&v_i.clone(), &Py::str("R1"))?)?, &py::getitem(&v_o.clone(), &Py::str("Vout"))?)?, &py::getitem(&v_i.clone(), &Py::str("Vin"))?)?)?, &py::minmax(&[py::abs(&py::getitem(&v_i.clone(), &Py::str("Vin"))?)?, Py::Float(1e-300)], true)?)?, &py::truediv(&py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("Vout"))?, &py::mul(&py::getitem(&v_o.clone(), &Py::str("I"))?, &py::getitem(&v_i.clone(), &Py::str("R2"))?)?)?)?, &py::minmax(&[py::abs(&py::getitem(&v_i.clone(), &Py::str("Vin"))?)?, Py::Float(1e-300)], true)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3150(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::add(&py::truediv(&py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("Vout"))?, &py::mul(&py::getitem(&v_i.clone(), &Py::str("Vs"))?, &py::sub(&py::truediv(&py::getitem(&v_i.clone(), &Py::str("R2"))?, &py::add(&py::getitem(&v_i.clone(), &Py::str("R1"))?, &py::getitem(&v_i.clone(), &Py::str("R2"))?)?)?, &py::truediv(&py::getitem(&v_i.clone(), &Py::str("R4"))?, &py::add(&py::getitem(&v_i.clone(), &Py::str("R3"))?, &py::getitem(&v_i.clone(), &Py::str("R4"))?)?)?)?)?)?)?, &py::minmax(&[py::abs(&py::getitem(&v_i.clone(), &Py::str("Vs"))?)?, Py::Float(1e-300)], true)?)?, &f__rel(py::mul(&py::getitem(&v_o.clone(), &Py::str("R4_balance"))?, &py::getitem(&v_i.clone(), &Py::str("R1"))?)?, py::mul(&py::getitem(&v_i.clone(), &Py::str("R2"))?, &py::getitem(&v_i.clone(), &Py::str("R3"))?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3156(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__verify_dw(v_i.clone(), v_o.clone(), Py::Bool(false))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_637(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::add(&py::add(&py::add(&py::float(&py::not(&{ let t645 = { let t638 = py::mul(&py::getitem(&v_o.clone(), &Py::str("code"))?, &py::getitem(&v_o.clone(), &Py::str("lsb"))?)?; Py::Bool({ let t639 = py::mul(&py::getitem(&v_i.clone(), &Py::str("Vin"))?, &py::add(&Py::Int(1i64), &Py::Float(1e-12))?)?; if py::le(&t638, &t639)? { true } else { false } }) }; if !t645.truthy() { t645 } else { { let t644 = { let t640 = py::getitem(&v_i.clone(), &Py::str("Vin"))?; Py::Bool({ let t641 = py::mul(&py::add(&py::getitem(&v_o.clone(), &Py::str("code"))?, &Py::Int(1i64))?, &py::getitem(&v_o.clone(), &Py::str("lsb"))?)?; if py::lt(&t640, &t641)? { true } else { false } }) }; if t644.truthy() { t644 } else { { let t642 = py::getitem(&v_o.clone(), &Py::str("code"))?; Py::Bool({ let t643 = py::sub(&py::pow(&Py::Int(2i64), &py::int(&py::getitem(&v_i.clone(), &Py::str("bits"))?)?)?, &Py::Int(1i64))?; if py::eq(&t642, &t643) { true } else { false } }) } } } } }))?, &f__rel(py::mul(&py::getitem(&v_o.clone(), &Py::str("lsb"))?, &py::pow(&Py::Int(2i64), &py::int(&py::getitem(&v_i.clone(), &Py::str("bits"))?)?)?)?, py::getitem(&v_i.clone(), &Py::str("Vref"))?)?)?, &py::float(&{ let t646 = py::int_base(&py::getitem(&v_o.clone(), &Py::str("binary"))?, &Py::Int(2i64))?; Py::Bool({ let t647 = py::getitem(&v_o.clone(), &Py::str("code"))?; if py::ne(&t646, &t647) { true } else { false } }) })?)?, &f__rel(py::getitem(&v_o.clone(), &Py::str("lsb_mV"))?, py::mul(&py::getitem(&v_o.clone(), &Py::str("lsb"))?, &Py::Float(1000.0))?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_827(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("value"))?, &py::getitem(&f_boolean_evaluate(py::getitem(&v_i.clone(), &Py::str("expression"))?, py::getitem(&v_i.clone(), &Py::str("values"))?)?, &Py::str("value"))?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}
