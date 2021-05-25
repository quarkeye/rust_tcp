//加载相关库
use std::io::{self, prelude::*, BufReader, Write};
use std::net::TcpStream;
use std::str;

fn main() -> std::io::Result<()> {
	let mut stream = TcpStream::connect("127.0.0.1:8080")?; //创建一个流，连接到server端
	for _ in 0..10 {
		let mut input = String::new();
		io::stdin().read_line(&mut input).expect("Failed to read from stdin");  //读取标准输入
		stream.write(input.as_bytes()).expect("Failed to write to stream"); //读取的内容写到流里
		let mut reader = BufReader::new(&stream);  //从服务端读取流
		let mut buffer: Vec<u8> = Vec::new();   
		reader.read_until(b'\n', &mut buffer).expect("Could not read into buffer"); //读取流的内容
		println!("Read from server: {}",str::from_utf8(&buffer).expect("Could not write buffer as string"));//打印流的内容
		println!("");
	}
	Ok(())
}

