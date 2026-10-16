use mailifica::{Mailifica, emails::CreateEmailBaseOptions};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mailifica = Mailifica::new("ma_live_123456789");

    let email = CreateEmailBaseOptions::new(
        "onboarding@suaempresa.ao",
        vec!["destinatario@exemplo.com"],
        "Código de Segurança: 981-204",
    )
    .with_html("<h1>Olá!</h1><p>Seu código é <strong>981-204</strong>.</p>");

    let response = mailifica.emails.send(email).await?;
    println!("E-mail enviado! ID: {}", response.id);

    Ok(())
}
