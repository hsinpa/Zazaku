use tokio::net::{TcpStream};
use fastwebsockets::{WebSocket, OpCode, Role};
use anyhow::Result;

fn main() {
    println!("Hello, world!");
}

async fn handle(socket: TcpStream, ) -> Result<(), Box<dyn std::error::Error>> {
    let mut ws = WebSocket::after_handshake(socket, Role::Server);
    ws.set_writev(false);
    ws.set_auto_close(true);
    ws.set_auto_pong(true);
  
    loop {
      let frame = ws.read_frame().await?;
      match frame.opcode {
        OpCode::Close => break,
        OpCode::Text | OpCode::Binary => {
          ws.write_frame(frame).await?;
        }
        _ => {}
      }
    }
    Ok(())
  }