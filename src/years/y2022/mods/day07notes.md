# notes

## required functions
### current dir
- indicated by commands like `cd dir_name`

### dir history
- keep track of dir path for commands like `cd ..`

### dir contents
- indicated by commands like `ls` which are following by dir contents
- files are preceeded by a number representing memory
- then something like `file_name.file_extension`
- dirs are preceeded by the `dir` keyword then something like `dir_name`

## how do we want to store all of this?
- recursive structure makes sense
- but will have to store flatly as a first step no matter what
- this can be a hashmap with
    - key = path
    - value = children
