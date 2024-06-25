use std::{
    fs,
    io::{prelude::*, BufReader, Read},
    net::{TcpListener, TcpStream},
    env,
    thread,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let port = &args[0];
    let listener = TcpListener::bind(format!("127.0.0.1:{port}")).unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        thread::spawn(|| {
            handle_connection(stream);
        });
    }
}


fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request = buf_reader.lines().next().unwrap().unwrap();

    if request.starts_with("GET ") && request.ends_with(" HTTP/1.1") {
        let path:String = request.chars().skip(5).take(request.len() - 13).collect();
        println!("{path}");
        match read(&path) {
            Ok(file_content) => {
                let status_line = "HTTP/1.1 200 OK";
                let length = file_content.len();
                let response = format!(
                    "{status_line}\r\nContent-Length: {length}\r\n\r\n{file_content}"
                );
                println!("{}", response);
                stream.write_all(response.as_bytes()).unwrap();
            }
            Err(err) => {
                let status_line = "HTTP/1.1 404 NOT FOUND";
                let contents = err.to_string();
                let length = contents.len();

                let response = format!(
                    "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
                );

                stream.write_all(response.as_bytes()).unwrap();
            }
        }
    }
    else {
        let response = "HTTP/1.1 404 NOT FOUND";
        println!("HTTP/1.1 404 NOT FOUND");
        stream.write_all(response.as_bytes()).unwrap();
    }
}

fn read(file: &str) -> Result<String, std::io::Error> {
    let mut file = fs::File::open(file)?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;
    Ok(data)
}