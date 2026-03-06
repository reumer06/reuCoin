use ecdsa::{Signature as ECDSASignature, SigningKey, VerifyingKey};

use k256::Secp256k1;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Signature(ECDSASignature<Secp256k1>);
#[derive(Debug, Serialize, Deserialize, Clone)]

pub struct PublicKey(VerifyingKey<Secp256k1>);
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PrivateKey(#[serde(with = "signkey_serde")] pub SigningKey<Secp256k1>); //  use serialize and deserialize from this mod.
mod signkey_serde {
    use serde::Deserialize;

    pub fn serialize<S>(
        key: &super::SigningKey<super::Secp256k1>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_bytes(&key.to_bytes()) // convert the key into a slice of bytes then serialize it.
    }

    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<super::SigningKey<super::Secp256k1>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // use a byte slice or array instead of Vec for better performance
        let bytes: Vec<u8> = Vec::<u8>::deserialize(deserializer)?;
        Ok(super::SigningKey::from_slice(&bytes).unwrap())
    }
}
