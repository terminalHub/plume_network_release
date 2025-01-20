use std::{fs, io, str::FromStr, path::{
    Path,
    PathBuf}
};
use std::error::Error;
use serde::de::DeserializeOwned;

///# feature
///     根据传入文件路径按行读取信息 
/// 
///# param
///     文件路径
/// 
///# return
///     - 每行的信息封装成的Vec<String>
///     - 路径文件不存在将会抛出异常
pub async fn read_file_line_by_line(file_path : &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let path = Path::new(file_path);
    // 检查文件是否存在
    if !path.exists() {
        return Err(Box::new(io::Error::new(io::ErrorKind::NotFound, format!("该{}路径下的文件不存在，请检查路径下所需文件的完整性！: ", file_path))));
    }
    //配置文件路径
    let path_buf = PathBuf::from_str(file_path)?;
    let private_keys_str = fs::read_to_string(path_buf)?;
    let data_vec: Vec<String> = private_keys_str
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(String::from)
        .collect();
    Ok(data_vec)
}

/// 从文件路径中读取内容，并将其反序列化为指定类型的对象
///
/// # 参数
/// - `file_path`: 配置文件的路径
/// - 泛型参数 `T`: 目标序列化对象的类型
///
/// # 返回值
/// - `Result<T, Box<dyn Error>>`: 成功时返回反序列化后的对象，失败时返回错误
pub async fn deserialize_to_object<T>(file_path: &str) -> Result<T, Box<dyn Error>>
where
    T: DeserializeOwned,
{
    // 检查文件是否存在
    if !Path::new(file_path).exists() {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("配置文件不存在，请检查路径：{}", file_path),
        )));
    }
    // 读取文件内容
    let config_content = fs::read_to_string(file_path)?;
    // 将内容反序列化为目标类型
    let config: T = toml::de::from_str(&config_content)?;
    Ok(config)
}