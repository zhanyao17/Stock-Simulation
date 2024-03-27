use std::sync::mpsc::{channel, Sender, Receiver};

pub fn view_stock_channel<T>() ->(Sender<T>,Receiver<T>){
    channel()
}