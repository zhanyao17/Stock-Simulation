use std::{ sync::{ Arc, Mutex}, time::Duration};
use amiquip::{Connection, ConsumerMessage, ConsumerOptions, Exchange, Publish, QueueDeclareOptions, Result};
use serde::{Deserialize, Serialize};

// Colour reformating 
const ANSI_BOLD_GREEN: &str = "\x1b[1;32m"; // Bold green color
const ANSI_RESET: &str = "\x1b[0m"; // Reset color and style
const ANSI_BOLD_RED: &str = "\x1b[1;31m"; // Bold red color

#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct User{
    pub id:i8,
    pub stock_name: String,
    pub bid_price:f64,
    pub take_profit:f64,
    pub cut_loss:f64,
    pub num_stock:i128,
}

#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct Stock{
    pub name:String,
    pub value:f64,
}

pub struct PurchaseDetails{
    pub id:i8,
    pub stock_name: String,
    pub take_profit:f64,
    pub cut_loss:f64,
    pub num_stock:i128,
}

lazy_static::lazy_static!{
    static ref PURCHASE_HISTORY: Arc<Mutex<Vec<PurchaseDetails>>> = Arc::new(Mutex::new(Vec::new()));
}

impl PurchaseDetails{
   pub fn add_order(id: i8,stock_name:String, take_profit:f64, cut_loss:f64,num_stock:i128) {
        let mut new_user_stocks = true; // refer to this user didn't purchase this stock before
        let mut records = PURCHASE_HISTORY.lock().unwrap();
        for d in records.iter_mut(){
            if d.stock_name == stock_name{
                d.num_stock+=num_stock;               
                new_user_stocks = false;
            }
        }
        if new_user_stocks==true{records.push(PurchaseDetails{id,stock_name,take_profit,cut_loss,num_stock})};
   }

