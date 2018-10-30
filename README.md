# Parallel indexer written in Rust #

## What does it do ##
It's a small app that indexes all directories and files on your hard drive. It's really fast (indexes the whole harddrive in less than a minute on a macbook with SSD drive).

It lists everything on your computer and pututs them line by line in a file called index.txt that is put in the base directory where you run the program from.

## How to use it ##
First of all, you need to change the base directory if you're not on osx or linux, i.e. to "C:\" on Windows.

You need to compile it using cargo:

`cargo build --release`