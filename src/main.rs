use std::{vec, thread, ops::Deref,time::Duration, sync::{Mutex, Arc, mpsc::channel}};
mod body;
use body::stock;
use body::user;

fn main() {
    // user::choose_stocks();
    stock::stock_sim();

    loop{};
}

