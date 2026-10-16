use lumi::{Lumi, emails::CreateEmailBaseOptions};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lumi = Lumi::new("lu_live_123456789");

    let email = CreateEmailBaseOptions::new(
        "onboarding@suaempresa.com",
        vec!["destinatario@exemplo.com"],
        "Código de Segurança: 981-204",
    )
    .with_html("<h1>Olá!</h1><p>Seu código é <strong>981-204</strong>.</p>");

    let response = lumi.emails.send(email).await?;
    println!("E-mail enviado! ID: {}", response.id);

    Ok(())
}
