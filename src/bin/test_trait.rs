use std::{fmt::Display};
pub trait Summary {
    // 定义一个方法签名
    fn summrise(&self) -> String;
    fn default_summrise(&self) -> String {
        // 提供一个默认实现
        String::from("(Read more...)")
    }
}
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

/**
 * 一个使用 trait 作为参数的函数
 */
fn notify(item: &impl Summary) {
    // Duck typing: 只要传入的类型实现了 Summary trait 就可以
    println!("Breaking news! {}", item.summrise());
}

impl Summary for NewsArticle {
    fn summrise(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn new(x: T, y: T) -> Self {
        Point { x, y }
    }
}

// 现在我们可以自定义打印方法,当然这通常是通过derive宏来实现的
impl<T: Display> Display for Point<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Point({}, {})", self.x, self.y) 
    }
}
fn main() {
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best \
                    hockey team in the NHL."),
    };
    println!("New article available! {}", article.summrise());
    println!("Default summary: {}", article.default_summrise());

    notify(&article);

    let point = Point::new(3, 4);
   
    println!("{point}");
}