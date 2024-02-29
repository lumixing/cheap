## cheap: a chip8 emulator
> this is a tool in the [charm](https://lumixing.github.io/charm/) toolset

written in rust using [macroquad](https://github.com/not-fl3/macroquad)

![image](https://github.com/lumixing/cheap/assets/45235073/5459cfe6-9db2-4600-b44b-161bb4561ea6)

## usage
first argument is rom input path  
second argument is clock speed (per second) (optional, default is 240)  
run with `--release` flag for faster speed
```console
$ cargo run -- input.ch8 1000
```

## resources
- https://github.com/aquova/chip8-book/
