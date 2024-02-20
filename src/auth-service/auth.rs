use std::sync::Mutex;

use tonic::{Request, Response, Status};

use crate::{sessions::Sessions, users::Users};

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
        users_service: Box<Mutex<Users + Send + Sync>>,
        sessions_service: Box<Mutex<Sessions + Send + Sync>>,
    ) -> Self {
        Self {
            users_service,
            sessions_service,
        }
    }
}

impl Auth for AuthService {
    fn sign_in(&self, request: Request<SignInRequest>) -> Result<Response<SignInResponse>, Status> {
        todo!();
    }

    fn sign_up(&self, request: Request<SignUpRequest>) -> Result<Response<SignUpResponse>, Status> {
        todo!();
    }

    fn sign_out(
        &self,
        request: Request<SignOutRequest>,
    ) -> Result<Response<SignOutResponse>, Status> {
        todo!();
    }
}
