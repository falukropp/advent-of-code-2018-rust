use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

type GenError = Box<std::error::Error>;

fn read_stars_from_file(path: &str) -> Result<Vec<Vec<i32>>, GenError> {
    let f = File::open(path)?;
    let r = BufReader::new(f);

    let mut result = Vec::new();

    for line_result in r.lines() {
        let line = line_result?;

        let coords: Vec<i32> = line.split(",").map(|s| s.parse().unwrap()).collect();
        result.push(coords);
    }
    Ok(result)
}

fn is_neighbours(star1: &Vec<i32>, star2: &Vec<i32>) -> bool {
    let manhattan_distance = star1
        .iter()
        .zip(star2.iter())
        .fold(0, |s, (c1, c2)| s + (c1 - c2).abs());
    manhattan_distance <= 3
}

fn find_constellations(stars: &Vec<Vec<i32>>) -> Vec<usize> {
    let mut constellations = Vec::with_capacity(stars.len());
    for (i, s) in stars.iter().enumerate() {
        let mut member_of_constellation: Option<usize> = None;
        for j in 0..i {
            if is_neighbours(&s, &stars[j]) {
                // println!(
                //     "{} and {} is neighbours, the neigbour was in constallation {}",
                //     i, j, constellations[j]
                // );
                if let Some(constellation) = member_of_constellation {
                    // Neighbour with several stars. If they're the same constellation, nothing more to do.
                    if constellation != constellations[j] {
                        let other_constellation = constellations[j];
                        // println!(
                        //     "Should merge constallation {} into {}, constellations was {:?}, i was {}",
                        //     constellations[j], constellation, constellations, i
                        // );
                        // ... but if they're different, merge them!
                        for k in 0..i {
                            // println!("k is {}, constellation[k] is {}, ", k, constellations[k]);
                            if constellations[k] == other_constellation {
                                constellations[k] = constellation;
                                // println!(" constellations[k] is now {}", constellations[k]);
                            }
                        }
                        // println!("Constallation is now {:?} ", constellations);
                    }
                } else {
                    member_of_constellation = Some(constellations[j]);
                }
            }
        }
        if let Some(constellation) = member_of_constellation {
            constellations.push(constellation);
        } else {
            constellations.push(i);
        }
    }
    constellations
}

fn count_constellations(path: &str) -> usize {
    let stars = read_stars_from_file(path).unwrap();
    println!("{:?}", stars);
    let mut constellations = find_constellations(&stars);
    println!("{:?}", constellations);
    constellations.sort();
    constellations.dedup();
    constellations.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_constellations() {
        assert_eq!(4, count_constellations("src/res/day_25_ex_1.txt"));
        assert_eq!(3, count_constellations("src/res/day_25_ex_2.txt"));
        assert_eq!(377, count_constellations("src/res/day_25.txt"));
    }
}

fn main() {
    println!(
        "day_25 : number of constellations : {}",
        count_constellations("src/res/day_25.txt")
    );
}
