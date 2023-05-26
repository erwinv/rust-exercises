fn main() {
    let ref_x: &i32;
    {
        let x: i32 = 10;
        ref_x = &x;
        println!("ref_x: {ref_x}"); // ok here
    } // referenced x dealloc'ed here
      // println!("ref_x: {ref_x}"); // ref is now dangling here
}
