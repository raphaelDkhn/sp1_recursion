use lambdaworks_math::field::fields::fft_friendly::stark_252_prime_field::Stark252PrimeField;
use platinum_prover::air::{verify_cairo_proof, PublicInputs};
use stark_platinum_prover::proof::options::{ProofOptions, SecurityLevel};
use stark_platinum_prover::proof::stark::StarkProof;

pub(crate) fn verify_platinum_proof(
    proof: StarkProof<Stark252PrimeField, Stark252PrimeField>,
    pub_inputs: PublicInputs,
) -> bool {
    
    let proof_options = ProofOptions::new_secure(SecurityLevel::Conjecturable100Bits, 3);

    println!("Verifying ...");
    let proof_verified = verify_cairo_proof(&proof, &pub_inputs, &proof_options);
    println!("Verification value: {}", proof_verified);

    proof_verified
}
