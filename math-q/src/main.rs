use math_q::{math_types::{math_type::MathType, math_type_name::MathTypeName}, typst_api};


fn main() {

    let left = MathType::new(MathTypeName::Variable("y".to_string()), vec![]);
    let right = MathType::new(MathTypeName::NaturalNumber(2), vec![]);
    let sum = MathType::new(MathTypeName::Product, vec![right, left]);

    let factor = MathType::new(MathTypeName::Variable("x".to_string()), vec![]);
    let mut product = MathType::new(MathTypeName::Sum, vec![sum, factor]);

        
    let typst_string = product.get_typst_string();
    println!("{}", typst_string);
    product.sort();
    let typst_string = product.get_typst_string();
    println!("{}", typst_string);

    typst_api::show_equation(&typst_string);
}