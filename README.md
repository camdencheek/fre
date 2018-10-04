# topd

`topd` is a CLI tool for tracking your most-used directories and files. 


## Usage

### Shell integration

#### `zsh`

```zsh
topd_chpwd() {
  topd --add "$(pwd)"
}
typeset -gaU chpwd_functions
chpwd_functions+=topd_chpwd
```
