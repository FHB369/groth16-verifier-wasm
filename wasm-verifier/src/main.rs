use ark_bls12_381::{Bls12_381, Fr};
use ark_groth16::{ Groth16, prepare_verifying_key };
use ark_relations::{
    lc,
    r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError},
};
use ark_ff::UniformRand;
use rand::thread_rng;
use std::time::Instant;

struct MyCircuit {
    a: Fr,
    b: Fr,
    c: Fr,
}

impl ConstraintSynthesizer<Fr> for MyCircuit {
    fn generate_constraints(self, cs: ConstraintSystemRef<Fr>) -> Result<(), SynthesisError> {
        let a = cs.new_input_variable(|| Ok(self.a))?;
        let b = cs.new_input_variable(|| Ok(self.b))?;
        let c = cs.new_input_variable(|| Ok(self.c))?;

        cs.enforce_constraint(lc!() + a, lc!() + b, lc!() + c)?;

        Ok(())
    }
}

fn main() {
    let mut rng = thread_rng();

    // Generate parameters
    let params: ark_groth16::ProvingKey<Bls12_381> = {
        let c = MyCircuit {
            a: Fr::rand(&mut rng),
            b: Fr::rand(&mut rng),
            c: Fr::rand(&mut rng),
        };
        Groth16::<Bls12_381, ark_groth16::r1cs_to_qap::LibsnarkReduction>::generate_random_parameters_with_reduction(c, &mut rng).unwrap()
    };

    // Prepare the verification key
    let pvk = prepare_verifying_key(&params.vk);

    // Create a proof
    let proof = {
        let c = MyCircuit {
            a: Fr::from(1u32),
            b: Fr::from(2u32),
            c: Fr::from(2u32),
        };
        Groth16::<Bls12_381, ark_groth16::r1cs_to_qap::LibsnarkReduction>::create_random_proof_with_reduction(c, &params, &mut rng).unwrap()
    };

    let public_inputs = [Fr::from(1u32), Fr::from(2u32), Fr::from(2u32)];

    // Verify the proof
    let start = Instant::now();
    let result = Groth16::<Bls12_381, ark_groth16::r1cs_to_qap::LibsnarkReduction>::verify_proof(&pvk, &proof, &public_inputs).unwrap();
    let duration = start.elapsed();

    assert!(result);
    println!("Proof verified successfully!");
    println!("Verification time: {:?}", duration);
}