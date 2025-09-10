mod authenticate;
mod cookie;
mod jwt;
pub mod middleware;

// You'll need to implement this function using a password hashing library
pub fn verify_password(password: &str, hash: &str) -> bool {
    // Example with bcrypt (you'll need to add bcrypt to Cargo.toml):
    // bcrypt::verify(password, hash).unwrap_or(false)

    // Placeholder - replace with actual password verification
    password == hash
}
