fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    // Safe way to modify the vector
    vec[0] = 4;

    println!("Vec: {:?}", vec);
} 