#[cfg_attr(test, derive(Debug))]
#[derive(PartialEq)]
enum Entry {

}

#[cfg_attr(test, derive(Debug))]
#[derive(PartialEq)]
enum Error {
    EmptyFunctionNames,
    EmptyLineEnding,
}

/// Looks in a specified data for one of specified function names.
/// If it find some function name it checks that a first argument, which is string,
/// ends with a specified line ending sequence.
/// # Parameters
/// * *data* - data to check
/// * *function_names* - function names to find
/// * *line_ending* - a line ending to check
fn find_missing_line_endings(data: &str,
                             function_names: &[&str],
                             line_ending: &str)
                             -> Result<Vec<Entry>, Error> {
    if function_names.is_empty() {
        return Err(Error::EmptyFunctionNames);
    }

    if line_ending.is_empty() {
        return Err(Error::EmptyLineEnding);
    }

    Ok(Vec::new())
}

fn main() {}

#[cfg(test)]
mod tests {
    mod find_missing_line_endings {
        use ::{Error, find_missing_line_endings};

        #[test]
        fn returns_error_if_no_function_names() {
            let actual = find_missing_line_endings("", &[], "\n");

            let expected = Err(Error::EmptyFunctionNames);

            assert_eq!(actual, expected);
        }

        #[test]
        fn returns_error_if_line_ending_is_empty() {
            let actual = find_missing_line_endings("", &["foo"], "");

            let expected = Err(Error::EmptyLineEnding);

            assert_eq!(actual, expected);
        }

        #[test]
        fn returns_empty_vec_if_data_is_empty() {
            let actual = find_missing_line_endings("", &["foo"], "\n");

            let expected = Ok(Vec::new());

            assert_eq!(actual, expected);
        }

        #[test]
        fn returns_entities_if_data_contains_missing_line_endings() {
            let data = r#""#;

            let actual = find_missing_line_endings()
        }
    }
}