# cdoc

A very tiny cli tool that helps me spin up empty latex homework templates to write homeworks on.

## Configuration

This tool reads optional configuration from a file called `cdocrc` in the current directory. If the file is not found, it will create a empty one for you. The configuration file is a file in [INI format](https://en.wikipedia.org/wiki/INI_file) that looks like this:

```ini
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
- [ ] Make it be able to read configuation from ~/.cdocrc
- [ ] Automate CI/CD
