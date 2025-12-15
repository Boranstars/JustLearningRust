// practice for struct and impl
struct Rectangle {
    width: u32,
    height: u32,
    name: String,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
       self.width > other.width && self.height > other.height
    }

    /**
     * 关联函数，不需要self参数,类似于静态方法,通常用作构造函数
     */
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
            name: String::from("square"),
        }
    }   
}

fn main () {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
        name: String::from("MyRect"),
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
        name: String::from("MyRect"),
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
        name: String::from("testRect"),
    };

    let _rect4 = Rectangle {
        width: 100,
        ..rect2 // 使用结构体更新语法
    };

    let square_rect = Rectangle::square(20); // 调用关联函数
    let square_area = Rectangle::area(&square_rect); // 实际上也可以这样调用方法
    println!("Square rect area: {}", square_area);

    println!("The area of the rectangle is {} square pixels.", rect1.area());
    // println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2)); // 但是结构体更新语法会导致rect2被部分移动
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}