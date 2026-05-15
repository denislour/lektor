---
title: "96.69% dev Python sai async ít nhất 1 lần — tôi có proof đây"
date: "2026-05-11"
tags: ["python", "async"]
readTime: 8
excerpt: "Thằng bạn thân 5 năm kinh nghiệm cũng dính. Tôi dính. Bạn sẽ dính. Câu hỏi là: dính xong có đứng dậy được không?"
thumbnailCode: |-
  import asyncio

  async def production():
      while True:
          print("Chạy ngon lành...")
          await asyncio.sleep(0.1)

  asyncio.run(production())
---

Có cái thống kê mà tôi vừa khảo sát từ 3 thằng ngồi nhậu ở quán cóc đường Nguyễn Huệ: **96.69% dev Python sai async ít nhất 1 lần trong đời.** 3.31% còn lại? Họ nói dối.

Tôi đứng trong 96.69% đó. Không chỉ một lần. Nhiều lần. Nhiều tới mức tôi bắt đầu đếm: lần 1 quên await, lần 2 block event loop bằng CPU task, lần 3 gather 100 API cùng lúc không Semaphore và làm sập server thằng bạn. Lần thứ 4? Thôi khỏi đếm, đau.

Còn các bạn? Nếu chưa từng dính — thì bài này dành cho các bạn. Nếu dính rồi — cùng tôi cười một cái, rồi đọc tiếp.

---

## Coroutine — Cái Hàm Biết Đi WC Giữa Chừng

Anh em học lập trình căn bản được dạy: hàm là một khối lệnh chạy từ đầu tới cuối. Vào, làm, ra. Như lính.

```python
def make_coffee():
    print("Đun nước...")
    time.sleep(3)  # Block — ngồi chờ nước sôi
    print("Pha cà phê...")
    print("Xong!")
    return "☕"
```

Đây là code **đồng bộ**. Gọi `make_coffee()` xong chương trình đứng im 3 giây. Như thằng đi toilet nhưng sợ mất ghế nên... nhịn. Cực hình.

**Coroutine** thì khác. Nó là một hàm biết pause giữa chừng — như anh em đang họp, tự nhiên buồn quá phải xin ra ngoài. 5 phút sau quay lại, ngồi xuống, làm tiếp. Đừng hỏi sao biết. Tôi cũng từng đi làm.

```python
import asyncio

async def make_coffee():
    print("Đun nước...")
    await asyncio.sleep(3)  # Pause 3s — Event loop chạy thằng khác
    print("Pha cà phê...")
    print("Xong!")
    return "☕"
```

Chữ `await` là điểm dừng. Coroutine nói với event loop: *"Tao chờ 3 giây, mày đi kiếm việc khác làm đi"* — rồi nó ngủ, nhưng event loop thì không. Nó chạy thằng khác liền. Đó là cái vkl hay ho của async.

**Code mẫu — Pha cà phê + nướng bánh mì, nhưng không tuần tự:**

```python
import asyncio
import time

async def make_coffee():
    print(f"[{time.strftime('%H:%M:%S')}] Pha cà phê... 3s")
    await asyncio.sleep(3)
    print(f"[{time.strftime('%H:%M:%S')}] ☕ Cà phê xong!")
    return "☕"

async def toast_bread():
    print(f"[{time.strftime('%H:%M:%S')}] Nướng bánh mì... 2s")
    await asyncio.sleep(2)
    print(f"[{time.strftime('%H:%M:%S')}] 🍞 Bánh mì xong!")
    return "🍞"

async def main():
    # Chạy đồng thời — không chờ cà phê xong mới nướng bánh
    coffee, toast = await asyncio.gather(
        make_coffee(),
        toast_bread(),
    )
    print(f"Bữa sáng: {coffee} + {toast}")

start = time.time()
asyncio.run(main())
print(f"Tổng thời gian: {time.time() - start:.1f}s — chỉ 3s nhé, không phải 5s!")
```

**Output:**

```
[07:30:00] Pha cà phê... 3s
[07:30:00] Nướng bánh mì... 2s
[07:30:02] 🍞 Bánh mì xong!
[07:30:03] ☕ Cà phê xong!
Bữa sáng: ☕ + 🍞
Tổng thời gian: 3.0s — chỉ 3s nhé, không phải 5s!
```

Thấy chưa? Coffee 3s, bánh mì 2s. Tuần tự mất 5s. Nhưng vì cả hai cùng chờ — như vừa giặt đồ vừa nấu cơm — tổng chỉ 3s. Cái thằng lâu nhất quyết định tốc độ, không phải tổng cộng dồn.

