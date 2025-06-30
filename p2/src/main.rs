fn main() {
    p2_1();
}

fn p2() {
    // 有符号整数（signed integers）：i8、i16、i32、i64、i128 和 isize（指针宽度）
    // 无符号整数（unsigned integers）： u8、u16、u32、u64、u128 和 usize（指针宽度）
    // 浮点数（floating point）： f32、f64
    // char（字符）：单个 Unicode 字符，如 'a'，'α' 和 '∞'（每个都是 4 字节）
    // bool（布尔型）：只能是 true 或 false
    // 单元类型（unit type）：()。其唯一可能的值就是 () 这个空元组
    let si: i8 = -8;
    let ui: u8 = 12;
    let fb: f32 = 3.12;
    let ch: char = 'a';
    let undefined = ();
    // 复合类型（compound type）
    // 数组（array）：如 [1, 2, 3]
    // 元组（tuple）：如 (1, true)
    let array = vec![1, 2, 3];
    let tuple = (1, true);
}

fn p2_1() {
    println!("1 + 2 = {}", 1u32 + 2);

    // 整数相减
    println!("1 - 2 = {}", 1i32 - 2);
    // 试一试 ^ 尝试将 `1i32` 改为 `1u32`，体会为什么类型声明这么重要

    // 短路求值的布尔逻辑
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    // 位运算
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    // 使用下划线改善数字的可读性！
    println!("One million is written as {}", 1_000_000u32);
}
