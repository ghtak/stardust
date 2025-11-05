use std::env;

pub fn manifest_dir() -> crate::Result<String> {
    // std::env::var() 함수를 사용하여 환경 변수 값을 읽습니다.
    env::var("CARGO_MANIFEST_DIR")
        .map_err(|e| crate::Error::LoadError(e.into()))
}
