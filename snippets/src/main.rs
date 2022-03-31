enum Color {
    Red(u8),
    Green,
    Blue,
}

/// 枚举元素的 Borrow check
pub fn main() {
    /* 枚举中的元素是常量, 不存在 move 的情况 */
    paint(Color::Red(255));
    paint(Color::Red(255));
    paint(Color::Green);
    paint(Color::Green);
    paint(Color::Blue);
    paint(Color::Blue);

    /* 枚举类型的变量, 要遵守 borrow check */
    let color1 = Color::Green;
    let _v = color1;
    // paint(color1); // Error: used of moved value
}

fn paint(_: Color) {}
