
pub mod constants {
    pub mod global_constants{
        // 定义常量
        pub const CONFIG_TOML_FILE_PATH: &str ="config_file/config_file.toml";
        pub const PRIVATE_KEYS_FILE_CONTENT: &str ="config_file/private_keys.toml";
        pub const META_MASK_SIGN_MESSAGE: &str ="By signing this message, I confirm that I have read and agreed to Plume's Airdrop Terms of Service. This does not cost any gas to sign.";
    }
    pub  mod  plume_network_constants{
        pub const REGISTRATION_URL: &str ="https://registration.plumenetwork.xyz/api/sign-write";
        pub const CHECK_REGISTRATION_URL: &str ="https://registration.plumenetwork.xyz/api/sign-read";
    }
}