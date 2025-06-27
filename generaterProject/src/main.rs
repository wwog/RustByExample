use std::env;
use std::fs;

fn main() {
    //获取当前文件夹
    let current_dir = env::current_dir().unwrap();
    println!("当前文件夹: {}", current_dir.display());

    //用于跟踪当前项目
    let mut project_number = 0i32;

    //获取当前文件夹下的所有文件夹
    let entries = std::fs::read_dir(&current_dir).unwrap();
    for entry in entries {
        let entry = entry.unwrap();
        if entry.file_type().unwrap().is_dir() {
            let folder_name = entry.file_name().to_string_lossy().to_string();

            if folder_name.starts_with("p") && folder_name.len() > 1 {
                if let Ok(number) = folder_name[1..].parse::<i32>() {
                    if number > project_number {
                        project_number = number;
                    }
                }
            }
        }
    }

    println!("当前项目编号: {}", project_number);

    //创建新项目文件夹
    let new_project_folder = current_dir.join(format!("p{}", project_number + 1));
    std::fs::create_dir_all(&new_project_folder).unwrap();

    println!("创建新项目文件夹: {}", new_project_folder.display());
    //使用cargo init初始化项目
    let output = std::process::Command::new("cargo")
        .arg("init")
        .arg("--vcs=none")
        .current_dir(&new_project_folder)
        .output()
        .unwrap();

    if output.status.success() {
        //将Cargo.toml文件追加内容
        //[lints.rust]
        //dead_code = "allow"
        let cargo_toml_path = new_project_folder.join("Cargo.toml");
        fs::write(&cargo_toml_path, "[lints.rust]\ndead_code = \"allow\"").unwrap();
        println!("成功初始化项目");
    } else {
        println!("初始化项目失败");
    }
}
