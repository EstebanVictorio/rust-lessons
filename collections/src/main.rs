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

    
}
