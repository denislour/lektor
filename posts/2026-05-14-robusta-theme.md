---
title: "51 màu, 0 design skill, 1 thằng dev mù tịt — và một ly cà phê phin"
date: "2026-05-14"
tags: ["json", "pi", "theme", "coffee"]
readTime: 8
excerpt: "Tôi không biết phối màu. Tôi không biết độ bão hoà là cái quái gì. Hồi nhỏ tôi vẽ con gà 3 chân. Nhưng tôi biết uống cà phê. Và tôi đã dùng ly cà phê phin để làm ra cái theme terminal cho pi coding agent — vì terminal mặc định nó... vô hồn quá."
---

2 giờ sáng. Màn hình tối om. Chữ trắng muốt. Tôi ngồi nhìn chằm chằm vào cái JSON dài 200 dòng — thiếu dấu phẩy ở dòng 142.

45 phút. Để nhận ra một cái dấu phẩy.

Bốn mươi lăm phút nhìn chằm chằm vào một cái file JSON, mỗi dòng đều trông giống nhau, mỗi dấu ngoặc nhọn như đang cười nhạo tôi, và cái bug cứ tồn tại một cách bí ẩn như một hung thủ đã xóa dấu vết nhưng quên mất rằng thám tử cũng có 45 phút rảnh rỗi để lần ra nó.

Tôi ngả người ra ghế. Mắt đỏ. Đầu óc như cục bông gòn thấm nước — nặng, ẩm, chẳng nghĩ được gì tử tế. Đờ đẫn. Như zombie. Nhưng không thèm ăn não.

Nhìn màn hình.

Rồi nhìn ly cà phê phin trên bàn.

Ly cafe đã nguội từ bao giờ. Nhưng màu thì vẫn còn.

Nâu đen dưới đáy. Lớp bọt sáng vàng trên mặt. Ánh lên dưới đèn bàn — cái thứ ánh sáng vàng vọt của một thằng dev thức đêm. Đẹp. Mà cũng buồn.

Một ý nghĩ ngu ngốc xuất hiện:

*"Sao terminal của mình không thể có màu giống ly cafe này nhỉ?"*

Câu hỏi đó. Một tuần sau. 51 tokens màu. 0 kiến thức design.

Và tôi kể anh em nghe câu chuyện.

## Mở file theme ra, tôi suýt khóc

Tôi nghĩ làm theme dễ lắm. Đổi màu nền. Đổi màu chữ. Thêm một màu accent. 5 phút.

Ngây thơ vkl.

Mở file JSON của pi coding agent ra — tôi thấy 51 tokens.

Năm mươi mốt.

Mỗi token là một quyết định màu sắc. Màu cho code block. Màu cho border. Màu cho cái bullet đầu dòng — cái dấu chấm nhỏ xíu mà tôi còn chẳng bao giờ để ý, nhưng bây giờ phải chọn màu cho nó.

Rồi tới mấy cái mà tôi còn không biết nó là cái quần gì:

```json
{
  "thinkingOff": "...",
  "thinkingMinimal": "...",
  "thinkingLow": "...",
  "thinkingMedium": "...",
  "thinkingHigh": "...",
  "thinkingXhigh": "..."
}
```

Sáu mức tư duy. Sáu màu.

Cho một cái CLI tool.

Tôi nhìn cái danh sách rồi nhìn lại màn hình tối om. Cảm giác như lần đầu tiên đi chợ và thấy có 51 loại rau — mày chỉ muốn mua rau muống, nhưng người ta hỏi "rau muống luộc hay xào, mua kèm tỏi không, loại già hay non, cắt khúc sẵn hay để nguyên?"

Tôi mù tịt. Mù hoàn toàn. Ngu về màu. Ngu về phối. Ngu hết. Và tôi nhận ra mình sắp phải đối mặt với 51 quyết định màu sắc mà không có một chút kiến thức nào về cái gọi là "thị giác" hay "thẩm mỹ" hay bất kỳ thứ gì liên quan đến việc nhìn bằng mắt và đánh giá bằng não.

Hồi mới vào nghề, tôi thiết kế cái tool internal cho công ty. Đồng nghiệp feedback: *"Nhìn như Windows 95 — nhưng xấu hơn nhiều."*

Tôi từng hỏi đồng nghiệp designer: "Màu xanh navy với royal blue khác nhau chỗ nào?"

Cả phòng cười.

Im lặng 3 giây.

Rồi một đứa bảo: *"Mày hỏi thật à?"*

