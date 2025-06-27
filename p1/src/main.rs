fn main() {
    // p1();
    p2()
}

fn p1() {
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
    println!("Pi is roughly {PI:.3}.")
}

fn p2() {
    //https://rustwiki.org/zh-CN/rust-by-example/hello/print/print_debug.html
    /* 所有的类型，若想用 std::fmt 的格式化打印，都要求实现至少一个可打印的 traits。
    仅有一些类型提供了自动实现，比如 std 库中的类型。所有其他类型都必须手动实现。 */
    struct UnPrintable(i32);

    //fmt::Debug trait ,所有类型都能推导, Display trait 需要手动实现
    #[derive(Debug)]
    struct DebugPrintable(i32);

    let _unprintable = UnPrintable(3);
    let printable = DebugPrintable(3);
    //println!("{:?}", unprintable);
    println!("{:?}", printable);
}
