
pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let row_len = if let Some(row) = minefield.get(0) {
        row.len()
    } else {
        return vec![];
    };
    let num_rows = minefield.len();
    let mut result = vec![vec![(0, false); row_len]; num_rows];
    let star = '*' as u8;
    for (y, row) in minefield.iter().enumerate() {
        for (x, val) in row.as_bytes().iter().enumerate() {
            if *val == star {
                let lower_bound_x = x.saturating_sub(1);
                let upper_bound_x = if x + 1 == row_len { x } else { x + 1 };
                let lower_bound_y = y.saturating_sub(1);
                let upper_bound_y = if y + 1 == num_rows { y } else { y + 1 };
                for xs in lower_bound_x..=upper_bound_x {
                    for ys in lower_bound_y..=upper_bound_y {
                        result[ys][xs].0 += 1;
                    }
                }
                result[y][x].1 = true; 
            }
        }
    }
    let result: Vec<String> = result
        .iter_mut()
        .map(|row: &mut Vec<(i32, bool)>| -> String {
            let row: String = row
                .iter()
                .map(|val| match val {
                    (_, true) => "*".to_string(),
                    (0, _) => " ".to_string(),
                    (num, _) => num.to_string(),
                })
                .collect();
            row
        })
        .collect();
    result
}


fn main() {
    let minefield = &[" * * ", "  *  ", "  *  ", "     "];
    annotate(minefield);
} 