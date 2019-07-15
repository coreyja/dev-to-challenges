#[macro_use]
extern crate lazy_static;

use regex::Regex;

#[derive(Debug)]
struct Transaction {
    check_number: String, // This is just an identifier to me. Who am I to stop your UUID check number
    category: String,
    amount: f32,
}

impl Transaction {
    fn from_line(line: &str) -> Self {
        lazy_static! {
            static ref STRIP_REGEX: Regex = Regex::new(r"[^\w \.]").unwrap();
            static ref PARSE_REGEX: Regex = Regex::new(r"(\d+) (.+) (\d*\.\d*)").unwrap();
        }

        let stripped_line: String = STRIP_REGEX.replace_all(line, "").to_string();

        println!("{}", stripped_line);
        let captures = PARSE_REGEX.captures(&stripped_line).unwrap();

        Self {
            check_number: captures.get(1).unwrap().as_str().to_owned(),
            category: captures.get(2).unwrap().as_str().to_owned(),
            amount: captures.get(3).unwrap().as_str().parse().unwrap(),
        }
    }

    fn to_string(&self) -> String {
        format!("{} {} {:.2}", self.check_number, self.category, self.amount)
    }
}

pub fn process_checkbook_string(input: &str) -> String {
    let mut lines = input.lines();
    let starting_balance: f32 = lines.next().unwrap().parse().unwrap();

    let transactions = lines.map(Transaction::from_line);

    let mut current_balance = starting_balance;
    let mut total_expense = 0.;
    let mut transaction_count = 0;
    let mut transaction_strings: Vec<String> = transactions
        .map(|transaction| {
            current_balance -= transaction.amount;
            total_expense += transaction.amount;
            transaction_count += 1;
            format!("{} Balance {:.2}", transaction.to_string(), current_balance)
        })
        .collect();

    let starting_balance_string = format!("Original_Balance: {:.2}", starting_balance);
    let total_expense_string = format!("Total expense {:.2}", total_expense);
    let avg_expense_string = format!(
        "Average expense {:.2}",
        total_expense / transaction_count as f32
    );

    let mut output = vec![starting_balance_string];
    output.append(&mut transaction_strings);
    output.push(total_expense_string);
    output.push(avg_expense_string);
    output.join("\n")
}

#[cfg(test)]
mod tests {
    use crate::*;

    const EXAMPLE_INPUT: &str = "1000.00
125 Market 125.45
126 Hardware 34.95
127 Video 7.45
128 Book 14.32
129 Gasoline 16.10";

    const EXAMPLE_OUTPUT: &str = "Original_Balance: 1000.00
125 Market 125.45 Balance 874.55
126 Hardware 34.95 Balance 839.60
127 Video 7.45 Balance 832.15
128 Book 14.32 Balance 817.83
129 Gasoline 16.10 Balance 801.73
Total expense 198.27
Average expense 39.65";

    const CHALLENGE_INPUT: &str = "1233.00
125 Hardware;! 24.8?;
123 Flowers 93.5
127 Meat 120.90
120 Picture 34.00
124 Gasoline 11.00
123 Photos;! 71.4?;
122 Picture 93.5
132 Tires;! 19.00,?;
129 Stamps 13.6
129 Fruits{} 17.6
129 Market;! 128.00?;
121 Gasoline;! 13.6?;";

    const CHALLENGE_OUTPUT: &str = "Original_Balance: 1233.00
125 Hardware 24.80 Balance 1208.20
123 Flowers 93.50 Balance 1114.70
127 Meat 120.90 Balance 993.80
120 Picture 34.00 Balance 959.80
124 Gasoline 11.00 Balance 948.80
123 Photos 71.40 Balance 877.40
122 Picture 93.50 Balance 783.90
132 Tires 19.00 Balance 764.90
129 Stamps 13.60 Balance 751.30
129 Fruits 17.60 Balance 733.70
129 Market 128.00 Balance 605.70
121 Gasoline 13.60 Balance 592.10
Total expense 640.90
Average expense 53.41";

    #[test]
    fn it_works_for_the_example() {
        assert_eq!(EXAMPLE_OUTPUT, process_checkbook_string(EXAMPLE_INPUT));
    }

    #[test]
    fn it_works_for_the_challenge() {
        assert_eq!(CHALLENGE_OUTPUT, process_checkbook_string(CHALLENGE_INPUT));
    }
}
