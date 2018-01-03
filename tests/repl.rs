/// Verifies a REPL can be implemented for rvs.
///
/// More specifically, it verifies state can be accumulated over multiple parse and transform
/// calls.
extern crate rvs;

#[test]
fn basic() {
    let search_path: rvs::SearchPath = Default::default();
    let seed = Default::default();

    let mut transform = rvs::Transform::new(seed);
    let mut model = rvs::Model::new();

    let mut parser = rvs::Parser::new(&search_path);
    parser.parse("a = Pattern(1, 2, 3, 4);").unwrap();
    transform.transform(&mut model, parser.ast()).unwrap();
    {
        let a = model.get_most_recently_added().unwrap();
        assert_eq!(a.borrow_mut().next(), 1);
    }

    let mut parser = rvs::Parser::new(&search_path);
    parser.parse("b = a;").unwrap();
    transform.transform(&mut model, parser.ast()).unwrap();
    {
        let b = model.get_most_recently_added().unwrap();
        assert_eq!(b.borrow_mut().next(), 2);
    }

    let mut parser = rvs::Parser::new(&search_path);
    parser.parse("a = Pattern(5, 6, 7, 8);").unwrap();
    transform.transform(&mut model, parser.ast()).unwrap();
    {
        let a = model.get_most_recently_added().unwrap();
        assert_eq!(a.borrow_mut().next(), 5);

        let b = model.get_variable_by_name("b").unwrap();
        // FIXME?: b points to previous a which has been dropped.  The weak pointer can no longer
        // be upgraded.  The previous value will be returned.
        assert_eq!(b.borrow_mut().next(), 2);
    }
}
