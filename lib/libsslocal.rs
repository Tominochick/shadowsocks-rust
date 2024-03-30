#![cfg(all(feature = "local-http"))]


use shadowsocks_service::{
    config::{Config, ConfigType}, run_local
};
use tokio::{self, runtime::Runtime};
#[no_mangle]  
pub extern fn startLocalHTTPPoxyService(
    local_address_str:*const u8,
    local_address_str_len:usize,
    local_port:i32,
    server_address_str:*const u8,
    server_address_str_len:usize,
    server_port:i32,
    passwd_str:*const u8,
    passwd_str_len:usize){  

    let local_address = unsafe {
        std::str::from_utf8_unchecked(
            std::slice::from_raw_parts(local_address_str, local_address_str_len)
        )
    };
    let server_address = unsafe {
        std::str::from_utf8_unchecked(
            std::slice::from_raw_parts(server_address_str, server_address_str_len)
        )
    };
    let passwd = unsafe {
        std::str::from_utf8_unchecked(
            std::slice::from_raw_parts(passwd_str, passwd_str_len)
        )
    };
    let config = format!(r#"{{
        "locals": [
            {{
                "local_port": {},
                "local_address": "{}",
                "protocol": "http"
            }}
        ],
        "server": "{}",
        "server_port": {},
        "password": "{}",
        "method": "aes-128-gcm"
    }}"#,local_port,local_address,server_address,server_port,passwd);

    let local_config = Config::load_from_str(config.as_str()
        , ConfigType::Local).unwrap();
    let run = Runtime::new().unwrap();
    run.block_on(run_local(local_config)).unwrap();
    print!("exit!");
}


pub fn http_proxy() {
    let local_config = Config::load_from_str(
        r#"{
            "locals": [
                {
                    "local_port": 19689,
                    "local_address": "127.0.0.1",
                    "protocol": "http"
                }
            ],
            "server": "cmthkd-d.net-lag.xyz",
            "server_port": 564,
            "password": "76691bbe-7387-4a44-b088-0a4c83627c74",
            "method": "aes-128-gcm"
        }"#,
        ConfigType::Local,
    )
    .unwrap();
    tokio::runtime::Runtime::new()
    .unwrap()
    .block_on(run_local(local_config))
    .unwrap();
    print!("exit!");
}