---
title: "93.7% Rust dev từng khóc vì lifetime — và tôi là 1 trong 93.7% đó"
date: "2026-05-10"
tags: ["rust", "lifetime"]
readTime: 8
excerpt: "93.7% lập trình viên Rust từng khóc vì lifetime (khảo sát từ 3 thằng nhậu ở quán cóc). Đây là chuyện một người trong số họ vượt qua lifetime — và trở nên mạnh mẽ hơn."
thumbnailCode: |-
  struct User<'a> {
      name: &'a str,
      posts: &'a [Post<'a>],
  }

  fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
      if x.len() > y.len() { x } else { y }
  }
---

Hồi mới học Rust, tôi ghét lifetime. Ghét cái `'a` vô tri vô giác. Ghét cái compiler cứ nhai đi nhai lại *"borrowed value does not live long enough"* — nhưng không thèm chỉ cho tôi sống bao nhiêu mới là đủ.

Như kiểu đi thi mà giám thị bảo "em làm bài cẩn thận nhé" xong không cho đề.

2 giờ sáng. Trước mặt tôi là một struct `User` với một cái `&str` làm tên. Có mỗi thế. Và cái compiler nó không chịu chạy. Màn hình xanh le lói, mắt tôi đỏ ngầu, bên cạnh là ly cà phê phin đã nguội từ lâu — cũng như hy vọng của tôi về việc compile thành công.

Tôi thêm `'a` chỗ này, thêm `'a` chỗ kia, viết `impl<'a>` xong lại `struct<'a>`. Càng thêm càng sai. Càng sai càng thêm. Giống như hồi cấp 3 tôi giải phương trình vô tỷ: ra nghiệm sai, thử lại càng sai, cuối cùng đập bút bảo "chắc đề sai". Nhưng lần này đề không sai — tôi mới sai.

93.7% lập trình viên Rust từng khóc vì lifetime — số liệu từ cuộc khảo sát 3 thằng ngồi nhậu ở quán cà phê cóc Nguyễn Huệ. 6.3% còn lại? Họ nói dối.

3 ngày sau tôi mới nhận ra: lifetime không phải kẻ thù. Nó là bảo hiểm nhân thọ cho code. Phiền thật — nhưng lỡ có chuyện thì mới thấy nó đáng giá.

Đây là những gì tôi muốn nói với anh em, từ một thằng đã từng mất ngủ vì `'a`.

---

## Lifetime Là Gì? — Nói Như Thằng Đã Từng Khóc Vì Nó

Sách Rust bảo: *"Lifetime là khoảng thời gian một tham chiếu còn hiệu lực"*.

Đọc xong tôi gật gù. Xong vẫn không hiểu gì. Như đọc báo sức khỏe: *"Ăn uống lành mạnh giúp cơ thể khỏe mạnh"* — chuẩn, nhưng ăn cái gì? Im mẹ luôn.

Nói đơn giản hơn: mỗi biến trong Rust đều có tuổi thọ. Sinh ra ở đâu, chết hồi nào. Anh em viết `&x` là mượn tạm `x` để xài. Mượn thì phải trả — và compiler muốn chắc chắn anh em không xài cái đã trả.

Cái `'a` là một cái **nhãn dán**. Post-it note anh em dán lên tham chiếu: "Thằng này sống ít nhất tới lúc em gọi `println!` xong nhé."

```rust
fn main() {
    let r;                // ---------+-- r bắt đầu sống
    {                     //          |
        let x = 5;       // -+-- 'b  |  
        r = &x;          //  |       |
    }                     // -+-- 'b hết: x chết cmn
    println!("{}", r);    //          |
}                         // ---------+-- r kết thúc
// ^ LỖI: `x` does not live long enough — câu nói ám ảnh nhất đời tôi
```

Kinh điển. Ai học Rust cũng gặp. `x` sinh trong `{}`, chết ngay khi ra ngoài. Nhưng `r` vẫn sống, vẫn đòi xài `x` khi `x` đã về với cát bụi. Compiler bảo: "Mày định gọi điện cho người yêu cũ à? Họ đổi số từ lâu rồi."

Lúc mới học tôi cứ nghĩ: *"Cho x sống lâu lên tí chết ai? Sao không cho nó tồn tại hết cái main?"*

Đấy là tư duy C/C++. Quen cấp phát rồi tự xóa — muốn sống bao lâu tùy, dangling pointer thì chịu. Rust không cho làm thế. Rust nói: "Xin lỗi, tao phải đảm bảo mày không chết vì memory bug lúc nửa đêm."

