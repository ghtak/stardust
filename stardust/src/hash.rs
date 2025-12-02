#[async_trait::async_trait]
pub trait Hasher: Sync + Send {
    async fn hash(&self, password: &str) -> crate::Result<String>;
    async fn verify(&self, password: &str, hash: &str) -> crate::Result<bool>;
    async fn needs_rehash(&self, stored_hash: &str) -> crate::Result<bool>;
}

#[derive(Default)]
pub struct NoOpHasher;

#[async_trait::async_trait]
impl Hasher for NoOpHasher {
    async fn hash(&self, password: &str) -> crate::Result<String> {
        Ok(format!("noop:{}", password))
    }

    async fn verify(&self, password: &str, hash: &str) -> crate::Result<bool> {
        Ok(hash == format!("noop:{}", password))
    }

    async fn needs_rehash(&self, _stored_hash: &str) -> crate::Result<bool> {
        Ok(false)
    }
}

#[derive(Debug)]
pub struct Sha256Hasher;

#[async_trait::async_trait]
impl Hasher for Sha256Hasher {
    async fn hash(&self, password: &str) -> crate::Result<String> {
        use base64::prelude::*;
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(password.as_bytes());
        let result = hasher.finalize();
        Ok(format!("sha256:{}", BASE64_STANDARD.encode(result)))
    }

    async fn verify(&self, password: &str, hash: &str) -> crate::Result<bool> {
        let computed_hash = self.hash(password).await?;
        Ok(computed_hash == hash)
    }

    async fn needs_rehash(&self, _stored_hash: &str) -> crate::Result<bool> {
        Ok(false)
    }
}

pub struct Argon2Hasher {
    argon2: argon2::Argon2<'static>,
    config: crate::config::Argon2Config,
}

impl Argon2Hasher {
    pub fn new(config: crate::config::Argon2Config) -> crate::Result<Self> {
        let params = argon2::Params::new(
            config.memory_kib,
            config.iterations,
            config.parallelism,
            config.output_len.map(|v| v as usize),
        )
        .map_err(|e| {
            anyhow::anyhow!("Failed to create Argon2 params {:?}", e)
        })?;
        let argon2 = argon2::Argon2::new(
            config.algorithm.parse().unwrap_or(argon2::Algorithm::Argon2id),
            argon2::Version::try_from(config.version)
                .unwrap_or(argon2::Version::V0x13),
            params,
        );
        Ok(Self { argon2, config })
    }
}

#[async_trait::async_trait]
impl Hasher for Argon2Hasher {
    async fn hash(&self, password: &str) -> crate::Result<String> {
        use argon2::{PasswordHasher, password_hash::SaltString};
        use base64::prelude::*;
        let seed = BASE64_STANDARD_NO_PAD.encode(b"stardust");
        let salt = SaltString::from_b64(&seed).unwrap();
        let hash = self
            .argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| anyhow::anyhow!("argon2 hashing error {:?}", e))?
            .to_string();
        Ok(hash)
    }

    async fn verify(&self, password: &str, hash: &str) -> crate::Result<bool> {
        use argon2::password_hash::{PasswordHash, PasswordVerifier};
        let parsed = PasswordHash::new(hash)
            .map_err(|e| anyhow::anyhow!("argon2 parse error {:?}", e))?;
        Ok(self.argon2.verify_password(password.as_bytes(), &parsed).is_ok())
    }

    async fn needs_rehash(&self, stored_hash: &str) -> crate::Result<bool> {
        use argon2::password_hash::PasswordHash;
        let stored_hash = PasswordHash::new(stored_hash)
            .map_err(|e| anyhow::anyhow!("argon2 parse error {:?}", e))?;
        let required_algorithm = self
            .config
            .algorithm
            .parse()
            .unwrap_or(argon2::Algorithm::Argon2id)
            .ident();
        if stored_hash.algorithm != required_algorithm {
            return Ok(true);
        }
        if let Some(version) = stored_hash.version {
            if version != self.config.version {
                return Ok(true);
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
                return Ok(true);
            }
        }
        Ok(false)
    }
}

impl Default for crate::config::Argon2Config {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_noop_hasher() {
        let hasher = NoOpHasher::default();
        let password = "password";
        let hash = hasher.hash(password).await.unwrap();
        print!("Hash: {}, Len: {}\n", hash, hash.len());
        let is_valid = hasher.verify(password, &hash).await.unwrap();
        assert!(is_valid);
        let is_invalid = hasher.verify("wrong_password", &hash).await.unwrap();
        assert!(!is_invalid);
    }

    #[tokio::test]
    async fn test_sha256_hasher() {
        let hasher = Sha256Hasher;
        let password = "password";
        let hash = hasher.hash(password).await.unwrap();
        print!("Hash: {}, Len: {}\n", hash, hash.len());
        let is_valid = hasher.verify(password, &hash).await.unwrap();
        assert!(is_valid);
        let is_invalid = hasher.verify("wrong_password", &hash).await.unwrap();
        assert!(!is_invalid);
    }

    #[tokio::test]
    async fn test_argon_hasher() {
        let config = crate::config::Argon2Config::default();
        let hasher = super::Argon2Hasher::new(config.clone()).unwrap();
        let password = "password";
        let default_hash = hasher.hash(password).await.unwrap();

        let result = hasher.verify(&password, &default_hash).await.unwrap();
        let needs_rehash = hasher.needs_rehash(&default_hash).await.unwrap();
        assert!(result);
        assert!(!needs_rehash);

        let result =
            hasher.verify("wrongpassword", &default_hash).await.unwrap();
        let needs_rehash = hasher.needs_rehash(&default_hash).await.unwrap();
        assert!(!result);
        assert!(!needs_rehash);

        let mut newcfg = config.clone();
        newcfg.memory_kib = 131072;
        let new_hasher = super::Argon2Hasher::new(newcfg).unwrap();
        let result = new_hasher.verify(&password, &default_hash).await.unwrap();
        let needs_rehash =
            new_hasher.needs_rehash(&default_hash).await.unwrap();
        assert!(result);
        assert!(needs_rehash);

        let mut newcfg = config.clone();
        newcfg.version = 0x10;
        let new_hasher = super::Argon2Hasher::new(newcfg).unwrap();
        let result = new_hasher.verify(&password, &default_hash).await.unwrap();
        let needs_rehash =
            new_hasher.needs_rehash(&default_hash).await.unwrap();
        assert!(result);
        assert!(needs_rehash);
    }
}
