use rvs;

use std::io;
use std::rc::Rc;
use std::cell::RefCell;

pub fn expr_to_var<S>(expr: S) -> rvs::Result<Rc<RefCell<Box<rvs::Variable>>>>
where
    S: AsRef<str>,
{
    let s = format!("variable = {};", expr.as_ref());
    let model = rvs::parse(Default::default(), &s)?;
    match model.get_variable_by_name("variable") {
        Some(variable) => Ok(Rc::clone(variable)),
        None => Err(rvs::Error::Io(io::Error::new(
            io::ErrorKind::NotFound,
            "cannot find 'variable' in model",
        ))),
    }
}
