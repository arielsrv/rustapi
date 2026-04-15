use crate::errors::Error;
use crate::models::user::hash_password;
use crate::models::user::User;
use crate::settings::SETTINGS;
use crate::utils::models::ModelExt;
use crate::utils::token;
use std::time::{SystemTime, UNIX_EPOCH};

pub async fn create_user<T: AsRef<str>>(email: T) -> Result<User, Error> {
    let name = "Nahuel";
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or_default();
    let password = format!("TestPassword1!{}", timestamp);

    let password_hash = hash_password(&password).await?;
    let user = User::new(name, email.as_ref(), password_hash);
    let user = User::create(user).await?;

    Ok(user)
}

pub async fn create_user_token(user: User) -> Result<String, Error> {
    let secret = SETTINGS.auth.secret.as_str();
    let token = token::create(user, secret).unwrap();

    Ok(token)
}
