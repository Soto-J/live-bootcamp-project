#[derive(PartialEq, Debug)]
pub enum AuthAPIError {
    UserAlreadyExists,
    InvalidCredentials,
    IncorrectCredentials,
    UnexpectedError,
    // UnprocessableContent,
    InvalidToken,
    MissingToken,
}
