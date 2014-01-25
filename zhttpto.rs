//
// zhttpto.rs
//
// Starting code for PS1
// Running on Rust 0.9
//
// Note that this code has serious security risks!  You should not run it 
// on any system with access to sensitive files.
// 
// University of Virginia - cs4414 Spring 2014
// Weilin Xu and David Evans
// Version 0.3

#[feature(globs)];
use std::io::*;
use std::io::net::ip::{SocketAddr};
use std::{str};

static IP: &'static str = "127.0.0.1";
static PORT:        int = 4414;

fn main() {
    let addr = from_str::<SocketAddr>(format!("{:s}:{:d}", IP, PORT)).unwrap();
    let mut acceptor = net::tcp::TcpListener::bind(addr).listen();
    let mut visitor_count = 0;
    
    println(format!("Listening on [{:s}] ...", addr.to_str()));
    
    for stream in acceptor.incoming() {
        // Spawn a task to handle the connection

	visitor_count += 1;
	let count: int = visitor_count;

	do spawn {
            let mut stream = stream;
            
            match stream {
                Some(ref mut s) => {
                             match s.peer_name() {
                                Some(pn) => {println(format!("Received connection from: [{:s}]", pn.to_str()));},
                                None => ()
                             }
                           },
                None => ()
            }
            
            let mut buf = [0, ..500];
            stream.read(buf);
            let request_str = str::from_utf8(buf);
            println(format!("Received request :\n{:s}", request_str));
	    println!("Visitor count: {}", count);
	    
	    let wordvec: ~[&str] = request_str.split(' ').collect();
	    let current_dir = std::os::getcwd(); 
	    let filepath = Path::new(current_dir.as_str().unwrap() + wordvec[1]);
	    let response = open_file(filepath, current_dir, count);
	    
	    stream.write(response.as_bytes());
		
	    println!("Connection terminates.\n");
        }
    }
}

fn open_file(filepath: Path, current_dir: Path, counter: int) -> ~str {  
  
  match (filepath.exists()) {
    true => {	      
	      let filestring = filepath.as_str().unwrap();
	      
	      if filestring.ends_with(".html") {
		let mut file = File::open(&filepath);
		
		let temp = ~"HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n";
		let response = temp + file.read_to_str();
		
		return response;
	      }
	      else if filepath != current_dir {
		let response: ~str = 
		        ~"HTTP/1.1 403 FORBIDDEN\r\nContent-Type: text/html; charset=UTF-8\r\n
			<doctype !html><html><head><title>Nope. Not allowed!</title>
			<style>body { background-color: #FFF; color: #FFEEAA }
				h1 { font-size:2cm; text-align: center; color: black; text-shadow: 0 0 4mm black}
				h2 { font-size:2cm; text-align: center; color: black; text-shadow: 0 0 4mm green}
			</style></head>
			<body>
			<h1>ERROR 403</h1>
			<h2>Sorry bud, you don't have permission for that</h2>
			</body></html>\r\n";
		return response;
	      }
	    },
    false => println("File doesn't exist, so we display the homepage!")
  };
  let temp = ~"HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n
	      <doctype !html><html><head><title>Hello, Rust!</title>
	      <style>body { background-color: #111; color: #FFEEAA }
		    h1 { font-size:2cm; text-align: center; color: black; text-shadow: 0 0 4mm red}
		    h2 { font-size:2cm; text-align: center; color: black; text-shadow: 0 0 4mm green}
	      </style></head>
	      <body>
	      <h1>Greetings, visitor $$!</h1>
	    </body></html>\r\n";

  let response = std::str::replace(temp, "$$", counter.to_str());
  return response;
}
