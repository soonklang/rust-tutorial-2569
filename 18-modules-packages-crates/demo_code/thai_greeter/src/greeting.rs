fn decorate(text: &str) -> String {
    format!("*** {} ***", text)
}

pub fn hello(name: &str) -> String {
    decorate(&format!("สวัสดี {}", name))
}
