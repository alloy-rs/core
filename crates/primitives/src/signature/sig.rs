use crate::{
    hex,
    signature::{Parity, SignatureError},
    U256,
};
use alloc::vec::Vec;
use core::str::FromStr;

/// An Ethereum ECDSA signature.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Signature<T> {
    /// Memoized ecdsa signature (if any)
    inner: T,

    v: Parity,
    r: U256,
    s: U256,
}

#[cfg(feature = "k256")]
impl<'a> TryFrom<&'a [u8]> for Signature<k256::ecdsa::Signature> {
    type Error = SignatureError;

    /// Parses a raw signature which is expected to be 65 bytes long where
    /// the first 32 bytes is the `r` value, the second 32 bytes the `s` value
    /// and the final byte is the `v` value in 'Electrum' notation.
    fn try_from(bytes: &'a [u8]) -> Result<Self, Self::Error> {
        if bytes.len() != 65 {
            return Err(k256::ecdsa::Error::new().into());
        }
        Self::from_bytes_and_parity(&bytes[..64], bytes[64] as u64)
    }
}

impl<'a> TryFrom<&'a [u8]> for Signature<()> {
    type Error = SignatureError;

    fn try_from(bytes: &'a [u8]) -> Result<Self, Self::Error> {
        if bytes.len() != 65 {
            return Err(SignatureError::FromBytes("expected exactly 65 bytes"));
        }
        Self::from_bytes_and_parity(bytes, bytes[64] as u64)
    }
}

#[cfg(feature = "k256")]
impl FromStr for Signature<k256::ecdsa::Signature> {
    type Err = SignatureError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = hex::decode(s)?;
        Self::try_from(&bytes[..])
    }
}

impl FromStr for Signature<()> {
    type Err = SignatureError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = hex::decode(s)?;
        Self::try_from(&bytes[..])
    }
}

impl From<&crate::Signature> for [u8; 65] {
    #[inline]
    fn from(value: &crate::Signature) -> [u8; 65] {
        value.as_bytes()
    }
}

impl From<crate::Signature> for [u8; 65] {
    #[inline]
    fn from(value: crate::Signature) -> [u8; 65] {
        value.as_bytes()
    }
}

impl From<&crate::Signature> for Vec<u8> {
    #[inline]
    fn from(value: &crate::Signature) -> Self {
        value.as_bytes().to_vec()
    }
}

impl From<crate::Signature> for Vec<u8> {
    #[inline]
    fn from(value: crate::Signature) -> Self {
        value.as_bytes().to_vec()
    }
}

#[cfg(feature = "k256")]
impl From<(k256::ecdsa::Signature, k256::ecdsa::RecoveryId)> for Signature<k256::ecdsa::Signature> {
    fn from(value: (k256::ecdsa::Signature, k256::ecdsa::RecoveryId)) -> Self {
        Self::from_signature_and_parity(value.0, value.1).unwrap()
    }
}

#[cfg(feature = "rlp")]
impl crate::Signature {
    pub fn decode_rlp_vrs(buf: &mut &[u8]) -> Result<Self, alloy_rlp::Error> {
        use alloy_rlp::Decodable;

        let parity: Parity = Decodable::decode(buf)?;
        let r = Decodable::decode(buf)?;
        let s = Decodable::decode(buf)?;

        Self::from_rs_and_parity(r, s, parity)
            .map_err(|_| alloy_rlp::Error::Custom("attempted to decode invalid field element"))
    }
}

#[cfg(feature = "k256")]
impl Signature<k256::ecdsa::Signature> {
    #[doc(hidden)]
    pub fn test_signature() -> Self {
        Self::from_scalars_and_parity(
            b256!("840cfc572845f5786e702984c2a582528cad4b49b2a10b9db1be7fca90058565"),
            b256!("25e7109ceb98168d95b09b18bbf6b685130e0562f233877d492b94eee0c5b6d1"),
            false,
        )
        .unwrap()
    }

    /// Instantiate from a signature and recovery id
    pub fn from_signature_and_parity<T: TryInto<Parity, Error = E>, E: Into<SignatureError>>(
        sig: k256::ecdsa::Signature,
        parity: T,
    ) -> Result<Self, SignatureError> {
        let r = U256::from_be_slice(sig.r().to_bytes().as_ref());
        let s = U256::from_be_slice(sig.s().to_bytes().as_ref());
        Ok(Self { inner: sig, v: parity.try_into().map_err(Into::into)?, r, s })
    }

