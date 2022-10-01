# aptos pprint

With `debug::print(&b"aptos debug made easy")` in code.

Before 🤯

```
$ aptos move test
...
[debug] (&) [97, 112, 116, 111, 115, 32, 100, 101, 98, 117, 103, 32, 109, 97, 100, 101, 32, 101, 97, 115, 121]
```

After 😇

```
$ aptos move test | aptos-pprint
...
[debug] aptos debug made easy
```

## Install

```
cargo install aptos-pprint
```
