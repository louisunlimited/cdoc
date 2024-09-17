use clap::Parser;
use std::fs::File;
use std::io::Write;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// title of homework
    #[arg(short, long)]
    title: String,

    /// Course Number
    #[arg(short, long)]
    course: String,

    /// No. of questions
    #[arg(short, long)]
    questions: u8,
}

fn main() {
    let args = Args::parse();

    let latex_file = generate_latex_file(&args.title, &args.course, args.questions);

    let course_parts: Vec<&str> = args.course.split(":").collect();
    let course_prefix = course_parts[0];

    let formatted_title = args.title.replace(" ", "");

    write_to_file(&format!("{}_{}.tex", course_prefix, formatted_title), &latex_file);
}

fn generate_latex_file(title: &str, course: &str, length: u8) -> String {
    let environment = format!(
        r#"\documentclass[12pt,a4paper]{{article}}
\usepackage{{enumitem}}
\usepackage{{array}}

% Define the custom environment with optional argument for sub-question style
\newcounter{{question}}
\newenvironment{{questions}}[2][\alph]
{{%
  \setcounter{{question}}{{#2}}
  \renewcommand{{\labelenumii}}{{#1{{enumii}})}}
  % Apply bold and different font to the main question text
  \noindent \textbf{{\bfseries Question \thequestion.}}%
  \begin{{enumerate}}[label=#1*)]
}}
{{%
  \end{{enumerate}}
}}

\newcommand{{\question}}{{\item}}"#
    );

    let heading = format!(
        r#"\title{{{}}}
\author{{Louis Qian \\ {}}}
\maketitle"#,
        title,
        course
    );

    let mut questions = String::new();

    for i in 1..=length {
        questions.push_str(&format!(
            r#"\begin{{questions}}{{{}}}
    \question
    Type something here...
\end{{questions}}
"#,
            i
        ));
    }

    let template = format!(
        r#"{}

\begin{{document}}

{}

{}

\end{{document}}"#,
        environment, heading, questions
    );

    return template;
}

fn write_to_file(file_name: &str, content: &str) {
    let mut file = File::create(file_name).expect("Unable to create file");
    file.write_all(content.as_bytes())
        .expect("Unable to write data to file");
}