pub const EMAIL_MAX_LENGTH: usize = 254;
pub const PASSWORD_MIN_LENGTH: usize = 8;
pub const PASSWORD_MAX_LENGTH: usize = 256;

// TODO: Come up with better messages handling e.g. i18
pub const FIELD_REQUIRED_MESSAGE: &str = "This field is required";
pub const EMAIL_TOO_LONG_MESSAGE: &str = "Email must be at most 254 characters";
pub const INVALID_EMAIL_MESSAGE: &str = "Invalid email";
pub const PASSWORD_TOO_SHORT_MESSAGE: &str = "Password must be at least 8 characters";
pub const PASSWORD_TOO_LONG_MESSAGE: &str = "Password must be at most 256 characters";
pub const PASSWORD_MISMATCH_MESSAGE: &str = "Password doesn't match";
pub const EMAIL_IS_ALREADY_TAKEN_MESSAGE: &str = "Email is already taken";
pub const INVALID_CREDENTIALS_MESSAGE: &str = "Incorrect email or password";
