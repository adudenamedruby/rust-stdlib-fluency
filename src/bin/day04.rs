// ## Day 4 - Arrays, slices, `Vec`, and borrowing collections
//
// - [ ] Focus: contiguous data and borrowed views.
//   - Detail: Most day-to-day Rust data processing uses contiguous memory somewhere: arrays, slices, or vectors. The main idea is to learn when a function needs ownership and when a borrowed slice is the more flexible API.
// - [ ] Read/inspect:
//   - What to look for: Look for which methods live on `Vec<T>` versus slices. Notice how slice methods are often more broadly useful because arrays and vectors can both be borrowed as slices.
//   - `std::vec::Vec`
//   - primitive slice docs
// - [ ] Exercise:
//   - Goal: The point is to practice taking borrowed views over collections and using built-in slice operations. These exercises make you choose when mutation, sorting, allocation, and safe indexing are appropriate.
//   - Implement:
//     - `median(numbers: &mut [i32]) -> Option<f64>`
//     - `dedup_sorted(numbers: &mut Vec<i32>)`
//     - `window_sums(numbers: &[i32], window: usize) -> Vec<i32>`
//   - Use `sort`, `windows`, `chunks`, `split_at`, and indexing safely.
//   - Decisions to make: for even-length `median`, do you average the two middles or pick one? For `window_sums`, what should happen when `window` is `0` or larger than the slice (panic, empty `Vec`, `Option`)?
// - [ ] Done when:
//   - You can explain why function parameters should often be `&[T]` instead of `&Vec<T>`.

fn median(numbers: &mut [i32]) -> Option<f64> {
    numbers.sort();
    if numbers.len().is_multiple_of(2) {
        let index = numbers.len() / 2;
        Some(((numbers[index] + numbers[index - 1]) as f64) / 2.0)
    } else {
        Some(numbers[numbers.len() / 2] as f64)
    }
}

fn main() {
    _ = median(&mut [1, 2, 3]);
}

#[cfg(test)]
mod day04_tests {
    use super::*;

    #[test]
    fn median_get_middle_number() {
        assert_eq!(Some(2.0), median(&mut [3, 1, 2]));
        assert_eq!(Some(3.0), median(&mut [5, 4, 2, 3, 1]));
    }

    #[test]
    fn median_get_two_middle_numbers() {
        assert_eq!(Some(2.0), median(&mut [3, 2, 1, 2]));
        assert_eq!(Some(2.5), median(&mut [3, 3, 5, 1, 2, 1]));
    }
}
