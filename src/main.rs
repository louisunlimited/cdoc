mod config;

use clap::Parser;
use std::fs::File;
use std::io::Write;
use config::Config;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Title of homework
    #[arg(short, long)]
    title: String,

    /// Course Number
    #[arg(short, long)]
    course: String,

    /// Optional Author's name
    #[arg(short, long)]
    author: Option<String>,

    /// Number of questions to gererate
    #[arg(short, long)]
    questions: u8,
}

fn main() {
    // Reads and loads the configuration file from home directory
    let config = Config::load().expect("Error loading configuration");

    // Parses cli args
    let args = Args::parse();

    // Generates the latex file
    let latex_file = generate_latex_file(&args.title, &args.course, &args.author, args.questions, &config);

    // Writes the latex file to disk with the specified name
    write_to_file(
        &format!("{}_{}.tex", args.course, args.title),
        &latex_file,
    );
}

fn generate_latex_file(title: &str, course: &str, author: &Option<String>, length: u8, config: &Config) -> String {
    let environment = r#"\documentclass[12pt,a4paper]{article}
\usepackage{enumitem}
\usepackage{array}

% Define the custom environment with optional argument for sub-question style
\newcounter{question}
\newenvironment{questions}[2][\alph]
{%
  \setcounter{question}{#2}
  \renewcommand{\labelenumii}{#1{enumii})}
  \noindent \textbf{\bfseries Question \thequestion.}%
  \begin{enumerate}[label=#1*)]
}
{%
  \end{enumerate}
}

\newcommand{\question}{\item}"#.to_string();

    let config_author = &config.author;

    let heading = format!(
        r#"\title{{{}}}
\author{{{} \\ {}}}
\maketitle"#,
        title,
        author.clone().unwrap_or(config_author.to_string()),
        config.get_course_name(course)
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

    template
}

fn write_to_file(file_name: &str, content: &str) {
    let mut file = File::create(file_name).expect("Unable to create file");
    file.write_all(content.as_bytes())
        .expect("Unable to write data to file");
}
