# abuild

> a tool for building

## language support

- Rust
- C/C++
- C#

## usage

```shell
$ abuild create -w my-worksapce
...
$ cd my-workspace
$ abuild create -j my-project
...
$ vi ./my-project/main.rs # edit your code
$ abuild build
...
$ ./target/debug/my-project
Hello, world!
$ abuild clean
...
$ 
```
