fn drain_filter(name: &str, list: &mut Vec<String>) {
    let mut i = 0;
    while i < list.len() {
        if list[i] == name || list[i] == name.chars().rev().collect::<String>() {
            list.remove(i);
            i = 0;
        }
        i += 1;
    }
}

fn sort_string(name: String) -> String {
    let mut tmp_vec = name.chars().collect::<Vec<char>>();
    tmp_vec.sort();
    tmp_vec.iter().collect::<String>()
}

fn is_new_word_present(temp_str: String, list: &Vec<String>) -> Vec<String> {
    let mut new_list: Vec<String> = Vec::new();
    let mut _temp_sort_list = list.clone();
    for item in _temp_sort_list {
        new_list.push(sort_string(item));
    }
    let mut new_words:Vec<String> = Vec::new();
    for (index, name) in new_list.iter().enumerate() {
        if &temp_str == name {
            new_words.push(list[index].clone());
        }
    }
    new_words
}

fn main() {
    // let name = "hello";
    // let name = "liSTen";
    let name = "thRoW";
    let mut _list = vec!["enLIsts", "GOogle", "inlETs", "banana", "hello", "olleh", "wOrth"];
    // convert all in lowercase
    let mut lower_case_list = _list.iter().map(|&item| item.to_lowercase()).collect::<Vec<String>>();
    // remove same word or reversed of it
    drain_filter(name, &mut lower_case_list);
    // sort string for comparison
    let name_lowered = name.to_lowercase();
    let tmp_str = sort_string(name_lowered);
    let list = is_new_word_present(tmp_str, &lower_case_list);
    println!("{list:?}");
}
