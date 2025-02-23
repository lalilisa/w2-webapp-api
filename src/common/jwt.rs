use std::collections::HashMap;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Serialize, Deserialize};
use chrono::{Utc, Duration};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde_json::json;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
   pub sub: String, // The username (subject)
   pub user_id: String,
   pub username: String,
   // pub exp: String,  // Expiry timestamp
}

const SECRET_KEY: &str = "your-secret-key"; // Change this to a secure key


pub fn create_jwt(claims: HashMap<String, String>) -> Result<String, jsonwebtoken::errors::Error> {
    let mut claims_json = json!(claims); // Convert HashMap to JSON

    // Add expiration timestamp (24 hours from now)
    claims_json["exp"] = json!(
        Utc::now()
            .checked_add_signed(Duration::hours(24))
            .expect("Invalid time")
            .timestamp()
    );

    let token = encode(
        &Header::default(),
        &claims_json,
        &EncodingKey::from_secret(SECRET_KEY.as_ref()),
    )?;

    Ok(token)
}



pub fn verify_jwt(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let decoded = decode::<Claims>(
        token,
        &DecodingKey::from_secret(SECRET_KEY.as_ref()),
        &Validation::default(),
    )?;
    Ok(decoded.claims)
}