    pub fn stock_sell_monitoring(stock_name:String, current_stock_price: f64,broker_no: i8)-> Vec<(String,i128)> {
        let mut records = PURCHASE_HISTORY.lock().unwrap(); 
        let mut to_remove: Vec<usize> = Vec::new();
        let mut sold_stocks: Vec<(String,i128)> = Vec::new();
        for (index,d) in records.iter_mut().enumerate(){
            if d.stock_name == stock_name{
                // println!("**&& Stock Name: {} cut_loss: {} | take profit: {} | current price: {}",stock_name,d.cut_loss,d.take_profit,current_stock_price);
                if current_stock_price <= d.cut_loss {
                    let loss_rate = format!("{:.2}",(((d.cut_loss - current_stock_price)/d.cut_loss) * 100.00));
                    println!("{}Broker {}: Had sold User {}'s [{}] for cutting loss! [with ↓ {}%]{}",ANSI_BOLD_RED,broker_no,d.id,d.stock_name,loss_rate,ANSI_RESET);
                    to_remove.push(index);
                    sold_stocks.push((d.stock_name.clone(),d.num_stock));
                }else if current_stock_price >= d.take_profit   {
                    let earn_rate = format!("{:.2}",(((current_stock_price - d.take_profit)/d.take_profit) * 100.00));
                    println!("{}Broker 1: Had sold User {}'s [{}] for taking profit! [with ↑ {}%]{}",ANSI_BOLD_GREEN,d.id,d.stock_name,earn_rate,ANSI_RESET);
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
    // stock list queue
    let stock_list_shared = channel.queue_declare("linktobr1", QueueDeclareOptions::default())?;
    // users' purchase order
    let stock_info_queue = channel.queue_declare("sentStockInfoBrk1", QueueDeclareOptions::default())?; 
    // stock trend infro queue
    let stock_trending_queue = channel.queue_declare("sentStockTrendingBrk1", QueueDeclareOptions::default())?; 
    
    // Start a broker1 receiver
    let consumer = stock_list_shared.consume(ConsumerOptions::default())?;
    let exch_brk1_stock_list= stock_info_queue.consume(ConsumerOptions::default())?;
    let exch_brk1_stock_trend= stock_trending_queue.consume(ConsumerOptions::default())?;
    
    /* ---------------------- Sender --------------------- */
    let update_vol_status = Exchange::direct(&channel);
    
    println!("Waiting for messages. Press Ctrl-C to exit.");

    #[allow(non_snake_case)]
    let mut STOCK_LIST: Vec<Stock> = Vec::new(); // Define Stock vec list
    loop {
        let mut ending = 0;
        
        // Get order list
        let timeout_orderlist_duration = Duration::from_secs(5);
        loop{
            match exch_brk1_stock_list.receiver().recv_timeout(timeout_orderlist_duration) {
                Ok (stock_list) =>{
                    match stock_list {
                        ConsumerMessage::Delivery(delivery) => {
                            let stock_list_body = String::from_utf8_lossy(&delivery.body);
                            STOCK_LIST = serde_json::from_str(&stock_list_body).expect("Failed to deserialize");
                            exch_brk1_stock_list.ack(delivery)?;
                            break; // loop one time only
                        }
                        other => {
                            println!("Broker1: Stock list ended: {:?}", other);
                            break;
                        }
                    }
                }
                Err(_) => {
                    println!("Order List: Timeout reached. No message received.");
                    break;
                }
            }  
        }


        // User request -> buy
        let timeout_purchase_duration = Duration::from_secs(5); // Adjust as needed
        loop {
            match consumer.receiver().recv_timeout(timeout_purchase_duration) {
                Ok(message) => {
                    match message {
                        ConsumerMessage::Delivery(delivery) => {
                            ending-=1; // ending set to nega to prevent first exit
                            let body = String::from_utf8_lossy(&delivery.body);
                            let user_list: User = serde_json::from_str(&body).expect("Failed to deserialize");
                            println!("Broker 1: had received order from User {}", user_list.id);
                            // Get order list from exchange
                            // Check user budget
                            let user_list_clone = user_list.clone();
                            let stock_name = user_list.stock_name.clone();
                            let chosen_stock = iterate_stock_list(&STOCK_LIST, user_list.stock_name, user_list.bid_price);
                            match chosen_stock {
                                Some(chosen_stock) => {
                                    // thread::sleep(Duration::from_secs(1));
                                    println!("Broker 1: had {}successfully purchased [{}] stock {} with {} units for (User {}) - At Price: {} | {}Cut Loss: {}{} | {}Take Profit: {} {}", 
                                        ANSI_BOLD_GREEN, chosen_stock.name,ANSI_RESET, user_list.num_stock, user_list.id, chosen_stock.value.round(),ANSI_BOLD_RED,user_list.cut_loss.round(),
                                        ANSI_RESET,ANSI_BOLD_GREEN,user_list.take_profit.round(),ANSI_RESET);
                                    // Save purchase records
                                    PurchaseDetails::add_order(user_list.id, stock_name, user_list.take_profit, user_list.cut_loss, user_list.num_stock);
                                    // Send info back to exchange channel
                                    let user_list_json = serde_json::to_string(&user_list_clone).expect("Failed to serialize");
                                    update_vol_status.publish(Publish::new(user_list_json.as_bytes(), "updatePurVol"))?;
                                }
                                None => {
                                    println!("Broker 2: {}unsuccessfully{} shares [{}] were overpriced for User {}'s order!", 
                                        ANSI_BOLD_RED, ANSI_RESET, user_list_clone.stock_name,user_list.id);
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
                Err(_) => {
                    println!("Purchasing: Timeout reached. No message received.");
                    // Handle timeout condition here, e.g., break the loop or return from function
                    ending+=1;
                    break;
                }
            }
        }
        
        // monitoring -> sell action
        println!("Broker 1: Monitoring the stokcs...");
        let timeout_selling_monitor_duration = Duration::from_secs(5); // Adjust as needed
        loop {
            match exch_brk1_stock_trend.receiver().recv_timeout(timeout_selling_monitor_duration) {
                Ok(stocks) => {
                    match stocks {
                        ConsumerMessage::Delivery(delivery) => {
                            ending=0; // means still got new order comming in
                            let stock_profile_body = String::from_utf8_lossy(&delivery.body);
                            let stock_profile: (String, f64) = serde_json::from_str(&stock_profile_body).expect("Failed to deserialize");
                            let sold_result :Vec<(String,i128)> = PurchaseDetails::stock_sell_monitoring(stock_profile.0, stock_profile.1,1);
                            if !sold_result.is_empty(){
                                let sold_result_json = serde_json::to_string(&sold_result).expect("Failed to serialize");
                                update_vol_status.publish(Publish::new(sold_result_json.as_bytes(),"updateSoldVol"))?;
                            }
                        }
                        other => {
                            println!("Broker1: Stock trending list ended: {:?}", other);
                        }
                    }
                }
                Err(_) => {
                    println!("Monitor: Timeout reached. No message received.");
                    ending+=1;
                    // Handle timeout condition here, e.g., break the loop or return from function
                    break;
                }
            }
        }
        // last round check before ending the broker1 threads
        if ending == 2{
            println!("Broker 1 system end with ending");
            break;
        }
    }
    connection.close()
}
