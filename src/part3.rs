/*
    CIS198 Homework 1
    Part 3: Ownership, move semantics, and lifetimes

    Complete and write at least one unit test for each function you implement.
    If it already has a unit test, either add assertions to it or add a new one.
    Also answer the questions in text.
*/

// Remove these once you are done editing the file!
#![allow(dead_code)]
// #![allow(unused_variables)]

/*
    Problem 1: Swap ints

    Implement the function that swaps two integers, and write unit tests.

    Make sure you write a test for when x1 and x2 are the same!

    The Rust borrow checker may help avoid some possible bugs.
*/
pub fn swap_ints(x1: &mut i32, x2: &mut i32) {
    let temp = *x2;
    *x2 = *x1;
    *x1 = temp;
}

#[test]
fn test_swap_ints() {
    let mut x = 1;
    let mut y = 2;
    swap_ints(&mut x, &mut y);
    assert_eq!(x, 2);
    assert_eq!(y, 1);
    // swap_ints(&mut x, &mut x);
    // assert_eq!(x, 2);
}

/*
    Problem 2: String duplication
*/
#[test]
fn copy_string_test() {
    let str1 = String::from("foo");
    let str2 = String::from(&str1);
    assert_eq!(str1, str2);
}
// This test doesn't work. Fix it by copying strings properly.
// Q1. What went wrong?

// Q2. How come it works fine here?
#[test]
fn copy_int_test() {
    let i1 = 1;
    let i2 = i1;
    assert_eq!(i1, i2);
}

// Now implement the following function that duplicates a string n times.
fn duplicate_string(s: &str, times: usize) -> Vec<String> {
    let mut res = Vec::new();
    for _ in 0..times {
        res.push(String::from(s));
    }
    res
}

/*
    Problem 3: String duplication continued

    These two don't work either. Fix by changing the type of "string" in the
    function copy_me ONLY, and by adjusting the parameter to "copy_me" where
    it's called.
*/

fn copy_me(string: /* Change in here only*/ &str) -> String {
    string.to_string()
}

#[test]
fn copy_me_test() {
    let str1 = String::from("foo");
    assert_eq!(str1, copy_me(/* Change in here only*/ &str1));
}

#[test]
fn copy_me_test2() {
    let str1 = String::from("foo");
    let str2 = copy_me(&str1 /* Change in here only*/);
    assert_eq!(str1, str2);
}

/*
    Problem 4: Lifetime specifiers

    For each of the following three functions, either implement it by adding
    lifetime specifiers, or explain why this is not possible.

    (It's not truly impossible -- we will see later on that advanced features
    such as "unsafe code" can be used to turn off Rust's safety and lifetime
    checks.)
*/
// fn new_ref_string() -> &'static String {
//     // const STATIC_STR: &'static String = &String::from("Hi");
//     // STATIC_STR
// }

fn new_ref_str() -> &'static str {
    "hi"
}

// The same function from part2
fn pick_longest2<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() >= s2.len() {
        s1
    } else {
        s2
    }
}

/*
    Problem 5: Using functions with lifetimes

    Write two versions of a function which returns the longest string in a
    vector, using pick_longest2 as a helper function.

    You can add lifetime specifiers if necessary.
    What are the pros and cons of v1 and v2?
*/

fn pick_longest_in_v1(v: Vec<String>) -> String {
    let mut longest = String::from("");
    for s in v {
        let curr_longest = &longest;
        longest = pick_longest2(curr_longest, &s).to_string();
    }
    longest
}

fn pick_longest_in_v2(v: Vec<&str>) -> &str {
    let mut longest : &str = "";
    for s in v {
        let curr_longest = longest;
        longest = pick_longest2(curr_longest, s);
    }
    longest
}

#[test]
fn test_pick_longest_in_v1() {
    assert_eq!(
        pick_longest_in_v1(vec!["cat".to_string(), "dog".to_string(), "cockroach".to_string()]),
        "cockroach".to_string()
    );
}

#[test]
fn test_pick_longest_in_v2() {
    assert_eq!(
        pick_longest_in_v2(vec!["cat", "dog", "cockroach"]),
        "cockroach"
    );
    assert_eq!(
        pick_longest_in_v2(vec![]), ""
    );
}

/*
    Problem 6: Move semantics

    Write three versions of a function that pads a vector with zeros.
    Fail if the vector is larger than the desired length.

    Use .clone() as necessary to make the unit tests compile.

    Which of these functions do you prefer? Which is the most efficient?
*/

