use amiquip::{Connection, Exchange, QueueDeclareOptions,ConsumerOptions,Publish, Result};
use core::slice;
use std::{ops::Deref, str::Bytes, sync::{mpsc::channel, Arc, Mutex}, thread::{self, spawn}, time::Duration, vec};
use rand::Rng;
use crossbeam_channel::{bounded, unbounded, Receiver, Sender};
use serde::Serialize;
use serde_json;
use lazy_static::lazy_static;

#[derive(Clone,Serialize)]
pub struct Stock{
    pub name:String,
    pub value:f64,
}
pub struct User{
    id:i8,
    stock_name: String,
    bid_price:f64,
    take_profit:f64,
    cut_loss:f64,
    num_stock:i128,
    // holding_time: i32, // capture enter time
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
    let numstock = rng.gen_range(1..=100);
    
    User{id:id,stock_name:stockname,bid_price:bidprice,take_profit:takeprofit,cut_loss:cutloss,num_stock:numstock}
}

pub fn iterate_stock_list(share_stock: &Arc<Mutex<Vec<Stock>>>,stock_name:String,bid_price:f64)-> Option<Stock>{
    let stock = share_stock.lock().unwrap();
    for s in stock.iter(){
        if s.name == stock_name && s.value == bid_price{
            return Some(s.clone())
        }
    }
    None
}

fn to_string(user: &User)->String{
    let delimiter = "+";
    format!(
        "{}{}{}{}{}{}{}{}{}{}{}",
        user.id, delimiter, user.stock_name, delimiter,
        user.bid_price, delimiter, user.take_profit, delimiter,
        user.cut_loss, delimiter, user.num_stock
    )

}

fn main(){
    // internal channel
    let (sl_tx,sl_rx) = channel(); 

    // Exhanges
    thread::spawn(move||->Result<()>{
        let mut connection = Connection::insecure_open("amqp://guest:guest@localhost:5672")?;
        loop{
            // send list for customer
            println!("Exchange: Stock list publishing..");
            thread::sleep(Duration::from_secs(2));
            sl_tx.send(STOCK_LIST.clone()).unwrap();
            let ex_br_mq = connection.open_channel(None)?; // sent stock list to broker
            let send_stock_list = Exchange::direct(&ex_br_mq);
            // convert to json 
            let json = serde_json::to_string(&*STOCK_LIST.lock().unwrap()).unwrap();
            let vec_u8 = json.into_bytes();
            let stock_list_u8 = vec_u8.as_slice();
            
            send_stock_list.publish(Publish::new(stock_list_u8, "sendOrderListbr1"))?; 
        }
        connection.close()
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
                            // let reqest_list = user_request(i, stock_list.clone());
                            let user_req_list = to_string(&user_request(i, stock_list.clone())); // join struc into one string
                            send_order.publish(Publish::new(user_req_list.as_bytes(), "linktobr1"))?;
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
    // Publish a message to the "hello" queue.
    // connection.close()
}