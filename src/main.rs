use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Hash)]
pub struct IAAAUserInfo {
    // example 'Tom'
    #[serde(rename = "name")]
    name: String,

    // example: 'Kaitong'
    #[serde(rename = "status")]
    status: String,

    // example: '2200088888'
    #[serde(rename = "identityId")]
    identity_id: String,

    // example: '00048'
    #[serde(rename = "deptId")]
    dept_id: String,

    // example: '信息科学技术学院'
    #[serde(rename = "dept")]
    dept: String,

    // example: '学生'
    #[serde(rename = "identityType")]
    identity_type: String,

    // example: '本专科学生'
    #[serde(rename = "detailType")]
    detail_type: String,

    // example: '在校'
    #[serde(rename = "identityStatus")]
    identity_status: String,

    // example: '燕园'
    #[serde(rename = "campus")]
    campus: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Hash)]
pub struct IAAAValidateResponse {
    #[serde(rename = "success")]
    success: bool,
    #[serde(rename = "errCode")]
    err_code: String,
    #[serde(rename = "errMsg")]
    err_msg: String,
    #[serde(rename = "userInfo")]
    user_info: IAAAUserInfo,
}

fn md5_hash(msg: &String) -> String {
    let digest = md5::compute(msg);
    format!("{:x}", digest)
}

const VALIDATE_ENDPOINT: &'static str = "https://iaaa.pku.edu.cn/iaaa/svc/token/validate.do";

pub async fn validate(
    remote_addr: String,
    app_id: String,
    app_key: String,
    token: String,
) -> Result<IAAAValidateResponse> {
    let payload = format!("appId={app_id}&remoteAddr={remote_addr}&token={token}");
    let sign = md5_hash(&(payload.clone() + &app_key));
    let url = format!("{VALIDATE_ENDPOINT}?{payload}&msgAbs={sign}");
    let data = reqwest::get(url)
        .await
        .map_err(|e| Error::Get(e.to_string()))?
        .json::<IAAAValidateResponse>()
        .await
        .map_err(|e| Error::Deserialize(e.to_string()))?;
    return Ok(data);
}

pub type Result<T> = std::result::Result<T, Error>;

pub enum Error {
    Get(String),
    Deserialize(String),
}

fn main() {
    println!("Hello, world!");
}
