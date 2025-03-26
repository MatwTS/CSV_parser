mod csv_parser {
    use nom::{
        bytes::complete::is_not,
        character::complete::{char, newline},
        multi::separated_list1,
        IResult,
    };

    #[derive(Debug, PartialEq)]
    pub enum ParseError {
        ParseIntError(std::num::ParseIntError),
        IndexOutOfBounds,
        CsvParseError,
    }

    impl From<std::num::ParseIntError> for ParseError {
        fn from(err: std::num::ParseIntError) -> Self {
            ParseError::ParseIntError(err)
        }
    }

    impl From<()> for ParseError {
        fn from(_: ()) -> Self {
            ParseError::CsvParseError
        }
    }

    ///
    ///                      parse_csv
    ///---------------------------------------------------------
    ///Input: CSV file as a string (full string).
    ///Examples of input:
    ///- "Alex,M,41,74,170\nBert,M,42,68,166\nCarl,F,32,70,155\n"
    ///
    ///Output: `Result<Vec<Vec<String>>, String>`
    ///- Ok(Vec<Vec<String>>): Each line of the CSV file is transformed into a vector of fields (a vector of vectors).
    ///  Example output:
    ///  Ok(vec![
    ///      vec!["Alex", "M", "41", "74", "170"],
    ///      vec!["Bert", "M", "42", "68", "166"],
    ///      vec!["Carl", "F", "32", "70", "155"]
    ///  ])
    ///- Err(String): An error containing a message if parsing fails.
    ///  Example output in case of failure:
    ///  Err("Error while parsing the CSV".to_string())
    ///
    pub fn parse_csv(input: &str) -> Result<Vec<Vec<String>>, ParseError> {
        match separated_list1(newline, parse_record)(input) {
            Ok((_, records)) => Ok(records), // Return successfully parsed lines
            Err(_) => Err(ParseError::CsvParseError), // Error handling if parsing fails
        }
    }

    ///
    ///                      parse_record
    ///---------------------------------------------------------
    ///Input: A line from a CSV file as `&str`.
    ///Examples of input: "Alex,M,41,74,170", "Carl,F,32,70,155"
    ///
    ///Output: `IResult<&str, Vec<String>>`
    ///- `&str`: Unparsed remaining part (usually empty if everything is parsed).
    ///- `Vec<String>`: Vector containing the CSV line fields.
    ///  Example output:
    ///  Ok(("", vec!["Alex", "M", "41", "74", "170"]))
    ///
    fn parse_record(input: &str) -> IResult<&str, Vec<String>> {
        separated_list1(char(','), parse_field)(input) // Split a CSV line into fields based on commas
    }

    ///
    ///                      parse_field
    ///---------------------------------------------------------
    ///Input: A single field as `&str`.
    ///Examples of input: "Alex", " 41 ", "Weight"
    ///
    ///Output: `IResult<&str, String>`
    ///- `&str`: Unparsed remaining part (after extracting a field).
    ///- `String`: Cleaned field containing only alphanumeric characters.
    ///  Example output:
    ///  Ok(("", "Alex".to_string()))
    ///
    fn parse_field(input: &str) -> IResult<&str, String> {
        let (next_input, field) = is_not(",\n")(input)?; // Read until the next comma or newline
        let cleaned_field = clean_field(field); // Clean the field (e.g., remove spaces, special characters)
        Ok((next_input, cleaned_field))
    }

    ///
    ///                      clean_field
    ///---------------------------------------------------------
    ///Input: A string representing a CSV field.
    ///Example input: " Alex ", " 41 ", "Carl!"
    ///
    ///Output: A cleaned string containing only alphanumeric characters.
    ///Example output:
    ///- "Alex" becomes "Alex"
    ///- "     41 " becomes "41"
    ///- "Carl!" becomes "Carl"
    ///
    fn clean_field(field: &str) -> String {
        field
            .chars()
            .filter(|c| c.is_alphanumeric()) // Keep only alphanumeric characters
            .collect()
    }

    ///
    ///                      pretty_print_csv
    ///---------------------------------------------------------
    ///Input: A vector of vectors of strings (the parsed CSV lines).
    ///Example input:
    ///
    ///vec![
    ///    vec!["Alex", "M", "41", "74", "170"],
    ///    vec!["Bert", "M", "42", "68", "166"],
    ///    vec!["Carl", "F", "32", "70", "155"]
    ///]
    ///
    ///Output: No return value (`()`), but prints the lines with aligned columns in the console.
    ///Example output:
    ///
    ///Alex            M               41              74              170             
    ///Bert            M               42              68              166             
    ///Carl            F               32              70              155
    ///
    pub fn pretty_print_csv(csv_lines: Vec<Vec<String>>) {
        if csv_lines.is_empty() {
            println!("The CSV is empty!");
            return;
        }

        println!("Pretty CSV display:");

        for line in csv_lines {
            for cell in line {
                print!("{:15} ", cell); // Align columns with fixed-width formatting
            }
            println!(); // New line after each CSV line
        }
    }

    ///
    ///                      parse_and_get_line_from_csv
    ///---------------------------------------------------------
    ///Input: 
    ///- `input`: CSV content as `&str`.
    ///- `line_number`: Line number to retrieve.
    ///
    ///Output: `Result<String, String>`
    ///- Ok(String): The retrieved line as a string, with fields separated by commas.
    ///  Example output for `line_number = 1`: "Bert, M, 42, 68, 166"
    ///- Err(String): An error message if the line doesn't exist or if parsing fails.
    ///
    pub fn parse_and_get_line_from_csv(input: &str, line_number: usize) -> Result<String, ParseError> {
        match parse_csv(input) {
            Ok(records) => records
                .get(line_number)
                .map(|line| line.join(", ")) // Join fields with commas
                .ok_or(ParseError::IndexOutOfBounds),
            Err(e) => Err(e), // Error handling in case of parsing issues
        }
    }

    ///
    ///                      parse_and_get_col_from_csv
    ///---------------------------------------------------------
    ///Input: 
    ///- `input`: CSV content as `&str`.
    ///- `col_number`: Column number to retrieve.
    ///
    ///Output: `Result<Vec<String>, String>`
    ///- Ok(Vec<String>): The retrieved column as a vector of string.
    ///  Example output for `col_number = 0`: ["Name", "Alex", "Bert", "Carl", "Dave", ... ,"Ruth"]
    ///- Err(String): An error message if the column doesn't exist or if parsing fails.
    ///
    pub fn parse_and_get_col_from_csv(input: &str, col_number: usize) -> Result<Vec<String>, ParseError> {
        match parse_csv(input) {
            Ok(records) => {
                let mut column = Vec::new();
                for line in records {
                    if let Some(value) = line.get(col_number){
                        column.push(value.clone());
                    } else {
                        return Err(ParseError::IndexOutOfBounds);
                    }
                    
                }
                Ok(column)
            }
            Err(e) => Err(e),
        }
    }

    ///
    ///                      parse_and_sum_col_from_csv
    ///---------------------------------------------------------
    ////!\ Consider that the first line is a header so it ignores it
    ///Input: 
    ///- `input`: CSV content as `&str`.
    ///- `col_number`: Column number to sum.
    ///
    ///Output: `Result<i32, String>`
    ///- Ok(i32): The summed column as a i32.
    ///  Example output for `col_number = 4`: 2641
    ///- Err(String): An error message if the column doesn't exist, if parsing fails or if the column is composed by non-digit.
    ///
    pub fn parse_and_sum_col_from_csv(input: &str, col_number: usize) -> Result<i32, ParseError> {
        match parse_and_get_col_from_csv(input, col_number) {
            Ok(column) => {
                Ok(column
                    .iter() // For each element (string) of the column
                    .skip(1) // To ignore column header
                    .map(|value| value.parse::<i32>().map_err(ParseError::ParseIntError)) // Convert in i32
                    .collect::<Result<Vec<i32>, ParseError>>()? // Collect as a i32 vector or return an error
                    .iter() // For each element (i32) of the column, except the header
                    .sum::<i32>() // Sum each number
                )
            }
            Err(e) => Err(e),
        }
    }
}


