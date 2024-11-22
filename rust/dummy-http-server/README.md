# Dummy http server

## 实现的功能

1. GET / POST 请求
2. 一级目录静态文件访问
3. 基本的优雅关闭
4. 支持 Router

## 使用的 crate

1. tokio: 异步运行时
2. httparse / http / tokio_util / mime_guess: 解析 http 请求
3. tracing / tracing-subscriber: 日志

## 运行

```shell
cargo r
```

## 测试

1. GET 请求

```shell
curl localhost:8080/hello
```

```text
Hello, World! (using GET method), body=
```

2. POST 请求

```shell
curl -X POST localhost:8080/hello -d fdsafsdafs
```

```text
Hello, World! (using POST method), body=fdsafsdafs
```

3. 静态文件访问

直接使用浏览器打开 `http://localhost:8080/`，会显示 `index.html` 的内容。
index.html 引用了 a.css，所以页面上的 `Hello, World!` 会变成红色。

4. 优雅关闭

运行服务后，使用 `Ctrl + C` 关闭服务，会输出以下日志后关闭服务：

```text
2024-11-22T17:29:14.501661Z  INFO dummy_http_server::server: Shutting down server...
2024-11-22T17:29:14.501715Z  INFO dummy_http_server::server: All connections closed, server shut down, bye!
```


