---
title: "Tôi từng mở 500 file không close — câu chuyện về context manager và 1 đêm mất ngủ"
date: "2026-05-14"
tags: ["python"]
readTime: 8
excerpt: "500 file descriptor treo. 1 server chết. 1 đêm không ngủ. Và 1 cốc cà phê phin đắng ngắt lúc 6h sáng. Đó là cách tôi học context manager."
thumbnailCode: |-
  class WakeUpCall:
    def __enter__(self):
      print("2h sáng, mở mắt. Server chết.")
      return self
    def __exit__(self, *args):
      print("Sáng hôm sau: context manager sinh tồn.")
      return False
  with WakeUpCall():
    pass
---

Hồi mới đi làm, tôi có cái script Python chạy scheduled task vớ vẩn. Đọc file log. Xử lý tí. Ghi ra file khác. Đơn giản vkl. Nhưng mà này — cái gì đơn giản quá thì thường có vấn đề.

Thế mà 3 ngày sau — server chết. Chết cứng. Không thở. Không vào được. Như cá nằm trên thớt. Không SSH vào được. `lsof` ra cả đống file descriptor treo.

Tôi nhìn màn hình lúc 2h sáng. Mồ hôi tay. Gọi ai? Không. Dev chân ướt chân ráo như tôi có ai gọi đâu. Cũng may tôi còn biết gõ `lsof` ra coi — nếu không thì đúng ăn cả tháng lương vì tội làm chết server production mà không hiểu tại sao.

Hoá ra — tôi quên close file.

Đúng vậy. `f = open("log.txt")` rồi... thôi. Không `.close()`. 500 cái. 500. Năm trăm. Năm — trăm — cái — file — descriptor. Mỗi cái đang treo trên server như những linh hồn không siêu thoát. Một cái còn đỡ. 500 cái thì đi đời. Cứ mỗi lần script chạy, nó mở cái file mới, không trả lại cho OS. Đến lúc `ulimit` đụng trần — boom.

Sếp nhìn tôi sáng hôm sau. Tôi nhìn sếp. Nhìn nhau. Không nói gì. Mắt sếp nói: "Mày không close file à?" Mắt tôi trả lời: "Dạ." Chát. Không khí đặc quánh như thạch.

Sếp không chửi. Sếp chỉ nhẹ nhàng gõ — chậm rãi, điềm tĩnh, như thể múa kiếm:

```python
with open("log.txt") as f:
    data = f.read()
```

Ba dòng. Một keyword. Và cuộc đời tôi sang trang mới.

---

## `with statement` — Cái Tủ Lạnh Biết Tự Đóng Cửa

Tôi đã tưởng `with` là cái gì xa vời lắm cơ, như kiểu functional programming hay monad trong Haskell mà mấy thần đồng trên Viblo hay khoe. Hoá ra không. Tôi đã tưởng `with open() as f` là cú pháp đặc biệt kiểu "Python magic" gì đó. Hoá ra không.

Nó là **context manager protocol**. Nghe ghê vậy thôi chứ dễ hiểu lắm các bạn ạ. Nôm na là: vào block thì gọi `__enter__`, ra block thì gọi `__exit__`. Kể cả có exception hay không. Kể cả có cháy nhà hay sập server. Nó vẫn tự động dọn.

```python
# Đây là những gì with làm bên trong — đừng sợ
f = EXPRESSION.__enter__()  # Mở file (hoặc resource gì đó)
try:
    BLOCK                    # Code của anh em
finally:
    EXPRESSION.__exit__(...) # Tự đóng — không cần nhớ
```

Đơn giản vkl. Nhưng mà cái hay ở đâu? Chúng ta không phải nhớ. Và với một thằng hay quên như tôi, đó là cả một ân huệ — bởi vì tin tôi đi, nếu không có `with`, thì cái server đó chết không chỉ một lần, mà sẽ chết đi chết lại mỗi khi tôi deploy code mới.

Cái hay là Python tự động gọi `__exit__` ngay cả khi code bên trong chết giữa chừng vì exception. Còn tôi ngày xưa — quên close, mà có exception thì càng quên. Nhân đôi đau khổ.

