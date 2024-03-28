use amiquip::{Connection, ConsumerOptions, Exchange, Publish, QueueDeclareOptions, Result,ConsumerMessage};
use core::slice;
use std::{ops::Deref, str::Bytes, sync::{mpsc::channel, Arc, Mutex}, thread::{self, spawn}, time::Duration, vec};
use rand::Rng;
use crossbeam_channel::{bounded, unbounded, Receiver, Sender};
use serde::{Deserialize, Serialize};
use serde_json;
use lazy_static::lazy_static;

// Colour reformating 
const ANSI_BOLD_GREEN: &str = "\x1b[1;32m"; // Bold green color
const ANSI_RESET: &str = "\x1b[0m"; // Reset color and style
const ANSI_BOLD_RED: &str = "\x1b[1;31m"; // Bold red color

#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct Stock{
    pub name:String,
    pub value:f64,
}

pub struct  StockProfile{
    pub name:String,
    pub cur_price: f64,
    pub sold_vol:i128,
    pub buy_vol: i128,
}
#[derive(Clone, Debug, Serialize, Deserialize)]

pub struct User{
    id:i8,
    stock_name: String,
    bid_price:f64,
    take_profit:f64,
    cut_loss:f64,
    num_stock:i128,
    // holding_time: i32, // capture enter time
}


lazy_static::lazy_static! {
    static ref STOCK_PROFILES: Arc<Mutex<Vec<StockProfile>>> = Arc::new(Mutex::new(Vec::new()));
}



impl StockProfile {
    // Update buy vol
    pub fn add_stock_profile(name: String, sold_vol: i128, buy_vol: i128) {
        let mut new_profile = true;
        let mut profiles = STOCK_PROFILES.lock().unwrap();
        for stock in profiles.iter_mut(){
            if stock.name == name{
                stock.buy_vol+=buy_vol;
                new_profile = false;
            }
        }
        let name_cloned = name.clone();
        // if the stock havent been purchased before
        if new_profile==true{
            let mut stocklist = STOCK_LIST.lock().unwrap(); 
            for i in stocklist.iter(){
                if name_cloned == i.name{
                    profiles.push(StockProfile{name:name_cloned.clone(),cur_price:i.value,sold_vol,buy_vol});
                }
            }
        }
    }

    /// Update sell vol
    pub fn update_sell_vol(sold_stocks: Vec<(String,i128)>){
        let mut profiles = STOCK_PROFILES.lock().unwrap();
        for stock in profiles.iter_mut(){
           for s in sold_stocks.iter(){
                if stock.name == s.0{
                    stock.sold_vol+=s.1;
                }
           } 
        } 
    }

    // // Get up or down status
    pub fn detect_up_trend() -> (Vec<(String,f64)>,bool){
        let mut profiles = STOCK_PROFILES.lock().unwrap();
        let mut stock_up_trend: Vec<(String,f64)> = Vec::new();
        let mut uptrend = false;
        println!("**uptrend validate");
        for stock in profiles.iter_mut(){
            println!("******* stock [{}] quntity {}",stock.name,stock.buy_vol);
            if stock.buy_vol>=15{ // validate how many stock will affect upternd
                let stock_name_cloned = stock.name.clone();
                // increase 10%
                let new_stock_price = stock.cur_price*1.1;
                stock.cur_price *=1.1;
                stock_up_trend.push((stock_name_cloned,new_stock_price));
                stock.buy_vol-=100;
                uptrend = true;
            }
        }
        (stock_up_trend,uptrend)
    }

    pub fn detect_down_trend() -> (Vec<(String,f64)>,bool){
        let mut profiles = STOCK_PROFILES.lock().unwrap();
        let mut stock_down_trend: Vec<(String,f64)> = Vec::new();
        let mut dwntrend = false;
        println!("**dwntrend validate");
        for stock in profiles.iter_mut(){
            if stock.sold_vol>=50{
                let stock_name_cloned = stock.name.clone();
                // decrease stock price
                let new_stock_price = stock.cur_price*1.1;
                stock.cur_price *=1.1;
                stock_down_trend.push((stock_name_cloned,new_stock_price));
                stock.sold_vol-=100;
                dwntrend = true;
            }
        }
        (stock_down_trend,dwntrend) 
    }
}

