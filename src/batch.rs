use std::*;

use dusk_bls12_381::BlsScalar;
use dusk_jubjub::{JubJubAffine, JubJubScalar};

use rand_core::OsRng;
use crate::commitment_scheme::*;
use crate::composer::*;
use crate::constraint_system::*;
use crate::error::Error;
use std::time::{Instant, SystemTime};
use std::fs::File;
use std::io::Write;

// Implement a circuit that checks:
// 1) a + b = c where C is a PI
// 2) a <= 2^6
// 3) b <= 2^5
// 4) a * b = d where D is a PI

#[derive(Debug, Default)]
pub struct TestCircuit {
    a: BlsScalar,
    b: BlsScalar,
    c: BlsScalar,
    d: BlsScalar,
    // e: JubJubScalar,
    // f: JubJubAffine,
}

impl TestCircuit {
    fn initiate_circuit() -> Self {
        Self {
            a: BlsScalar::from(2u64),
            b: BlsScalar::from(3u64),
            c: BlsScalar::from(5u64),
            d: BlsScalar::from(6u64),
            // e: JubJubScalar::from(7u64),
            // f: dusk_jubjub::GENERATOR_EXTENDED * &JubJub::from(7u64),
        }
    }
}


impl Circuit for TestCircuit {
    fn circuit<C>(&self, composer: &mut C) -> Result<(), Error>
        where
            C: Composer,
    {
        let a = composer.append_witness(self.a);
        let b = composer.append_witness(self.b);

        let num_add = 429;
        let num_mul = 184;

        // Make first constraint a + b = c addition gate
        for i in 0..num_add {
            let constraint =
                Constraint::new().left(1).right(1).public(-self.c).a(a).b(b);
            composer.append_gate(constraint);
        }

        // Check that a and b are in range
        // composer.component_range(a, 1 << 6);
        // composer.component_range(b, 1 << 5);

        // Make second constraint a * b = d multiplication gate
        for i in 0..num_mul {
            let constraint =
                Constraint::new().mult(1).public(-self.d).a(a).b(b);

            composer.append_gate(constraint);
        }

        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct CircuitExp {
    a: Vec<BlsScalar>,
    t: Vec<BlsScalar>,
    a_sum: BlsScalar
}

// impl Circuit for CircuitExp{
//     fn circuit<C>(&self, composer: &mut C) -> Result<(), Error> where C: Composer {
//         let a = composer.append_witness(&self.a);
//         let t =composer.append_witness(&self.t);
//         let a_sum = composer.append_witness(&self.a_sum);
//
//         //make constraint for 2 ^ a_max = d_xr_max
//         for i in 0..
//         let constraint = Constraint::new().left()
//         Ok(())
//     }
// }

#[test]
pub fn test_example() {
    let label = b"transcript-arguments";
    let pp = PublicParameters::setup(1 << 12, &mut OsRng)
        .expect("failed to setup");

    let (prover, verifier) = Compiler::compile::<TestCircuit>(&pp, label)
        .expect("failed to compile circuit");

    let init_circuit_1 = TestCircuit {
        a: BlsScalar::from(2u64),
        b: BlsScalar::from(3u64),
        c: BlsScalar::from(5u64),
        d: BlsScalar::from(6u64),
    };

    let init_circuit_2 = TestCircuit {
        a: BlsScalar::from(2u64),
        b: BlsScalar::from(3u64),
        c: BlsScalar::from(5u64),
        d: BlsScalar::from(6u64),
        // e: JubJubScalar::from(7u64),
        // f: dusk_jubjub::GENERATOR * &JubJubScalar::from(7u64),
    };

// Generate the proof and its public inputs
    // flag to show the position of the zeros (the position of each prover)
    // num_p is the number of the provers, which is used to determine the length of the evaluation domain
    let sy_time_1 = SystemTime::now();
    let (proof_1, public_inputs_1) = prover
        .prove(&mut OsRng, &init_circuit_1)
        .expect("failed to prove");
    println!("proving time:");
    println!("{:?}", SystemTime::now().duration_since(sy_time_1).unwrap().as_millis());
    /// output the size of the proof
    // println!("{:?}", proof);
    let path = "./src/data/proofsize.txt";
    let mut output = File::create(path).unwrap();
    writeln!(output, "{:?}", proof_1).unwrap();

    let (proof_2, public_inputs_2) = prover
        .prove(&mut OsRng, &init_circuit_2)
        .expect("failed to prove");

// Verify the generated proof
    let sy_time_2 = SystemTime::now();
    verifier
        .verify(&proof_1, &public_inputs_1)
        .expect("failed to verify proof");
    println!("verification time:");
    println!("{:?}", SystemTime::now().duration_since(sy_time_2).unwrap().as_millis());
}

// #[test]
// pub fn exp2() {
//     let mut temp = 2;
//     for i in 0..10 {
//         temp = 2 * temp;
//         let mut ex = 2_u32.pow(temp);
//         println!("{:?}", ex)
//     }
//
// }