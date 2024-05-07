// Import necessary crates
use nng::{options::{protocol::pubsub::Subscribe, Options, RecvBufferSize,SendBufferSize},Protocol, Socket, Message};
use serde::{Serialize, Deserialize};
use rmp_serde::{Serializer, Deserializer};
use std::io::Cursor;
use std::thread;
use std::time::Duration;
use centralstruct::*;
use centrallib::*;

// Define your struct


fn main() {
    let config = match read_marketconf( "../marketcfg.json") {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Error reading configuration: {}", e);
            return; // Exit the program if there's an error
        }
    };
    // Create a socket
    let mut socket = Socket::new(Protocol::Push0).unwrap();  // Changed from Push0 to Pub0
    // Bind the socket over TCP
    socket.dial(&config.iprcv).unwrap();
    
   // thread::sleep(Duration::from_secs(2));

    // Create your struct
    let order = LimitOrder {
       
        market: "USDTAR".to_string(),
        broker_identifier: "KTKL101".to_string(),
        trader_identifier: 7630002003791859102,
        order_identifier: None,
        order_quantity: 5,
        order_side: "sell".to_string(),
        expiration: "GTC".to_string(),
        price: 10,
    };

    // Serialize your struct with MessagePack
    let order_message = Structs::LimitOrder(order);
    let mut buf = Vec::new();
    order_message.serialize(&mut Serializer::new(&mut buf)).unwrap();

    //loop {
        for _ in 0..=0 {
        // Send the serialized data
        let msg = Message::from(buf.as_slice());
        socket.send(msg).unwrap();
        thread::sleep(Duration::from_nanos(10));
        
        }
   // }
    
}