lazy_static! {
    pub static ref STOCK_LIST: Arc<Mutex<Vec<Stock>>> = {
        let stocks = vec![
            Stock { name: String::from("apl"), value: 101.00 },
            Stock { name: String::from("mst"), value: 91.00 },
            Stock { name: String::from("dell"), value: 73.00 },
            Stock { name: String::from("ibm"), value: 51.00 },
            Stock { name: String::from("petg"), value: 82.00 },
            Stock { name: String::from("mly"), value: 65.00 },
            Stock { name: String::from("max"), value: 44.00 },
            Stock { name: String::from("gnt"), value: 77.00 },
            Stock { name: String::from("airm"), value: 89.00 },
            Stock { name: String::from("bbt"), value: 60.00 },
            Stock { name: String::from("tpx"), value: 72.00 },
            Stock { name: String::from("mmh"), value: 55.00 },
            Stock { name: String::from("sunr"), value: 68.00 },
            Stock { name: String::from("kct"), value: 93.00 },
            Stock { name: String::from("pgg"), value: 78.00 },
            Stock { name: String::from("tem"), value: 81.00 },
            Stock { name: String::from("sbc"), value: 47.00 },
            Stock { name: String::from("cmn"), value: 64.00 },
            Stock { name: String::from("smb"), value: 70.00 },
            Stock { name: String::from("jlg"), value: 85.00 },
            Stock { name: String::from("cap"), value: 49.00 },
            Stock { name: String::from("gmx"), value: 76.00 },
            Stock { name: String::from("ant"), value: 67.00 },
            Stock { name: String::from("bbl"), value: 90.00 },
            Stock { name: String::from("mmc"), value: 58.00 },
            Stock { name: String::from("dph"), value: 71.00 },
            Stock { name: String::from("szb"), value: 83.00 },
            Stock { name: String::from("fsl"), value: 52.00 },
            Stock { name: String::from("pcb"), value: 74.00 },
            Stock { name: String::from("sjc"), value: 69.00 },
            Stock { name: String::from("sel"), value: 87.00 },
            Stock { name: String::from("pwr"), value: 63.00 },
            Stock { name: String::from("klh"), value: 80.00 },
            Stock { name: String::from("svm"), value: 54.00 },
            Stock { name: String::from("rbx"), value: 86.00 },
            Stock { name: String::from("grd"), value: 50.00 },
            Stock { name: String::from("tep"), value: 75.00 },
            Stock { name: String::from("nmb"), value: 62.00 },
            Stock { name: String::from("tlc"), value: 79.00 },
            Stock { name: String::from("apm"), value: 53.00 },
            Stock { name: String::from("ktm"), value: 48.00 },
            Stock { name: String::from("bnt"), value: 66.00 },
            Stock { name: String::from("pdm"), value: 91.00 },
            Stock { name: String::from("qlt"), value: 84.00 },
            Stock { name: String::from("trn"), value: 61.00 },
            Stock { name: String::from("txl"), value: 88.00 },
            Stock { name: String::from("mnt"), value: 59.00 },
            Stock { name: String::from("kbb"), value: 46.00 },
            Stock { name: String::from("snc"), value: 72.00 },
            Stock { name: String::from("gmp"), value: 81.00 },
            Stock { name: String::from("npx"), value: 73.00 },
            Stock { name: String::from("sln"), value: 54.00 },
            Stock { name: String::from("mbt"), value: 67.00 },
            Stock { name: String::from("gkt"), value: 92.00 },
            Stock { name: String::from("pld"), value: 75.00 },
            Stock { name: String::from("nff"), value: 60.00 },
            Stock { name: String::from("zmx"), value: 78.00 },
            Stock { name: String::from("bpc"), value: 83.00 },
            Stock { name: String::from("klb"), value: 45.00 },
            Stock { name: String::from("bsn"), value: 74.00 },
        ];
        Arc::new(Mutex::new(stocks))
    };
}

impl STOCK_LIST {
    pub fn update_stock_price(name:String,uptrend:bool){
        let mut stocks = STOCK_LIST.lock().unwrap();
        for s in stocks.iter_mut(){
            if s.name == name && uptrend{
                s.value *= 1.1;
            }else if s.name == name && !uptrend  {
                s.value *= 0.9; 
            }
        }
    }
}

// #NOTE: change
pub fn check_methods(){
    println!("** Checked!");
}

fn user_request(id:i8,stock_list: Arc<Mutex<Vec<Stock>>>)-> User{
    let mut rng = rand::thread_rng();
    let stocks = stock_list.lock().unwrap();
    let stock = &stocks[rng.gen_range(0..60)];
    let stockname = stock.name.clone();
    let bidprice = stock.value; 
    let takeprofit = stock.value *  (1.0 + rng.gen_range(0.15..=2.0)); 
    let cutloss = stock.value *  (1.0 - rng.gen_range(0.12..=0.4)); 
    // let hldtime = rng.gen_range(3..8);
    let numstock = rng.gen_range(1..=30);
    
    User{id:id,stock_name:stockname,bid_price:bidprice,take_profit:takeprofit,cut_loss:cutloss,num_stock:numstock}
}




