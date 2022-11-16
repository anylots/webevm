use crate::model_info::app_info::AppInfo;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
use std::time::Duration;


pub fn query_info(app: AppInfo, apps :AppInfo) -> String {
    let a = serde_json::to_string(&apps);
    return serde_json::to_string(&app).unwrap();
}

pub fn parallel_count(count: &AtomicUsize) {
    count.fetch_add(1, Ordering::Relaxed);
    thread::sleep(Duration::from_millis(10));
}
