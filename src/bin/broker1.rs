use std::{ sync::{ Arc, Mutex}, thread::{self, spawn}, time::Duration, vec};
use amiquip::{Connection, ConsumerMessage, ConsumerOptions, Delivery, Exchange, Publish, QueueDeclareOptions, Result};
use serde::{Deserialize, Serialize};

use crate::stock::StockProfile;
mod stock;

// Colour reformating 
const ANSI_BOLD_GREEN: &str = "\x1b[1;32m"; // Bold green color
const ANSI_RESET: &str = "\x1b[0m"; // Reset color and style
const ANSI_BOLD_RED: &str = "\x1b[1;31m"; // Bold red color

#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct User{
    id:i8,
    stock_name: String,
    bid_price:f64,
    take_profit:f64,
    cut_loss:f64,
    num_stock:i128,
}
#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct Stock{
    pub name:String,
    pub value:f64,
}

struct purchase_details{
    id:i8,
    stock_name: String,
    take_profit:f64,
    cut_loss:f64,
    num_stock:i128,
}

lazy_static::lazy_static!{
    static ref PURCHASE_HISTORY: Arc<Mutex<Vec<purchase_details>>> = Arc::new(Mutex::new(Vec::new()));
}

impl purchase_details{
   fn add_order(id: i8,stock_name:String, take_profit:f64, cut_loss:f64,num_stock:i128) {
        let mut new_user_stocks = true; // refer to this user didn't purchase this stock before
        let mut records = PURCHASE_HISTORY.lock().unwrap();
        for d in records.iter_mut(){
            if d.stock_name == stock_name{
                d.num_stock+=num_stock;               
                new_user_stocks = false;
            }
        }
        if new_user_stocks==true{records.push(purchase_details{id,stock_name,take_profit,cut_loss,num_stock})};
   }

    fn stock_sell_monitoring(stock_name:String, current_stock_price: f64)-> Vec<(String,i128)> {
        let mut records = PURCHASE_HISTORY.lock().unwrap(); 
        let mut to_remove: Vec<usize> = Vec::new();
        let mut sold_stocks: Vec<(String,i128)> = Vec::new();
        println!("** sell monitoring");
        for (index,d) in records.iter_mut().enumerate(){
            if d.stock_name == stock_name{
                if d.cut_loss <= current_stock_price{
                    println!("{}Broker 1: Had sold User {}'s [{}] for cutting loss!{}",ANSI_BOLD_RED,d.id,d.stock_name,ANSI_RESET);
                    to_remove.push(index);
                    sold_stocks.push((d.stock_name.clone(),d.num_stock));
                }else if d.take_profit >= current_stock_price {
                    println!("{}Broker 1: Had sold User {}'s [{}] for taking profit!{}",ANSI_BOLD_GREEN,d.id,d.stock_name,ANSI_RESET);
                    to_remove.push(index);
                    sold_stocks.push((d.stock_name.clone(),d.num_stock));
                }
            }
        } 
        // remove old records
        for &index in to_remove.iter().rev() {
            records.remove(index);
        }
        sold_stocks
    }
}

// check whether user's bid price reach the budget or not
pub fn iterate_stock_list(stock_list: &Vec<Stock>,stock_name:String,bid_price:f64)-> Option<Stock>{
    for s in stock_list.iter(){
        if s.name == stock_name && s.value == bid_price{
            return Some(s.clone())
        }
    }
    None
}


