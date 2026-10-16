use crossterm::terminal::size;

fn main() {
    let i_r1: f64 = 10.0;
    let o_r2: f64 = 2.0;
    let tau: f64 = std::f64::consts::TAU;
    let scale: f64 = 1.0;

    let (width, height) = size().unwrap();

    let mut output = vec![' '; (width * height) as usize];

    let mut theta: f64 = 0.0;

    while theta < tau {
        let mut theta1: f64 = 0.0;

        while theta1 < tau {
            let d = i_r1 + o_r2 * theta1.cos();

            let x = d * theta.cos();
            let y = d * theta.sin();

            let screen_x = (width as f64 / 2.0 + x * scale) as usize;
            let screen_y = (height as f64 / 2.0 - y * scale) as usize;

            if screen_x < width as usize && screen_y < height as usize {
                let index = screen_y * width as usize + screen_x;
                output[index] = '@';
            }

            theta1 += 0.01;
        }

        theta += 0.01;
    }

    for row in output.chunks(width as usize) {
        let line: String = row.iter().collect();
        println!("{line}");
    }
}
