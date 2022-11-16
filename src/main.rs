use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};
use webevm::model_info::AppInfo;
use webevm::service::execute;

static CURRENT_SEQ: AtomicUsize = AtomicUsize::new(0);

fn main() {

    println!("execute starting!");
    let uniswap: AppInfo = AppInfo {
        name: String::from("uniswap"),
        app_type: String::from("dex"),
        tvl: 10000,
    };


    let uniswap_s: AppInfo = AppInfo {
        name: String::from("uniswap"),
        app_type: String::from("dex"),
        tvl: 10000,
    };

    let app_msg = execute::query_info(uniswap, uniswap_s);
    println!("{}", app_msg);

    execute_parallel();
}


fn execute_parallel() {
    let start = SystemTime::now();

    for _ in 0..1000 {
        thread::spawn(|| execute::parallel_count(&CURRENT_SEQ));
        let count = CURRENT_SEQ.load(Ordering::Relaxed);
        if count % 10 == 0 {
            let mut duration = SystemTime::now().duration_since(start).unwrap().as_millis();
            if duration == 0 {
                duration = 1;
            }
            println!("{}", duration);
            println!(
                "============> prove per second: {}",
                count * 1000 / (duration as usize)
            )
        }
    }

    let end = SystemTime::now();
    println!("{}", CURRENT_SEQ.load(Ordering::Relaxed));
    println!(
        "done!start : {:?},end :{:?},duration:{:?}",
        start,
        end,
        end.duration_since(start).unwrap().as_millis()
    );
}
