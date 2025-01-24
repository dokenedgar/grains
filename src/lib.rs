pub fn square(s: u32) -> u64 {
    if s == 1 {
        1
    } else {
        let mut counter = 0;
        let mut result = 1;
        loop {
            if counter < s - 1 {
                result *= 2;
                counter += 1;
            } else {
                break;
            }
        }
        result
    }
}

pub fn total() -> u64 {
    let mut total_grains: u64 = 0;
    let mut counter = 1;
    loop {
        if counter <= 64 {
            total_grains += square(counter);
            counter += 1;
        } else {
            break;
        }
    }
    total_grains
}
