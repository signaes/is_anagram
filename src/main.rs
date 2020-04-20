use std::collections::HashMap;

fn main() {
    println!("is_anagram(\"\", \"\") {}", is_anagram("", ""));
    println!("is_anagram(\"aaz\", \"zza\") {}", is_anagram("aaz", "zza"));
    println!("is_anagram(\"anagram\", \"nagaram\") {}", is_anagram("anagram", "nagaram"));
    println!("is_anagram(\"rat\", \"car\") {}", is_anagram("rat", "car"));
    println!("is_anagram(\"awesome\", \"awesom\") {}", is_anagram("awesome", "awesom"));
    println!("is_anagram(\"amanaplanacanalpanama\", \"acanalmanplanpamana\") {}", is_anagram("amanaplanacanalpanama", "acanalmanplanpamana"));
    println!("is_anagram(\"qwerty\", \"qeywrt\") {}", is_anagram("qwerty", "qeywrt"));
    println!("is_anagram(\"texttwisttime\", \"timetwisttext\") {}", is_anagram("texttwisttime", "timetwisttext"));
}

fn is_anagram(a: &str, b: &str) -> bool {
    if a.chars().count() != b.chars().count() {
        return false;
    }

    let mut a_map: HashMap<String, usize> = HashMap::new();
    let mut b_map: HashMap<String, usize> = HashMap::new();

    for c in a.chars() {
        *a_map.entry(c.to_string()).or_insert(0) += 1;
    }

    for b in b.chars() {
        *b_map.entry(b.to_string()).or_insert(0) += 1;
    }

    for (c, count) in &a_map {
        if b_map.get(&c.to_string()) == None || b_map.get(&c.to_string()).unwrap() != count {
            return false;
        }
    }

    true
}
