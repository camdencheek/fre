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

## TODO 

- [] Implement automatic re-scaling to prevent overflow
- [] Investigate supporting multiple half-lives for more flexible decay rates
- [] Figure out how to get Serde to serialize/deserialize with rc pointers
