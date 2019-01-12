use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn difference(word1: &str, word2: &str) -> usize {
    return word1.len()
        - word1
            .chars()
            .zip(word2.chars())
            .filter(|(c1, c2)| c1 == c2)
            .count();
}

fn remove_differing(word1: &str, word2: &str) -> String {
    let mut result = String::new();
    word1.chars().zip(word2.chars()).for_each(|(c1, c2)| {
        if c1 == c2 {
            result.push(c1);
        }
    });
    return result;
}

fn find_similar_stupid(path: &str) -> Result<(String, String), std::io::Error> {
    let f = File::open(path)?;
    let r = BufReader::new(f);
    let orig_ids: Vec<String> = r.lines().map(|line| line.unwrap()).collect();

    for i in 0..orig_ids.len() - 1 {
        for j in 0..orig_ids.len() {
            if difference(&orig_ids[i], &orig_ids[j]) == 1 {
                return Ok((orig_ids[i].to_string(), orig_ids[j].to_string()));
            }
        }
    }

    return Ok(("".to_string(), "".to_string()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_2_2() {
        let closely_matching = find_similar_stupid("src/res/day_2.txt").unwrap();
        assert_eq!(
            "rmyxgdlihczskunpfijqcebtv".to_owned(),
            remove_differing(&closely_matching.0, &closely_matching.1)
        );
    }
}

fn main() {
    let closely_matching = find_similar_stupid("src/res/day_2.txt").unwrap();

    println!("{}, {}", &closely_matching.0, &closely_matching.1);

    println!(
        "{}",
        remove_differing(&closely_matching.0, &closely_matching.1)
    );
}
