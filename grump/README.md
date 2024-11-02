# Grump

## Running

```shell
$ go run cmd/server/main.go
```

## Todo

- [ ] Embed template into binary with `embed` directive controlled by build options
- [ ] Support markdown 
- [ ] Support `.mdx` files
- [ ] enable Gzip in/out using `Compress` and `Gzip` middlewares
- [ ] Static files
- [ ] Decide on app example
- [ ] Use [env lib](https://github.com/caarlos0/env) to manage environment
- [ ] Containerise
- [ ] set up ORM with https://gorm.io/docs/
- [ ] enable prometheus / opentelemetry metrics
- [ ] Look into https://gorilla.github.io


- Implement in Rust instead? using https://github.com/ntex-rs/ntex and https://docs.rs/markdown/1.0.0-alpha.21/markdown/struct.ParseOptions.html