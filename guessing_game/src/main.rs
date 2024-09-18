use rand::Rng; //trait
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("请你猜一个数字");

    let secret_num = rand::thread_rng().gen_range(1, 101);
    loop {
        let mut guess_num = String::new();

        // io::Result OK, Err
        io::stdin().read_line(&mut guess_num).expect("无法读取行");

        let guess_num: u32 = match guess_num.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("请输入有效的数字！");
                continue;
            }
        };

        match guess_num.cmp(&secret_num) {
            Ordering::Less => println!("太小了！"),
            Ordering::Greater => println!("太大了！"),
            Ordering::Equal => {
                println!("猜对了！");
                break;
            }
        }
    }
}
