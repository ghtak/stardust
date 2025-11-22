use std::sync::Arc;

use crate::config::Argon2Config;
use argon2::{Algorithm, Argon2, Params, PasswordHash, Version};

pub struct VerifyResult {
    pub is_valid: bool,
    pub needs_rehash: bool,
}

pub trait Hasher: Sync + Send {
    fn hash(&self, password: &str) -> crate::Result<String>;
    fn verify(&self, password: &str, hash: &str) -> crate::Result<VerifyResult>;
}

pub struct Argon2Hasher {
    argon2: Arc<Argon2<'static>>,
    config: Argon2Config,
}

impl TryFrom<Argon2Config> for argon2::Params {
    type Error = argon2::Error;

    fn try_from(config: Argon2Config) -> Result<Self, Self::Error> {
        argon2::Params::new(
            config.memory_kib,
            config.iterations,
            config.parallelism,
            config.output_len.map(|v| v as usize),
        )
    }
}

impl Default for Argon2Config {
    fn default() -> Self {
        Self {
            memory_kib: 16384,
            iterations: 1,
            parallelism: 1,
            algorithm: "argon2id".into(),
            version: 0x13,
            output_len: Some(32),
        }
    }
}

impl Argon2Hasher {
    pub fn new(config: Argon2Config) -> Self {
        let params: Params = config.clone().try_into().expect("invalid argon2 params from config");
        let argon2 = Arc::new(Argon2::new(
            config.algorithm.parse().unwrap_or(Algorithm::Argon2id),
            Version::try_from(config.version).unwrap_or(Version::V0x13),
            params,
        ));
        Self { argon2, config }
    }

    pub fn needs_rehash(&self, stored_hash: &PasswordHash<'_>) -> bool {
        let required_algorithm =
            self.config.algorithm.parse().unwrap_or(Algorithm::Argon2id).ident();
        if stored_hash.algorithm != required_algorithm {
            return true;
        }
        if let Some(version) = stored_hash.version {
            if version != self.config.version {
                return true;
            }
        }
        let required_params = self.argon2.params();
        for (k, required_value) in [
            ("m", required_params.m_cost()),
            ("t", required_params.t_cost()),
            ("p", required_params.p_cost()),
        ]
        .iter()
        {
            let needs_rehash = stored_hash
                .params
                .get(*k)
                .and_then(|v| v.decimal().ok())
                .map(|stored_value| stored_value < *required_value)
                .unwrap_or(true);
            if needs_rehash {
                return true;
            }
        }
        false
    }
}

impl Default for Argon2Hasher {
    fn default() -> Self {
        Self::new(Argon2Config::default())
    }
}

impl Hasher for Argon2Hasher {
    fn hash(&self, password: &str) -> crate::Result<String> {
        use argon2::{PasswordHasher, password_hash::SaltString};
        use base64::prelude::*;
        let seed = BASE64_STANDARD_NO_PAD.encode(b"testsalt_testsalt");
        let salt = SaltString::from_b64(&seed).unwrap();
        let hash = self
            .argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| crate::Error::HashError(e.to_string().into()))?
            .to_string();
        Ok(hash)
    }

    fn verify(&self, password: &str, hash: &str) -> crate::Result<VerifyResult> {
        use argon2::password_hash::{PasswordHash, PasswordVerifier};
        let parsed =
            PasswordHash::new(hash).map_err(|e| crate::Error::HashError(e.to_string().into()))?;
        let is_valid = self.argon2.verify_password(password.as_bytes(), &parsed).is_ok();
        let needs_rehash = is_valid && self.needs_rehash(&parsed);
        Ok(VerifyResult {
            is_valid,
            needs_rehash,
        })
    }
}

#[derive(Default)]
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

    fn verify(&self, password: &str, hash: &str) -> crate::Result<VerifyResult> {
        Ok(VerifyResult {
            is_valid: self.hash(password)? == hash,
            needs_rehash: false,
        })
    }
}

#[derive(Default)]
pub struct DummyHasher {}

impl Hasher for DummyHasher {
    fn hash(&self, password: &str) -> crate::Result<String> {
        Ok(password.to_string())
    }

    fn verify(&self, password: &str, hash: &str) -> crate::Result<VerifyResult> {
        Ok(VerifyResult {
            is_valid: password == hash,
            needs_rehash: false,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_argon_hasher() {
        let cfg: Argon2Config = Default::default();
        let hasher = super::Argon2Hasher::new(cfg.clone());
        let password = "password";
        let hash = hasher.hash(&password).unwrap();

        let result = hasher.verify(&password, &hash).unwrap();
        assert!(result.is_valid);
        assert!(!result.needs_rehash);

        let result = hasher.verify("wrongpassword", &hash).unwrap();
        assert!(!result.is_valid);
        assert!(!result.needs_rehash);

        let mut newcfg = cfg.clone();

        newcfg.memory_kib = 131072; // 128 MiB
        let new_hasher = super::Argon2Hasher::new(newcfg);
        let result = new_hasher.verify(&password, &hash).unwrap();
        assert!(result.is_valid);
        assert!(result.needs_rehash);

        let mut newcfg = cfg.clone();
        newcfg.version = 0x10;
        let new_hasher = super::Argon2Hasher::new(newcfg);
        let result = new_hasher.verify(&password, &hash).unwrap();
        assert!(result.is_valid);
        assert!(result.needs_rehash);
    }

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
