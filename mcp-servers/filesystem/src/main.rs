#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    filesystem::run().await
}
