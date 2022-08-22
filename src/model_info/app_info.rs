use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AppInfo{
    pub name:String,
    pub app_type:String,
    pub tvl:u32,
}