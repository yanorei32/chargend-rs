use std::io;
use std::net::SocketAddr;

use clap::Parser;
use tokio::net::TcpListener;
use std::path::PathBuf;

#[derive(Debug, Parser)]
struct Cli {
    #[clap(env, long, default_value= "/etc/chargen.txt")]
    file: PathBuf,

    #[clap(env, long, default_value = "10ms")]
    interval: humantime::Duration,

    #[clap(env, long, default_value = "0.0.0.0:19")]
    listen: SocketAddr,
}

#[tokio::main]
async fn main() -> io::Result<()> {
    tracing_subscriber::fmt::init();


    let cli = Cli::parse();

    let file = std::fs::read_to_string(&cli.file).expect("Failed to open chargen file");
    let file = &*file.leak();
    let file = file.as_bytes();


    tracing::info!("Server is ready on {}", cli.listen);

    let listener = TcpListener::bind(cli.listen).await?;

    loop {
        let (mut socket, remote) = listener.accept().await?;
        tracing::info!("{remote} connected!");

        tokio::spawn(async move {
            let (_rx, tx) = socket.split();
            let mut nth_byte = 0;

            loop {
                if let Err(e) = tx.writable().await {
                    tracing::error!("Write Error [0] {e}");
                    break;
                }

                match tx.try_write(&[file[nth_byte]]) {
                    Ok(_) => {}
                    Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                        continue;
                    }
                    Err(e) => {
                        tracing::error!("Write Error [1] {e}");
                        break;
                    }
                }

                tokio::time::sleep(cli.interval.into()).await;
                nth_byte += 1;
                nth_byte %= file.len();
            }

            tracing::info!("{remote} disconnected!");
        });
    }
}
