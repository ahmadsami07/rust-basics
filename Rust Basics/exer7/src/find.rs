pub fn find_elt<T: Eq>(values: &Vec<T>, elt: T) -> Option<usize> {
    // TODO

    for (i, v) in values.iter().enumerate() {
        if *v == elt {
            return Some(i);
        }
    }
    return None;
}
