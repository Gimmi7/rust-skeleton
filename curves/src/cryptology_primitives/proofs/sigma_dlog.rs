use std::marker::PhantomData;

use curv::elliptic::curves;
use curv::cryptographic_primitives::hashing::{Digest, DigestExt};
use curv::elliptic::curves::Point;

#[derive(Debug)]
pub struct DLogProof<E: curves::Curve, H: Digest + Clone> {
    pub pk: curves::Point<E>,
    pub pk_t_rand_commitment: curves::Point<E>,
    pub challenge_response: curves::Scalar<E>,
    pub hash_choice: PhantomData<fn(H)>,
}

impl<E: curves::Curve, H: Digest + Clone> DLogProof<E, H> {
    pub fn prove(sk: &curves::Scalar<E>) -> DLogProof<E, H> {
        let generator = curves::Point::<E>::generator();

        let sk_t_rand = curves::Scalar::random();
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
    use curv::elliptic::curves;
    use crate::cryptology_primitives::proofs::sigma_dlog::DLogProof;

    #[test]
    fn test_dlog_proof() {
        let witness = curves::Scalar::random();
        let dlog_proof = DLogProof::<curves::Secp256k1, sha3::Keccak256>::prove(&witness);
        println!("dlog_proof={:?}", dlog_proof);
        assert!(DLogProof::verify(&dlog_proof).is_ok());
    }
}