/*
    CIS198 Homework 1
    Part 1: Implementing functions

    Complete and write at least one unit test for each function you implement.
    If it already has a unit test, either add assertions to it or add a new one.
    Also answer the questions in text.
*/

// Remove these once you are done editing the file!
// This will result in useful warnings if you missed something.
#![allow(dead_code)]
#![allow(unused_variables)]

/*
    Problem 1: Double

    Implement the function that doubles an integer in three different ways.

    What are some differences between them? Can you write unit tests
    which fail (or fail to compile) for some but not others?

    Which of the three do you prefer?
*/

pub fn double_v1(n: i32) -> i32 {
    n * 2
}

pub fn double_v2(n: &i32) -> i32 {
    n * 2
}

pub fn double_v3(n: &mut i32) {
    // double n in place
    *n *= 2;
}

// Example unit test (so you can recall the syntax)
#[test]
fn test_double_v1() {
    assert_eq!(double_v1(2), 4);
    assert_eq!(double_v1(-3), -6);
}

#[test]
fn test_double_v2() {
    let a1 = 2;
    let a2 = 4;
    assert_eq!(double_v2(&a1), a2);
    let b1 = -3;
    let b2 = -6;
    assert_eq!(double_v2(&b1), b2);
    assert_eq!(double_v2(&a1), a2);
}

#[test]
fn test_double_v3() {
    let mut a1 = 2;
    let a2 = 4;
    double_v3(&mut a1);
    assert_eq!(a1, a2);
    let mut b1 = -3;
    let b2 = -6;
    double_v3(&mut b1);
    assert_eq!(b1, b2);
    double_v3(&mut a1);
    assert_eq!(a1, 8);
}

/*
    Problem 2: Integer square root

    Implement the integer square root function: sqrt(n) should return the
    largest m such that m * m <= n. For a 'harder' version, try to do it more
    efficiently than trying every possibility.
*/
pub fn sqrt(n: usize) -> usize {
    let mut res = 1;
    let end = n/2 + 1;
    for i in (1..end).rev() {
        if i * i <= n {
            res = i;
            break;
        }
    }
    res
}

// Remember to write unit tests here (and on all future functions)

#[test]
fn test_sqrt() {
    assert_eq!(sqrt(4), 2);
    assert_eq!(sqrt(8), 2);
    assert_eq!(sqrt(10), 3);
}
/*
    Problem 3: Slice sum

    Implement the sum function on slices in two different ways
    (using different for loop patterns).
    Do not use the predefined sum function.
    Also, try to do it without an unnecessary `return` statement at the end --
    Clippy should detect if you mess this up.

    Which of the two ways do you prefer?
*/
pub fn sum_v1(slice: &[i32]) -> i32 {
    let mut acc = 0;
    for &v in slice {
        acc += v;
    }
    acc
}

pub fn sum_v2(slice: &[i32]) -> i32 {
    let mut acc = 0;
    for v in slice {
        acc += v;
    }
    acc
}

#[test]
fn test_sum_v1() {
    let a = [42, 8, 3, 2, 5];
    assert_eq!(sum_v1(&a[..5]), 60);
    assert_eq!(sum_v1(&a[..4]), 55);
    assert_eq!(sum_v1(&a[1..4]), 13);
}

#[test]
fn test_sum_v2() {
    let a = [42, 8, 3, 2, 5];
    assert_eq!(sum_v2(&a[..5]), 60);
    assert_eq!(sum_v2(&a[..4]), 55);
    assert_eq!(sum_v2(&a[1..4]), 13);
}

/*
    Problem 4: Unique

    Make unique. Create a new vector which contains each item in the vector
    only once! Much like a set would.
    This doesn't need to be efficient; you can use a for loop.
*/

pub fn unique(slice: &[i32]) -> Vec<i32> {
    let mut v_slice = slice.to_vec();
    v_slice.sort_unstable();
    let mut unique_vec = Vec::new();
    // Is there a better way to do it?
    if !v_slice.is_empty() {
        let mut prev = v_slice[0];
        for x in v_slice {
            if x != prev {
                unique_vec.push(prev);
                prev = x;
            } 
        }
        unique_vec.push(prev);
    }
    unique_vec
}

#[test]
fn test_unique() {
    let a = [42, 8, 3, 2, 5, 8, 42, 5];
    assert_eq!(unique(&a), [2, 3, 5, 8, 42]);
}


/*
    Problem 5: Filter

    Return a new vector containing only elements that satisfy `pred`.
    This uses some unfamiliar syntax for the type of pred -- all you need
    to know is that pred is a function from i32 to bool.
*/
pub fn filter(slice: &[i32], pred: impl Fn(i32) -> bool) -> Vec<i32> {
    let mut res_vec = Vec::new();
    for x in slice {
        if pred(*x) {
            res_vec.push(*x);
        } 
    }
    res_vec
}

