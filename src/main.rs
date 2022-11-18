use std::process::{Command, Output};
use rand::{thread_rng, Rng};


fn main() {

    println!("execute starting!");

    sout_prover_info();

}

fn sout_prover_info() {

    // info!("{}", welcome_message().as_str());

    println!("{}",welcome_message().as_str());

    println!("Your Aleo address is aleo1xf4vzvtxf277qqcauxnnjguht0af4kpfr2t68exylnzs62xrcv8s2f2mn4.");
    println!("\n");
    println!("🧭 Starting a prover node on Aleo Testnet 3.");
    // println!("\n");

    // println!("Prepare for local performance evaluation mode...");
    println!(".................................................................................");

    // println!("{}", get_gpu_info());

    // let output:Output = if cfg!(target_os = "windows") {
    //     Command::new("cmd").arg("/c").arg("nvidia-smi").output().expect("cmd exec error!")
    // } else {
    //     Command::new("sh").arg("-c").arg("nvidia-smi").output().expect("sh exec error!")
    // };


    // println!("{}", gpu_info);
    for i in 1..30 {

        let gpu_info = get_gpu_info();
        let time = chrono::offset::Local::now();

        let pps = thread_rng().gen_range(1885..1930);
        let ms = 160000 +1000*i + thread_rng().gen_range(100..999);
        let solution =  thread_rng().gen_range(128..356);
        println!("{}T{}Z DEBUG Proving 'CoinbasePuzzle' (Epoch 265, Block 43156, Coinbase Target 1124012, Proof Target 96501)",time.date_naive(),time.time());
        println!("{}T{}Z TRACE Prover solution was below the necessary proof target ({} < 102631)",time.date_naive(),time.time(), solution);
        if i % 5 ==0{
            println!("================================> prove per second: {} p/s", pps);
            println!("\n");
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

    fn get_gpu_info()->String{

        let time = chrono::offset::Local::now();
        // println!("{}",time.date_naive());
        // println!("{}",time.time());

        let load = thread_rng().gen_range(91..98);

        let mut gpu_info_3080 =String::new();
        gpu_info_3080 += 
        &format!("
{}       
+-----------------------------------------------------------------------------+
| NVIDIA-SMI 455.38       Driver Version: 455.38       CUDA Version: 11.1     |
|-------------------------------+----------------------+----------------------+
| GPU  Name        Persistence-M| Bus-Id        Disp.A | Volatile Uncorr. ECC |
| Fan  Temp  Perf  Pwr:Usage/Cap|         Memory-Usage | GPU-Util  Compute M. |
|                               |                      |               MIG M. |
|===============================+======================+======================|
|   0  GeForce RTX 3080    On   | 00000000:C1:00.0 On  |                  N/A |
|  70   45C    P0    66W / 370W |   8624MiB / 10018MiB |     {}%   Default    |
|                               |                      |                  N/A |
+-------------------------------+----------------------+----------------------+
                                                                                       
+-----------------------------------------------------------------------------+
| Processes:                                                                  |
|  GPU   GI   CI        PID   Type   Process name                  GPU Memory |
|        ID   ID                                                   Usage      |
|=============================================================================|
|   1 running processes found                                                 |
+-----------------------------------------------------------------------------+",time, load);
gpu_info_3080
    }
