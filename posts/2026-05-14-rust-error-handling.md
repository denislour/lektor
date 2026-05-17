---
title: "Tôi từng ghét error handling — Cho đến khi production sập lúc 3 giờ sáng"
date: "2026-05-14"
tags: ["rust"]
readTime: 8
thumbnailCode: |-
  fn main() -> Result<(), Box<dyn std::error::Error>> {
      // Production không chết lúc 3h sáng
      // Nếu có lỗi thì return, không phải panic
      Ok(())
  }
---

3 giờ sáng. Điện thoại rung.

Tôi mở mắt. Nhìn màn hình. Slack đỏ rực.

"Replication lag quá 30 giây. Service timeout hàng loạt."

Production chết. Mồ hôi tay ướt cả bàn phím. Tôi mò lên server, nhìn stack trace. 200 dòng unwrap() của chính mình.

Đm.

Mồ hôi. Tay run. Tim đập 150 nhịp/phút.

Tôi không nhớ mình đã viết bao nhiêu cái `.unwrap()` trong cái service đó. Nhưng 3 giờ sáng, tôi nhớ từng cái. Từng cái một.

Đấy là lúc tôi quyết định: học error handling Rust cho tử tế, hoặc bỏ nghề về bán bánh mì và không bao giờ phải nhìn stack trace đỏ rực lúc 3 giờ sáng khi cả team đang ngủ và chỉ có mình tôi thức với một cái server sắp chết và con mèo nhìn tôi như thể bảo mày cũng vô dụng như cái unwrap() kia vậy.

## Option — Có hoặc không, đừng có unwrap

Tôi từng nghĩ: "À, chỗ này chắc chắn có user, unwrap cái là xong."

Thằng nào nói thế là thằng đấy chưa deploy lên production.

`Option<T>` trong Rust rất đơn giản — nó là một enum: `Some(T)` hoặc `None`. Nhưng cái hay là: **trình biên dịch bắt mày phải xử lý cả hai trường hợp.** Không có kiểu "chắc chắn có" ở đây.

```rust
// Hàm tìm user — xài trong API production hằng ngày
fn find_user(users: &[User], id: u32) -> Option<&User> {
    users.iter().find(|u| u.id == id)
    // Nếu không có → None. Đơn giản vkl.
}
```

Hồi xưa tôi viết thế này:

```rust
// Cách của thằng sắp phá production
let user = find_user(&users, 42).unwrap();
// Nếu không có user 42 → panic. Boom.
```

Xong phim.

Chết.

Thì ra cái thằng `unwrap()` chỉ nên dùng khi nào? **Không bao giờ.** À mà có một ngoại lệ: khi mày viết prototype lúc 2 giờ chiều, chưa uống cà phê, và biết chắc là sẽ sửa lại sau. Còn lại thì đừng.

Cách đúng:

```rust
// Cách của thằng muốn ngủ ngon
let user = find_user(&users, 42).unwrap_or_else(|| {
    // Log + fallback. Production không panic
    eprintln!("⚠️ User 42 không tồn tại — dùng default");
    &User::default()
});

// Hoặc map + and_then — chaining mượt như bún bò
let role = find_user(&users, 42)
    .map(|u| u.role.as_str())
    .unwrap_or("guest");

println!("User role: {}", role);
// Output: User role: admin (hoặc guest nếu không có user)
```

`map` biến `Some` thành `Some` khác, `None` thành `None`. `unwrap_or` cho giá trị mặc định. Combinator không phải là xa xỉ — nó là survival skill cho bất kỳ ai đã từng nếm mùi production 3 giờ sáng khi không biết cái unwrap() nào đang phá server và đồng nghiệp nhắn tin chửi vì không thể ngủ được.

## Result — Lỗi là giá trị, không phải exception

Python có try/except. Java có try/catch. Rust có... một enum.

Chỉ một enum.

Nhưng cái enum đó thay đổi cách cả thế giới nghĩ về lỗi — từ thứ gì đó sợ hãi thành thứ gì đó bình thường, từ exception bất ngờ thành giá trị có thể dự đoán và kiểm soát như bất kỳ biến nào khác trong code của bạn.

`Result<T, E>` = `Ok(T)` hoặc `Err(E)`. Lỗi không phải thứ gì đó ném ra từ đâu — nó là giá trị trả về, như số 42 hay chữ "hello".

Nghe hơi lạ. Nhưng production thực tế nó thay đổi cách mày nghĩ về lỗi.

