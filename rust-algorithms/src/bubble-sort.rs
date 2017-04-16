extern crate rand;

use rand::distributions::{IndependentSample, Range};

// Create a vector of random integers of size n
fn create_random_vector(n: i32) -> Vec<i32> {
    let mut myvec = Vec::new();
    let range = Range::new(0, 1000);
    let mut rng = rand::thread_rng();

    for _ in 0..n {
        myvec.push(range.ind_sample(&mut rng));
    }
    myvec
}

fn sort(myvec: &mut Vec<i32>) {
    let mut tmp: i32;

    for _ in 0..myvec.len() - 1 {
        for j in 0..myvec.len() - 2 {
            if myvec[j] > myvec[j + 1] {
                tmp = myvec[j];
                myvec[j] = myvec[j + 1];
                myvec[j + 1] = tmp;
            }
        }
    }
    return;
}

fn main() {
    println!("Bubble sort!");
    let mut myvec = create_random_vector(10);
    println!("Sorting {:?}", myvec);

    sort(&mut myvec);

    println!("Sorted {:?}", myvec);
}
