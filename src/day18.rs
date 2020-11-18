use itertools::Itertools;
use std::collections::HashSet;
use std::rc::Rc;

#[derive(Debug)]
pub struct Vault {
    tunnels: Vec<Vec<char>>,
    entrance: (i32, i32),
}

const START_SYMBOL: char = '@';
const WALL_SYMBOL: char = '#';
const FLOOR_SYMBOL: char = '.';

impl Vault {
    pub fn parse(input: &str) -> Self {
        let mut tunnels = Vec::<Vec<char>>::new();
        let mut entrance = (0, 0);
        for (row, line) in input.lines().enumerate() {
            tunnels.push(line.trim().chars().collect::<Vec<char>>());
            if let Some((col, _)) = tunnels.last().unwrap().iter().find_position(|c| c == &&START_SYMBOL) {
                entrance = (row as i32, col as i32);
            }
        }
        Vault { tunnels, entrance }
    }
}

mod v1 {
    use super::*;

    #[derive(Debug)]
    struct Node {
        parent: Option<Rc<Node>>,
        length_to_parent: u32,
        key: char,
    }

    struct NodeIterator {
        current: Option<Rc<Node>>
    }

    impl NodeIterator {
        fn from(node: Rc<Node>) -> NodeIterator { NodeIterator { current: Some(node) } }
    }

    impl Iterator for NodeIterator {
        type Item = Rc<Node>;
        fn next(&mut self) -> Option<Rc<Node>> {
            let next = self.current.as_ref().map(|n| Rc::clone(n));
            self.current =
                match self.current.as_ref() {
                    Some(n) => n.parent.as_ref().map(|n| Rc::clone(n)),
                    None => None
                };
            next
        }
    }

    impl Node {
        fn new(parent: Option<Rc<Node>>, length_to_parent: u32, key: char) -> Self {
            Node { parent, length_to_parent, key }
        }
    }


    fn iter(node: Rc<Node>) -> NodeIterator {
        NodeIterator::from(node)
    }

    fn nb_of_keys(node: Rc<Node>) -> u32 { iter(node).count() as u32 - 1 }

    fn length(node: Rc<Node>) -> u32 { iter(node).fold(0, |sum, node| sum + node.length_to_parent) }

    fn can_open(node: Rc<Node>, door: char) -> bool {
        let key_needed = door.to_ascii_lowercase();
        iter(node).any(|node| node.key == key_needed)
    }

    fn has_key(node: Rc<Node>, key: char) -> bool {
        iter(node).any(|node| node.key == key)
    }

    #[allow(dead_code)]
    pub fn nb_steps_to_collect_all_key(vault: &Vault) -> u32 {
        //dbg!(vault);

        fn find_keys(from : (i32, i32), parent: Rc<Node>, vault: &Vault) -> Vec<Rc<Node>> {
            let mut to_visit = vec![(from, 1)];
            let mut visited_positions: HashSet<(i32, i32)> = HashSet::new();
            let mut reachable_keys = Vec::<Rc<Node>>::new();

            //println!("find_keys: from:{:?}", from);
            //println!("Nb of keys: {}", nb_of_keys(Rc::clone(&parent)));

            while let Some((pos, steps)) = to_visit.pop() {
                //println!("Pos to visit: {:?}", pos);
                visited_positions.insert(pos);
                //steps += 1;
                for pos_d in &[(-1, 0), (0, 1), (1, 0), (0, -1)] {
                    let adjacent = (pos.0 + pos_d.0, pos.1 + pos_d.1);
                    //println!("Adjacent: {:?}", adjacent);
                    if !visited_positions.contains(&adjacent) {
                        match vault.tunnels[adjacent.0 as usize][adjacent.1 as usize] {
                            // Simple floor or a door or a owned key.
                            c if c == FLOOR_SYMBOL || c == START_SYMBOL || c.is_ascii_uppercase() && can_open(Rc::clone(&parent), c) || c.is_ascii_lowercase() && has_key(Rc::clone(&parent), c) => {
                                //println!("-> To visit");
                                to_visit.push((adjacent, steps + 1));
                            },
                            c if c.is_ascii_lowercase() => { // A non-owned key.
                                //println!("-> A new key! {:?}", c);
                                visited_positions.insert(adjacent);
                                let node = Rc::new(Node::new(Some(Rc::clone(&parent)), steps, c));
                                reachable_keys.append(&mut find_keys(adjacent, node, vault));
                            },
                            _ => (), //println!("-> WALL"),
                        }
                    }
                }
            }

            if reachable_keys.is_empty() {
                reachable_keys.push(parent);
            }

            reachable_keys
        }

        let root = Rc::new(Node::new(None, 0, START_SYMBOL));
        let nodes = find_keys(vault.entrance, root, vault);

        nodes.iter().map(|n| (length(Rc::clone(n)), nb_of_keys(Rc::clone(n)))).sorted_by(|(l1, n1), (l2, n2)| n1.cmp(&n2).then(l1.cmp(&l2))).next().unwrap().0
    }
}

mod v2 {
    use super::*;

    #[derive(Debug)]
    struct Path {
        to_visit: Vec<(i32, i32)>,
        visited: HashSet<(i32, i32)>,
        keys: Vec<char>
    }

