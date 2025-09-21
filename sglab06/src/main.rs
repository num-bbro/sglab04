use std::env;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let now = std::time::SystemTime::now();
    let a1 = env::args().nth(1).unwrap_or("?".to_string());
    //let a2 = env::args().nth(2).unwrap_or("?".to_string());
    //let a3 = env::args().nth(3).unwrap_or("?".to_string());
    match a1.as_str() {
        "var0" => sglib06::c00::var_type()?,
        "cal0" => sglib06::c00::calc00()?,
        "run1" => sglib06::p01::run1().await?,
        _ => sglib06::p01::run1().await?,
    }
    let se = now.elapsed().unwrap().as_secs();
    let mi = se / 60;
    println!("time {se} sec = {mi} min");
    Ok(())
}