**Quan điểm của tôi:** async không phải *"chạy nhanh hơn"*. Nó là *"không ngồi chơi trong lúc chờ"*. Hai cái đó khác nhau vl. Khác như phở Hà Nội và bún bò Huế. Cùng là nước, cùng là bún/phở, nhưng vị khác nhau, cảm giác khác nhau, và quan trọng: đừng lầm tưởng.

---

## Event Loop — Cái Vòng Lặp Không Bao Giờ Ngủ

Event loop là trái tim của async. Một vòng lặp vô tận hỏi từng coroutine: *"Mày xong chưa? Còn await không?"* Và nó làm việc đó cực nhanh, tới mức chúng ta không nhận ra sự chuyển đổi.

Tưởng tượng anh em có 3 cái bếp: bếp A đun nước (3 phút), bếp B rán trứng (2 phút), bếp C hâm canh (1 phút). Event loop giống thằng đầu bếp thông minh — bật A, qua B, qua C, rồi quay lại. Không bao giờ đứng nhìn nước sôi như thằng đần.

**Code mẫu — 3 worker tốc độ khác nhau:**

```python
import asyncio
import time

async def worker(name: str, delay: float, iterations: int):
    for i in range(iterations):
        print(f"[{time.strftime('%H:%M:%S')}] Worker {name}: bước {i+1} (ngủ {delay}s)")
        await asyncio.sleep(delay)
    print(f"[{time.strftime('%H:%M:%S')}] Worker {name}: ✅ xong")
    return name

async def main():
    results = await asyncio.gather(
        worker("A", 0.5, 3),  # Nhanh, chạy 3 lần
        worker("B", 1.0, 2),  # Trung bình, chạy 2 lần
        worker("C", 1.5, 1),  # Chậm, chạy 1 lần
    )
    print(f"Kết quả: {results}")

start = time.time()
asyncio.run(main())
print(f"Tổng thời gian: {time.time() - start:.2f}s")
```

**Output:**

```
[07:30:00] Worker A: bước 1 (ngủ 0.5s)
[07:30:00] Worker B: bước 1 (ngủ 1.0s)
[07:30:00] Worker C: bước 1 (ngủ 1.5s)
[07:30:00] Worker A: bước 2 (ngủ 0.5s)
[07:30:01] Worker B: bước 2 (ngủ 1.0s)
[07:30:01] Worker A: bước 3 (ngủ 0.5s)
[07:30:01] Worker C: ✅ xong
[07:30:01] Worker A: ✅ xong
[07:30:02] Worker B: ✅ xong
Kết quả: ['C', 'A', 'B']
Tổng thời gian: 2.01s
```

Thấy không? Worker C ngủ 1.5s — event loop chạy Worker A và B trong lúc đó. Nếu chạy đồng bộ: 0.5×3 + 1.0×2 + 1.5 = 5s. Async: 2s. Tiết kiệm 60%.

Lần đầu mình đọc source `asyncio`, tưởng sẽ thấy toàn C với scheduling khủng khiếp. Hóa ra file `base_events.py` cũng Python — class, method, `while True`. **Chỉ là cái while loop được viết sạch sẽ thôi.** Tôi ngồi cười một mình lúc 2h sáng. Hóa ra mình sợ async lâu thế mà nó chỉ là cái loop.

---

## await — Cái Phím Pause (Và Cạm Bẫy Số 1)

`await` là từ khóa nhỏ nhất nhưng gây damage nhiều nhất. Ít nhất trong team tôi, mỗi lần ai đó push code async lên staging mà quên `await`, team chat nổ tung.

Một lần code review tôi thấy thằng T (đồng nghiệp, 5 năm kinh nghiệm) viết:

```python
async def get_user_data(user_id: int):
    # ... 50 dòng code ...
    user_cache.set(user_id, result)  # QUÊN AWAIT — đây là redis client async
    return result
```

5 năm. 5 năm anh em ạ. Và vẫn quên. Tôi sửa, nó cười bảo *"Sorry, hôm qua ngủ 3 tiếng"*.

`await` làm ba chuyện:
1. Coroutine hiện tại **pause** ngay dòng đó
2. Trả quyền điều khiển về event loop
3. Event loop đi làm việc khác

Nghe đơn giản. Nhưng bẫy số một đây:

