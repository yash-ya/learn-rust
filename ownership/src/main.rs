use std::cmp::max;
use std::i32::MIN;
fn main() {
    let mut a = 10;
    let mut b = 20;
    swap_number(&mut a, &mut b);
    println!("{} {}", a, b);

    let nums = [10, 20, 30, 40];
    let average = calculate_average(&nums);
    println!("{}", average);

    let max = maximum_number(&nums);
    println!("{}", max);

    let word = "education";
    let vowel_count = count_vowels(word);
    println!("{}", vowel_count);
}

fn swap_number(a: &mut i32, b: &mut i32) {
    let temp = *a;
    *a = *b;
    *b = temp;
}

fn calculate_average(nums: &[i32]) -> f64 {
    let sum: i32 = nums.iter().sum();
    let count: f64 = nums.len() as f64;
    f64::from(sum) / count
}

fn maximum_number(nums: &[i32]) -> i32 {
    let mut max_number = MIN;
    for i in nums {
        max_number = max(max_number, *i);
    }
    max_number
}

fn count_vowels(word: &str) -> usize {
    let mut count = 0;
    for i in word.chars() {
        if "aeiouAEIOU".contains(i) {
            count += 1;
        }
    }

    word.chars().filter(|c| "aeiouAEIOU".contains(*c)).count();

    count
}