**Quan điểm của tôi**: Học lifetime giống như tập squat. Lúc đầu đau đớn, tưởng sắp chết. Làm đúng kỹ thuật xong — khoẻ cả đời. Bỏ qua — toạc dây chằng lúc nào không hay.

---

## Lifetime Elision — Khi Rust Bảo Vệ Anh Em Khỏi Chính Mình

Tôi từng thắc mắc: "Nếu lifetime quan trọng thế, sao tôi viết `fn first_word(s: &str) -> &str` không thấy `'a` đâu?"

Đấy là vì **lifetime elision rules**. Ba luật compiler dùng để tự suy ra lifetime:

1. Mỗi tham số `&T` hoặc `&mut T` được tự động gán một lifetime riêng.
2. Nếu chỉ có **một** lifetime ở tham số, nó được gán cho kết quả trả về.
3. Nếu có `&self` hoặc `&mut self`, lifetime của self gán cho kết quả.

```rust
// Không cần 'a — compiler tự suy, đỡ đau đầu
fn first_word(s: &str) -> &str {
    s.split_whitespace().next().unwrap_or("")
}

fn main() {
    let greeting = String::from("Xin chào anh em Rust developer");
    let word = first_word(&greeting);
    println!("Từ đầu tiên: {}", word);
    // Output: Từ đầu tiên: Xin
}
```

Ngon, lành, không đời `'a`.

Nhưng rồi một ngày tôi viết cái này:

```rust
// LỖI: thiếu lifetime — compiler bắt đền liền
fn choose_display(x: &str, y: &str) -> &str {
    if x.len() > 10 { x } else { y }
}
```

```text
error[E0106]: missing lifetime specifier
 --> src/main.rs:1:38
  |
1 | fn choose_display(x: &str, y: &str) -> &str {
  |                      ----     ----     ^ expected named lifetime parameter
  |
help: consider introducing a named lifetime parameter
  |
1 | fn choose_display<'a>(x: &'a str, y: &'a str) -> &'a str {
  |                  ++++        ++          ++          ++
```

Compiler không nói "mày ngu". Nó lịch sự gợi ý: *"Thêm 'a vào đi bạn hiền"*. Tôi mất 10 phút đọc cái thông báo này lần đầu. Đến lúc hiểu, tôi thấy mình như thằng ngốc — cảm giác quen thuộc khi học Rust: compiler thông minh hơn anh em, và nó biết điều đó. Hơi xấu hổ nhưng mà cũng an tâm.

Sửa lại:

```rust
fn choose_display<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > 10 { x } else { y }
}

fn main() {
    let long_str = String::from("Tôi là một câu cực kỳ dài, dài vkl!");
    let short_str = "Ngắn";
    
    let result = choose_display(&long_str, &short_str);
    println!("Kết quả: {}", result);
    // Output: Kết quả: Tôi là một câu cực kỳ dài, dài vkl!
}
```

Cái hay: `'a` nói rằng `x`, `y`, và kết quả trả về sống trong cùng khoảng thời gian. Compiler lấy lifetime ngắn nhất — như tổ chức tiệc có ba người: thằng về sớm nhất quyết định giờ giải tán. Thằng say nhất cũng phải về theo.

Đấy là `'a` trong thực tế.

---

## Struct Với Tham Chiếu — Chỗ Đau Thực Sự

Function với lifetime dễ hiểu. Nhưng struct với tham chiếu? Chỗ này tôi suýt bỏ Rust.

```rust
// LỖI: struct chứa tham chiếu mà không có lifetime — toang
struct Config {
    db_url: &str,
}
```

Lúc mới học, tôi thấy cái lỗi này và nghĩ: *"Compiler bị khùng à? Tao chỉ muốn lưu đường dẫn thôi!"*

Nhưng nghĩ kỹ thì đúng: struct `Config` không biết nó sống bao lâu. Chứa tham chiếu thì Rust phải biết dữ liệu nó trỏ tới tồn tại tới khi nào. Nếu không, dangling reference — thứ mà anh em C++ gọi là "thứ Sáu vui vẻ" còn tôi gọi là "3 giờ sáng mất ngủ".

```rust
// Đúng: cho struct biết nó sống bao lâu
struct Config<'a> {
    db_url: &'a str,
}

impl<'a> Config<'a> {
    fn new(db_url: &'a str) -> Self {
        Self { db_url }
    }
    
    fn connect(&self) {
        println!("Đang kết nối tới: {}", self.db_url);
    }
}

fn main() {
    let url = "postgres://localhost:5432/mydb";  // URL này sống tới cuối main
    let cfg = Config::new(url);
    cfg.connect();
    // Output: Đang kết nối tới: postgres://localhost:5432/mydb
}
```

