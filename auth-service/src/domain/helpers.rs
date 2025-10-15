pub fn validate_email(email: &str) -> Result<(), validator::ValidationError> {
    if !email.contains('@') {
        return Err(validator::ValidationError::new("invalid_email"));
    }
    Ok(())
}
