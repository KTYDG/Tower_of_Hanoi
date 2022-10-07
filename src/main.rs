use tower_of_hanoi::list::List;



fn main() {
    let towers: Vec<Vec<u8>> = vec![
        vec![3, 2, 1],
        vec![8, 6, 4],
        vec![9, 7, 5]
    ];
    check_order(&towers);
    check_duplicate(&towers);
    let end: Vec<Vec<u8>> = vec![
        vec![1],
        vec![7, 6, 5, 4, 3, 2],
        vec![9, 8]
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
    // let mut path: Vec<(usize, usize)> = Vec::new();
    let mut counter: i64 = -1;
    let mut found = false;

    while !open.is_empty() {
        counter += 1;
        let x = open.pop_front().expect("Проблемы с pop_front");

        if x == end {
            println!("\nСделано {} шагов, верная башня:", counter);
            print_towers(&x);
            // print!("сделано {} шагов, {}", counter, create_path(path));
            found = true;
            break;
        }

        closed.push_back(x.clone());

        for row in 0..x.len() {
            let mut clone = x.clone();
            match clone[row].pop() {
                Some(num) => {
                    for row_clone in 0..x.len() {
                        let mut clone2 = clone.clone();
                        if row != row_clone {
                            match x[row_clone].last() {
                                Some(num_clone) => {
                                    if *num_clone > num {
                                        clone2[row_clone].push(num);
                                    } else {
                                        continue;
                                    }
                                },
                                None => {
                                    clone2[row_clone].push(num);
                                },
                            }
                            if !(closed.exist(&clone2) || open.exist(&clone2)) {
                                println!("Шаг {}:", counter);
                                print_towers(&clone2);
                                println!("#############################");
                                open.push_back(clone2);
                            }
                        }
                    }
                },
                None => continue,
            };
        }
    }
    found
}

fn print_towers(towers: &Vec<Vec<u8>>) {
    for (rods, rod) in towers.iter().enumerate() {
        print!("Башня {}: ", rods+1);
        for disk in rod {
            print!("{} ", disk);
        }
        println!();
    }
}