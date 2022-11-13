use crate::model_info::app_info::AppInfo;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::{thread};
use std::time::Duration;

pub fn query_info(count: &AtomicUsize) {
    // return serde_json::to_string(&app).unwrap();
    count.fetch_add(1, Ordering::Relaxed);
    thread::sleep(Duration::from_millis(10));

    // println!("{}", "dosomthing");
}
