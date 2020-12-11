mod seat;

use common::input;
use seat::Seat;

fn main() {
    let seats: Vec<Seat> = input::read_lines()
        .iter()
        .map(|x| x.clone().into())
        .collect();

    let ids = seats.iter().map(|x| x.get_id()).collect::<Vec<u16>>();

    let mut sorted = ids.clone();
    sorted.sort_unstable();

    println!("[PART ONE] highest seat ID: {}", sorted.pop().unwrap());

    let mut possible = Vec::with_capacity(1024);

    for row in 1..126 {
        for column in 0..7 {
            possible.push(Seat::new(row, column));
        }
    }

    possible.retain(|x| !seats.contains(x));

    let mut possible_ids: Vec<u16> = possible.iter().map(|x| x.get_id()).collect();

    possible_ids.retain(|x| (ids.contains(&(x + 1)) && ids.contains(&(x - 1))));

    println!("[PART ONE] seat ID: {}", possible_ids.pop().unwrap());
}

#[cfg(test)]
mod tests {
    use super::Seat;

    #[test]
    fn part_one() {
        assert_eq!(Seat::from("FBFBBFFRLR".to_string()).get_id(), 357);
        assert_eq!(Seat::from("BFFFBBFRRR".to_string()).get_id(), 567);
        assert_eq!(Seat::from("FFFBBBFRRR".to_string()).get_id(), 119);
        assert_eq!(Seat::from("BBFFBBFRLL".to_string()).get_id(), 820);
    }
}