Anh em thấy không? Lifetime không làm code khó hơn. Nó chỉ bắt anh em **nói rõ ý đồ** trước khi chạy. Như ký hợp đồng hôn nhân — phiền một tí nhưng sau này khỏi kiện cáo.

Cái struct này làm tôi nhớ một chuyện. Hồi làm ở FPT, tôi viết một struct tương tự — lưu connection pool, database URL, cả Redis URL. Toàn tham chiếu. Code chạy ngon 3 tháng. Rồi một ngày tôi sửa hàm khởi tạo một tẹo — toàn bộ lifetime vỡ vụn như cốc thủy tinh rơi từ tầng 10. Tôi ngồi đó, mắt nhìn màn hình, miệng lẩm bẩm "đm" như một lời nguyền.

Lúc đấy các bạn thấy: **lifetime không phải lý thuyết suông**. Nó là dây an toàn. Hiểu từ đầu thì không mất 3 tháng code sạch rồi một ngày sập hết.

---

## Nhiều Lifetime — Khi Cuộc Đời Phức Tạp Hơn

Một lifetime bình thường. Hai lifetime — thú vị bắt đầu.

Có lúc dữ liệu đến từ nhiều nguồn, với tuổi thọ khác nhau. Ví dụ nhé: chúng ta đọc config từ file, parse input từ stdin. Một thằng sống lâu, một thằng chết sớm. Hai lifetime khác nhau — và Rust muốn biết cái nào ra cái nào.

```rust
struct Config {
    pub endpoint: String,
}

// Hai lifetime riêng biệt: 'a cho input, 'b cho config
struct Parser<'a, 'b> {
    input: &'a str,       // input sống tới khi nào?
    config: &'b Config,    // config sống tới khi nào?
}

impl<'a, 'b> Parser<'a, 'b> {
    fn parse(&self) -> Vec<&'a str> {
        self.input.lines().collect()
    }
}

fn main() {
    let config = Config {
        endpoint: String::from("https://api.example.com"),
    };
    let input = "dòng 1\ndòng 2\ndòng 3";
    // Truyền input (sống ngắn) và config (sống dài) — Rust phân biệt được
    let parser = Parser { input, config: &config };
    
    let result = parser.parse();
    for (i, line) in result.iter().enumerate() {
        println!("Dòng {}: {}", i + 1, line);
    }
}
```

```text
Output:
Dòng 1: dòng 1
Dòng 2: dòng 2
Dòng 3: dòng 3
```

`'a` và `'b` là hai lifetime độc lập. `input` có thể sống 5 giây, `config` 10 giây — miễn là khi xài `Parser`, cả hai còn sống.

Như mời hai bạn đi nhậu ở quán bánh mì Nguyễn Huệ. Một người về lúc 9h vì mai đi làm sớm, người kia 10h vì sếp không ở cơ quan. Không cần bắt cả hai về cùng lúc — chỉ cần cả hai còn ở đây khi anh em muốn nói chuyện. Ai về trước thì về, Rust không quan tâm — miễn là lúc còn ngồi, cả hai còn đó.

Tôi từng nghĩ nhiều lifetime là ác mộng. Chiến đấu vài lần rồi thấy nó tuyệt nhất: **cho phép viết code linh hoạt mà không sợ sai**. Mix dữ liệu từ nhiều nguồn, nhiều kiểu sống — compiler đảm bảo không bao giờ dangling pointer.

---

## 'static — Sống Mãi Hay Sống Dai?

`'static` là lifetime đặc biệt nhất: "Sống suốt chương trình, từ đầu tới cuối."

Nghe như Elon Musk hứa hẹn đưa người lên Sao Hoả — hấp dẫn đấy, nhưng có thật không?

```rust
let s: &'static str = "Tôi sống mãi với chương trình!";
// ^ String literal được nhúng vào binary — sống tới khi app tắt
```

String literal được nhúng vào binary — tồn tại suốt runtime. But `'static` không chỉ dành cho string. Nó xuất hiện ở thread spawning, trait bounds, error handling...

```rust
use std::thread;

fn spawn_task() -> thread::JoinHandle<()> {
    thread::spawn(|| {
        println!("Chạy trong thread con... an toàn vì 'static");
    })
}

fn main() {
    let handle = spawn_task();
    handle.join().unwrap();
}
```

