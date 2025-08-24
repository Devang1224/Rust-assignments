use std::net::TcpListener;
use std::net::TcpStream;
use std::io::{Write,Read};


fn handle_connection(mut stream:TcpStream){
    let mut buffer = [0;1024];
    stream.read(&mut buffer).unwrap();
     println!("resultant data: {}",String::from_utf8_lossy(&buffer[..]));

// Your buffer is [u8; 1024] — an array of 1024 bytes.
// When you do &buffer, you’re giving a reference to the entire 1024-byte array, even if you only read, say, 64 bytes from the stream.
// The rest of the buffer is still zeros (0x00).
// So if you did:
// println!("Request: {}", String::from_utf8_lossy(&buffer));
// It will try to decode all 1024 bytes, meaning you’ll see your request text followed by a bunch of \0 characters (nulls).

 let response = "HTTP/1.1 200 OK\r\nContent-Length: 13\r\n\r\nHello, world!";
  
  stream.write(response.as_bytes()).unwrap();
  stream.flush().unwrap(); 
//  The OS or Rust’s buffering layer may keep them in a temporary buffer (for performance reasons).
// flush() forces those bytes to be pushed out of the buffer and onto the actual network socket.
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Error occurred");
    println!("Server running at http://127.0.0.1:7878");
    
    for stream in listener.incoming(){
        let mut data = stream.unwrap();
        handle_connection( data);
    }


}
