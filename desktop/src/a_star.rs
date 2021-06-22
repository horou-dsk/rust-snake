use std::collections::{HashMap, VecDeque};

const WIDTH: i32 = 30;
const HEIGHT: i32 = 30;

fn neighbors(pos: &[i32]) -> Vec<[i32; 2]> {
    let mut ns = Vec::new();
    if pos[0] + 1 <= WIDTH { ns.push([pos[0] + 1, pos[1]]) }
    if pos[0] - 1 >= 0 { ns.push([pos[0] - 1, pos[1]]) }
    if pos[1] + 1 <= HEIGHT { ns.push([pos[0], pos[1] + 1]) }
    if pos[1] - 1 >= 0 { ns.push([pos[0], pos[1] - 1]) }
    ns
}

pub fn a_star_search(obs: &[[i32; 2]], start: [i32; 2], goal: [i32; 2]) -> Vec<[i32; 2]> {
    let mut heap = VecDeque::new();
    let mut came_from = HashMap::new();
    heap.push_back(start);
    came_from.insert(start, None);
    while let Some(current) = heap.pop_front() {
        if current == goal { break }

        for next in neighbors(&current).into_iter().filter(|v| !obs.contains(v)) {
            if !came_from.contains_key(&next) {
                heap.push_back(next);
                came_from.insert(next, Some(current));
            }
        }
    }
    let mut paths = vec![goal];
    let mut prev = goal;
    while let Some(pos) = came_from[&prev] {
        if pos == start { break }
        paths.push(pos);
        prev = pos;
    }
    paths
}

#[cfg(test)]
mod tests {
    use crate::a_star::{a_star_search};

    // #[test]
    // fn pos_cmp() {
    //     let a = Pos(1, 2);
    //     let b = Pos(1, 3);
    //     assert_eq!(a > b, false);
    //     assert_eq!([1, 2] > [1, 3], false);
    // }

    #[test]
    fn search() {
        println!("{:?}", a_star_search(&[[5, 5], [5, 6], [5, 7]], [5, 4], [5, 8]));
    }
}