**Quan điểm của mình**: `'static` là bẫy của người lười. Thấy ai viết `&'static str` trong struct — khả năng cao họ đang lười nghĩ về lifetime thực sự. Như kiểu thấy bán bảo hiểm "trọn đời" — nghe hay nhưng đọc kỹ điều khoản xong mới thấy mình bị hố.

```rust
// Bẫy: 'static khi không cần — đừng làm như này
struct Logger {
    name: &'static str,
}

// Tốt hơn: dùng lifetime thực tế
struct Logger<'a> {
    name: &'a str,
}

// Hoặc đơn giản nhất: xài owned type, không lifetime, đỡ đau đầu
struct Logger {
    name: String,
}
```

Tôi từng thấy code review thế này trên Viblo:

```rust
fn process<'a>(data: &'a str) -> &'static str {
    "processed"
}
```

Cheat code. Nếu kết quả không phải string literal, nó báo lỗi ngay. Dùng `'static` kiểu này là "lười biếng nguy hiểm" — compile được, nhưng sai ngữ nghĩa. Như đeo đồng hồ Rolex giả — nhìn thì ngon, nhưng không có thật.

> **Mẹo**: Tự hỏi "Dữ liệu này có tồn tại suốt chương trình không?" Nếu không — đừng dùng `'static`. Xài `String` đi cho lành.

---

## Cạm Bẫy Thực Tế

Anh em nghĩ: "Thêm `'a` vào là xong"?

Haha. Nếu dễ thế, tôi đã không mất 3 ngày debug chỉ vì một cái lifetime.

### 1. Lifetime Coercion — Co Kéo Lifetime

Truyền lifetime dài vào chỗ mong lifetime ngắn — Rust co rút được. Làm ngược lại — chết.

```rust
fn takes_static(_s: &'static str) {}

fn main() {
    let msg = String::from("xin chào");  // msg sống tới cuối main
    takes_static(&msg); // LỖI: msg không sống đủ lâu cho 'static
}
```

Nhưng mà `msg` còn sống tới cuối `main` mà? Không. `'static` là vĩnh cửu — sống tới khi chương trình tắt. `main` kết thúc sau vài milliseconds. Đủ lâu với anh em, không đủ với Rust. Nó như bạn bảo "tôi sẽ yêu em mãi mãi" — ý tốt đấy, nhưng mà mãi mãi là bao lâu?

### 2. NLL — Cứu Tinh Của Giới Lập Trình

Trước Rust 2018, lifetime lexical — kéo dài tới hết scope `{}`. Từ 2018 có NLL (Non-Lexical Lifetime): lifetime kết thúc ngay khi không còn xài nữa.

```rust
fn main() {
    let mut s = String::from("hello");
    let r = &s;          // mượn immutable — chỉ đọc thôi
    println!("{}", r);   // xài lần cuối, r hết tác dụng ở đây
    
    let r2 = &mut s;     // OK! vì r không còn ai xài nữa
    r2.push_str(" world");
    println!("{}", s);    // check: hé lô world
}
```

Trước NLL, code này báo lỗi. Giờ chạy ngon. Một trong những cải tiến làm Rust "dễ thở" hơn. Như từ xe ôm lên Grab — cùng một việc nhưng đỡ khổ hơn nhiều.

---

## Code Review — Khi Nào Dùng Lifetime, Khi Nào Bỏ

Câu hỏi thực tế nhất. Kinh nghiệm 2 năm chiến đấu với Rust của mình:

| Tình huống | Nên dùng | Lý do |
|---|---|---|
| Function nhận &str, trả về &str | Lifetime (elision) | Không cần viết gì, compiler lo |
| Function nhận 2+ &str, trả về &str | Lifetime (`'a`) | Compiler không biết chọn thằng nào |
| Struct lưu tham chiếu | Lifetime (`'a`) | Bắt buộc — không có là compile lỗi |
| Struct đơn giản, data cố định | `String` / `Box` | Dễ đọc hơn, không đau đầu |
| Config, toàn string | `String` | Đừng cố ra vẻ ngầu |
| Parser, view layer | Lifetime | Nhẹ, không copy — performance tốt |

Quy tắc ngón tay cái của mình: **Thấy mình thêm lifetime không hiểu tại sao — dùng owned type**. Đừng cố ra vẻ ngầu bằng `&'a str` khi không cần. Sức mạnh không đến từ việc xài lifetime — nó đến từ biết khi nào cần và khi nào là over-engineering.

Viết code mà người đọc sau không hiểu — khác gì viết văn xuôi bằng tiếng Pháp cho thằng bạn chỉ biết tiếng Việt? Ngầu đấy, nhưng mà để làm gì?

