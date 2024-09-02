# Usage of Private Key PEM Files

This document explains how to generate and use `private_key.pem` and `public_key.pem` files.

## Generating RSA Private Key

To generate a private key using the RSA (Rivest-Shamir-Adleman) encryption algorithm, use the following commands:

# Generate RSA Private Key (2048-bit)
```bash
openssl genpkey -algorithm RSA -out private_rsa_key.pem -pkeyopt rsa_keygen_bits:2048
```

# Extract the Public Key from the Private Key
```bash
openssl rsa -pubout -in private_rsa_key.pem -out public_rsa_key.pem
```
# Using RSA Private Key
You can use the generated private_rsa_key.pem and public_rsa_key.pem files for signing and verifying JSON Web Tokens (JWT).

# Example usage:

```rust
let private_key = include_bytes!("../private_rsa_key.pem");
let public_key = include_bytes!("../public_rsa_key.pem");

let token = create_jwt(&claims, private_key, JwtAlgorithm::RS256).expect("Failed to create token");
let decoded_claims = validate_jwt(&token, public_key, JwtAlgorithm::RS256).expect("Failed to validate token");
```
# Example usage:
If you do not want to use real keys in a test environment, you can use mock keys like this:

```rust
let private_key = b"mock_private_key";
let public_key = b"mock_public_key";
```
# Important Notes
Security: Never store your private key as plain text within your code. Instead, use environment variables, secure file storage, or hardware wallets.
Backup: Ensure you securely back up your private key. If you lose your private key, you will lose access to your wallet.
Test Networks: Never use real mainnet keys in development and testing processes. Instead, use testnet networks.




