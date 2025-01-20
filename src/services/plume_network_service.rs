use std::{
    fs,
    rc::Rc,
    str::FromStr,
    sync::Arc
};
use alloy::signers::local::PrivateKeySigner;
use ethers::{
    signers::LocalWallet,
    utils::hash_message
};
use log::info;
use reqwest::{Client, Proxy};
use crate::utils::{
    read_file_utils,
    constants::constants::{
        global_constants,
        plume_network_constants,
    },
};
use crate::domain::entity::{
    config::Config,
    payload::Payload,
    response_body::ResponseBody,
};
 pub async fn test()-> Result<(), Box<dyn std::error::Error>>{
     let private_keys = read_file_utils::read_file_line_by_line(global_constants::PRIVATE_KEYS_FILE_CONTENT).await?;
     let config = read_file_utils::deserialize_to_object::<Config>(global_constants::CONFIG_TOML_FILE_PATH).await?;
     //从config/private_keys.txt下逐行读取字符串，使用vec存储
     for cur_private_key in &private_keys {
         // 初始化钱包
         let wallet: LocalWallet = cur_private_key.parse()?;
         let signer =
             Arc::new(PrivateKeySigner::from_str(cur_private_key).expect("Private key to be valid!"));
         let address = signer.address().to_string();
         //检查该用户是否注册
         let res_registered = self::check_is_registered(&address).await;
         if res_registered.expect("Check registration status") {
             //已注册，跳过本次循环✘
             info!("wallet_address: {:?}已注册，跳过本次注册！✔", &address);
             continue;
         }
         // 对消息进行哈希处理（符合以太坊标准）
         let message_hash = hash_message(global_constants::META_MASK_SIGN_MESSAGE);
         // 使用私钥对消息进行签名
         let signature_str = format!("0x{}", wallet.sign_hash(message_hash)?);
         // 准备请求的 JSON 数据
         let payload = Payload {
             message: global_constants::META_MASK_SIGN_MESSAGE.to_string(),
             signature: signature_str.to_string(), // 将签名格式化为字符串
             address: address.clone(),
             twitter_encrypted_username: None,
             twitter_encrypted_id: None,
             discord_encrypted_username: None,
             discord_encrypted_id: None,
         };
         let proxy = Proxy::http(config.proxy_ip.clone())?;
         //推特认证
         let wallet_authorization_client = Rc::new(Client::builder()
             .proxy(proxy.clone())
             .build()?) ;

         // 向目标 URL 发送 POST 请求
         let response = wallet_authorization_client.clone()
             .post(plume_network_constants::REGISTRATION_URL)
             .json(&payload) // 设置请求体为 JSON 数据
             .send()
             .await?;
         if self::check_is_registered(&address).await.expect("Check registration status") {
             info!("wallet_address: {:?} 完成注册！", &address);
         }else {
             info!("wallet_address: {:?} 注册失败，失败原因：{:?}！", &address,response);
         }
     }
     Ok(())
}


async fn _bind_to_twitter(address: &str) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

async fn _bind_to_discard(address: &str) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

// //读取配置文件
// async fn read_config() -> Result<Config, Box<dyn std::error::Error>> {
//     // 获取当前工作目录
//     let current_dir = env::current_dir()?;
//     // println!("Current directory: {:?}", current_dir);
//     // 配置文件路径
//     let config_path = current_dir.join("../../config_file/config.toml");
//     // 读取配置文件内容
//     let config_content = fs::read_to_string(config_path)?;
//     // 解析 TOML 格式的配置文件
//     let config: Config = toml::de::from_str(&config_content)?;
//     Ok(config)
// }
// 
// async fn read_private_keys() -> Result<Vec<String>, Box<dyn std::error::Error>> {
//     let current_dir = env::current_dir()?;
//     // 配置文件路径
//     let config_path = current_dir.join("../../config_file/private_keys.txt");
//     let private_keys_str = fs::read_to_string(config_path)?;
//     let lines: Vec<String> = private_keys_str
//         .lines()
//         .filter(|line| !line.trim().is_empty())
//         .map(String::from)
//         .collect();
//     // println!("Private keys: {:?}", lines);
//     Ok(lines)
// }
async fn check_is_registered(wallet_address:&str) -> Result<bool, Box<dyn std::error::Error>> {
    let check_client = Client::builder().build()?;
    let response = check_client
        .get( plume_network_constants::CHECK_REGISTRATION_URL)
        .query(&[("address", wallet_address)])
        .send()
        .await?;
    if response.status().is_success() {
        let result = response.json::<ResponseBody>().await.expect("Response to be valid JSON");
        return Ok(result.registered);
    }
    Ok(false)
}
