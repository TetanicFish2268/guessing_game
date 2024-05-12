use std::io;
use std::cmp::Ordering;
use std::env;
use rand::Rng;

const MIN: i32 = 1;
const MAX: i32 = 100;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut uragane = false;

    // コマンドライン引数が "uragane" の場合、チートモードにする
    if let Some(arg) = args.get(1) {
        if arg == "uragane" {
            uragane = true;
        }
    }

    println!("数を当ててみて!");

    //数の生成
    let secret_number = rand::thread_rng().gen_range(MIN..=MAX);
    if uragane {
        println!("ここだけの話、秘密の数字は{}だよ!", secret_number);
    }

    loop {
        println!("予想を入力しよう 範囲は{}から{}だよ", MIN, MAX);

        //入力
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("ごめん、なんかエラーでたわ");

        //i32にキャスト
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("数字入れろボケ!");
                continue
            },
        };

        //判定
        println!("あなたの予想は{}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("もっと大きいよ!"),       //小さすぎ！
            Ordering::Greater => println!("もっと小さいよ!"),      //大きすぎ！
            Ordering::Equal => {
                println!("正解!おめでとう!");
                break;
            },
        }
    }
}
