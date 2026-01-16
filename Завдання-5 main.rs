fn main() {
    let v1 = 251_u8.wrapping_add(8);
    let v2 = u8::checked_add(251, 8).unwrap_or(0);

    println!("{},{}", v1, v2);
}
