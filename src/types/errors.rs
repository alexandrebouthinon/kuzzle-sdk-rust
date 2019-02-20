use std::error;
use std::fmt;

/// A KuzzleError is an error specific to Kuzzle backend. It's often seen in types::Response
/// when request failed. It allow you to create your own througth the `new` constructor.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct KuzzleError {
    status: Option<u16>,
    message: String,
    stack: Option<String>,
}

impl KuzzleError {
    /// Returns a custom KuzzleError with the given status and message
    ///
    /// # Arguments
    ///
    /// * `status` - An `Option<u16>` that can be `Some(value)` or `None`.
    /// Depends you want to specify a custom status code.
    /// * `message` - A string slice that holds your custom error message.
    ///
    /// # Example
    ///
    /// ```
    /// use kuzzle_sdk::types::KuzzleError;
    /// let custom_not_found = KuzzleError::new(Some(404), "A custom not found error");
    /// // or
    /// let custom_not_found = KuzzleError::new(None, "A custom error without status code");
    /// ```
    pub fn new(status: Option<u16>, message: &str) -> KuzzleError {
        KuzzleError {
            status,
            message: message.to_string(),
            stack: None,
        }
    }

    /// KuzzleError status getter.
    pub fn status(&self) -> Option<u16> {
        self.status
    }

    /// KuzzleError message getter.
    pub fn message(&self) -> &String {
        &self.message
    }

    /// KuzzleError stack getter.
    pub fn stack(&self) -> String {
        match &self.stack {
            Some(stack) => stack.clone(),
            None => String::new(),
        }
    }
}

impl error::Error for KuzzleError {}

impl fmt::Display for KuzzleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Provide error description following https://docs-v2.kuzzle.io/api/1/errors
        let description: &str = match &self.status {
            None => "UnidentifiedError",
            Some(status) => match &status {
                206 => "PartialError",
                400 => "BadRequestError",
                401 => "UnauthorizedError",
                403 => "ForbiddenError",
                404 => "NotFoundError",
                412 => "PreconditionError",
                413 => "SizeLimitError",
                500 => "InternalError",
                503 => "ServiceUnavailableError",
                504 => "GatewayTimeoutError",
                _ => "CustomError",
            },
        };

        // Check `self.stack` presence.
        match &self.stack {
            // If Some(stack) drop `self.message`
            // since there is an error message in `self.stack`...
            Some(stack) => write!(f, "[{}] {}", self.status.unwrap(), stack),
            // ... else take `self.message`.
            None => write!(
                f,
                "[{}] {} : {}",
                self.status.unwrap(),
                description,
                self.message
            ),
        }
    }
}

/// SDK relative error. Triggered when function arguments mismatched, bad format...
#[derive(Debug, Clone, PartialEq)]
pub struct SdkError {
    cause: String,
    message: String,
}

impl SdkError {
    /// Returns a custom SdkError with the given cause and message
    ///
    /// # Arguments
    ///
    /// * `cause` - A `&str` containing name of the function, method or controller
    /// that triggered the error.
    /// * `message` - A `&str` slice that holds your custom error message.
    ///
    /// # Example
    ///
    /// ```
    /// use kuzzle_sdk::types::SdkError;
    /// let fake_error = SdkError::new("FakeController", "A fake error");
    /// assert_eq!(
    ///     format!("{}", fake_error),
    ///     format!("[{}] {}", fake_error.cause(), fake_error.message())
    /// );
    /// ```
    pub fn new(cause: &str, message: &str) -> SdkError {
        SdkError {
            cause: cause.to_string(),
            message: message.to_string(),
        }
    }

    pub fn cause(&self) -> &String {
        &self.cause
    }

    pub fn message(&self) -> &String {
        &self.message
    }
}

impl error::Error for SdkError {}

impl fmt::Display for SdkError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}] {}", self.cause, self.message)
    }
}
