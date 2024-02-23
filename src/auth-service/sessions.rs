use std::collections::HashMap;

pub trait Sessions {
    fn create_session(&mut self, user_uuid: &str) -> String;
    fn delete_session(&mut self, user_uuid: &str) -> String;
}

#[derive(Default)]
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
