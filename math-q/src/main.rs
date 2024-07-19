use math_q::{math_types::{math_type::{self, MathType}, math_type_name::MathTypeName}, typst_api};


fn main() {
    let delta_var = math_type::variable("x");

    let term = math_type::sum(vec![
        math_type::product(vec![
            math_type::natural_number(2),
            delta_var.clone(),
            math_type::sum(vec![
                delta_var.clone(),
                math_type::natural_number(3)
            ]),
        ]),
    ]);

    
    // let term = math_type::sum(vec![
        //     math_type::product(vec![math_type::natural_number(2), delta_var.clone(), delta_var.clone()]),
        //     math_type::product(vec![math_type::natural_number(6), delta_var.clone()]),
        // ]);
        
    println!("{}", term.get_string());
    let deriv = term.get_derivative(&delta_var);
    println!("{}", deriv.get_string());
    
    let mut sorted = deriv.clone();
    sorted.sort();
    
    let reduced = sorted.reduce_neutral();
    
    let expaned = reduced.expand();
    
    println!("Calculating derivative of:");
    let end = expaned.reduce_neutral().flatten().combine();
    // let end = expaned.reduce_neutral().flatten().reduce_neutral().combine().flatten().reduce_neutral();
    
    
    
    //product.sort();
    // let typst_string = product.get_typst_string();
    println!("{}", expaned.get_string());
    
    typst_api::show_equations(vec![&term.get_typst_string(), &deriv.get_typst_string(), &sorted.get_typst_string(), &reduced.get_typst_string(), &expaned.get_typst_string(), &end.get_typst_string()]);
}