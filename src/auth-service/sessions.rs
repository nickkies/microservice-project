use std::collections::HashMap;

use uuid::Uuid;

pub trait Sessions {
    fn create_session(&mut self, user_uuid: &str) -> String;
    fn delete_session(&mut self, user_uuid: &str);
}

#[derive(Default)]
pub struct SessionsImpl {
    uuid_to_ssession: HashMap<String, String>,
}

impl Sessions for SessionsImpl {
    fn create_session(&mut self, user_uuid: &str) -> String {
        let session = Uuid::new_v4().to_string();
        self.uuid_to_ssession
            .insert(user_uuid.to_string(), session.clone());
        session
    }

    fn delete_session(&mut self, user_uuid: &str) {
        self.uuid_to_ssession.remove(user_uuid);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_session() {
        let mut service = SessionsImpl::default();
        assert_eq!(service.uuid_to_ssession.len(), 0);

        let user_uuid = "123";
        let session = service.create_session(user_uuid);
        assert_eq!(service.uuid_to_ssession.len(), 1);
        assert_eq!(service.uuid_to_ssession.get(user_uuid).unwrap(), &session);
    }

    #[test]
    fn should_delete_session() {
        let mut service = SessionsImpl::default();

        service.create_session("123");
        service.delete_session("123");

        assert_eq!(service.uuid_to_ssession.len(), 0);
    }
}
