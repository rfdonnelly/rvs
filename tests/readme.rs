extern crate rvs;

use std::env::current_dir;
use std::collections::HashMap;

use rvs::Context;

#[test]
fn readme() {
    let mut context = Context::new();
    context.search_path.add(&current_dir().unwrap());
    rvs::parse("import 'examples/readme.rvs';", &mut context).unwrap();
    rvs::transform(&mut context).unwrap();

    {
        let pattern = context.get("pattern").unwrap();
        let expected: Vec<u32> = vec![2, 0, 1, 0, 2, 0, 1, 0];
        let mut actual: Vec<u32> = Vec::new();
        for _ in 0..8 {
            actual.push(pattern.borrow_mut().next());
        }
        assert_eq!(expected, actual);
    }

    {
        let sample = context.get("sample").unwrap();
        let mut results: HashMap<u32, u32> = HashMap::new();
        for _ in 0..90 {
            let result = sample.borrow_mut().next();
            let entry = results.entry(result).or_insert(0);
            *entry += 1;;
        }
        assert_eq!(results.len(), 3);
        for i in 0..3 {
            assert!(results[&i] >= 30 - 5 && results[&i] <= 30 + 5);
        }
    }

    {
        let weighted = context.get("weighted").unwrap();
        let mut results: HashMap<u32, u32> = HashMap::new();
        for _ in 0..1000 {
            let result = weighted.borrow_mut().next();
            let entry = results.entry(result).or_insert(0);
            *entry += 1;;
        }
        println!("weighted: {:?}", results);
        assert_eq!(results.len(), 3);
        assert!(results[&0] >= 400 - 50 && results[&0] <= 400 + 50);
        assert!(results[&1] >= 500 - 50 && results[&1] <= 500 + 50);
        assert!(results[&2] >= 100 - 50 && results[&2] <= 100 + 50);
    }
}