---

## Tôi Ngồi Lại Và Nghĩ Về Những Gì Đã Qua

Hồi đầu năm ngoái, có một đêm tôi không ngủ được. Sếp giao cái task refactor module xử lý config — toàn bộ code cũ dùng `String` clone đi clone lại, chạy chậm như rùa bò qua sa mạc.

Tôi nghĩ: "OK, chuyển qua `&str` hết, dùng lifetime, performance sẽ tăng vọt."

3 giờ sáng. Tôi ngồi một mình trong phòng trọ ở quận Bình Thạnh. Màn hình đầy lỗi lifetime. Compiler báo lỗi hết cái này tới cái khác. Tôi sửa một chỗ — vỡ ba chỗ khác. Như trò whack-a-mole — đập con này, con khác chui lên.

Lúc đó tôi cay lắm. Cay compiler. Cay Rust. Cay cả cái thằng đã nghĩ ra khái niệm lifetime.

Tôi mở tủ lạnh, lấy lon bia Sài Gòn, ra ban công hút một điếu. Nhìn xuống đường Nguyễn Huệ vắng tanh, đèn đường vàng vọt, tôi tự hỏi: "Sao không làm Java hay Go cho nó lành? Mấy thằng bạn học cùng ngày xưa làm Java giờ lương 3-4K, tối về ngủ ngon. Mình thì 2 giờ sáng ngồi chiến đấu với compiler."

Nhưng rồi tôi nhận ra một điều.

Cũng như hồi học đại học — tôi ghét môn Cấu Trúc Dữ Liệu vì bắt code linked list, tree, graph bằng tay. Cảm giác "học để làm gì, đời có dùng đâu". Nhưng 5 năm sau, chính mấy cái linked list đấy giúp tôi hiểu Rust ownership. Chính cái tree traversal giúp tôi debug memory leak.

Cái khó ngày hôm nay — mai kia sẽ là sức mạnh của mình.

Tôi xuống bếp pha thêm ly cà phê. Ngồi lại. Hít một hơi. Và viết từng dòng lifetime một, chậm rãi, có hiểu mới gõ.

Sáng hôm sau, code chạy. Performance tăng 40%. PR được approve.

Tôi đóng laptop, ra ngoài mua ổ bánh mì thịt nguội trứng ốp la ở đầu hẻm — một trong những bữa sáng ngon nhất đời tôi.

---

## Tổng Kết

Học lifetime giống như sống chung với người bạn khó tính.

Lúc đầu — khó chịu. Bực mình. Muốn bỏ cuộc. Compiler nhai đi nhai lại "does not live long enough".

Giữa đường — bắt đầu hiểu pattern. Thấy cái hay. Tin tưởng compiler hơn. Như người bạn khó tính ấy — lúc quen rồi mới thấy họ là người tốt nhất, chỉ là họ không biết cách thể hiện.

Về sau — không thể tưởng tượng code mà không có lifetime. Nhìn lại code C++ ngày xưa thấy nó như bãi mìn. Sợ. Từng hồn nhiên `int* p = &local;` — giờ nghĩ lại thấy mình liều vkl.

Trên Viblo tôi đọc được một comment. Lúc đấy thấy nó sáo rỗng. Giờ thì tâm phục khẩu phục:

> *"Lifetime không phải để Rust kiểm tra code của anh em — nó là thứ giúp compiler hiểu ý đồ, để nó không bao giờ để anh em tự bắn vào chân mình."*

Đúng vậy. Compiler không phải kẻ thù. Nó là thằng bạn khó ưa nhưng trung thành. Có thể nó hơi phiền, hơi cứng nhắc, nhưng mà nó sẽ không bao giờ để mày ngã.

Chốt: Anh em không cần nhớ hết variance, coercion, NLL internals. Chỉ cần nhớ một điều: **viết lifetime khi có nhiều input trả về tham chiếu, hoặc struct chứa tham chiếu**. Còn lại để compiler lo.

Và nếu thấy mệt — cứ dùng `String`. Không ai phán xét đâu.

Tôi cũng từng làm thế.

Có hôm đang ngồi code lifetime, bà chủ quán cà phê cóc hỏi: "Anh gõ gì mà mặt căng thẳng thế?" Tôi bảo: "Dạ, con đang chiến đấu với lifetime ạ." Bà cười: "Chắc tại anh chưa có người yêu." Ừ, cũng có thể. Nhưng ít nhất, code của tôi không bao giờ bị dangling pointer.
