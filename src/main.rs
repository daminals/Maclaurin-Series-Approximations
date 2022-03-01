fn main() {
    sin(5.0);
}

fn sin(x: f64) {
    // (-1)^n * x^(2n+1)
    // -----------------
    // (2n+1)!
    let mut n = 0;
    let mut sum: f64 = 0.0;
    while n<9 {
        let negative_1: i32 = -1;
        let two_n_plus_1: f64 = (*&n  * 2 + 1) as f64;
        sum += ((negative_1.pow(*&n) as f64 ) * (*&x).powf(*&two_n_plus_1))/(factorial(two_n_plus_1 as u128) as f64);
        //print!("{esc}[2J{esc}[1;1H", esc = 27 as char); // clear terminal
        println!("{}", sum);
        n+=1;
    }    

}

fn factorial (num: u128) -> u128 {
    match num {
        0 => 1,
        1 => 1,
        _ => factorial(num - 1) * num,
    }
}