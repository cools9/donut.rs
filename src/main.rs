fn main() {
    let i_r1: f64 = 10.0;
    let o_r2: f64 = 2.0;

    // let x_i_r1:f64= i_r1 * theta;
    let mut theta: f64 = 0.0;
    loop {
        let mut theta1: f64 = 0.0;

        theta += 0.001;
        if theta >= 6.283 {
            theta = 0.0;
        }
        let d: f64 = i_r1 + o_r2 * theta1.cos();
        let x_r1: f64 = d * theta.cos();
        let y_r1: f64 = d * theta.sin();
        let z_r1: f64 = o_r2 * theta.sin();

        loop {
            theta1 += 0.001;
            let x_r2: f64 = i_r1 + o_r2 * theta1.cos();
            let y_r2: f64 = i_r1 + o_r2 * theta1.sin();
            if theta1 >= 6.283 {
                theta1 = 0.0;
                break;
            }
        }
    }
}
