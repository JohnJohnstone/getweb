use clap::{Parser, ValueEnum};
use playwright::Playwright;

use std::{fs, path::PathBuf};

#[derive(Parser)]
#[clap(version = "0.1.0", author = "John Johnstone", about = "getweb")]
pub struct Cli {
    /// Webpage URL
    #[clap(required = true)]
    pub url: String,
    /// playwright browser
    #[clap(short, long, default_value = "chromium")]
    pub browser: PlaywrightBrowser,
    /// Where to save the output
    #[clap(short, long, required = false)]
    output: Option<PathBuf>,
}

fn output_to_file(file_path: PathBuf, content: &str) -> std::io::Result<()> {
    fs::write(file_path, content)
}

#[derive(Clone, ValueEnum)]
pub enum PlaywrightBrowser {
    Chromium,
    Firefox,
    Webkit,
}

async fn get_html(url: String, browser: PlaywrightBrowser) -> Result<String, playwright::Error> {
    let playwright = Playwright::initialize().await?;

    // TODO this should only need to be run once
    playwright.prepare()?; // Install browsers

    let browser = match browser {
        PlaywrightBrowser::Chromium => {
            playwright
                .chromium()
                .launcher()
                .headless(true)
                .launch()
                .await?
        }
        PlaywrightBrowser::Firefox => {
            playwright
                .firefox()
                .launcher()
                .headless(true)
                .launch()
                .await?
        }
        PlaywrightBrowser::Webkit => {
            playwright
                .webkit()
                .launcher()
                .headless(true)
                .launch()
                .await?
        }
    };

    let context = browser
        .context_builder()
        .js_enabled(true)
        .user_agent("Mozilla/5.0 (Windows NT 6.1; Win64; x64; rv:47.0) Gecko/20100101 Firefox/47.0")
        .build()
        .await?;
    let page = context.new_page().await?;
    page.goto_builder(url.as_str()).goto().await?;
    let html = page.content().await?;
    // let screenshot = page.screenshot_builder();
    Ok(html.to_string())
}

#[tokio::main]
async fn main() -> Result<(), playwright::Error> {
    // parse the cli arguments
    let cli = Cli::parse();
    let html = get_html(cli.url, cli.browser).await?;

    match cli.output {
        Some(path) => {
            output_to_file(path, &html).unwrap();
        }
        None => {
            println!("{}", html);
        }
    }
    Ok(())
}
