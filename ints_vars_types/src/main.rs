fn main() {
    let a = 10;
    let b: i32 = 20;
    let c = 30i32;
    let d = 30_i32;
    let e = add(add(a, b), add(c, d));
    //println!("(a + b) + (c + d) = {}", e);
    //basic_operations();
    //base_2_base_8_base_16();
    compare_different_types();
}

fn add(i: i32, j: i32) -> i32 {
    i + j
}

fn basic_operations() {
    let twenty = 20;
    let twenty_one: i32 = 21;
    let twenty_two = 22i32;

    let addition = twenty + twenty_one + twenty_two;
    println!(
        "{} + {} + {} = {}",
        twenty, twenty_one, twenty_two, addition
    );

    let one_million: i64 = 1_000_000;
    println!("{}", one_million.pow(2));

    let forty_twos = [42.0, 42f32, 42.0_f32];

    println!("{:02}", forty_twos[0]);
}

fn base_2_base_8_base_16() {
    let three = 0b11;
    let thirty = 0o36;
    let three_hundred = 0x12C;

    println!("base 10: {} {} {}", three, thirty, three_hundred);
    println!("base 2:  {:b} {:b} {:b}", three, thirty, three_hundred);
    println!("base 8:  {:o} {:o} {:o}", three, thirty, three_hundred);
    println!("base 16: {:x} {:x} {:x}", three, thirty, three_hundred);
}

fn compare_different_types() {
    let a: i32 = 10;
    let b: u16 = 100;

    let b_ = b.try_into().unwrap();

    if a < b_ {
        println!("Ten is less than one hundred.");
    }
}
