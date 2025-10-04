use colored::*;

/// 检查终端是否支持颜色输出
pub fn supports_color() -> bool {
    atty::is(atty::Stream::Stdout) && std::env::var("NO_COLOR").is_err()
}

/// 打印成功消息（绿色加粗）
pub fn print_success(msg: &str) {
    if supports_color() {
        println!("{}", msg.green().bold());
    } else {
        println!("{}", msg);
    }
}

/// 打印信息消息（蓝色）
pub fn print_info(msg: &str) {
    if supports_color() {
        println!("{}", msg.blue());
    } else {
        println!("{}", msg);
    }
}

/// 打印警告消息（黄色）
pub fn print_warning(msg: &str) {
    if supports_color() {
        println!("{}", msg.yellow());
    } else {
        println!("{}", msg);
    }
}

/// 打印错误消息（红色加粗）
pub fn print_error(msg: &str) {
    if supports_color() {
        println!("{}", msg.red().bold());
    } else {
        println!("{}", msg);
    }
}

/// 打印横幅消息（品红色加粗）
pub fn print_banner(msg: &str) {
    if supports_color() {
        println!("{}", msg.magenta().bold());
    } else {
        println!("{}", msg);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_supports_color() {
        // 这个测试只是确保函数能够被调用
        let _ = supports_color();
    }

    #[test]
    fn test_print_functions() {
        // 确保打印函数不会 panic
        print_success("Success test");
        print_info("Info test");
        print_warning("Warning test");
        print_error("Error test");
        print_banner("Banner test");
    }
}
