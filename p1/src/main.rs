fn main() {
    //格式化文本
    let greeting = format!("Hello, {}!", "world");
    println!("{}", greeting);
    //命名参数
    println!("Hello, {var1}!", var1 = "world");
    // 可以在 `:` 后面指定特殊的格式。
    println!("{} of {:b} people know binary, the other half don't", 1, 2);
    println!("{number:>width$}", number = 1, width = 6);
    println!("{number:0>width$}", number = 1, width = 6);

    const PI: f64 = 3.14159265358979323846;
    println!("Pi is roughly {PI:.3}.");
}
