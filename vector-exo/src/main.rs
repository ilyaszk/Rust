fn main() {

    let mut vec: Vec<i32> = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);
    vec.push(4);
    vec.push(5);
    vec.push(6);
    vec.push(7);
    vec.push(8);
    vec.push(9);
    vec.push(10);

    std::println!("Vector: {:?}", vec);


    std::println!("Slice: {:?}", &vec[2..5]);


}
