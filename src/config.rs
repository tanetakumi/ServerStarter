use std::fs::File;
use std::io;
use std::io::{Read, Error};
use std::path::PathBuf;

pub struct Config {
    token: String,
    port: u16,
    server: String,
    java_path: String,
    memory: u16,
    folder_name: String,
}

impl Config {
    pub fn new(path: &PathBuf) -> Result<Config, Error> {

        let mut config = Config {
            token: String::new(),
            port: 25565,
            server: String::from("paper"),
            java_path: String::from("java"),
            memory: 4,
            folder_name: String::from("myserver"),
        };

        config.load(&path)?;
        Ok(config)
    }

    fn load(&mut self, file_path: &PathBuf) -> io::Result<()> {
        let mut file = File::open(file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        for line in contents.lines() {
            if let Some(value) = line.split_once('=') {
                let (key, val) = (value.0.trim(), value.1.trim().replace("\"",""));
                match key {
                    "token" => self.token = val.to_string(),
                    "port" => {
                        if let Ok(p) = val.parse::<u16>() {
                            self.port = p;
                        }
                    },
                    "server" => self.server = val.to_string(),
                    "java_path" => self.java_path = val.to_string(),
                    "memory" => {
                        if let Ok(p) = val.parse::<u16>() {
                            self.memory = p;
                        }
                    },
                    "folder_name" => self.folder_name = val.to_string(),
                    _ => (),
                }
            }
        }
        Ok(())
    }

    pub fn get_token(&self) -> &str {
        &self.token
    }

    pub fn get_port(&self) -> u16 {
        self.port
    }

    pub fn get_server(&self) -> &str {
        &self.server
    }

    pub fn get_java_path(&self) -> &str {
        &self.java_path
    }

    pub fn get_memory(&self) -> u16 {
        self.memory
    }

    pub fn get_folder_name(&self) -> &str {
        &self.folder_name
    }

    pub fn show_config(&self){
        println!("========== 設定項目 ==========");
        println!("トークン\t: {}","*".repeat(self.get_token().len()));
        println!("ポート番号\t: {}",self.get_port());
        println!("サーバー\t: {}",self.get_server());
        println!("Javaのパス\t: {}",self.get_java_path());
        println!("割り当てメモリ\t: {}GB",self.get_memory());
        println!("フォルダ名\t: {}",self.get_folder_name());
        println!("==============================");
        println!("\n");
    }
}