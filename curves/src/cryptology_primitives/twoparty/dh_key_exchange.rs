//! in ec-Diffie-Hellman key exchange, Alice choose a random secret "a" and sends Bob public key A=a*G
//! Bob choose a random secret "b" and sends public key B=b*G to Alice
//! Both parties can compute a joint secret: C= aB = bA = ab*G
//! which cannot be computed by a man in the middle attacker.

use curv::elliptic::curves::{Point, Curve, Scalar};


#[derive(Debug, Clone)]
pub struct EcKeyPair<E: Curve> {
    pub pubic_share: Point<E>,
    secret_share: Scalar<E>,
}

#[derive(Debug)]
pub struct Party1FirstMessage<E: Curve> {
    pub public_share: Point<E>,
}

#[derive(Debug)]
pub struct Party2FirstMessage<E: Curve> {
    pub public_share: Point<E>,
}


impl<E: Curve> Party1FirstMessage<E> {
    pub fn first() -> (Party1FirstMessage<E>, EcKeyPair<E>) {
        let generator = Point::<E>::generator();

        let secret_share = Scalar::<E>::random();

        let public_share = &secret_share * generator;

        let ec_key_pair = EcKeyPair {
            pubic_share: public_share.clone(),
            secret_share,
        };

        (Party1FirstMessage { public_share }, ec_key_pair)
    }

    pub fn first_with_secret_share(secret_share: Scalar<E>) -> (Party1FirstMessage<E>, EcKeyPair<E>) {
        let public_share = Point::<E>::generator() * &secret_share;

        let ec_key_pair = EcKeyPair {
            pubic_share: public_share.clone(),
            secret_share,
        };
        (Party1FirstMessage { public_share }, ec_key_pair)
    }
}

impl<E: Curve> Party2FirstMessage<E> {
    pub fn first() -> (Party2FirstMessage<E>, EcKeyPair<E>) {
        let generator = Point::<E>::generator();

        let secret_share = Scalar::<E>::random();

        let public_share = &secret_share * generator;

        let ec_key_pair = EcKeyPair {
            pubic_share: public_share.clone(),
            secret_share,
        };

        (Party2FirstMessage { public_share }, ec_key_pair)
    }

    pub fn first_with_secret_share(secret_share: Scalar<E>) -> (Party2FirstMessage<E>, EcKeyPair<E>) {
        let public_share = Point::<E>::generator() * &secret_share;

        let ec_key_pair = EcKeyPair {
            pubic_share: public_share.clone(),
            secret_share,
        };
        (Party2FirstMessage { public_share }, ec_key_pair)
    }
}

pub fn compute_pubkey<E: Curve>(local_share: &EcKeyPair<E>, peer_public_share: &Point<E>) -> Point<E> {
    &local_share.secret_share * peer_public_share
}

#[cfg(test)]
mod tests {
    use curv::elliptic::curves::{Point, Secp256k1};
    use curv::elliptic::curves::{Scalar};
    use crate::cryptology_primitives::twoparty::dh_key_exchange::{compute_pubkey, Party1FirstMessage, Party2FirstMessage};

    #[test]
    fn test_dh_key_exchange_random_secret() {
        let (one_first_msg, one_key_pair) = Party1FirstMessage::<Secp256k1>::first();
        let (two_first_msg, two_key_pair) = Party2FirstMessage::<Secp256k1>::first();

        assert_eq!(
            compute_pubkey(&one_key_pair, &two_first_msg.public_share),
            compute_pubkey(&two_key_pair, &one_first_msg.public_share)
        )
    }

    #[test]
    fn test_dh_key_exchange_with_secret() {
        let one_secret = Scalar::from(1i32);
        let (one_first_msg, one_key_pair) = Party1FirstMessage::<Secp256k1>::first_with_secret_share(one_secret);

        let two_secret = Scalar::from(2i32);
        let (two_first_msg, two_key_pair) = Party2FirstMessage::<Secp256k1>::first_with_secret_share(two_secret.clone());

        assert_eq!(
            compute_pubkey(&one_key_pair, &two_first_msg.public_share),
            compute_pubkey(&two_key_pair, &one_first_msg.public_share)
        );


        assert_eq!(
            compute_pubkey(&two_key_pair, &one_first_msg.public_share),
            Point::generator() * two_secret
        )
    }

    #[test]
    fn test_public_key_serialize() {
        let generator = Point::<Secp256k1>::generator();
        let secret = Scalar::<Secp256k1>::from(1);
        let pk = secret * generator;
        let compressed_pk = &pk.to_bytes(true);
        println!("{:?}", compressed_pk.as_ref());
        println!("{}", compressed_pk.as_ref().len());
        let uncompressed_pk = pk.to_bytes(false);
        println!("{:?}", uncompressed_pk.as_ref());
        println!("{}", uncompressed_pk.as_ref().len());
    }
}