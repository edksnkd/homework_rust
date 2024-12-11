
use std::fs::File;
use std::io::{self, BufRead};


fn main() -> io::Result<()> {


    // Открываем файл
    let file = File::open("file.txt").expect("Не удалось открыть файл");
    let reader = io::BufReader::new(file);

    // Читаем строки и разбиваем их на слова
    let mut table: Vec<Vec<String>> = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let row: Vec<String> = line.split_whitespace().map(String::from).collect();
        table.push(row);
    }

    // Транспонируем таблицу
    let num_rows = table.len();
    let num_cols = table[0].len();

    let mut transposed: Vec<Vec<String>> = vec![vec![String::new(); num_rows]; num_cols];
    for i in 0..num_rows {
        for j in 0..num_cols {
            transposed[j][i] = table[i][j].clone();
        }
    }

    // Выводим транспонированную таблицу
    for row in transposed {
        println!("{}", row.join(" "));
    }

    Ok(())
}
