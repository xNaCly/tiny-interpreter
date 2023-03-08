use crate::logger::log;

pub fn file(file_in: &str, file_out: &str) {
    if file_in.is_empty() {
        log().error("no input file specified");
    } else if file_out.is_empty() {
        log().error("no output file specified");
    } else if file_in == file_out {
        log().error("input and output file are the same, ti would override the input file");
    } else {
        log().debug(&format!("input file: {}", file_in));
        log().debug(&format!("output file: {}", file_out));
    }

    // TODO: read lines from file_in
    // TODO: lex lines
    // TODO: write lines to file_out
    log().error("file mode not implemented yet");
}
