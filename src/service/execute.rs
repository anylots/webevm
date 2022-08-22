use crate::model_info::app_info::AppInfo;

pub fn query_info(app: &AppInfo) -> String {
    return serde_json::to_string(&app).unwrap();
}
