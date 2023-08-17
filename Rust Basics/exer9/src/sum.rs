pub fn sum_loop_index(vector: &Vec<i64>) -> i64 {
    let mut sum = 0;
    for i in 0..vector.len() {
        sum += vector[i]
    }

    return sum;
}

pub fn sum_loop_iter(vector: &Vec<i64>) -> i64 {
    let mut sum = 0;
    for value in vector {
        sum += value
    }

    return sum;
}

pub fn sum_fold(vector: &Vec<i64>) -> i64 {
    return vector.iter().fold(0, |acc, x| acc + x);
}

pub fn sum_method(vector: &Vec<i64>) -> i64 {
    return vector.iter().sum();
}
