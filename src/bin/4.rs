fn main() {
    let mut count = 0;
    let mut double;
    'outer: for num in 246515..=739105 {
        double = false;
        let s = num.to_string();
        for (a, b) in pairwise(&s) {
            if a == b {
                double = true;
            }
            else if a > b {
                continue 'outer;
            }
        }
        if double {
            count += 1;
        }
    }
    println!("{}", count);

    count = 0;
    let mut in_run;
    let mut too_long;
    'outer: for num in 246515..=739105 {
        double = false;
        in_run = false;
        too_long = false;
        let s = num.to_string();
        for (a, b) in pairwise(&s) {
            if a == b {
                if !double {
                    if in_run {
                        too_long = true;
                    }
                    else {
                        in_run = true;
                    }
                }
            }
            else if a > b {
                continue 'outer;
            }
            else {
                if in_run & !too_long {
                    double = true;
                }
                in_run = false;
                too_long = false;
            }
        }
        if double | (in_run & !too_long) {
            count += 1;
        }
    }
    println!("{}", count);
}

fn pairwise(s: &str) -> impl Iterator<Item = (char, char)> + '_ {
    let a = s.chars();
    let mut b = s.chars();
    b.next();
    a.zip(b)
}
