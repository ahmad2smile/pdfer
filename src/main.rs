use lopdf::content::Operation;
use lopdf::Document;
use lopdf::Object;

use std::fs;
use std::path;

pub fn add_header(header_text: &str, name: &str, src_path: &str, dest_path: &str) {
    let file = format!("{src_path}/{name}");
    let file = path::Path::new(&file);

    let doc_mem = fs::read(file.as_os_str()).unwrap_or(vec![]);

    let mut doc = Document::load_mem(&doc_mem).unwrap_or_default();

    let page_id = *doc.get_pages().get(&1).unwrap_or(&(0, 0));

    let mut page_content = doc.get_and_decode_page_content(page_id).unwrap();
    let default_op = Operation::new("Tf", vec![]);
    let binding = page_content
        .operations
        .clone()
        .into_iter()
        .filter(|o| o.operator == "Tf")
        .collect::<Vec<Operation>>();

    let current_font = binding
        .first()
        .unwrap_or(&default_op)
        .operands
        .first()
        .unwrap()
        .as_name_str()
        .unwrap();

    let mut operations = vec![
        Operation::new("BT", vec![]),
        Operation::new("Tf", vec![current_font.into(), 12.into()]),
        Operation::new("Td", vec![20.into(), 770.into()]),
        Operation::new("Tj", vec![Object::string_literal(header_text)]),
        Operation::new("ET", vec![]),
    ];

    page_content.operations.append(&mut operations);

    doc.change_page_content(page_id, page_content.encode().unwrap())
        .unwrap();

    doc.adjust_zero_pages();

    let _ = fs::create_dir_all(dest_path);
    let dest_file = format!("{dest_path}/{name}");
    let dest_file = path::Path::new(&dest_file);

    doc.save(dest_file).unwrap();

    println!("Processed file: {name}");
}

fn main() {
    add_header(
        "Pdfer Here...........",
        "sample_v13.pdf",
        "./",
        "./generated/",
    );
    add_header(
        "Pdfer Here...........",
        "sample_v14.pdf",
        "./",
        "./generated/",
    );
    add_header(
        "Pdfer Here...........",
        "sample_v15.pdf",
        "./",
        "./generated/",
    );
    // TODO: Fix this
    // add_header(
    //     "Pdfer Here...........",
    //     "sample_v17.pdf",
    //     "./",
    //     "./generated/",
    // );
    println!("Hello, world!");
}
