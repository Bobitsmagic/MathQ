use std::{fs::File, io::Write, process::Command, thread};

fn create_type_document(equation: Vec<String>) -> String {
    format!("#set page(width: auto, height: auto, margin: 0cm);\n{}", equation.iter().map(|eq| format!("$ {} $", eq)).collect::<Vec<_>>().join("\n"))
}

pub fn show_equation(equation: String) {
    let document = create_type_document(vec![equation]);

    create_image(&document);
}

pub fn show_equations(equations: Vec<String>) {
    let document = create_type_document(equations);
    create_image(&document);
}

pub fn render_graph(equation: &str, graph: &str) {
    let start_string = "#import \"@preview/diagraph:0.2.5\": *\n#set page(width: auto, height: auto, margin: 0cm);\n";

    let middle_string = "#raw-render(```\ndigraph {\ngraph[splines=line] \nnode[shape=circle]  
edge [arrowhead=none] rankdir=LR \n";
    let end_string = "}```)\n";
    
    let document = format!("{} $ {} $ {} {} {}", start_string, equation, middle_string, graph, end_string);
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

    thread::sleep(std::time::Duration::from_secs(1));
}