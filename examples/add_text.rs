use pdfer::text::add_header;

fn main() {
    let _ = add_header(
        "Pdfer Here...........",
        "sample_v13.pdf",
        "./",
        "./generated/",
    );

    // TODO: Not working
    let _ = add_header(
        "Pdfer Here...........",
        "sample_v14.pdf",
        "./",
        "./generated/",
    );
    let _ = add_header(
        "Pdfer Here...........",
        "sample_v15.pdf",
        "./",
        "./generated/",
    );
    let _ = add_header(
        "Pdfer Here...........",
        "sample_v17.pdf",
        "./",
        "./generated/",
    );
}