fn main() {
    use crate::csv_parser::{
        parse_and_sum_col_from_csv,
        parse_and_get_col_from_csv,
        parse_and_get_line_from_csv,
        parse_csv,
        pretty_print_csv
    };
    use std::fs;

    // Load the CSV file
    let file_path = "biostats1.csv";

    let csv_content = match fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Error reading the file: {}", err);
            return;
        }
    };

    // Parse and print the CSV file :)
    match parse_csv(&csv_content) {
        Ok(records) => pretty_print_csv(records),
        Err(err) => eprintln!("Parsing error: {:?}", err),
    }

    // Getting data from the third line of the CSV file
    let line_number = 2;
    match parse_and_get_line_from_csv(&csv_content, line_number) {
        Ok(line) => println!("Line {}: {}", line_number, line),
        Err(err) => eprintln!("Error: {:?}", err),
    }

    // Getting data fro the first line of the CSV file
    let col_number = 0;
    match parse_and_get_col_from_csv(&csv_content, col_number) {
        Ok(column) => println!("Column {}: {:?}", col_number, column),
        Err(err) => eprintln!("Error: {:?}", err),
    }

    // Sum the 5th column (weigth) of the CSV file
    let col_to_sum = 4;
    match parse_and_sum_col_from_csv(&csv_content, col_to_sum) {
        Ok(sum) => println!("Sum of the column {}: {:?}", col_to_sum, sum),
        Err(err) => eprintln!("Error: {:?}", err),
    }

}

