<p align="center">
    <img src="../static/images/geoffrey-logo.png" class="logo">
</p>

# geoff create

```shell
geoff create [OPTIONS] NAME
```

Creates a new project called `NAME` and creates the 4 sub directories that geoff manages along with a README and project scoping template.

## Arguments

`name`
The name of the project. This can be a project name or a path with the final directory in the path being the project name.

Project name

```shell
foo@bar:~$ geoff create test_project
```

Project with path

```shell

foo@bar:~$ geoff create path/to/test_project
```

If a path is supplied and the parents of the project name don't exist, the parents either need to be created manually or `--parents` needs to be supplied.

## Options

`-p, --parents`

Whether to create the parent directories in the project name

`--help`

Prints help information

## Examples

Create a project

```shell
foo@bar:~$ geoff create test_project
🚀 test_project created!

test_project
├── 🖿 data_sources
├── 🖿 explorations
├── 🖿 models
├── 🖿 products
├── 🗋 README.md
└── 🗋 project_scoping.md
```

Create a project and parents of specified path

```shell
foo@bar:~$ geoff create --parents path/to/test_project
🚀 test_project created!

test_project
├── 🖿 data_sources
├── 🖿 explorations
├── 🖿 models
├── 🖿 products
├── 🗋 README.md
└── 🗋 project_scoping.md
```
