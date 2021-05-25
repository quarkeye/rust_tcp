//加载相关库
use std::io::{Error, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time;

//处理来自客户端的流
fn handle_client(mut stream: TcpStream) -> Result<(), Error>{
	let mut buf = [0; 512];
	for _ in 0..1000 {
		let bytes_read = stream.read(&mut buf)?;  //从流里读内容
		if bytes_read == 0 {   //没有读到内容
			return Ok(());
		}
		stream.write(&buf[..bytes_read])?; //流的内容写回去
		thread::sleep(time::Duration::from_secs(1 as u64)); //睡眠1秒钟
	}
	Ok(())
}

fn main() -> std::io::Result<()> {
	let listener = TcpListener::bind("127.0.0.1:8080")?; //绑定监听的ip和端口
	let mut thread_vec: Vec<thread::JoinHandle<()>> = Vec::new(); //申请一个容器用来放线程的句柄
	for stream in listener.incoming() {     //接收流的内容
		let stream = stream.expect("failed!"); //如果流有问题，就打印failed
		let handle = thread::spawn(move || {    //闭包调用handle_client
			handle_client(stream).unwrap_or_else(|error| eprintln!("{:?}", error));
			});
		thread_vec.push(handle); //句柄加到容器里
	}
	for handle in thread_vec {    //等待线程的结束
		handle.join().unwrap();
	}
	Ok(())
}

