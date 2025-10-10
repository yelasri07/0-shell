pub fn echo_handler(args: Vec<String>) {
    let text = args.join(" ");
    println!("{}", text);
}