use std::fmt::{self, Formatter};

fn main() {
    // p1();
    //p2()
    // p3()
    // p4();
    p5()
}

fn p1() {
    /*
    fmt::Debug：使用 {:?} 标记。格式化文本以供调试使用。
    fmt::Display：使用 {} 标记。以更优雅和友好的风格来格式化文本
    */

    // 通常情况下，`{}` 会被任意变量内容所替换。
    // 变量内容会转化成字符串。
    println!("{} days", 31);
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

fn p3() {
    // https://rustwiki.org/zh-CN/rust-by-example/hello/print/print_display.html
    //fmt::Display 采用 {} 标记

    use std::fmt;

    struct Structure(i32);

    impl fmt::Display for Structure {
        // fmt::Display trait 需要实现一个 fmt 方法
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Structure i32: {}", self.0)
        }
    }

    let s = Structure(3);
    println!("{}", s);
}

fn p4() {
    use std::fmt;

    struct List(Vec<i32>);

    impl fmt::Display for List {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            // 使用元组的下标获取值，并创建一个 `vec` 的引用。
            let vec = &self.0;
            //?号 用于处理Result,当写入失败时会返回错误,如果成功则继续执行
            //等同于 match write!(f, "[") {
            //     Ok(_) => {},
            //     Err(e) => return Err(e),
            // };
            //老的 try! 也是同理
            write!(f, "[")?;

            // 使用 `v` 对 `vec` 进行迭代，并用 `count` 记录迭代次数。
            for (count, v) in vec.iter().enumerate() {
                // 对每个元素（第一个元素除外）加上逗号。
                // 使用 `?` 或 `try!` 来返回错误。
                if count != 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}:{}", count, v)?;
            }

            // 加上配对中括号，并返回一个 fmt::Result 值。
            write!(f, "]")
        }
    }

    let l = List(vec![1, 2, 3]);
    println!("{}", l);
}

fn p5() {
    struct Color {
        red: u8,
        green: u8,
        blue: u8,
    }

    impl fmt::Display for Color {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            write!(
                f,
                "RGB ({0:width$},{1:width$},{2:width$}) #0x{0:02X}{1:02X}{2:02X}",
                self.red,
                self.green,
                self.blue,
                width = 3
            )
        }
    }

    for color in [
        Color {
            red: 128,
            green: 255,
            blue: 90,
        },
        Color {
            red: 0,
            green: 3,
            blue: 254,
        },
        Color {
            red: 0,
            green: 0,
            blue: 0,
        },
    ]
    .iter()
    {
        // 在添加了针对 fmt::Display 的实现后，请改用 {} 检验效果。
        println!("{}", *color)
    }
}
