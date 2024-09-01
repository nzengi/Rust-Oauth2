// src/cli.rs

use clap::{App, Arg, SubCommand};
use web3_oauth2::{pkce::PkceCode, device_flow::DeviceFlow};
use tokio::runtime::Runtime;

fn main() {
    let matches = App::new("Web3 OAuth2 CLI")
        .version("0.2.0")
        .author("Your Name <you@example.com>")
        .about("CLI tool for Web3 OAuth2 operations")
        .subcommand(
            SubCommand::with_name("pkce")
                .about("Generate PKCE code")
        )
        .subcommand(
            SubCommand::with_name("device_flow")
                .about("Perform Device Flow")
                .arg(
                    Arg::with_name("client_id")
                        .required(true)
                        .help("Client ID for the OAuth application"),
                )
                .arg(
                    Arg::with_name("device_code_url")
                        .required(true)
                        .help("Device code endpoint URL"),
                )
                .arg(
                    Arg::with_name("token_url")
                        .required(true)
                        .help("Token endpoint URL"),
                )
        )
        .get_matches();

    if let Some(_) = matches.subcommand_matches("pkce") {
        let pkce = PkceCode::new().expect("Failed to generate PKCE code");
        println!("Code Verifier: {}", pkce.code_verifier);
        println!("Code Challenge: {}", pkce.code_challenge);
    }

    if let Some(matches) = matches.subcommand_matches("device_flow") {
        let client_id = matches.value_of("client_id").unwrap();
        let device_code_url = matches.value_of("device_code_url").unwrap();
        let token_url = matches.value_of("token_url").unwrap();

        let device_flow = DeviceFlow::new(client_id, device_code_url, token_url);

        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            let device_code_response = device_flow.request_device_code().await.expect("Failed to request device code");
            println!("User Code: {}", device_code_response.user_code);
            println!("Verification URI: {}", device_code_response.verification_uri);

            let token = device_flow.poll_for_token(&device_code_response.device_code, device_code_response.interval).await;

            match token {
                Ok(Some(token)) => println!("Token: {}", token),
                Ok(None) => println!("Authorization pending"),
                Err(e) => println!("Error: {}", e),
            }
        });
    }
}