    /// Instantiate from v, r, s.
    pub fn from_rs_and_parity<T: TryInto<Parity, Error = E>, E: Into<SignatureError>>(
        r: U256,
        s: U256,
        parity: T,
    ) -> Result<Self, SignatureError> {
        Self::from_scalars_and_parity(r.into(), s.into(), parity)
    }

    /// Parses a signature from a byte slice, with a v value
    #[inline]
    pub fn from_bytes_and_parity<T: TryInto<Parity, Error = E>, E: Into<SignatureError>>(
        bytes: &[u8],
        parity: T,
    ) -> Result<Self, SignatureError> {
        let sig = k256::ecdsa::Signature::from_slice(bytes)?;
        Self::from_signature_and_parity(sig, parity)
    }

    /// Creates a [`Signature`] from the serialized `r` and `s` scalar values, which comprise the
    /// ECDSA signature, alongside a `v` value, used to determine the recovery ID.
    ///
    /// See [`k256::ecdsa::Signature::from_scalars`] for more details.
    #[inline]
    pub fn from_scalars_and_parity<T: TryInto<Parity, Error = E>, E: Into<SignatureError>>(
        r: crate::B256,
        s: crate::B256,
        parity: T,
    ) -> Result<Self, SignatureError> {
        let inner = k256::ecdsa::Signature::from_scalars(r.0, s.0)?;
        Self::from_signature_and_parity(inner, parity)
    }

    /// Normalizes the signature into "low S" form as described in
    /// [BIP 0062: Dealing with Malleability][1].
    ///
    /// [1]: https://github.com/bitcoin/bips/blob/master/bip-0062.mediawiki
    #[inline]
    pub fn normalize_s(&self) -> Option<Self> {
        // Normalize into "low S" form. See:
        // - https://github.com/RustCrypto/elliptic-curves/issues/988
        // - https://github.com/bluealloy/revm/pull/870
        self.inner.normalize_s().map(|normalized| {
            let s = U256::from_be_slice(normalized.s().to_bytes().as_ref());
            Self { inner: normalized, v: self.v.inverted(), r: self.r, s }
        })
    }

    /// Returns the recovery ID.
    #[inline]
    pub const fn recid(&self) -> k256::ecdsa::RecoveryId {
        self.v.recid()
    }

    #[doc(hidden)]
    #[deprecated(note = "use `Signature::recid` instead")]
    pub const fn recovery_id(&self) -> k256::ecdsa::RecoveryId {
        self.recid()
    }

    /// Recovers an [`Address`] from this signature and the given message by first prefixing and
    /// hashing the message according to [EIP-191](crate::eip191_hash_message).
    ///
    /// [`Address`]: crate::Address
    #[inline]
    pub fn recover_address_from_msg<T: AsRef<[u8]>>(
        &self,
        msg: T,
    ) -> Result<crate::Address, SignatureError> {
        self.recover_from_msg(msg).map(|vk| crate::Address::from_public_key(&vk))
    }

    /// Recovers an [`Address`] from this signature and the given prehashed message.
    ///
    /// [`Address`]: crate::Address
    #[inline]
    pub fn recover_address_from_prehash(
        &self,
        prehash: crate::B256,
    ) -> Result<crate::Address, SignatureError> {
        self.recover_from_prehash(prehash).map(|vk| crate::Address::from_public_key(&vk))
    }

    /// Recovers a [`VerifyingKey`] from this signature and the given message by first prefixing and
    /// hashing the message according to [EIP-191](crate::eip191_hash_message).
    ///
    /// [`VerifyingKey`]: k256::ecdsa::VerifyingKey
    #[inline]
    pub fn recover_from_msg<T: AsRef<[u8]>>(
        &self,
        msg: T,
    ) -> Result<k256::ecdsa::VerifyingKey, SignatureError> {
        self.recover_from_prehash(crate::eip191_hash_message(msg))
    }

    /// Recovers a [`VerifyingKey`] from this signature and the given prehashed message.
    ///
    /// [`VerifyingKey`]: k256::ecdsa::VerifyingKey
    #[inline]
    pub fn recover_from_prehash(
        &self,
        prehash: crate::B256,
    ) -> Result<k256::ecdsa::VerifyingKey, SignatureError> {
        let this = self.normalize_s().unwrap_or(*self);
        k256::ecdsa::VerifyingKey::recover_from_prehash(
            prehash.as_slice(),
            &this.inner,
            this.recid(),
        )
        .map_err(Into::into)
    }
}

