mod helpers;
mod websocket;

pub use websocket::*;

#[cfg(test)]
mod tests {
    use crate::websocket::server::WebSocketServer;
    // #[test]
    // fn it_works() {
    //     let x = match WebSocketServer::new(5000) {
    //         Ok(x) => x,
    //         Err(e) => {
    //             println!("{:?}", e);
    //             return;
    //         }
    //     };
    //
    //     while let Ok(mut ws) = x.listen() {
    //         println!("connected");
    //         let mut buffer: [u8; 1024] = [0u8; 1024];
    //         while let Ok(_) = ws.recv(&mut buffer) {
    //             println!("{:?}", buffer.to_vec());
    //         }
    //     }
    // }
    #[test]
    fn whackjob() {
        let y: u16 = 1;
        let z: u16 = 256;

        let a: u8 = 0b100_1011;
        let b: u8 = 0b111_1111;

        println!("jank = {}", a & b);
    }
}
