use mailifica::{
    emails::CreateEmailBaseOptions,
    webhooks::Webhooks,
};
use hmac::{Hmac, Mac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

#[test]
fn test_create_email_options_builder() {
    let email = CreateEmailBaseOptions::new(
        "onboarding@empresa.ao",
        vec!["cliente@gmail.com"],
        "Boas-vindas",
    )
    .with_html("<strong>Olá!</strong>")
    .with_reply_to("suporte@empresa.ao");

    assert_eq!(email.from, "onboarding@empresa.ao");
    assert_eq!(email.to, vec!["cliente@gmail.com".to_string()]);
    assert_eq!(email.subject, "Boas-vindas");
    assert_eq!(email.html, Some("<strong>Olá!</strong>".to_string()));
    assert_eq!(email.reply_to, Some("suporte@empresa.ao".to_string()));
}

#[test]
fn test_webhook_verification() {
    let secret = "whsec_test_secret";
    let payload = br#"{"id":"evt_123","type":"email.delivered"}"#;

    let mut mac = HmacSha256::new_from_slice(secret.as_bytes()).unwrap();
    mac.update(payload);
    let signature = hex::encode(mac.finalize().into_bytes());

    let is_valid = Webhooks::verify_signature(payload, &signature, secret);
    assert!(is_valid);

    let is_invalid = Webhooks::verify_signature(payload, "invalid_sig", secret);
    assert!(!is_invalid);
}
