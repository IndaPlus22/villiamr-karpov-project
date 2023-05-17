# TODOS into Issues

This Github action will turn all your "TODO" comments into an issue tagged `TODO` in your Github repository as well as tag duplicate issues with a "duplicate" or "duplicate title" tag.
The goal is to assist developers in keeping track of what is to be done by organizing all todos in an easy to find place instead of having to search through the code. 

## How to use

It is simple! Just add this action to your Github repository. The action will each time you commit a new change look for TODOS and turn them into an issue. Does not get easier than that.

```yml
name: Todo to issue

on:
    push:
        branches: ["main"]

jobs:
    todo-issues:
        runs-on: ubuntu-latest
        steps:
            - uses: actions/checkout@v3
            - uses: IndaPlus22/villiamr-krapov-project@alpha
```

Comments inside your code should follow the examples below in order to be converted into issues:

```C
//TODO: This is the title of the created issue
//Here you can add a body describing the issue.
//The body will continue until a line missing a comment is reached.

//tODo: Capitalization does not matter and this will also become an issue.
```
### IMPORTANT NOTE

This action is currently private and can only be used on this repository. Hence, using the yml above does not actually work on other repositories. In order to use this action it has to be 
published to `Github marketplace` which requires permissions from a `IndaPlus22` administrator. To test run the action you may go to `Actions->Integration-test` and run the workflow on this repository. If you want an issue to be created feel free to first close an existing issue tagged `TODO`, preforably one whithout any comments or useful human input. 

## Supported languages

Currently, we do not offer support for all languages. The supported languages are:

* C
* Cpp
* Rust

Soon to be supported languages include:

* Java
* JavaScript
* Go
* PHP
* Kotlin

And more! All languages using c-style comments could theoretically already be supported but are untested. 









