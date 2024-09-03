# Rust-Oauth2
=======
# Web3 OAuth2 Library

This Rust library provides OAuth 2.0 support for Web3 applications. It includes various OAuth flows, Web3 client integration, multi-signature support, and hardware wallet integration.

## Features

- **PKCE (Proof Key for Code Exchange)**
- **Device Flow**
- **Multi-chain Support**
- **Smart Contract Audit**
- **Multi-signature**
- **Hardware Wallet Integration**

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
web3-oauth2 = "0.2.0"
```
## Usage

## PKCE Example

```bash
use rust_oauth2::pkce::PkceCode;

fn main() {
    let pkce = PkceCode::new().expect("Failed to generate PKCE code");
    println!("Code Verifier: {}", pkce.code_verifier);
    println!("Code Challenge: {}", pkce.code_challenge);
}
```
## Device Flow Example

```bash
use web3_oauth2::device_flow::DeviceFlow;

#[tokio::main]
async fn main() {
    let device_flow = DeviceFlow::new(
        "client_id_example",
        "https://example.com/device_code",
        "https://example.com/token",
    );

    let device_code_response = device_flow.request_device_code().await.expect("Failed to request device code");

    println!("User Code: {}", device_code_response.user_code);
    println!("Verification URI: {}", device_code_response.verification_uri);

    // Poll for the token
    let token = device_flow.poll_for_token(&device_code_response.device_code, device_code_response.interval).await;

    match token {
        Ok(Some(token)) => println!("Token: {}", token),
        Ok(None) => println!("Authorization pending"),
        Err(e) => println!("Error: {}", e),
    }
}
```

## FAQ

## Common Errors and Solutions
Error: Failed to exchange authorization code

This error occurs when the authorization code is invalid or expired. Ensure that the code is correct and has not been used or expired.
Error: Chain not found

This error occurs when the specified blockchain network is not found in the MultiChainClient. Verify that the network name and URL are correctly configured.

## under development 

be carefull before run the code or contact to me on TG:(view profile)



