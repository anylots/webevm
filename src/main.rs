use std::io::stdout;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};
use std::{string, thread};
use webevm::model_info::AppInfo;
use webevm::service::execute;

static CURRENT_SEQ: AtomicUsize = AtomicUsize::new(0);

fn main() {
    let mut start = SystemTime::now();
    let mut updated = false;

    println!("execute starting!");
    let uniswap: AppInfo = AppInfo {
        name: String::from("uniswap"),
        app_type: String::from("dex"),
        tvl: 10000,
    };

    // let app_msg = execute::query_info(&uniswap);

    let count = 0;

    for i in 0..1000 {
        if (!updated) {
            updated = true;
            start = SystemTime::now();
            println!("{}", "SystemTime updated");
        }

        thread::spawn(|| execute::query_info(&CURRENT_SEQ));

        thread::sleep(Duration::from_millis(10));

        let count = CURRENT_SEQ.load(Ordering::Relaxed);
        if (count % 100 == 0) {
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
    thread::sleep(Duration::from_millis(100));

    let end = SystemTime::now();

    println!("{}", CURRENT_SEQ.load(Ordering::Relaxed));

    println!(
        "done!start : {:?},end :{:?},duration:{:?}",
        start,
        end,
        end.duration_since(start).unwrap().as_millis()
    );
}
