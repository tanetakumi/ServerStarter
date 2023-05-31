use serde_json::Value;
use std::path::{ PathBuf, Path };
use std::fs::File;
use std::io::Write;
use std::error::Error;
use std::process::{Command, Child, Stdio};

pub fn run_rathole(token :&str, port : u16, temp_dir_path: &PathBuf) -> Result<Child, Box<dyn Error>> {
    download(&temp_dir_path)?;
    create_config(&token, port, &temp_dir_path)?;
    let rathole_exe = temp_dir_path.join("rathole.exe").into_os_string().into_string().unwrap();
    let config_path = temp_dir_path.join("client.toml").into_os_string().into_string().unwrap();

    let output = Command::new(&rathole_exe)
        .args(&[&config_path])
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .spawn()?;
    return Ok(output)
}

pub fn stop_rathole(child: &mut Child) -> Result<(), Box<dyn Error>> {
    child.kill()?;
    child.wait()?;
    Ok(())
}

fn download(destination: &PathBuf) -> Result<(), Box<dyn Error>> {
    if !destination.join("rathole.exe").exists() {
        let client = reqwest::blocking::Client::builder()
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64)")
            .build()?;
        
        let response = client.get("https://api.github.com/repos/rapiz1/rathole/releases/latest").send()?;
        let body = response.text()?;
        let json: Value = serde_json::from_str(&body)?;

        if let Some(assets) = json["assets"].as_array() {
            for asset in assets {
                if let Some(name) = asset["name"].as_str() {
                    if name.contains("windows") {
                        if let Some(url) = asset["browser_download_url"].as_str() {
                            let file_path = destination.join(name);
                            println!("必要なものをダウンロードしています...");
                            if let Err(err) = download_file(url, &file_path) {
                                eprintln!("ファイルのダウンロードに失敗しました。: {}", err);
                            } else {
                                println!("ダウンロード完了");
                                if let Err(err) = extract_zip_file(&file_path) {
                                    eprintln!("ファイルの展開に失敗しました。: {}", err);
                                } else {
                                    println!("ファイルを展開しました。");
                                }
                            }
                        }
                    }
                }
            }
        }        
    }
    Ok(()) // 追加した行
}

fn create_config(token :&str, port : u16, path: &PathBuf) -> Result<(),  Box<dyn Error>> {

    let file_path = path.join("client.toml");

    let contents = format!("
    [client]
    remote_addr = \"157.120.32.103:2333\"
    [client.services.{}]
    token = \"{}\"
    local_addr = \"127.0.0.1:{}\"
    ", 
    token, token, port);
    let mut file = File::create(file_path)?;
    file.write_all(contents.as_bytes())?;
    Ok(())
}

fn download_file(url: &str, destination: &std::path::Path) -> Result<(), Box<dyn Error>> {
    let mut response = reqwest::blocking::get(url)?;
    let mut file = std::fs::File::create(destination)?;
    std::io::copy(&mut response, &mut file)?;
    Ok(())
}

fn extract_zip_file(file_path: &Path) -> Result<(), Box<dyn Error>> {
    let mut archive = zip::ZipArchive::new(File::open(file_path)?)?;

    if let Some(parent) = file_path.parent() {
        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;
            let file_path = parent.join(file.mangled_name());

            if file.is_dir() {
                std::fs::create_dir_all(&file_path)?;
            } else {
                if let Some(parent) = file_path.parent() {
                    std::fs::create_dir_all(parent)?;
                }
                let mut output_file = File::create(&file_path)?;
                std::io::copy(&mut file, &mut output_file)?;
            }
        }
    } else {
        return Err("ファイルの親ディレクトリを取得できません".into());
    }

    Ok(())
}