До ваших послуг приклад програми на Rust, яка виконує основну обробку даних. Ця програма включає в себе структуру "Person", яка зберігає інформацію про ім'я та вік особи. Програма також включає в себе функції, які обробляють ці дані.

```rust
// Оголошення зовнішніх модулів
extern crate rand;
use rand::Rng;
use std::collections::HashMap;

// Визначення структури Person
#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// Визначення методів для структури Person
impl Person {
    fn new(name: &str, age: u8) -> Person {
        Person {
            name: name.to_string(),
            age,
        }
    }
}

// Головна функція
fn main() {
    let mut rng = rand::thread_rng();
    let mut people: Vec<Person> = Vec::new();
    
    // Додавання даних до вектора
    for i in 0..50 {
        let age: u8 = rng.gen_range(0..100);
        let name = format!("Person{}", i);
        people.push(Person::new(&name, age));
    }

    // Виведення імен та віку усіх людей
    for person in &people {
        println!("{} is {} years old.", person.name, person.age);
    }

    // Обчислення середнього віку
    let mut sum_ages: u32 = 0;
    for person in &people {
        sum_ages += person.age as u32;
    }
    let average_age: f32 = sum_ages as f32 / people.len() as f32;
    println!("Average age is {}.", average_age);

    // Підрахунок людей за віком
    let mut age_count: HashMap<u8, u32> = HashMap::new();
    for person in &people {
        let count = age_count.entry(person.age).or_insert(0);
        *count += 1;
    }
    
    // Виведення кількості людей за віком
    for (age, count) in &age_count {
        println!("There are {} people of age {}.", count, age);
    }
}
```

Ця програма створює 50 випадкових людей з віком в діапазоні від 0 до 100 років, обчислює та виводить середній вік цих людей, підраховує та виводить кількість людей для кожного віку. Примітка: для роботи цього коду потрібен пакет `rand`, який можна додати до вашого `Cargo.toml` таким чином:

```toml
[dependencies]
rand = "0.8.3"
```