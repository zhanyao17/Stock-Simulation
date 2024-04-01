use amiquip::{Connection, ConsumerOptions, Exchange, Publish, QueueDeclareOptions, Result,ConsumerMessage};
use scheduled_thread_pool::ScheduledThreadPool;
use std::{sync::{mpsc::channel, Arc, Mutex}, thread, time::Duration, vec};
use rand::Rng;
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use serde_json;
use lazy_static::lazy_static;


#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct Stock{
    pub name:String,
    pub value:f64,
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

pub struct NewsTitle{
    content: String,
}

impl NewsTitle {
    pub fn gen_content()-> Vec<NewsTitle>{
        vec![
            NewsTitle {content: "'Economic Slowdown Predicted: Analysts Warn of Recession' - The Star News".to_string()},
            NewsTitle {content: "'Major Company Reports Disappointing Quarterly Earnings' - Sin Chew Daily News".to_string()},
            NewsTitle {content: "'Trade Tensions Escalate: Tariffs Imposed on Key Imports' - Nan Yang News".to_string()},
            NewsTitle {content: "'Government Announces Tightening of Monetary Policy' - The Chinese News".to_string()},
            NewsTitle {content: "'Tech Giant Faces Regulatory Probe Over Data Privacy Concerns' - BBC News".to_string()},
        ]
    }
}

pub struct  StockProfile{
    pub name:String,
    pub cur_price: f64,
    pub sold_vol:i128,
    pub buy_vol: i128,
}

#[allow(dead_code)]
#[warn(unused_mut,unreachable_code)]
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
            let stocklist = STOCK_LIST.lock().unwrap(); 
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

    // Randomly generate stock decrease base on news
    pub fn external_down_trend()->(i32,Vec<String>){
        let mut rng = rand::thread_rng();
        let num_stock = rng.gen_range(2..5);
        let mut profiles = STOCK_PROFILES.lock().unwrap();
        let news_topic = rng.gen_range(0..5);
        let mut affected_stock: Vec<String> = Vec::new();
        for _ in 0..num_stock{
            let stock = rng.gen_range(0..profiles.len());
            profiles.shuffle(&mut rng); // suffle the profile seq (prev choosing the same stock)
            profiles[stock].sold_vol +=20; // increase the sold vol 
            affected_stock.push(profiles[stock].name.clone());
        }
        (news_topic,affected_stock)
    }

    // detect up trend
    pub fn detect_up_trend() -> (Vec<(String,f64)>,bool){
        let mut profiles = STOCK_PROFILES.lock().unwrap();
        let mut stock_up_trend: Vec<(String,f64)> = Vec::new();
        let mut uptrend = false;
        for stock in profiles.iter_mut(){
            let count = stock.buy_vol/30; // same logic like if stock.buy_vol >=30
            if count >= 1{
                let stock_name_cloned = stock.name.clone();
                stock.cur_price *=1.0+(0.1*(count as f64)); 
                let new_stock_price = stock.cur_price;
                stock_up_trend.push((stock_name_cloned,new_stock_price));
                uptrend = true;
                stock.buy_vol-=count * 30; // minus back the converted num of stocks
            }
        }
        (stock_up_trend,uptrend)
    }
    
    // detect down trend
    pub fn detect_down_trend() -> (Vec<(String,f64)>,bool){
        let mut profiles = STOCK_PROFILES.lock().unwrap();
        let mut stock_down_trend: Vec<(String,f64)> = Vec::new();
        let mut dwntrend = false;
        for stock in profiles.iter_mut(){
            let count = stock.sold_vol/15;
            if count >= 1{
                let stock_name_cloned = stock.name.clone();
                stock.cur_price *=1.0-(0.1*(count as f64));
                let new_stock_price = stock.cur_price;
                stock_down_trend.push((stock_name_cloned,new_stock_price));
                stock.sold_vol-=count * 15;
                dwntrend = true;
            }
        }
        (stock_down_trend,dwntrend) 
    }
}