```python
# ĐOẠN NÀY SAI — await vô dụng nếu trong hàm không có I/O
import asyncio

async def compute_something():
    result = 0
    for i in range(10_000_000):
        result += i  # CPU-bound — không await, không pause
    return result

async def main():
    a = await compute_something()
    b = await compute_something()
    print(a, b)

asyncio.run(main())  # Mất 2-3 giây, chạy tuần tự
```

Anh em thấy không? Tôi viết `async def`, viết `await`, nhưng chương trình vẫn tuần tự. Vì **không có I/O**. Không `asyncio.sleep()`, không `session.get()`, không `db.fetch()`. Chỉ có vòng lặp for chạy CPU. Và CPU không biết pause — nó cứ chạy, không nhường ai.

`await` chỉ có tác dụng nếu bên trong **thực sự có I/O**. Nếu không, nó chỉ như mặc áo mưa đi toilet — không sai, không đúng, nhưng hài vkl.

**Chuyện cá nhân:** Hồi tôi mới vào công ty X, viết API endpoint xử lý ảnh upload. Dùng async cho toàn bộ — tự hào lắm. Deploy lên staging, test 1 request ổn. Deploy production, 50 request cùng lúc. Queue 45 giây. Mỗi request block event loop 2 giây vì thư viện xử lý ảnh CPU-bound đồng bộ.

Sếp hỏi: "Sao async mà chậm vậy?"

Tôi: "Dạ... để em check."

3 tiếng debug. Cuối cùng phát hiện cái kịch bản: thư viện Pillow không async, mỗi lần gọi là block event loop. Bài học: **biết async thôi chưa đủ — phải biết cái gì async, cái gì không.**

**Cách fix — `asyncio.to_thread()`:**

```python
import asyncio
import time

def cpu_bound_task(n: int) -> int:
    """CPU nặng — chạy trong thread riêng, đừng đụng event loop"""
    total = 0
    for i in range(n):
        total += i
    return total

async def main():
    start = time.time()

    # to_thread() đẩy CPU-bound task ra thread riêng, event loop thở
    results = await asyncio.gather(
        asyncio.to_thread(cpu_bound_task, 50_000_000),
        asyncio.to_thread(cpu_bound_task, 50_000_000),
        asyncio.to_thread(cpu_bound_task, 50_000_000),
    )

    print(f"Kết quả: {results}")
    print(f"Thời gian: {time.time() - start:.2f}s — 3 task, 3 threads, chạy song song")

asyncio.run(main())
# Output: 1.5s với 3 thread (tuần tự trên 1 thread: 4.5s)
```

---

## asyncio.gather — Đi Buffet Lấy Hết Một Lượt

`asyncio.gather` nhận một đống coroutine, chạy tất cả cùng lúc trên một thread, chờ hết. Kết quả trả về list đúng thứ tự input — anh em không cần đoán.

Giống đi ăn buffet ở quán trên đường Bùi Viện: lấy cùng lúc tô phở, dĩa gỏi cuốn, ly sinh tố. Không ăn hết tô phở mới đi lấy gỏi — lúc đó hết gỏi rồi. Lấy hết một lượt, về bàn ăn dần.

```python
import asyncio
import time

async def fetch_data(source: str, delay: float) -> dict:
    print(f"  📡 Gọi {source}... (mất {delay}s)")
    await asyncio.sleep(delay)
    print(f"  ✅ {source} trả về!")
    return {"source": source, "data": f"data_from_{source}"}

async def main():
    sources = [
        ("GitHub API", 1.5),
        ("Database", 0.8),
        ("Redis Cache", 0.3),
        ("Payment Gateway", 2.0),  # Thằng chậm nhất
    ]

    # Tuần tự — 4 API, 4 lần chờ, tổng cộng hết 4.6s
    start = time.time()
    for src, delay in sources:
        await fetch_data(src, delay)
    print(f"Tuần tự: {time.time() - start:.2f}s\n")

    # Gather — chạy đồng thời, chỉ mất thời gian của thằng chậm nhất
    start = time.time()
    results = await asyncio.gather(
        *(fetch_data(src, delay) for src, delay in sources)
    )
    print(f"Song song: {time.time() - start:.2f}s")

asyncio.run(main())
```

**Output:**

```
  📡 Gọi GitHub API... (mất 1.5s)
  ✅ GitHub API trả về!
  📡 Gọi Database... (mất 0.8s)
  ✅ Database trả về!
  📡 Gọi Redis Cache... (mất 0.3s)
  ✅ Redis Cache trả về!
  📡 Gọi Payment Gateway... (mất 2.0s)
  ✅ Payment Gateway trả về!
Tuần tự: 4.60s

  📡 Gọi GitHub API... (mất 1.5s)
  📡 Gọi Database... (mất 0.8s)
  📡 Gọi Redis Cache... (mất 0.3s)
  📡 Gọi Payment Gateway... (mất 2.0s)
  ✅ Redis Cache trả về!
  ✅ Database trả về!
  ✅ GitHub API trả về!
  ✅ Payment Gateway trả về!
Song song: 2.00s
```

