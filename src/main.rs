use async_std::{io, task};
use async_std::net::TcpListener;
use async_std::prelude::*;
use fantasy_util::time::system_time::SystemLocalTime;

async fn say_hello() -> io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:3391").await?;
    let mut incoming = listener.incoming();

    while let Some(Ok(mut stream)) = incoming.next().await {
        task::spawn(async move {
            let id = SystemLocalTime::unix_nanos();
            println!("开始创建连接{}", id);
            let mut salt = [0u8; 32];
            stream.read(&mut salt).await.unwrap();
            println!("salt {:?}", salt);

            let mut encrypted_payload_length = [0u8; 2];
            stream.read(&mut encrypted_payload_length).await.unwrap();
            println!("encrypted_payload_length {:?}", encrypted_payload_length);
            println!("此连接生命周期结束")
        });
    }
    Ok(())
}

fn main() {
    task::block_on(say_hello());

    let salt: [u8; 32] = [165, 40, 153, 211, 131, 224, 132, 29, 109, 1, 191, 49, 178, 236, 71,
        181, 103, 153, 113, 90, 116, 168, 203, 94, 108, 124, 190, 7, 255, 84, 161, 251];
}