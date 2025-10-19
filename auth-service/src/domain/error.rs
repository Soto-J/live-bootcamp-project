#[derive(PartialEq, Debug)]
pub enum AuthAPIError {
    UserAlreadyExists,
    InvalidCredentials,
    UnexpectedError,
    UnprocessableContent,
    Unauthorized,
    BadRequest,
}
