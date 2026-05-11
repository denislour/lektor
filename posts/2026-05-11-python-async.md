---
title: "Python async: Từ coroutine đến asyncio"
date: "2026-05-11"
tags: ["python", "async"]
readTime: 5
excerpt: "async/await trong Python không phải phép thuật. Nó chỉ là mấy cái hàm biết 'đi toilet' giữa chừng — pause rồi quay lại làm tiếp."
thumbnailCode: |-
  import asyncio

  async def fetch_data(url: str) -> dict:
    async with httpx.AsyncClient() as client:
      resp = await client.get(url)
      return resp.json()

  async def main():
    results = await asyncio.gather(
      fetch_data("https://api.one.com"),
      fetch_data("https://api.two.com"),
    )
    print(results)
---

# Python async: Từ coroutine đến asyncio

Anh em viết Python, chắc cũng nghe đồn *"Python chậm vì GIL"*. Đúng đấy, nhưng async không phải để làm Python nhanh hơn — nó để làm Python đỡ chờ đợi.

## Coroutine Là Gì? Giải Thích Cho Anh Em Dễ Hiểu

Hàm thường chạy từ đầu đến cuối. Coroutine thì biết dừng giữa chừng — như đi toilet giữa lúc làm việc vậy:

```python
import asyncio

async def make_coffee():
    print("Bắt đầu pha cà phê...")
    await asyncio.sleep(3)  # Chờ nước sôi — nhưng không block
    print("Cà phê xong!")
    return "☕"

async def toast_bread():
    print("Bắt đầu nướng bánh mì...")
    await asyncio.sleep(2)
    print("Bánh mì xong!")
    return "🍞"

async def main():
    # Làm đồng thời — cả hai cùng chạy
    coffee, toast = await asyncio.gather(
        make_coffee(),
        toast_bread(),
    )
    print(f"Bữa sáng: {coffee} + {toast}")

asyncio.run(main())
# Output:
# Bắt đầu pha cà phê...
# Bắt đầu nướng bánh mì...
# Bánh mì xong!
# Cà phê xong!
# Bữa sáng: ☕ + 🍞
```

Thấy chưa? Cả hai chạy gần như cùng lúc. Coffee mất 3s, bánh mì mất 2s, nhưng tổng thời gian chỉ ~3s — chứ không phải 5s. Vì trong lúc chờ nước sôi, anh em nướng bánh mì luôn.

## Event Loop — Thằng Dàn Xếp

Event loop là thằng quản lý: nó nhìn xem coroutine nào đang `await` cái gì, nếu đang chờ I/O thì nó chuyển sang coroutine khác ngay. Không ai phải ngồi không mà chờ.

```python
import asyncio
import time

async def worker(name: str, delay: float):
    for i in range(3):
        print(f"[{name}] Lần {i + 1}")
        await asyncio.sleep(delay)

async def main():
    # Cùng lúc chạy 3 worker — không tuần tự
    await asyncio.gather(
        worker("A", 0.5),
        worker("B", 1.0),
        worker("C", 1.5),
    )

start = time.time()
asyncio.run(main())
print(f"Tổng thời gian: {time.time() - start:.2f}s")
# Chạy 3 thằng trong ~3s chứ không phải 9s
```

Nếu chạy tuần tự (await từng thằng một), 3 worker mất 0.5+1.0+1.5 = 3 lần lặp × tổng delay = 9s. Chạy song song kiểu gather thì chỉ mất ~3s thôi — tiết kiệm hơn đi ăn sáng.

> *"Async không phải multi-threading. Nó là một thread nhưng biết không lãng phí thời gian ngồi chờ I/O."*

## await Là Gì? Dễ Như Ăn Kẹo

`await` là điểm dừng. Khi anh em `await` một coroutine, event loop nói: *"Mày ngồi chờ đi, tao đi làm việc khác"*. Xong việc rồi quay lại — như đang nấu mì gói thì chạy đi giặt đồ, nước sôi tắt bếp rồi ra giặt tiếp.

```python
async def fetch_user(user_id: int):
    # await: tạm dừng ở đây, event loop chạy thằng khác
    data = await db.query("SELECT * FROM users WHERE id = ?", user_id)
    # Sau khi db trả về, chạy tiếp từ đây
    return data
```

## asyncio.gather — Chạy Đồng Thời

`asyncio.gather` giống như `ThreadPoolExecutor` nhưng nhẹ hơn. Nó nhận nhiều coroutine, chạy tất cả "cùng lúc" và chờ tất cả hoàn thành:

```python
async def main():
    async with aiohttp.ClientSession() as session:
        urls = [
            "https://api.github.com/repos/rust-lang/rust",
            "https://api.github.com/repos/python/cpython",
        ]
        tasks = [fetch(session, url) for url in urls]
        results = await asyncio.gather(*tasks)
```

## Khi Nào Nên Dùng, Khi Nào Tránh

**Nên dùng async khi:** gọi API, query database, đọc file, hay bất cứ tác vụ I/O nào mà anh em phải chờ.

**Đừng dùng async khi:** tính toán CPU-bound như xử lý ảnh, crypto, hay vòng lặp toán học nặng. GIL vẫn là GIL — async không thoát được đâu.

```python
import asyncio
import math

# Sai: CPU-bound mà xài async — vừa chậm vừa mệt
async def compute_prime(n: int) -> bool:
    for i in range(2, int(math.sqrt(n)) + 1):
        if n % i == 0:
            return False
    return True

# Đúng: dùng multiprocessing cho CPU-bound
from multiprocessing import Pool
with Pool() as pool:
    results = pool.map(compute_prime, [11, 13, 17, 19])
```

## Kết Luận

Async là skill sinh tồn trong Python hiện đại. Xài cho I/O-bound tasks (API, database, file), đừng đụng vào CPU-bound. Còn cái GIL thì... sống chung với nó thôi.

*"Async Python không làm code chạy nhanh hơn — nó làm code chờ đợi ít hơn."* ⏱️
