use clap::Parser;
use tenhou_dl::{download_json, filename_to_id, get_filename_list, id_to_link};
use tokio;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let Args { input, output } = Args::parse();
    let output = output.unwrap_or_else(|| input.clone());
    let urls = get_filename_list(&input)
        .into_iter()
        .flat_map(|s| filename_to_id(&s))
        .map(|id| id_to_link(&id));
    // dbg!(&output);
    let client = reqwest::Client::new();
    let tasks = urls
        .map(|url| download_json(&client, url, &output))
        .collect::<Vec<_>>();
    let _ = futures::future::join_all(tasks).await;

    Ok(())

}

#[derive(Parser)]
#[command(name = "Tenhou downloader")]
struct Args {
    #[arg(long, short)]
    input: String,
    #[arg(long, short)]
    output: Option<String>,
}
