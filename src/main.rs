fn main() {
    let i_r1: f64 = 10.0;
    let o_r2: f64 = 2.0;
    let tau: f64 = std::f64::consts::TAU;

    let mut theta: f64 = 0.0;

    loop {
        let mut theta1: f64 = 0.0;

        while theta1 < tau {
            let d = i_r1 + o_r2 * theta1.cos();

            let x = d * theta.cos();
            let y = d * theta.sin();
            let z = o_r2 * theta1.sin();

            theta1 += 0.05;
        }

        theta += 0.05;

        if theta >= tau {
            theta = 0.0;
        }
    }
}
