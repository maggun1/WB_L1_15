fn qsort<T>(collection: &mut [T])
where T: Ord + Clone {
    if collection.len() <= 1 {
        return;
    }

    let pivot_index = partition(collection);
    qsort(&mut collection[0..pivot_index]);
    qsort(&mut collection[pivot_index + 1..]);
}

fn partition<T>(collection: &mut [T]) -> usize
where T: Ord + Clone {
    let pivot = collection.len() - 1;
    let mut i = 0;
    for j in 0..pivot {
        if collection[j] <= collection[pivot] {
            collection.swap(i, j);
            i += 1;
        }
    }
    collection.swap(i, pivot);
    i
}

fn main() {
    let mut int_vec = vec![3, 6, 8, 10, 1, 2, 1, 9, 1234, 13, 445, 1356];
    println!("int vec: {:?}", int_vec);
    qsort(&mut int_vec);
    println!("Sorted int vec: {:?}", int_vec);
    println!();

    let mut str_vec = vec!["rust", "python", "c++"];
    println!("&str vec: {:?}", str_vec);
    qsort(&mut str_vec);
    println!("Sorted &str vec: {:?}", str_vec);
}
