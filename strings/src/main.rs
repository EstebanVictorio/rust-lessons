fn main() {
    let data = "initial content";
    let s = data.to_string();
    let s = "initial content".to_string();
    let mut s = String::from(data);
    println!("{}", s);
    s.push_str(" is chido");
    s.push('r');
    s.push('i');
    println!("{}", s);
}
