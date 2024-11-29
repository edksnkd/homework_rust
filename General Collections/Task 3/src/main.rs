use itertools::sorted;
use std::collections::HashMap;
use std::io;

fn main() {
    // хранение сотрудников и отделов: HashMap<Отдел, Vec<Сотрудники>>
    let mut departments: HashMap<String, Vec<String>> = HashMap::new();

    println!("Добро пожаловать! Введите команды:");
    println!("  Добавить <Имя> в <Отдел>");
    println!("  Список <Отдел>");
    println!("  Вывести все");
    println!("  Выйти");

    loop {
        // считываем ввод пользователя
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).expect("Ошибка ввода");
        let user_input = user_input.trim();

        if user_input.eq_ignore_ascii_case("Выйти") {
            println!("Вы вышли из программы.");
            break;
        }

        if let Some((employee, department)) = add_command(user_input) {
            add_employee(&mut departments, employee, department);
        } else if let Some(department_name) = list_command(user_input) {
            list_department_employees(&departments, department_name);
        } else if user_input.eq_ignore_ascii_case("Вывести все") {
            list_all_employees(&departments); // вызов функции для вывода всех сотрудников
        } else {
            println!("Неверная команда. Попробуйте снова.");
        }
    }
}

fn add_command(input: &str) -> Option<(String, String)> {
    let parts: Vec<&str> = input.split_whitespace().collect();
    if parts.len() >= 4 && parts[0].eq_ignore_ascii_case("Добавить") && parts[2].eq_ignore_ascii_case("в") {
        let employee = parts[1].to_string();
        let department = parts[3..].join(" ");
        Some((employee, department))
    } else {
        None
    }
}

fn list_command(input: &str) -> Option<String> {
    let parts: Vec<&str> = input.split_whitespace().collect();
    if parts.len() >= 2 && parts[0].eq_ignore_ascii_case("Список") {
        Some(parts[1..].join(" ")) // Возвращаем отдел, если команда "List <Отдел>"
    } else {
        None
    }
}

/// добавление сотрудника в отдел
fn add_employee(departments: &mut HashMap<String, Vec<String>>, employee: String, department: String) {
    departments.entry(department.clone())
        .or_insert_with(Vec::new)
        .push(employee.clone());
    println!("Добавлен сотрудник {} в отдел {}", employee, department);
}

/// вывод всех сотрудников из конкретного отдела
fn list_department_employees(departments: &HashMap<String, Vec<String>>, department_name: String) {
    match departments.get(&department_name) {
        Some(employees) => {
            println!("Сотрудники из отдела {}:", department_name);
            for employee in sorted(employees.iter()) {
                println!("- {}", employee);
            }
        }
        None => println!("Отдел {} не найден.", department_name),
    }
}

// вывод всех сотрудников компании, отсортированных по отделам
fn list_all_employees(departments: &HashMap<String, Vec<String>>) {
    let mut department_names: Vec<_> = departments.keys().collect();
    department_names.sort();  // Сортируем отделы

    if department_names.is_empty() {
        println!("Нет сотрудников в компании.");
        return;
    }

    for department in department_names {
        println!("\nОтдел {}:", department);
        if let Some(employees) = departments.get(department) {
            for employee in sorted(employees.iter()) {
                println!("- {}", employee);
            }
        }
    }
}
