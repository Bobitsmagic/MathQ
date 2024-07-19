use std::{fs::File, io::Write, process::Command};

fn create_type_document(equation: &str) -> String {
    format!("#set page(width: auto, height: auto, margin: 0cm);\n$ {} $", equation)
}

pub fn show_equation(equation: &str) {
    let document = create_type_document(equation);

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