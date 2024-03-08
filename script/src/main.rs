//! A simple script to generate and verify the proof of a given program.

use lambdaworks_math::field::fields::fft_friendly::stark_252_prime_field::Stark252PrimeField;
use platinum_prover::air::PublicInputs;
use sp1_core::{SP1Prover, SP1Stdin};
use stark_platinum_prover::proof::stark::StarkProof;

const ELF: &[u8] = include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf");

fn main() {
    let proof_path = "../platinum_proofs/fibonacci.proof";

    let Ok(program_content) = std::fs::read(proof_path) else {
        eprintln!("Error opening {} file", proof_path);
        return;
    };
    let mut bytes = program_content.as_slice();
    if bytes.len() < 8 {
        eprintln!("Error reading proof from file: {}", proof_path);
        return;
    }

    // Proof len was stored as an u32, 4u8 needs to be read
    let proof_len = u32::from_le_bytes(bytes[0..4].try_into().unwrap()) as usize;

    bytes = &bytes[4..];
    if bytes.len() < proof_len {
        eprintln!("Error reading proof from file: {}", proof_path);
        return;
    }

    let Ok((proof, _)) = bincode::serde::decode_from_slice::<
        StarkProof<Stark252PrimeField, Stark252PrimeField>,
        _,
    >(&bytes[0..proof_len], bincode::config::standard()) else {
        println!("Error reading proof from file: {}", proof_path);
        return;
    };
    bytes = &bytes[proof_len..];

    let Ok((pub_inputs, _)) =
        bincode::serde::decode_from_slice::<PublicInputs, _>(bytes, bincode::config::standard())
    else {
        println!("Error reading proof from file: {}", proof_path);
        return;
    };

    // Generate proof.
    let mut stdin = SP1Stdin::new();
    stdin.write(&proof);
    stdin.write(&pub_inputs);
    let proof = SP1Prover::prove(ELF, stdin).expect("proving failed");

    proof
        .save("proof-with-io.json")
        .expect("saving proof failed");

    println!("succesfully generated proof for the program!")
}
