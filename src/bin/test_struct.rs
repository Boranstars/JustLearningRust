struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
// 为结构体实现方法
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let user1 = User{
        active : true,
        username : String::from("someusername123"),
        email : String::from("someone@example.com"),
        sign_in_count : 1,
    };

    let user2 = build_user(String::from("a@b.com"), String::from("boran"));
    println!("{0}", user2.username);
    println!("{0}", user2.email);
    dbg!(&user2.username);
    let name = &user2.username;


    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("{:?}", rect1);
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
  
}

/**
 * 构造User实例的函数，这里我们直接获得参数的所有权
 */
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username, // 简写形式，相当于username: username
        email,
        sign_in_count: 1,
    }
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}