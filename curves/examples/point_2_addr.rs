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
    use ethers::core::k256::elliptic_curve::weierstrass::add;
    use ethers::signers::{LocalWallet, Signer, Wallet};
    use secp256k1::rand;

    #[test]
    fn ether_wallet() {
        let wallet = LocalWallet::new(&mut rand::thread_rng());
        let addr = wallet.address();
        let signer = wallet.signer();
        signer.to_bytes();
        dbg!(signer);
        dbg!(addr);
        dbg!(wallet);
    }
}