    impl Path {
        pub fn new(initial_position: (i32, i32)) -> Self {
            Path { to_visit: vec![initial_position], visited: HashSet::new(), keys: Vec::new() }
        }
    }

    pub fn nb_steps_to_collect_all_key(vault: &Vault) -> u32 {
        let nb_of_keys: usize = vault.tunnels.iter().map(|line| line.iter().fold(0, |acc, c| if c.is_ascii_lowercase() { acc + 1 } else { acc })).sum();

        println!("nb_of_keys = {}", nb_of_keys);

        let mut paths = vec![Path::new(vault.entrance)];

        let mut step = 0;
        loop {
            step += 1;
            let mut new_paths: Vec<Path> = Vec::new();

            println!("----------------------------\n{:?}", paths);

            for i in (0 .. paths.len()).rev() {
                let path = &mut paths[i];

                let to_visit = path.to_visit.clone();
                path.to_visit.clear();

                let visited = path.visited.clone();
                path.visited.clear();

                for pos in to_visit {
                    path.visited.insert(pos);
                    for pos_d in &[(-1, 0), (0, 1), (1, 0), (0, -1)] {
                        let adjacent = (pos.0 + pos_d.0, pos.1 + pos_d.1);
                        if !visited.contains(&adjacent) {
                            let c = vault.tunnels[adjacent.0 as usize][adjacent.1 as usize];

                            if c == WALL_SYMBOL {
                            }
                            else if c.is_ascii_lowercase() && !path.keys.contains(&c) {
                                if path.keys.len() + 1 == nb_of_keys {
                                    return step;
                                }

                                let mut new_path = Path { to_visit: vec![adjacent], visited: HashSet::new(), keys: path.keys.clone() };
                                new_path.keys.push(c);
                                new_paths.push(new_path);
                            }
                            // Simple floor or a door or a owned key.
                            else if c == FLOOR_SYMBOL || c == START_SYMBOL || c.is_ascii_lowercase() || c.is_ascii_uppercase() && path.keys.contains(&c.to_ascii_lowercase()) {
                                path.to_visit.push(adjacent);
                            }
                        }
                    }
                }

                if path.to_visit.is_empty() {
                    paths.remove(i);
                }
            }

            paths.append(&mut new_paths);
        }
    }
}

pub fn nb_steps_to_collect_all_key(vault: &Vault) -> u32 {
    v2::nb_steps_to_collect_all_key(vault)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample1() {
        let input =
            "#########
             #b.A.@.a#
             #########";

        let vault = Vault::parse(input);

        let steps_v1 = v1::nb_steps_to_collect_all_key(&vault);
        let steps_v2 = v2::nb_steps_to_collect_all_key(&vault);

        assert_eq!(steps_v1, steps_v2);

        println!("Steps: {}", steps_v1);
    }

    #[test]
    fn part1_sample2() {
        let input =
            "########################
             #f.D.E.e.C.b.A.@.a.B.c.#
             ######################.#
             #d.....................#
             ########################";

        let vault = Vault::parse(input);

        let steps_v1 = v1::nb_steps_to_collect_all_key(&vault);
        let steps_v2 = v2::nb_steps_to_collect_all_key(&vault);

        assert_eq!(steps_v1, steps_v2);

        println!("Steps: {}", steps_v1);
    }

    #[test]
    fn part1_sample3() {
        let input =
            "########################
             #...............b.C.D.f#
             #.######################
             #.....@.a.B.c.d.A.e.F.g#
             ########################";

        let vault = Vault::parse(input);

        let steps_v1 = v1::nb_steps_to_collect_all_key(&vault);
        let steps_v2 = v2::nb_steps_to_collect_all_key(&vault);

        assert_eq!(steps_v1, steps_v2);

        println!("Steps: {}", steps_v1);
    }

    #[test]
    fn part1_sample4() {
        let input =
            "#################
             #i.G..c...e..H.p#
             ########.########
             #j.A..b...f..D.o#
             ########@########
             #k.E..a...g..B.n#
             ########.########
             #l.F..d...h..C.m#
             #################";

        let vault = Vault::parse(input);

        //let steps_v1 = v1::nb_steps_to_collect_all_key(&vault);
        let steps_v2 = v2::nb_steps_to_collect_all_key(&vault);

        //assert_eq!(steps_v1, steps_v2);

        println!("Steps: {}", steps_v2);
    }

    #[test]
    fn part1_sample4b() {
        let input =
            "#################
             #j.A..b...a..D.o#
             ########@########
             #k.E..e...d..B.n#
             #################";

        let vault = Vault::parse(input);

        //let steps_v1 = v1::nb_steps_to_collect_all_key(&vault);
        let steps_v2 = v2::nb_steps_to_collect_all_key(&vault);

        //assert_eq!(steps_v1, steps_v2);

        println!("Steps: {}", steps_v2);
    }

    #[test]
    fn part1_sample5() {
        let input =
            "########################
             #@..............ac.GI.b#
             ###d#e#f################
             ###A#B#C################
             ###g#h#i################
             ########################";

        let vault = Vault::parse(input);

        let steps_v1 = v1::nb_steps_to_collect_all_key(&vault);
        let steps_v2 = v2::nb_steps_to_collect_all_key(&vault);

        assert_eq!(steps_v1, steps_v2);

        println!("Steps: {}", steps_v1);
    }
}