Tôi hỏi thật. Và từ đó, mỗi lần có việc gì liên quan đến UI, cả team đều né — sợ lây cái bệnh mù màu.

Nhưng mà này — tôi có một thứ. Chỉ một thứ thôi.

Một ly cà phê trên bàn, và một câu hỏi ngu ngốc.

Đủ rồi.

## Eye dropper — tool của những thằng mù design

Tôi chụp hình ly cà phê. Mở Photoshop. Dùng eye dropper.

Nghe chuyên nghiệp chứ?

Không. Vì màu trên màn hình điện thoại khác màu dưới đèn bàn. Nhưng tôi kệ.

**Màu hạt rang:** `#2c1a0e`. Nâu gần như đen. Đặt tên `roast`.

**Màu nước cà phê:** `#3e2723`. Nâu ánh đỏ. `brew`.

**Crema — lớp bọt:** `#8d7a6b`. `crema`.

**Sữa đặc Ông Thọ:** `#efebe9`. `milk`.

**Kem sữa hoà vào cà phê:** `#fff8e1`. `cream`.

Năm màu. Tôi tự hào. Cứ nghĩ thế là xong.

Nhìn lại danh sách. 51 tokens. Còn 46 cái trống.

Thôi thì lấp đầy dần.

## Accent — 2 ngày, 7 phiên bản, 3 màu

Màu accent là linh hồn của theme. Nó làm nên cá tính.

Tôi không biết chọn màu gì. Google 2h sáng: *"what color goes with brown terminal"*

Kết quả: amber, gold, warm orange.

**Amber `#ff8f00`.**

Vàng cam. Chói quá? Hay đẹp? Tôi không biết — sau 12 tiếng code mắt tôi chỉ phân biệt được hai thứ: tối và sáng. Lúc đó tôi đang ở trạng thái thứ ba — mờ dần như bình ắc quy hết điện.

Gửi lên nhóm bạn.

> **Thắng:** "Nhìn như app đặt đồ ăn TQ."

Chê rồi.

**Golden `#ffab00`:** Vàng hơn. Ấm. Nhưng nhìn giống theme của mấy ông bán vàng bạc đá quý.

Chê tiếp.

**Caramel `#f57f17`:** Đậm, trầm, có chiều sâu. Nhưng hơi tối — chữ trên nền nâu không nổi.

2 ngày. 7 phiên bản. 0 kiến thức design.

Cuối cùng tôi chọn cả ba.

- `amber` — border, logo, selected item. Chính.
- `golden` — keyword, heading. Nổi.
- `caramel` — function name, link. Ấm.

Ba anh em nhà họ cam. Không thằng nào hoàn hảo. Nhưng cả ba hợp lại — ra một quán cafe.

> *Design là khi mày không biết làm sao cho đẹp, nhưng biết làm sao cho đỡ xấu.*

## Matcha — cú plot twist không ai ngờ

Khi tô màu syntax highlight, tôi gặp vấn đề.

Nếu tất cả code đều nâu và cam — mắt không phân biệt được string với function với keyword.

Xanh dương phối nâu? Nhìn như quần áo thập niên 70 — cái thời mà ai cũng mặc suit màu đất, cà vạt to quá khổ, và không ai dám bảo ai.

Tím? Như cái club ở Bùi Viện. Nhìn phát biết ngay là ngồi code không nổi 5 phút.

Đỏ? Như cảnh báo cháy rừng. Cảm giác sắp có thiên tai, không tập trung nổi.

Rồi tôi nhìn cái bánh matcha ăn dở trên bàn.

...

**Matcha `#aed581`:** Xanh nhạt, hơi ngả vàng. Tương phản vừa đủ với nâu. Không chói. Không lòe loẹt.

Tôi thử. String màu matcha — function caramel — keyword golden.

Đẹp. Không phải WOW. Mà là kiểu "ừ, có thể ngồi 8 tiếng không muốn đập màn hình."

Gửi lên nhóm.

Thắng bảo: "Ngộ. Nhưng được."

Thằng bạn tôi khen UI 3 lần trong đời. "Ngộ" từ nó là 8/10.

## 51 cái ô — 51 quyết định nhỏ

Mỗi cái ô màu là một quyết định. Có cái tôi chọn trong 3 giây. Có cái tôi ngồi 15 phút.

`toolPendingBg` — cái màu tô cái nền khi tool đang chạy, một cái chi tiết vô cùng nhỏ nhưng nếu sai thì mỗi khi chạy lệnh anh em sẽ nhìn vào đó và cảm thấy không hiểu sao cứ bồn chồn như đang đợi nước sôi mà ấm cứ ì ạch. Tôi để nâu sẫm. Như hạt rang. 5 giây.

