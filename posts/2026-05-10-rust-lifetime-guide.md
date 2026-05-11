---
title: "Rust Lifetime - 'a Là Gì? Khi Nào Cần Viết 'a?"
date: "2026-05-10"
tags: ["rust", "lifetime"]
readTime: 8
excerpt: "Viết Rust mà không hiểu lifetime chẳng khác gì học võ mà không biết đấm. 'a là một cái nhãn dán, báo cho compiler biết biến này sống lâu hơn biến kia."
thumbnailCode: |-
  struct User<'a> {
      name: &'a str,
      posts: &'a [Post<'a>],
  }

  struct Post<'a> {
      title: &'a str,
      content: &'a str,
  }

  fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
      if x.len() > y.len() { x } else { y }
  }
---

# Rust Lifetime - 'a Là Gì? Khi Nào Cần Viết 'a?

Hồi mới học Rust, anh em nhìn cái signature function như `fn foo<'a>(x: &'a str, y: &'a str) -> &'a str` xong chỉ muốn quăng laptop vào thùng rác.

*"Đm, sao lại có 'a ở đây? 'a là cái quái gì? Có phải 'b không? Nhiều 'a quá viết mỏi tay không?"*

Hiểu rồi thì thấy nó đơn giản. Để anh em mình giải thích.

## Lifetime Là Gì? Nói Cho Anh Em Dễ Hiểu

Lifetime là **khoảng thời gian một biến tồn tại trong bộ nhớ**. Mỗi biến trong Rust đều có lifetime. Khi anh em viết `&'a str`, anh em đang nói: *"Cái tham chiếu này chỉ hợp lệ trong khoảng thời gian 'a thôi nha."*

Compiler dùng lifetime để đảm bảo anh em không bao giờ có **dangling reference** — tham chiếu tới dữ liệu đã bị xóa.

```rust
fn main() {
    let r;                // ---------+-- 'a
    {                     //          |
        let x = 5;       // -+-- 'b  |
        r = &x;          //  |       |
    }                     // -+       |
    println!("{}", r);    //          |
}                         // ---------+
```

Đoạn code này không compile được vì `x` (sống trong `'b`) chết trước khi `r` (sống trong `'a`) được dùng. Compiler bảo: *"'b không sống đủ lâu để thỏa mãn 'a"*. Cơ bản là thằng đi sau phải sống lâu hơn thằng đi trước, như xếp hàng vậy.

## Khi Nào Cần Viết Lifetime? Trả Lời Nhanh Cho Anh Em

Câu trả lời nhanh: **Khi function trả về một tham chiếu**.

Rust có *lifetime elision rules* — compiler tự suy ra lifetime trong nhiều trường hợp. Anh em không cần viết:

```rust
// Compiler tự hiểu - không cần viết 'a
fn first_word(s: &str) -> &str {
    s.split_whitespace().next().unwrap_or("")
}
```

Nhưng khi có nhiều tham số, compiler không biết cái nào nên trả về:

```rust
// Lỗi! Compiler không biết trả về cái nào
fn choose_display(x: &str, y: &str) -> &str {
    if x.len() > 10 { x } else { y }
}
```

Sửa lại:

```rust
fn choose_display<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > 10 { x } else { y }
}
```

`'a` là cầu nối: nó nói rằng `x`, `y`, và kết quả trả về đều sống với cùng một khoảng thời gian. Dễ như ăn bánh.

## Struct Với Tham Chiếu — Cơn Ác Mộng Của Anh Em

Đây là lúc anh em sẽ thấy lifetime xuất hiện nhiều nhất:

```rust
struct Config<'a> {
    db_url: &'a str,
    redis_url: &'a str,
}

impl<'a> Config<'a> {
    fn new(db_url: &'a str, redis_url: &'a str) -> Self {
        Self { db_url, redis_url }
    }
}
```

Tại sao Config cần `'a`? Vì struct này chứa tham chiếu. Rust cần biết dữ liệu mà `db_url` và `redis_url` trỏ tới tồn tại bao lâu. Không có `'a` là compiler ăn gạch ngay.

## Nhiều Lifetime — Khi Cuộc Đời Anh Em Phức Tạp Hơn

```rust
struct Parser<'a, 'b> {
    input: &'a str,
    config: &'b Config,
}

impl<'a, 'b> Parser<'a, 'b> {
    fn parse(&self) -> Vec<&'a str> {
        self.input.lines().collect()
    }
}
```

Ở đây `'a` và `'b` độc lập. `input` và `config` có thể sống với các khoảng thời gian khác nhau. Compiler sẽ kiểm tra rằng khi anh em dùng `Parser`, cả hai tham chiếu đều còn sống. Phân biệt chứ đừng có nhầm.

## 'static — Sống Mãi Mãi (Hoặc Gần Như Vậy)

`'static` là lifetime đặc biệt — nó tồn tại suốt chương trình.

```rust
// String literals là 'static
let s: &'static str = "Tôi sống mãi với chương trình";

// Cũng hay dùng với trait objects
fn run() -> Result<(), Box<dyn std::error::Error + 'static>> {
    Ok(())
}
```

Nhưng đừng lạm dụng `'static` nha anh em. Nếu thấy mình viết `&'static str` trong struct, hãy tự hỏi: *"Liệu dữ liệu này có thực sự sống mãi không?"* Hay là mình chỉ lười?`

## Mẹo Nhỏ Cho Anh Em Khi Xài Lifetime

1. **Bắt đầu với việc compile — sửa lỗi lifetime sau.** Rust compiler chỉ cho anh em chính xác chỗ cần thêm lifetime, khỏi đoán mò.

2. **Nếu thấy mệt, xài `Box` hoặc `String` thay vì tham chiếu.** Không có lifetime, dễ thở hơn.

```rust
// Thay vì struct với lifetime
struct Config<'a> {
    db_url: &'a str,
}

// Xài owned type — không cần 'a
struct Config {
    db_url: String,
}
```

> *"Lifetime trong Rust giống như bảo hiểm nhân thọ — lúc đầu thấy tốn kém, nhưng về sau anh em sẽ thấy đôi khi nó cũng có ít hoặc không"*

