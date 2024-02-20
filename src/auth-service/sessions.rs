use std::collections::HashMap;

pub trait Sessions {
    fn create_session(&mut self, user_uuid: &str) -> String;
    fn delete_session(&mut self, user_uuid: &str) -> String;
}

pub struct SessionsImpl {
    uuid_to_ssession: HashMap<String, String>,
}

impl Sessions for SessionsImpl {
    fn create_session(&mut self, user_uuid: &str) -> String {
        todo!();
    }

    fn delete_session(&mut self, user_uuid: &str) -> String {
        todo!();
    }
}
