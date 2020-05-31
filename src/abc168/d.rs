#[allow(unused_macros)]
macro_rules! parse_line {
    [$( $x:tt : $t:ty ),+] => {
        //declare variables
        $( let $x: $t; )+
        {
            use std::str::FromStr;
            // read
            let mut buf = String::new();
            std::io::stdin().read_line(&mut buf).unwrap();
            #[allow(deprecated)]
            let mut splits = buf.trim_right().split_whitespace();
            // assign each variable
            $(
                $x = splits.next().and_then(|s| <$t>::from_str(s).ok()).unwrap();
            )+
            // all strs should be used for assignment
            assert!(splits.next().is_none());
        }
    };
}

use std::collections::HashMap;

fn main() {
    parse_line![node_count: usize, edge_count: usize];

    let connections = {
        let mut temp = HashMap::with_capacity(node_count);

        for i in 1..node_count + 1 {
            temp.insert(i, vec![]);
        }

        for _ in 0..edge_count {
            parse_line![node_label1: usize, node_label2: usize];

            temp.get_mut(&node_label1).unwrap().push(node_label2);
            temp.get_mut(&node_label2).unwrap().push(node_label1);
        }

        temp
    };

    let depths = scan_all_node_minimum_depth(1, &connections);
    let mut guides = HashMap::with_capacity(node_count);

    search(1, &connections, &depths, &mut guides);

    let mut buffer = String::with_capacity(1000000);
    for i in 2..node_count + 1 {
        if let Some(to) = guides.get(&i) {
            buffer.push_str(&format!("{}\n", to));
        } else {
            println!("No",);
            return;
        }
    }
    println!("Yes");
    println!("{}", buffer);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Guide<N> {
    from: N,
    to: N,
}

fn scan_all_node_minimum_depth<N: Copy + Eq + std::hash::Hash>(
    root_node: N,
    connections: &HashMap<N, Vec<N>>,
) -> HashMap<N, usize> {
    let mut depths = HashMap::new();
    let mut current_depth = 0;

    depths.insert(root_node, current_depth);

    let mut scan_target_nodes = match connections.get(&root_node) {
        Some(connected_nodes) => connected_nodes.to_vec(),
        None => return HashMap::new(),
    };

    while !scan_target_nodes.is_empty() {
        current_depth += 1;

        let mut next_targets = vec![];

        for target in scan_target_nodes.into_iter() {
            if depths.get(&target).is_some() {
                continue;
            }
            depths.insert(target, current_depth);

            if let Some(connected_to_target) = connections.get(&target) {
                for &next_target in connected_to_target.into_iter() {
                    next_targets.push(next_target);
                }
            }
        }

        scan_target_nodes = next_targets;
    }

    depths
}

fn search<N: Copy + Eq + std::hash::Hash>(
    current_node: N,
    connections: &HashMap<N, Vec<N>>,
    node_depths: &HashMap<N, usize>,
    current_guides: &mut HashMap<N, N>,
) {
    for &connected_node in connections[&current_node]
        .iter()
        .filter(|n| node_depths[&current_node] + 1 == node_depths[n])
    {
        if current_guides.get(&connected_node).is_some() {
            continue;
        }
        current_guides.insert(connected_node, current_node);
        search(connected_node, connections, node_depths, current_guides);
    }
}
