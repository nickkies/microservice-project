use auth::AuthService;

mod auth;
mod sessions;
mod users;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let users_service = todo!();
    let sessions_service = todo!();

    let auth_service = AuthService::new(users_service, sessions_service);

    Ok(())
}
