use iqoption_rs::IQOptionBuilder;

// A simple type alias so as to DRY.
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() -> Result<()> {
    let email = "example@example.com";
    let password = "example123";

    // FIXME: Incomplete example

    let client = IQOptionBuilder::default()
        .identification(email)
        .password(password)
        .build()
        .await;

    if client.is_err() {
        panic!("Failed to connect to IQOption API");
    }

    // let client = client.unwrap();

    println!("Connected to IQOption API");

    Ok(())
}
