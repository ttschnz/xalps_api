use xalps::Overview;

#[tokio::main]
async fn main() {
    match Overview::request().await {
        Ok(overview) => {
            println!("{:?}", overview);
        }
        Err(err_content) => {
            print!("Failed to get Overview: {}", err_content);
        }
    }
}
