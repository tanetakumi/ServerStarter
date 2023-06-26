// use std::path::PathBuf;
// use std::io::Read;
// use std::fs::File;


// pub fn eula_check(path: &PathBuf) -> bool {
//     let eula_file = path.join("eula.txt");
//     if eula_file.is_file() {
//         let mut f = File::open(&eula_file).expect("ファイルが開けませんでした。");
//         let mut contents = String::new();
//         f.read_to_string(&mut contents).expect("ファイルが読み込めませんでした。");
//         // ファイルの内容に "eula=true" が含まれているかを確認
//         if contents.contains("eula=true") {
//             return true;
//         } else {
//             println!("eula=true が読み取れませんでした。");
//             return false;
//         }
//     } else {
//         println!("\x1b[31meula=true が読み取れませんでした。\x1b[0m");
//         return false;
//     }
// }


// pub fn server_check(path: &PathBuf, server_type: &str) {
//     if server_type == "paper" {
//         println!("paper");
//         println!("{}", paper_check(&path));
//     } else if server_type == "fabric" {
//         println!("{}", fabric_check(&path));
//     } else {
//         eprintln!("サーバーはpaperかfabricを指定してください。対応してないバージョンもあります。");
//     }
// }

// fn fabric_check(path: &PathBuf) -> bool{
//     let mods_dir = path.join("mods");  
//     let regex = Regex::new(r"HaneMod.*\.jar").unwrap();
//     if mods_dir.is_dir() {
//         let entries = fs::read_dir(mods_dir).unwrap();
//         for entry in entries {
//             if let Ok(entry) = entry {
//                 if let Some(file_name) = entry.file_name().to_str() {
//                     if regex.is_match(file_name) {
//                         println!("必要なMODが見つかりました。: {}", file_name);
//                         return true;
//                     }
//                 }
//             }
//         }
//     } else {
//         println!("フォルダが存在しません。\n => {} ",mods_dir.display());
//         return false;
//     }
//     println!("fabric を接続するためには HaneMod が必要です。");
//     return false;
// }

// fn paper_check(path: &PathBuf) -> bool{
//     let config_file = path.join("config").join("paper-global.yml"); 
//     println!("{}", config_file.display());
//     if config_file.is_file() {
//         ファイルを開く
//         let mut file = File::open(&config_file).expect("ファイルが開けませんでした。");
//         ファイルの内容を読み取る
//         let mut contents = String::new();
//         file.read_to_string(&mut contents).expect("ファイルが読み込めませんでした。");

//         YAML形式のパース
//         let yaml_document = match YamlLoader::load_from_str(&contents) {
//             Ok(documents) => {
//                 if documents.is_empty() {
//                     println!("test.config file is empty");
//                 }
//                 println!("{:?}", documents[0]["proxies"]["velocity"].clone());
//             }
//             Err(err) => {
//                 println!("Failed to parse test.config file: {}", err);
//             }
//         };

//         println!("{}", contents);
//     }
//     return false;
// }