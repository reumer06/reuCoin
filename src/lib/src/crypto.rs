use crate::sha256::Hash;
use crate::util::Saveable;
use ecdsa::signature::Signer;
use ecdsa::signature::Verifier;
use ecdsa::{Signature as ECDSASignature, SigningKey, VerifyingKey};
use k256::Secp256k1;
use k256::pkcs8::spki;
use rand;
use serde::{Deserialize, Serialize};
use spki::EncodePublicKey;
use std::io::{Error as IoError, ErrorKind as IoErrorKind, Read, Result as IoResult, Write};
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Signature(ECDSASignature<Secp256k1>);
#[derive(Debug, Serialize, Deserialize, Clone)]

pub struct PublicKey(VerifyingKey<Secp256k1>);
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PrivateKey(#[serde(with = "signkey_serde")] pub SigningKey<Secp256k1>); //  use serialize and deserialize from this mod.

impl Saveable for PrivateKey {
    fn load<I: Read>(reader: I) -> IoResult<Self> {
        ciborium::de::from_reader(reader)
            .map_err(|_| IoError::new(IoErrorKind::InvalidData, "Failed to deserialize data"))
    }
    fn save<O: Write>(&self, writer: O) -> IoResult<()> {
        ciborium::ser::into_writer(self, writer)
            .map_err(|_| IoError::new(IoErrorKind::InvalidData, "Failed to serialize data"))?;
        Ok(())
    }
}

// save and load as PEM
impl Saveable for PublicKey {
    fn load<I: Read>(mut reader: I) -> IoResult<Self> {
        // read PEM-encoded public key into string
        let mut buf = String::new();
        reader.read_to_string(&mut buf)?;
        let public_key = buf
            .parse()
            .map_err(|_| IoError::new(IoErrorKind::InvalidData, "Failed to parse PublicKey"))?;
        Ok(PublicKey(public_key))
    }
    fn save<O: Write>(&self, mut writer: O) -> IoResult<()> {
        let s = self
            .0
            .to_public_key_pem(Default::default())
            .map_err(|_| IoError::new(IoErrorKind::InvalidData, "Failed to serialize PublicKey"))?;
        writer.write_all(s.as_bytes())?;
        Ok(())
    }
}
impl PrivateKey {
    pub fn new_key() -> Self {
        let bytes: [u8; 32] = rand::random();
        let signing_key = SigningKey::from_slice(&bytes).expect("failed to create signing key");
        PrivateKey(signing_key)
    }
    pub fn public_key(&self) -> PublicKey {
        PublicKey(self.0.verifying_key().clone())
    }
}
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

impl Signature {
    pub fn sign_output(output_hash: &Hash, private_key: &PrivateKey) -> Self {
        let signing_key = &private_key.0;
        let signature = signing_key.sign(&output_hash.as_bytes());
        Signature(signature)
    }

    pub fn verify(&self, output_hash: &Hash, public_key: &PublicKey) -> bool {
        public_key
            .0
            .verify(&output_hash.as_bytes(), &self.0)
            .is_ok()
    }
}
