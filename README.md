# CSV parser in Rust


## Table of Contents  
1. [Introduction](#introduction)  
2. [Usage](#usage)  
3. [The-nom-crate](#the-nom-crate)  
4. [Project-license](#project-license)  

---

## Introduction

### Presentation

This project proposes a draft parser for CSV files.
It includes:
- 1 source file, main.rs: implementation, usage examples, and unit tests
- 2 configuration files for cargo, Cargo.lock and Cargo.toml
- 1 CSV language description file, csv_description.ebnf
- 1 Git configuration file, .gitignore
- 1 CSV file example, biostats1.csv
- 1 LICENSE
- 1 documentation file, README.md

### Crate used

This project uses the [nom](https://crates.io/crates/nom) crate for parsing CSV files.  
The `nom` crate is licensed under the [MIT License](https://opensource.org/licenses/MIT).

---

## Usage

### parse_csv

Function used to parse a CSV file. Need the functions ```parse_record```, ```parse_field``` and ```clean_field```.

```
pub fn parse_csv(input: &str) -> Result<Vec<Vec<String>>, String>
```

**Input:** CSV file as a string (full string).
Examples of input:
- ```"Alex,M,41,74,170\nBert,M,42,68,166\nCarl,F,32,70,155\n"```

**Output:** `Result<Vec<Vec<String>>, String>`
- Ok(Vec<Vec<String>>): Each line of the CSV file is transformed into a vector of fields (a vector of vectors).
Example of output:
```
Ok(vec![
vec!["Alex", "M", "41", "74", "170"],
vec!["Bert", "M", "42", "68", "166"],
vec!["Carl", "F", "32", "70", "155"]
])
```
- Err(String): An error containing a message if parsing fails. Example output in case of failure:
```
Err("Error while parsing the CSV".to_string())
```

### parse_record

Function used to parse records (= lines of CSV file). Need the functions ```parse_field``` and ```clean_field```.

```
fn parse_record(input: &str) -> IResult<&str, Vec<String>>
```

**Input:** A line from a CSV file as `&str`.
Examples of input: ```"Alex,M,41,74,170", "Carl,F,32,70,155"```

**Output:** `IResult<&str, Vec<String>>`
- `&str`: Unparsed remaining part (usually empty if everything is parsed).
- `Vec<String>`: Vector containing the CSV line fields.
  Example output:
  ```
  Ok(("", vec!["Alex", "M", "41", "74", "170"]))
  ```   
    
### parse_field

Function used to parse the fields. Need the function ```clean_field```.

```
fn parse_field(input: &str) -> IResult<&str, String>
```

**Input:** A single field as `&str`.
Examples of input: `"Alex"`, `" 41 "`, `"Weight"`

**Output:** `IResult<&str, String>`
- `&str`: Unparsed remaining part (after extracting a field).
- `String`: Cleaned field containing only alphanumeric characters.
  Example output:
  ```
  Ok(("", "Alex".to_string()))
  ```   

### clean_field

Functions used to remove undiserable charactere from a field.

```
fn clean_field(field: &str) -> String
```

**Input:** A string representing a CSV field.
Example input: `" Alex "`, `" 41 "`, `"Carl!"`

**Output:** A cleaned string containing only alphanumeric characters.
Example output:
- `"Alex"` becomes `"Alex"`
- `"     41 "` becomes `"41"`
- `"Carl!"` becomes `"Carl"`

### pretty_print_csv

Function used to print a CSV content into the terminal.

```
pub fn pretty_print_csv(csv_lines: Vec<Vec<String>>)
```

**Input:** A vector of vectors of strings (the parsed CSV lines).
Example input:
```
vec![
    vec!["Alex", "M", "41", "74", "170"],
    vec!["Bert", "M", "42", "68", "166"],
    vec!["Carl", "F", "32", "70", "155"]
]
```

**Output:** No return value (`()`), but prints the lines with aligned columns in the console.
Example output:
```
Alex            M               41              74              170             
Bert            M               42              68              166             
Carl            F               32              70              155
```
 
### get_line_from_csv

Function to get a specific line from a CSV file.

```
pub fn get_line_from_csv(input: &str, line_number: usize) -> Result<String, String>
```

**Input:**
- `input`: CSV content as `&str`.
- `line_number`: Line number to retrieve.

**Output:** `Result<String, String>`
- Ok(String): The retrieved line as a string, with fields separated by commas.
  Example output for `line_number = 1`: ```"Bert, M, 42, 68, 166"```
- Err(String): An error message if the line doesn't exist or if parsing fails.

### get_col_from_csv

Function to get a specific column from a CSV file.

```
pub fn get_col_from_csv(input: &str, col_number: usize) -> Result<Vec<String>, String> 
```

**Input:** 
- `input`: CSV content as `&str`.
- `col_number`: Column number to retrieve.

**Output:** `Result<Vec<String>, String>`
- Ok(Vec<String>): The retrieved column as a vector of string.
  Example output for `col_number = 0`: ```["Name", "Alex", "Bert", "Carl", "Dave", ... ,"Ruth"]```
- Err(String): An error message if the column doesn't exist or if parsing fails.

### sum_col_from_csv

Function to sum a column full of numbers, from a CSV file.

```
pub fn sum_col_from_csv(input: &str, col_number: usize) -> Result<i32, String>
```

**Input:** 
- `input`: CSV content as `&str`.
- `col_number`: Column number to sum.

**Output:** `Result<i32, String>`
- Ok(i32): The retrieved column as a vector of string.
  Example output for `col_number = 4`: `2641`
- Err(String): An error message if the column doesn't exist, if parsing fails or if the column is composed by non-digit.
     
---

## The nom crate

### Type Aliase

**nom::IResult**

Holds the result of parsing functions. I for input, O for ouput and E for error.
```
Ok((I,O))
Err(Err<E>)
```
https://docs.rs/nom/latest/nom/type.IResult.html

### multi module

**nom::multi::separated_list1**

Alternates between two parsers to produce a list of elements until ```Err::Error```.
```separated_list1(sep,f)``` => parses the separator ```sep``` between list of elements ```f```.<br>
https://docs.rs/nom/latest/nom/multi/fn.separated_list1.html

### character module

**nom::character::complete**

Functions recognizing spectif characters. Example ```newline``` matches a newline character ```\n```.<br>
https://docs.rs/nom/latest/nom/character/complete/index.html

### bytes module

**nom::bytes::complete**

Parsers recognizing bytes streams, complete input version.<br>
https://docs.rs/nom/latest/nom/bytes/complete/index.html

**nom::bytes::complete::is_not**

Parse till certain characters are met.<br>
https://docs.rs/nom/latest/nom/bytes/complete/fn.is_not.html

---

## Project license

This project is licensed under the [MIT License](./LICENSE).