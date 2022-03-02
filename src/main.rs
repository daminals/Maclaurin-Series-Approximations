// taylor
// Daniel Kogan
// 02.28.2022


use statrs::function::factorial::factorial;

fn main() {
    e_x(7.0);
}

fn sin(x: f64) {
    // (-1)^n * x^(2n+1)
    // -----------------
    // (2n+1)!
    let mut n = 0;
    let mut sum: f64 = 0.0;
    while n<150 { // goes up to 19 factorial before overflow
        let negative_1: i32 = -1;
        let two_n_plus_1: f64 = (*&n  * 2 + 1) as f64;
        sum += ((negative_1.pow(*&n) as f64 ) * (*&x).powf(*&two_n_plus_1))/(factorial(two_n_plus_1 as u64) as f64);
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char); // clear terminal
        println!("{}", sum);
        n+=1;
    }    
}

fn cos(x: f64) {
    // (-1)^n * x^(2n)
    // -----------------
    // (2n)!
    let mut n = 0;
    let mut sum: f64 = 0.0;
    while n<150 { // goes up to 19 factorial before overflow
        let negative_1: i32 = -1;
        let two_n: f64 = (*&n  * 2 ) as f64;
        sum += ((negative_1.pow(*&n) as f64 ) * (*&x).powf(*&two_n))/(factorial(two_n as u64) as f64);
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char); // clear terminal
        println!("{}", sum);
        n+=1;
    }    
}

fn e_x(x: f64) {
    // x^(n)
    // -----------------
    // (n)!
    let mut n = 0;
    let mut sum: f64 = 0.0;
    while n<150 { // goes up to 19 factorial before overflow
        let negative_1: i32 = -1;
        let n_f64: f64 = (*&n) as f64;
        sum += ((*&x).powf(*&n_f64))/(factorial(n_f64 as u64)) as f64;
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char); // clear terminal
        println!("{}", sum);
        n+=1;
    }    
}

/* statrs factorial allows for up to 170! before overflow
this fn only allows 21!
fn factorial (num: u128) -> u128 {
    match num {
        0 => 1,
        1 => 1,
        _ => factorial(num - 1) * num,
    }
}
*/