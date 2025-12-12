use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier, password_hash::{SaltString, rand_core::OsRng}};

pub fn hashing_password(password:String)->String{
    let argon2 = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);
    let password = password.trim();
    let hashed_password =  argon2.hash_password(&password.as_bytes(), &salt).expect("Could not hash the password ");
    hashed_password.to_string()
}

pub fn verify_password(password:String,hashed:String)->bool{
    let parsed_hash = PasswordHash::new(&hashed);
    if parsed_hash.is_err(){
        return false;
    }
    let argon2 = Argon2::default();
    argon2.verify_password(password.as_bytes(), &parsed_hash.unwrap()).is_ok()
}