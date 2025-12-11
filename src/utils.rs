use argon2::{Argon2,PasswordHasher, password_hash::{SaltString, rand_core::OsRng}};

pub fn hashing_password(password:String)->String{
    let argon2 = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);
    let password = password.trim();
    let hashed_password =  argon2.hash_password(&password.as_bytes(), &salt).expect("Could not hash the password ");
    hashed_password.to_string()
}