#[derive(PartialEq, Debug)]
pub enum AuthAPIError {
    UserAlreadyExists,
    InvalidCredentials,
    IncorrectCredentials,
    MissingToken,
    InvalidToken,
    Missing2FA,
    UnexpectedError,
}
