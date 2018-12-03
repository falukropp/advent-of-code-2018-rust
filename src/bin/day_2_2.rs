use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

// fn rotate_words(words: &mut Vec<String>) {
//     for i in 0..words.len() {
//         let mut rotated_word = String::new();
//         rotated_word += &words[i][1..];
//         rotated_word += &words[i][0..1];
//         words[i] = rotated_word;
//     }
// }

fn difference(word1: &str, word2: &str) -> usize {
    return word1.len() - word1
        .chars()
        .zip(word2.chars())
        .filter(|(c1, c2)| c1 == c2)
        .count();
}

// fn find_similar(path: &str) -> Result<(String, String), std::io::Error> {
//     let f = File::open(path)?;
//     let r = BufReader::new(f);
//     let orig_ids: Vec<String> = r.lines().map(|line| line.unwrap()).collect();
//     let mut ids = orig_ids.clone();

//     for _ in 0..ids[0].len() {
//         ids.sort();
//         for i in 0..ids.len() - 1 {
//             if difference(&ids[i], &ids[i + 1]) == 1 {
//                 println!(
//                     "difference {} for {} {} position {}",
//                     difference(&ids[i], &ids[i + 1]),
//                     ids[i],
//                     ids[i + 1],
//                     i
//                 );
//                 // This isn't correct. Must unrotate first... bleh. do it stupid instead.
//                 return Ok((orig_ids[i].to_string(), orig_ids[i + 1].to_string()));
//             }
//         }
//         rotate_words(&mut ids);
//     }

//     return Ok(("".to_string(), "".to_string()));
// }

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

fn main() {
    // let mut closely_matching = find_similar("src/res/day_2.txt").unwrap();

    // println!(
    //     "difference {}",
    //     difference("rmsxgdlshczskenpfwbjqoeatv", "rmbxgdlihcmskgnpfwbjqoeatv"),
    // );

    // println!("{}, {}", &closely_matching.0, &closely_matching.1);

    // println!(
    //     "{}",
    //     remove_differing(&closely_matching.0, &closely_matching.1)
    // );

    let closely_matching = find_similar_stupid("src/res/day_2.txt").unwrap();

    println!("{}, {}", &closely_matching.0, &closely_matching.1);

    println!(
        "{}",
        remove_differing(&closely_matching.0, &closely_matching.1)
    );
}
