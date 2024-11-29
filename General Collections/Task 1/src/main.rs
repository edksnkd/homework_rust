// fn main() {
//     let a: i32;
//
//     {
//         a = 1;
//     }
//     {
//
//     }
//     println!("{}", a);
// }
use std::collections::HashMap;
fn median(num: &Vec<i32>) -> i32 {
    let count = num.len();
    let rez = if count % 2 == 0 {num[count/2]} else {(num[count/2]+num[count/2-1])/2};
    rez
}


fn moda(numbers: &Vec<i32>)  -> i32  {
    let mut counts = HashMap::new();
    let mut max_count = 0;
    let mut mode = 0;
    for number in numbers {
        *counts.entry(number).or_insert(0) += 1;
    }


    for (number, count) in counts.iter() {
        if *count > max_count {
            max_count = *count;
            mode = **number
        }
    }
    7
}

fn main() {
    let mut numbers = vec![4, -5, 1, -3, 2];
    numbers.sort();
    let medians = median(&numbers);
    let mode = moda(&numbers);
    println! ("median , moda {}, {}", medians, mode);
}
