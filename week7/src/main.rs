fn main() {
    // Get the user input
    let mut input = String::new();
    println!("Enter a word:");
    std::io::stdin().read_line(&mut input).unwrap();

    // Remove whitespace and convert to lowercase
    let word = input.trim().to_lowercase();

    // Check if the word is a palindrome
    if is_palindrome(&word) {
        println!("{} is a palindrome!", word);
    } else {
        println!("{} is not a palindrome.", word);
    }
}

fn is_palindrome(word: &str) -> bool {
    let mut chars: Vec<char> = word.chars().collect();
    let mut i = 0;
    let mut j = chars.len() - 1;

    while i < j {
        if chars[i] != chars[j] {
            return false;
        }
        i += 1;
        j -= 1;
    }

    true
}
