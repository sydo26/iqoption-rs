use iqoption_rs::IQOptionClient;

// A simple type alias so as to DRY.
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() -> Result<()> {
    let email = "example@example.com";
    let password = "example123";

    // FIXME: Incomplete example

    let client = IQOptionClient::default()
        .identification(email)
        .password(password)
        .connect()
        .await;

    Ok(())
}