lazy_static::lazy_static! {
    static ref STOCK_PROFILES: Arc<Mutex<Vec<StockProfile>>> = Arc::new(Mutex::new(Vec::new()));
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

#[allow(dead_code)]
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

#[allow(dead_code)]
fn user_request(id:i8,stock_list: Arc<Mutex<Vec<Stock>>>)-> User{
    let mut rng = rand::thread_rng();
    let stocks = stock_list.lock().unwrap();
    let stock = &stocks[rng.gen_range(0..60)];
    let stockname = stock.name.clone();
    let bidprice = stock.value; 
    let takeprofit = stock.value *  (1.0 + rng.gen_range(0.05..=0.1)); 
    let cutloss = stock.value *  (1.0 - rng.gen_range(0.02..=0.08));
    // let hldtime = rng.gen_range(3..8);
    let numstock = rng.gen_range(1..=30);
    
    User{id:id,stock_name:stockname,bid_price:bidprice,take_profit:takeprofit,cut_loss:cutloss,num_stock:numstock}
}


#[allow(dead_code)]
fn main(){
    // define formating colour
    const ANSI_BOLD_GREEN: &str = "\x1b[1;32m"; // Bold green color
    const ANSI_RESET: &str = "\x1b[0m"; // Reset color and style
    const ANSI_BOLD_RED: &str = "\x1b[1;31m"; // Bold red color 

    // Generate news struct
    let new_title_list = NewsTitle::gen_content();

    // internal channel
    let (sl_tx,sl_rx) = channel();  //#NOTE: change to crossbeam-channel

    // no of customer
    let no_cust_ori = Arc::new(Mutex::new(0));
    let no_cust_ex = Arc::clone(&no_cust_ori);
    let no_cust_user = Arc::clone(&no_cust_ori);

    // Triggered infinity loop in main threads
    let ex_final = Arc::new(Mutex::new(false));
    let ex_final_ex = Arc::clone(&ex_final);
    let ex_final_main = Arc::clone(&ex_final);
    // static mut no_cust: Mutex<i32> = Mutex::new(0);
 
    // define num of shed threads
    let sched = ScheduledThreadPool::new(2);

    // Exhanges threads
    sched.execute_at_fixed_rate(
        Duration::from_millis(50), 
        Duration::from_secs(1), 
        move || {
            let mut connection = Connection::insecure_open("amqp://guest:guest@localhost:5672").expect("Failed to open connection");
        
            // Triggered stock news
            let mut trig_news = 0;
            
            // Define sender
            let ex_br_mq = connection.open_channel(None).expect("Failed to open channel");
            let send_stock_list = Exchange::direct(&ex_br_mq);

            /* ---------------------- receiver--------------------- */
            // Define queue for exchange to receive stock's vol info 
            let stock_pur_vol = ex_br_mq.queue_declare("updatePurVol", QueueDeclareOptions::default())
            .unwrap_or_else(|err| panic!("Error declaring queue: {:?}", err));
            let stock_sold_vol = ex_br_mq.queue_declare("updateSoldVol", QueueDeclareOptions::default())
            .unwrap_or_else(|err| panic!("Error declaring queue: {:?}", err));
            
            // let ex_pur_recv = stock_pur_vol.consume(ConsumerOptions::default()).expect("Failed to start consumer");
            let ex_pur_recv = stock_pur_vol.consume(ConsumerOptions::default()).unwrap_or_else(|err| panic!("Error starting consumer: {:?}", err));
            let ex_sell_recv = stock_sold_vol.consume(ConsumerOptions::default()).unwrap_or_else(|err| panic!("Error starting consumer: {:?}", err));
            // let ex_sell_recv = stock_sold_vol.consume(ConsumerOptions::default()).expect("Failed to start consumer");


            loop{
                // Detect ending 
                let mut ending=0;
                trig_news +=1;
                
                // Send list for customer
                println!("Exchange: Stock list publishing..");
                thread::sleep(Duration::from_secs(1)); 
                let no_cust_clone = no_cust_ex.lock().unwrap();
                if *no_cust_clone < 10{sl_tx.send(STOCK_LIST.clone()).unwrap();} // prevent threads panic

                // Send stock list to broker
                let clone_stock_arc = Arc::clone(&STOCK_LIST);
                let vec_stock_list = clone_stock_arc.lock().unwrap().clone();
                let stock_list_json = serde_json::to_string(&vec_stock_list).expect("Failed to serialize"); 
                let _ = send_stock_list.publish(Publish::new(stock_list_json.as_bytes(),"sentStockInfoBrk1")); // broker1
                let _ = send_stock_list.publish(Publish::new(stock_list_json.as_bytes(),"sentStockInfoBrk2")); // broker2
                println!("Exchange: Had send stock list to broker1 & 2");
                
                // Get purchaase info from broker and update to stock profile
                let timeout_purchasing_monitor = Duration::from_secs(5); // Adjust as needed
                let mut receive_purchase_order = false;
                loop{
                    if !receive_purchase_order{println!("Exchange: Had received new purchase order!!")};
                    receive_purchase_order = true;
                    match ex_pur_recv.receiver().recv_timeout(timeout_purchasing_monitor) {
                        Ok (ul)=>{
                            match ul{
                                ConsumerMessage::Delivery(delivery)=>{
                                    ending-=1;
                                    let body = String::from_utf8_lossy(&delivery.body);
                                    let user_list: User= serde_json::from_str(&body).expect("Failed to desialize");
                                    StockProfile::add_stock_profile(user_list.stock_name, 0,user_list.num_stock);
                                    let _ = ex_pur_recv.ack(delivery);
                                }
                                other =>{
                                    println!("Exchange ended here{:?}",other);
                                    break;
                                }
                            }
                        }
                        Err(_)=>{
                            // println!("Exchange - update pur vol: Timeout reached. No message received.");
                            ending+=1;
                            break;
                        }
                    }
                }

                // Update sell vol
                let timeout_selling_monitor = Duration::from_secs(5);
                loop{
                    match ex_sell_recv.receiver().recv_timeout(timeout_selling_monitor){
                        Ok(ul)=>{
                            match ul{
                                ConsumerMessage::Delivery(delivery)=>{
                                    ending=0;
                                    let body = String::from_utf8_lossy(&delivery.body);
                                    let received_stocks: Vec<(String, i128)> = serde_json::from_str(&body).expect("Failed to deserialize");
                                    StockProfile::update_sell_vol(received_stocks.clone());
                                    let _ = ex_sell_recv.ack(delivery);
                                }
                                other =>{
                                    println!("Exchange ended here{:?}",other);
                                    break;
                                }
                            }
                        }
                        Err(_)=>{
                            println!("Exchange - update sell vol: Timeout reached. No message received.");
                            ending+=1;
                            break;
                        }
                    }
                }

                // Check up trends
                let (up_stock_list, uptrend) = StockProfile::detect_up_trend();
                println!("Exchange: Currently checking on uptrend...");
                thread::sleep(Duration::from_millis(200)); // NOTE: try this  
                if uptrend{
                    for stock in up_stock_list.iter(){
                        println!("{}Exchange: Stock [{}] was on fire!! - current price: {} {}",ANSI_BOLD_GREEN,stock.0,stock.1.round(),ANSI_RESET);
                        // update STOCK_LIST price
                        STOCK_LIST::update_stock_price((stock.0).clone(),true);
                        // send uptrend info for broker 1
                        let stock_profile_json = serde_json::to_string(&stock).expect("Failed to serialize");
                        let _ = send_stock_list.publish(Publish::new(stock_profile_json.as_bytes(),"sentStockTrendingBrk1"));
                        // send uptrend info for broker 2
                        let _ = send_stock_list.publish(Publish::new(stock_profile_json.as_bytes(),"sentStockTrendingBrk2")); 
                    }    
                }

                // Adding external factor to sped up the cut lose action
                let mut got_news = false;
                let mut downtrend_news= 0;
                let mut affected_stocks: Vec<String> = Vec::new();
                if trig_news == 2 || trig_news == 4 || trig_news == 5 || trig_news == 7{
                    got_news = true;
                    (downtrend_news,affected_stocks) = StockProfile::external_down_trend();

                }
                
                // Check down trend
                let (down_trend_stock,dwntrend) = StockProfile::detect_down_trend();
                println!("Exchange: Currently checking on downstrend...");
                if dwntrend{
                    if got_news{
                        let news = &new_title_list[downtrend_news as usize];
                        let message_length = (news.content.to_owned()+" Exchange: Breaking news!! ").chars().count();
                        // Draw the top line of the box
                        println!("--{}--", "-".repeat(message_length + 4)); // +4 for extra spacing
                        println!("|  Exchange: Breaking news!! {}  |", news.content);
                        println!("--{}--", "-".repeat(message_length + 4));
                        print!("--  Affected stock: ");
                        for s in affected_stocks.iter(){
                            print!("[{}]  ",s);
                        }
                        print!("--\n")
                    }
                    for stock in down_trend_stock.iter(){
                        println!("{}Exchange: Stock [{}] was dropping!! - current price: {} {}",ANSI_BOLD_RED,stock.0,stock.1.round(),ANSI_RESET);
                        // update stock price
                        STOCK_LIST::update_stock_price((stock.0).clone(),false);
                        // send to broker 1
                        let stock_profile_json = serde_json::to_string(&stock).expect("Failed to serialize");
                        let _ = send_stock_list.publish(Publish::new(stock_profile_json.as_bytes(),"sentStockTrendingBrk1"));
                        // send to broker 2
                        let _ = send_stock_list.publish(Publish::new(stock_profile_json.as_bytes(),"sentStockTrendingBrk2"));
                    }     
                }
                //  Last round check before ending the exchange threads
                if ending == 2 && *no_cust_clone == 10{
                    println!("Exchange: There isn't have any update on stocks' orders");
                    let mut ex_final_ex_clone = ex_final_ex.lock().unwrap();
                    *ex_final_ex_clone = true; 
                    break;
                }
            }
        // Close the connection.
        connection.close()
            .unwrap_or_else(|err| eprintln!("Error closing connection: {:?}", err));
            }
    );
    

    // Users threads
    sched.execute_at_fixed_rate(
        Duration::from_millis(50),
            Duration::from_secs(1),
            move||{
            let mut connection = Connection::insecure_open("amqp://guest:guest@localhost:5672").expect("Failed to open connection");
            let mut count_user = 0;
            for i in 1..11{
                println!("User{}: Enter page..",i);
                println!("User{}: Page loading..",i);
                count_user+=1;
                // make sure didn't miss out customer in the laoding page
                loop{
                    match sl_rx.try_recv(){
                        Ok(stock_list)=>{
                            let usr_br_mq = connection.open_channel(None).expect("Failed to open channel");
                            let send_order = Exchange::direct(&usr_br_mq);
                            println!("User{}: Viewing the stock list",i); 
                            println!("User{}: Selecting stokcs...",i); 
                            thread::sleep(Duration::from_millis(5));  
                            let mut rng = rand::thread_rng();
                            // decide buy how many type of stock 
                            for _ in 1..=rng.gen_range(1..=10){
                                // let choose_broker = rng.gen_range(1..=2);
                                println!("User{}: System choosing brokers..",i); 
                                thread::sleep(Duration::from_millis(5));  
                                // let choose_broker = rng.gen_range(1..=2);
                                println!("User{}: Order had send to brokers..",i); 
                                let user_req_list = user_request(i,stock_list.clone());
                                let user_list_json =serde_json::to_string(&user_req_list).expect("Failed to serialized");
                                let _ = send_order.publish(Publish::new(user_list_json.as_bytes(), "linktobr1"));
                            }
                            break;
                        }
                        Err(_)=>{
                            println!("User{}: Still loading",i);
                            thread::sleep(Duration::from_secs(3));  
                        } 
                    }
                }
                let mut no_cust_clone = no_cust_user.lock().unwrap();
                *no_cust_clone+=1;
            }
            // Close the connection.
            connection.close().unwrap_or_else(|err| eprintln!("Error closing connection: {:?}", err));
            if count_user == 10 {thread::sleep(Duration::from_secs(50));} // triggered infinity loop in cust threads
        }
    );

    loop{
       let ex_final_main_clone = ex_final_main.lock().unwrap();
       if *ex_final_main_clone == true{break;} 
    };
}