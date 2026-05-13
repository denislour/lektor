---
title: "Theme Robusta — Một ly cà phê phin giữa terminal"
date: "2026-05-14"
tags: ["json", "pi", "theme", "coffee"]
readTime: 6
excerpt: "Tôi đã dành cả đêm để mix màu cà phê vào pi coding agent. Kết quả là robusta — một theme màu nâu đậm, hổ phách, caramel và một chút matcha xanh cho nó tươi."
thumbnailCode: |-
  {
    "roast": "#2c1a0e",
    "brew": "#3e2723",
    "cream": "#fff8e1",
    "amber": "#ff8f00",
    "golden": "#ffab00",
    "matcha": "#aed581"
  }
---

Pi là CLI tool tôi dùng hằng ngày. Nó có theme mặc định, Catppuccin, Dracula — nhưng chưa có cái nào mang màu cà phê Việt Nam.

Thế là tôi tự pha chế.

## Phin — cái tên sinh ra từ filtre

Tôi đặt tên repo là **phin** — từ tiếng Pháp *filtre* (cái lọc). Ai search "từ mượn tiếng Pháp trong tiếng Việt" sẽ thấy:

> *"Cà phê phải được lọc từ cái phin (filtre à café) mới đúng điệu."*

Tự nhiên như bánh mì, như nhà ga, như xi măng.

## Màu sắc — lấy cảm hứng từ ly cà phê

Đang ngồi code lúc 2h sáng, ly cà phê phin trên bàn, tôi nhìn lên màn hình rồi nhìn xuống ly — sao không lấy màu từ nó nhỉ?

```
🖤  roast   #2c1a0e  — Hạt rang đậm (nền tools)
🟤  brew    #3e2723  — Giọt cà phê phin (nền chat)
🤎  crema   #8d7a6b  — Lớp bọt crema (border mờ)
🥛  milk    #efebe9  — Sữa đặc (text chính)
🥛  cream   #fff8e1  — Kem sữa (text nổi bật)
```

Từ hạt rang → giọt cà phê phin → sữa → kem. Đúng quy trình pha cà phê sữa đá.

## Accent — ba anh em nhà họ cam

```css
amber   #ff8f00  → Hổ phách — accent chính
golden  #ffab00  → Vàng ong — keyword, heading
caramel #f57f17  → Caramel cháy — function, link
```

Màu của lửa rang, của caramel, của nắng — nhìn là thấy ấm.

## Matcha — điểm nhấn 🍵

Tôi thêm màu xanh matcha `#aed581` cho syntax highlight. Nếu tất cả đều là nâu, mắt anh em sẽ không biết đâu là string, đâu là keyword. Matcha làm code có nhịp — như uống cafe ăn kèm bánh matcha, hai vị đối nhau nhưng lại hợp.

## Lời kết

Tôi không phải designer. Tôi chỉ là thằng dev thích uống cà phê và ghét màn hình tối om không có hồn. Robusta sinh ra từ việc nhìn ly cà phê trên bàn và tự hỏi: *"Sao không đưa nó vào đây nhỉ?"*

> *"Đừng code trên nền tối — hãy code trên nền cà phê."* ☕🟤
