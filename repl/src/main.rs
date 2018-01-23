extern crate rvs;

use std::io;
use std::io::prelude::*;

fn main() {
    let search_path = Default::default();
    let seed = Default::default();

    let mut transform = rvs::Transform::new(seed);
    let mut model = rvs::Model::new();

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let s = read_statement().unwrap();
        if s.is_empty() {
            return;
        }

        let mut parser = rvs::Parser::new(&search_path);
        if let Err(e) = eval(&s, &mut parser, &mut transform, &mut model) {
            println!("error: {}", e);
        }
    }
}

fn eval(
    s: &str,
    parser: &mut rvs::Parser,
    transform: &mut rvs::Transform,
    model: &mut rvs::Model,
) -> rvs::Result<()> {
    parser.parse(s)?;
    transform.transform(model, parser.ast())?;

    let rv = model.get_most_recently_added().unwrap();
    let mut rv = rv.borrow_mut();

    let values: Vec<String> = vec![(0, false); 15]
        .iter()
        .map(|_| (rv.next(), rv.done()))
        .map(|(next, done)| {
            if done {
                format!("0x{:x} <done>", next)
            } else {
                format!("0x{:x}", next)
            }
        })
        .collect();

    let values = values.join(", ");
    println!("=> {}", values);

    Ok(())
}

fn read_statement() -> Result<String, io::Error> {
    let mut bytes: Vec<u8> = Vec::new();
    let stdin = io::stdin();

    // FIXME: Read until ';' OR blank line (for enums)
    stdin.lock().read_until(';' as u8, &mut bytes)?;

    Ok(String::from_utf8(bytes).unwrap())
}
