#![feature(fs_try_exists, file_create_new)]

use reqwest;
use std::io::{stdout, Write};
use tokio::time::{sleep, Duration};

const URL: &str = "https://api.discord.gx.games/v1/direct-fulfillment";

#[tokio::main]
async fn main() {
	let counter = std::sync::atomic::AtomicUsize::new(0);
	let mut stdout = stdout();

	if std::fs::try_exists("tokens.txt").is_err() {
		std::fs::File::create_new("tokens.txt").unwrap();
	}

	loop {
		// Sleep for 2000 milliseconds
		sleep(Duration::from_millis(2000)).await;

		if let Ok(token) = make_request().await {
			if let Err(e) = std::fs::OpenOptions::new()
				.append(true)
				.open("tokens.txt")
				.and_then(|mut file| file.write_all(format!("{token}\n").as_bytes()))
			{
				eprintln!("Error writing to file: {}", e);
			}

			let count = counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
			stdout
				.write(format!("\rGenerated: {}", count).as_bytes())
				.unwrap();
			stdout.flush().unwrap();
		}
	}
}

async fn make_request() -> Result<String, reqwest::Error> {
	let client = reqwest::Client::new();
	let body =
		r#"{"partnerUserId":"e534e4d94bb2450fb6eda2027fdec128965a237fcdbc49c62ac12bc9c799afd1"}"#;

	let response = client
		.post(URL)
		.headers(headers())
		.body(body)
		.send()
		.await?;

	let json: serde_json::Value = response.json().await?;
	let token = json["token"].as_str().unwrap_or_default();

	Ok(token.to_string())
}

fn headers() -> reqwest::header::HeaderMap {
	let mut headers = reqwest::header::HeaderMap::new();
	headers.insert(
		reqwest::header::CONTENT_TYPE,
		"application/json".parse().unwrap(),
	);
	headers.insert(
		reqwest::header::HOST,
		"api.discord.gx.games".parse().unwrap(),
	);
	headers.insert(reqwest::header::ACCEPT, "*/*".parse().unwrap());
	headers.insert(
		reqwest::header::ACCEPT_LANGUAGE,
		"en-US,en;q=0.9".parse().unwrap(),
	);
	headers.insert(
		reqwest::header::ORIGIN,
		"https://www.opera.com".parse().unwrap(),
	);
	headers.insert(
		reqwest::header::REFERER,
		"https://www.opera.com/".parse().unwrap(),
	);
	headers.insert(
        reqwest::header::USER_AGENT,
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/119.0.0.0 Safari/537.36 OPR/105.0.0.0".parse().unwrap(),
    );

	headers
}
