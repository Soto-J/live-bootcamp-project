use dotenvy::dotenv;
use lazy_static::lazy_static;
use secrecy::Secret;
use std::env as std_env;

pub const TOKEN_TTL_SECONDS: i64 = 600; // Token valid for 10 minutes

pub const JWT_COOKIE_NAME: &str = "jwt";

pub const DEFAULT_REDIS_HOSTNAME: &str = "127.0.0.1";

lazy_static! {
    pub static ref JWT_SECRET: Secret<String> = set_token();
    pub static ref DATABASE_URL: Secret<String> = set_database_url();
    pub static ref MYSQL_SERVER_URL: Secret<String> = set_mysql_server_url();
    pub static ref MYSQL_PASSWORD: Secret<String> = set_mysql_password();
    pub static ref MYSQL_ROOT_PASSWORD: Secret<String> = set_mysql_root_password();
    pub static ref REDIS_HOST_NAME: Secret<String> = set_redis_host();
    pub static ref POSTMARK_AUTH_TOKEN: Secret<String> = set_postmark_auth_token();
}

pub mod env {
    pub const JWT_SECRET_ENV_VAR: &str = "JWT_SECRET";
    pub const DATABASE_URL_ENV_VAR: &str = "DATABASE_URL";
    pub const MYSQL_SERVER_URL_ENV_VAR: &str = "MYSQL_SERVER_URL";
    pub const MYSQL_PASSWORD_ENV_VAR: &str = "MYSQL_PASSWORD";
    pub const MYSQL_ROOT_PASSWORD_ENV_VAR: &str = "MYSQL_ROOT_PASSWORD";
    pub const REDIS_HOST_NAME_ENV_VAR: &str = "REDIS_HOST_NAME";
    pub const POSTMARK_AUTH_TOKEN_ENV_VAR: &str = "POSTMARK_AUTH_TOKEN";
}

pub mod prod {
    pub const APP_ADDRESS: &str = "0.0.0.0:3000";

    pub mod email_client {
        use std::time::Duration;

        pub const BASE_URL: &str = "https://api.postmarkapp.com/";
        pub const SENDER: &str = "john@johnsoto.dev";
        pub const TIMEOUT: Duration = std::time::Duration::from_secs(10);
    }
}

pub mod test {
    pub const APP_ADDRESS: &str = "127.0.0.1:0";

    pub mod email_client {
        use std::time::Duration;

        pub const SENDER: &str = "test@email.com";
        pub const TIMEOUT: Duration = std::time::Duration::from_millis(200);
    }
}

fn set_token() -> Secret<String> {
    dotenv().ok();

    let secret = std_env::var(env::JWT_SECRET_ENV_VAR).expect("JWT_SECRET must be set.");
    if secret.is_empty() {
        panic!("JWT_SECRET must not be empty.");
    }

    Secret::new(secret)
}

fn set_mysql_server_url() -> Secret<String> {
    dotenv().ok();

    let secret =
        std_env::var(env::MYSQL_SERVER_URL_ENV_VAR).expect("MYSQL_SERVER_URL must be set.");
    if secret.is_empty() {
        panic!("MYSQL_SERVER_URL must not be empty.");
    }

    Secret::new(secret)
}

fn set_database_url() -> Secret<String> {
    dotenv().ok();

    let secret = std_env::var(env::DATABASE_URL_ENV_VAR).expect("DATABASE_URL must be set.");
    if secret.is_empty() {
        panic!("DATABASE_URL must not be empty.");
    }

    Secret::new(secret)
}

fn set_mysql_password() -> Secret<String> {
    dotenv().ok();

    let secret = std_env::var(env::MYSQL_PASSWORD_ENV_VAR).expect("MYSQL_PASSWORD must be set.");
    if secret.is_empty() {
        panic!("MYSQL_PASSWORD must not be empty.");
    }

    Secret::new(secret)
}

fn set_mysql_root_password() -> Secret<String> {
    dotenv().ok();

    let secret =
        std_env::var(env::MYSQL_ROOT_PASSWORD_ENV_VAR).expect("MYSQL_ROOT_PASSWORD must be set.");
    if secret.is_empty() {
        panic!("MYSQL_ROOT_PASSWORD must not be empty.");
    }

    Secret::new(secret)
}

fn set_redis_host() -> Secret<String> {
    dotenv().ok();
    let secret =
        std_env::var(env::REDIS_HOST_NAME_ENV_VAR).unwrap_or(DEFAULT_REDIS_HOSTNAME.to_owned());

    Secret::new(secret)
}

fn set_postmark_auth_token() -> Secret<String> {
    dotenv().ok();
    Secret::new(
        std_env::var(env::POSTMARK_AUTH_TOKEN_ENV_VAR).expect("POSTMARK_AUTH_TOKEN must be set."),
    )
}
