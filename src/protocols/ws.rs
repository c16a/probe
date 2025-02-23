use clap::Parser;
use crossterm::{
    cursor::MoveToColumn,
    execute,
    terminal::{Clear, ClearType},
};
use futures_util::{SinkExt, stream::StreamExt};
use hyper::Uri;
use std::{
    io,
    io::Write
};
use tokio::task;
use tokio_tungstenite::connect_async;
use tungstenite::{Message, Utf8Bytes};

#[derive(Parser, Debug)]
pub(crate) struct WebSocketCommand {
    #[arg(help = "The WebSocket URL to connect to")]
    url: String,
}

pub async fn handle_request(cmd: WebSocketCommand) {
    let uri: Uri = cmd.url.parse().expect("Invalid URL");
    rustls::crypto::aws_lc_rs::default_provider()
        .install_default()
        .expect("Failed to install rustls crypto provider");

    let (socket, _response) = connect_async(uri).await.unwrap();
    let (mut write, mut read) = socket.split();

    println!("Connected to the server. Type 'exit' to disconnect.");

    // Spawn a task to receive messages from the server
    let recv_task = task::spawn(async move {
        while let Some(Ok(msg)) = read.next().await {
            if let Message::Text(text) = msg {
                // Clear current prompt
                execute!(io::stdout(), MoveToColumn(0), Clear(ClearType::CurrentLine)).unwrap();
                println!("[Server]: {}", text);

                // Reprint the prompt
                print!("> ");
                io::stdout().flush().unwrap();
            }
        }
        println!("\nConnection closed by server.");
    });

    // User input loop
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim().to_string();

        if input.eq_ignore_ascii_case("exit") {
            println!("Closing connection...");
            let _ = write.send(Message::Close(None)).await;
            break;
        }

        // Clear the prompt and print the sent message
        execute!(io::stdout(), MoveToColumn(0), Clear(ClearType::CurrentLine)).unwrap();
        println!("[Client]: {}", input.trim_end());

        write
            .send(Message::Text(Utf8Bytes::from(input)))
            .await
            .unwrap();
    }

    // Wait for the receiver task to finish before exiting
    recv_task.await.unwrap();
}
