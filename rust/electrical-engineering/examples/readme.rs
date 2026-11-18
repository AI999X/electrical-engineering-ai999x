//! Electrical Engineering: 15 calculations through the Rust library (the same engine as the desktop app, the
//! command line and the browser). Run from the rust/ folder:
//!     cargo run --release --example readme -p electrical-engineering-ai999x
//! Inputs are text, read like a form field ("2 m", "0.5", "x^3 - 3x"); a JSON list stays a list.

use serde_json::{json, Value};

/// One calculation: the request in, its outcome and every value with its unit out.
fn show(request: Value) {
    let answer: Value = serde_json::from_str(&electrical_engineering_ai999x::solve_json(&request.to_string())).unwrap();
    println!("{}", answer["outcome"].as_str().unwrap_or(""));
    for v in answer["values"].as_array().into_iter().flatten() {
        let unit = v[2].as_str().unwrap_or("");
        let unit = if unit.is_empty() || unit == "1" { String::new() } else { format!(" {unit}") };
        println!("{} = {}{}", v[0].as_str().unwrap_or(""), v[1], unit);
    }
}

fn main() {
    show(json!({"key": "ac_mesh_currents", "inputs": {"R": [[10, -4], [-4, 8]], "X": [[5, -2], [-2, -3]], "V_mag": [10, 0], "V_deg": [0, 0]}}));
    println!();
    show(json!({"key": "sinusoid_values", "inputs": {"Vm": "200 V", "w": "314.159 rad/s", "phi": "0", "t": "0.0025 s"}}));
    println!();
    show(json!({"key": "cell_battery", "inputs": {"E": "1.5 V", "r": "0.5 ohm", "n": "4", "RL": "10 ohm", "connection": "series"}}));
    println!();
    show(json!({"key": "parallel_plate_capacitor", "inputs": {"V": "200 V", "d": "0.0001 m", "A": "0.0004 m^2", "eps_r": "5"}}));
    println!();
    show(json!({"key": "closed_loop_poles", "inputs": {"num": [2], "den": [1, 3, 0]}}));
    println!();
    show(json!({"key": "dc_shunt_generator", "inputs": {"V": "250 V", "IL": "100 A", "Ra": "0.1 ohm", "Rf": "125 ohm"}}));
    println!();
    show(json!({"key": "current_divider", "inputs": {"I": "6 A", "R1": "4 ohm", "R2": "2 ohm"}}));
    println!();
    show(json!({"key": "adc_conversion", "inputs": {"Vin": "1.2 V", "Vref": "5 V", "bits": "8"}}));
    println!();
    show(json!({"key": "bjt_divider_bias", "inputs": {"VCC": "12 V", "VBE": "0.7 V", "R1": "10000 ohm", "R2": "2200 ohm", "RC": "3600 ohm", "RE": "1000 ohm", "beta": "100"}}));
    println!();
    show(json!({"key": "faraday_emf", "inputs": {"N": "200", "dPhi": "0.025 Wb", "dt": "0.05 s"}}));
    println!();
    show(json!({"key": "plane_wave_lossy", "inputs": {"f": "1000000 Hz", "eps_r": "1", "mu_r": "1", "sigma": "58000000"}}));
    println!();
    show(json!({"key": "magnetic_circuit", "inputs": {"N": "500", "flux": "0.0006 Wb", "lengths": [0.4, 0.001], "areas": [0.0005, 0.0005], "mu_r": [800, 1]}}));
    println!();
    show(json!({"key": "ammeter_shunt", "inputs": {"Ifs": "0.001 A", "Rm": "100 ohm", "I": "1 A"}}));
    println!();
    show(json!({"key": "difference_amplifier", "inputs": {"R1": "10000 ohm", "R2": "100000 ohm", "R3": "10000 ohm", "R4": "100000 ohm", "V1": "1 V", "V2": "1.5 V"}}));
    println!();
    show(json!({"key": "coil_capacitor_parallel", "inputs": {"R": "10 ohm", "L": "0.1 H", "C": "1e-05 F", "V": "100 V", "f": "50 Hz"}}));
}
