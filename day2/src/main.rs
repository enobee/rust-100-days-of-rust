fn find_nemo(sentence :&str) {
    let words = sentence.split_whitespace();
    let mut found = false;
    for (i,word )in words.enumerate() {
        let position = i + 1;
        let cleaned_word = word.trim_matches(|c: char| !c.is_alphanumeric());
        if cleaned_word == "Nemo" {
            println!("I found Nemo at {position}!");
            found = true;
            break;
        } 
    }

    if found == false {
        println!("I can't find Nemo :(");
    }
}

fn main() {
    find_nemo("I am finding Nemo !");
    find_nemo("Nemo is me");
    find_nemo("I Nemo am");
    find_nemo("I have Nemo's Nemo");
    find_nemo("men Nemo Nemo");
    find_nemo("i love you");
}
