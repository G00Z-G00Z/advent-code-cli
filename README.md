# Advent of code CLI

This cli will help you manage your advent calendar code using templates in yml
file.

# Format YML file

A `templates.yml` file will be generated like this

```yml
language: rust
    commands:
        - cargo new {{project_name}}
    files:
        - name: .env
          content: |
             # Demo 
             DEMO_APP=1
        - src/lib.rs
        - input.txt
        - demo-input.txt
    folders: 
        - docs/
```
