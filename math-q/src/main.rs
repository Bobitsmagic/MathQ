use std::fmt::format;

use math_q::{math_types::{math_type::{self, MathType}, math_type_name::MathTypeName}, typst_api};


fn main() {
    let layer_count = vec![2, 2, 1];

    generate_sub_paths(layer_count.clone());

    let mut start_layer = Vec::new();
    for i in 0..layer_count[0] {
        start_layer.push(math_type::variable(&format!("x_{}", i)));
    }

    let mut layers = vec![start_layer];

    for depth in 1..layer_count.len() {
        let mut layer = Vec::new();

        for i in 0..layer_count[depth] {
            let mut sum = Vec::new();
            for j in 0..layer_count[depth - 1] {
                sum.push(math_type::product(vec![layers[depth - 1][j].clone(), math_type::variable(&format!("w^({})_({}{})", depth, j, i))]));
            }
            layer.push(math_type::function("f", vec![math_type::sum(sum)]));
        }
        layers.push(layer);
    }
    
    let term = layers.last().unwrap().first().unwrap().clone();
    // let mut weight_count = 0;
    // for layer_index in 1..layer_count.len() {
    //     for right in 0..layer_count[layer_index] {
    //         for left in 0..layer_count[layer_index - 1] {
    //             term = term.replace(&math_type::variable(&format!("w^({})_({}{})", layer_index, left, right)), &math_type::variable(&format!("w_{}", weight_count)));
    //             weight_count += 1;
    //         }
    //     }
    // }

    let mut vec = vec![term.clone()];
    
    println!("Created network function {}", term.count_nodes());
    let var1 = math_type::variable("w^(1)_(00)");
    let var2 = math_type::variable("w^(1)_(10)");
    let deriv = term.get_derivative(&var1).get_derivative(&var2);
    
    println!("Finished derivative {}", deriv.count_nodes());
    // vec.push(deriv.clone());

    // println!("{}", deriv.get_string());
    
    let mut sorted = deriv.clone();
    sorted.sort();
    
    let mut prev = sorted.clone();

    for layer_index in (1..layer_count.len()).rev() {
        for node_index in 0..layer_count[layer_index] {
            let mut val = layers[layer_index][node_index].clone();
            val.sort();

            let deri = math_type::function("f'", val.parameter.clone());
            prev = prev.replace(&deri, &math_type::variable(&format!("delta^({})_({})", layer_index, node_index)));
        }

        println!("Finished delta replacement {}", layer_index);
    }

    println!("Finished delta replacement");

    prev.sort();
    for layer_index in (1..layer_count.len()).rev() {
        for node_index in 0..layer_count[layer_index] {
            let mut val = layers[layer_index][node_index].clone();
            val.sort();

            let deri = math_type::function("f''", val.parameter.clone());
            prev = prev.replace(&deri, &math_type::variable(&format!("Delta^({})_({})", layer_index, node_index)));
            
            // vec.push(prev.clone());
        }
    }

    println!("Finished Delta replacement");

    prev.sort();
    for layer_index in (1..layer_count.len()).rev() {
        for node_index in 0..layer_count[layer_index] {
            layers[layer_index][node_index].sort();

            prev = prev.replace(&layers[layer_index][node_index], &math_type::variable(&format!("n^({})_({})", layer_index, node_index)));
            
            // vec.push(prev.clone());
        }
    }

    println!("Finished n replacement");

    prev = prev.simplify();
    
    println!("{}", prev.count_nodes());

    vec.push(prev.clone());

    // let mut index = 0;
    // for depth in 1..layer_count.len() {
    //     for right in 0..layer_count[depth] {
    //         for left in 0..layer_count[depth - 1] {
    //             let w = math_type::variable(&format!("w^({})_({}{})", depth, left, right));
    //             let w_n = math_type::variable(&format!("w_{}", index));

    //             println!("Replacing {} with {}", w.get_string(), w_n.get_string());

    //             for i in 0..vec.len() {
    //                 vec[i] = vec[i].replace(&w, &w_n);
    //             }

    //             index += 1;
    //         }
    //     }
    // }

    typst_api::show_equations(vec.iter().map(|t| t.get_typst_string()).collect());

    println!("Trace count {}", vec.last().unwrap().parameter.len());

    if !matches!(vec.last().unwrap().type_name, MathTypeName::Sum) {
        vec = vec![math_type::sum(vec![vec.last().unwrap().clone()])];
    }

    generate_graphs(vec, layer_count, var1, var2);
}