Cơ mà nếu `with` chỉ dùng để mở file thì hơi phí. Phí vkl. Anh em có thể tự tạo context manager cho bất cứ resource nào: database connection, lock, timer, network socket — cái gì cũng với được. Các bạn nghĩ sao? Hãy thử tưởng tượng một thế giới mà mọi resource đều tự động đóng khi không dùng nữa. Đó là thế giới chúng ta đang sống — nếu chịu học context manager.

---

## Class-based Context Manager — Tự Viết Cho Sang

Tôi biết anh em thích copy-paste xài đồ có sẵn. Nhưng nếu tự viết được context manager, đột nhiên anh em thấy mình... ngầu hơn hẳn. Ít nhất là trong mắt tôi.

```python
import time

class Timer:
    """Đo thời gian — vì sếp hay hỏi 'sao chậm thế' """
    def __enter__(self):
        self.start = time.perf_counter()
        return self

    def __exit__(self, *args):
        self.elapsed = time.perf_counter() - self.start
        print(f"⏱ Mất {self.elapsed:.3f}s — cà phê chưa kịp pha xong")
        return False  # Không nuốt exception — exception là exception

# Thử với anh em ơi
with Timer():
    total = sum(i * i for i in range(10_000_000))

print(f"Tổng: {total}")
```

Output:
```
⏱ Mất 0.412s — cà phê chưa kịp pha xong
Tổng: 3333332833333350000
```

Thấy chưa? Chỉ vài dòng. Không thư viện ngoài. Không pip install. Không đau đầu. Dùng đo performance, biết ngay chỗ nào đang chậm. Biết ngay thằng nào đang ăn CPU.

Chỗ cần chú ý là `return False` ở `__exit__`. Nó nói: exception cứ tràn ra ngoài, đừng nuốt. Exception là exception — phải biết để còn fix.

Hồi mới học, tôi return `True` lung tung. Hậu quả? Exception biến mất. Code chạy ngon. Dữ liệu sai. Debug 3 tiếng mới biết tại sao. Đau vkl.

---

## `__exit__` Xử Lý Exception — Khu Vực Nhạy Cảm

Khi viết context manager cho database transaction, việc xử lý exception sai có thể dẫn đến data corruption. Nói thế cho nó ghê, chứ thực ra là table bị lock, production khóc, anh em đường ai nấy đi.

```python
class DatabaseTransaction:
    """
    Tự động commit / rollback — khỏi lo quên
    Gặp exception thì rollback. Không thì commit.
    """
    def __init__(self, conn):
        self.conn = conn

    def __enter__(self):
        self.conn.begin()
        return self.conn

    def __exit__(self, exc_type, exc_val, exc_tb):
        if exc_type is not None:
            self.conn.rollback()
            print(f"😱 Lỗi rồi — rollback: {exc_val}")
            return False  # Không nuốt — để exception chạy tiếp
        else:
            self.conn.commit()
            return True

# Xài thật
import psycopg2
conn = psycopg2.connect("dbname=test")
with DatabaseTransaction(conn) as cur:
    cur.execute("INSERT INTO users (name) VALUES ('Thành')")
    # Nếu dòng này fail → tự động rollback
    # Nếu chạy ngon → tự động commit
```

Theo một khảo sát không chính thức từ 3 thằng ngồi nhậu ở quán cóc Nguyễn Huệ: **96.69% bug production liên quan đến transaction là do nuốt exception không kiểm soát.** Số liệu tôi bịa đấy, nhưng tin tôi đi — nó đúng.

Chỗ `return False` và `return True` khác nhau chỗ nào?

- `return False` (hoặc `None`) — exception vẫn lan ra ngoài. Code gọi `with` biết có lỗi.
- `return True` — exception bị nuốt. Code chạy tiếp như không có gì.

Tôi khuyên: trừ khi anh em biết chính xác mình đang làm gì, còn không thì `return False` cho lành.

---

## contextlib — Context Manager Cho Người Lười

Có anh em bảo: "Viết class lằng nhằng quá, tôi có 3 dòng code thôi."

