use std::{error::Error, io, process};

use regex::Regex;

#[derive(Debug, serde::Deserialize)]
struct Transaction {
    date: String,
    category: Option<String>,
    title: String,
    amount: f64,
}

impl Transaction {
    fn to_sheets_format(&self) -> String {
        let installments_regex = Regex::new(r"(\d+)/(\d+)$").unwrap();
        format!(
            "{};{};{};{};{};{}",
            Transaction::format_date(&self.date),
            "Despesa".to_string(),
            &self.format_amount(),
            &self.title,
            self.category
                .clone()
                .map(|category| capitalize(&category))
                .unwrap_or("Outros".to_string()),
            self.extract_installment(&installments_regex).unwrap_or(""),
        )
    }

    fn format_amount(&self) -> String {
        format!("{:.2}", &self.amount).replace(".", ",")
    }

    fn extract_installment(&self, installments_regex: &Regex) -> Option<&str> {
        installments_regex
            .find(&self.title)
            .map(|mtch| mtch.as_str())
    }

    fn format_date(date: &str) -> String {
        let date_parts: Vec<_> = date.split("-").collect();
        let formatted_date: Vec<_> = date_parts.into_iter().rev().collect();

        formatted_date.join("/")
    }
}

fn capitalize(s: &str) -> String {
    format!(
        "{}{}",
        s.get(0..1).unwrap().to_ascii_uppercase(),
        s.get(1..).unwrap()
    )
}

fn adapt_csv_file() -> Result<(), Box<dyn Error>> {
    let mut reader = csv::Reader::from_reader(io::stdin());

    print_header();
    for result in reader.deserialize() {
        let transaction: Transaction = result?;
        println!("{}", transaction.to_sheets_format());
    }
    Ok(())
}

fn print_header() {
    println!("Data;Tipo;Valor;Descrição;Categoria;Parcela");
}

fn main() {
    if let Err(err) = adapt_csv_file() {
        println!("Error parsing file: {}", err);
        process::exit(1);
    }
}
