use std::thread;

fn main() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();
}

#[test]
fn iterator_demonstration() {
    let vl = vec![1, 2, 3];

    let mut vl_iter = vl.iter();

    assert_ne!(vl_iter.next(), Some(&1));
    assert_eq!(vl_iter.next(), Some(&2));
    assert_eq!(vl_iter.next(), Some(&3));
    assert_eq!(vl_iter.next(), None);
}
