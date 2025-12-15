fn main() {
    // Rust 中没有 null 值的概念，取而代之的是 Option<T> 枚举类型
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;

    // Option<T> 和 T 之间不能直接进行运算，需要先处理 Option<T>
    let x: i32 = 5;
    let y: Option<i32> = Some(10);
    let y: Option<i32> = add_one(y); // 使用自定义函数处理 Option<T>
    // let sum = x + y; // 这样是不行的
    let sum = x + y.unwrap_or(0); // 使用 unwrap_or 提供一个默认值
    println!("Sum: {}", sum);
    // 或者直接unwrap，但如果是None会panic
    let sum2 = x + y.unwrap();
    println!("Sum2: {}", sum2);
    // let make_it_panic = x + absent_number.unwrap(); // 这行代码会导致程序panic

    // 使用 match 处理 Option<T>
    match some_number {
        Some(value) => println!("The number is: {}", value),
        None => println!("No number found"),
    }

    // 使用 if let 处理 Option<T>,这是语法糖
    if let Some(value) = some_string {
        println!("The string is: {}", value);
    } else {
        println!("No string found");
    }

    // 等价于下面的 match，这只匹配Some变体,else相当于_
    match some_string {
        Some(value) => println!("The string is: {}", value),
        _ => println!("No string found"),
    }

    // 可以完全忽略不是匹配的情况
    if let Some(value) = some_number {
        println!("We have a number {value}");
    }


}

/**
 * 一个接受Option<i32>并返回Option<i32>的函数
 */
fn add_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(value) => Some(value + 1),
        None => None,
    }
}