fn generate_sub_paths(layer_count: Vec<usize>) {
    let mut d1_paths = Vec::new();
    let mut d2_paths = Vec::new();

    for depth in 0..layer_count.len() {
        d1_paths.push(vec![vec![math_type::sum(vec![]); layer_count[depth]]; layer_count[depth]]);
    }

    d2_paths = d1_paths.clone();

    // d1_paths[layer_count.len() - 1][0][0].parameter.push(delta_var(layer_count.len() - 1, 0));
    // d2_paths[layer_count.len() - 1][0][0].parameter.push(Delta_var(layer_count.len() - 1, 0));
    
    for depth in (1..layer_count.len()).rev() {
        for right_0 in 0..layer_count[depth] {
            for right_1 in 0..layer_count[depth] {
                let right_path_d1 = math_type::sum(vec![delta_var(depth, right_0), delta_var(depth, right_1), d1_paths[depth][right_0][right_1].clone()]);
                let mut right_path_d2 = math_type::sum(vec![delta_var(depth, right_0), delta_var(depth, right_1), d2_paths[depth][right_0][right_1].clone()]);
                
                if right_0 == right_1 {
                    //factor 2?
                    right_path_d2 = math_type::sum(vec![Delta_var(depth, right_0), d1_paths[depth][right_0][right_1].clone()]);
                }
                
                for left_0 in 0..layer_count[depth - 1] {
                    for left_1 in 0..layer_count[depth - 1] {
                        let w_0 = weight_var(depth, left_0, right_0);
                        let w_1 = weight_var(depth, left_1, right_1);

                        d1_paths[depth - 1][left_0][left_1].parameter.push(math_type::product(vec![w_0.clone(), w_1.clone(), right_path_d1.clone()]));
                        d2_paths[depth - 1][left_0][left_1].parameter.push(math_type::product(vec![w_0.clone(), w_1.clone(), right_path_d2.clone()]));
                    }
                }
            }
        }
    }

    let mut vec = Vec::new();
    for depth in layer_count.len() - 2 .. layer_count.len() - 1 {
        for right_0 in 0..layer_count[depth] {
            for right_1 in 0..layer_count[depth] {
                // vec.push(d1_paths[depth][right_0][right_1].simplify());
                vec.push(d2_paths[depth][right_0][right_1].simplify());

                println!("{}", d2_paths[depth][right_0][right_1].simplify().get_string())

            }
        }
    }
    
    typst_api::show_equations(vec.iter().map(|t| t.get_typst_string()).collect());

    fn delta_var(layer_index: usize, node_index: usize) -> MathType {
        return math_type::variable(&format!("delta^({})_({})", layer_index, node_index));
    }
    fn Delta_var(layer_index: usize, node_index: usize) -> MathType {
        return math_type::variable(&format!("Delta^({})_({})", layer_index, node_index));
    }
    fn weight_var(layer_index: usize, left: usize, right: usize) -> MathType {
        return math_type::variable(&format!("w^({})_({}{})", layer_index, left, right));
    }
}

