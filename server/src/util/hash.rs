use bcrypt::{hash, verify};

pub fn hash_password(input: &str) -> String {
    let password = hash(input, 6).unwrap();

    assert!(verify(input.to_string(), &password).unwrap());

    return password;
}
