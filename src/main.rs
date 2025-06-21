use std::io;
use genpdf::{elements, Alignment, Document};

fn calculate_average(total: f32, subjects: u32) -> f32 {
    total / subjects as f32
}

fn assign_grade(avg: f32) -> &'static str {
    if avg >= 90.0 {
        "A"
    } else if avg >= 75.0 {
        "B"
    } else if avg >= 60.0 {
        "C"
    } else {
        "D"
    }
}

fn main() {
    let mut name = String::new();
    let mut total_marks = String::new();
    let mut num_subjects = String::new();

    println!("Enter student name:");
    io::stdin().read_line(&mut name).unwrap();

    println!("Enter total marks:");
    io::stdin().read_line(&mut total_marks).unwrap();
    let total_marks: f32 = total_marks.trim().parse().unwrap();

    println!("Enter number of subjects:");
    io::stdin().read_line(&mut num_subjects).unwrap();
    let num_subjects: u32 = num_subjects.trim().parse().unwrap();

    let average = calculate_average(total_marks, num_subjects);
    let grade = assign_grade(average);

    // Load font
    let font_family = genpdf::fonts::from_files("./fonts", "LiberationSans", None)
        .expect("Failed to load font family");

    let mut doc = Document::new(font_family);
    doc.set_title("Student Report Card");

    let mut decorator = genpdf::SimplePageDecorator::new();
    decorator.set_margins(10);
    doc.set_page_decorator(decorator);

    let mut layout = elements::LinearLayout::vertical();
    layout.push(elements::Paragraph::new(format!("Name: {}", name.trim())));
    layout.push(elements::Paragraph::new(format!("Total Marks: {}", total_marks)));
    layout.push(elements::Paragraph::new(format!("Subjects: {}", num_subjects)));
    layout.push(elements::Paragraph::new(format!("Average: {:.2}", average)));
    layout.push(elements::Paragraph::new(format!("Grade: {}", grade)));

    doc.push(layout);
    doc.render_to_file("report_card.pdf").expect("Failed to write PDF");

    println!("\n✅ Report card saved as `report_card.pdf`");
}
