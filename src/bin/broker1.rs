use std::borrow::Cow;

use amiquip::{Connection, ConsumerMessage, ConsumerOptions, QueueDeclareOptions, Result};
mod stock;
use serde::Serialize;
use serde_json;


fn main() -> Result<()> {
    // Open connection.
    let mut connection = Connection::insecure_open("amqp://guest:guest@localhost:5672")?;

    // Open a channel - None says let the library choose the channel ID.
    let channel = connection.open_channel(None)?;

    // Key
    let queue = channel.queue_declare("linktobr1", QueueDeclareOptions::default())?;
    // let stock_list_byte = channel.queue_declare("sendOrderListbr1", QueueDeclareOptions::default())?;

    // Start a consumer.
    let consumer = queue.consume(ConsumerOptions::default())?;
    // let get_stock_list = stock_list_byte.consume(ConsumerOptions::default())?;

    println!("Waiting for messages. Press Ctrl-C to exit.");

    for (i, message) in consumer.receiver().iter().enumerate() {
        match message {
            ConsumerMessage::Delivery(delivery) => {
                let body = String::from_utf8_lossy(&delivery.body);
                let user_list: Vec<&str> = body.split('+').collect();
                println!("Broker 1: had received order from User{:?}",user_list[0]);
                // check user budget meet anot
                let chosen_stock = stock::iterate_stock_list(&stock::STOCK_LIST, 
                        user_list[1].to_string(), user_list[2].parse().expect("Failed to parse string to f64"));
                match chosen_stock{
                    Some(chosen_stock)=>{
                        println!("User preferences stock: {}",user_list[0]);
                        println!("Stock name: {}",chosen_stock.name);
                    }
                    None =>{
                        println!("Budget over?")
                    }    
                }
                // Iterate the parts vec 
                // for i in user_list {
                //     println!("Broker 1: had received order from User: {}",i);
                // }
                consumer.ack(delivery)?;
            }
            other => {
                println!("Consumer ended: {:?}", other);
                break;
            }
        }
    }

    connection.close()
}
