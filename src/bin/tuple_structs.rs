struct Color(u8, u8, u8);
struct Point(i32, i32, i32);
struct Date(u8,u8,u8);

struct AlwaysEqual;
fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    let today = Date(24,6,12);

    let Date(year, month, day) = today; // 解构元组结构体
    println!("today date extracted: ({}-{}-{})", year, month, day);

    println!("black color rgb: ({}, {}, {})", black.0, black.1, black.2);
    println!("origin point coordinates: ({}, {}, {})", origin.0, origin.1, origin.2);
    println!("today date: ({}-{}-{})", today.0, today.1, today.2);

    let subject = AlwaysEqual;
    show_color_in_console(&black);
    // show_color_in_console(&Date); // 这样是不行的，因为Date不是Color类型
}

fn show_color_in_console(c: &Color) {
    println!("color rgb: ({}, {}, {})", c.0, c.1, c.2);
}