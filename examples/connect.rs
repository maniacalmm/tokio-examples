extern crate bytes;
extern crate futures;
extern crate tokio;
extern crate tokio_io;

use std::env;
use std::io::{self, Read, Write};
use std::net::SocketAddr;
use std::thread;

use futures::sync::mpsc;
use tokio::prelude::*;

fn main() -> Result<(), Box<std::error::Error>> {
    let mut args = env::args().skip(1).collect::<Vec<_>>();
    let tcp = match args.iter().position(|a| a == "--udp") {
        Some(i) => {
            args.remove(i);
            false
        }
        None => true,
    };

    let addr = match args.first() {
        Some(addr) => addr,
        None => Err("this program requires at least one argument")?,
    }
    .parse::<SocketAddr>()?;

    let (stdin_tx, stdin_rx) = mpsc::channel(0);
    thread::spawn(|| read)

}

fn read_stdin(mut tx: mpsc::Sender<Vec<u8>>) {
    // get stdin
    let mut stdin = io::stdin();
    loop {
        let mut buf = vec![0; 1024];
        // read the content of stdin to buf, if nothing, break
        let n = match stdin.read(&mut buf) {
            Err(_) | Ok(0) => break,
            Ok(n) => n,
        };

        buf.truncate(n);
        // send the content to channel
        // which usually received by another thread
        // and if successful, this loop will not break, but resume to read more
        // until we input nothing, so it will break?
        tx = match tx.send(buf).wait() {
            Ok(tx) => tx,
            Err(_) => break,
        };
    }
}