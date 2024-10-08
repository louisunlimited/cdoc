# cdoc

A very tiny cli tool that helps me spin up empty latex homework templates to write homeworks on.

## Installation

With Homebrew:

```sh
brew tap louisunlimited/cdoc
brew install cdoc
```

## Usage

```sh
cdoc --help
```

```plaintext
Generate LaTeX template for homework!

Usage: cdoc [OPTIONS] --title <TITLE> --course <COURSE> --questions <QUESTIONS>

Options:
  -t, --title <TITLE>          Title of homework
  -c, --course <COURSE>        Course Number
  -a, --author <AUTHOR>        Optional Author's name
  -q, --questions <QUESTIONS>  Number of questions to gererate
  -h, --help                   Print help
  -V, --version                Print version
```

## Configuration

This tool reads optional configuration from a file called `~/.cdocrc` in the `$HOME` directory. If the file is not found, it will create a empty one for you. The configuration file is a file in [INI format](https://en.wikipedia.org/wiki/INI_file) that looks like this:

```ini
[Settings]
Author = "John Doe" # The author of the document, defaults to "John Doe"

# A list of abbreviations for courses that you are taking
[courses]
CS433 = "CS433: Computer System Organization"
CS412 = "CS412: Introduction to Data Mining"
CS425 = "CS425: Distributed Systems"
CS101 = "CS101: Introduction to Programming"
```

## TODO

(These are in no particular order)

- [ ] Move away from string literals to a template file
- [ ] Add a way to specify the output directory
- [x] Make it be able to read configuation from ~/.cdocrc
- [ ] Automate CI/CD
- [ ] Standardize config reading & handling
