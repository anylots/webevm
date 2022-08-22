use webevm::model_info::AppInfo;
use webevm::service::execute;

fn main() {
    println!("execute starting!");
    let  uniswap: AppInfo = AppInfo {
        name: String::from("uniswap"),
        app_type: String::from("dex"),
        tvl: 10000,
    };
    let app_msg = execute::query_info(&uniswap);
    println!("{}", app_msg);
}
