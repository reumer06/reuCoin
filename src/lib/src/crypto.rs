use ecdsa::{Signature as EcdsaSignature, SigningKey, VerifyingKey, signature::Signer};
use k256::Secp256k1;
pub struct Signature(EcdsaSignature<Secp256k1>);
pub struct PublicKey(VerifyingKey<Secp256k1>);
pub struct PrivateKey(SigningKey<Secp256k1>);
