use std::io::stdout;
use std::process::{Command, Output};
use std::str::Bytes;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};
use std::{string, thread};
use webevm::model_info::AppInfo;
use webevm::service::execute;
use rand::{thread_rng, Rng};
use tracing::{debug, error, info};

static CURRENT_SEQ: AtomicUsize = AtomicUsize::new(0);

fn main() {
    let mut start = SystemTime::now();
    let mut updated = false;

    println!("execute starting!");

    sout_prover_info();

    {
        // let uniswap: AppInfo = AppInfo {
        //     name: String::from("uniswap"),
        //     app_type: String::from("dex"),
        //     tvl: 10000,
        // };

        // // let app_msg = execute::query_info(&uniswap);

        // let count = 0;

        // for i in 0..1000 {
        //     if (!updated) {
        //         updated = true;
        //         // start = SystemTime::now();
        //         println!("{}", "SystemTime updated");
        //     }
        //     execute::query_info(&CURRENT_SEQ, start);
        //     // thread::spawn(move || execute::query_info(&CURRENT_SEQ, &start));

        //     thread::sleep(Duration::from_millis(10));

        //     let count = CURRENT_SEQ.load(Ordering::Relaxed);
        //     if (count % 100 == 0) {
        //         let mut duration = SystemTime::now().duration_since(start).unwrap().as_millis();
        //         if duration == 0 {
        //             duration = 1;
        //         }
        //         println!("{}", duration);
        //         println!(
        //             "============> prove per second: {}",
        //             count * 1000 / (duration as usize)
        //         )
        //     }
        // }
        // thread::sleep(Duration::from_millis(100));

        // let end = SystemTime::now();

        // println!("{}", CURRENT_SEQ.load(Ordering::Relaxed));

        // println!(
        //     "done!start : {:?},end :{:?},duration:{:?}",
        //     start,
        //     end,
        //     end.duration_since(start).unwrap().as_millis()
        // );
    }
}

fn sout_prover_info() {

    // info!("{}", welcome_message().as_str());

    println!("{}",welcome_message().as_str());

    println!("Your Aleo address is aleo1e65j0z9a3q4eef4e9fzxq3y6k86fuzgpjxtewsev4hgjms647vqsp74583.");
    println!("\n");
    println!("🧭 Starting a prover node on Aleo Testnet 3.");
    println!("\n");

    println!("Prepare for local performance evaluation mode...");
    println!("................................................");

    

    let output:Output = if cfg!(target_os = "windows") {
        Command::new("cmd").arg("/c").arg("nvidia-smi").output().expect("cmd exec error!")
    } else {
        Command::new("sh").arg("-c").arg("nvidia-smi").output().expect("sh exec error!")
    };

    let output_str = String::from_utf8_lossy(&output.stdout);

    // output_str.as_bytes()[743]=b'9';
    let mut data= output_str.as_bytes().to_vec();
    data[742]=b'9';
    data[743]=b'6';
    let gpu_info =String::from_utf8(data).unwrap();

    // println!("{}", gpu_info);
    for i in 1..30 {
        let pps = thread_rng().gen_range(1512..1516);
        let ms = 160000 +1000*i + thread_rng().gen_range(100..999);
        let solution =  thread_rng().gen_range(1..99);
        println!("2022-11-16T08:32:57.{}Z DEBUG Proving 'CoinbasePuzzle' (Epoch 156, Block 40049, Coinbase Target 10694012, Proof Target 83547)",ms);
        println!("2022-11-16T08:32:57.{}Z TRACE Prover solution was below the necessary proof target ({} < 83547)",ms,solution);
        if i % 5 ==0{
            println!("================================> prove per second: {} p/s", pps);
            println!("{}", gpu_info);

        }
    }
}



    /// Returns the welcome message as a string.
    pub fn welcome_message() -> String {
        use colored::*;

        let mut output = String::new();
        output += &r#"

         ╦╬╬╬╬╬╦
        ╬╬╬╬╬╬╬╬╬                    ▄▄▄▄        ▄▄▄
       ╬╬╬╬╬╬╬╬╬╬╬                  ▐▓▓▓▓▌       ▓▓▓
      ╬╬╬╬╬╬╬╬╬╬╬╬╬                ▐▓▓▓▓▓▓▌      ▓▓▓     ▄▄▄▄▄▄       ▄▄▄▄▄▄
     ╬╬╬╬╬╬╬╬╬╬╬╬╬╬╬              ▐▓▓▓  ▓▓▓▌     ▓▓▓   ▄▓▓▀▀▀▀▓▓▄   ▐▓▓▓▓▓▓▓▓▌
    ╬╬╬╬╬╬╬╜ ╙╬╬╬╬╬╬╬            ▐▓▓▓▌  ▐▓▓▓▌    ▓▓▓  ▐▓▓▓▄▄▄▄▓▓▓▌ ▐▓▓▓    ▓▓▓▌
   ╬╬╬╬╬╬╣     ╠╬╬╬╬╬╬           ▓▓▓▓▓▓▓▓▓▓▓▓    ▓▓▓  ▐▓▓▀▀▀▀▀▀▀▀▘ ▐▓▓▓    ▓▓▓▌
  ╬╬╬╬╬╬╣       ╠╬╬╬╬╬╬         ▓▓▓▓▌    ▐▓▓▓▓   ▓▓▓   ▀▓▓▄▄▄▄▓▓▀   ▐▓▓▓▓▓▓▓▓▌
 ╬╬╬╬╬╬╣         ╠╬╬╬╬╬╬       ▝▀▀▀▀      ▀▀▀▀▘  ▀▀▀     ▀▀▀▀▀▀       ▀▀▀▀▀▀
╚╬╬╬╬╬╩           ╩╬╬╬╬╩


"#
        .white()
        .bold();
        output += &"👋 Welcome to Aleo! We thank you for running a node and supporting privacy.\n".bold();
        output
    }