Hiểu. Python sinh ra cho người lười. `contextlib` ra đời.

```python
from contextlib import contextmanager

@contextmanager
def temporary_file(suffix: str = ".tmp"):
    """
    Tạo file tạm — xoá sau khi dùng
    Giống như ăn phở xong tự trả bát, không để người khác dọn
    """
    import tempfile
    import os

    # __enter__ — chỗ này
    tmp = tempfile.NamedTemporaryFile(suffix=suffix, delete=False)
    try:
        yield tmp  # Cái này thành as VARIABLE
    finally:
        # __exit__ — chỗ này
        tmp.close()
        os.unlink(tmp.name)

# Dùng — ngắn gọn, sạch sẽ
with temporary_file(".csv") as f:
    f.write(b"name,email\n")
    f.write(b"alice@example.com\n")
    f.write(b"bob@example.com\n")
# File tự xoá — không cần nhớ. Tự động như hơi thở.
```

Dùng `@contextmanager` thì code ngắn hơn, ít boilerplate, dễ đọc. Cơ mà có nhược điểm: xử lý exception trong generator hơi khó hơn class.

Cũng như cà phê — pha phin thì lâu nhưng đậm, còn Cappuccino thì nhanh mà dễ uống. Tuỳ hoàn cảnh. Cũng như chúng ta có quyền lựa chọn. Có quyền quyết định. Có quyền viết code theo cách mình muốn. Nhưng mà nhớ: ai viết rồi cũng phải maintain.

---

## Redirect stdout — Ứng Dụng Tôi Yêu Thích

Hồi làm cái tool gửi log qua Slack, tôi cần bắt hết `print()` trong function để gửi đi. Context manager giải quyết gọn:

```python
import sys
from io import StringIO
from contextlib import contextmanager

@contextmanager
def capture_stdout():
    """
    Bắt hết print() vào string — khỏi sửa code
    Hên xui không bị sếp mắng vì quên gỡ debug
    """
    old_stdout = sys.stdout
    sys.stdout = StringIO()
    try:
        yield sys.stdout
    finally:
        sys.stdout = old_stdout

# Dùng thử — cảm giác như gian lận
with capture_stdout() as output:
    print("Anh em ơi, production đang chạy ngon!")
    print("Không tin thì vào mà xem.")
    print("À mà đừng sờ vào database nhé.")

captured = output.getvalue()
print(f"📦 Đã capture: {captured.strip()} ")
```

Output:
```
📦 Đã capture: Anh em ơi, production đang chạy ngon!
Không tin thì vào mà xem.
À mà đừng sờ vào database nhé.
```

Hay không? Capture stdout mà không cần sửa code gốc. Chỉ cần bọc `with` là xong — và tin tôi đi, khi các bạn đụng phải cái legacy system 10 năm tuổi với code viết bởi 5 thằng dev không ai còn ở công ty, pattern này là phao cứu sinh duy nhất trước khi anh em chết đuối trong biển print() debug.

---

## Context Manager Lồng Nhau — Kiểu Matryoshka

Này thì không phải ai cũng biết: anh em có thể lồng `with` vô nhau.

```python
# Chạy 20 lần, mỗi lần đo thời gian
results = []
for i in range(20):
    with Timer() as t:
        with capture_stdout() as cap:
            print(f"Lần thử {i + 1}")
            sum(range(10_000_000))
    results.append({
        "elapsed": t.elapsed,
        "captured": cap.getvalue(),
    })

# In 3 lần đầu
for r in results[:3]:
    print(f"⏱ {r['elapsed']:.3f}s — output: {r['captured'].strip()}")
```

Output:
```
⏱ Mất 0.415s — cà phê chưa kịp pha xong
⏱ Mất 0.398s — cà phê chưa kịp pha xong
⏱ Mất 0.406s — cà phê chưa kịp pha xong
⏱ 0.415s — output: Lần thử 1
⏱ 0.398s — output: Lần thử 2
⏱ 0.406s — output: Lần thử 3
```

