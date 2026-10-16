use crossterm::{
    cursor, execute,
    terminal::{self, ClearType},
};
use std::{
    f64::consts::TAU,
    io::{Write, stdout},
    thread,
    time::Duration,
};

fn main() {
    let r1: f64 = 10.0;
    let r2: f64 = 3.0;

    let (width, height) = terminal::size().unwrap();
    let w = width as usize;
    let h = height as usize;

    let chars = b".,-~:;=!*#$@";

    let mut a: f64 = 0.0;
    let mut b: f64 = 0.0;

    let mut out = stdout();

    execute!(out, terminal::Clear(ClearType::All), cursor::Hide).unwrap();

    loop {
        let mut screen = vec![b' '; w * h];
        let mut zbuf = vec![f64::NEG_INFINITY; w * h];

        let (sa, ca) = (a.sin(), a.cos());
        let (sb, cb) = (b.sin(), b.cos());

        let mut theta: f64 = 0.0;

        while theta < TAU {
            let (st, ct) = (theta.sin(), theta.cos());

            let mut phi: f64 = 0.0;

            while phi < TAU {
                let (sp, cp) = (phi.sin(), phi.cos());

                let x = (r1 + r2 * cp) * ct;
                let y = (r1 + r2 * cp) * st;
                let z = r2 * sp;

                let x1 = x * ca + z * sa;
                let z1 = -x * sa + z * ca;

                let y2 = y * cb - z1 * sb;
                let z2 = y * sb + z1 * cb;

                let sx = (w as f64 / 2.0 + x1 * 2.0).round();
                let sy = (h as f64 / 2.0 - y2).round();

                if sx >= 0.0 && sx < w as f64 && sy >= 0.0 && sy < h as f64 {
                    let px = sx as usize;
                    let py = sy as usize;
                    let idx = py * w + px;

                    if z2 > zbuf[idx] {
                        zbuf[idx] = z2;

                        let nx = cp * ct;
                        let ny = cp * st;
                        let nz = sp;

                        let nx1 = nx * ca + nz * sa;
                        let nz1 = -nx * sa + nz * ca;
                        let ny2 = ny * cb - nz1 * sb;
                        let nz2 = ny * sb + nz1 * cb;

                        let light = (ny2 - nz2) / 2.0_f64.sqrt();

                        let brightness = ((light + 1.0) * 0.5 * (chars.len() - 1) as f64) as usize;

                        screen[idx] = chars[brightness.min(chars.len() - 1)];
                    }
                }

                phi += 0.04;
            }

            theta += 0.02;
        }

        execute!(out, cursor::MoveTo(0, 0)).unwrap();

        for row in screen.chunks(w) {
            let line: String = row.iter().map(|&c| c as char).collect();
            writeln!(out, "{line}").unwrap();
        }

        out.flush().unwrap();

        a += 0.04;
        b += 0.02;

        thread::sleep(Duration::from_millis(30));
    }
}
