use std::collections::HashMap;
fn pig_latin(input: &str) -> String {
    // проверяем, является ли символ гласной
    fn is_vowel(c: char) -> bool {
        "aeiouAEIOU".contains(c)}
    let mut word_map: HashMap<char, Vec<String>> = HashMap::new();
    for word in input.split_whitespace() {
        if let Some(first_char) = word.chars().next() {word_map.entry(first_char)
                .or_insert_with(Vec::new)
                .push(word.to_string());}
    }
    // преобразуем слова в Pig Latin
    let mut result = Vec::new();
    for (key, words) in word_map {
        for word in words {
            let pig_latin_word = if is_vowel(key) {
                format!("{}-hay", word)} else {
                let mut chars = word.chars();
                let first_char = chars.next().unwrap();
                let rest: String = chars.collect();
                format!("{}-{}ay", rest, first_char)};
            result.push(pig_latin_word);}}

    result.join(" ") // собираем строку
}
fn main() {
    let input = "random suggestion for an example";
    let output = pig_latin(input);
    println!("Pig Latin: {}", output);
}