`toolSuccessBg` — xanh rêu nhạt. 3 giây.

`toolErrorBg` — đỏ sẫm. Đừng nghĩ nhiều. Ngắn thôi. Ầm. Xong. 2 giây.

`mdCodeBlockBorder` — màu quế `#8d6e63`. Ấm hơn crema một tí. 10 giây.

`mdCode` — inline code. Matcha. Vì nó cần nổi bật.

15 phút.

Ừ. 15 phút. Cho một cái màu. Tôi ngồi nhìn màn hình, chuyển qua chuyển lại matcha với caramel, hỏi mình: *"Mày đang làm cái quái gì vậy?"*

`mdLink` — caramel. Link cần có cảm giác "ngọt, mời gọi". 5 phút.

`mdQuote` — màu latte. Nhạt. Vì quote là thứ phụ — không cần khoe. 3 giây.

`mdListBullet` — amber. Dấu chấm đầu dòng — nhỏ mà cũng phải có màu. 3 giây.

Đến export section — màu cho phần export HTML.

Nửa tiếng.

Chỉ để chọn màu cho ba cái card.

`pageBg` kem, `cardBg` trắng ngà, `infoBg` latte. Ba màu hợp nhau như một bộ ấm trà — cái ấm mà mẹ tôi mua ở chợ Đồng Xuân về mà tôi thấy cũng đẹp đấy chứ, dù không ai bảo thế.

Tokens cuối cùng — cái thứ 51, `exportPageBg`.

3h sáng. Mắt không còn nhìn rõ nữa.

Để đại `#fff8e1`. Kem cho lành.

Xong.

Tôi thở phào như vừa chạy xong marathon — mặc dù thực ra cả buổi tối chỉ ngồi một chỗ và bấm chuột.

Mở pi. Apply.

Nó hoạt động.

Tôi ngồi nhìn màn hình 5 phút. Không code. Chỉ nhìn.

Rồi mỉm cười.

## Cộng đồng — thứ tôi không ngờ tới nhất

Tôi push lên GitHub. Đặt tên **phin** — từ tiếng Pháp *filtre* (cái lọc). Cà phê phin ấy mà.

Feedback đầu tiên:

> **@khoi_t:** "Code 30 phút thấy mắt đỡ mỏi hẳn."

Một người dùng thật. Không phải bạn tôi. Không phải tôi fake 2 acc. Người thật ngồi code 30 phút với màu của tôi — không chửi thề.

Cảm giác đó khó tả lắm. Như nấu một món ăn — và có người ăn hết, không bị đau bụng. Đỉnh cao cuộc đời của thằng dev mù design.

> **@linh:** "Tại sao không để sáng hơn? Nâu đen khó đọc."

Phản hồi hay. Tôi giải thích: nâu đen là background cho tool boxes — không phải để đọc. Có phân cấp hết.

Dù tôi biết cái quái gì về phân cấp thị giác đâu.

Biết gì đâu.

Zero.

Tôi chỉ biết uống cafe.

Rồi **Denis** — cũng dùng pi — thấy repo:

> **Denis:** "Hay quá. Sao không làm thêm theme arabica — nhạt hơn, hồng tím oải hương?"

Tôi nói: *"Làm đi, tôi pull request cho."*

Và Denis làm thật. Anh ấy tạo **arabica** — pastel, hồng tím, nhẹ nhàng. Tôi merge.

Bây giờ `phin` package có 2 themes. 3 người đóng góp. Vài chục người dùng.

Cộng đồng mở — đơn giản vkl.

Tôi làm một thứ nho nhỏ cho riêng mình. Người khác thấy hay, họ thêm cái của họ. Một người khác nữa góp vô.

Rồi bỗng dưng cái dự án một thằng dev thức đêm vì không chịu nổi màn hình tối đã thành thứ gì đó có người dùng, có contributor, có đời sống riêng.

## Dev làm design được không?

Được. Nhưng đừng ảo tưởng.

Tôi không biết color theory.

Không biết độ bão hoà.

Tỷ lệ tương phản?

Cái quái gì thế?

Hue? Saturation? Lightness?

Bó tay.

Completely.

Hồi nhỏ tôi vẽ con gà — nó có 3 chân. Cô giáo mỹ thuật nhìn rồi hỏi: "Con gà này bị đột biến gen hả em?" — tôi cười trừ, không dám bảo là mình vẽ thiếu chân sau, tô thêm chân thứ ba vào vì tưởng gà có... 3 chân.

