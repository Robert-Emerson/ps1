use std::os;
use std::io::File;

fn main() {
    let args: ~[~str] = os::args();
    if args.len() != 3 {
        println!("Usage: {:s} <inputfile1> <inputfile2>", args[0]); 
    } else {
	let current_dir = std::os::getcwd(); 
	
        let share1fname = args[1].clone();
        let share2fname = args[2].clone();
        
        let path1 = Path::new(share1fname.clone());
        let path2 = Path::new(share2fname.clone());
        let path3 = Path::new(current_dir.as_str().unwrap() + "/message.out");

	let share1_file = File::open(&path1);
	let share2_file = File::open(&path2);
	let message_file = File::create(&path3);
	
	 match (share1_file, share2_file) {
	  (Some(mut share1), Some(mut share2)) => { 
	    let share1_msg: ~[u8] = share1.read_to_end();
	    let share2_msg: ~[u8] = share2.read_to_end();
	    
	    match (message_file) {
	      Some(msg) => join(share1_msg, share2_msg, msg) ,
	      None => fail!("Error opening output file!"),
	    }
	  }, 
	  (_, _) => fail!("Error opening input files!"),
	}
    }
}

fn xor(a: &[u8], b: &[u8]) -> ~[u8] {
    let mut ret = ~[];
    for i in range(0, a.len()) {
	ret.push(a[i] ^ b[i]);
    }
    ret
}

fn join(share1_bytes: &[u8], share2_bytes: &[u8], mut message: File) {
    
    let message_bytes = xor(share1_bytes, share2_bytes);
    message.write(message_bytes);
}