fn main(){
    // internal channel
    let (sl_tx,sl_rx) = channel(); 

    // Exhanges
    thread::spawn(move||->Result<()>{
        
        /* ---------------------- Sender & Connection--------------------- */
        // define connections
        let mut connection = Connection::insecure_open("amqp://guest:guest@localhost:5672")?;
        
        // define sender
        let ex_br_mq = connection.open_channel(None)?; 
        let send_stock_list = Exchange::direct(&ex_br_mq);

        /* ---------------------- receiver--------------------- */
        // define queue for exchange to receive stock's vol info 
        let stock_pur_vol = ex_br_mq.queue_declare("updatePurVol", QueueDeclareOptions::default())?;
        let stock_sold_vol = ex_br_mq.queue_declare("updateSoldVol", QueueDeclareOptions::default())?;
        
        let ex_pur_recv = stock_pur_vol.consume(ConsumerOptions::default())?;


        loop{
            // send list for customer
            println!("Exchange: Stock list publishing..");
            thread::sleep(Duration::from_secs(3)); // NOTE: try this 
            sl_tx.send(STOCK_LIST.clone()).unwrap();
            // convert arc::mutex back to normal vec
            let clone_stock_arc = Arc::clone(&STOCK_LIST);
            let vec_stock_list = clone_stock_arc.lock().unwrap().clone();
            let stock_list_json = serde_json::to_string(&vec_stock_list).expect("Failed to serialize");
            // send stock list to broker1
            send_stock_list.publish(Publish::new(stock_list_json.as_bytes(),"sentStockInfoBrk1"))?;
            
            // Get purchaase info from broker and update to stock profile
            for ul in ex_pur_recv.receiver().iter(){
                match ul {
                    ConsumerMessage::Delivery(delivery)=>{
                            let body = String::from_utf8_lossy(&delivery.body);
                            let user_list: User= serde_json::from_str(&body).expect("Failed to desialize");
                            StockProfile::add_stock_profile(user_list.stock_name, 0,user_list.num_stock);
                        ex_pur_recv.ack(delivery)?;
                        break;
                    }
                    other =>{
                        println!("Exchange ended here{:?}",other);
                        break;
                    }
                }
            }

            // check up trends
            let (up_stock_list, uptrend) = StockProfile::detect_up_trend();
            println!("Exchange: Currently checking on uptrend...");
            thread::sleep(Duration::from_millis(200)); // NOTE: try this  
            if uptrend{
                for stock in up_stock_list.iter(){
                    println!("{}Exchange: Stock [{}] was on fire!!{}",ANSI_BOLD_GREEN,stock.0,ANSI_RESET);
                    // update STOCK_LIST price
                    STOCK_LIST::update_stock_price((stock.0).clone(),true);
                    // send uptrend info for broker 1
                    let stock_profile_json = serde_json::to_string(&stock).expect("Failed to serialize");
                    println!("** here 1{}",stock_profile_json);
                    println!("** here 1 as bytes{:?}",stock_profile_json.as_bytes());
                    send_stock_list.publish(Publish::new(stock_profile_json.as_bytes(),"sentStockTrendingBrk1"))?;
                    // send uptrend info for broker 2
                }    
            }

            // check down trend
            let (down_trend_stock,dwntrend) = StockProfile::detect_down_trend();
            println!("Exchange: Currently checking on downstrend...");
            if dwntrend{
                for stock in down_trend_stock.iter(){
                    println!("{}Exchange: Stock [{}] was dropping!!{}",ANSI_BOLD_RED,stock.0,ANSI_RESET);
                    // update stock price
                    STOCK_LIST::update_stock_price((stock.0).clone(),false);
                    // let join_stock_info = format!("{}+{}",stock.0,stock.1);
                    // send downtrend info for broker 1
                    // send_stock_list.publish(Publish::new(join_stock_info.as_bytes(),"sentStockInfoBrk1"))?; 
                    // send uptrend info for broker 2
                }     
            }

            // send_stock_list.publish(Publish::new(stock_list_u8, "sentStockInfoBrk1"))?; 
        }
        connection.close() //#TODO: CHECK ON THIS
    });


    
    // Users
    thread::spawn(move|| -> Result<()> {
        let mut connection = Connection::insecure_open("amqp://guest:guest@localhost:5672")?;
        for i in 1..11{
            println!("User{}: Enter page..",i);
            println!("User{}: Page loading..",i);
            // make sure didn't miss out customer in the laoding page
            loop{
                match sl_rx.try_recv(){
                    Ok(stock_list)=>{
                       
                        let usr_br_mq = connection.open_channel(None)?; // user and br mq channel
                        let send_order = Exchange::direct(&usr_br_mq);
                        println!("User{}: Viewing the stock list",i); 
                        println!("User{}: Selecting stokcs...",i); 
                        thread::sleep(Duration::from_millis(5));  
                        let mut rng = rand::thread_rng();
                        // decide buy how many type of stock 
                        for _ in 1..=rng.gen_range(1..=5){
                            let choose_broker = rng.gen_range(1..=2);
                            println!("User{}: System choosing brokers..",i); 
                            thread::sleep(Duration::from_millis(5));  
                            let choose_broker = rng.gen_range(1..=2);
                            println!("User{}: System choosing brokers..",i); 
                            thread::sleep(Duration::from_millis(5));  
                            let user_req_list = user_request(i,stock_list.clone());
                            let user_list_json =serde_json::to_string(&user_req_list).expect("Failed to serialized");
                            send_order.publish(Publish::new(user_list_json.as_bytes(), "linktobr1"))?;
                        }
                        break;
                    }
                    Err(_)=>{
                        println!("User{}: Still loading",i);
                        thread::sleep(Duration::from_secs(3));  
                    } 
                }
            }
        }
        connection.close()
    });
    loop{};
}