```rust
use std::fs;
use std::io::{self, Read};
use std::path::Path;

// Đọc file config — nếu lỗi thì trả về Err
fn read_config(path: &str) -> Result<String, io::Error> {
    let mut file = match fs::File::open(path) {
        Ok(f) => f,
        Err(e) => return Err(e),  // Tường minh. Không có magic.
    };
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    // ? ở trên = match Err(e) → return Err(From::from(e))
    // Tự động convert lỗi. Kẹo ngọt.
    Ok(contents)
}
```

Thấy chưa? Không có stack trace đỏ rực ở runtime. Code compile được là mày biết mọi path lỗi đã được handle — hoặc được propagate về caller.

### Combinators — đừng match mọi thứ

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = read_config("config.toml")
        .map_err(|e| format!("Không đọc được config: {e}"))?;

    let timeout = config
        .lines()
        .find(|l| l.starts_with("timeout="))
        .and_then(|l| l.split('=').nth(1))
        .and_then(|v| v.trim().parse::<u64>().ok())
        .unwrap_or(30);

    println!("⏱️ Timeout: {}s", timeout);
    // Output: ⏱️ Timeout: 30s (hoặc giá trị từ file)
    Ok(())
}
```

`.map_err()` convert lỗi. `.and_then()` chain fallible ops. `.unwrap_or()` fallback.

97.3% khảo sát từ 3 thằng ngồi nhậu ở quán cà phê cóc Nguyễn Huệ cho thấy: dùng combinator làm code ngắn hơn 60% và giảm unwrap() xuống gần zero.

## `?` — Operator cứu mạng

Toán tử `?` là phát minh vĩ đại nhất của Rust, sau ownership.

```rust
// Nếu không có ? — viết tay. 30 dòng.
fn process_old_way() -> Result<i32, MyError> {
    let file_content = match read_file("data.txt") {
        Ok(c) => c,
        Err(e) => return Err(MyError::Io(e)),
    };
    let parsed = match file_content.trim().parse::<i32>() {
        Ok(n) => n,
        Err(e) => return Err(MyError::Parse(e)),
    };
    let result = match db_query(parsed) {
        Ok(r) => r,
        Err(e) => return Err(MyError::Db(format!("Query failed: {e}"))),
    };
    Ok(result)
}
```

Mệt vkl.

```rust
// Có ? — 7 dòng. Sướng như uống trà sữa trân châu.
fn process_new_way() -> Result<i32, MyError> {
    let content = read_file("data.txt")?;    // Err → return auto
    let number = content.trim().parse::<i32>()?;  // Err → auto convert
    let result = db_query(number)?;          // Err → auto convert
    Ok(result)                               // Ok → wrap trong Ok
}
```

Mỗi dấu `?` là một: "nếu Ok thì unwrap, nếu Err thì return."

Đơn giản vkl.

Cơ chế From trait tự động convert giữa các error types — nếu error type của bạn implement `From<io::Error>`, thì `?` trên `read_file` tự gọi `From::from`, biến mọi loại lỗi rời rạc từ đọc file, parse JSON, query database, gọi API bên ngoài, tất cả thành một error type thống nhất mà không cần match tay từng cái một.

## Custom errors với thiserror — Từ gà lên pro

Khi service của bạn có 5-10 loại lỗi khác nhau — IO, parse, network, validation, business logic — bạn cần một error type thống nhất.

Không có thiserror, bạn phải viết tay:

```rust
// Cách truyền thống — viết tay. 20 dòng boilerplate.
#[derive(Debug)]
enum MyError {
    Io(std::io::Error),
    Parse(std::num::ParseIntError),
    NotFound(String),
    DbQuery(String),
}

impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            MyError::Io(e) => write!(f, "IO error: {e}"),
            MyError::Parse(e) => write!(f, "Parse error: {e}"),
            MyError::NotFound(msg) => write!(f, "Not found: {msg}"),
            MyError::DbQuery(msg) => write!(f, "DB query error: {msg}"),
        }
    }
}
```

Mệt vkl × 2.

Mệt.

Với `thiserror`:

```rust
use thiserror::Error;

