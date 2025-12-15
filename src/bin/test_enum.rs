enum IPAddrKinds {
    V4(u8, u8, u8, u8),
    V6(String),
} // 定义一个枚举类型，包含两种变体

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
    NeverUse,
} // 定义一个包含不同数据类型的枚举

// 枚举也可以实现方法
impl Message {
    fn call(&self) {
        // match表达式来处理不同的枚举变体
        match self {
            Message::Quit => {
                println!("Quit message")
            },
            Message::Move { x, y } => println!("Move to ({}, {})", x, y),  //当然可以简写语句，不用大括号
            Message::Write(text) => println!("Write message: {}", text), 
            Message::ChangeColor(r, g, b) => println!("Change color to RGB({}, {}, {})", r, g, b), // 可以解构数据，然后使用
            _ => {
                println!("Other message")
            } // _ 会忽略其他未列出的变体的值

            // 也可以使用other通配符变量来接收未列出的变体，但是other会绑定到具体的值上
            // other => {}
        }
    }
}

impl IPAddrKinds {
    fn display(&self) {
        match self {
            IPAddrKinds::V4(a, b, c, d) => println!("IPv4 Address: {}.{}.{}.{}", a, b, c, d),
            IPAddrKinds::V6(addr) => println!("IPv6 Address: {}", addr),
        }
    }
}



fn main() {
    
    let localhost = IPAddrKinds::V4(127, 0, 0, 1);
    let loopback = IPAddrKinds::V6(String::from("::1"));

    localhost.display();
    loopback.display();

    let msg1 = Message::Quit;
    let msg2 = Message::Move { x: 10, y: 20 };
    let msg3 = Message::Write(String::from("Hello, world!"));
    let msg4 = Message::ChangeColor(255, 0, 0);
    msg1.call();
    msg2.call();
    msg3.call();
    msg4.call();
}