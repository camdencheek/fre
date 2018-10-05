# topd

`topd` is a CLI tool for tracking your most-used directories and files. Though inspired by tools like `autojump` or the `z` plugin for `zsh`, it takes a slightly different to tracking and providing usage data. The primary difference is `topd` does not actually support jumping. Instead, it just keeps track of and provides sorting methods for directories, which can then be filtered by another application like `fzf`, which does a much better job of filtering than something I can write.  


## Usage

`topd` is primarily designed to interface with `fzf`. For general usage, a user will create a shell hook that adds a directory every time the current directory is changed. This will start to build your profile of most-used directories. Then, `topd` can be used as a source for `fzf`. I personally use the `fzf`-provided control-T bindings, modified to use `topd` as input. Some examples are below.

Basic usage
```sh
topd --sorted | fzf
```

For integration with `fzf` CTRL-T, define the following environment variables
```zsh
export FZF_CTRL_T_COMMAND='command topd --sorted'
export FZF_CTRL_T_OPTS='--tiebreak=index'
```

To preferentially use results from topd, but fall back to other results, we can use `cat` to combine results before sending them to `fzf`. My favorite alternate source is `fd` ([link](https://github.com/sharkdp/fd)), but the more common `find` can also be used. The following options first use `topd` results, then use all the subdirectories of the current directory, then use every subdirectory in your home directory. This is what I personally use.
```zsh
export FZF_CTRL_T_COMMAND='command cat <(topd --sorted) <(fd -t d) <(fd -t d . ~)'
export FZF_CTRL_T_OPTS='--tiebreak=index'
```

### Shell integration

#### zsh

```zsh
topd_chpwd() {
  topd --add "$(pwd)"
}
typeset -gaU chpwd_functions
chpwd_functions+=topd_chpwd
```


#### vim
You can also track most used files in vim
```viml
" Topd integration
function IncrementTopd()
  execute  "!topd --store_name 'files.json' --add " . expand('%:p')
endfunction
autocmd BufNewFile,BufReadPost * call IncrementTopd()
```

More shells to come

## TODO 

- [ ] Implement automatic re-scaling to prevent overflow
- [ ] Investigate supporting multiple half-lives for more flexible decay rates
- [ ] Figure out how to get Serde to serialize/deserialize with rc pointers
