
fn long_word (word: &str) -> String {
    let allWords: Vec<&str> = word.split(' ').collect();
    let mut longest = allWords[0];
    for w in allWords {
        if w.len() > longest.len() {
            longest = w;
        }
    }
    longest.to_string()
}
fn main() {
    let sentence = "the quick brown fox jumps over the verylazy dog".to_string();
    // Use slicing to get the first three characters of the sentence
    //println!("{}", &sentence[0..=4]);

    // concatenate using format!
    let description = format!("Title: Quick story\n{}", sentence);
    //println!("{}", description);

    // iterate over the characters in the sentence
    let mut vowel = 0;
    for c in sentence.chars() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => vowel += 1,
            _ => continue,
        }
    }
    println!("Vowels: {}", vowel);
    // Split and collect into a vector
    //let words: Vec<&str> = sentence.split_whitespace().collect();
    let words = sentence.split(' ').collect::<Vec<&str>>();
    println!("{:?}", words);

    // let reversed = sentence.chars().rev().collect::<String>();
    // println!("{}", reversed);

    let longestWord = long_word(&sentence);
    println!("LongetsWord: {}", longestWord);
}