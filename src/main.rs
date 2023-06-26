mod config; // other.rsをモジュールとしてインポート
mod rathole;
mod data;
mod check;


use std::io::{self, BufRead, Write};
use std::env;
use std::fs;
use std::fs::File;
use std::thread::sleep;
use std::time::Duration;
use std::path::PathBuf;
use std::process::{ Command, Child };
use config::Config;

fn main() {

    let temp_dir = env::temp_dir();
    let current_dir = env::current_dir().unwrap();
    let craftershub_tmp_dir = temp_dir.join("craftershub");
    let config_file_path = current_dir.join("設定.txt");

    data::show_logo();
    sleep(Duration::from_millis(800));
    data::attention();
    sleep(Duration::from_millis(1500));


    if !config_file_path.is_file(){
        let mut config_file = File::create(&config_file_path).expect("エラー : ファイルの作成に失敗しました。");
        config_file.write_all(data::default_config().as_bytes()).expect("エラー : ファイルの書き込みに失敗しました。");
        println!("初回起動のため設定ファイルを作成しました。\n設定.txt ファイルを見て、設定してみましょう。\n");
    }
    
    let config_result = Config::new(&config_file_path);
    match config_result {
        Ok(config) => {

            config.show_config();
            sleep(Duration::from_millis(500));

            let server_dir = current_dir.join(config.get_folder_name());
            if server_dir.is_dir() {
                data::instraction();
                sleep(Duration::from_millis(500));

                if let Some(file) = get_first_jar(&server_dir) {

                    

                    let mut child: Option<Child> = None;
                    if config.get_token().len() > 0 {
                        match rathole::run_rathole(&config.get_token(), config.get_port(), &craftershub_tmp_dir){
                            Ok(c) => child = Some(c),
                            Err(err) => println!("Port解放なし通信が機能できませんでした。\n{}", err),
                        }
                    }
                        
                    let command_args = [
                        "-jar",
                        &format!("-Xms{}G", config.get_memory()),
                        &format!("-Xmx{}G", config.get_memory()),
                        &server_dir.join(file.clone()).into_os_string().into_string().unwrap(),
                        "nogui",
                    ];

                    // println!("{:?}",command_args);
                    println!("サーバーを起動します。");

                    execute_command(&config.get_java_path(), &command_args, &server_dir);

                    if let Some(mut child) = child {
                        match rathole::stop_rathole(&mut child){
                            Ok(_) => println!("Port解放なし通信機能の終了"),
                            Err(err) => println!("Port解放なし通信機能が正常終了できませんでした。\n{}", err),
                        }
                    }

                } else {
                    println!("jar ファイルが見つかりませんでした。\n{} フォルダにサーバーファイルを入れてください。\n",config.get_folder_name());
                }
            } else {
                match fs::create_dir(config.get_folder_name()) {
                    Ok(_) => println!("{} フォルダを作成しました。サーバーのファイルはこのフォルダに入れてください。\n",config.get_folder_name()),
                    Err(_) => eprintln!("ディレクトリを作成できませんでした"),
                }
            }
        }
        Err(err) => {
            println!("エラー: {:?}", err);
        }
    }

    println!("Enterを入力すると終了します....\nもし終了しない場合は ✖ で閉じてください。(コマンドプロンプトだとなるかもです。)");

    let stdin = io::stdin();
    let _ = stdin.lock().lines().next();
}

fn execute_command(exe : &str, args: &[&str], dir : &PathBuf) {
    match Command::new(&exe).current_dir(dir).args(args).spawn(){
        Ok(mut child) => {
            match child.wait(){
                Ok(status) => {
                    if status.success() {
                        eprintln!("サーバーが正常に終了されました。");
                    } else {
                        eprintln!("コマンドが異常終了しました。");
                    }
                }
                Err(err) => {
                    eprintln!("エラー : {}", err);   
                }
            }
        }
        Err(err) => {
            eprintln!("エラー : {}", err);
        }
    }
}

fn get_first_jar(path: &PathBuf) -> Option<String> {
    // 拡張子が一致するファイル名を抽出
    if let Ok(entries) = fs::read_dir(path){
       for entry in entries {
            if let Ok(entry) = entry {
                let file_path = entry.path();
                if let Some(extension) = file_path.extension() {
                    if extension == "jar" {
                        if let Some(file_name) = file_path.file_name() {
                            if let Some(file_name_str) = file_name.to_str() {
                                return Some(file_name_str.to_owned());
                            }
                        }
                    }
                }
            }
        } 
    }
    None
}

