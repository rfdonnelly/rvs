extern crate rvs;

use std::io;
use std::io::prelude::*;

use rvs::types::Context;
use rvs::parse_rvs;

fn main() {
    let mut context = Context::new();
    let mut idx = 0;

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let s = read_statement().unwrap();
        if let Err(e) = parse_rvs(&s, &mut context) {
            println!("error: {}", e);
        }

        for rv in context.variables[idx..].iter_mut() {
            let values: Vec<String> = vec![(0, false); 15]
                .iter()
                .map(|_| (rv.next(), rv.done()))
                .map(|(next, done)| if done {
                         format!("{} <done>", next)
                    } else {
                         format!("{}", next)
                    })
                .collect();
            let values = values.join(", ");

            println!("=> {}", values);
        }

        idx = context.variables.len();
    }
}

fn read_statement() -> Result<String, io::Error> {
    let mut bytes: Vec<u8> = Vec::new();
    let stdin = io::stdin();

    // FIXME: Read until ';' OR blank line (for enums)
    stdin.lock().read_until(';' as u8, &mut bytes)?;

    Ok(String::from_utf8(bytes).unwrap())
}
