use xxhash_rust::xxh3::{xxh3_64, xxh3_64_with_secret};

const MIN_SECRET_LEN: usize = 256;

/// Provides a hash for a passed in string slice using the `xxh3` hasher
/// which is currently the fastest quality hasher available to userland. It
/// generates a 64-bit hash but should not be confused with an earlier **xxhash**
/// algorithm `XXH64`. [More Info](https://cppget.org/xxhash?q=testing).
///
/// Note: you may optionally provide a secret as well to help obfuscate
/// the underlying document but xxHash is _not_ considered a cryptographic
/// hash.
pub fn hash(content: &str, secret: Option<&str>) -> u64 {
    match secret {
        Some(secret) => {
            if secret.len() >= MIN_SECRET_LEN {
                xxh3_64_with_secret(content.as_bytes(), secret.as_bytes())
            } else {
                panic!("Hashing with a secret requires that the secret be at least {} characters long but the secret provided was only {} characters.", MIN_SECRET_LEN, secret.len());
            }
        }
        None => xxh3_64(content.as_bytes()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn consistency_without_secret() {
        let content = String::from("There I was, There I was, ... in the Congo");
        let h = hash(&content, None);
        // test result is consistent
        for _i in [0..100] {
            let r = hash(&content, None);
            assert_eq!(h, r);
        }
    }

    #[test]
    fn consistency_with_secret() {
        let content = String::from("There I was, There I was, ... in the Congo");
        use rand::distributions::{Alphanumeric, DistString};
        let secret = Alphanumeric.sample_string(&mut rand::thread_rng(), MIN_SECRET_LEN);
        let h = hash(&content, Some(&secret));
        for _i in [0..100] {
            let r = hash(&content, Some(&secret));
            assert_eq!(h, r);
        }
    }
}
