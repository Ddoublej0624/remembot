use std::{
    //Prelude is for including many commonly used traits in standard library, it is common
    //convention in public crates as well.
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream}
};

use server_api::{self, api};

pub static DEFAULT_BIND_ADDR: &str = "127.0.0.1:7878";

//Block the main thread for the entirety of the session. We can use a concurrent approach later,
//like a thread pool or maybe even a local pool.   
fn handle_connection(mut stream: TcpStream) -> anyhow::Result<()> {
    let peer = stream.peer_addr();
    log::trace!("Incoming request from {peer:#?}");
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    if request_line == "GET / HTTP/1.1" {
        api::serve_index(stream)?
    } else {
        api::serve_error_404(stream)?
    }
    Ok(())
}


pub fn server_main(bind_addr: &str) -> anyhow::Result<()> {
    //Start listening on our bind address, this is blocking
    log::trace!("Server main called with bind addr: {}", bind_addr);
    let listener = TcpListener::bind(bind_addr).unwrap();
    for stream in listener.incoming() {
        let stream = stream?;
        handle_connection(stream)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_handle_connection() {
        //TODO: Need MockTcpStream impl, rust book has an example but I dont know of any publically
        //used crates for this
        assert_eq!(true, true);
    }
}
