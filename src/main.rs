use std::collections::HashMap;
use std::fs::{File};
use std::io::{BufRead, BufReader, Write};

fn main() {
    let mut quote_dict = HashMap::new();
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
}
