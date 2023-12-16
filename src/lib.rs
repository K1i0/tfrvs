#[macro_use]
extern crate matrix;

use std::{env, time::Instant};

use matrix::prelude::*;

// Индекс макс элемента в векторе B
fn get_max_index_from_vec_b(vec_b: &Vec<usize>) -> usize {
    let mut result_max = 0;
    let mut index = 0;

    for i in 0..vec_b.len() {
        if vec_b[i] > result_max {
            result_max = vec_b[i];

            index = i
        }
    }

    return index;
}
// Индекс макс элемента в векторе A
fn get_min_index_from_vec_a(vec_a: &Vec<usize>) -> usize {
    let mut index = 0;

    let mut get_min = usize::MAX;

    for j in 0..vec_a.len() {
        if get_min > vec_a[j] {
            get_min = vec_a[j];
            index = j
        }
    }
    return index;
}
// Вектор платежей (столбец) по номеру столбца
fn get_column_by_num(sparse: &Compressed<usize>, n: usize, num: usize) -> Vec<usize> {
    let mut vec_res = Vec::new();

    for j in 0..n {
        vec_res.push(sparse.get((num, j)));
    }

    return vec_res;
}
// Вектор платежей (строка) по номеру столбца
fn get_str_by_num(sparse: &Compressed<usize>, n: usize, num: usize) -> Vec<usize> {
    let mut vec_res = Vec::new();

    for i in 0..n {
        vec_res.push(sparse.get((i, num)));
    }

    return vec_res;
}
// Просуммировать векторы платежей (столбец)
fn sum_matrix_column_by_num(
    now_index_b: usize,

    default: &Compressed<usize>,
    n: usize,

    vec_b: &mut Vec<usize>,
) {
    for i in 0..n {
        vec_b[i] = vec_b[i] + get_column_by_num(&default, n, now_index_b)[i];
    }
}
// Просуммировать векторы платежей (строка)
fn sum_matrix_str_by_num(
    now_index_a: usize,

    default: &Compressed<usize>,
    n: usize,

    vec_a: &mut Vec<usize>,
) {
    for j in 0..n {
        vec_a[j] = vec_a[j] + get_str_by_num(&default, n, now_index_a)[j];
    }
}

#[no_mangle]
pub extern "C" fn main_loop(n: usize, c1: usize, c2: usize, c3: usize, e: f64) {
    // let args: Vec<String> = env::args().collect();

    // // кол-во ЭМ
    // let n: usize = args[1].parse::<usize>().unwrap();
    // // let n: usize = 3;
    // // Параметры для генерации матрицы платежей
    // let c1 = args[2].parse::<usize>().unwrap();
    // let c2 = args[3].parse::<usize>().unwrap();
    // let c3 = args[4].parse::<usize>().unwrap();
    // // Погрешность
    // let e = args[5].parse::<f64>().unwrap();

    // Вектора сумм платежей (по стратегиям)
    let mut vec_a: Vec<usize> = Vec::new();
    let mut vec_b: Vec<usize> = Vec::new();

    // Матрица размерностью nxn
    let mut sparse_const: Compressed<usize> = Compressed::zero((n, n));

    // Инициализация матрицы
    for i in 0..n {
        for j in 0..n {
            if i >= j {
                sparse_const.set((i, j), j * c1 + (i - j) * c2)
            } else {
                sparse_const.set((i, j), j * c1 + (j - i) * c3)
            }
        }
    }

    // for i in 0..n {
    //     for j in 0..n {
    //         print!("{} ", sparse_const.get((i, j)))
    //     }
    //     println!()
    // }

    // Вектора оптимальных стратегий A, B (кол-во выборов каждой стратегии)
    let mut stratetgy_a: Vec<f64> = Vec::new();
    let mut stratetgy_b: Vec<f64> = Vec::new();

    // Инициализация векторов выборов стратегий
    for _ in 0..n {
        stratetgy_a.push(0.0);
        stratetgy_b.push(0.0);
    }

    let start = Instant::now();

    // Выбирает b выбор исходной стратегию (первичные ходы)
    vec_b = get_column_by_num(&sparse_const, n, 0);
    // Игрок А выбирает стратегию платежей
    let mut hod_a = get_max_index_from_vec_b(&vec_b);
    // Выбирает a выбор исходной стратегию (первичные ходы)
    vec_a = get_str_by_num(&sparse_const, n, hod_a);
    // Игрок B выбирает стратегию платежей
    let mut hod_b = get_min_index_from_vec_a(&vec_a);
    vec_b = get_column_by_num(&sparse_const, n, hod_b);

    let mut t = 1;

    let mut result = -1.0;

    loop {
        t += 1;

        hod_a = get_max_index_from_vec_b(&vec_b); //получили номер строки А с наибольшим числом в столбце
        // Суммирует платежи
        sum_matrix_str_by_num(hod_a, &sparse_const, n, &mut vec_a);

        stratetgy_a[hod_a] += 1.0;

        // for i in 0..n {
        //     print!("{} ", vec_a[i])
        // }
        // print!(" -- ");
        // for i in 0..n {
        //     print!("{} ", vec_b[i])
        // }

        // println!();

        hod_b = get_min_index_from_vec_a(&vec_a); //не меняет матрицу Sum
        sum_matrix_column_by_num(hod_b, &sparse_const, n, &mut vec_b);
        stratetgy_b[hod_b] += 1.0;

        // for i in 0..n {
        //     print!("{} ", vec_a[i])
        // }
        // print!(" -- ");
        // for i in 0..n {
        //     print!("{} ", vec_b[i])
        // }

        // println!();

        let v_n = *vec_a.iter().min().unwrap() as f64 / t as f64;
        let v_v = *vec_b.iter().max().unwrap() as f64 / t as f64;

        let v = (v_n + v_v) / 2.0;

        if (v - result).abs() < e {
            break;
        }
        result = v;
    }

    stratetgy_a = stratetgy_a.iter_mut().map(|x| *x / (t as f64)).collect();
    stratetgy_b = stratetgy_b.iter_mut().map(|x| *x / (t as f64)).collect();

    let duration = start.elapsed();

    for i in 0..n {
        print!("{} ", stratetgy_a[i])
    }
    println!();
    for i in 0..n {
        print!("{} ", stratetgy_b[i])
    }
    println!();

    println!("Iter: {t}");

    println!("V: {result}");

    println!("Time is: {:?}", duration.as_nanos());
}