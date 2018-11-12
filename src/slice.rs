pub fn first_word(s: &String) -> &str {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                // string slice and the range is non-inclusive
                // inclusive syntax would look like:
                // return &s[0..=(i-1)]
                return &s[0..i];
            }
        }

        // This just return the entire string because there was no space character found
        &s[..]
}
