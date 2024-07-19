use std::{fs::File, io::Write, process::Command};

fn create_type_document(equation: Vec<&str>) -> String {
    format!("#set page(width: auto, height: auto, margin: 0cm);\n{}", equation.iter().map(|eq| format!("$ {} $", eq)).collect::<Vec<_>>().join("\n"))
}

pub fn show_equation(equation: &str) {
    let document = create_type_document(vec![equation]);

    create_image(&document);
}

pub fn show_equations(equations: Vec<&str>) {
    let document = create_type_document(equations);
    create_image(&document);
}

fn create_image(document: &str) {
    let mut file = File::create("test.typ").expect("failed to create file");
    file.write_all(document.as_bytes()).expect("failed to write to file");
    file.flush().expect("failed to flush file");
    
    Command::new("typst")
        .args(["compile", "--ppi", "500" ,"test.typ", "test.png"])
        .output()
        .expect("failed to execute process");

    open::that("test.png").expect("failed to open image");
}