# FREcency tracking (`fre`)

`fre` is a CLI tool for tracking your most-used directories and files. 
Though inspired by tools like `autojump` or the `z` plugin for `zsh`, it takes a slightly 
different approach to tracking and providing usage data. 
The primary difference is `fre` does not support jumping. Instead, 
it just keeps track of and provides sorting methods for directories, 
which can then be filtered by another application like `fzf`, 
which does a much better job of filtering than something I can write.
Additionally, it uses an algorithm in which the weights of each directory
decay exponentially, so more recently used directories are ranked more highly
in a smooth manner.


## Usage

`fre` is primarily designed to interface with `fzf`. For general usage, 
a user will create a shell hook that adds a directory every time the current 
directory is changed. This will start to build your profile of most-used directories. 
Then, `fre` can be used as a source for `fzf`. I personally use the `fzf`-provided 
control-T bindings, modified to use `fre` as input. Some examples are below.

Basic usage
```sh
# Print directories, sorted by frecency, then pipe to fzf
fre --sorted | fzf --no-sort

# Print directories and their associated frecency, sorted by frecency
fre --stat

# Log a visit to a directory
fre --add /home/user/new_dir

# Decrease weight of a directory by 10 visits
fre --decrease 10 /home/user/too_high_dir

# Print directories and the time since they were last visited in hours
fre --stat --sort_method recent

# Print directories and the number of times they've been visited
fre --stat --sort_method frequent

# Purge directories that no longer exist
fre --sorted | while read dir ; do if [ ! -d "$dir" ] ; then fre --delete "$dir";  fi ; done
```

## Installation

From source: `git clone https://github.com/camdencheek/fre.git && cargo install --path ./fre`

From crate: `cargo install fre`

Arch linux: `yay -S fre`

macOS: `brew install camdencheek/brew/fre`

For integration with `fzf` CTRL-T, define the following environment variables 
```zsh
export FZF_CTRL_T_COMMAND='command fre --sorted'
export FZF_CTRL_T_OPTS='--tiebreak=index'
```

To preferentially use results from `fre`, but fall back to other results, we can use 
`cat` to combine results before sending them to `fzf`. My favorite alternate source 
is `fd` ([link](https://github.com/sharkdp/fd)), but the more common `find` can also be 
used. The following options first use `fre` results, then use all the subdirectories 
of the current directory, then use every subdirectory in your home directory. 
This is what I personally use.

```zsh
export FZF_CTRL_T_COMMAND='command cat <(fre --sorted) <(fd -t d) <(fd -t d . ~)'
export FZF_CTRL_T_OPTS='--tiebreak=index'
```

### Shell integration

#### zsh
(credit to `autojump`)

```zsh
fre_chpwd() {
  fre --add "$(pwd)"
}
typeset -gaU chpwd_functions
chpwd_functions+=fre_chpwd
```

More shells to come

### Vim integration

Want to track what files are most frecently opened in vim? Me too. I'm working on making that functional.


## Comparison to existing solutions

The three projects I'm familiar with that are closest in function to this are `autojump`, the `z` shell plugin, and the `d` portion (and maybe the `f` in the future) of `fasd`. 

The primary difference from the rest of these is its reliance on a tool like `fzf` to provide any solid directory jumping functionality. This was an intentional choice, sticking to the Unix philosophy of "do one thing, and do it well". 

The other major change from these pieces of software is the algorithm used to rank directories.  `autojump` uses the following formula:

```python

def add_path(data, path, weight=10):
    # ...
    data[path] = sqrt((data.get(path, 0) ** 2) + (weight ** 2))
    # ...
```

Looking at it closely, it seems to just be calculating the hypotenuse of a triangle where one side is the length of the previous weight and the other is the length of the weight being added. This does not take into account time passed since access at all, which is not ideal since I would rather not have directories from years ago ranked highly.

`fasd` and `z` both use the same frecency function that looks something like this:

```zsh
function frecent(rank, time) {
    dx = t-time
    if( dx < 3600 ) return rank*4
    if( dx < 86400 ) return rank*2
    if( dx < 604800 ) return rank/2
    return rank/4
}
```

This works fine until you re-visit an old directory. Then, suddenly, `dx` is small again and all the old visits are re-weighted to `rank*4`, causing it to jump up in the sorted output. This is not really ideal. I want to be able to re-visit an old directory once without messing up my directory ranking. 

`fre` uses a frecency algorithm where the weight of a directory visit decays over time. Given a list of visit times (bold x), the frecency of the directory would look something like this (using lambda as the half life and "now" as the current time at calculation):

<a href="https://user-images.githubusercontent.com/12631702/48453749-a1bbbc00-e782-11e8-9c4e-4c367db02794.png"><img src="https://user-images.githubusercontent.com/12631702/48453749-a1bbbc00-e782-11e8-9c4e-4c367db02794.png" align="center" height="100" width="450" ></a>

With a little bit of mathemagics, we don't actually have to store the vector of access times. We can compress everything down into one number as long as we're okay not being able to dynamically change the half life. 

This algorithm provides a much more intuitive implementation of frecency that tends to come up with results that more closely match those we would naturally expect.

## Support

I use this regularly on MacOS and Linux. I wrote it to be usable on Windows as well, 
but I don't run any tests for it. Caveat emptor.


## Stability

I've been using this for over a year with no chnages now, and it does everything I need it to do. I'm happy to add features or accept changes if this is not the case for you.

## About the algorithm

The algorithm used combines the concepts of frequency and recency into a single, sortable statistic called "frecency".
To my knowledge, this term was first coined by Mozilla to describe their URL suggestions algorithm. 
In fact, Mozilla already came up with nearly this exact algorithm and 
[considered using it to replace Firefox's frecency algorithm](https://wiki.mozilla.org/User:Jesse/NewFrecency?title=User:Jesse/NewFrecency).
The algorithm is also very similar to the cache replacement problem, and a more formal treatment of the
math behind it can be found in this [IEEE article](https://ieeexplore.ieee.org/document/970573) (sorry for the paywall).

