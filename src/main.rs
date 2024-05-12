use std::io;
use std::cmp::Ordering;
use std::env;
use rand::Rng;

fn main() {
    let args: Vec<String> = env::args().collect();
    let arg = args.get(1).unwrap().clone();
    let mut uragane = false;

    if arg == String::from("uragane") {
        uragane = true;
    }

    println!("数を当ててみて!");

    let secret_number = rand::thread_rng().gen_range(1..101);
    if uragane {
        println!("ここだけの話、秘密の数字は{}だよ!", secret_number);
    }

    loop {
        println!("予想を入力しよう");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("ごめん、なんかエラーでたわ");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("数字入れろボケ!");
                continue
            },
        };

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
