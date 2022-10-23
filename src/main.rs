use tower_of_hanoi::list::List;

fn main() {
    let towers: Vec<Vec<u8>> = vec![
        vec![6, 4],
        vec![5, 3, 1],
        vec![2],
        vec![],
        vec![],
        vec![],
    ];
    check_order(&towers);
    check_duplicate(&towers);
    let end: Vec<Vec<u8>> = vec![
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![6, 5, 4, 3, 2, 1],
    ];
    check_order(&end);
    check_duplicate(&end);

    check_same_disks(&towers, &end);

    if breadth_search(&towers, end) {
    } else {
        println!("\nПуть не найден");
    }
}

fn check_order(towers: &Vec<Vec<u8>>) {
    for rod in towers {
        let last = rod.first();
        for disk in 1..rod.len() {
            if last.unwrap() <= &rod[disk] {
                panic!("Неправильные башни");
            }
        }
    }
}

fn check_duplicate(towers: &Vec<Vec<u8>>) {
    for (rods, rod) in towers.iter().enumerate() {
        for (disks, disk) in rod.iter().enumerate() {
            let pos_y = rods;
            let pos_x = disks;

            for row in 0..towers.len() {
                for col in 0..towers[row].len() {
                    if disk == &towers[row][col] && (pos_y != row || pos_x != col) {
                        panic!("Не может быть одинаковых дисков");
                    }
                }
            }
        }
    }
}

fn check_same_disks(start: &Vec<Vec<u8>>, end: &Vec<Vec<u8>>) {
    if start.len() != end.len() {
        panic!("Количество башен не совпадает");
    }
    let mut first = Vec::new();
    let mut second = Vec::new();

    for row in start {
        for col in row {
            first.push(col.clone());
        }
    }
    for row in end {
        for col in row {
            second.push(col.clone());
        }
    }

    first.sort();
    second.sort();

    if first != second {
        panic!("Диски начального и конечного состояния не совпадают");
    }
}

pub fn breadth_search(start: &Vec<Vec<u8>>, end: Vec<Vec<u8>>) -> bool {
    let mut open: List = List::new_one(start.clone());
    let mut closed: List = List::new();
    let mut counter: i64 = -1;
    let mut found = false;

    while !open.is_empty() {
        counter += 1;
        let current_state = open.pop_front().expect("Проблемы с pop_front");

        if current_state == end {
            println!("\nСделано {} шагов, верная башня:", counter);
            print_towers(&current_state);
            found = true;
            break;
        }

        closed.push_back(current_state.clone());

        // Проходим по каждой башне текущего состояния и пытаемся достать верхний диск
        for row in 0..current_state.len() {
            let mut clone = current_state.clone();
            // Тут мы достаем из копии текущего состояния верхний диск
            match clone[row].pop() {
                // Если диска не существует, двигаем дальше
                None => continue,
                // Если диск существует смотрим куда его можно положить
                Some(num) => {
                    // Проверяем каждую башню куда поставить
                    for row_clone in 0..current_state.len() {
                        let mut clone2 = clone.clone();
                        // Кроме той башни, где лежал диск изначально
                        if row != row_clone {
                            // Смотрим чему равен последний элемент башни
                            match current_state[row_clone].last() {
                                // Если башня была пуста, просто кладем диск
                                None => {
                                    clone2[row_clone].push(num);
                                }
                                // Если башня не пустая смотрим какой диск больше
                                Some(num_clone) => {
                                    if *num_clone > num {
                                        // Если лежит диск больше нашего, кладем сверху наш
                                        clone2[row_clone].push(num);
                                    } else {
                                        continue;
                                    }
                                }
                            }
                            // Проверяем, а существует ли такая комбинация уже в Closed или Open
                            if !(closed.exist(&clone2) || open.exist(&clone2)) {
                                // Раскомментировать, если нужны логи того, как это делается
                                // println!("Шаг {}:", counter);
                                // print_towers(&clone2);
                                // println!("#############################");

                                // Если не существует, добавляем
                                open.push_back(clone2);
                            }
                        }
                    }
                }
            };
        }
    }
    found
}

fn print_towers(towers: &Vec<Vec<u8>>) {
    for (rods, rod) in towers.iter().enumerate() {
        print!("Башня {}: ", rods + 1);
        for disk in rod {
            print!("{} ", disk);
        }
        println!();
    }
}
