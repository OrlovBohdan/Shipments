use rand::Rng;

fn count_permutation(shipments: &Vec<u32>) -> usize {
    // Обчислюємо загальний вантаж та середнє значення
    let total: u32 = shipments.iter().sum();
    let average = total / shipments.len() as u32;

    // Створюємо змінну для підрахунку мінімальних перенесених вантажів
    let mut moves = 0;
    let mut current_shipments = shipments.clone();

    println!("{:?}", current_shipments);
    println!("total   = {}", total);
    println!("average = {}", average);

    // Виконуємо перенос вантажів, щоб зрівняти їх
    loop {
        let mut changed = false;
        for i in 0..current_shipments.len() {
            let diff = current_shipments[i] as i32 - average as i32;

            if diff > 0 {
                // Переносимо вантаж з корабля i до інших кораблів
                let mut found = false;
                for j in 0..current_shipments.len() {
                    if i != j {
                        let required = average as i32 - current_shipments[j] as i32;
                        if required > 0 {
                            let transfer = required.min(diff);
                            current_shipments[i] = (current_shipments[i] as i32 - transfer) as u32;
                            current_shipments[j] = (current_shipments[j] as i32 + transfer) as u32;
                            moves += transfer.abs() as usize;
                            println!("{:?}", current_shipments);
                            found = true;
                            break;
                        }
                    }
                }
                if found {
                    changed = true;
                }
            }
        }

        // Якщо переноси завершено, зупиняємо цикл
        if !changed {
            break;
        }
    }

    moves
}

// Функція для генерації вантажів для кораблів, які можуть бути рівномірно розподілені
fn gen_shipments(n: usize) -> Vec<u32> {
    let mut rng = rand::thread_rng();

    // Визначаємо середнє значення для кожного корабля, щоб вантажі можна було рівномірно розподілити
    let average = rng.gen_range(5..10); // Середнє значення вантажу для кожного корабля
    let total: u32 = average * n as u32; // Загальний вантаж, щоб сума була кратна кількості кораблів

    // Створюємо вектор для зберігання вантажів
    let mut shipments = Vec::with_capacity(n);

    // Заповнюємо вектор випадковими значеннями близькими до середнього
    for _ in 0..n {
        shipments.push(rng.gen_range(average - 2..=average + 2));
    }

    // Коригуємо суму так, щоб вона дорівнювала необхідному total
    let sum: u32 = shipments.iter().sum();
    let diff = total as i32 - sum as i32;

    if diff != 0 {
        shipments[n - 1] = (shipments[n - 1] as i32 + diff) as u32;
    }

    shipments
}

fn main() {
    // Приклад 1
    let shipments_1 = vec![8, 2, 2, 4, 4];
    println!("\nanswer = {}", count_permutation(&shipments_1));

    // Приклад 2
    let shipments_2 = vec![9, 3, 7, 2, 9];
    println!("\nanswer = {}", count_permutation(&shipments_2));

    // Приклад 3: генеруємо вантажі для 5 кораблів
    let shipments_1 = gen_shipments(5);
    println!("\nGenerated shipments: {:?}", shipments_1);
    println!("answer = {}", count_permutation(&shipments_1));

    // Приклад 4: генеруємо вантажі для 5 кораблів
    let shipments_2 = gen_shipments(5);
    println!("\nGenerated shipments: {:?}", shipments_2);
    println!("answer = {}", count_permutation(&shipments_2));
}
/*
shipments — вектор вантажів на кораблях.
За допомогою суми всіх вантажів та кількості кораблів ми знаходимо середнє значення вантажу.
Проходимо по кожному кораблю і намагаємось перенести вантаж до інших кораблів, щоб привести їх до однакового рівня. Для кожного перенесеного вантажу додаємо кількість в змінну moves.
Виводимо проміжні етапи переносу вантажу і фінальний результат (загальну кількість перенесених вантажів).

Функція gen_shipments(n: usize) -> Vec<u32>:

Генерує вектор вантажів для n кораблів.
Спочатку генерується середнє значення для вантажу кожного корабля в діапазоні від 5 до 10 (можна змінити діапазон за потребою).
Потім генерується вектор вантажів, кожен з яких є випадковим числом в межах ±2 від середнього значення.
Після цього коригується сума вантажів, щоб вона була кратною кількості кораблів, шляхом коригування останнього елемента вектора.

Використовуємо функцію gen_shipments для генерації випадкових вантажів для 5 кораблів.
Потім викликається функція count_permutation, щоб обчислити кількість перенесених вантажів та вивести результат.
*/





