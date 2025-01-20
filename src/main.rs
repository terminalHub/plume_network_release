mod  domain;
mod services;
mod handler;
mod utils;
mod config;

use log::LevelFilter;

#[tokio::main]
async  fn main() {
  //日志系统初始化
  env_logger::builder()
      .filter_level(LevelFilter::Info)
      .init();
}