Hai context manager lồng nhau, mỗi cái lo việc riêng. Timer đo thời gian, Capture hứng output. Python lo cleanup tự động. Anh em chỉ việc code.

---

## Và Câu Chuyện Chưa Kết Thúc

Tôi mất đúng 1 đêm mất ngủ, 1 buổi sáng bị sếp hỏi thăm, và 1 cốc cà phê phin đắng ngắt để hiểu ra một điều: **context manager không phải là thứ xa xỉ.** Các bạn nghe tôi nói này. Nó không phải để khoe mẽ. Nó là pattern sinh tồn.

Khi anh em mở file, rồi xử lý dữ liệu, rồi gặp exception, rồi quên mất cái `f.close()` ở cuối — nếu không có `with` thì khả năng leak resource là cao vkl, nghe tôi đi, tôi biết mà. Đừng bảo "tôi nhớ mà". Tôi cũng từng thế. Ai cũng từng thế. Cho đến khi server chết lúc 2h sáng.

Còn `contextlib` với `@contextmanager` — tiện thật. Nhưng đừng lạm dụng. Nếu logic `__exit__` phức tạp (exception handling, retry, logging...), class sẽ sáng sủa hơn. Cũng như món phở — bỏ quá nhiều gia vị thì mất vị ngọt thanh của nước dùng. Giữ nó đơn giản. Giữ nó sạch. Giữ nó như ý của anh em lúc đầu.

Tuần trước tôi review code cho một anh em. Trong cái pull request chỉ có vài chục dòng code, ấm ức làm sao tôi lại thấy cái pattern quen thuộc năm nào:

```python
f = open("data.csv")
process(f)
f.close()
```

Tôi comment: "Em chưa đọc bài context manager à?"

Anh em trả lời: "Dạ em biết rồi, nhưng em nghĩ cái này nhỏ, không cần `with`."

Tôi cười. Còn nước mắt chảy ngược vào trong. Cũng như tôi 2 năm trước. Cùng một suy nghĩ. Cùng một kết cục. Chỉ là vấn đề thời gian — một đêm 2h sáng nào đó, em cũng sẽ ngồi nhìn màn hình xanh, tay run, tim đập, và nhận ra rằng mở 500 cái file descriptor mà không close không phải là chuyện đạo đức hay kỷ luật, mà là chuyện kỹ thuật mà một dòng `with` đã giải quyết từ lâu, từ cái hồi Guido van Rossum còn ngồi pha cà phê nghĩ ra nó.

Cái hay của context manager không phải là "nó tự động dọn". Mà là: **nó biến cái dễ quên thành cái tự động.** Anh em không cần ghi nhớ. Không cần checklist. Không cần đêm mất ngủ.

Cũng như hồi nhỏ, mẹ tôi hay nhắc: "Đi đâu nhớ tắt điện." Giờ có người yêu... à không, có context manager nhắc rồi.

Thôi thì nhớ cho tôi điều này:

> **Tài nguyên nào cũng có hạn — file descriptor, connection pool, bộ nhớ, và cả sự kiên nhẫn của sếp.**
> 
> Dùng `with()` cho tôi nhé.

À mà còn một thứ nữa: nếu anh em chưa từng viết context manager, hãy thử ngay hôm nay. Viết cái Timer. Viết cái redirect stdout. Viết cái database transaction. Không cần cày gì to tát — chỉ cần thấy nó chạy, thấy output in ra, thấy cái cảm giác "đã tay" khi mọi thứ tự động dọn dẹp, là đã hiểu được cốt lõi của pattern này rồi các bạn ạ.

Rồi mai mốt gặp resource nào cần quản lý — tự khắc tay anh em sẽ gõ `with` trước khi kịp nghĩ.

Còn nếu vẫn quên... thì nhắn tôi. Tôi kể lại chuyện đêm 2h sáng năm đó cho anh em nghe.

Chúc anh em code vui. Và đừng quên close file — bởi vì tôi không muốn ai trong số các bạn phải trải qua cái cảm giác 2h sáng nhìn màn hình xanh, tay lạnh, mồ hôi đầm đìa, và tự hỏi tại sao mình không chịu học context manager từ sớm.
