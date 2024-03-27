use std::{ops::Deref, str::Bytes, sync::{mpsc::channel, Arc, Mutex}, thread::{self, spawn}, time::Duration, vec};
use lazy_static::lazy_static;
#[derive(Clone)] 

pub struct Stock{
    name:String,
    value:f64,
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


fn main(){

}