use std::sync::Mutex;

static ARRAY: Mutex<Vec<i32>> = Mutex::new(Vec::new());

pub fn do_a_call() {
    ARRAY.lock().unwrap().push(1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        do_a_call();
        do_a_call();
        do_a_call();

        println!("Called {} times", ARRAY.lock().unwrap().len());
    }
}
