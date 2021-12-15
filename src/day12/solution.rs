use std::collections::HashMap;

#[derive(Debug)]
struct Node {
    children: Vec<String>,
    name: String,
}

fn parse_graph(input: String) -> HashMap<String, Node> {
    let mut nodes: HashMap<String, Node> = HashMap::new();
    input
        .lines()
        .for_each(|connection| {
            let path = connection.split('-').collect::<Vec<&str>>();
            if !nodes.contains_key(path[0]) {
                nodes.insert(path[0].to_string(), Node {
                    children: Vec::new(),
                    name: path[0].to_string(),
                });
            }
            if !nodes.contains_key(path[1]) {
                nodes.insert(path[1].to_string(), Node {
                    children: Vec::new(),
                    name: path[1].to_string(),
                });
            }
            nodes.get_mut(path[0]).unwrap().children.push(path[1].to_string());
            nodes.get_mut(path[1]).unwrap().children.push(path[0].to_string());
        });
    nodes
}

fn find_paths(map: HashMap<String, Node>, can_double_small_cave: bool) -> i64 {
    let mut visited: HashMap<String, bool> = HashMap::new();
    visited.insert("start".to_string(), true);
    let mut paths = 0;
    let mut curr_path = vec!["start".to_string()];
    let mut children_indices = vec![0];
    let mut doubled_small: String = "".to_string();
    'main_loop: loop {
        // Perform a modified DFS to find the next full path
        if curr_path.is_empty() {
            // Emptied the queue
            break;
        }
        loop {
            // Find a valid child to visit
            if children_indices[children_indices.len() - 1]
                >= map.get(&*curr_path.last().unwrap()).unwrap().children.len() {
                // Current path end has been fully explored, back up and continue elsewhere
                children_indices.pop();
                let bail_out = curr_path.pop().unwrap();
                if bail_out == doubled_small {
                    doubled_small = "".to_string();
                } else {

                    visited.insert(bail_out, false);
                }
                continue 'main_loop;
            }

            let next = map
                .get(&*curr_path.last().unwrap())
                .unwrap()
                .children[children_indices[children_indices.len() - 1]]
                .clone();
            let is_small = next.to_lowercase() == next;
            if next.to_lowercase() == "end" {
                // Full path found
                paths += 1;
            } else if !visited.contains_key(&*next) {
                // We found a fresh cave
                visited.insert(next.to_string(), false);
                break;
            } else if !is_small || !(*(visited.get(&*next).unwrap())) {
                // Either a big cave or an unvisited small cave
                break;
            } else if can_double_small_cave && is_small && doubled_small.is_empty() && next != *"start" {
                // We found a small cave that we can double
                doubled_small = next.to_string();
                break;
            }
            let new_index = children_indices.pop().unwrap() + 1;
            children_indices.push(new_index);
        }

        let next = map
            .get(&*curr_path.last().unwrap())
            .unwrap()
            .children[children_indices[children_indices.len() - 1]]
            .clone();

        // Increment since this path has been used now
        let new_index = children_indices.pop().unwrap() + 1;
        children_indices.push(new_index);

        curr_path.push(next.clone());
        children_indices.push(0);
        visited.insert(next.to_string(), true);
    }
    paths as i64
}

pub fn part1(input: String) -> i64 {
    find_paths(parse_graph(input), false)
}

pub fn part2(input: String) -> i64 {
    find_paths(parse_graph(input), true)
}
