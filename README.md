# h(acks in)r(u)s(t)
For when you just need to hack together a quick project for testing.  
Use `--temp` to make it entirely throwaway, `--hack` for small hacky testing projects, or without either flag to add it to your default projects folder.

```
Usage: hrs [OPTIONS] <name>

Arguments:
  <name>  The name of the project

Options:
  -t, --temp  Create the project in the OS's temporary directory
  -k, --hack  Create the project in your folder for small test projects
  -h, --help  Print help
```

## Installation
`cargo install hrs`

## Tips for using
This spits out the directory of the project it creates. Get started even quicker with `cd $(hrs --temp parser_testing)`, or a shell script like this:  
```sh
# rproj
#!/bin/sh
directory=$(hrs $@)
if [[ $? -eq 0 ]]; then
    cd $directory
    vim src/main.rs
fi
```
This allows you to call `. rproj --temp parser_testing` to immediately jump to your project directory and start editing `src/main.rs`.  
Since changing directory is finicky, I am fairly certain that you have to use `. rproj` instead of `rproj`, so it might make sense to alias this!
