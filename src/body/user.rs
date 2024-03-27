use std::{vec, thread, ops::Deref,time::Duration, sync::{Mutex, Arc, mpsc::channel}};
use crate::body::shared_channel;
use crate::body::stock;

struct Request{
    id:String,
    bid_price:i32,
    take_profit:i32,
    cut_loss:i32,
}

pub fn choose_stocks(){
    // let (_,sl_rx) = shared_channel::view_stock_channel::<Vec<stock::Stock>>();
    println!("**");
    // loop{
    //     match sl_rx.try_recv() {
    //         Ok(stock_final) =>{
    //                 println!("User previewing the stocks");       
    //                 println!("Stock Received: {:?}",stock_final);    
    //         } 
    //         Err(_) => {
    //             println!("Still loading");
    //             thread::sleep(Duration::from_secs(2));
    //         }
    //     }
    // }

    
}