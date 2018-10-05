union IntOrFloat {
    i: i32,
    f: f32,
}

fn print_int_or_float(iof: IntOrFloat) {
    unsafe {
        match iof {
            IntOrFloat { i: 42 } => println!("Meaning of life!"),
            IntOrFloat { f } => println!("f32 = {}", f),
        }
    }
}

pub fn unions() {
    // Solo int
    let mut iof = IntOrFloat { i: 64 };
    iof.i = 42;

    // Es unsafe porque no se cual de los dos viene
    let val = unsafe { iof.i };
    println!("val = {}", val);

    print_int_or_float(iof);
    print_int_or_float(IntOrFloat { f: 1.25 });
}
