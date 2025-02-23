use clap::Parser;
use hyper::body::Bytes;
use hyper::{Request, Uri};
use hyper_tls::HttpsConnector;
use hyper_util::client::legacy::Client;
use hyper_util::rt::TokioExecutor;

#[derive(Parser)]
pub(crate) struct HttpCommand {
    #[arg(help = "The URL to request")]
    url: String,

    #[arg(
        short = 'X',
        long = "request",
        default_value = "GET",
        help = "HTTP method (GET, POST, etc.)"
    )]
    method: String,

    #[arg(short = 'd', long = "data", help = "Data to send in a POST request")]
    data: Option<String>,

    #[arg(long = "header", action = clap::ArgAction::Append, help = "Headers to include in the request")]
    headers: Vec<String>,

    #[arg(long = "allow-insecure", help = "Allow insecure HTTP requests")]
    allow_insecure: bool,
}

pub async fn handle_request(cmd: HttpCommand) {
    let mut https = HttpsConnector::new();
    if cmd.allow_insecure {
        https.https_only(false)
    }
    let client = Client::builder(TokioExecutor::new()).build::<_, String>(https);

    let uri: Uri = cmd.url.parse().expect("Invalid URL");

    let mut req_builder = Request::builder().method(cmd.method.as_str()).uri(uri);

    for header in &cmd.headers {
        if let Some((key, value)) = header.split_once(": ") {
            req_builder = req_builder.header(key, value);
        }
    }

    let req = match cmd.method.as_str() {
        "POST" | "PUT" | "PATCH" => req_builder.body(cmd.data.unwrap().into()).unwrap(),
        _ => req_builder.body(String::new()).unwrap(),
    };

    match client.request(req).await {
        Ok(res) => {
            println!("Response: {}", res.status());
            let body_bytes = Bytes::new();
            println!("Body: {}", String::from_utf8_lossy(&body_bytes));
        }
        Err(err) => eprintln!("Request failed: {}", err),
    }
}