#[cfg(test)]
mod tests {
    use crate::csv_parser::{
        ParseError,
        // Functions under testing
        parse_and_get_col_from_csv, parse_and_get_line_from_csv, parse_csv, pretty_print_csv, parse_and_sum_col_from_csv
    };
    use std::fs;
    ///
    ///Tets of the function csv_parser::parse_csv
    ///
    #[test]
    fn test_parse_csv(){ // Given a valid csv file, return the vector (of vector (of string)) as intended
        let file_path = "biostats1.csv";

        let csv_content = match fs::read_to_string(file_path) {
            Ok(content) => content,
            Err(err) => {
                eprintln!("Error reading the file: {}", err);
                return;
            }
        };

        let result = parse_csv(&csv_content);
        assert_eq!(result, Ok(
            vec![
                vec!["Name", "Sex", "Age", "Heightin", "Weightlbs"],
                vec!["Alex", "M", "41", "74", "170"],
                vec!["Bert", "M", "42", "68", "166"],
                vec!["Carl", "M", "32", "70", "155"],
                vec!["Dave", "M", "39", "72", "167"],
                vec!["Elly", "F", "30", "66", "124"],
                vec!["Fran", "F", "33", "66", "115"],
                vec!["Gwen", "F", "26", "64", "121"],
                vec!["Hank", "M", "30", "71", "158"],
                vec!["Ivan", "M", "53", "72", "175"],
                vec!["Jake", "M", "32", "69", "143"],
                vec!["Kate", "F", "47", "69", "139"],
                vec!["Luke", "M", "34", "72", "163"],
                vec!["Myra", "F", "23", "62", "98"],
                vec!["Neil", "M", "36", "75", "160"],
                vec!["Omar", "M", "38", "70", "145"],
                vec!["Page", "F", "31", "67", "135"],
                vec!["Quin", "M", "29", "71", "176"],
                vec!["Ruth", "F", "28", "65", "131"]
            ]
            .into_iter()
            .map(|line| line.into_iter().map(String::from).collect()) // Convert each field into String as intended
            .collect::<Vec<Vec<String>>>()
        ));
    }
    ///
    ///Pseudo test of the function csv_parser::pretty_print_csv
    ///
    #[test]
    fn test_pretty_print_csv(){ // Given a valid CSV file, should print a nice sheet in the standard output (trust me)
        let file_path = "biostats1.csv";

        let csv_content = match fs::read_to_string(file_path) {
            Ok(content) => content,
            Err(err) => {
                eprintln!("Error reading the file: {}", err);
                return;
            }
        };
        // I don't know, just check the output of the main and validate it :')
        match parse_csv(&csv_content) {
            Ok(records) => {
                pretty_print_csv(records);
                assert!(true);
            },
            Err(err) => {
                eprintln!("Parsing error: {:?}", err);
                assert!(false);
            },
        }
    }
    ///
    ///Tests of the function csv_parsser::parse_and_get_line_from_csv
    ///
    #[test]
    fn test_parse_and_get_valid_line_from_csv() { // Given a valid line number of the CSV, return the corresponding line as string
        let file_path = "biostats1.csv";

        let csv_content = match fs::read_to_string(file_path) {
            Ok(content) => content,
            Err(err) => {
                eprintln!("Error reading the file: {}", err);
                return;
            }
        };
        let line_number = 2;
        let result = parse_and_get_line_from_csv(&csv_content, line_number);
        assert_eq!(result, Ok("Bert, M, 42, 68, 166".to_string()));
    }

