use dotenvy::dotenv;
use lazy_static::lazy_static;
use std::env as std_env;

pub const TOKEN_TTL_SECONDS: i64 = 600; // Token valid for 10 minutes
pub const JWT_COOKIE_NAME: &str = "jwt";

lazy_static! {
    pub static ref JWT_SECRET: String = set_token();
    pub static ref DATABASE_URL: String = set_database_url();
    pub static ref MYSQL_SERVER_URL: String = set_mysql_server_url();
    pub static ref MYSQL_PASSWORD: String = set_mysql_password();
    pub static ref MYSQL_ROOT_PASSWORD: String = set_mysql_root_password();
}

pub mod env {
    pub const JWT_SECRET_ENV_VAR: &str = "JWT_SECRET";
    pub const DATABASE_URL_ENV_VAR: &str = "DATABASE_URL";
    pub const MYSQL_SERVER_URL_ENV_VAR: &str = "MYSQL_SERVER_URL";
    pub const MYSQL_PASSWORD_ENV_VAR: &str = "MYSQL_PASSWORD";
    pub const MYSQL_ROOT_PASSWORD_ENV_VAR: &str = "MYSQL_ROOT_PASSWORD";
}

pub mod prod {
    pub const APP_ADDRESS: &str = "0.0.0.0:3000";
}

pub mod test {
    pub const APP_ADDRESS: &str = "127.0.0.1:0";
}

fn set_token() -> String {
    dotenv().ok();

    let secret = std_env::var(env::JWT_SECRET_ENV_VAR).expect("JWT_SECRET must be set.");
    if secret.is_empty() {
        panic!("JWT_SECRET must not be empty.");
    }

    secret
}

fn set_mysql_server_url() -> String {
    dotenv().ok();

    let secret =
        std_env::var(env::MYSQL_SERVER_URL_ENV_VAR).expect("MYSQL_SERVER_URL must be set.");
    if secret.is_empty() {
        panic!("MYSQL_SERVER_URL must not be empty.");
    }

    secret
}

fn set_database_url() -> String {
    dotenv().ok();

    let secret = std_env::var(env::DATABASE_URL_ENV_VAR).expect("DATABASE_URL must be set.");
    if secret.is_empty() {
        panic!("DATABASE_URL must not be empty.");
    }

    secret
}

fn set_mysql_password() -> String {
    dotenv().ok();

    let secret = std_env::var(env::MYSQL_PASSWORD_ENV_VAR).expect("MYSQL_PASSWORD must be set.");
    if secret.is_empty() {
        panic!("MYSQL_PASSWORD must not be empty.");
    }

    secret
}

fn set_mysql_root_password() -> String {
    dotenv().ok();

    let secret =
        std_env::var(env::MYSQL_ROOT_PASSWORD_ENV_VAR).expect("MYSQL_ROOT_PASSWORD must be set.");
    if secret.is_empty() {
        panic!("MYSQL_ROOT_PASSWORD must not be empty.");
    }

    secret
}