fn generate_graphs(vec: Vec<MathType>, layer_count: Vec<usize>, var1: MathType, var2: MathType) {
    for first_sum in vec.last().unwrap().parameter.iter() {
    let mut used_weights = Vec::new();
    let mut used_n = Vec::new();
    let mut used_delta = Vec::new();
    let mut used_Delta = Vec::new();

    // println!("Function: {}", first_sum.get_typst_string());


    for m in &first_sum.parameter {
        // println!("Parameter: {}", m.get_string());

        if m.get_string().contains("w") {
            used_weights.push(weight_index_from_string(&m.get_string()));
        }
        if m.get_string().contains("n") {
            used_n.push(index_from_string(&m.get_string()));
        }
        if m.get_string().contains("x") {
            used_n.push((0, m.get_string().split("_").last().unwrap().parse().unwrap()));
        }
        if m.get_string().contains("delta") {
            used_delta.push(index_from_string(&m.get_string()));
        }
        if m.get_string().contains("Delta") {
            used_Delta.push(index_from_string(&m.get_string()));
        }

        fn weight_index_from_string(s: &str) -> (usize, usize, usize) {
            let trim = s.replace("(", "").replace(")", "");
            let kek = trim.split("^").last().unwrap().split("_").collect::<Vec<_>>();



            let chars = kek[1].chars().map(|c| c.to_string()).collect::<Vec<_>>();
        

            return (kek[0].parse().unwrap(), chars[0].parse().unwrap(), chars[1].parse().unwrap());
        }

        fn index_from_string(s: &str) -> (usize, usize) {
            let trim = s.replace("(", "").replace(")", "");
            let kek = trim.split("^").last().unwrap().split("_").collect::<Vec<_>>();

            return (kek[0].parse().unwrap(), kek[1].parse().unwrap());
        }
    }

    // used_Delta.clear();
    // used_delta.clear();
    // used_weights.clear();
    // used_n.clear();

    let mut s = "".to_string();

    s += "subgraph cluster_0{\n";
    for i in (0..layer_count[0]).rev() {
    
        let format = if used_n.contains(&(0, i)) {
            "[style=filled, color=red]"
        } else {
            ""
        };

        s.push_str(&format!("\tx_{} {}\n", i, format));
    }

    s+= "}\n";

    for layer_index in 1..layer_count.len() {
        s += &format!("subgraph cluster_{}{{\n", layer_index);
        for node_index in (0..layer_count[layer_index]).rev() {
            let format = if used_n.contains(&(layer_index, node_index)) {
                "[style=filled, color=red]"
            } else {
                if used_Delta.contains(&(layer_index, node_index)) {
                    "[style=filled, color=blue]"
                } else {
                    if used_delta.contains(&(layer_index, node_index)) {
                        "[style=filled, color=green]"
                    } else {
                        ""
                    }
                }
            };

            s.push_str(&format!("\t\"n^({})_{}\" {}\n", layer_index, node_index, format));
        }
        s += "}\n";
    }

    for layer_index in 1..layer_count.len() {
        s += &format!("subgraph cluster_{}{{\n", layer_index);
        for right in (0..layer_count[layer_index]).rev() {
            for left in (0..layer_count[layer_index - 1]).rev() {
                let n_left = &if layer_index - 1 == 0 {
                    format!("\"x_{}\"", left)
                }
                else {
                    format!("\"n^({})_{}\"", layer_index - 1, left)
                };

                let format = if var1.get_typst_string() == format!("w^({})_({}{})", layer_index, left, right) || var2.get_typst_string() == format!("w^({})_({}{})", layer_index, left, right) {
                    "[color=purple]"
                } else {
                    if used_weights.contains(&(layer_index, left, right)) {
                        "[color=red]"
                    } else {
                        ""
                    }
                };
            

            
                let n_right = &format!("\"n^({})_{}\"", layer_index, right);

                s.push_str(&format!("\t{} -> {} {}\n", n_left, n_right, format));
            }
        }
        s += "}\n";
    }

    // println!("{s}");

    // println!("{:?}", used_n);

    typst_api::render_graph(&first_sum.get_typst_string(), &s);
    }
}