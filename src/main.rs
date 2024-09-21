use clap::Parser;
use std::fs::File;
use std::io::Write;
use ini::Ini;
use std::path::PathBuf;
use std::collections::HashMap;
use dirs;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// title of homework
    #[arg(short, long)]
    title: String,

    /// Course Number
    #[arg(short, long)]
    course: String,

    /// Author
    #[arg(short, long)]
    author: Option<String>,

    /// No. of questions
    #[arg(short, long)]
    questions: u8,
}

fn main() {
    // Reads and loads the configuration file from home directory
    let config = load_config();

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

fn load_config() -> Ini {
    let home_dir = dirs::home_dir().expect("Could not find home directory");
    let mut config_path = PathBuf::from(home_dir);
    config_path.push(".cdocrc");

    // if not found, create one
    if !config_path.exists() {
        File::create(&config_path).expect("Could not create config file");
    }

    Ini::load_from_file(config_path).expect("Could not load config file")
}

fn generate_latex_file(title: &str, course: &str, author: &Option<String>,  length: u8, config: &Ini) -> String {
    let mut course_map = HashMap::new();
    if let Some(section) = config.section(Some("courses")) {
        for (key, value) in section.iter() {
            course_map.insert(key, value);
        }
    }

    let config_author = config.get_from(Some("Settings"), "Author").unwrap_or("John Doe");

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
\author{{{} \\ {}}}
\maketitle"#,
        title,
        author.clone().unwrap_or(config_author.to_string()),
        course_map.get(course).unwrap_or(&course)
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