26 tuổi. Vẫn dốt design.

Vẫn dốt.

Ai hỏi cũng dốt.

Nhưng tôi biết mình thích uống cà phê. Và tôi muốn cái terminal giống ly cà phê.

Đó là điểm xuất phát. Không cần chuyên môn. Chỉ cần một cảm xúc.

Tôi học về tương phản khi bị chê *"khó đọc"*. Học về màu bổ sung khi string matcha không hợp background nâu. Học về phân cấp thị giác khi đặt heading và body — và nhận ra heading phải nổi trội hơn.

Dù điều đó hiển nhiên như 2+2. Nhưng không ai dạy. Phải tự mò.

> *Cách tốt nhất để học một thứ là làm một thứ ngu ngốc với nó.*

Một tuần không code. Chỉ mix màu. Cuối tuần, tôi hiểu về màu sắc hơn cả 5 năm mỹ thuật cấp 2.

## Lời kết

Sáng thứ Bảy. Ly cafe thứ ba đã nguội.

Nhưng theme vẫn còn ấm.

Màn hình bây giờ là cả một quán cafe thu nhỏ — background nâu rang đậm đà như ly đen đá đầu ngày, chat message màu brew ấm áp như từng giọt cà phê đang nhỏ xuống phin, heading vàng ong nổi bật như ánh nắng sớm mai xuyên qua tách cafe, code block viền quế nhẹ nhàng, và những dòng string xanh matcha tươi mát lạc lõng giữa cả một biển nâu.

Tôi không phải designer. Tôi chưa bao giờ là designer. Và tôi nghi ngờ sẽ không bao giờ là.

Nhưng tôi cũng không cần phải là designer để có opinion về cái thứ mình nhìn mỗi ngày 8-12 tiếng.

Cái terminal ấy. Cái editor ấy. Cái theme mặc định với chữ trắng trên nền xanh dương đậm — thứ mà ai cũng dùng, ai cũng quen, không ai thắc mắc.

Nhưng một ngày đẹp trời — hay đúng hơn là 2h sáng, mắt đỏ au, deadline dí — mình nhìn nó và tự hỏi: *"Sao không thể ấm hơn nhỉ?"*

Anh em biết không — có những thứ mình tưởng là cố định, là mặc định, là "nó phải thế". Nhưng thực ra, không ai bắt mình phải sống với nó cả.

Cái màu chữ trắng trên nền đen ấy — không phải là luật trời sinh.

Mình có thể đổi nó.

Không phải vì mình giỏi. Mà vì mình muốn.

Thế thôi.

Và nếu một thằng từng vẽ con gà 3 chân, không biết màu gì hợp với màu gì, từng hỏi navy vs royal blue khác nhau chỗ nào và bị cười — mà vẫn làm được một cái theme cho pi, thì anh em cũng làm được.

Không phải là design. Mà là dám thử.

Là nhìn ly cafe lúc 2h sáng và nghĩ: *"Sao không đưa nó vào terminal nhỉ?"*

Rồi mở editor lên. Bắt đầu thay đổi — dù không biết mình đang làm gì.

Bởi vì đôi khi cái "muốn" nó mạnh hơn cái "biết" rất nhiều, nó mạnh đến nỗi một thằng dev mù tịt về thẩm mỹ, không biết color theory là gì, không phân biệt được độ bão hòa 70% với 80% — cái mà nếu là thời đi học mỹ thuật sẽ được bạn cùng bàn giải thích 3 lần mà vẫn không hiểu — vẫn có thể ngồi 7 ngày trời mix 51 cái ô màu cho một cái terminal chỉ vì không chịu nổi cái màu mặc định, anh em ạ.

Cảm giác khi nhìn màn hình và thấy nó giống một thứ gì đó quen thuộc — ấm áp, như đang ngồi ở quán cafe góc phố Lê Văn Sỹ chứ không phải trong căn phòng tối om lúc 2h sáng với deadline đang rình rập phía sau — cái cảm giác đó, đáng để thức thêm vài đêm.

> *"Đừng code trên nền tối — hãy code trên nền cà phê."* ☕

---
*Nếu anh em dùng pi, thử cài `phin` package — npm:@mariozechner/phin hoặc git cá nhân. Cả Robusta (đậm, giống tôi — hơi đắng nhưng có hậu) và Arabica (nhẹ, thanh xuân — cho ai muốn dễ uống). Có cà phê, có chuyện, có 51 cái màu từ một thằng dev không biết design. Pull request luôn open — Denis đã làm, sao anh em không thử?*
