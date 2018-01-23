extern crate rvs;

use std::env::current_dir;
use std::collections::HashMap;

#[test]
fn readme() {
    let model = rvs::parse(
        rvs::SearchPath::new(vec![current_dir().unwrap()]),
        "import examples::readme;",
    ).unwrap();

    let pattern = model.get_variable_by_name("pattern").unwrap();
    let mut pattern = pattern.borrow_mut();
    let expected: Vec<u32> = vec![2, 0, 1, 0, 2, 0, 1, 0];
    let actual: Vec<u32> = (0..expected.len()).map(|_| pattern.next()).collect();
    assert_eq!(expected, actual);

    let sample = model.get_variable_by_name("sample").unwrap();
    let mut sample = sample.borrow_mut();
    let mut results: HashMap<u32, u32> = HashMap::new();
    for _ in 0..90 {
        let entry = results.entry(sample.next()).or_insert(0);
        *entry += 1;;
    }
    assert_eq!(results.len(), 3);
    for i in 0..3 {
        assert!(results[&i] >= 30 - 5 && results[&i] <= 30 + 5);
    }

    let weighted = model.get_variable_by_name("weighted").unwrap();
    let mut weighted = weighted.borrow_mut();
    let mut results: HashMap<u32, u32> = HashMap::new();
    for _ in 0..1000 {
        let entry = results.entry(weighted.next()).or_insert(0);
        *entry += 1;;
    }
    println!("weighted: {:?}", results);
    assert_eq!(results.len(), 3);
    assert!(results[&0] >= 400 - 50 && results[&0] <= 400 + 50);
    assert!(results[&1] >= 500 - 50 && results[&1] <= 500 + 50);
    assert!(results[&2] >= 100 - 50 && results[&2] <= 100 + 50);
}
