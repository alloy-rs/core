use super::FixedBytes;
use alloc::fmt;
use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};

impl<const N: usize> Serialize for FixedBytes<N> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        if serializer.is_human_readable() {
            let mut buf = hex::Buffer::<N, true>::new();
            serializer.serialize_str(buf.format(&self.0))
        } else {
            serializer.serialize_bytes(self.as_slice())
        }
    }
}

impl<'de, const N: usize> Deserialize<'de> for FixedBytes<N> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct FixedVisitor<const N: usize>;

        impl<'de, const N: usize> Visitor<'de> for FixedVisitor<N> {
            type Value = FixedBytes<N>;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(
                    formatter,
                    "{} bytes, represented as a hex string of length {}, an array of u8, or raw bytes",
                    N,
                    N * 2
                )
            }

            fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                <[u8; N]>::try_from(v)
                    .map(FixedBytes)
                    .map_err(de::Error::custom)
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: de::SeqAccess<'de>,
            {
                let mut bytes = [0u8; N];

                bytes.iter_mut().enumerate().try_for_each(|(i, b)| {
                    *b = seq.next_element()?.ok_or_else(|| {
                        de::Error::invalid_length(i, &format!("exactly {} bytes", N).as_str())
                    })?;
                    Ok(())
                })?;

                if let Ok(Some(_)) = seq.next_element::<u8>() {
                    return Err(de::Error::invalid_length(
                        N + 1,
                        &format!("exactly {} bytes", N).as_str(),
                    ))
                }

                Ok(FixedBytes(bytes))
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let v = hex::decode(v).map_err(|_| {
                    de::Error::invalid_value(de::Unexpected::Str(v), &"a valid hex string")
                })?;
                <[u8; N]>::try_from(v.as_slice())
                    .map_err(|_| {
                        de::Error::invalid_length(
                            v.len(),
                            &format!("exactly {} bytes, as {} hex chars", N, N * 2).as_str(),
                        )
                    })
                    .map(FixedBytes)
            }
        }
        deserializer.deserialize_any(FixedVisitor::<N>)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;
    #[derive(Debug, Deserialize)]
    struct TestCase<const N: usize> {
        fixed: FixedBytes<N>,
    }

    #[test]
    fn serde() {
        let bytes = FixedBytes([0, 0, 0, 0, 1, 35, 69, 103, 137, 171, 205, 239]);
        let ser = serde_json::to_string(&bytes).unwrap();
        assert_eq!(ser, "\"0x000000000123456789abcdef\"");
        assert_eq!(serde_json::from_str::<FixedBytes<12>>(&ser).unwrap(), bytes);
    }

    #[test]
    fn serde_num_array() {
        let json = serde_json::json! {{"fixed": [0,1,2,3,4]}};

        assert_eq!(
            serde_json::from_value::<TestCase<5>>(json.clone())
                .unwrap()
                .fixed,
            FixedBytes([0, 1, 2, 3, 4])
        );

        assert!(format!(
            "{}",
            serde_json::from_value::<TestCase<4>>(json.clone()).unwrap_err()
        )
        .contains("invalid length 5, expected exactly 4 bytes"),);
    }
}
