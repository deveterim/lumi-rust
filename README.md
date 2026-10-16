# Mailifica Rust SDK (`mailifica`)

> SDK oficial em Rust para a infraestrutura de e-mails transacionais **Mailifica**, com interface drop-in replacement compatível com o Resend Rust SDK.

---

## 📦 Instalação (`Cargo.toml`)

```toml
[dependencies]
mailifica = "1.0.2"
tokio = { version = "1", features = ["full"] }
```

---

## 🚀 Como Usar

### 1. Enviar E-mail Transacional

```rust
use mailifica::{Mailifica, emails::CreateEmailBaseOptions};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mailifica = Mailifica::new("ma_live_123456789");

    let email = CreateEmailBaseOptions::new(
        "onboarding@suaempresa.ao",
        vec!["destinatario@exemplo.com"],
        "Boas-vindas!",
    )
    .with_html("<h1>Olá!</h1><p>Seu e-mail transacional foi entregue.</p>");

    let response = mailifica.emails.send(email).await?;
    println!("E-mail enviado! ID: {}", response.id);

    Ok(())
}
```

### 2. Envio em Lote (Batch)

```rust
use mailifica::{Mailifica, emails::CreateEmailBaseOptions};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mailifica = Mailifica::new("ma_live_123456789");

    let emails = vec![
        CreateEmailBaseOptions::new("news@suaempresa.ao", vec!["user1@gmail.com"], "News #1")
            .with_html("<p>Conteúdo 1</p>"),
        CreateEmailBaseOptions::new("news@suaempresa.ao", vec!["user2@gmail.com"], "News #2")
            .with_html("<p>Conteúdo 2</p>"),
    ];

    let batch_resp = mailifica.batch.send(emails).await?;
    println!("Total enviados: {}", batch_resp.data.len());

    Ok(())
}
```

### 3. Gestão de Domínios e API Keys

```rust
use mailifica::{Mailifica, domains::CreateDomainOptions};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mailifica = Mailifica::new("ma_live_123456789");

    let domain = mailifica.domains.create(CreateDomainOptions {
        name: "meudominio.ao".into(),
        region: None,
    }).await?;

    let verified = mailifica.domains.verify(&domain.id).await?;
    println!("Status do domínio: {}", verified.status);

    Ok(())
}
```

---

## 📄 Licença

MIT © [Mailifica](https://mailifica.com)
