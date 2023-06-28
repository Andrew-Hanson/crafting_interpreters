pub struct Report {
    line: i32,
    context: String,
    message: String,
}

pub fn handle_error(report: Report) {
    let line = report.line;
    let context = report.context;
    let message = report.message;
    report_error(line, "", &message);
}

fn report_error(line: i32, context: &str, message: &str) {
    println!("[line {line}] Error{context}: {message}");
}