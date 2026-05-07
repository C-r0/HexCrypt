use std::env;
use std::fs::File;
use std::io::{self, Read};
use once_cell::sync::OnceCell;

static FILENAME: OnceCell<String> = OnceCell::new();
static KEY: OnceCell<u8> = OnceCell::new();

fn crypt() -> std::io::Result<()> {
	let namefile: &str = FILENAME.get().expect("FILENAME ALREADY INITIALIZED").as_str();
	println!("File: {}", namefile);
	
	KEY.set(0xAA);
	let key: &u8 = KEY.get().expect("KEY ALREADY INITIALIZED");
	
	let mut file = File::open(namefile)?;
	let mut bytes = Vec::new();
	
	file.read_to_end(&mut bytes)?;
	
	for (i, chunk) in bytes.chunks(16).enumerate() {
		let addr = i * 16;
		let hex = chunk.iter()
			.map(|b| format!("{:02x}", b ^ key))
			.collect::<Vec<_>>()
			.join(" ");
		let ascii = chunk.iter()
			.map(|&b| {let bcif = b ^ key; if bcif.is_ascii_graphic() || bcif == b' ' { bcif as char } else { '.' }})
			.collect::<String>();
		println!("{:08x}: {:47}  {}", addr, hex, ascii);
	}
	
	println!("ORIGINAL: ");
	
	for (i, chunk) in bytes.chunks(16).enumerate() {
		let addr = i * 16;
		let hex = chunk.iter()
			.map(|b| format!("{:02x}", b))
			.collect::<Vec<_>>()
			.join(" ");
		let ascii = chunk.iter()
			.map(|&b| if b.is_ascii_graphic() || b == b' ' { b as char } else { '.' })
			.collect::<String>();
		println!("{:08x}: {:47}  {}", addr, hex, ascii);
	}
	
	Ok(())
}

fn main() {
	let args: Vec<String> = env::args().collect();
	
	if args.len() == 2 {
		FILENAME.set(args[1].clone());
		crypt();
	} else {
		println!("Invalid Arguments | hexcrypt file");
	}
}
