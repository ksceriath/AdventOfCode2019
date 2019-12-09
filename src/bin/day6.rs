use std::collections::HashMap;
use std::fs;

fn gen_graph(input: String) -> (Vec<Vec<usize>>, HashMap<String, usize>) {
    input
        .split("\n")
        .map(|s| {
            s.split(")")
                .map(|z| String::from(z))
                .collect::<Vec<String>>()
        })
        .fold(
            (Vec::new(), HashMap::new()),
            |(mut orbiter_list, mut objects), list| {
                let c = String::from(&list[0]);
                let o = String::from(&list[1]);
                if !objects.contains_key(&c) {
                    orbiter_list.push(Vec::new());
                    objects.insert(c, orbiter_list.len() - 1);
                }
                if !objects.contains_key(&o) {
                    orbiter_list.push(Vec::new());
                    objects.insert(o, orbiter_list.len() - 1);
                }
                orbiter_list[*objects.get(&list[0]).unwrap()].push(*objects.get(&list[1]).unwrap());
                (orbiter_list, objects)
            },
        )
}

fn main() {
    let mut input = fs::read_to_string("resources/day6.input").unwrap();
    input.pop();

    let (orbiter_list, objects) = gen_graph(input);

    // println!("orbiter list : {:?}", orbiter_list);
    // println!("object : {:?}", objects);

    let total_edges = count_connections(&orbiter_list);
    println!("Total edges : {}", total_edges);

    let you = *objects.get("YOU").unwrap();
    let san = *objects.get("SAN").unwrap();
    let total_transfers = count_orbital_transfers(&orbiter_list, you, san);
    println!("Total transfers : {}", total_transfers);
}

enum moves {
    None,
    You(i32),
    San(i32),
    Transfers(i32),
}

fn count_orbital_transfers(adj_list: &Vec<Vec<usize>>, you: usize, san: usize) -> i32 {
    fn count_orbital_transfers_rec(
        adj_list: &Vec<Vec<usize>>,
        start: usize,
        visited: &mut Vec<bool>,
        you: usize,
        san: usize,
    ) -> moves {
        let list = &adj_list[start];
        if start == you {
            println!("YOU at {}", start);
            return moves::You(0);
        }
        if start == san {
            println!("SAN at {}", start);
            return moves::San(0);
        }
        let mut m = Vec::new();
        for &v in list {
            let x = count_orbital_transfers_rec(adj_list, v, visited, you, san);
            match x {
                moves::You(a) => {
                    println!("Y{} at {}", a, start);
                    m.push(x);
                }
                moves::San(a) => {
                    println!("S{} at {}", a, start);
                    m.push(x);
                }
                moves::Transfers(a) => {
                    println!("T{} at {}", a, start);
                    return x;
                }
                moves::None => (),
            }
            if m.len() == 2 {
                println!("Found S & Y at {}", start);
                return moves::Transfers(
                    (match m.pop().unwrap() {
                        moves::You(t) => t,
                        moves::San(t) => t,
                        _ => 0,
                    }) + (match m.pop().unwrap() {
                        moves::You(t) => t,
                        moves::San(t) => t,
                        _ => 0,
                    }),
                );
            }
        }
        visited[start] = true;
        if let Some(x) = m.pop() {
            match x {
                moves::San(a) => moves::San(a + 1),
                moves::You(a) => moves::You(a + 1),
                _ => moves::None,
            }
        } else {
            moves::None
        }
    }

    let mut visited = vec![false; adj_list.len()];
    for i in 0..adj_list.len() {
        if !visited[i] {
            match count_orbital_transfers_rec(adj_list, i, &mut visited, you, san) {
                moves::Transfers(x) => return x,
                _ => (),
            }
        }
    }
    -1
}

fn count_connections(adj_list: &Vec<Vec<usize>>) -> i32 {
    fn count_connections_rec(
        adj_list: &Vec<Vec<usize>>,
        start: usize,
        visited: &mut Vec<bool>,
    ) -> (i32, i32) {
        let list = &adj_list[start];
        let mut nodes_under = 1;
        let mut total_edges = 0;
        for &v in list {
            if !visited[v] {
                let (n, t) = count_connections_rec(adj_list, v, visited);
                nodes_under = nodes_under + n;
                total_edges = total_edges + t + n;
            } else {
                let (n, _) = count_connections_rec(adj_list, v, visited);
                nodes_under = nodes_under + n;
                total_edges = total_edges + n;
            }
        }
        visited[start] = true;
        (nodes_under, total_edges)
    }
    let mut visited = vec![false; adj_list.len()];
    let mut total_edges = 0;
    for i in 0..adj_list.len() {
        if !visited[i] {
            total_edges = total_edges + count_connections_rec(adj_list, i, &mut visited).1;
        }
    }
    total_edges
}
