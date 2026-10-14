fn show_fat_pointer<T: std::fmt::Debug>(data: &[T]) {
    let ptr = data.as_ptr();
    let len = data.len();

    println!("ptr : {:p} | len : {}", ptr, len);
    println!("size of &[T] : {} bytes", std::mem::size_of::<&[T]>());
    println!("size of T : {} bytes", std::mem::size_of::<T>());

    let s1 = &data[..data.len() - 2];
    println!("slice ptr : {:p} | len : {}", s1.as_ptr(), s1.len());
    println!("{:?}", s1);
}

pub main() {
    let arr = [1, 2, 3, 4, 5, 6];
    let vec = vec![1, 2, 3, 4, 5, 6];

    println!("--- array ---");
    println!("{:?}", arr);
    show_fat_pointer(&arr);

    println!("--- vector ---");
    println!("{:?}", vec);
    show_fat_pointer(&vec);
}
