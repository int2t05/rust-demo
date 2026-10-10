// ============================================================
// 31-web-server: 多线程 Web 服务器（入口）
// ============================================================
// 学习点：
// 1. TcpListener 监听端口
// 2. TcpStream 读写
// 3. HTTP 请求解析（简化版：读第一行）
// 4. HTTP 响应构造（status line + headers + body）
// 5. 用 ThreadPool 多线程处理连接
//
// 对应教程：docs/notes/14-实战项目/02-多线程Web服务器.md
// 运行方式：cargo run -p web_server_demo
// 测试：浏览器访问 http://127.0.0.1:7878
//       或 curl http://127.0.0.1:7878
//       curl http://127.0.0.1:7878/sleep
//       Ctrl+C 停止（会触发优雅关闭）
// ============================================================

use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;
use std::time::Duration;

use web_server_lib::ThreadPool;

fn main() {
    // 绑定 127.0.0.1:7878
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap_or_else(|e| {
        eprintln!("绑定端口失败: {}", e);
        std::process::exit(1);
    });
    println!("服务器启动: http://127.0.0.1:7878");
    println!("按 Ctrl+C 停止");

    // 创建 4 线程的线程池
    let mut pool = ThreadPool::new(4);

    // 接收连接；for 循环在 listener 关闭时结束
    for stream in listener.incoming() {
        let stream = match stream {
            Ok(s) => s,
            Err(e) => {
                eprintln!("连接失败: {}", e);
                continue;
            }
        };

        // 把连接处理交给线程池
        pool.execute(|| {
            handle_connection(stream);
        });
    }

    // listener.incoming() 结束后优雅关闭
    println!("服务器正在关闭...");
    pool.shutdown();
    println!("所有 worker 已停止");
}

/// 处理一个 HTTP 连接
fn handle_connection(mut stream: std::net::TcpStream) {
    let buf_reader = BufReader::new(&stream);

    // 读取请求行（HTTP 请求第一行：GET /path HTTP/1.1）
    let request_line = match buf_reader.lines().next() {
        Some(Ok(line)) => line,
        _ => return, // 连接异常，直接返回
    };

    // 解析路径：形如 "GET /sleep HTTP/1.1"
    let (status_line, contents) = match parse_request(&request_line) {
        Route::Root => ("HTTP/1.1 200 OK", root_html()),
        Route::Sleep => {
            std::thread::sleep(Duration::from_secs(2));
            ("HTTP/1.1 200 OK", sleep_html())
        }
        Route::NotFound => ("HTTP/1.1 404 NOT FOUND", not_found_html()),
    };

    // 构造响应：status line + headers + 空行 + body
    let length = contents.len();
    let response = format!(
        "{}\r\nContent-Length: {}\r\nContent-Type: text/html; charset=utf-8\r\n\r\n{}",
        status_line, length, contents
    );

    if let Err(e) = stream.write_all(response.as_bytes()) {
        eprintln!("写入响应失败: {}", e);
    }
    let _ = stream.flush();
}

/// 路由枚举
enum Route {
    Root,
    Sleep,
    NotFound,
}

/// 解析请求行得到路由
fn parse_request(request_line: &str) -> Route {
    // 期望格式："GET /path HTTP/1.1"
    let parts: Vec<&str> = request_line.split_whitespace().collect();
    if parts.len() < 2 {
        return Route::NotFound;
    }
    let method = parts[0];
    let path = parts[1];

    if method != "GET" {
        return Route::NotFound;
    }

    match path {
        "/" => Route::Root,
        "/sleep" => Route::Sleep,
        _ => Route::NotFound,
    }
}

// ===== 响应 HTML =====

fn root_html() -> String {
    "<!DOCTYPE html>
<html lang=\"zh-CN\">
<head><meta charset=\"utf-8\"><title>Rust Server</title></head>
<body>
<h1>Hello from Rust Web Server</h1>
<p>这是一个用纯标准库写的多线程 Web 服务器。</p>
<ul>
<li><a href=\"/\">/</a> 首页</li>
<li><a href=\"/sleep\">/sleep</a> 模拟耗时请求（2 秒）</li>
<li><a href=\"/unknown\">/unknown</a> 触发 404</li>
</ul>
</body>
</html>".to_string()
}

fn sleep_html() -> String {
    "<!DOCTYPE html>
<html lang=\"zh-CN\">
<head><meta charset=\"utf-8\"><title>Sleep</title></head>
<body>
<h1>Sleep 响应</h1>
<p>这个请求故意 sleep 了 2 秒，演示线程池并发。</p>
<p>同时访问 / 和 /sleep，你会发现 / 不会被阻塞。</p>
</body>
</html>".to_string()
}

fn not_found_html() -> String {
    "<!DOCTYPE html>
<html lang=\"zh-CN\">
<head><meta charset=\"utf-8\"><title>404</title></head>
<body>
<h1>404 Not Found</h1>
<p>路径不存在，试试 <a href=\"/\">/</a></p>
</body>
</html>".to_string()
}
