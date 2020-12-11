mod map;
mod ride;

use common::input;
use map::Map;
use ride::Ride;

fn main() {
    let map: Map = input::read_lines().into();

    let ride = Ride::create(map.clone(), 3, 1);
    println!("[PART ONE] number of trees encountered: {}", ride.ride());

    let slopes: Vec<(usize, usize)> = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let rides = slopes.iter().map(|x| {
        let (change_x, change_y) = x;
        Ride::create(map.clone(), *change_x, *change_y).ride()
    });

    println!(
        "[PART TWO] product of tree encounters: {}",
        rides.product::<u32>()
    );
}

#[cfg(test)]
mod tests {
    use super::{Map, Ride};

    fn generate_map() -> Map {
        let input = vec![
            "..##.......",
            "#...#...#..",
            ".#....#..#.",
            "..#.#...#.#",
            ".#...##..#.",
            "..#.##.....",
            ".#.#.#....#",
            ".#........#",
            "#.##...#...",
            "#...##....#",
            ".#..#...#.#",
        ];

        input
            .into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .into()
    }

    #[test]
    fn part_one() {
        let ride = Ride::create(generate_map(), 3, 1);
        assert_eq!(ride.ride(), 7);
    }

    #[test]
    fn part_two() {
        let map: Map = generate_map();
        let slopes: Vec<(usize, usize)> = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
        let rides = slopes.iter().map(|x| {
            let (change_x, change_y) = x;
            Ride::create(map.clone(), *change_x, *change_y).ride()
        });
        assert_eq!(rides.product::<u32>(), 336);
    }
}
