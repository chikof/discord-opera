#![feature(file_create_new)]

use reqwest;
use std::io::{self, stdout, Write};
use tokio::time::{sleep, Duration};
use uuid::Uuid;

const URL: &str = "https://api.discord.gx.games/v1/direct-fulfillment";

#[tokio::main]
async fn main() {
	let counter = std::sync::atomic::AtomicUsize::new(0);
	let mut stdout = stdout();

	loop {
		// Sleep for 1 second
		sleep(Duration::from_secs(1)).await;

		if let Ok(token) = make_request().await {
			if let Err(e) = std::fs::OpenOptions::new()
				.append(true)
				.open("tokens.txt")
				.and_then(|mut file| file.write_all(format!("{token}\n").as_bytes()))
			{
				if e.kind() == io::ErrorKind::NotFound {
					std::fs::File::create_new("tokens.txt").unwrap();
				}
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
	// Generate a random UUID every time
	let parent_user_id_str = format!("{}{}", Uuid::new_v4().simple(), Uuid::new_v4().simple());

	let body = format!("{{\"partnerUserId\":\"{}\"}}", parent_user_id_str);

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
