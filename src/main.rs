use std::io;
fn go(mut g: Vec<i8>, p: bool) {
    let mut success = false;
    while !success {
        disp(g.clone(), p);
        if check(g.clone(), 0) {
            print!("\n{} wins!", if p { 'x' } else { 'o' });
            return;
        }
        let n = reader() - 1;
        let mut m = 1;
        if p {
            m -= 2;
        };
        if g[n] == 0 {
            g[n] += m;
            success = true;
        } else {
            println! {"space already taken\n"}
        }
    }
    go(g, !p)
}

fn reader() -> usize {
    let mut success = false;
    let mut out = 0;
    while !success {
        let mut input_text = String::new();
        io::stdin()
            .read_line(&mut input_text)
            .expect("failed to read from stdin");

        let trimmed = input_text.trim();
        match trimmed.clone().parse::<usize>() {
            Ok(i) => {
                if i < 10 {
                    out = i;
                    success = true;
                } else {
                    println!("select from 1-9")
                }
            }
            Err(..) => {
                println!("this was not an integer: {}", trimmed);
            }
        };
    }
    out
}
fn disp(g: Vec<i8>, p: bool) {
    println!();
    let mut j = 1;
    for i in g.iter().enumerate() {
        let out = x_o(i.1);
        if j < 3 {
            print! {"{} ", out};
        } else {
            println! {"{} ",out};
            j = 0;
        }
        j += 1;
    }
    println!("\n{}:", if p { 'o' } else { 'x' });
}
fn x_o(i: &i8) -> char {
    match i {
        1 => 'x',
        -1 => 'o',
        0 => '_',
        _ => 'E',
    }
}

fn check(g: Vec<i8>, i: i8) -> bool {
    match i {
        0 => {
            if (g[0] + g[1] + g[2]).abs() == 3 {
                true
            } else {
                check(g, i + 1)
            }
        }
        1 => {
            if (g[3] + g[4] + g[5]).abs() == 3 {
                true
            } else {
                check(g, i + 1)
            }
        }
        2 => {
            if (g[6] + g[7] + g[8]).abs() == 3 {
                true
            } else {
                check(g, i + 1)
            }
        }
        3 => {
            if (g[0] + g[3] + g[6]).abs() == 3 {
                true
            } else {
                check(g, i + 1)
            }
        }
        4 => {
            if (g[1] + g[4] + g[7]).abs() == 3 {
                true
            } else {
                check(g, i + 1)
            }
        }
        5 => {
            if (g[2] + g[5] + g[8]).abs() == 3 {
                true
            } else {
                check(g, i + 1)
            }
        }
        6 => {
            if (g[0] + g[4] + g[8]).abs() == 3 {
                true
            } else {
                check(g, i + 1)
            }
        }
        7 => {
            if (g[2] + g[4] + g[7]).abs() == 3 {
                true
            } else {
                check(g, i + 1)
            }
        }
        _ => false,
    }
}
fn main() {
    println!("\nRust tic-tac-toe, select positions like from a keypad(1-9)");
    go(vec![0i8; 9], true);
}
