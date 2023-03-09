use clap::*;
use simple_logger::SimpleLogger;

type MyResult<OkType> = Result<OkType, String>;

#[derive(Parser, Debug)]
#[command(author, version, about="Downloading file from url", long_about = None)]
struct Args {
    #[clap(short = 'u', long = "url", help = "public url pointing to file")]
    url: String,

    #[clap(
        short = 'o',
        long = "output",
        help = "output path with file name and extension"
    )]
    output: String,

    #[clap(short = 'r', long = "rewrite", help = "rewrite file if exists")]
    rewrite: bool,
}

async fn download_file(url: &String, output: &String, rewrite: bool) -> MyResult<()> {
    log::info!("Downloading {} to {}...", url, output);
    let path_to_output = std::path::Path::new(output);
    if path_to_output.exists() && !rewrite {
        log::info!(
            "File {} already exists and rewrite is false, won't do anything",
            output
        );
        return Ok(());
    }
    if let Some(output_base_dir) = path_to_output.parent() {
        std::fs::create_dir_all(output_base_dir).map_err(|e| {
            format!(
                "Failed to create base dir {:?} for output {}, error : {}",
                output_base_dir, output, e
            )
        })?;
    }
    let response = reqwest::get(url)
        .await
        .map_err(|e| format!("Failed to get {}, error : {}", url, e))?;

    let mut output_file = std::fs::File::create(output)
        .map_err(|e| format!("Failed to create output file {}, error : {}", output, e))?;
    std::io::copy(
        &mut response
            .text()
            .await
            .map_err(|e| format!("Failed to get content from {}, error {}", url, e))?
            .as_bytes(),
        &mut output_file,
    )
    .map_err(|e| {
        format!(
            "Failed to copy content from {} to {}, error : {}",
            url, output, e
        )
    })?;
    log::info!("Downloaded {} to {}", url, output);
    Ok(())
}

#[tokio::main]
async fn main() -> MyResult<()> {
    SimpleLogger::new().with_level(log::LevelFilter::Info).init().unwrap();
    let args = Args::parse_from(std::env::args());
    match download_file(&args.url, &args.output, args.rewrite).await {
        Ok(()) => log::info!("Done"),
        Err(e) => log::error!("Not Done -- {}", e),
    };
    Ok(())
}
