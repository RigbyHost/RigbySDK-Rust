# RigbySDK Rust

Rust SDK для Rigby API. Интерфейс повторяет TypeScript SDK (`@rigbyhost/sdk-ts`): группы `gdps`, `notifications`, `user` и их методы.

## Установка

```toml
# Cargo.toml
[dependencies]
rigbysdk = { path = "../rigbysdk-rs" } # или с crates.io после публикации
```

## Пример

```rust
use rigbysdk::RigbySDK;
use serde_json::json;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sdk = RigbySDK::new("YOUR_API_TOKEN");

    let cfg = sdk.gdps.config.get(json!({ "srvId": "my-server-id" }))?;
    println!("Config: {cfg}");

    let levels = sdk
        .gdps
        .levels
        .search(json!({ "srvId": "my-server-id", "query": "demon" }))?;
    println!("Levels: {levels}");

    let me = sdk.user.me()?;
    println!("User: {me}");

    Ok(())
}
```

## Ошибки

HTTP ошибки возвращаются как `rigbysdk::SDKError` c `status` и `body`. Ошибки сериализации/сети возвращаются как `Box<dyn Error>`.

## Конфигурация

`RigbySDK::with_base_url(token, url)` — использовать альтернативный URL (по умолчанию `https://api.rigby.host`). Можно также переиспользовать `Client` напрямую.
