use std::collections::HashMap;

fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut hash_map: HashMap<&str, u8> = HashMap::new();
    for i in magazine {
        let counter = hash_map.entry(i).or_insert(0);
        *counter += 1;
    }

    for item in note {
        let counter = hash_map.entry(item).or_insert(0);
        if  *counter == 0{
            return false;
        }
        *counter -= 1;
    }
    return true;
}

fn main() {
    let magazine = "Astronomer Amy Mainzer spent hours chatting with Leonardo DiCaprio for Netflix's 'Don't Look Up'".split_whitespace().collect::<Vec<&str>>();
    let note ="Amy Mainzer chatting with Leonardo DiCaprio".split_whitespace().collect::<Vec<&str>>();
    println!("Can I construct {}", can_construct_note(&magazine, &note))
    
}
