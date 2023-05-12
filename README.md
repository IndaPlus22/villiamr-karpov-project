# TODOS into Issues

This Github action will turn all your "TODO" comments into a issue in your Github repository. 

## How to use
It is simple! Just add this action to your Github repository. The action will each time you commit a new change look for TODOS and turn them into a issue. Does not get easier than that.

```yml
name: Todo to issue

on:
    pull_request:
        branches: ["main"]

jobs:
    todo-issues:
        runs-on: ubuntu-latest
        steps:
            - uses: actions/checkout@v3
            - uses: IndaPlus22/villiamr-krapov-project@main
```

## Notes
The action will for now only look for todos on the main branch. Other branches will be supported soon.  



