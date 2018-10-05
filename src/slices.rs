fn print_slice(slice: &[i32]){
    println!("slice = {:?}", slice);
}

pub fn slice(){
    let data_arr = [1,2,3,4,5,6,7,8,9];
    print_slice(&data_arr[3..8]);
}