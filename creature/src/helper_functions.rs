///Converts a Option<&str> to a Option<String> returning None if input is None of &str is ""
pub fn option_str_to_option_string(input: Option<&str>) -> Option<String>{
    match input {
        Some(str) => str_to_option_string(str),
        None => None,
    }
}

///Converts a &str to a Option<String> returning None if input is ""
pub fn str_to_option_string(input: &str) -> Option<String>{
    match input{
        "" => None,
        str => Some(str.to_string())
    }
}