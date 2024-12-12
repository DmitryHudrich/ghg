use clap::{Arg, Command};
use tokio::{fs::File, io::AsyncWriteExt};

#[tokio::main]
async fn main() {
    let matches = Command::new("Github file downloader.")
        .about("Github file downloader.\nAccepts a single URL to the file which stores in some repo.\nExample: 'ghg https://github.com/asya4u/asya-daemon/blob/main/src/main.rs'")
        .arg(Arg::new("url").required(true).value_name("URL").index(1))
        .get_matches();

    let input_url = matches.get_one::<String>("url").unwrap();

    let endpoint = if let Some(pos) = input_url.find(".com") {
        &input_url[(pos + 4)..] // skip ".com"
    } else {
        todo!()
    };

    let parsed_endpoint = Vec::from_iter(
        endpoint
            .split('/')
            .filter(|line| !line.trim().is_empty())
            .map(|e| e.to_string()),
    );
    let parsed = FileInRepo {
        organization: parsed_endpoint[0].clone(),
        repo: parsed_endpoint[1].clone(),
        branch: parsed_endpoint[3].clone(),
        file_path: parsed_endpoint[4..].join("/"),
    };

    let usercontent_url = format!(
        "https://raw.githubusercontent.com/{}/{}/{}/{}",
        parsed.organization, parsed.repo, parsed.branch, parsed.file_path
    );

    let body = reqwest::get(usercontent_url)
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let mut file = File::create(parsed_endpoint.last().unwrap().to_string())
        .await
        .unwrap();

    file.write_all(body.as_bytes()).await.unwrap();
}

#[derive(Debug)]
struct FileInRepo {
    organization: String,
    repo: String,
    branch: String,
    file_path: String,
}
