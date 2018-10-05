pub fn strings(){
    let s:&'static str = "Hola string slice";
    print_upper(s);
}

fn print_upper(s: &'static str){
    let result:Vec<_>=  s.chars().map(|c| c.to_uppercase().next().unwrap()).collect();
    println!("{:?}",result);
}