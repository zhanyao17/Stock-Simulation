use std::{any::Any, clone, ops::Deref, sync::{mpsc::channel, Arc, Mutex}, thread, time::Duration, vec};
use rand::Rng;
use crossbeam_channel::{bounded, unbounded, Receiver, Sender};

#[derive(Debug,Clone)] 
#[allow(unused_variables)]
#[allow(unused_imports)]
struct Stock{
    name:String,
    value:f64,
}

struct User{
    id:i8,
    stock_name: String,
    bid_price:f64,
    take_profit:f64,
    cut_loss:f64,
    num_stock:i128,
    // holding_time: i32, // capture enter time
}

fn prep_stock_list() -> Vec<Stock> {
    let apple = Stock { name: String::from("apl"), value: 101.00 };
    let microsoft = Stock { name: String::from("mst"), value: 91.00 };
    let dell = Stock { name: String::from("dell"), value: 73.00 };
    let ibm = Stock { name: String::from("ibm"), value: 51.00 };
    let petg = Stock { name: String::from("petg"), value: 82.00 };
    let mly = Stock { name: String::from("mly"), value: 65.00 };
    let max = Stock { name: String::from("max"), value: 44.00 };
    let gnt = Stock { name: String::from("gnt"), value: 77.00 };
    let airm = Stock { name: String::from("airm"), value: 89.00 };
    let bbt = Stock { name: String::from("bbt"), value: 60.00 };
    let tpx = Stock { name: String::from("tpx"), value: 72.00 };
    let mmh = Stock { name: String::from("mmh"), value: 55.00 };
    let sunr = Stock { name: String::from("sunr"), value: 68.00 };
    let kct = Stock { name: String::from("kct"), value: 93.00 };
    let pgg = Stock { name: String::from("pgg"), value: 78.00 };
    let tem = Stock { name: String::from("tem"), value: 81.00 };
    let sbc = Stock { name: String::from("sbc"), value: 47.00 };
    let cmn = Stock { name: String::from("cmn"), value: 64.00 };
    let smb = Stock { name: String::from("smb"), value: 70.00 };
    let jlg = Stock { name: String::from("jlg"), value: 85.00 };
    let cap = Stock { name: String::from("cap"), value: 49.00 };
    let gmx = Stock { name: String::from("gmx"), value: 76.00 };
    let ant = Stock { name: String::from("ant"), value: 67.00 };
    let bbl = Stock { name: String::from("bbl"), value: 90.00 };
    let mmc = Stock { name: String::from("mmc"), value: 58.00 };
    let dph = Stock { name: String::from("dph"), value: 71.00 };
    let szb = Stock { name: String::from("szb"), value: 83.00 };
    let fsl = Stock { name: String::from("fsl"), value: 52.00 };
    let pcb = Stock { name: String::from("pcb"), value: 74.00 };
    let sjc = Stock { name: String::from("sjc"), value: 69.00 };
    let sel = Stock { name: String::from("sel"), value: 87.00 };
    let pwr = Stock { name: String::from("pwr"), value: 63.00 };
    let klh = Stock { name: String::from("klh"), value: 80.00 };
    let svm = Stock { name: String::from("svm"), value: 54.00 };
    let rbx = Stock { name: String::from("rbx"), value: 86.00 };
    let grd = Stock { name: String::from("grd"), value: 50.00 };
    let tep = Stock { name: String::from("tep"), value: 75.00 };
    let nmb = Stock { name: String::from("nmb"), value: 62.00 };
    let tlc = Stock { name: String::from("tlc"), value: 79.00 };
    let apm = Stock { name: String::from("apm"), value: 53.00 };
    let ktm = Stock { name: String::from("ktm"), value: 48.00 };
    let bnt = Stock { name: String::from("bnt"), value: 66.00 };
    let pdm = Stock { name: String::from("pdm"), value: 91.00 };
    let qlt = Stock { name: String::from("qlt"), value: 84.00 };
    let trn = Stock { name: String::from("trn"), value: 61.00 };
    let txl = Stock { name: String::from("txl"), value: 88.00 };
    let mnt = Stock { name: String::from("mnt"), value: 59.00 };
    let kbb = Stock { name: String::from("kbb"), value: 46.00 };
    let snc = Stock { name: String::from("snc"), value: 72.00 };
    let gmp = Stock { name: String::from("gmp"), value: 81.00 };
    let npx = Stock { name: String::from("npx"), value: 73.00 };
    let sln = Stock { name: String::from("sln"), value: 54.00 };
    let mbt = Stock { name: String::from("mbt"), value: 67.00 };
    let gkt = Stock { name: String::from("gkt"), value: 92.00 };
    let pld = Stock { name: String::from("pld"), value: 75.00 };
    let nff = Stock { name: String::from("nff"), value: 60.00 };
    let zmx = Stock { name: String::from("zmx"), value: 78.00 };
    let bpc = Stock { name: String::from("bpc"), value: 83.00 };
    let klb = Stock { name: String::from("klb"), value: 45.00 };
    let bsn = Stock { name: String::from("bsn"), value: 74.00 };

    
    // convert stock struc to vector
    let stocks = vec![
        apple, microsoft, dell, ibm, petg, mly, max, gnt, airm, bbt, tpx, mmh,
        sunr, kct, pgg, tem, sbc, cmn, smb, jlg, cap, gmx, ant, bbl, mmc, dph, 
        szb, fsl, pcb, sjc, sel, pwr, klh, svm, rbx, grd, tep, nmb, tlc, apm, 
        ktm, bnt, pdm, qlt, trn, txl, mnt, kbb, snc, gmp, npx, sln, mbt, gkt, 
        pld, nff, zmx, bpc, klb, bsn
    ];
    stocks
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

fn iterate_stock_list(share_stock: &Arc<Mutex<Vec<Stock>>>,stock_name:String,bid_price:f64)-> Option<Stock>{
    let stock = share_stock.lock().unwrap();
    for s in stock.iter(){
        if s.name == stock_name && s.value == bid_price{
            return Some(s.clone())
        }
    }
    None
}

pub fn stock_sim(){
    // Define channel
    let (sl_tx,sl_rx) = channel(); // #NOTE: direct access stock vector
    // let (sl_tx,sl_rx) = unbounded(); // #NOTE: direct access stock vector
    // let sl_rx2 = sl_rx.clone();
    // let (br_ex_tx,br_ex_rx) = channel(); // broker & exchange
    // let (br_usr_tx,br_usr_rx) = unbounded(); // user & brokers 
    // let br_usr_rx2 = br_usr_rx.clone();
    let (br_usr_tx, br_usr_rx): (Sender<User>, Receiver<User>) = unbounded(); // user & brokers channel
    // let br_usr_rx2 = br_usr_rx.clone(); // clone receive channel for broker 2
    // vector list
    let stocks = prep_stock_list();
    let shared_stocks = Arc::new(Mutex::new(stocks));
    let stock_list = shared_stocks.clone(); 

    // Exhanges
    thread::spawn(move||{
        loop{
            // send list for customer
            println!("Exchange: Stock list publishing..");
            thread::sleep(Duration::from_secs(2));
            sl_tx.send(shared_stocks.clone()).unwrap();
        }
    });

    // 1. Brokers
    thread::spawn(move||{
        loop{
            match br_usr_rx.try_recv(){
                Ok(request_list) => {
                    println!("Broker 1: had received order from User{:?}",request_list.id); // user list
                    // check price
                    let chosen_stock = iterate_stock_list(&stock_list, request_list.stock_name,request_list.bid_price);
                    match chosen_stock{
                        Some(chosen_stock)=>{
                            println!("User preferences stock: {}",request_list.id);
                            println!("Stock name: {}",chosen_stock.name);
                        }
                        None =>{
                            println!("Budget over?")
                        }    
                    }
                }
                Err(_)=>{
                    println!("Broker 1: Waiting for order..");
                    thread::sleep(Duration::from_secs(1));   
                }
            }
        }
    });

    // 2. Brokers
    // br_usr_rx2
    // thread::spawn(move||{
    //     loop{
    //         match br_usr_rx2.try_recv(){
    //             Ok(request_list) => {
    //                 println!("Broker 2: had received order from User{:?}",request_list.id); // user list
    //             }
    //             Err(_)=>{
    //                 println!("Broker 2: Waiting for order..");
    //                 thread::sleep(Duration::from_secs(1));   
    //             }
    //         }
    //     }
    // });
    
    // Users
    thread::spawn(move||{
        for i in 1..11{
            println!("User{}: Enter page..",i);
            println!("User{}: Page loading..",i);
            
            // make sure didn't miss out customer in the laoding page
            loop{
                match sl_rx.try_recv(){
                    Ok(stock_list)=>{
                        println!("User{}: Viewing the stock list",i); 
                        println!("User{}: Selecting stokcs...",i); 
                        thread::sleep(Duration::from_millis(5));  
                        let mut rng = rand::thread_rng();
                        for _ in 1..=rng.gen_range(1..=5){
                            let choose_broker = rng.gen_range(1..=2);
                            let br_usr_tx2 = br_usr_tx.clone();
                            println!("User{}: System choosing brokers..",i); 
                            thread::sleep(Duration::from_millis(5));  
                            // let reqest_list = user_request(i, stock_list.clone());
                            br_usr_tx.send(user_request(i, stock_list.clone())).unwrap();
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
    });
    
}
