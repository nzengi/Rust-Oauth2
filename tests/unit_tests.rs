#[cfg(test)]
mod tests {
    use rust_oauth2::jwt::{create_jwt, validate_jwt, Claims, JwtAlgorithm};
    use rust_oauth2::wallet::Wallet;
    use rust_oauth2::oauth::token::OAuthToken;
    use rust_oauth2::oauth::client_credentials::ClientCredentialsFlow;

    #[test]
    fn test_oauth_token_creation() {
        let token = OAuthToken::new("access_token_example".to_string(), "Bearer".to_string(), Some(3600));
        assert_eq!(token.token_type, "Bearer");
        assert_eq!(token.is_expired(), false);
    }

    #[test]
    fn test_wallet_sign_message() {
        let wallet = Wallet::new("0xAddress", "PrivateKeyHex").unwrap();
        let signature = wallet.sign_message("Test message").unwrap();
        assert!(!signature.is_empty());
    }

    #[test]
    fn test_jwt_rs256() {
        let claims = Claims {
            sub: "1234567890".to_owned(),
            company: "YourCompany".to_owned(),
            exp: 10000000000,
        };

        // let private_key = include_bytes!("../private_rsa_key.pem"); // Gerçek dosya
        let private_key = b"mock_private_key"; // Mock anahtar

        let token = create_jwt(&claims, private_key, JwtAlgorithm::RS256).expect("Failed to create token");

        // let public_key = include_bytes!("../public_rsa_key.pem"); // Gerçek dosya
        let public_key = b"mock_public_key"; // Mock anahtar

        let decoded_claims = validate_jwt(&token, public_key, JwtAlgorithm::RS256).expect("Failed to validate token");

        assert_eq!(decoded_claims.sub, claims.sub);
    }

    #[test]
    fn test_jwt_es256() {
        let claims = Claims {
            sub: "0987654321".to_owned(),
            company: "YourCompany".to_owned(),
            exp: 10000000000,
        };

        // let private_key = include_bytes!("../private_ec_key.pem"); // Gerçek dosya
        let private_key = b"mock_private_key"; // Mock anahtar

        let token = create_jwt(&claims, private_key, JwtAlgorithm::ES256).expect("Failed to create token");

        // let public_key = include_bytes!("../public_ec_key.pem"); // Gerçek dosya
        let public_key = b"mock_public_key"; // Mock anahtar

        let decoded_claims = validate_jwt(&token, public_key, JwtAlgorithm::ES256).expect("Failed to validate token");

        assert_eq!(decoded_claims.sub, claims.sub);
    }

    #[tokio::test]
    async fn test_client_credentials_flow() {
        let client_credentials = ClientCredentialsFlow::new(
            "client_id_example",
            "client_secret_example",
            "http://localhost/token",
        );

        let token = client_credentials.request_token().await;
        assert!(token.is_ok());
    }
}
