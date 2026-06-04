use std::fs;
use std::io;
use std::collections::HashMap;

fn main() -> io::Result<()>{
    let mut word_finder: HashMap<String,usize> = HashMap::new();


    let contents = fs::read_to_string("input.txt")?;

    let uppercase = contents.to_uppercase();

    let characters = contents.len();
    let words = contents.split_whitespace().count();
    let lines = contents.lines().count();

    for word in contents.split_whitespace() {
        *word_finder.entry(word.to_string()).or_insert(0) += 1;
    };

    let most_used_word = word_finder
        .iter()
        .max_by_key(|(_, count)| *count);

    let most_used_word = match most_used_word {
        Some((word,count)) => format!("{} ({})", word, count),
        None => "No words found".to_string(),
    };


    let output = format!(
        "{}\nCharacters: {}\nWords: {}\nLines: {}\nMost Used Word: {}",
        uppercase,
        characters,
        words,
        lines,
        most_used_word
    );

    fs::write("output.txt", output)?;


    println!("File written successfully!");

    Ok(())

}
