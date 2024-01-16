use std::io;

fn main() {
    loop {
        // プロンプトを表示
        print!("> ");

        // 標準入力から文字列を読み取る
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("faild to read line");

        println!("You enterd: {}", input);

        // 空白で区切り、ベクタに格納
        let tokens: Vec<&str> = input.split_whitespace().collect();

        // コマンドと引数を分ける
        let command = tokens[0];
        let args = tokens[1..].to_vec();

        println!("コマンド: {}", command);
        println!("引数: {:?}", args);
    }
}
