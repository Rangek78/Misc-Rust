fn bubble_sort<T: PartialOrd>(list: &mut [T]) -> &[T] {
    let mut comparisons: usize = 0;
    let mut assignments: usize = 0;

    let mut end = list.len();

    loop {
        assignments += 1;
        let mut swapped = false;
        for i in 1..end {
            comparisons += 1;
            if list[i - 1] > list[i] {
                assignments += 4;
                list.swap(i - 1, i);
                swapped = true;
            }
        }
        comparisons += 1;
        if !swapped {
            println!(
                "Bolha: {} comparações, {} atribuições",
                comparisons, assignments
            );
            return list;
        }
        assignments += 1;
        end -= 1;
    }
}

fn selection_sort<T: PartialOrd>(list: &mut [T]) -> &[T] {
    let mut comparisons: usize = 0;
    let mut assignments: usize = 0;

    let mut min;
    let len = list.len();
    for i in 0..(len - 1) {
        assignments += 1;
        min = i;
        for j in (i + 1)..len {
            comparisons += 1;
            if list[j] < list[min] {
                assignments += 1;
                min = j;
            }
        }
        comparisons += 1;
        if min != i {
            assignments += 3;
            list.swap(i, min);
        }
    }
    println!(
        "Seleção: {} comparações, {} atribuições",
        comparisons, assignments
    );
    list
}

fn insertion_sort<T: PartialOrd>(list: &mut [T]) -> &[T] {
    let mut comparisons: usize = 0;
    let mut assignments: usize = 0;

    for i in 1..list.len() {
        for j in (1..=i).rev() {
            comparisons += 1;
            if list[j - 1] > list[j] {
                assignments += 3;
                list.swap(j, j - 1);
            }
        }
    }

    println!(
        "Inserção: {} comparações, {} atribuições",
        comparisons, assignments
    );
    list
}

fn main() {
    let v1 = vec![10, 9, 8, 7, 6, 5, 1, 2, 3, 4];
    let v2 = vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
    let v3 = vec![10, 7, 3, 2, 8, 6, 1, 9, 4, 5];

    println!("V1: {:?}\n", bubble_sort(&mut v1.clone()[..]));
    println!("V2: {:?}\n", bubble_sort(&mut v2.clone()[..]));
    println!("V3: {:?}\n", bubble_sort(&mut v3.clone()[..]));

    println!("===============================\n");

    println!("V1: {:?}\n", insertion_sort(&mut v1.clone()[..]));
    println!("V2: {:?}\n", insertion_sort(&mut v2.clone()[..]));
    println!("V3: {:?}\n", insertion_sort(&mut v3.clone()[..]));

    println!("===============================\n");

    println!("V1: {:?}\n", selection_sort(&mut v1.clone()[..]));
    println!("V2: {:?}\n", selection_sort(&mut v2.clone()[..]));
    println!("V3: {:?}\n", selection_sort(&mut v3.clone()[..]));
}
