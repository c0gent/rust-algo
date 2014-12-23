
pub fn test() {
	let pangram: &'static str = "the quick brown fox jumps over the lazy dog";

	println!("Words");
    for word in pangram.words() {
        println!("> {}", word);
    }

	println!("Words in reverse");
    for word in pangram.words().rev() {
        println!("> {}", word);
    }
}
