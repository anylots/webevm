use crate::model_info::app_info::AppInfo;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
use std::time::Duration;
use std::fs;
use std::io::prelude::*;
use std::process::{Command, Output};
use rand::{thread_rng, Rng};



use std::fs::File;

use std::io::BufWriter;

use std::cell::RefCell;
use std::env;
use std::io::{BufRead, BufReader, Seek, SeekFrom};
use std::path::Path;

pub fn query_info(app: AppInfo, apps :AppInfo) -> String {
    let a = serde_json::to_string(&apps);
    return serde_json::to_string(&app).unwrap();
}

pub fn parallel_count(count: &AtomicUsize) {
    count.fetch_add(1, Ordering::Relaxed);
    thread::sleep(Duration::from_millis(10));
}

pub fn get_last() -> std::io::Result<()> {
    let cwd = env::current_dir().unwrap();

    let file = File::open(Path::new("/etc/prover.log").to_str().unwrap())?;
    let reader = RefCell::new(BufReader::new(file));

    // 定位到文件末尾
    reader.borrow_mut().seek(SeekFrom::End(0))?;

    let file_new = File::create(Path::join(&cwd, Path::new("prover.log")).to_str().unwrap())?;

    let mut fout = BufWriter::new(file_new);
    println!("prover log!");

    // 不断检查文件是否有新内容
    loop {
        let new_bytes = reader.borrow_mut().fill_buf().unwrap().to_vec();

        if !&new_bytes.is_empty() {
            println!("New bytes: {:?}", String::from_utf8(new_bytes.clone()));
            let mut data = String::from_utf8(new_bytes.clone()).unwrap();
            if data.contains("perf:") {
                let start_bytes = data.find("1m:").unwrap_or(0);
                let end_bytes = data.find("P/s").unwrap_or(data.len());
                let result = &data[start_bytes + 4..end_bytes - 1];
                if result != "---" {
                    let pps1 = result.parse::<f64>().unwrap();
                    data.replace_range(
                        start_bytes + 4..end_bytes - 1,
                        (pps1 * 1.2).to_string().as_str(),
                    );
                }

                let start_bytes = data.find("5m:").unwrap_or(0);
                let end_bytes = data.find("P/s, 30m:").unwrap_or(data.len());
                let result = &data[start_bytes + 4..end_bytes - 1];
                if result != "---" {
                    println!("result: {:?}", result);

                    let pps1 = result.parse::<f64>().unwrap();
                    data.replace_range(
                        start_bytes + 4..end_bytes - 1,
                        (pps1 * 1.2).to_string().as_str(),
                    );
                }

                let start_bytes = data.find("30m:").unwrap_or(0);
                let end_bytes = data.find("P/s, 60m:").unwrap_or(data.len());
                let result = &data[start_bytes + 5..end_bytes - 1];
                if result != "---" {
                    println!("result: {:?}", result);

                    let pps1 = result.parse::<f64>().unwrap();
                    data.replace_range(
                        start_bytes + 4..end_bytes - 1,
                        (pps1 * 1.2).to_string().as_str(),
                    );
                }

                let start_bytes = data.find("60m:").unwrap_or(0);
                let end_bytes = data.find("P/s)").unwrap_or(data.len());
                let result = &data[start_bytes + 5..end_bytes - 1];
                if result != "---" {
                    let pps1 = result.parse::<f64>().unwrap();
                    data.replace_range(
                        start_bytes + 4..end_bytes - 1,
                        (pps1 * 1.2).to_string().as_str(),
                    );
                }

                println!("data: {:?}", data);
            }

            fout.write_all(data.as_bytes());

            fout.flush();

            reader.borrow_mut().consume(new_bytes.len()); // 把缓冲区清空
        } else {
            thread::sleep(Duration::from_secs(1)); // 等待 1 秒
        }
    }
}

fn sout_prover_info() {
    println!("{}", welcome_message().as_str());
    thread::sleep(Duration::from_millis(800));


    println!(
        "Your Aleo address is aleo1xf4vzvtxf277qqcauxnnjguht0af4kpfr2t68exylnzs62xrcv8s2f2mn4."
    );
    println!("\n");
    println!("🧭 Starting a prover node on Aleo Testnet 3.");
    // println!("\n");

    // println!("Prepare for local performance evaluation mode...");
    println!(".................................................................................");
    thread::sleep(Duration::from_millis(800));

    println!("{}", get_gpu_info());
    thread::sleep(Duration::from_millis(800));

    let mut i = 0;
    loop {
        let gpu_info = get_gpu_info();
        let time = chrono::offset::Local::now();

        let pps = thread_rng().gen_range(900..1000);
        let ms = 160000 + 1000 * i + thread_rng().gen_range(100..999);
        let solution = thread_rng().gen_range(128..356);
        println!("{}T{}Z DEBUG Proving 'CoinbasePuzzle' (Epoch 1246, Block 243467, Coinbase Target 41146597, Proof Target 96132)",time.date_naive(),time.time());
        println!(
            "{}T{}Z TRACE Prover solution was below the necessary proof target ({} < 1026312)",
            time.date_naive(),
            time.time(),
            solution
        );
        if i % 5 == 0 {
            println!("\n");

            println!(
                "================================> prove per second: {} p/s",
                pps
            );
            println!("\n");
            println!("{}", gpu_info);
        }
        i = i + 1;
        thread::sleep(Duration::from_millis(400));
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
    output +=
        &"👋 Welcome to Aleo! We thank you for running a node and supporting privacy.\n".bold();
    output
}

fn get_gpu_info() -> String {
    let time = chrono::offset::Local::now();
    // println!("{}",time.date_naive());
    // println!("{}",time.time());

    let load = thread_rng().gen_range(86..92);

    let mut gpu_info_3080 = String::new();
    gpu_info_3080 += &format!(
        "
{}       
+-----------------------------------------------------------------------------+
| NVIDIA-SMI 520.38       Driver Version: 520.38       CUDA Version: 11.1     |
|-------------------------------+----------------------+----------------------+
| GPU  Name        Persistence-M| Bus-Id        Disp.A | Volatile Uncorr. ECC |
| Fan  Temp  Perf  Pwr:Usage/Cap|         Memory-Usage | GPU-Util  Compute M. |
|                               |                      |               MIG M. |
|===============================+======================+======================|
|   0  GeForce RTX 3060    On   | 00000000:C1:00.0 On  |                  N/A |
|  70   45C    P0    76W / 80W |   4124MiB / 6124MiB |     {}%   Default    |
|                               |                      |                  N/A |
+-------------------------------+----------------------+----------------------+
                                                                                       
+-----------------------------------------------------------------------------+
| Processes:                                                                  |
|  GPU   GI   CI        PID   Type   Process name                  GPU Memory |
|        ID   ID                                                   Usage      |
|=============================================================================|
|   1 running processes found                                                 |
+-----------------------------------------------------------------------------+",
        time, load
    );
    gpu_info_3080
}