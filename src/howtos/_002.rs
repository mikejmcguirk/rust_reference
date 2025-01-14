// Imported core module for later
use core::mem;

pub fn start() {
    let x = 5;
    println!("x = {x}");
    //x = 6; // If uncommented, this is a compiler error. x is is immutable

    let mut y = 5;
    println!("y = {y}");
    y = 6;
    println!("y = {y}");

    // constants must be known at compile time
    // constants must have type annotations
    // Constants are valid the entire time your program runs within the scope they are declared
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("THREE_HOURS_IN_SECONDS is {THREE_HOURS_IN_SECONDS}");

    // Rust allows shadowing. This will print 4
    let x = 4;
    println!("x = {x}");

    // Bare braces let you declare a scope
    {
        let x = 3;
        // This will now print 3
        println!("x = {x}");
    }

    // But this will now print 4 again
    println!("x = {x}");

    // Shadowing lets you change the type
    let z: u32 = 3;
    println!("The size of z in bytes is: {}", mem::size_of_val(&z));
    let z: u64 = 4;
    println!("The size of z in bytes is: {}", mem::size_of_val(&z));

    // Primitive number types are listed below
    // 8-bit
    let i8_var: i8 = -128;
    let u8_var: u8 = 255;
    println!(
        "Type: i8, Size: {} bytes, Min: {}, Max: {}",
        mem::size_of_val(&i8_var),
        i8::MIN,
        i8::MAX
    );
    println!(
        "Type: u8, Size: {} bytes, Min: {}, Max: {}",
        mem::size_of_val(&u8_var),
        u8::MIN,
        u8::MAX
    );

    // 16-bit
    let i16_var: i16 = -32768;
    let u16_var: u16 = 65535;
    println!(
        "Type: i16, Size: {} bytes, Min: {}, Max: {}",
        mem::size_of_val(&i16_var),
        i16::MIN,
        i16::MAX
    );
    println!(
        "Type: u16, Size: {} bytes, Min: {}, Max: {}",
        mem::size_of_val(&u16_var),
        u16::MIN,
        u16::MAX
    );

    // 32-bit
    let i32_var: i32 = -2_147_483_648;
    let u32_var: u32 = 4_294_967_295;
    println!(
        "Type: i32, Size: {} bytes, Min: {}, Max: {}",
        mem::size_of_val(&i32_var),
        i32::MIN,
        i32::MAX
    );
    println!(
        "Type: u32, Size: {} bytes, Min: {}, Max: {}",
        mem::size_of_val(&u32_var),
        u32::MIN,
        u32::MAX
    );

    // 64-bit
    let i64_var: i64 = -9_223_372_036_854_775_808;
    let u64_var: u64 = 18_446_744_073_709_551_615;
    println!(
        "Type: i64, Size: {} bytes, Min: {}, Max: {}",
        mem::size_of_val(&i64_var),
        i64::MIN,
        i64::MAX
    );
    println!(
        "Type: u64, Size: {} bytes, Min: {}, Max: {}",
        mem::size_of_val(&u64_var),
        u64::MIN,
        u64::MAX
    );

    // 128-bit
    let i128_var: i128 = -170_141_183_460_469_231_731_687_303_715_884_105_728;
    let u128_var: u128 = 340_282_366_920_938_463_463_374_607_431_768_211_455;
    println!(
        "Type: i128, Size: {} bytes, Min: {}, Max: {}",
        mem::size_of_val(&i128_var),
        i128::MIN,
        i128::MAX
    );
    println!(
        "Type: u128, Size: {} bytes, Min: {}, Max: {}",
        mem::size_of_val(&u128_var),
        u128::MIN,
        u128::MAX
    );

    // Architecture-dependent
    let isize_var: isize = -1;
    let usize_var: usize = 1;
    println!(
        "Type: isize, Size: {} bytes, Min: {}, Max: {}",
        mem::size_of_val(&isize_var),
        isize::MIN,
        isize::MAX
    );
    println!(
        "Type: usize, Size: {} bytes, Min: {}, Max: {}",
        mem::size_of_val(&usize_var),
        usize::MIN,
        usize::MAX
    );
}
