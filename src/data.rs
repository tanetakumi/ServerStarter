
pub fn show_logo(){
    println!("
     ____                       __                              ____    ____         ___       
    6MMMMb.                    69MM                             `MM'    `MM'          MM       
   8P    YM                   6M'                                MM      MM           MM       
  6M      Y ___  __    ___   _MM__/M      ____  ___  __   ____   MM      MM ___   ___ MM____   
  MM        `MM 6MM  6MMMMb  MMMM/MMMMM  6MMMMb `MM 6MM  6MMMMb. MM      MM `MM    MM MMMMMMb  
  MM         MM69 \" 8M'  `Mb  MM  MM    6M'  `Mb MM69 \" MM'    ` MMMMMMMMMM  MM    MM MM'  `Mb 
  MM         MM'        ,oMM  MM  MM    MM    MM MM'    YM.      MM      MM  MM    MM MM    MM 
  MM         MM     ,6MM9'MM  MM  MM    MMMMMMMM MM      YMMMMb  MM      MM  MM    MM MM    MM 
  YM      6  MM     MM'   MM  MM  MM    MM       MM          `Mb MM      MM  MM    MM MM    MM 
   8b    d9  MM     MM.  ,MM  MM  YM.  ,YM    d9 MM     L    ,MM MM      MM  YM.   MM MM.  ,M9 
    YMMMM9  _MM_    `YMMM9'Yb_MM_  YMMM9 YMMMM9 _MM_    MYMMMM9 _MM_    _MM_  YMMM9MM_MYMMMM9");
}

pub fn attention(){
    println!("\n");
    println!("==================== 注意 ====================");
    println!("||                                          ||");
    println!("||    このコンソールは ✖ で閉じないでね。  ||");
    println!("||    サーバーは stop で終了させましょう。  ||");
    println!("||                                          ||");
    println!("==============================================");
    println!("\n");
}

pub fn instraction(){
    println!("================ CraftersHub のロビーとつなげる方法 ================");
    println!("● Paper の場合");
    println!("   1. config -> paper-global.ymlを以下のように編集しましょう。");
println!("
      proxies:
        ..
        velocity:
          enabled: true
          online-mode: false
          secret: haneserver
");
    println!("   2. server.properties の項目で online-mode=false に変更しましょう。\n");
    println!("● Fabric の場合");
    println!("   Hane さんから HaneMOD をもらって mods フォルダーに入れましょう。\n");
    println!("====================================================================");
    println!("\n");
}

pub fn default_config() -> String {
    return "### Haneさんからもらったトークン  (外部公開しないように！！)
    ### ポート解放なしで接続ができるようになる
    
    token=
    
    
    ### サーバーのポート番号 (デフォルト:25565)
    
    port=25565
    
    ### サーバーの種類 (現在の対応 fabric / paper )
    
    server=paper
    
    ### javaの\"パス\"（デフォルトはパスが通っている場合）
    ### マイクラバージョン	 推奨Javaバージョン
    ###  1.7.10 - 1.16.5	    Java8
    ###  1.17 - 1.17.1	        Java16
    ###  1.18 -                 Java17
    
    java_path=java
    
    ### 割り当てメモリ (単位はGB)
    ### おすすめ 16Gあるとき 3G - 4G、32Gあるとき 6G - 8G
    
    memory=3
    
    ### サーバーを作るフォルダー名
    
    folder_name=myserver".to_string();
}