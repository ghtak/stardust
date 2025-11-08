pub struct VerifyResult {
    pub is_valid: bool,
    pub needs_rehash: bool,
}

pub trait Hasher: Sync + Send {
    fn hash(&self, password: &str) -> crate::Result<String>;
    fn verify(&self, password: &str, hash: &str)
    -> crate::Result<VerifyResult>;
}

#[derive(Default, Clone, Copy)]
pub struct Sha256Hasher {}

impl Hasher for Sha256Hasher {
    fn hash(&self, password: &str) -> crate::Result<String> {
        use base64::prelude::*;
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(password.as_bytes());
        let hash = hasher.finalize();
        Ok(BASE64_STANDARD.encode(hash))
    }

    fn verify(
        &self,
        password: &str,
        hash: &str,
    ) -> crate::Result<VerifyResult> {
        Ok(VerifyResult {
            is_valid: self.hash(password)? == hash,
            needs_rehash: false,
        })
    }
}

#[derive(Default, Clone, Copy)]
pub struct DummyHasher {}

impl Hasher for DummyHasher {
    fn hash(&self, password: &str) -> crate::Result<String> {
        Ok(password.to_string())
    }

    fn verify(
        &self,
        password: &str,
        hash: &str,
    ) -> crate::Result<VerifyResult> {
        Ok(VerifyResult {
            is_valid: password == hash,
            needs_rehash: false,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // use crate::{common::Argon2Config, security::crypto::hash};

    // #[tokio::test]
    // async fn test_argon_hasher() {
    //     let mut policy: Argon2Config = Default::default();
    //     let hasher = hash::Argon2Hasher::new(policy.clone());
    //     let password = "password";

    //     let hash = hasher.hash(&password).unwrap();
    //     println!("hash: {}", hash);

    //     let result = hasher.verify(&password, &hash).unwrap();
    //     assert!(result.is_valid);
    //     assert!(!result.needs_rehash);

    //     let result = hasher.verify("wrongpassword", &hash).unwrap();
    //     assert!(!result.is_valid);
    //     assert!(!result.needs_rehash);

    //     policy.argon2_memory_kib = 131072; // 128 MiB
    //     let new_hasher = hash::Argon2Hasher::new(policy.clone());
    //     let result = new_hasher.verify(&password, &hash).unwrap();
    //     assert!(result.is_valid);
    //     assert!(result.needs_rehash);
    // }

    #[tokio::test]
    async fn test_sha256_hasher() {
        let hasher = Sha256Hasher {};
        let password = "password";

        let hash = hasher.hash(&password).unwrap();
        println!("hash: {}", hash);

        let result = hasher.verify(&password, &hash).unwrap();
        assert!(result.is_valid);
        assert!(!result.needs_rehash);
    }
}
