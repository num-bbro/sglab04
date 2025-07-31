#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("HELLO");
    sglib06::p01::run1().await?;
    Ok(())
}
