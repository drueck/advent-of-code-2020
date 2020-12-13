struct SeatScheme {
    upper_half_char: char,
    lower_half_char: char,
    min: usize,
    max: usize,
}

const ROW_SCHEME: SeatScheme = SeatScheme {
    lower_half_char: 'F',
    upper_half_char: 'B',
    min: 0,
    max: 127,
};

const COLUMN_SCHEME: SeatScheme = SeatScheme {
    lower_half_char: 'L',
    upper_half_char: 'R',
    min: 0,
    max: 7,
};

pub fn seat_id(seat_code: &str) -> usize {
    let row_code = &seat_code[..7];
    let column_code = &seat_code[7..];

    let row = find_index(row_code, &ROW_SCHEME);
    let column = find_index(column_code, &COLUMN_SCHEME);

    return row * 8 + column;
}

fn find_index(code: &str, scheme: &SeatScheme) -> usize {
    let mut min = scheme.min;
    let mut max = scheme.max;
    let lower_half_char = scheme.lower_half_char;
    let upper_half_char = scheme.upper_half_char;

    for letter in code.chars() {
        if min == max {
            break;
        }

        let offset = (max - min + 1) / 2;
        if letter == lower_half_char {
            max -= offset;
        } else if letter == upper_half_char {
            min += offset;
        }
    }

    return min;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seat_id() {
        assert_eq!(seat_id("FBFBBFFRLR"), 357);
        assert_eq!(seat_id("BFFFBBFRRR"), 567);
        assert_eq!(seat_id("FFFBBBFRRR"), 119);
        assert_eq!(seat_id("BBFFBBFRLL"), 820);
    }
}