fn main() -> Result<()> {
    // Open connection.
    
    let mut connection = Connection::insecure_open("amqp://guest:guest@localhost:5672")?;

    // Open a channel - None says let the library choose the channel ID.
    let channel = connection.open_channel(None)?;
    
    /* ---------------------- Receiver --------------------- */
    // Key
    let queue = channel.queue_declare("linktobr1", QueueDeclareOptions::default())?;
    let stock_info_queue = channel.queue_declare("sentStockInfoBrk1", QueueDeclareOptions::default())?;
    let stock_trending_queue = channel.queue_declare("sentStockTrendingBrk1", QueueDeclareOptions::default())?;
    // Start a broker1 receiver
    let consumer = queue.consume(ConsumerOptions::default())?;
    let exch_brk1_stock_list= stock_info_queue.consume(ConsumerOptions::default())?;
    let exch_brk1_stock_trend= stock_trending_queue.consume(ConsumerOptions::default())?;
    
    // let message = queue.consume(amiquip::ConsumerOptions::default())?.receiver().recv()?;

    /* ---------------------- Sender --------------------- */
    let updateVolStatus = Exchange::direct(&channel);
    
    println!("Waiting for messages. Press Ctrl-C to exit.");
    // Define Stock vec list
    let mut STOCK_LIST: Vec<Stock> = Vec::new();

    // Get stock list
    for (i,stock_list )in exch_brk1_stock_list.receiver().iter().enumerate(){
        match stock_list{
            ConsumerMessage::Delivery(delivery) => {
                    let stock_list_body = String::from_utf8_lossy(&delivery.body);
                    STOCK_LIST = serde_json::from_str(&stock_list_body).expect("Failed to deserialize");
                    exch_brk1_stock_list.ack(delivery)?;
                    break;
                }
                other => {
                    println!("Broker1: Stock list endedf:{:?}",other);
                    break;
                }
        }
    } 
    
    
    // User request -> buy
    for (i, message) in consumer.receiver().iter().enumerate() {
        if i == 10{break;}
        
        match message {
            ConsumerMessage::Delivery(delivery) => {
                let body = String::from_utf8_lossy(&delivery.body);
                let user_list: User = serde_json::from_str(&body).expect("Failed to desialize");
                println!("Broker 1: had received order from User{}",user_list.id);
                // Get order list from exchange
                // check user budget meet anot
                let user_list_clone = user_list.clone();
                let stock_name = user_list.stock_name.clone();
                let chosen_stock = iterate_stock_list(&STOCK_LIST, user_list.stock_name, user_list.bid_price);
                match chosen_stock{
                    Some(chosen_stock)=>{
                        thread::sleep(Duration::from_secs(1));
                        println!("{}Broker 1: had successfully purchased [{}] stock with {} units for User {}{}",ANSI_BOLD_GREEN,chosen_stock.name,user_list.num_stock,user_list.id,ANSI_RESET);
                        // save purchase records
                        purchase_details::add_order(user_list.id,stock_name,user_list.take_profit,user_list.cut_loss,user_list.num_stock);
                        // Send info back to exchange channel
                        let user_list_json = serde_json::to_string(&user_list_clone).expect("Failed to serialized");
                        updateVolStatus.publish(Publish::new(user_list_json.as_bytes(),"updatePurVol"))?;
                    }
                    None =>{
                        println!("{}Broker 1: shares was overpriced for User {}'s order!{}",ANSI_BOLD_RED,user_list.id,ANSI_RESET);
                    }    
                }
                consumer.ack(delivery)?;
            }
            other => {
                println!("Consumer ended: {:?}", other);
                break;
            }
        }
    }

    // User request -> sell (monitoring)
    for stocks in exch_brk1_stock_trend.receiver().iter(){
        match stocks {
            ConsumerMessage::Delivery(delivery) =>{
                let stock_profile_body = String::from_utf8_lossy(&delivery.body);
                // let mut cleaned_str = body.into_owned();
                let stock_profile: (String,f64) = serde_json::from_str(&stock_profile_body).expect("Failed to deserialize");
                let stock_name = stock_profile.0;
                purchase_details::stock_sell_monitoring(stock_name, stock_profile.1);
                exch_brk1_stock_trend.ack(delivery)?;
            }
            other => {
                println!("Broker1: Stock trending list ended{:?}",other);
            }
        }
    }    


    connection.close()
}


// ["szb",91.30000000000001]
// [\"txl\",96.80000000000001]

