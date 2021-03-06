use std::collections::HashMap;
use std::fs::{File};
use std::io::{BufRead, BufReader, Write};

fn main() {
    // Save file into a String
    let mut quote_dict = HashMap::new(); // All the information is stored in this variable
    let filename = "My Clippings.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader
        	.lines()
            .map(|l| l.expect("Could not parse line"))
            .collect();

    let mut quote_info = Vec::new();
    let mut quote_container = Vec::new();
    for line in lines {
        if line.trim() == "==========" {
            quote_container.push(quote_info);
            quote_info = Vec::new();
        } else {
            quote_info.push(line);
        }
    }

    // Generate quotes
    for q in quote_container {
        let (book, date, quote) = (q[0].clone(), q[1].clone(), q[3].clone());

        let mut aux_dict = HashMap::new();
        aux_dict.insert(String::from("date"), date);
        aux_dict.insert(String::from("quote"), quote);

        let aux_vector = quote_dict.entry(book).or_insert(vec!());
        aux_vector.push(aux_dict);
    }

    let j = serde_json::to_string_pretty(&quote_dict).unwrap();

    let mut f = File::create("./quotes.json").expect("Unable to create file");
    f.write_all(j.as_bytes()).expect("Unable to write data");

    // Find book names and write them into a file
    let mut book_titles = Vec::new();
    for (key, value) in quote_dict {
        book_titles.push(key);
    }

    let mut book_file = File::create("./books.txt").expect("Unable to create file");
    writeln!(book_file, "{}", book_titles.join("\n"));
}
