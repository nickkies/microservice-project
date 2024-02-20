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

pub struct UserImpl {
    uuid_to_user: HashMap<String, User>,
    username_to_user: HashMap<String, User>,
}

impl Users for UserImpl {
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
