use ecdsa::{
    signature::Signer,
    Signature as ECDSASignature,
    VerifyingKey,
    SigningKey
};

use k256::Secp256k1;
pub struct Signature(ECDSASignature<Secp256k1>);
pub struct PublicKey(VerifyingKey<Secp256k1>);
pub struct PrivateKey(SigningKey<Secp256k1>);
