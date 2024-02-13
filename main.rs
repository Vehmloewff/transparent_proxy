use std::{sync::Arc, time::Duration};

use clap::Parser;
use futures::future::join_all;
use tokio::{
	io::{copy, split},
	net::{TcpListener, TcpStream},
	sync::RwLock,
	time::timeout,
};

#[derive(Debug, Parser)]
struct CliParams {
	/// The port to bind local connections to
	#[arg(long, short = 'p', value_name = "PORT")]
	port: u16,
	/// Where to proxy connections to
	#[arg(long, short = 'd', value_name = "ADDRESS")]
	destination: String,
	/// How long to wait until closing an connection
	#[arg(long, short = 't', value_name = "SECONDS", default_value_t = 120)]
	timeout: u64,
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
	let args = CliParams::parse();
	let addr = format!("localhost:{}", &args.port);

	let listener = TcpListener::bind(&addr).await.unwrap();
	println!("Listening at {addr}");

	let connections_count = Arc::new(RwLock::new(0));
	let proxy_to = args.destination;
	let timeout_duration = Duration::from_secs(args.timeout);

	loop {
		let (mut stream, _ip) = listener.accept().await.unwrap();
		let connections_count = connections_count.clone();
		let proxy_to = proxy_to.clone();
		let timeout_duration = timeout_duration.clone();

		tokio::spawn(async move {
			let mut remote = TcpStream::connect(&proxy_to).await.unwrap();

			{
				let mut count = connections_count.write().await;
				*count = *count + 1;

				println!("Connection opened to {proxy_to}. {count} connections open");
			};

			let (mut stream_reader, mut stream_writer) = split(&mut stream);
			let (mut remote_reader, mut remote_writer) = split(&mut remote);

			let _ = timeout(
				timeout_duration,
				join_all([copy(&mut stream_reader, &mut remote_writer), copy(&mut remote_reader, &mut stream_writer)]),
			)
			.await;

			{
				let mut count = connections_count.write().await;
				*count = *count - 1;

				println!("Connection closed. {count} connections open");
			};
		});
	}
}
