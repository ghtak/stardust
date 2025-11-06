use std::env;

pub fn manifest_dir() -> crate::Result<String> {
    // Read the environment variable using `std::env::var()`.
    env::var("CARGO_MANIFEST_DIR")
        .map_err(|e| crate::Error::LoadError(e.into()))
}

pub fn workspace_dir() -> crate::Result<String> {
    let manifest_dir = manifest_dir()?;
    let workspace_dir = std::path::Path::new(&manifest_dir)
        .parent()
        .ok_or_else(|| {
            crate::Error::LoadError(anyhow::anyhow!(
                "Failed to get parent directory"
            ))
        })?
        .to_string_lossy()
        .to_string();
    Ok(workspace_dir)
}

pub fn current_exe_dir() -> crate::Result<String> {
    env::current_exe()
        .map_err(|e| crate::Error::LoadError(e.into()))
        .map(|path| path.parent().unwrap().to_string_lossy().to_string())
}

pub fn generate_uid() -> String {
    uuid::Uuid::new_v4().as_simple().to_string()
}

pub fn contains<'a, C: ?Sized, T: 'a, U: ?Sized>(
    container: &'a C,
    element: &'a U,
) -> bool
where
    &'a C: IntoIterator<Item = &'a T>,
    T: PartialEq<U>,
{
    container.into_iter().any(|item| item == element)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manifest_dir() {
        println!("manifest {:?}", manifest_dir());
        println!("worspace {:?}", workspace_dir());
        println!("current exe {:?}", current_exe_dir());
    }

    #[test]
    fn test_contains() {
        let vec = vec![1, 2, 3, 4, 5];
        assert!(contains(&vec, &1));
        assert!(!contains(&vec, &6));

        let vec = vec!["a", "b", "c", "d", "e"];
        assert!(contains(&vec, &"a"));
        assert!(!contains(&vec, &"f"));

        let vec = vec!["a".to_string(), "b".to_string()];
        assert!(contains(&vec, "a"));
        assert!(!contains(&vec, "f"));
    }
}
