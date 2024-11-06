enum IpAddKind {
    v4,
    v6,
}

// enum IpAddKind {
//     v4(u8, u8, u8, u8),
//     v6(String)
// }

fn main() {
    let four = IpAddKind::V4;
    let six = IpAddKind::V6;

    route(four);
    route(six);
    route(IpAddKind::V4);
}

fn route(ip_kind: IpAddKind) {}

// // 定义枚举 Coin，其中 Quarter 包含一个附加值（例如：一个字符串来表示州）
// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter(String),  // Quarter 现在绑定了一个 String 类型的值
// }

// // 定义函数来返回硬币的面值
// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => 1, 
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter(state) => {
//             println!("State quarter from {}!", state);  // 使用绑定的值
//             25
//         }
//     }
// }

// fn main() {
//     // 测试不同硬币的值
//     let coin1 = Coin::Quarter(String::from("California"));
//     let coin2 = Coin::Penny;

//     println!("Value of coin1: {} cents", value_in_cents(coin1));
//     println!("Value of coin2: {} cents", value_in_cents(coin2));
// }

fn main() {
    let v = Some(0u8); // 这里的 v 是 Option<u8> 类型，表示变量 v 中确实包含了一个 u8 值为 0。
    match v {
        Some(3) => println!("three"),
        _ => (),
    }

    if let Some(3) = v {
        println!("three");
    }
}