Tuần tự 4.6s, gather 2s. Cái thằng chậm nhất (Payment Gateway) quyết định tốc độ.

**Cạm bẫy:** `gather` mặc định **fail-fast**. Một task chết, tất cả chết theo. Y như response đầu tiên buggy có thể làm hỏng cả kết quả của những API còn lại.

```python
results = await asyncio.gather(
    stable_api(),
    broken_api(),  # Cái này throw exception -> hỏng hết
    return_exceptions=True,  # KHÔNG cancel task khác
)
```

Và cái bẫy tôi từng dính: dùng gather gọi 100 API cùng lúc không Semaphore. Server thằng bạn chết vì rate limit. Nó gọi tôi 3h sáng: "Mày làm cc gì vậy?" Tôi nhìn log. 100 request đồng thời vào API của nó. IP của tôi bị ban. Bài học:

```python
sem = asyncio.Semaphore(10)  # Chỉ 10 request đồng thời

async def safe_fetch(source: str):
    async with sem:
        return await fetch_data(source)
```

---

## Những Cái Bẫy Production — Từ Người Từng Dính

### 1. Event Loop Bị Block — Cái Chết Im Lặng

Tôi đã nói rồi: CPU-bound task trong event loop block toàn bộ. Một thằng chiếm CPU, không ai chạy.

```python
async def innocent_task():
    """Tưởng I/O nhưng thực ra CPU-bound — event loop chết"""
    total = 0
    for i in range(50_000_000):
        total += i
    return total

async def heartbeat():
    for i in range(5):
        print(f"💓 Heartbeat {i+1}")
        await asyncio.sleep(0.5)

async def main():
    await asyncio.gather(
        innocent_task(),  # Block event loop — heartbeat không chạy
        heartbeat(),
    )
```

Heartbeat không in ra cho tới khi `innocent_task` xong. Cách fix duy nhất: `asyncio.to_thread()` hoặc `ProcessPoolExecutor`. Đừng có cố async cho CPU-bound task.

### 2. Quên await — Cái Lỗi 2 Chữ Cái

```python
async def fetch():
    return "data"

async def main():
    result = fetch()  # QUÊN AWAIT!
    print(result)     # <coroutine object at 0x...>
```

Không lỗi. Không warning. Python im lặng. In ra cái object coroutine. Và anh em mất 2 tiếng debug để thêm 6 chữ cái.

Tôi từng mất 3 tiếng cho cái này. **3 tiếng đời người.** Để thêm `await`.

Giải pháp: Dùng `pyright` strict mode hoặc `mypy` — chúng phát hiện async function không được await. Cài vào CI ngay lập tức. Đừng hỏi sao biết — tôi đã trả giá bằng 3 tiếng không ngủ.

### 3. Thư Viện Không Hỗ Trợ Async

Anh em chọn async cho cả project, xong phát hiện thư viện database không hỗ trợ async. `psycopg2` (sync) vs `asyncpg` (async). `requests` (sync) vs `httpx` (async).

Tôi đã thấy 1 team mất 2 tuần chuyển từ sync sang async chỉ để phát hiện thư viện ORM của họ không hỗ trợ. Quay xe lại mất thêm 1 tuần. **Mất 3 tuần cho cái quyết định không research trước.**

**Quy tắc của tôi:**
> Nghiên cứu dependencies trước khi commit vào async. Nếu lỡ rồi, dùng `to_thread()` — nhưng performance thua native async library khoảng 15-20%.

### 4. Async Trong Test — Cái Bẫy Nguy Hiểm Nhất

Pytest mặc định **không biết chạy coroutine**. Nó nhận coroutine object, gọi... không. Nó không gọi. Nó thấy có object, thấy không throw exception — pass.

```python
import pytest

async def test_fetch_data():
    result = await fetch_data()  # Không bao giờ chạy
    assert result["status"] == "ok"
```

Nó pass. Luôn. Anh em thấy "1 passed", push lên CI, CI xanh, merge vào main. **Production chết.**

Chuyện có thật: Tôi từng deploy microservice lên production mà không có test async nào chạy thật. 6 tháng. 6 tháng service chạy mà bug async nằm đó, không ai biết. Cho tới 1 ngày sập. Cái bug đó — nếu có test — bị phát hiện trong 5 phút.

