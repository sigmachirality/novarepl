use std::collections::HashMap;

use ff::PrimeField;
use nova_scotia::FileLocation;
use nova_scotia::{
    circom::{circuit::CircomCircuit, reader::load_r1cs},
    create_public_params, create_recursive_circuit, EE1, EE2, F1, F2, G1, G2, S1, S2,
};
use nova_snark::{
    spartan::RelaxedR1CSSNARK,
    traits::{circuit::TrivialTestCircuit, Group},
    CompressedSNARK, PublicParams,
};
use serde::{Serialize, Deserialize};
use serde_json::Value;
use wasm_bindgen::prelude::*;

pub use wasm_bindgen_rayon::init_thread_pool;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

extern crate console_error_panic_hook;

#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

#[derive(Serialize, Deserialize)]
pub struct ProofInput {
    pub step_in: Vec<String>,
    pub witness: Vec<HashMap<String, Value>>
}

#[derive(Serialize, Deserialize)]
pub struct VerifyInput {
    pub step_in: Vec<String>
}

#[wasm_bindgen]
pub async fn generate_params(r1cs_url: String) -> String {
    let r1cs = load_r1cs(&FileLocation::URL(r1cs_url.clone())).await;
    let pp = create_public_params(r1cs.clone());
    let serialised = serde_json::to_string(&pp).unwrap();
    return serialised;
}

#[wasm_bindgen]
pub async fn generate_proof(
    pp_str: String,
    inputs: JsValue,
    r1cs_url: String,
    wasm_url: String
) -> String {
    let r1cs = load_r1cs(&FileLocation::URL(r1cs_url.clone())).await;
    let witness_generator_wasm = FileLocation::URL(wasm_url.clone());

    let inputs: ProofInput = serde_wasm_bindgen::from_value(inputs).unwrap();

    let pp =
        serde_json::from_str::<PublicParams<G1, G2, CircomCircuit<F1>, TrivialTestCircuit<F2>>>(
            &pp_str,
        )
        .unwrap();

    console_log!(
        "Number of constraints per step (primary circuit): {}",
        pp.num_constraints().0
    );
    console_log!(
        "Number of constraints per step (secondary circuit): {}",
        pp.num_constraints().1
    );

    console_log!(
        "Number of variables per step (primary circuit): {}",
        pp.num_variables().0
    );
    console_log!(
        "Number of variables per step (secondary circuit): {}",
        pp.num_variables().1
    );

    let start_public_inputs: Vec<_> = inputs.step_in.iter().map(|x| F1::from_str_vartime(x).unwrap()).collect();

    console_log!("Creating a RecursiveSNARK...");
    let recursive_snark = create_recursive_circuit(
        witness_generator_wasm,
        r1cs,
        inputs.witness.clone(),
        start_public_inputs.clone(),
        &pp,
    )
    .await
    .unwrap();

    // TODO: empty?
    let z0_secondary = vec![<G2 as Group>::Scalar::zero()];

    // verify the recursive SNARK
    console_log!("Verifying a RecursiveSNARK...");
    let res = recursive_snark.verify(
        &pp,
        inputs.witness.len(),
        start_public_inputs.clone(),
        z0_secondary.clone(),
    );
    assert!(res.is_ok());

    // produce a compressed SNARK
    console_log!("Generating a CompressedSNARK using Spartan with IPA-PC...");
    let (pk, _vk) = CompressedSNARK::<_, _, _, _, S1, S2>::setup(&pp).unwrap();
    let res = CompressedSNARK::<_, _, _, _, S1, S2>::prove(&pp, &pk, &recursive_snark);
    assert!(res.is_ok());
    let compressed_snark = res.unwrap();
    return serde_json::to_string(&compressed_snark).unwrap();
}

#[wasm_bindgen]
pub async fn verify_compressed_proof(
    pp_str: String,
    inputs: JsValue,
    proof_str: String
) -> bool {
    let inputs: VerifyInput = serde_wasm_bindgen::from_value(inputs).unwrap();

    let pp =
        serde_json::from_str::<PublicParams<G1, G2, CircomCircuit<F1>, TrivialTestCircuit<F2>>>(
            &pp_str,
        )
        .unwrap();
    let (_pk, vk) = CompressedSNARK::<_, _, _, _, S1, S2>::setup(&pp).unwrap();
    let iteration_count = inputs.step_in.len();
    let z0_secondary = vec![<G2 as Group>::Scalar::zero()];

    let compressed_proof = serde_json::from_str::<
        CompressedSNARK<
            G1,
            G2,
            CircomCircuit<F1>,
            TrivialTestCircuit<F2>,
            RelaxedR1CSSNARK<G1, EE1>,
            RelaxedR1CSSNARK<G2, EE2>,
        >,
    >(&proof_str)
    .unwrap();

    let start_public_inputs: Vec<_> = inputs.step_in.iter().map(|x| F1::from_str_vartime(x).unwrap()).collect();

    let res = compressed_proof.verify(
        &vk,
        iteration_count,
        start_public_inputs,
        z0_secondary,
    );
    return res.is_ok();
}
