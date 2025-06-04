# abuild

> a tool for building

## language support

- Rust ![feature]
- C/C++ ![feature]
- C# ![feature]

## usage ![feature]

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

---


[note]: https://img.shields.io/badge/note-orange.svg?color=ddbb00

[bug]: https://img.shields.io/badge/bug-red.svg

[feature]: https://img.shields.io/badge/feature-orange.svg