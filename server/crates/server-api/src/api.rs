//TODO: Maybe include any kind of rest apis or w/e in here, or we could go with a different
//approach I was just offering this as a sort of skeleton setup

use std::{
    //Prelude is for including many commonly used traits in standard library, it is common
    //convention in public crates as well.
    io::prelude::*,
    net::TcpStream
};

//I am just statically embeddeding the index file into the executable, it's really small for now so
//it's dumb to block on io for it. Obviously when we make a more complicated file we may need
//dynamic file I/O!
const INDEX_FILE : &'static str = include_str!("../res/index.html");
const ERROR_404_FILE : &'static str = include_str!("../res/404.html");

pub fn serve_index(mut stream: TcpStream) -> anyhow::Result<()> {
    let status_line = "HTTP/1.1 200 OK";
    let contents = INDEX_FILE;
    let length = contents.len();

    let response = format!(
        "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
    );
    Ok(stream.write_all(response.as_bytes())?)
}

pub fn serve_error_404(mut stream: TcpStream) -> anyhow::Result<()> {
    let status_line = "HTTP/1.1 404 NOT FOUND";
    let contents = ERROR_404_FILE;
    let length = contents.len();

    let response = format!(
        "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
    );

    Ok(stream.write_all(response.as_bytes())?)
}