impl Signature<()> {
    /// Parses a signature from a byte slice, with a v value
    ///
    /// # Panics
    ///
    /// If the slice is not at least 64 bytes long.
    #[inline]
    pub fn from_bytes_and_parity<T: TryInto<Parity, Error = E>, E: Into<SignatureError>>(
        bytes: &[u8],
        parity: T,
    ) -> Result<Self, SignatureError> {
        let r = U256::from_be_slice(&bytes[..32]);
        let s = U256::from_be_slice(&bytes[32..64]);
        Self::from_rs_and_parity(r, s, parity)
    }

    /// Instantiate from v, r, s.
    pub fn from_rs_and_parity<T: TryInto<Parity, Error = E>, E: Into<SignatureError>>(
        r: U256,
        s: U256,
        parity: T,
    ) -> Result<Self, SignatureError> {
        Ok(Self { inner: (), v: parity.try_into().map_err(Into::into)?, r, s })
    }
}

impl<S: Copy> Signature<S> {
    /// Returns the inner ECDSA signature.
    #[inline]
    pub const fn into_inner(self) -> S {
        self.inner
    }

    /// Modifies the recovery ID by applying [EIP-155] to a `v` value.
    ///
    /// [EIP-155]: https://eips.ethereum.org/EIPS/eip-155
    #[inline]
    pub fn with_chain_id(self, chain_id: u64) -> Self {
        self.with_parity(self.v.with_chain_id(chain_id))
    }

    /// Modifies the recovery ID by dropping any [EIP-155] v value, converting
    /// to a simple parity bool.
    pub fn with_parity_bool(self) -> Self {
        self.with_parity(self.v.to_parity_bool())
    }
}

impl<S> Signature<S> {
    /// Returns the inner ECDSA signature.
    #[inline]
    pub const fn inner(&self) -> &S {
        &self.inner
    }

    /// Returns the `r` component of this signature.
    #[inline]
    pub const fn r(&self) -> U256 {
        self.r
    }

    /// Returns the `s` component of this signature.
    #[inline]
    pub const fn s(&self) -> U256 {
        self.s
    }

    /// Returns the recovery ID as a `u8`.
    #[inline]
    pub const fn v(&self) -> Parity {
        self.v
    }

    /// Returns the byte-array representation of this signature.
    ///
    /// The first 32 bytes are the `r` value, the second 32 bytes the `s` value
    /// and the final byte is the `v` value in 'Electrum' notation.
    #[inline]
    pub fn as_bytes(&self) -> [u8; 65] {
        let mut sig = [0u8; 65];
        sig[..32].copy_from_slice(&self.r.to_be_bytes::<32>());
        sig[32..64].copy_from_slice(&self.s.to_be_bytes::<32>());
        sig[64] = self.v.y_parity_byte();
        sig
    }

    /// Sets the recovery ID by normalizing a `v` value.
    #[inline]
    pub fn with_parity<T: Into<Parity>>(self, parity: T) -> Self {
        Self { inner: self.inner, v: parity.into(), r: self.r, s: self.s }
    }

    /// Length of RLP RS field encoding
    #[cfg(feature = "rlp")]
    pub fn rlp_rs_len(&self) -> usize {
        alloy_rlp::Encodable::length(&self.r) + alloy_rlp::Encodable::length(&self.s)
    }

    #[cfg(feature = "rlp")]
    /// Length of RLP V field encoding
    pub fn rlp_vrs_len(&self) -> usize {
        self.rlp_rs_len() + alloy_rlp::Encodable::length(&self.v)
    }

    /// Write R and S to an RLP buffer in progress.
    #[cfg(feature = "rlp")]
    pub fn write_rlp_rs(&self, out: &mut dyn alloy_rlp::BufMut) {
        alloy_rlp::Encodable::encode(&self.r, out);
        alloy_rlp::Encodable::encode(&self.s, out);
    }

    /// Write the V to an RLP buffer without using EIP-155.
    #[cfg(feature = "rlp")]
    pub fn write_rlp_v(&self, out: &mut dyn alloy_rlp::BufMut) {
        alloy_rlp::Encodable::encode(&self.v, out);
    }

    /// Write the VRS to the output. The V will always be 27 or 28.
    #[cfg(feature = "rlp")]
    pub fn write_rlp_vrs(&self, out: &mut dyn alloy_rlp::BufMut) {
        self.write_rlp_v(out);
        self.write_rlp_rs(out);
    }
}

