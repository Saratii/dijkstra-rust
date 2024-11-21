use std::io::{stdin, BufRead};
use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn main() {
    let mut matrix: Vec<Vec<i32>> = Vec::new();
    for line in stdin().lock().lines() {
        let line = line.expect("Whoopsie");
        if line.trim().is_empty() {
            break;
        }
        matrix.push(line.split_whitespace().map(|x| x.parse::<i32>().unwrap_or(i32::MAX)).collect::<Vec<i32>>());}
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).expect("Whoopsie");
    let input: Vec<char> = buffer.trim().split_whitespace().map(|x| x.parse().expect("Whoopsie")).collect();
    let (distance, path) = dijkstra(&matrix, convert_letter_to_number(input[0]) as usize, convert_letter_to_number(input[1]) as usize);
    println!("{:?}", path.iter().map(|x| convert_number_to_letter(*x as u8)).collect::<Vec<char>>());
    println!("{:?}", distance);
}

fn dijkstra(matrix: &Vec<Vec<i32>>, start: usize, end: usize) -> (i32, Vec<usize>) {
    let mut distances = vec![i32::MAX; matrix.len()];
    let mut paths = vec![usize::MAX; matrix.len()];
    let mut queue = BinaryHeap::new();
    distances[start] = 0;
    queue.push(Reverse((0, start)));
    while let Some(Reverse((dist, row))) = queue.pop() {
        if row == end {
            break;
        }
        if dist > distances[row] {
            continue;
        }
        for col in 0..matrix.len() {
            let weight = matrix[row][col];
            if weight == i32::MAX {
                continue;
            }
            let other_node = distances[row] + weight;
            if other_node < distances[col] {
                distances[col] = other_node;
                paths[col] = row;
                queue.push(Reverse((other_node, col)));
            }
        }
    }
    let mut path = Vec::new();
    let mut row = end;
    while paths[row] != usize::MAX {
        let next_node = paths[row];
        path.push(row);
        row = next_node;
    }
    path.push(start);
    path.reverse();
    (distances[end], path)
}

fn convert_number_to_letter(n: u8) -> char {
    (b'a' + n) as char
}

fn convert_letter_to_number(c: char) -> u8 {
    (c as u8) - b'a'
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dik_1() {
        let matrix = vec![
            vec![i32::MAX, 6,        3,        i32::MAX],
            vec![6,        i32::MAX, 4,        1       ],
            vec![3,        4,        i32::MAX, 1       ],
            vec![i32::MAX, 1,        1,        i32::MAX],
        ];
        let (distance, path) = dijkstra(&matrix, convert_letter_to_number('a') as usize, convert_letter_to_number('b') as usize);
        assert_eq!(vec![0, 2, 3, 1], path);
        assert_eq!(distance, 5);
        assert_eq!(path.iter().map(|x| convert_number_to_letter(*x as u8)).collect::<Vec<char>>(), vec!['a', 'c', 'd', 'b']);
    }

    #[test]
    fn test_dik_2() {
        let matrix = vec![
            vec![i32::MAX, 2, i32::MAX, 8, i32::MAX, i32::MAX],
            vec![2, i32::MAX, i32::MAX, 5, 6, i32::MAX],
            vec![i32::MAX, i32::MAX, i32::MAX, i32::MAX, 9, 3],
            vec![8, 5, i32::MAX, i32::MAX, 3, 2],
            vec![i32::MAX, 6, 9, 3, i32::MAX, 1],
            vec![i32::MAX, i32::MAX, 3, 2, 1, i32::MAX],
        ];
        let (distance, path) = dijkstra(&matrix, convert_letter_to_number('a') as usize, convert_letter_to_number('c') as usize);
        assert_eq!(vec![0, 1, 3, 5, 2], path);
        assert_eq!(distance, 12);
        assert_eq!(path.iter().map(|x| convert_number_to_letter(*x as u8)).collect::<Vec<char>>(), vec!['a', 'b', 'd', 'f', 'c']);
    }

    #[test]
    fn test_dik_3() {
        let matrix = vec![
            vec![i32::MAX, 2, i32::MAX, 8, i32::MAX, i32::MAX],
            vec![2, i32::MAX, i32::MAX, 5, 6, i32::MAX],
            vec![i32::MAX, i32::MAX, i32::MAX, i32::MAX, 9, 3],
            vec![8, 5, i32::MAX, i32::MAX, 3, 2],
            vec![i32::MAX, 6, 9, 3, i32::MAX, 1],
            vec![i32::MAX, i32::MAX, 3, 2, 1, i32::MAX],
        ];
        let (distance, path) = dijkstra(&matrix, convert_letter_to_number('a') as usize, convert_letter_to_number('c') as usize);
        assert_eq!(vec![0, 1, 3, 5, 2], path);
        assert_eq!(distance, 12);
        assert_eq!(path.iter().map(|x| convert_number_to_letter(*x as u8)).collect::<Vec<char>>(), vec!['a', 'b', 'd', 'f', 'c']);
    }

    #[test]
    fn test_dik_4() {
        let matrix = vec![
            vec![i32::MAX, 3, 20, 12, i32::MAX, i32::MAX],
            vec![3, i32::MAX, i32::MAX, 5, i32::MAX, i32::MAX],
            vec![20, i32::MAX, i32::MAX, 9, 2, i32::MAX],
            vec![12, 5, 9, i32::MAX, i32::MAX, 7],
            vec![i32::MAX, i32::MAX, 2, i32::MAX, i32::MAX, 8],
            vec![i32::MAX, i32::MAX, i32::MAX, 7, 8, i32::MAX],
        ];
        let (distance, path) = dijkstra(&matrix, convert_letter_to_number('a') as usize, convert_letter_to_number('e') as usize);
        assert_eq!(distance, 19);
        assert_eq!(path.iter().map(|x| convert_number_to_letter(*x as u8)).collect::<Vec<char>>(), vec!['a', 'b', 'd', 'c', 'e']);
    }
}

/*
∞ 3 20 12 ∞ ∞
3 ∞ ∞ 5 ∞ ∞
20 ∞ ∞ 9 2 ∞
12 5 9 ∞ ∞ 7
∞ ∞ 2 ∞ ∞ 8
∞ ∞ 2 ∞ ∞ 8  
*/