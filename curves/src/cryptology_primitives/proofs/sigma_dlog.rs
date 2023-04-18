use std::marker::PhantomData;

use curv::cryptographic_primitives::hashing::{Digest, DigestExt};
use curv::elliptic::curves::{Curve, Point, Scalar};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DLogProof<E: Curve, H: Digest + Clone> {
    pub pk: Point<E>,
    pub pk_t_rand_commitment: Point<E>,
    pub challenge_response: Scalar<E>,
    pub hash_choice: PhantomData<fn(H)>,
}

impl<E: Curve, H: Digest + Clone> DLogProof<E, H> {
    pub fn prove(sk: &Scalar<E>) -> DLogProof<E, H> {
        let generator = Point::<E>::generator();

        let sk_t_rand = Scalar::random();
        let pk_t_rand_commitment = generator * &sk_t_rand;

        let pk = generator * sk;

        let challenge = H::new()
            .chain_point(&pk_t_rand_commitment)
            .chain_point(&generator.to_point())
            .chain_point(&pk)
            .result_scalar();

        let challenge_response = &sk_t_rand - challenge * sk;
        DLogProof {
            pk,
            pk_t_rand_commitment,
            challenge_response,
            hash_choice: PhantomData,
        }
    }

    pub fn verify(proof: &DLogProof<E, H>) -> Result<(), String> {
        let generator = Point::<E>::generator();

        let challenge = H::new()
            .chain_point(&proof.pk_t_rand_commitment)
            .chain_point(&generator.to_point())
            .chain_point(&proof.pk)
            .result_scalar();

        let pk_verifier = &proof.challenge_response * generator + &challenge * &proof.pk;

        if pk_verifier == proof.pk_t_rand_commitment {
            Ok(())
        } else {
            Err(String::from("ProofError: Error while verifying"))
        }
    }
}


#[cfg(test)]
mod tests {
    use curv::elliptic::curves::{Scalar, Secp256k1};

    use crate::cryptology_primitives::proofs::sigma_dlog::DLogProof;

    #[test]
    fn test_dlog_proof() {
        let witness = Scalar::random();
        let dlog_proof = DLogProof::<Secp256k1, sha3::Keccak256>::prove(&witness);
        let proof_json = serde_json::to_string(&dlog_proof).unwrap();
        println!("{}", proof_json);
        println!("dlog_proof={:?}", dlog_proof);
        assert!(DLogProof::verify(&dlog_proof).is_ok());
    }
}