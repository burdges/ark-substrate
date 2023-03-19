#![cfg_attr(not(feature = "std"), no_std)]

use ark_serialize::{CanonicalDeserialize, CanonicalSerialize, Compress, Validate};
use ark_std::{io::Cursor, vec, vec::Vec};

pub fn serialize_argument(result: impl CanonicalSerialize) -> Vec<u8> {
    let mut serialized_result = vec![0u8; result.serialized_size(Compress::No)];
    let mut cursor = Cursor::new(&mut serialized_result[..]);
    result.serialize_uncompressed(&mut cursor).unwrap();
    serialized_result
}

pub fn deserialize_result<Field: CanonicalDeserialize>(result: &Vec<u8>) -> Field {
    let cursor = Cursor::new(result);
    Field::deserialize_with_mode(cursor, Compress::No, Validate::No).unwrap()
}
