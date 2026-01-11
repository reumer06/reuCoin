use std::f64::consts::E; // Binomial Random Walk

fn attacker_success_possibility(q: f64,z : i32) -> f64{
    let p = 1.0 - q;
    let lamda =  z as f64 * (q/p);
    let mut sum = 1.0;
    for k in 0..=z {
        let mut poisson  = E.powf(-lamda);
        for i in 1..=k {
            poisson *=lamda / i as f64;
        }
        sum -= poisson * (1.0 - (q/p).powi(z - k));
    }
    sum
}

fn main() {
    attacker_success_possibility(67.69,12);
}