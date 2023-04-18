//! in ec-Diffie-Hellman key exchange, Alice choose a random secret "a" and sends Bob public key A=a*G
//! Bob choose a random secret "b" and sends public key B=b*G to Alice
//! Both parties can compute a joint secret: C= aB = bA = ab*G
//! which cannot be computed by a man in the middle attacker.
//!
//! The variant below is to protect ont only for man in the middle but also from malicious
//! Alice or Bob that can bias the result
//! https://eprint.iacr.org/2017/552.pdf protocol 3.1 first 3 steps.
//! <<Fast Secure Two-Party ECDSA Signing!>> Lindell

use curv::arithmetic::{Converter, Samplable};
use curv::BigInt;
use curv::cryptographic_primitives::commitments::hash_commitment::HashCommitment;
use curv::cryptographic_primitives::commitments::traits::Commitment;
use curv::elliptic::curves::{Curve, Point, Scalar};
use curv::cryptographic_primitives::hashing::Digest;
use crate::cryptology_primitives::proofs::sigma_dlog::DLogProof;

const SECURITY_BITS: usize = 256;

pub struct EcKeyPair<E: Curve> {
    pub public_share: Point<E>,
    secret_share: Scalar<E>,
}

pub struct CommWitness<E: Curve, H: Digest + Clone> {
    pub pk_commitment_blind_factor: BigInt,
    pub zk_pok_blind_factor: BigInt,
    pub dlog_proof: DLogProof<E, H>,
}

pub struct Party1FirstMessage {
    pub pk_commitment: BigInt,
    pub zk_pok_commitment: BigInt,
}

pub struct Party2FirstMessage<E: Curve, H: Digest + Clone> {
    pub dlog_proof: DLogProof<E, H>,
}


pub struct Party1SecondMessage<E: Curve, H: Digest + Clone> {
    pub comm_witness: CommWitness<E, H>,
}

pub struct Party2SecondMessage {}


impl Party1FirstMessage {
    pub fn create_commitments_with_secret<E: Curve, H: Digest + Clone>(secret_share: Scalar<E>) -> (Party1FirstMessage, CommWitness<E, H>, EcKeyPair<E>) {
        let generator = Point::<E>::generator();
        let public_share = generator * &secret_share;

        let dlog_proof = DLogProof::prove(&secret_share);

        let pk_commitment_blind_factor = BigInt::sample(SECURITY_BITS);
        let pk_commitment = HashCommitment::<H>::create_commitment_with_user_defined_randomness(
            &BigInt::from_bytes(&dlog_proof.pk.to_bytes(true)),
            &pk_commitment_blind_factor,
        );

        let zk_pok_blind_factor = BigInt::sample(SECURITY_BITS);
        let zk_pok_commitment = HashCommitment::<H>::create_commitment_with_user_defined_randomness(
            &BigInt::from_bytes(&dlog_proof.pk_t_rand_commitment.to_bytes(true)),
            &zk_pok_blind_factor,
        );

        let ec_key_pair = EcKeyPair {
            public_share,
            secret_share,
        };
        (
            Party1FirstMessage { pk_commitment, zk_pok_commitment },
            CommWitness {
                pk_commitment_blind_factor,
                zk_pok_blind_factor,
                dlog_proof,
            },
            ec_key_pair
        )
    }
}

impl<E: Curve, H: Digest + Clone> Party2FirstMessage<E, H> {
    pub fn create_dlog_proof_with_secret(secret_share: Scalar<E>) -> (Party2FirstMessage<E, H>, EcKeyPair<E>) {
        let generator = Point::<E>::generator();
        let public_share = generator * &secret_share;
        let dlog_proof = DLogProof::prove(&secret_share);

        let ec_key_pair = EcKeyPair {
            public_share,
            secret_share,
        };
        (
            Party2FirstMessage {
                dlog_proof,
            },
            ec_key_pair
        )
    }
}

impl<E: Curve, H: Digest + Clone> Party1SecondMessage<E, H> {
    pub fn verify_and_decomit(
        comm_witness: CommWitness<E, H>,
        proof: &DLogProof<E, H>,
    ) -> Result<Party1SecondMessage<E, H>, String> {
        DLogProof::verify(proof)?;
        Ok(Party1SecondMessage { comm_witness })
    }
}

impl Party2SecondMessage {
    pub fn verify_commitments_and_dlog_proof<E: Curve, H: Digest + Clone>(
        party_one_first_message: &Party1FirstMessage,
        party_one_second_message: &Party1SecondMessage<E, H>,
    ) -> Result<Party2SecondMessage, String> {
        let party_one_dlog_proof = &party_one_second_message.comm_witness.dlog_proof;

        let party_one_public_share = &party_one_dlog_proof.pk;
        if party_one_public_share.is_zero() {
            return Err("party one use zero as public share".to_string());
        }

        let party_one_pk_commitment = &party_one_first_message.pk_commitment;
        let party_one_zk_pok_commitment = &party_one_first_message.zk_pok_commitment;


        let mut flag = true;
        if party_one_pk_commitment != &HashCommitment::<H>::create_commitment_with_user_defined_randomness(
            &BigInt::from_bytes(&party_one_public_share.to_bytes(true)),
            &party_one_second_message.comm_witness.pk_commitment_blind_factor,
        ) {
            flag = false
        };

        if party_one_zk_pok_commitment != &HashCommitment::<H>::create_commitment_with_user_defined_randomness(
            &BigInt::from_bytes(&party_one_dlog_proof.pk_t_rand_commitment.to_bytes(true)),
            &party_one_second_message.comm_witness.zk_pok_blind_factor,
        ) {
            flag = false
        };
        assert!(flag);
        DLogProof::verify(party_one_dlog_proof)?;
        Ok(Party2SecondMessage {})
    }
}

pub fn compute_pubkey<E: Curve>(
    local_share: &EcKeyPair<E>,
    peer_public_share: &Point<E>,
) -> Point<E> {
    &local_share.secret_share * peer_public_share
}

#[cfg(test)]
mod tests {
    use curv::elliptic::curves::{Scalar, Secp256k1};
    use crate::cryptology_primitives::twoparty::dh_key_exchange_with_pok::{compute_pubkey, Party1FirstMessage, Party1SecondMessage, Party2FirstMessage, Party2SecondMessage};


    #[test]
    fn test_ecdh_with_pok() {
        let p1_secret = Scalar::<Secp256k1>::random();

        let p2_secret = Scalar::random();

        let (p1_first_msg, comm_witness, p1_key_pair) =
            Party1FirstMessage::create_commitments_with_secret::<Secp256k1, sha3::Keccak256>(p1_secret);
        let (p2_first_msg, p2_key_pair) =
            Party2FirstMessage::<Secp256k1, sha3::Keccak256>::create_dlog_proof_with_secret(p2_secret);

        let p1_second_msg = Party1SecondMessage::verify_and_decomit(comm_witness, &p2_first_msg.dlog_proof)
            .expect("failed to verify and decommit");
        let _p2_second_msg = Party2SecondMessage::verify_commitments_and_dlog_proof(&p1_first_msg, &p1_second_msg)
            .expect("failed to verify commitments and dlog_proof");

        assert_eq!(
            compute_pubkey(&p1_key_pair, &p2_first_msg.dlog_proof.pk),
            compute_pubkey(&p2_key_pair, &p1_second_msg.comm_witness.dlog_proof.pk)
        )
    }
}