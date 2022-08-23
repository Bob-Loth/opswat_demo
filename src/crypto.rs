use sha2::{Digest, Sha256};
use std::fmt::Write;
pub(crate) fn sha256_from_bytes(bytes: &[u8]) -> Result<String, std::fmt::Error> {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    let val = hasher.finalize(); //a byte vector.

    //hex formatting
    let mut s = String::with_capacity(2 * val.len());
    for byte in val {
        //this really shouldn't error unless sha2's implementation is bad
        write!(s, "{:02X}", byte)?;
    }

    Ok(s)
}
