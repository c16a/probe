use clap::Parser;
use crossterm::{
    cursor::MoveToColumn,
    execute,
    terminal::{Clear, ClearType},
};
use std::{
    io::{self, Write},
    net::SocketAddr,
};
use tokio::net::lookup_host;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
    task,
};

#[derive(Parser, Debug)]
pub(crate) struct TcpCommand {
    #[arg(help = "The TCP address to connect to")]
    pub(crate) address: String,
}

async fn resolve_address(address: &str) -> SocketAddr {
    let mut addrs = lookup_host(address).await.expect("Failed to resolve host");
    addrs.next().expect("No valid addresses found")
}

pub async fn handle_request(cmd: TcpCommand) {
    let addr = resolve_address(&cmd.address).await;
    let stream = TcpStream::connect(addr).await.expect("Failed to connect to server");

    let (reader, mut writer) = stream.into_split();
    let mut reader = BufReader::new(reader);

    println!("Connected to the server. Type 'exit' to disconnect.");

    // Spawn a task to receive messages from the server
    let recv_task = task::spawn(async move {
        let mut line = String::new();
        while reader.read_line(&mut line).await.unwrap_or(0) > 0 {
            // Clear current prompt
            execute!(io::stdout(), MoveToColumn(0), Clear(ClearType::CurrentLine)).unwrap();
            println!("[Server]: {}", line.trim());

            // Reprint the prompt
            print!("> ");
            io::stdout().flush().unwrap();

            line.clear();
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
            let _ = writer.write_all(b"exit\n").await;
            break;
        }

        // Clear the prompt and print the sent message
        execute!(io::stdout(), MoveToColumn(0), Clear(ClearType::CurrentLine)).unwrap();
        println!("[Client]: {}", input.trim_end());

        writer.write_all(format!("{}\n", input).as_bytes()).await.unwrap();
    }

    // Ensure the receiver task finishes before exiting
    recv_task.await.unwrap();
}