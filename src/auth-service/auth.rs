use std::sync::Mutex;

use tonic::{Request, Response, Status};

use crate::{sessions::Sessions, users::Users};

use authentication::auth_server::Auth;
use authentication::{
    SignInRequest, SignInResponse, SignOutRequest, SignOutResponse, SignUpRequest, SignUpResponse,
    StatusCode,
};

pub mod authentication {
    tonic::include_proto!("authentication");
}

pub struct AuthService {
    users_service: Box<Mutex<dyn Users + Send + Sync>>,
    sessions_service: Box<Mutex<dyn Sessions + Send + Sync>>,
}

impl AuthService {
    pub fn new(
        users_service: Box<Mutex<dyn Users + Send + Sync>>,
        sessions_service: Box<Mutex<dyn Sessions + Send + Sync>>,
    ) -> Self {
        Self {
            users_service,
            sessions_service,
        }
    }
}

#[tonic::async_trait]
impl Auth for AuthService {
    async fn sign_in(
        &self,
        request: Request<SignInRequest>,
    ) -> Result<Response<SignInResponse>, Status> {
        println!("Got request: {request:?}");

        let req = request.into_inner();

        let result = self
            .users_service
            .lock()
            .expect("lock should not be poisoned")
            .get_user_uuid(req.username, req.password);

        let user_uuid = match result {
            Some(uuid) => uuid,
            None => {
                return Ok(Response::new(SignInResponse {
                    status_code: StatusCode::Failure.into(),
                    user_uuid: "".to_string(),
                    session_token: "".to_string(),
                }));
            }
        };

        let session_token = self
            .sessions_service
            .lock()
            .expect("lock should not be poisoned")
            .create_session(&user_uuid);

        Ok(Response::new(SignInResponse {
            status_code: StatusCode::Success.into(),
            user_uuid,
            session_token,
        }))
    }

    async fn sign_up(
        &self,
        request: Request<SignUpRequest>,
    ) -> Result<Response<SignUpResponse>, Status> {
        println!("Got request: {request:?}");

        let req = request.into_inner();

        let result = self
            .users_service
            .lock()
            .expect("lock should not be poisoned")
            .create_user(req.username, req.password);

        match result {
            Ok(_) => Ok(Response::new(SignUpResponse {
                status_code: StatusCode::Success.into(),
            })),
            Err(_) => Ok(Response::new(SignUpResponse {
                status_code: StatusCode::Failure.into(),
            })),
        }
    }

    async fn sign_out(
        &self,
        request: Request<SignOutRequest>,
    ) -> Result<Response<SignOutResponse>, Status> {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use crate::{sessions::SessionsImpl, users::UsersImpl};

    use super::*;

    #[tokio::test]
    async fn sign_in_should_fail_if_user_not_found() {
        let users_service = Box::new(Mutex::new(UsersImpl::default()));
        let sessions_service = Box::new(Mutex::new(SessionsImpl::default()));
        let auth_service = AuthService::new(users_service, sessions_service);

        let request = tonic::Request::new(SignInRequest {
            username: "username".to_string(),
            password: "password".to_string(),
        });

        let result = auth_service.sign_in(request).await.unwrap().into_inner();

        assert_eq!(result.status_code, StatusCode::Failure.into());
        assert!(result.user_uuid.is_empty());
        assert!(result.session_token.is_empty());
    }

    #[tokio::test]
    async fn sign_in_should_fail_if_incorrect_password() {
        let mut users_service = UsersImpl::default();
        let _ = users_service.create_user("username".to_string(), "password".to_string());

        let users_service = Box::new(Mutex::new(users_service));
        let sessions_service = Box::new(Mutex::new(SessionsImpl::default()));
        let auth_srvice = AuthService::new(users_service, sessions_service);

        let request = tonic::Request::new(SignInRequest {
            username: "username".to_string(),
            password: "wrong password".to_string(),
        });

        let result = auth_srvice.sign_in(request).await.unwrap().into_inner();

        assert_eq!(result.status_code, StatusCode::Failure.into());
        assert!(result.user_uuid.is_empty());
        assert!(result.session_token.is_empty());
    }

    #[tokio::test]
    async fn sign_in_should_succeed() {
        let (username, password) = ("username".to_string(), "password".to_string());
        let mut users_service = UsersImpl::default();
        let _ = users_service.create_user(username.clone(), password.clone());

        let users_service = Box::new(Mutex::new(users_service));
        let sessions_service = Box::new(Mutex::new(SessionsImpl::default()));
        let auth_service = AuthService::new(users_service, sessions_service);

        let request = tonic::Request::new(SignInRequest { username, password });

        let result = auth_service.sign_in(request).await.unwrap().into_inner();

        assert_eq!(result.status_code, StatusCode::Success.into());
        assert!(!result.user_uuid.is_empty());
        assert!(!result.session_token.is_empty());
    }

    #[tokio::test]
    async fn sign_up_should_fail_if_username_exists() {
        let (username, password) = ("username".to_string(), "password".to_string());
        let mut users_service = UsersImpl::default();
        let _ = users_service.create_user(username.clone(), password.clone());

        let users_service = Box::new(Mutex::new(users_service));
        let sessions_service = Box::new(Mutex::new(SessionsImpl::default()));
        let auth_service = AuthService::new(users_service, sessions_service);

        let request = tonic::Request::new(SignUpRequest { username, password });

        let result = auth_service.sign_up(request).await.unwrap().into_inner();

        assert_eq!(result.status_code, StatusCode::Failure.into());
    }

    #[tokio::test]
    async fn sign_up_should_succeed() {
        let users_service = Box::new(Mutex::new(UsersImpl::default()));
        let sessions_service = Box::new(Mutex::new(SessionsImpl::default()));
        let auth_service = AuthService::new(users_service, sessions_service);

        let request = tonic::Request::new(SignUpRequest {
            username: "username".to_string(),
            password: "password".to_string(),
        });

        let result = auth_service.sign_up(request).await.unwrap().into_inner();

        assert_eq!(result.status_code, StatusCode::Success.into());
    }

    #[tokio::test]
    async fn sign_out_should_succeed() {
        let users_service = Box::new(Mutex::new(UsersImpl::default()));
        let sessions_service = Box::new(Mutex::new(SessionsImpl::default()));
        let auth_service = AuthService::new(users_service, sessions_service);

        let request = tonic::Request::new(SignOutRequest {
            session_token: "".to_string(),
        });

        let result = auth_service.sign_out(request).await.unwrap().into_inner();

        assert_eq!(result.status_code, StatusCode::Success.into());
    }
}
