enum Hello {
    Int(i32),
    Text(String)
}

fn main() {
    let mut vector = vec![1,2,3];
    for i in &vector {
        print!("{}, ", i);
    }

    vector.push(4);
    vector.push(5);
    vector.push(6);
    vector.push(7);

    for i in &vector {
        print!("{}, ", i);
    }

    for i in &mut vector {
        *i+= 1;
        print!("{}, ", i);
    }
    

    let row = vec![
        Hello::Int(55),
        Hello::Text(String::from("Hello"))
    ];


}
