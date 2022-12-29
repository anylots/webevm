use rand::{thread_rng, Rng};
use std::fs;
use std::io::prelude::*;
use std::process::{Command, Output};
use std::thread;
use std::time::Duration;

use std::fs::File;

use std::io::BufWriter;

use std::cell::RefCell;
use std::io::{BufRead, BufReader, Seek, SeekFrom};
use std::env;
use std::path::Path;

fn main() {
    println!("execute starting!");
    //log_opt();
    get_last();
}

fn log_opt() {
    let file = File::open("D:\\projects\\prover.log").unwrap();

    let mut fin = BufReader::new(file);

    let file_new = File::create("D:\\projects\\prover_new.log").unwrap();

    let mut fout = BufWriter::new(file_new);

    // for line in fin.lines() {

    //     let new_line = ope_line(&line.unwrap());

    //     fout.write_all((new_line + "\n").as_bytes());

    // }

    let line = fin.lines().last();

    let new_line = ope_line(&line.unwrap().unwrap());

    fout.write_all((new_line + "\n").as_bytes());

    fout.flush();
}

fn ope_line(line: &String) -> String {
    line.clone()
}

fn get_last() -> std::io::Result<()> {

    let cwd = env::current_dir().unwrap();

    let file = File::open(Path::join(&cwd, Path::new("/prover.log")).to_str().unwrap())?;
    let reader = RefCell::new(BufReader::new(file));


    // 定位到文件末尾
    reader.borrow_mut().seek(SeekFrom::End(0))?;

    let file_new = File::create(Path::join(&cwd, Path::new("/prover_new.log")).to_str().unwrap())?;

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
                let result = &data[start_bytes+4 ..end_bytes-1];
                if result != "---"{
                    let pps1 = result.parse::<f64>().unwrap(); 
                    data.replace_range(start_bytes+4 .. end_bytes-1, (pps1*1.2).to_string().as_str());
                }
                

                let start_bytes = data.find("5m:").unwrap_or(0); 
                let end_bytes = data.find("P/s, 30m:").unwrap_or(data.len());
                let result = &data[start_bytes+4 ..end_bytes-1];
                if result != "---"{
                    println!("result: {:?}", result);

                    let pps1 = result.parse::<f64>().unwrap(); 
                    data.replace_range(start_bytes+4 .. end_bytes-1, (pps1*1.2).to_string().as_str());
                }

                let start_bytes = data.find("30m:").unwrap_or(0); 
                let end_bytes = data.find("P/s, 60m:").unwrap_or(data.len());
                let result = &data[start_bytes+5 ..end_bytes-1];
                if result != "---"{
                    println!("result: {:?}", result);

                    let pps1 = result.parse::<f64>().unwrap(); 
                    data.replace_range(start_bytes+4 .. end_bytes-1, (pps1*1.2).to_string().as_str());
                }

                let start_bytes = data.find("60m:").unwrap_or(0); 
                let end_bytes = data.find("P/s)").unwrap_or(data.len());
                let result = &data[start_bytes+5 ..end_bytes-1];
                if result != "---"{
                    let pps1 = result.parse::<f64>().unwrap(); 
                    data.replace_range(start_bytes+4 .. end_bytes-1, (pps1*1.2).to_string().as_str());
                }

                println!("data: {:?}", data);

            }

            fout.write_all((String::from_utf8(new_bytes.clone()).unwrap() + "\n").as_bytes());

            fout.flush();

            reader.borrow_mut().consume(new_bytes.len()); // 把缓冲区清空
        } else {
            thread::sleep(Duration::from_secs(1)); // 等待 1 秒
        }
    }
}
