use std::collections::HashMap;
use std::collections::hash_map::Entry::Occupied;
use std::panic::catch_unwind;
use std::sync::Mutex;

use sequences::Sequence;

type SequenceHandle = u32;
type SequenceErrorCode = u8;

lazy_static! {
    static ref IDSBYNAME: Mutex<HashMap<String, usize>> = {
        Mutex::new(HashMap::new())
    };

    static ref SEQSBYID: Mutex<Vec<Box<Sequence>>> = {
        Mutex::new(Vec::new())
    };
}

#[no_mangle]
pub extern fn parse(s: &str) -> SequenceErrorCode {
    let mut ids = IDSBYNAME.lock().unwrap();
    let mut sequences = SEQSBYID.lock().unwrap();
    ::parse_assignments(s, &mut *ids, &mut *sequences);

    0
}

#[no_mangle]
pub fn lookup(name: &str, handle_ptr: *mut SequenceHandle) -> SequenceErrorCode {
    if handle_ptr.is_null() {
        return 2
    }

    let mut ids = IDSBYNAME.lock().unwrap();

    if let Occupied(entry) = ids.entry(name.into()) {
        let id = *entry.get() as SequenceHandle;

        unsafe {
            *handle_ptr = id;
        };

        0
    } else {
        1
    }
}

#[no_mangle]
pub fn next(handle: SequenceHandle, result_ptr: *mut u32) -> SequenceErrorCode {
    if result_ptr.is_null() {
        return 2
    }

    let mut sequences = SEQSBYID.lock().unwrap();

    let idx = handle as usize;
    if sequences.is_empty() || idx > sequences.len() - 1 {
        1
    } else {
        let value = sequences[idx].next();
        unsafe { *result_ptr = value; };

        0
    }
}

#[cfg(test)]
mod tests {
    mod parse {
        use super::super::*;
        use std::collections::hash_map::Entry::Occupied;

        #[test]
        fn basic() {
            assert!(IDSBYNAME.lock().unwrap().is_empty());
            assert!(SEQSBYID.lock().unwrap().is_empty());

            let error = parse("a=5;");
            assert_eq!(error, 0);

            let mut ids = IDSBYNAME.lock().unwrap();
            let mut sequences = SEQSBYID.lock().unwrap();
            assert!(ids.contains_key("a"));
            if let Occupied(entry) = ids.entry("a".into()) {
                let id = entry.get();
                let value = sequences[*id].next();
                assert_eq!(value, 5);
            }

            ids.clear();
            sequences.clear();
        }

        #[test]
        fn range() {
            assert!(IDSBYNAME.lock().unwrap().is_empty());
            assert!(SEQSBYID.lock().unwrap().is_empty());

            let error = parse("a=[0,1];");
            assert_eq!(error, 0);

            let mut ids = IDSBYNAME.lock().unwrap();
            let mut sequences = SEQSBYID.lock().unwrap();
            assert!(ids.contains_key("a"));
            if let Occupied(entry) = ids.entry("a".into()) {
                let id = entry.get();
                let value = sequences[*id].next();
                assert!(value == 0 || value == 1);
            }

            ids.clear();
            sequences.clear();
        }
    }

    mod lookup {
        use super::super::*;
        use std::ptr;

        #[test]
        fn not_found() {
            assert!(IDSBYNAME.lock().unwrap().is_empty());
            assert!(SEQSBYID.lock().unwrap().is_empty());

            let mut handle: SequenceHandle = 0;
            let handle_ptr: *mut SequenceHandle = &mut handle;
            assert_eq!(lookup("a", handle_ptr), 1);
        }

        #[test]
        fn found() {
            assert!(IDSBYNAME.lock().unwrap().is_empty());
            assert!(SEQSBYID.lock().unwrap().is_empty());

            let error = parse("a=5;");
            assert_eq!(error, 0);

            let mut handle: SequenceHandle = 0;
            let handle_ptr: *mut SequenceHandle = &mut handle;
            assert_eq!(lookup("a", handle_ptr), 0);

            IDSBYNAME.lock().unwrap().clear();
            SEQSBYID.lock().unwrap().clear();
        }

        #[test]
        fn null_handle() {
            let handle_ptr: *mut SequenceHandle = ptr::null_mut();
            let error = lookup("a", handle_ptr);
            assert_eq!(error, 2);
        }
    }

    mod next {
        use super::super::*;
        use std::ptr;

        #[test]
        fn found() {
            assert!(IDSBYNAME.lock().unwrap().is_empty());
            assert!(SEQSBYID.lock().unwrap().is_empty());

            let error = parse("a=5;");
            assert_eq!(error, 0);

            let mut handle: SequenceHandle = 0;
            let handle_ptr: *mut SequenceHandle = &mut handle;
            let error = lookup("a", handle_ptr);
            assert_eq!(error, 0);

            let mut result: u32 = 0;
            let result_ptr: *mut u32 = &mut result;
            let error = next(handle, result_ptr);
            assert_eq!(error, 0);
            assert_eq!(result, 5);

            IDSBYNAME.lock().unwrap().clear();
            SEQSBYID.lock().unwrap().clear();
        }

        #[test]
        fn not_found() {
            assert!(IDSBYNAME.lock().unwrap().is_empty());
            assert!(SEQSBYID.lock().unwrap().is_empty());

            let handle = 0;
            let mut result: u32 = 0;
            let result_ptr: *mut u32 = &mut result;
            let error = next(handle, result_ptr);
            assert_eq!(error, 1);
            assert_eq!(result, 0);
        }

        #[test]
        fn null_result() {
            let handle = 0;
            let result_ptr: *mut u32 = ptr::null_mut();
            let error = next(handle, result_ptr);
            assert_eq!(error, 2);
        }
    }
}
