pub fn run() {
    println!("Macros!!!");

    let v1 = vec![1, 2, 3];
    let v2 = vec!["A", "B", "C"];

    println!("{:#?} - {:#?}", v1, v2)
}

#[macro_export]
macro_rules! vec {
    ($($x: expr), *) => {
        let mut temp_vec = Vec::new();
        $(
            temp_vec.push($x);
        )*

        temp_vec
    };
}
