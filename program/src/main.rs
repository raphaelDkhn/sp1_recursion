#![no_main]
sp1_zkvm::entrypoint!(main);

mod platinum_verifier;

use lambdaworks_math::field::fields::fft_friendly::stark_252_prime_field::Stark252PrimeField;
use platinum_prover::air::PublicInputs;
use platinum_verifier::verify_platinum_proof;
use stark_platinum_prover::proof::stark::StarkProof;

pub fn main() {
    println!("Start program...");
    // Read the Platinum proof and public inputs from the zkVM input
    let proof = sp1_zkvm::io::read::<StarkProof<Stark252PrimeField, Stark252PrimeField>>();
    println!("Read proof successfully!");
    let pub_inputs = sp1_zkvm::io::read::<PublicInputs>();
    println!("Read pub_inputs successfully!");

    // Verify the Platinum proof
    let verification_result = verify_platinum_proof(proof, pub_inputs);

    // Write the verification result to the zkVM output
    sp1_zkvm::io::write(&verification_result);
}