#[cfg(feature = "rlp")]
impl alloy_rlp::Encodable for crate::Signature {
    fn encode(&self, out: &mut dyn alloy_rlp::BufMut) {
        alloy_rlp::Header { list: true, payload_length: self.rlp_vrs_len() }.encode(out);
        self.write_rlp_vrs(out);
    }

    fn length(&self) -> usize {
        let payload_length = self.rlp_vrs_len();
        payload_length + alloy_rlp::length_of_length(payload_length)
    }
}

#[cfg(feature = "rlp")]
impl alloy_rlp::Decodable for crate::Signature {
    fn decode(buf: &mut &[u8]) -> Result<Self, alloy_rlp::Error> {
        let header = alloy_rlp::Header::decode(buf)?;
        let pre_len = buf.len();
        let decoded = Self::decode_rlp_vrs(buf)?;
        let consumed = pre_len - buf.len();
        if consumed != header.payload_length {
            return Err(alloy_rlp::Error::Custom("consumed incorrect number of bytes"));
        }

        Ok(decoded)
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for crate::Signature {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;

        let mut map = serializer.serialize_map(Some(3))?;

        map.serialize_entry("r", &self.r)?;
        map.serialize_entry("s", &self.s)?;

        match self.v {
            Parity::Eip155(v) => map.serialize_entry("v", &crate::U64::from(v))?,
            Parity::NonEip155(b) => map.serialize_entry("v", &(b as u8 + 27))?,
            Parity::Parity(true) => map.serialize_entry("yParity", "0x1")?,
            Parity::Parity(false) => map.serialize_entry("yParity", "0x0")?,
        }
        map.end()
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for crate::Signature {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::MapAccess;

        enum Field {
            R,
            S,
            V,
            YParity,
            Unknown,
        }

        impl<'de> serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> serde::de::Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(
                        &self,
                        formatter: &mut core::fmt::Formatter<'_>,
                    ) -> core::fmt::Result {
                        formatter.write_str("v, r, s, or yParity")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "r" => Ok(Field::R),
                            "s" => Ok(Field::S),
                            "v" => Ok(Field::V),
                            "yParity" => Ok(Field::YParity),
                            _ => Ok(Field::Unknown),
                        }
                    }
                }
                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct SignatureVisitor;
        impl<'de> serde::de::Visitor<'de> for SignatureVisitor {
            type Value = crate::Signature;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("a JSON signature object containing r, s, and v or yParity")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut v: Option<Parity> = None;
                let mut r = None;
                let mut s = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::V => {
                            let value: crate::U64 = map.next_value()?;
                            let parity = value.try_into().map_err(|_| {
                                serde::de::Error::invalid_value(
                                    serde::de::Unexpected::Unsigned(value.as_limbs()[0]),
                                    &"a valid v value matching the range 0 | 1 | 27 | 28 | 35..",
                                )
                            })?;
                            v = Some(parity);
                        }
                        Field::YParity => {
                            let value: crate::Uint<1, 1> = map.next_value()?;
                            if v.is_none() {
                                v = Some(value.into());
                            }
                        }
                        Field::R => {
                            let value: U256 = map.next_value()?;
                            r = Some(value);
                        }
                        Field::S => {
                            let value: U256 = map.next_value()?;
                            s = Some(value);
                        }
                        _ => {}
                    }
                }

                let v = v.ok_or_else(|| serde::de::Error::missing_field("v"))?;
                let r = r.ok_or_else(|| serde::de::Error::missing_field("r"))?;
                let s = s.ok_or_else(|| serde::de::Error::missing_field("s"))?;

                crate::Signature::from_rs_and_parity(r, s, v).map_err(serde::de::Error::custom)
            }
        }

        deserializer.deserialize_map(SignatureVisitor)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::str::FromStr;

    #[cfg(feature = "rlp")]
    use alloy_rlp::{Decodable, Encodable};

    #[test]
    #[cfg(TODO)] // TODO: Transaction
    fn can_recover_tx_sender() {
        // random mainnet tx: https://etherscan.io/tx/0x86718885c4b4218c6af87d3d0b0d83e3cc465df2a05c048aa4db9f1a6f9de91f
        let tx_rlp = hex::decode("02f872018307910d808507204d2cb1827d0094388c818ca8b9251b393131c08a736a67ccb19297880320d04823e2701c80c001a0cf024f4815304df2867a1a74e9d2707b6abda0337d2d54a4438d453f4160f190a07ac0e6b3bc9395b5b9c8b9e6d77204a236577a5b18467b9175c01de4faa208d9").unwrap();
        let tx: Transaction = rlp::decode(&tx_rlp).unwrap();
        assert_eq!(tx.rlp(), tx_rlp);
        assert_eq!(
            tx.hash,
            "0x86718885c4b4218c6af87d3d0b0d83e3cc465df2a05c048aa4db9f1a6f9de91f".parse().unwrap()
        );
        assert_eq!(tx.transaction_type, Some(2.into()));
        let expected = Address::from_str("0x95222290DD7278Aa3Ddd389Cc1E1d165CC4BAfe5").unwrap();
        assert_eq!(tx.recover_from().unwrap(), expected);
    }

    #[test]
    #[cfg(feature = "k256")]
    fn can_recover_tx_sender_not_normalized() {
        let sig = Signature::from_str("48b55bfa915ac795c431978d8a6a992b628d557da5ff759b307d495a36649353efffd310ac743f371de3b9f7f9cb56c0b28ad43601b4ab949f53faa07bd2c8041b").unwrap();
        let hash = b256!("5eb4f5a33c621f32a8622d5f943b6b102994dfe4e5aebbefe69bb1b2aa0fc93e");
        let expected = address!("0f65fe9276bc9a24ae7083ae28e2660ef72df99e");
        assert_eq!(sig.recover_address_from_prehash(hash).unwrap(), expected);
    }

    #[test]
    #[cfg(feature = "k256")]
    fn recover_web3_signature() {
        // test vector taken from:
        // https://web3js.readthedocs.io/en/v1.2.2/web3-eth-accounts.html#sign
        let sig = Signature::from_str(
            "b91467e570a6466aa9e9876cbcd013baba02900b8979d43fe208a4a4f339f5fd6007e74cd82e037b800186422fc2da167c747ef045e5d18a5f5d4300f8e1a0291c"
        ).expect("could not parse signature");
        let expected = address!("2c7536E3605D9C16a7a3D7b1898e529396a65c23");
        assert_eq!(sig.recover_address_from_msg("Some data").unwrap(), expected);
    }

    #[test]
    fn signature_from_str() {
        let s1 = crate::Signature::from_str(
            "0xaa231fbe0ed2b5418e6ba7c19bee2522852955ec50996c02a2fe3e71d30ddaf1645baf4823fea7cb4fcc7150842493847cfb6a6d63ab93e8ee928ee3f61f503500"
        ).expect("could not parse 0x-prefixed signature");

        let s2 = crate::Signature::from_str(
            "aa231fbe0ed2b5418e6ba7c19bee2522852955ec50996c02a2fe3e71d30ddaf1645baf4823fea7cb4fcc7150842493847cfb6a6d63ab93e8ee928ee3f61f503500"
        ).expect("could not parse non-prefixed signature");

        assert_eq!(s1, s2);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn deserialize_without_parity() {
        let raw_signature_without_y_parity = r#"{
            "r":"0xc569c92f176a3be1a6352dd5005bfc751dcb32f57623dd2a23693e64bf4447b0",
            "s":"0x1a891b566d369e79b7a66eecab1e008831e22daa15f91a0a0cf4f9f28f47ee05",
            "v":"0x1"
        }"#;

        let signature: crate::Signature =
            serde_json::from_str(raw_signature_without_y_parity).unwrap();

        let expected = crate::Signature::from_rs_and_parity(
            U256::from_str("0xc569c92f176a3be1a6352dd5005bfc751dcb32f57623dd2a23693e64bf4447b0")
                .unwrap(),
            U256::from_str("0x1a891b566d369e79b7a66eecab1e008831e22daa15f91a0a0cf4f9f28f47ee05")
                .unwrap(),
            1,
        )
        .unwrap();

        assert_eq!(signature, expected);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn deserialize_with_parity() {
        let raw_signature_with_y_parity = serde_json::json!(
            {
            "r":"0xc569c92f176a3be1a6352dd5005bfc751dcb32f57623dd2a23693e64bf4447b0",
            "s":"0x1a891b566d369e79b7a66eecab1e008831e22daa15f91a0a0cf4f9f28f47ee05",
            "v":"0x1",
            "yParity": "0x1"
        }
        );

        println!("{raw_signature_with_y_parity}");
        let signature: crate::Signature =
            serde_json::from_value(raw_signature_with_y_parity).unwrap();

        let expected = crate::Signature::from_rs_and_parity(
            U256::from_str("0xc569c92f176a3be1a6352dd5005bfc751dcb32f57623dd2a23693e64bf4447b0")
                .unwrap(),
            U256::from_str("0x1a891b566d369e79b7a66eecab1e008831e22daa15f91a0a0cf4f9f28f47ee05")
                .unwrap(),
            1,
        )
        .unwrap();

        assert_eq!(signature, expected);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serialize_both_parity() {
        // this test should be removed if the struct moves to an enum based on tx type
        let signature = crate::Signature::from_rs_and_parity(
            U256::from_str("0xc569c92f176a3be1a6352dd5005bfc751dcb32f57623dd2a23693e64bf4447b0")
                .unwrap(),
            U256::from_str("0x1a891b566d369e79b7a66eecab1e008831e22daa15f91a0a0cf4f9f28f47ee05")
                .unwrap(),
            1,
        )
        .unwrap();

        let serialized = serde_json::to_string(&signature).unwrap();
        assert_eq!(
            serialized,
            r#"{"r":"0xc569c92f176a3be1a6352dd5005bfc751dcb32f57623dd2a23693e64bf4447b0","s":"0x1a891b566d369e79b7a66eecab1e008831e22daa15f91a0a0cf4f9f28f47ee05","yParity":"0x1"}"#
        );
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serialize_v_only() {
        // this test should be removed if the struct moves to an enum based on tx type
        let signature = crate::Signature::from_rs_and_parity(
            U256::from_str("0xc569c92f176a3be1a6352dd5005bfc751dcb32f57623dd2a23693e64bf4447b0")
                .unwrap(),
            U256::from_str("0x1a891b566d369e79b7a66eecab1e008831e22daa15f91a0a0cf4f9f28f47ee05")
                .unwrap(),
            1,
        )
        .unwrap();

        let expected = r#"{"r":"0xc569c92f176a3be1a6352dd5005bfc751dcb32f57623dd2a23693e64bf4447b0","s":"0x1a891b566d369e79b7a66eecab1e008831e22daa15f91a0a0cf4f9f28f47ee05","yParity":"0x1"}"#;

        let serialized = serde_json::to_string(&signature).unwrap();
        assert_eq!(serialized, expected);
    }

    #[cfg(feature = "rlp")]
    #[test]
    fn signature_rlp_decode() {
        // Given a hex-encoded byte sequence
        let bytes = crate::hex!("f84301a048b55bfa915ac795c431978d8a6a992b628d557da5ff759b307d495a36649353a010002cef538bc0c8e21c46080634a93e082408b0ad93f4a7207e63ec5463793d");

        // Decode the byte sequence into a Signature instance
        let result = Signature::decode(&mut &bytes[..]).unwrap();

        // Assert that the decoded Signature matches the expected Signature
        assert_eq!(
            result,
            Signature::from_str("48b55bfa915ac795c431978d8a6a992b628d557da5ff759b307d495a3664935310002cef538bc0c8e21c46080634a93e082408b0ad93f4a7207e63ec5463793d01").unwrap()
        );
    }

    #[cfg(feature = "rlp")]
    #[test]
    fn signature_rlp_encode() {
        // Given a Signature instance
        let sig = Signature::from_str("48b55bfa915ac795c431978d8a6a992b628d557da5ff759b307d495a36649353efffd310ac743f371de3b9f7f9cb56c0b28ad43601b4ab949f53faa07bd2c8041b").unwrap();

        // Initialize an empty buffer
        let mut buf = vec![];

        // Encode the Signature into the buffer
        sig.encode(&mut buf);

        // Define the expected hex-encoded string
        let expected = "f8431ba048b55bfa915ac795c431978d8a6a992b628d557da5ff759b307d495a36649353a0efffd310ac743f371de3b9f7f9cb56c0b28ad43601b4ab949f53faa07bd2c804";

        // Assert that the encoded buffer matches the expected hex-encoded string
        assert_eq!(hex::encode(&buf), expected);
    }

    #[cfg(feature = "rlp")]
    #[test]
    fn signature_rlp_length() {
        // Given a Signature instance
        let sig = Signature::from_str("48b55bfa915ac795c431978d8a6a992b628d557da5ff759b307d495a36649353efffd310ac743f371de3b9f7f9cb56c0b28ad43601b4ab949f53faa07bd2c8041b").unwrap();

        // Assert that the length of the Signature matches the expected length
        assert_eq!(sig.length(), 69);
    }
}
