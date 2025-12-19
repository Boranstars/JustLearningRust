
fn longest<'a>(x: &'a str,y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

/**
 * 生命周期的规则：
 * 1. 每一个引用都有它的生命周期，编译器通过生命周期来确保引用在有效范围内使用
 * 2. 函数或方法的每一个参数生命周期都是独立的
 * 3. 如果只有一个输入生命周期参数，那么它被赋予所有输出生命周期参数
 * 4. 如果有多个输入生命周期参数，但其中一个是 &self 或 &mut self，那么 self 的生命周期被赋予所有输出生命周期参数
 * 
 * 
 * 
 * */ 


fn main() {
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {result}");
    }
    
}