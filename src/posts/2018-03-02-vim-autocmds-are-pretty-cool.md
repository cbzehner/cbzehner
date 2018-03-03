---
title: Vim Autocommands are Pretty Cool
---

## Run tests with minimal effort
I have a mapping in my vim configuration to run tests for the current file I have open using `CTRL-t`:
```
map <C-t> :w !t %<CR>
```
Basically this command says `map` the combination of the Control key `C` and the "t" key `t` to the following command.

First, save the file with `:w` because there's no point running tests on the old version of the file.

Then, send the following command `t %` to the console. `!` means everything afterwards goes to the console. If I enter just `t` in my commandline it runs one of my company's test frameworks and `%` is replaced with the path of the current file. Pretty handy!

Finally, end the command with the "Carriage Return" `<CR>`, which is a fancy way of saying "press ENTER!".

Great! Now I can run tests super easily! But what about if I want to use `CTRL-t` outside of work?

## Attempt to customize the message per programming language

At work I mostly write PHP and Javascript code but for fun I mostly use Haskell and Rust, so maybe I can use a different command based on the file type? I know Vim has a `FileType` thingy. After a bit of searching I found out [how to do conditionals in Vim](http://learnvimscriptthehardway.stevelosh.com/chapters/21.html). So something like:
```
if FileType php
    map <C-t> :w !t %<CR>
elseif FileType Rust
    map <C-t> :w !cargo test<CR>
endif
```
Nope! That doesn't work like I thought it would at all. I just get this error message:
```
Error detected while processing /Users/cbzehner/.config/nvim/init.vim:
line    7:
E121: Undefined variable: FileType
E15: Invalid expression: FileType php
line  280:
E171: Missing :endif
```

## Enter Autocommands!

> "Autocommands are a way to tell Vim to run certain commands whenever certain events happen."
> - Steve Losh [explains autocommands](http://learnvimscriptthehardway.stevelosh.com/chapters/12.html) really well.

One of the events is a `FileType` event! Score! So we can just define a bunch of autocommands using the `au` shorthand:
```
au FileType php        nnoremap <C-t> :w !t %<CR>
au FileType javascript nnoremap <C-t> :w !jest %<CR>
au FileType rust       nnoremap <C-t> :w !cargo test<CR>
au FileType haskell    nnoremap <C-t> :w !stack test<CR>
```

This is fantastic! Now we can test any PHP, Javascript, Rust and Haskell with just one command! Could we improve this even more? Yes!

We can use [autocommand groups](http://learnvimscriptthehardway.stevelosh.com/chapters/14.html) to prevent duplicate autocommands from causing issues. So let's take our existing code and wrap it up in an autocommand group.
```
" Run tests on the current file
augroup testCurrentFile
  au!
  au FileType php           nnoremap <C-t> :w !t %<CR>
  au FileType javascript    nnoremap <C-t> :w !jest %<CR>
  au FileType rust          nnoremap <C-t> :w !cargo test<CR>
  au FileType haskell       nnoremap <C-t> :w !stack test<CR>
augroup END
```

Awesome! Now we don't have to worry about duplicate autocommands and we can just use `CTRL-t` to run tests on a bunch of different file types.

