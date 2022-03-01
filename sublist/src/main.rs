use std::cmp::Ordering;


fn check_list_status(l1: &[u32], l2: &[u32]) -> bool {
    let split_time = l1.len() - l2.len();
    for i in 0..=split_time {
        let (_, splited) = l1.split_at(i);
        let mut temp_list = splited.windows(l2.len());
        if l2 == temp_list.next().unwrap() {
            return true;
        }
    }
    false
}

fn compare_list(a: &[u32], b: &[u32]) {

    match a.len().cmp(&b.len()) {
        Ordering::Equal => {
            if a == b {
                println!("A is equal to b");
            } else {
                println!("A is not equal to b");
            }
        },
        Ordering::Less => {
            let status = check_list_status(b, a);
            if status {
                println!("A is sublist of b");
            } else {
                println!("A is not a superlist of, sublist of or equal to B")
            }
        },
        Ordering::Greater => {
            let status = check_list_status(a, b);
            if status {
                println!("A is a superlist of B");
            } else {
                println!("B is not a superlist of, sublist of or equal to A")
            }
        },
    }

}

fn main() {
    let B = [2,3,4];
    let A = [1,2,3,4,5];
    compare_list(&A, &B);
}