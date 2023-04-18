use ethers::types::Address;
use ethers::utils::{hex, keccak256};
use secp256k1::rand::rngs::OsRng;
use secp256k1::Secp256k1;

/// generate secp256k1 point, then calc the corresponding eth addr
fn main() {
    // rng: random number generator
    let secp = Secp256k1::new();
    let (_secret_key, public_key) = secp.generate_keypair(&mut OsRng);

    let uncompressed_public_key = public_key.serialize_uncompressed();
    let hashed_public_key = keccak256(&uncompressed_public_key[1..]);

    let address = Address::from_slice(&hashed_public_key[12..]);
    println!("Ethereum address: {}", hex::encode(address));
}

#[cfg(test)]
mod test {
    use std::path::Component::CurDir;
    use ethers::core::k256::elliptic_curve::weierstrass::add;
    use ethers::prelude::k256::ecdsa::SigningKey;
    use ethers::prelude::k256::elliptic_curve::{generic_array, NonZeroScalar, PrimeField};
    use ethers::prelude::k256::Secp256k1;
    use ethers::signers::{LocalWallet, Signer, Wallet};
    use ethers::types::Address;
    use ethers::utils::keccak256;
    use hex;
    use secp256k1::{self, rand, PublicKey, SecretKey};
    use std::str::FromStr;
    use curv::arithmetic::Converter;
    use curv::BigInt;
    use curv::elliptic::curves;


    #[test]
    fn ether_wallet() {
        let wallet = LocalWallet::new(&mut rand::thread_rng());
        let addr = wallet.address();
        let signer = wallet.signer();

        let priv_slice = signer.to_bytes();

        let hex_str = hex::encode(priv_slice);
        println!("hex_str= {}", hex_str);
        dbg!(signer);
        dbg!(addr);
        dbg!(wallet);

        let newWallet = LocalWallet::from_str(&hex_str).unwrap();
        dbg!(newWallet.address());
    }

    #[test]
    fn ether_from_priv() {
        let bytes = "0000000000000000000000000000000000000000000000000000000000000001";
        let wallet = LocalWallet::from_str(bytes).unwrap();
        let addr = wallet.address();
        dbg!(addr);

        let priv_slice = hex::decode(bytes).unwrap();
        let secp = secp256k1::Secp256k1::new();
        let privk = SecretKey::from_slice(priv_slice.as_slice()).unwrap();
        let pubk = PublicKey::from_secret_key(&secp, &privk);

        let uncompressed_public_key = pubk.serialize_uncompressed();
        let hashed_public_key = keccak256(&uncompressed_public_key[1..]);

        let address = Address::from_slice(&hashed_public_key[12..]);
        println!("Ethereum address: {}", hex::encode(address));
    }

    fn calc_addr(s: BigInt) {
        let generator = curves::Point::<curves::Secp256k1>::generator();

        let secret = curves::Scalar::from_bigint(&s);
        let pk = secret * generator;


        let pk_bytes = pk.to_bytes(false);

        let pk_hash = keccak256(pk_bytes.as_ref()[1..].to_vec());

        let addr = Address::from_slice(&pk_hash[12..]);
        println!("addr for priv={}, is {:?}", s, addr);
    }

    #[test]
    fn test_special_addr() {
        calc_addr(0.into());
        calc_addr(1.into());
        calc_addr(BigInt::from(-1));
    }
}
