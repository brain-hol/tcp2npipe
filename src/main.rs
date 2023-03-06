use std::time::Duration;
use tokio::net::{windows::named_pipe::ClientOptions, TcpListener, TcpStream};
use tokio::{io, time};
use windows_sys::Win32::Foundation::ERROR_PIPE_BUSY;

const SERVER_ADDRESS: &str = "0.0.0.0:12098";
const PIPE_NAME: &str = r"\\.\pipe\openssh-ssh-agent";

#[tokio::main]
async fn main() -> io::Result<()> {
    println!("Starting");
    let listener = TcpListener::bind(SERVER_ADDRESS).await?;
    println!("Listening on {SERVER_ADDRESS}");
    loop {
        match listener.accept().await {
            Ok((socket, addr)) => {
                println!("new client: {:?}", addr);
                tokio::spawn(async move {
                    handle_client(socket).await.unwrap();
                });
            }
            Err(e) => println!("couldn't get client: {:?}", e),
        }
    }
}

async fn handle_client(mut stream: TcpStream) -> io::Result<()> {
    let mut client = loop {
        match ClientOptions::new().open(PIPE_NAME) {
            Ok(client) => break client,
            Err(e) if e.raw_os_error() == Some(ERROR_PIPE_BUSY as i32) => (),
            Err(e) => return Err(e),
        }
        println!("Pipe not open yet");
        time::sleep(Duration::from_millis(50)).await;
    };
    println!("Pipe connected");
    io::copy_bidirectional(&mut stream, &mut client).await?;
    println!("Finished handling client");
    Ok(())
}
