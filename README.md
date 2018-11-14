# Frecency with Exponential decay (`fe`)

`fe` is a CLI tool for tracking your most-used directories and files. 
Though inspired by tools like `autojump` or the `z` plugin for `zsh`, it takes a slightly 
different approach to tracking and providing usage data. 
The primary difference is `fe` does not support jumping. Instead, it just keeps track of and provides sorting methods for directories, 
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
# Print directories, sorted by frecency, then pipe to fzf
fe --sorted | fzf

# Print directories and their associated frecency, sorted by frecency
fe --stat

# Log a visit to a directory
fe --add ~/new_dir

# Decrease weight of a directory by 10 visits
fe --decrease 10 ~/too_high_dir

# Print directories and the time since they were last visited in hours
fe --stat --sort_method recent

# Print directories and the number of times they've been visited
fe --stat --sort_method frequent

# Remove all directories that no longer exist from the database
fe --purge 
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
(credit to `autojump`)

```zsh
fe_chpwd() {
  fe --add "$(pwd)"
}
typeset -gaU chpwd_functions
chpwd_functions+=fe_chpwd
```

More shells to come

### Vim integration

Want to track what files are most frecently opened in vim? Me too. I'm working on making that functional.


## Support

I use this regularly on MacOS and Linux. I wrote it to be usable on Windows as well, 
but I don't run any tests for it. Caveat emptor.

## TODO 

- [ ] Investigate using `fe` as a source for tab completions à la `z`
- [ ] Investigate accepting paths from stdin to sort

## About the algorithm

The algorithm used combines the concepts of frequency and recency into a single, sortable statistic called "frecency".
To my knowledge, this term was first coined by Mozilla to describe their URL suggestions algorithm. 
In fact, Mozilla already came up with nearly this exact algorithm and 
[considered using it to replace Firefox's frecency algorithm](https://wiki.mozilla.org/User:Jesse/NewFrecency?title=User:Jesse/NewFrecency).
The algorithm is also very similar to the cache replacement problem, and a more formal treatment of the
math behind it can be found in this [IEEE article](https://ieeexplore.ieee.org/document/970573) (sorry for the paywall).

This algorithm calculates the frecency score of each directory as the sum of the weights of each visit to that directory.
The weight of a visit decays exponentially with time, causing more recently visited directories to be ranked higher. 
Additionally, by leveraging some special properties of exponential decay, we can collapse this number down into a 
single stored number for each directory so we don't have to store every time each directory was visited. I'll hopefully
get around to writing a blog post about this in the near future.  

