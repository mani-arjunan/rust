#[derive(Debug, PartialEq, Eq)]
enum CustomError {
    NotAllowed,
}

fn main() {
    let names: Vec<String> = vec![String::from("xxx"), String::from("yyy")];

    fn to_upper_case(str: &String) -> Result<String, CustomError> {
        if str == "" {
            return Err(CustomError::NotAllowed);
        }

        return Ok(str.to_uppercase().to_string());
    }

    fn convert_to_upper_case_1(names: Vec<String>) -> Vec<Result<String, CustomError>> {
        names.iter().map(|w| to_upper_case(w)).collect()
    }

    fn convert_to_upper_case_2(names: Vec<String>) -> Result<Vec<String>, CustomError> {
        names.iter().map(|w| to_upper_case(w)).collect()
    }

    println!("{:?}", convert_to_upper_case_1(names.clone()));
    println!("{:?}",convert_to_upper_case_2(names));
}
