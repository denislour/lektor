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

Anh em học Rust, chắc cũng từng gặp cái signature function như `fn foo<'a>(x: &'a str, y: &'a str) -> &'a str` xong muốn quăng laptop vào thùng rác.

*" WTF 'a là cái gì? Sao lại có 'a ở đây? 'b được không? Nhiều 'a quá viết mỏi tay vãi..."*

Hiểu rồi thì thấy nó đơn giản như ăn bánh. Để tôi giải thích nhẹ cho anh em dễ hiểu hơn.

## Lifetime Là Gì? Nói Cho Anh Em Dễ Hiểu

Lifetime là **khoảng thời gian một biến tồn tại trong bộ nhớ**. Mỗi biến trong Rust đều có lifetime. Khi anh em viết `&'a str`, anh em đang nói: *"Cái tham chiếu này chỉ hợp lệ trong khoảng thời gian 'a thôi nha"*.

Compiler dùng lifetime để đảm bảo anh em không bao giờ có **dangling reference** — tham chiếu tới dữ liệu đã bị xóa. Như kiểu anh em giữ số điện thoại của người yêu cũ, nhưng người ta đã đổi số — gọi là tạch.

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

Đoạn code này không compile được vì `x` (sống trong `'b`) chết trước khi `r` (sống trong `'a`) được xài. Compiler bảo: *"'b không sống đủ lâu để thỏa mãn 'a"*. Đơn giản là thằng đi sau phải sống lâu hơn thằng đi trước — như xếp hàng mua trà sữa vậy.

## Khi Nào Cần Viết Lifetime? Trả Lời Gọn Cho Anh Em

Câu trả lời gọn: **Khi function trả về một tham chiếu**.

Rust có *lifetime elision rules* — compiler tự suy ra lifetime trong nhiều trường hợp. Anh em không cần viết:

```rust
// Compiler tự hiểu — không cần 'a
fn first_word(s: &str) -> &str {
    s.split_whitespace().next().unwrap_or("")
}
```

Nhưng khi có nhiều tham số, compiler không biết nên trả về cái nào:

```rust
// Lỗi! Compiler không biết chọn thằng nào
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

`'a` là cầu nối: nó nói rằng `x`, `y`, và kết quả trả về đều sống trong cùng một khoảng thời gian. Không có 'a là compiler không biết đường nào mà lần.

> *"Lifetime là cách Rust nói: 'Mày sống bao lâu thì tao mới dám xài'."*

## Struct Với Tham Chiếu — Cơn Ác Mộng

Đây là chỗ anh em thấy lifetime xuất hiện nhiều nhất — và cũng là chỗ dễ sai nhất:

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

Tại sao Config cần `'a`? Vì struct này chứa tham chiếu. Rust cần biết dữ liệu mà `db_url` và `redis_url` trỏ tới tồn tại bao lâu. Không có `'a` là compiler ăn gạch ngay — kiểu như mượn sách của thư viện mà không nói khi nào trả.

## Nhiều Lifetime — Phức Tạp Hơn Nhưng Cũng Dễ

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

Ở đây `'a` và `'b` độc lập. `input` và `config` có thể sống với các khoảng thời gian khác nhau. Compiler kiểm tra rằng khi anh em xài `Parser`, cả hai tham chiếu đều còn sống — không ai chết trước ai hết.

Cũng như việc anh em giữ số của đồng nghiệp và số của người yêu: hai đứa không liên quan gì đến nhau, nhưng cả hai đều phải còn sống để anh em gọi được.

## 'static — Sống Mãi Với Chương Trình

`'static` là lifetime đặc biệt — nó tồn tại suốt chương trình.

```rust
// String literals là 'static
let s: &'static str = "Tôi sống mãi với chương trình";

// Hay dùng với trait objects
fn run() -> Result<(), Box<dyn std::error::Error + 'static>> {
    Ok(())
}
```

Nhưng đừng lạm dụng `'static` nha anh em. Nếu thấy mình viết `&'static str` trong struct, hãy tự hỏi: *"Dữ liệu này có thực sự sống mãi không?"* — hay là mình lười suy nghĩ?

## Mẹo Nhỏ Cho Anh Em

1. **Bắt đầu với compile — sửa lỗi lifetime sau.** Rust compiler chỉ cho anh em chính xác chỗ cần thêm lifetime — khỏi đoán mò, khỏi mất công.

2. **Nếu thấy mệt, xài `Box` hoặc `String` thay vì tham chiếu.** Không có lifetime, dễ thở hơn hẳn.

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

## Kết Luận

Lifetime là thứ anh em phải sống chung khi viết Rust. Hiểu được nó thì chẳng có gì ghê gớm — chỉ là compiler muốn chắc chắn anh em không xài dữ liệu đã bị xóa. Còn lười quá thì xài `String`, `Box`, `Rc` cho lành.

> *"Lifetime trong Rust giống như bảo hiểm nhân thọ — lúc đầu thấy phiền, nhưng đỡ được một lần là thấy đáng hoặc không."* 🦀
