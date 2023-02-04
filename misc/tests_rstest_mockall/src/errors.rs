use anyhow::anyhow;
use anyhow::Context;
#[allow(dead_code)]
#[allow(unused_imports)]
use thiserror::Error;
#[derive(Debug, Clone, Error, PartialEq)]
enum ErrorDb {
    #[error("cannot initialized database")]
    DbInitError,
    #[error("cannot execute command")]
    DbExecutionError,
}

fn return_something(testarg: i32) -> anyhow::Result<()> {
    if testarg == 0 {
        anyhow::bail!(ErrorDb::DbInitError)
    } else {
        anyhow::bail!(ErrorDb::DbExecutionError)
    }
}

fn process_with_context(arg: i32) -> anyhow::Result<()> {
    return_something(arg).with_context(|| format!("there is some content mesage here"))?;
    Ok(())
}
#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;
    #[rstest]
    #[case(0, ErrorDb::DbInitError)]
    #[case(1, ErrorDb::DbExecutionError)]
    #[should_panic]
    #[case(2, ErrorDb::DbInitError)]
    fn test_error_downcasting(#[case] input: i32, #[case] expect: ErrorDb) {
        let result = return_something(input);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .downcast_ref::<ErrorDb>()
            .is_some_and(|e| *e == expect));
    }

    #[rstest]
    fn test_err_context() {
        let res = process_with_context(1);
        assert!(res.is_err());
        if let Err(e) = res {
            println!("context message: {e:?}");
        }
    }
}
