/// returns : possible k-digit non-decreasing numbers, starting with : xxxxxx (k digits)
fn re(x: i32, k: i32) -> i32 {
    let mut result = 0;
    for i in 1..x + 1 {
        result = result + if k == 0 { i } else { re(i, k - 1) };
    }
    result
}

/// returns nCr
fn no_dups(n: i32, r: i32) -> i32 {
    if n < r {
        return 0;
    }
    let (mut nr, mut dr) = (1, 1);
    for i in 0..r {
        dr = dr * (r - i);
        nr = nr * (n - i);
    }
    nr / dr
}

/// returns the next possible password, starting from `start`
/// i.e., given start = 254032, returns 255555
fn get_first_password(start: i32) -> i32 {
    let mut max = -1;
    let mut position = -1;
    let mut multiplier = -1;
    let mut t = start;
    let mut i = 0;
    let mut mul = 1;
    while t > 0 {
        if t % 10 > max {
            max = t % 10;
            position = i;
            multiplier = mul;
        }
        i = i + 1;
        mul = mul * 10;
        t = t / 10;
    }
    let mut init = start / multiplier;
    while position > 0 {
        init = init * 10 + max;
        position = position - 1;
    }
    init
}

/// splits the least significant repeating digits
/// e.g. given n = 1_357_777, returns 1_366_666 and 7_777
fn split_repeating(n: i32) -> (i32, i32) {
    let lsd = n % 10;
    let mut sec = 0;
    let mut t = n;
    let mut mul = 1;
    let mut adder = 0;
    while t % 10 == lsd {
        sec = sec * 10 + lsd;
        adder = adder * 10 + 1;
        mul = mul * 10;
        t = t / 10;
    }
    if t != 0 {
        t = (t + 1) * mul + ((t + 1) % 10) * adder;
    }
    (t, sec)
}

fn password_count_for_repeating(mut t: i32) -> i32 {
    let x = t % 10;
    let mut count = 0;
    while t > 0 {
        t = t / 10;
        count = count + 1;
    }
    re(10 - x, count - 2) - no_dups(10 - x, count)
}

fn password_count(range_start: i32) -> i32 {
    let start = get_first_password(range_start);
    let (a, b) = split_repeating(start);

    password_count_for_repeating(b) + if a == 0 { 0 } else { password_count(a) }
}

fn main() {
    const START: i32 = 254032;
    const END: i32 = 789860;

    let larger_than_start = password_count(START);
    let larger_than_end = password_count(END);

    println!("Result = {}", larger_than_start - larger_than_end);
}
