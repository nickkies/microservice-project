use std::collections::HashMap;

use pbkdf2::{
    password_hash::{PasswordHasher, SaltString},
    Pbkdf2,
};
use rand_core::OsRng;
use uuid::Uuid;

pub trait Users {
    fn create_user(&mut self, username: String, password: String) -> Result<(), String>;
    fn get_user_uuid(&self, username: String, password: String) -> Option<String>;
    fn delete_user(&mut self, user_uuid: String);
}

#[derive(Clone)]
pub struct User {
    user_uuid: String,
    username: String,
    password: String,
}

#[derive(Default)]
pub struct UsersImpl {
    uuid_to_user: HashMap<String, User>,
    username_to_user: HashMap<String, User>,
}

impl Users for UsersImpl {
    fn create_user(&mut self, username: String, password: String) -> Result<(), String> {
        if self.username_to_user.contains_key(&username) {
            return Err("Username already exists. Unable to create user.".to_string());
        }

        let salt = SaltString::generate(&mut OsRng);

        let hashed_password = Pbkdf2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| format!("Failed to hash password.\n{e:?}"))?
            .to_string();

        let user = User {
            user_uuid: Uuid::new_v4().to_string(),
            username: username.clone(),
            password: hashed_password,
        };

        self.username_to_user.insert(username, user.clone());
        self.uuid_to_user.insert(user.user_uuid.clone(), user);

        Ok(())
    }

    fn get_user_uuid(&self, username: String, password: String) -> Option<String> {
        todo!();
    }

    fn delete_user(&mut self, user_uuid: String) {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_user() {
        let mut service = UsersImpl::default();
        service
            .create_user("username".to_string(), "password".to_string())
            .expect("should create user");

        assert!(service
            .get_user_uuid("username".to_string(), "password".to_string())
            .is_some());
    }

    #[test]
    fn should_fail_creating_user_with_existing_username() {
        let mut service = UsersImpl::default();
        service
            .create_user("username".to_string(), "password".to_string())
            .expect("should create user");

        let result = service.create_user("username".to_string(), "password".to_string());

        assert!(result.is_err());
    }
}