    #[test]
    fn test_parse_and_get_invalid_line_from_csv() { // Given a invalid line number of the CSV, return the corresponding error
        let file_path = "biostats1.csv";

        let csv_content = match fs::read_to_string(file_path) {
            Ok(content) => content,
            Err(err) => {
                eprintln!("Error reading the file: {}", err);
                return;
            }
        };
        let invalid_line_number = 42;
        let result = parse_and_get_line_from_csv(&csv_content, invalid_line_number);
        assert_eq!(result, Err(ParseError::IndexOutOfBounds));
    }

    ///
    ///Tests of the function csv_parsser::parse_and_get_col_from_csv
    ///
    #[test]
    fn test_parse_and_get_valid_col_from_csv() { // Given a valid column number of the CSV, return the corresponding vector of string
        let file_path = "biostats1.csv";

        let csv_content = match fs::read_to_string(file_path) {
            Ok(content) => content,
            Err(err) => {
                eprintln!("Error reading the file: {}", err);
                return;
            }
        };
        let col_number = 0;
        let result = parse_and_get_col_from_csv(&csv_content, col_number);
        assert_eq!(
            result,
            Ok(vec![
                "Name".to_string(),
                "Alex".to_string(),
                "Bert".to_string(),
                "Carl".to_string(),
                "Dave".to_string(),
                "Elly".to_string(),
                "Fran".to_string(),
                "Gwen".to_string(),
                "Hank".to_string(),
                "Ivan".to_string(),
                "Jake".to_string(),
                "Kate".to_string(),
                "Luke".to_string(),
                "Myra".to_string(),
                "Neil".to_string(),
                "Omar".to_string(),
                "Page".to_string(),
                "Quin".to_string(),
                "Ruth".to_string(),
            ])
        );
    }

    #[test]
    fn test_parse_and_get_invalid_col_from_csv() { // Given a invalid column number of the CSV, return the corresponding error
        let file_path = "biostats1.csv";

        let csv_content = match fs::read_to_string(file_path) {
            Ok(content) => content,
            Err(err) => {
                eprintln!("Error reading the file: {}", err);
                return;
            }
        };
        let invalid_col_number = 6;
        let result = parse_and_get_col_from_csv(&csv_content, invalid_col_number);
        assert_eq!(result,Err(ParseError::IndexOutOfBounds));
    }

    ///
    ///Tests of the function csv_parsser::parse_and_sum_col_from_csv
    ///
    #[test]
    fn test_parse_and_sum_valid_col_from_csv() { // Given a valid column number of the CSV, return the right sum as i32.
        let file_path = "biostats1.csv";

        let csv_content = match fs::read_to_string(file_path) {
            Ok(content) => content,
            Err(err) => {
                eprintln!("Error reading the file: {}", err);
                return;
            }
        };
        let col_to_sum = 4;
        let result = parse_and_sum_col_from_csv(&csv_content, col_to_sum);
        assert_eq!(result, Ok(2641));
    }

    #[test]
    fn test_parse_and_sum_col_of_string() { // Given a column number of a string column from the CSV, return the corresponding error
        let file_path = "biostats1.csv";

        let csv_content = match fs::read_to_string(file_path) {
            Ok(content) => content,
            Err(err) => {
                eprintln!("Error reading the file: {}", err);
                return;
            }
        };
        let err_col_to_sum = 0;
        let result = parse_and_sum_col_from_csv(&csv_content, err_col_to_sum);
        assert_eq!(result, Err(ParseError::ParseIntError("Alex".parse::<i32>().err().unwrap())));
    }

    #[test]
    fn test_parse_and_sum_col_with_outofbound_number() { // Given a invalid column number of the CSV, return the corresponding error
        let file_path = "biostats1.csv";

        let csv_content = match fs::read_to_string(file_path) {
            Ok(content) => content,
            Err(err) => {
                eprintln!("Error reading the file: {}", err);
                return;
            }
        };
        let err_col_to_sum = 6;
        let result = parse_and_sum_col_from_csv(&csv_content, err_col_to_sum);
        assert_eq!(result,Err(ParseError::IndexOutOfBounds));
    }
}
