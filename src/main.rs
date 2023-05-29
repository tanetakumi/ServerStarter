mod config; // other.rsをモジュールとしてインポート

use std::io::{self, BufRead};
use std::process::Command;
use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    // 一時フォルダのパスを取得します
    let temp_dir = env::temp_dir();

    // 新しいフォルダを作成します（既に存在する場合は無視されます）
    let new_folder_path = temp_dir.join("craftershub");

    // 新しいフォルダを作成します
    if let Err(e) = std::fs::create_dir(&new_folder_path) {
        println!("Failed to create folder: {}", e);
        return;
    }

    // ファイルのパスを作成します
    let file_path = new_folder_path.join("client.toml");

    // ファイルを作成しデータを書き込みます
    let _file = match File::create(&file_path) {
        Err(e) => {
            println!("Failed to create file: {}", e);
            return;
        }
        Ok(mut f) => {
            if let Err(e) = f.write_all(b"Hello, world!") {
                println!("Failed to write data to file: {}", e);
                return;
            }
        }
    };
    
    println!("{}", temp_dir.display());

    // let mut output = Command::new("cmd")
    //     .args(&["/C", "java", "-jar", "paper-1.19.4-540.jar","nogui"])
    //     .spawn()
    //     .expect("Failed to execute command");

    // let status = output.wait().expect("Failed to wait for command");

    // if status.success() {
    //     println!("Command executed successfully");
    // } else {
    //     println!("Command failed");
    // }

    
    let result = config::add(2, 3); // otherモジュールのadd関数を呼び出す
    println!("Result: {}", result);

    println!("Enterを入力すると終了します....");

    // Enterキーを待ちます
    let stdin = io::stdin();
    let _ = stdin.lock().lines().next();

    // プログラムを終了します
    println!("Program finished.");
}
