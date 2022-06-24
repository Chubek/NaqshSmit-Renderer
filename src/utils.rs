pub fn swap<T: Clone>(these: &mut [&mut T], with_these: &mut [&mut T]) {
    for i in 0..these.len() {
        let temp = these[i].clone();
        *these[i] = with_these[i].clone();
        *with_these[i] = temp;
    }
}
