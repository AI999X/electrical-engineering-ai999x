//! The question reader on Electrical Engineering's documented questions (ask.rs): a documented question the package
//! refuses for its values (Vin above Vref, a singular matrix, a zero resistor) is read as its capability and refused
//! by the engine with the documented outcome and the engine's reason, never answered and never left AMBIGUOUS when
//! the reading is clear; and no documented refused question is ever answered with a number.

use electrical_engineering_ai999x::pack;
use serde_json::Value;

fn ask(q: &str) -> Value {
    serde_json::from_str(&ax_engine::ask::ask_json(pack(), q)).unwrap()
}

#[test]
fn a_documented_refusal_is_refused_with_its_outcome_and_reason() {
    let a = ask("Find the ADC output code for Vin = 6 V, Vref = 5 V, bits = 8.");
    assert_eq!(a["outcome"], "INVALID_INPUT", "{a}");
    assert_eq!(a["capability"], "adc_conversion");
    assert!(a["message"].as_str().unwrap().contains("between 0 and Vref"), "{a}");
    let ok = ask("Find the ADC output code for Vin = 1.2 V, Vref = 5 V, bits = 8.");
    assert_eq!(ok["outcome"], "SOLVED", "{ok}");
}

#[test]
fn no_documented_refused_question_is_answered() {
    let doc: Value = serde_json::from_str(electrical_engineering_ai999x::PACK_JSON).unwrap();
    let mut n = 0;
    for f in doc["forms"].as_array().unwrap() {
        let want = f["outcome"].as_str().unwrap_or("");
        if f.get("key").is_some() || want == "SOLVED" {
            continue;
        }
        n += 1;
        let a = ask(f["q"].as_str().unwrap());
        assert_ne!(a["outcome"], "SOLVED", "{}: {a}", f["q"]);
        assert!(a["outcome"] == want || a["outcome"] == "AMBIGUOUS", "{}: {a}", f["q"]);
    }
    assert!(n >= 10, "only {n} documented refusals");
}
