use sha2::{Sha256, Digest};
use sha2::digest::Output;
use hex::ToHex;
pub(crate) fn sha256_from_bytes(bytes: &[u8]) -> String  {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    let val = hasher.finalize();

    hex::encode_upper(val)
}