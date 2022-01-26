use base32;
use std::time::SystemTime;
use totp_rs::{Algorithm, TOTP};

pub fn encode(input: &Vec<u8>) -> String {
    return base32::encode(base32::Alphabet::RFC4648 { padding: false }, input);
}
pub fn decode(input: &str) -> Option<Vec<u8>> {
    return base32::decode(base32::Alphabet::RFC4648 { padding: false }, input);
}

/**
 * Generate a TOTP string from a base32 encoded secret
 */
pub fn generate_totp(secret: &Vec<u8>) -> String {
    let totp = TOTP::new(Algorithm::SHA1, 6, 1, 30, secret);
    let time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let otp = totp.generate(time);
    return otp;
}

pub fn display_service(service: &crate::models::Service) {
	println!("Service {} ({}): {}", service.name, service.id, encode(&service.secret));
}
pub fn display_service_totp(service: &crate::models::Service) {
	let totp = generate_totp(&service.secret);
	println!("Service {} ({}): {}", service.name, service.id, totp);
}