#[test]
fn test_filter() {
    fn is_even(n: i32) -> bool {
        n % 2 == 0
    }
    assert_eq!(filter(&vec![1, 2, 3, 4, 5, 6], &is_even), vec![2, 4, 6]);
}

/*
    Problem 6: Fibonacci

    Given starting fibonacci numbers n1 and n2, compute a vector of
    length 'out_size'
    where v[i] is the ith fibonacci number.
*/
pub fn fibonacci(n1: i32, n2: i32, out_size: usize) -> Vec<i32> {
    let mut vec = Vec::new();
    let mut f1 = n1;
    let mut f2 = n2;
    let mut temp;
    for i in 0..out_size {
        vec.push(f1);
        temp = f1;
        f1 = f2;
        f2 += temp;
    }
    vec
}

#[test]
fn test_fibonacci() {
    let res = [1, 1, 2, 3, 5, 8, 13, 21, 34];
    assert_eq!(fibonacci(1, 1, 9), res);
    assert_eq!(fibonacci(1, 1, 0), &res[..0]);
}

/*
    Problem 7: String concatenation

    Create a function which concats 2 &strs and returns a String,
    and a function which concats 2 Strings and returns a String.

    You may use any standard library function you wish.

    What are some reasons the second function is not efficient?
*/
pub fn str_concat(s1: &str, s2: &str) -> String {
    let mut res = String::from(s1);
    res.push_str(s2);
    res
}

pub fn string_concat(s1: String, s2: String) -> String {
    str_concat(&s1, &s2)
}

#[test]
fn test_str_concat() {
    let a1 : &str = "hello, ";
    let a2 : &str = "world!";
    assert_eq!(str_concat(a1, a2), "hello, world!");
}

#[test]
fn test_string_concat() {
    let a1 = String::from("hello, ");
    let a2 = String::from("world!");
    assert_eq!(string_concat(a1, a2), "hello, world!");
}

/*
    Problem 8: String concatenation continued

    Convert a Vec<String> into a String.
    Your answer to the previous part may help.
*/

pub fn concat_all(v: Vec<String>) -> String {
    let mut acc = String::new();
    for string in v {
        acc = string_concat(acc, string);
    }
    acc
}

#[test]
fn test_concat_all() {
    let a1 = String::from("hello, ");
    let a2 = String::from("world! ");
    let a3 = String::from("How is life?");
    let vec = vec![a1, a2, a3];
    assert_eq!(concat_all(vec), "hello, world! How is life?");
}

/*
    Problem 9: Parsing

    Convert a Vec<String> into a Vec<i32> and vice versa.

    Assume all strings are correct numbers! We will do error handling later.
    Use `.expect("ignoring error")` to ignore Result from parse()
    See https://doc.rust-lang.org/std/primitive.str.html#method.parse

    The unit tests check if your functions are inverses of each other.

    A useful macro: format! is like println! but returns a String.
*/

pub fn parse_all(v: Vec<String>) -> Vec<i32> {
    let mut res = Vec::with_capacity(v.len());
    for string in v {
        res.push(string.parse().expect("no errors here!"));
    }
    res
}

pub fn print_all(v: Vec<i32>) -> Vec<String> {
    let mut res = Vec::with_capacity(v.len());
    for i in v {
        res.push(format!("{}", i));
    }
    res
}

#[test]
fn test_print_parse() {
    assert_eq!(parse_all(print_all(vec![1, 2])), vec![1, 2]);
}

#[test]
fn test_parse_print() {
    let v = vec!["1".to_string(), "2".to_string()];
    assert_eq!(print_all(parse_all(v.clone())), v);
}

/*
    Problem 10: Composing functions

    Implement a function which concatenates the even Fibonacci
    numbers out of the first n Fibonacci numbers.

    For example: if n = 6, the first 5 Fibonacci numbers are 1, 1, 2, 3, 5, 8,
    so the function should return the String "28".

    Don't use a for loop! Your previous functions should be sufficient.
*/

pub fn concat_even_fibonaccis(n: usize) -> String {
    fn is_even(n: i32) -> bool {
        n % 2 == 0
    }
    let fibs = fibonacci(1, 1, n);
    let filtered_fibs = filter(&fibs, &is_even);
    let string_fibs = print_all(filtered_fibs);
    concat_all(string_fibs)
}

#[test]
fn test_concat_even_fibonaccis() {
    assert_eq!(&concat_even_fibonaccis(6), "28");
    assert_eq!(&concat_even_fibonaccis(9), "2834");
}
