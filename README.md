# regmatch

正規表現の対話型テストツール

## Prerequisites

**build**

- [cargo](https://github.com/rust-lang/cargo)

## Getting Started

```
$ git clone https://github.com/TundraClimate/regmatch.git
$ cd regmatch

$ cargo build -r
$ ln -s $PWD/target/release/regmatch /usr/local/bin
```

## How to use

```
$ regmatch ^He(l){2}o$
Helo
>> No matches
Heoo
>> No matches
Hello
>> Match "Hello"

$ regmatch ^He(l){2}o
Hello, World
>> Match "Hello"
```
