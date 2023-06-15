use cli_table::{print_stdout, WithTitle};
use xalps::AthleteSummary;

#[tokio::main]
async fn main() {
    match AthleteSummary::request().await {
        Ok(summary) => print_stdout(summary.with_title()).unwrap(),
        Err(err_content) => {
            print!("Failed to get summary: {}", err_content);
        }
    }
}
