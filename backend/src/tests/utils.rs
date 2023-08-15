#[cfg(test)]
mod tests {
    use crate::utils::{server::connect, auth::{create_token, verify_token}};

    #[test]
    fn database_connection() {
        assert!(connect().is_ok());
    }

    #[test]
    fn token_creation_and_verification() { 
        let token = create_token();

        assert!(token.is_ok());

        let verified_token = verify_token(&token.unwrap());

        assert!(verified_token.is_ok());
    }
}
