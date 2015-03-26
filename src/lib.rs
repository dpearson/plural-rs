static NUNINFLECTED: [&'static str; 44] = ["bison", "flounder", "pliers", "bream", "gallows", "proceedings", "breeches", "graffiti", "rabies", "britches", "headquarters", "salmon", "carp", "herpes", "scissors", "chassis", "high-jinks", "sea-bass", "clippers", "homework", "series", "cod", "innings", "shears", "contretemps", "jackanapes", "species", "corps", "mackerel", "swine", "debris", "measles", "trout", "diabetes", "mews", "tuna", "djinn", "mumps", "whiting", "eland", "news", "wildebeest", "elk", "pincers"];

pub fn uninflected(noun: &str) -> bool {
	NUNINFLECTED.iter().find(|&&n| n == noun).is_some()
}

pub fn pl_noun(noun: &str) -> String {
	if noun.ends_with("fish") ||
		noun.ends_with("ois") ||
		noun.ends_with("sheep") ||
		noun.ends_with("deer") ||
		noun.ends_with("pos") ||
		noun.ends_with("itis") || /* TODO: Check [A-Z].*ese */
		uninflected(noun) {
		// Return the unchanged noun
		return noun.to_string();
	}

	// Handle pronouns of all shapes and sizes
	let res: &str = match noun {
		"I" => "we",
		"you" => "you",
		"thou" => "you",
		"she" => "they",
		"he" => "they",
		"it" => "they",
		"they" => "they",
		"me" => "us",
		"thee" => "you",
		"her" => "them",
		"him" => "them",
		//"it" => "them",
		"them" => "them",
		"myself" => "ourselves",
		"yourself" => "yourself",
		"thyself" => "yourself",
		"herself" => "themselves",
		"himself" => "themselves",
		"itself" => "themselves",
		"themself" => "themselves",
		"oneself" => "oneselves",
		"mine" => "ours",
		"yours" => "yours",
		"thine" => "yours",
		"hers" => "theirs",
		"his" => "theirs",
		"its" => "theirs",
		"theirs" => "theirs",
		_ => ""
	};

	// Return the appropriate plural pronoun if we have one
	if !res.eq("") {
		return res.to_string();
	}

	// TODO: Handle prepositional phrases

	// TODO: Handle standard irregular nouns

	// TODO: Handle standard irregular inflections for common nouns

	// TODO: Handle standard irregular nouns

	// TODO: Handle classical inflections

	// TODO: Handle suffixes that take -es

	// TODO: Handle words ending in -f and -fe

	// TODO: Handle words ending in -y and -ys

	// TODO: Handle words ending in -o

	// TODO: Handle compound words

	let mut res = noun.to_string();
	res.push_str("s");

	return res;
}

#[test]
fn test1() {
	assert_eq!(pl_noun("fish"), "fish".to_string());
	assert_eq!(pl_noun("I"), "we".to_string());
}
