//! Authentication token types.

use derive_more::{AsRef, Deref, DerefMut};
use serde::{Deserialize, Serialize};

/// Authentication token for SimpleFIN access.
///
/// Tokens are opaque strings used to authenticate requests to the SimpleFIN server.
/// They are typically obtained through the claim flow after user authorization.
///
/// # Examples
///
/// ```
/// use simplefin_data::token::Token;
///
/// let token = Token::new("my_secret_token");
/// let json = serde_json::to_string(&token).unwrap();
/// assert_eq!(json, r#""my_secret_token""#);
/// ```
#[derive(Serialize, Deserialize, PartialEq, Debug, Deref, DerefMut, AsRef)]
#[as_ref(forward)]
pub struct Token(String);

impl Token {
    /// Creates a new token from a string.
    ///
    /// # Examples
    ///
    /// ```
    /// use simplefin_data::token::Token;
    ///
    /// let token = Token::new("abc123");
    /// ```
    pub fn new(token: impl AsRef<str>) -> Self {
        Token(token.as_ref().to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(r#""test_token""#, Token::new("test_token"))]
    fn test_token(#[case] input: &str, #[case] expected: Token) {
        let deserialized: Token = serde_json::from_str(input).unwrap();
        assert_eq!(deserialized, expected);

        let serialized = serde_json::to_string_pretty(&deserialized).unwrap();
        assert_eq!(serialized, input);
    }
}
