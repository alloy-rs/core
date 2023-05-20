use crate::primitives::Signature;
use ethers_primitives::{keccak256, Address, B256, U256};
use secp256k1::{
    ecdsa::{RecoverableSignature, RecoveryId},
    Error, Message, Secp256k1, SecretKey,
};

/// Recovers the signer of the given message hash from the given signature.
pub fn recover_signer(data: &[u8; 64], id: i32, msg_hash: &[u8; 32]) -> Result<Address, Error> {
    let recid = RecoveryId::from_i32(id)?;
    let sig = RecoverableSignature::from_compact(data, recid)?;
    let public = Secp256k1::new().recover_ecdsa(as_message(msg_hash), &sig)?;
    let hash = keccak256(&public.serialize_uncompressed()[1..]);
    Ok(Address::from_word(hash))
}

/// Signs a message hash with the given secret key and returns the corresponding
/// signature.
pub fn sign_message(secret: &[u8; 32], msg_hash: &[u8; 32]) -> Result<Signature, Error> {
    let sk = SecretKey::from_slice(secret.as_ref())?;
    let sig = Secp256k1::new().sign_ecdsa_recoverable(as_message(msg_hash), &sk);
    let (recid, data) = sig.serialize_compact();
    let signature = Signature {
        r: U256::try_from_be_slice(&data[..32]).unwrap(),
        s: U256::try_from_be_slice(&data[32..64]).unwrap(),
        odd_y_parity: recid.to_i32() != 0,
    };
    Ok(signature)
}

#[inline]
fn as_message(msg_hash: &[u8; 32]) -> &Message {
    // SAFETY: `Message` wraps a `[u8; 32]`
    unsafe { &*(msg_hash as *const [u8; 32] as *const Message) }
}

#[cfg(test)]
mod tests {
    use ethers_primitives::Address;
    use hex_literal::hex;

    #[test]
    fn recover_signer() {
        let sig = hex!("650acf9d3f5f0a2c799776a1254355d5f4061762a237396a99a0e0e3fc2bcd6729514a0dacb2e623ac4abd157cb18163ff942280db4d5caad66ddf941ba12e03");
        let id = 0;
        let hash = hex!("47173285a8d7341e5e972fc677286384f802f8ef42a5ec5f03bbfa254cb01fad");
        let out: Address = hex!("c08b5542d177ac6686946920409741463a15dddb").into();

        assert_eq!(super::recover_signer(&sig, id, &hash), Ok(out));
    }
}
