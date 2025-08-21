use noir::{
    barretenberg::{
        prove::prove_ultra_honk_keccak,
        srs::{setup_srs, setup_srs_from_bytecode},
        verify::{get_ultra_honk_keccak_verification_key, verify_ultra_honk_keccak},
    },
    native_types::WitnessMap,
    witness::from_vec_str_to_witness_map,
    FieldElement,
};

// Flutter Rust Bridge imports
pub mod frb_generated;

#[flutter_rust_bridge::frb]
pub async fn noir_setup_srs(
    circuit_bytecode: String,
    srs_path: Option<String>,
    recursion: bool,
) -> Result<u32, String> {
    let circuit_size = smol::unblock(move || {
        let path_ref = srs_path.as_ref().map(|s| s.as_str());
        setup_srs_from_bytecode(&circuit_bytecode, path_ref, recursion)
    })
    .await
    .map_err(|e| format!("SRS setup failed: {}", e))?;

    println!("✔ SRS setup complete, circuit size: {}", circuit_size);
    Ok(circuit_size)
}

#[flutter_rust_bridge::frb]
pub async fn noir_setup_srs_with_size(
    circuit_size: u32,
    srs_path: Option<String>,
) -> Result<(), String> {
    smol::unblock(move || {
        let path_ref = srs_path.as_ref().map(|s| s.as_str());
        setup_srs(circuit_size, path_ref)
    })
    .await
    .map_err(|e| format!("SRS setup with size failed: {}", e))?;

    println!("✔ SRS with size {} setup complete", circuit_size);
    Ok(())
}

#[flutter_rust_bridge::frb]
pub async fn noir_get_verification_key(
    circuit_bytecode: String,
    disable_zk: bool,
    low_memory_mode: bool,
) -> Result<Vec<u8>, String> {
    smol::unblock(move || {
        get_ultra_honk_keccak_verification_key(&circuit_bytecode, disable_zk, low_memory_mode)
    })
    .await
    .map_err(|e| format!("Getting verification key failed: {}", e))
}

#[flutter_rust_bridge::frb]
pub async fn noir_witness_from_strings(witness_strings: Vec<String>) -> Result<Vec<u8>, String> {
    let witness_refs: Vec<&str> = witness_strings.iter().map(|s| s.as_str()).collect();
    let witness_map = from_vec_str_to_witness_map(witness_refs)
        .map_err(|e| format!("Failed to convert witness: {}", e))?;

    bincode::serialize(&witness_map).map_err(|e| format!("Failed to serialize witness: {}", e))
}

#[flutter_rust_bridge::frb]
pub async fn noir_prove(
    circuit_bytecode: String,
    initial_witness_bytes: Vec<u8>,
    verification_key: Vec<u8>,
    disable_zk: bool,
    low_memory_mode: bool,
) -> Result<Vec<u8>, String> {
    // Convert bytes back to WitnessMap
    let initial_witness: WitnessMap<FieldElement> = bincode::deserialize(&initial_witness_bytes)
        .map_err(|e| format!("Failed to deserialize witness: {}", e))?;

    // Use smol::unblock to run blocking operation in thread pool
    let proof = smol::unblock(move || {
        prove_ultra_honk_keccak(
            &circuit_bytecode,
            initial_witness,
            verification_key,
            disable_zk,
            low_memory_mode,
        )
    })
    .await
    .map_err(|e| format!("Proving failed: {}", e))?;

    Ok(proof)
}

#[flutter_rust_bridge::frb]
pub async fn noir_verify_proof(
    proof: Vec<u8>,
    verification_key: Vec<u8>,
    disable_zk: bool,
) -> Result<bool, String> {
    smol::unblock(move || verify_ultra_honk_keccak(proof, verification_key, disable_zk))
        .await
        .map_err(|e| format!("Proof verification failed: {}", e))
}

#[flutter_rust_bridge::frb(init)]
pub fn init_app() {
    flutter_rust_bridge::setup_default_user_utils();
}