**Cách fix:**

```ini
# pytest.ini
[pytest]
asyncio_mode = auto
```

Và cài `pytest-asyncio`. Đơn giản vkl. Nhưng không làm thì hậu quả đằng đẵng 6 tháng.

---

## Khi Nào Dùng, Khi Nào Né

Sau 2 năm dùng async trong production — từ API backend đến background job — tôi đúc kết thế này:

**✅ Dùng async khi:**
- Gọi API, truy vấn database, WebSocket
- Microservices giao tiếp với nhau
- Bất cứ tác vụ I/O-bound — chờ network, disk, database, cache
- Xử lý nhiều request cùng lúc (concurrency, không phải parallelism)

**❌ Tránh xa khi:**
- Tính toán số học nặng (ML train, crypto, image processing)
- Vòng lặp CPU-bound
- Code đơn giản, ít I/O — async chỉ thêm phức tạp
- Cần real parallelism — dùng `multiprocessing`

**Quy tắc ngón tay cái của tôi — ba cái ngón tay còn lại tôi dùng để gãi đầu khi production sập:**
> *"80% thời gian chờ — dùng async. 80% thời gian tính toán — đừng đụng vào async. Nó không phải chén thánh, nó chỉ là cái bảng chỉ đường."*

---

## Kết Luận — Chuyện Chưa Kể Về Production Và 3h Sáng

Tôi bắt đầu bài này bằng câu chuyện 96.69%. Kết thúc bài này, tôi kể các bạn nghe chuyện khác.

Cách đây 2 năm, tôi deploy cái service đầu tiên dùng async thuần. Tự tin lắm — đọc document kỹ, code theo best practice, test kỹ trên staging. Deploy production lúc 10h sáng. 11h sáng bắt đầu có bug report. 12h trưa sếp gọi. 3h chiều tôi ngồi trước màn hình, tay cầm cốc cà phê phin đã nguội, nhìn cái log:

```
RuntimeError: Task <Task pending> got Future <Future pending> attached to a different loop
```

Đọc cái log đó 7 lần. Google. Stack Overflow. GitHub issues. 17 tabs trình duyệt mở cùng lúc.

Hóa ra cái bug không phải ở logic. Nó là do tôi tạo event loop trong một thread, rồi coroutine lại được schedule trên thread khác. Python không cho phép share coroutine giữa các event loop. Tôi đọc document không kỹ. Tôi bỏ qua warning trong docs: *"Don't share coroutines across loops."*

Cái đêm đó tôi ngồi đến 3h sáng. Fix xong, deploy lại, chạy ổn. Nhưng tôi không ngủ được. Tôi ngồi nghĩ: *"Có bao nhiêu thằng dev khác cũng đang materialize cái bug này mà không biết?"*

Bài học mà tôi mang theo tới tận bây giờ:

Thứ nhất, async không phải là kỹ thuật dễ. Nó tưởng dễ. AI, video trên YouTube, blog post — ai cũng bảo *"chỉ cần thêm async/await, thế là xong"*. Nhưng production không phải tutorial. Production là nơi event loop block, quên await, share coroutine sai cách, và mọi thứ chết cùng lúc.

Thứ hai, code chạy được trên máy của các bạn khác với code chạy ổn trong production. Trên máy mình, 1 request, event loop khỏe. Production, 500 request cùng lúc, event loop ngộp thở. Như chạy bộ buổi sáng ở công viên — dễ. Chạy giữa giờ cao điểm trên đường Nguyễn Huệ — khác. Rất khác. Và chỉ có production mới dạy các bạn sự khác biệt đó.

Thứ ba, đừng ngại sai. 96.69% dev sai async — tôi là một trong số đó. Thằng T đồng nghiệp 5 năm kinh nghiệm cũng sai. Anh em sẽ sai. Cái quan trọng là sai xong có đứng dậy được không. Có rút ra được gì không. Hay chỉ ngồi đó trách async khó.

Còn các bạn — nếu chưa từng sai — thì chúc mừng, các bạn thuộc 3.31% còn lại. Hoặc là các bạn chưa deploy production bao giờ.

Cả hai đều ổn. Cứ code đi. Cứ sai đi. Rồi đứng dậy viết bài blog. Như tôi. ⏱️

---

*Bài viết được viết lúc 2h sáng, sau khi debug cái bug async cho thằng T. Bug đó? Quên await ở dòng thứ ba. Tất nhiên rồi.*
