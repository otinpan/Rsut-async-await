use tokio::io::{AsyncBufReadExt, AsyncWriteExt}; 
use tokio::io;
use tokio::net::TcpListener; 

#[tokio::main] 
async fn main() -> io::Result<()> {
    // 10000番ポートでTCPリッスン 
    let listener = TcpListener::bind("127.0.0.1:10000").await.unwrap();

    loop {
        // TCPコネクションアクセプト 
        let (mut socket, addr) = listener.accept().await?;
        println!("accept: {}", addr);

        // 非同期タスク生成 
        tokio::spawn(async move {
            // バッファ読み書き用オブジェクト生成 
            let (r, w) = socket.split(); 
            let mut reader = io::BufReader::new(r);
            let mut writer = io::BufWriter::new(w);

            let mut line = String::new();
            loop {
                line.clear(); 
                // クライアントからの入力を非同期で処理
                match reader.read_line(&mut line).await { 
                    Ok(0) => { 
                        println!("closed: {}", addr);
                        return;
                    }
                    Ok(_) => {
                        print!("read: {}, {}", addr, line);
                        writer.write_all(line.as_bytes()).await.unwrap();
                        writer.flush().await.unwrap();
                    }
                    Err(e) => { // エラー
                        println!("error: {}, {}", addr, e);
                        return;
                    }
                }
            }
        });
    }
}