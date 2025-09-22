pub mod password {

    use argon2::{
        Argon2,
        password_hash::{
            PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng,
        },
    };

    pub fn hash(password: &String) -> Result<String, argon2::password_hash::Error> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();

        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)?
            .to_string();

        Ok(password_hash)
    }

    pub fn verify(
        password: &String,
        password_hash: &String,
    ) -> Result<bool, argon2::password_hash::Error> {
        let parsed = PasswordHash::new(&password_hash)?;

        let argon2 = Argon2::default();

        Ok(argon2.verify_password(password.as_bytes(), &parsed).is_ok())
    }
}