#[derive(Error, Debug)]
enum ApiError {
    #[error("IO lỗi rồi anh ơi: {0}")]
    Io(#[from] std::io::Error),

    #[error("Parse không được nha: {0}")]
    Parse(#[from] std::num::ParseIntError),

    #[error("Không tìm thấy: {0}")]
    NotFound(String),

    #[error("DB die cmnr: {0}")]
    Database(#[from] sqlx::Error),
}
```

`#[from]` tự động implement `From<T>` — mày không cần viết `impl From<io::Error> for ApiError` nữa. Một dòng macro thay 6 dòng boilerplate.

Sau đó xài `?` ngon lành. chúng ta cùng xem:

```rust
async fn get_user_profile(db: &sqlx::Pool, user_id: i32) -> Result<UserProfile, ApiError> {
    // read_file → ApiError::Io
    let config = read_config("app.toml")?;

    // parse → ApiError::Parse
    let max_retries = config.lines()
        .find(|l| l.starts_with("max_retries="))
        .and_then(|l| l.split('=').nth(1))
        .unwrap_or("3")
        .parse::<i32>()?;

    // sqlx query → ApiError::Database
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(user_id)
        .fetch_optional(db)
        .await?
        .ok_or_else(|| ApiError::NotFound(format!("User {user_id} không tồn tại")))?;

    Ok(UserProfile {
        id: user.id,
        name: user.name,
        retries: max_retries,
    })
}
```

`.ok_or_else()` convert `Option` → `Result`. Nếu `None`, trả về `Err(ApiError::NotFound(...))`.

### Kết hợp với mã lỗi HTTP

```rust
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            ApiError::NotFound(m) => (StatusCode::NOT_FOUND, m.clone()),
            ApiError::Validation(m) => (StatusCode::BAD_REQUEST, m.clone()),
            ApiError::Database(_) => (
                StatusCode::SERVICE_UNAVAILABLE,
                "DB lỗi — thử lại sau 5 giây".to_string(),
            ),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Lỗi không xác định — gọi SRE gấp".to_string(),
            ),
        };
        (status, message).into_response()
    }
}
```

Production-ready. Không có `unwrap()` nào lọt qua khe.

các bạn thấy không? Từ 20 dòng match tay xuống 3 dòng macro. Tôi không nói đây là silver bullet. Nhưng tôi dám nói: team nào xài thiserror sớm thì đêm đó ngủ ngon hơn, thành viên ít cãi nhau hơn, và code review cũng bớt drama hơn hẳn.

## Box<dyn Error> — Dành cho thằng lười (và prototype)

Mày mới học Rust? Mày chỉ muốn code chạy trước khi muốn clean?

```rust
fn quick_and_dirty() -> Result<i32, Box<dyn std::error::Error>> {
    let a = std::fs::read_to_string("file.txt")?;  // io::Error → Box<dyn Error>
    let b = a.trim().parse::<i32>()?;               // ParseIntError → Box<dyn Error>
    Ok(b * 2)
}
```

`Box<dyn std::error::Error>` là type erasure — nó chứa bất kỳ error nào implement `std::error::Error`. Dễ xài. Nhưng mất thông tin type ở compile time — match pattern không được.

**Khi nào xài?** CLI tool nhỏ, prototype, script. **Khi nào không?** Library, production API, multi-service architecture.

## Kết luận — Không lẽ đi bán bánh mì thật?

Đêm đó, sau 3 tiếng ngồi xóa từng cái `.unwrap()` thay bằng `?` + `thiserror` + match xịn, tôi deploy lại.

4 giờ sáng. Server xanh trở lại. Slack im.

Tôi ngồi nhìn màn hình, tay cầm ly cà phê phin nguội ngắt. Và tôi nhận ra: error handling trong Rust không phải để làm khó mình. Nó là để mình ngủ ngon.

Option cho "có hoặc không" — dùng khi giá trị có thể không tồn tại, như tìm user theo email, hoặc key trong HashMap.

Result cho "thành công hoặc thất bại" — dùng khi mọi thứ có thể hỏng, từ đọc file đến query database.

`?` cho code gọn — giảm match boilerplate, tăng readability.

`thiserror` cho custom errors — biến error handling từ "đau khổ" thành "dễ thương".

Còn `unwrap()`? Dùng trong test. Dùng trong prototype. Dùng trong code mà mày biết chắc là sẽ không ai maintain sau 2 tuần. Còn lại? Đừng.

Tôi không nói error handling Rust là dễ. Nó khó. Nó khác. Nó bắt mày nghĩ về lỗi từ lúc viết code, chứ không phải lúc 3 giờ sáng khi production chết.

Nhưng mà nếu không có nó, cái service của tôi chắc chắn đã chết thêm 3 lần nữa. Và tôi chắc chắn đã đi bán bánh mì thật rồi.

À mà bán bánh mì cũng không dễ hơn đâu. Có hôm ế khách cũng panic như unwrap() vậy. Không có `?` cho đời.

Nhưng ít nhất bán bánh mì không có compile error lúc 2 giờ sáng. Vậy nên tôi vẫn ở lại với Rust.

Code mẫu đầy đủ trên [GitHub](https://github.com/jake/rust-error-handling-demo) nếu anh em muốn chạy thử. Nhớ star nha, star là động lực để tôi viết tiếp.

🦀

---

*Bài viết được tham khảo từ Viblo, TopDev, và kinh nghiệm xương máu của chính tôi. Đã đọc 3 lần trước khi publish. Vẫn có thể sai — comment để tôi sửa.*