fn pad_with_zeros_v1(v: Vec<usize>, desired_len: usize) -> Vec<usize> {
    assert!(v.len() <= desired_len);
    let pad_len : usize = desired_len - v.len();
    let pad = vec![0; pad_len];
    let result = [v, pad].concat();
    debug_assert_eq!(result.len(), desired_len);
    result
}

fn pad_with_zeros_v2(slice: &[usize], desired_len: usize) -> Vec<usize> {
    assert!(slice.len() <= desired_len);
    let pad_len : usize = desired_len - slice.len();
    let mut pad = Vec::new();
    pad.resize(pad_len, 0);
    let result = [slice, &pad].concat();
    debug_assert_eq!(result.len(), desired_len);
    result
}

fn pad_with_zeros_v3(v: &mut Vec<usize>, desired_len: usize) {
    assert!(v.len() <= desired_len);
    v.resize(desired_len, 0);
    debug_assert_eq!(v.len(), desired_len);
}

#[test]
fn test_pad_twice_v1() {
    let v = vec![1];
    let v = pad_with_zeros_v1(v, 2);
    let v = pad_with_zeros_v1(v, 4);
    assert_eq!(v, vec![1, 0, 0, 0]);
}

#[test]
fn test_pad_twice_v2() {
    let v = vec![1];
    let v = pad_with_zeros_v2(&v, 2);
    let v = pad_with_zeros_v2(&v, 4);
    assert_eq!(v, vec![1, 0, 0, 0]);
}

#[test]
fn test_pad_twice_v3() {
    let mut v = vec![1];
    pad_with_zeros_v3(&mut v, 2);
    pad_with_zeros_v3(&mut v, 4);
    assert_eq!(v, vec![1, 0, 0, 0]);
}

/*
    Problem 7: Move semantics continued

    Write a function which appends a row to a vector of vectors.
    Notice that it takes ownership over the row.
    You shouldn't need to use .clone().

    Why is this more general than being passed a &[bool]
    and cloning it?

    Second, write a function which returns whether
    a row equals the first row in the vector of vectors.
    Notice that it does not take ownership over the row.

    Why is this more general than being passed a Vec<bool>?
*/

fn append_row(grid: &mut Vec<Vec<bool>>, row: Vec<bool>) {
    grid.push(row)
}

fn is_first_row(grid: &[Vec<bool>], row: &[bool]) -> bool {
    // Check if row is the first row in grid
    // Remember to handle the case when grid is empty
    if grid.is_empty() {
        false
    } else {
        grid[0] == row
    }
}

/*
    Problem 8: Modifying while iterating

    In C and C++, you run into subtle bugs if you try to modify a data
    structure while iterating over it. Rust's move semantics prevents that.
*/

use std::collections::HashMap;

// To familiarize yourself with HashMaps,
// implement the following function which converts pairs from a slice
// into key-value pairs in a hashmap.
// Documentation:
// https://doc.rust-lang.org/std/collections/struct.HashMap.html

fn vector_to_hashmap(v: &[(i32, String)]) -> HashMap<i32, String> {
    let mut res = HashMap::new();
    for (k, s) in v {
        res.insert(*k, s.clone());
    }
    res
}

// Now rewrite this function to delete all entries in hashmap where the keys
// are negative.
fn delete_negative_keys(h: &mut HashMap<i32, i32>) {
    // This fails, uncomment to see error.
    let keys : Vec<i32> = h.keys().copied().filter(|k| k < &0).collect();
    for k in keys {
        h.remove(&k);
    }
}

#[test]
fn test_delete_negative_keys() {
    let mut v = HashMap::new();
    v.insert(-1, 2);
    v.insert(2, 2);
    delete_negative_keys(&mut v);
    assert_eq!(v.len(), 1);
}

/*
    Problem 9: The Entry API

    Move semantics present interesting API design choices not found in other
    languages.
    HashMap is an example of such a API.
    Specifically, the Entry API:
    https://doc.rust-lang.org/std/collections/hash_map/enum.Entry.html

    This allows for efficient HashMap access because we only access
    the entry in the map (computing an expensive hash function) once.

    Implement a function which does the following:
        For all entries in `add`: (k, v)
        If `k` exists in `merged`, append `v` to the value of `merged[k]`.
        If that `k` doesn't exist in `merged`, add the (k, v) to `merged`.
    Use `or_insert` and `and_modify`.
*/

fn merge_maps(
    merged: &mut HashMap<String, String>,
    add: HashMap<String,String>
) {
    for (k, v) in add {
        merged.entry(k)
            .and_modify(|e| e.push_str(&v))
            .or_insert(v);
    }
}
