use std::io::stdin;

fn str_to_num(strnum: &String) -> f32 {
    let strnum: f32 = match strnum.trim().parse() {
        Ok(num) => num,
        Err(_) => 0.0,
    };

    strnum
}

fn prepor(first_num: f32, first_den: f32, second_den: f32) -> f32 {
    return (first_num as f32 * second_den as f32) / first_den as f32;
}

fn main() {
    let mut fir_top = String::new();
    let mut fir_bottom = String::new();
    let mut sec_bottom = String::new();

    println!("Enter numarator of the first proportion");
    stdin()
        .read_line(&mut fir_top)
        .expect("Failed to read line");

    println!("Enter the denomanator of the first proportion");
    stdin()
        .read_line(&mut fir_bottom)
        .expect("Failed to read lne");

    println!("Enter the denomanator of the second proportion");
    stdin()
        .read_line(&mut sec_bottom)
        .expect("Faled to read line");

    let result = prepor(
        str_to_num(&fir_top),
        str_to_num(&fir_bottom),
        str_to_num(&sec_bottom),
    );

    println!("{}/{}", result, &sec_bottom)
}
