# fe

`fe` is a CLI tool for tracking your most-used directories and files. 
Though inspired by tools like `autojump` or the `z` plugin for `zsh`, it takes a slightly 
different approach to tracking and providing usage data. 
The primary difference is `fe` does not actually support jumping. 
Instead, it just keeps track of and provides sorting methods for directories, 
which can then be filtered by another application like `fzf`, 
which does a much better job of filtering than something I can write.  


## Usage

`fe` is primarily designed to interface with `fzf`. For general usage, 
a user will create a shell hook that adds a directory every time the current 
directory is changed. This will start to build your profile of most-used directories. 
Then, `fe` can be used as a source for `fzf`. I personally use the `fzf`-provided 
control-T bindings, modified to use `fe` as input. Some examples are below.

Basic usage
```sh
fe --sorted | fzf
```

For integration with `fzf` CTRL-T, define the following environment variables 
```zsh
export FZF_CTRL_T_COMMAND='command fe --sorted'
export FZF_CTRL_T_OPTS='--tiebreak=index'
```

To preferentially use results from fe, but fall back to other results, we can use 
`cat` to combine results before sending them to `fzf`. My favorite alternate source 
is `fd` ([link](https://github.com/sharkdp/fd)), but the more common `find` can also be 
used. The following options first use `fe` results, then use all the subdirectories 
of the current directory, then use every subdirectory in your home directory. 
This is what I personally use.

```zsh
export FZF_CTRL_T_COMMAND='command cat <(fe --sorted) <(fd -t d) <(fd -t d . ~)'
export FZF_CTRL_T_OPTS='--tiebreak=index'
```

### Shell integration

#### zsh
(credit to `fzf`)

```zsh
fe_chpwd() {
  fe --add "$(pwd)"
}
typeset -gaU chpwd_functions
chpwd_functions+=fe_chpwd
```

More shells to come


## TODO 

- [ ] Investigate using `fe` as a source for tab completions à la `z`
- [ ] Investigate supporting multiple half-lives for more flexible decay rates
- [ ] Investigate accepting paths from stdin to sort
- [ ] Implement auto-resetting the reference time whenever a large number of half lives has passed

## OTHER

Interesting reading: https://ieeexplore.ieee.org/document/970573
