use std::collections::HashMap;

pub trait Users {
    fn create_user(&mut self, username: String, password: String) -> Result<(), String>;
    fn get_user_uuid(&self, username: String, password: String) -> Option<String>;
    fn delete_user(&mut self, user_uuid: String);
}

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
        todo!();
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
