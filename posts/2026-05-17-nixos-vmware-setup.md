---
title: "3 lần cài NixOS trên VMware, 2 lần OOM, 1 lần quên UEFI — và cái kết"
date: "2026-05-17"
tags: ["nixos", "vmware", "linux"]
readTime: 7
excerpt: "Tôi tưởng cài NixOS trên VMware là chuyện 30 phút. 5 tiếng sau, tôi vẫn ngồi nhìn console với cái OOM và tự hỏi tại sao mình không học Kubernetes cho rồi."
thumbnailCode: |-
  { config, pkgs, ... }: {
    virtualisation.vmware.guest.enable = true;
    # 1 dòng. Mà quên mất cũng 3 lần.
  }
---

Tôi tưởng cài NixOS trên VMware là chuyện đơn giản.

Tôi đã sai.

5 tiếng. 3 lần cài lại. 2 lần out of memory. 1 lần quên chọn UEFI. Và vô số lần tự hỏi: "Sao tao lại chọn NixOS làm OS đầu tiên trên VM?"

Câu trả lời: vì tôi thích đau khổ. Và vì NixOS ngon thật — sau khi vượt qua được cái đau khổ đó.

---

## Lần 1: BIOS, UEFI, và cái bootloader không boot

Lần đầu tôi làm theo instinct: tạo VM xong để mặc định. VMware Workstation Pro 17 mặc định là **BIOS**. Tôi cài NixOS với `systemd-boot` — cái bootloader chỉ chạy trên UEFI.

Kết quả? Màn hình đen. Không boot. Tôi nhìn màn hình. Màn hình nhìn tôi.

Tôi không biết tại sao. Tôi ngồi 30 phút check lại config, rebuild, thử lại. Không ăn thua.

Hoá ra là: **VMware mặc định dùng BIOS, nhưng config của tôi dùng systemd-boot — thằng đó chỉ biết UEFI.** Hai đứa không nói chuyện được với nhau. Cũng như người yêu cũ và tôi — incompatible từ đầu.

**Bài học:** Vào **VM Settings → Options → Advanced → Firmware → chọn UEFI**. Trước khi cài. Không phải sau khi cài.

Lúc đấy tôi mới nhận ra mình đã mất 1 tiếng chỉ vì cái checkbox ở tab Options thay vì Hardware. Một tiếng. Cho một checkbox.

---

## Lần 2: keyboard.vusb.enable và cái TTY lag như máy tính 1990

Sau khi chỉnh UEFI, mọi thứ chạy. Nhưng mà chạy... chậm.

Gõ phím trong TTY — mỗi chữ xuất hiện sau 1-2 giây. Tôi gõ `ls`, đợi 3 giây mới thấy chữ `l` hiện ra. Cảm giác như hồi học tin học lớp 3 với cái máy Pentium 3 chạy Windows 98.

"Chắc tại máy yếu" — tôi nói với mình. Không phải.

Lỗi này kinh điển đến nỗi có cả 1 thread trên VMware Community dài 3 năm, hơn 100 comment. Fix chính thức từ VMware support:

```
keyboard.vusb.enable = "TRUE"
```

Thêm 1 dòng vào file `.vmx`. Tắt VM. Mở file Notepad. Thêm dòng. Lưu. Mở VM lại.

Hết lag. Như chưa từng có gì xảy ra.

Một dòng. Chỉ một dòng. Mà tôi mất 30 phút search Google, đọc 3 trang forum, thử 5 cách khác nhau trước khi ra được cái dòng đó.

Cũng như lỗi production — cái khó nhất không phải fix, mà là tìm ra chỗ sai.

---

## Lần 3: OOM — hay "tại sao 8GB RAM vẫn không đủ"

Lần này là lúc chạy `nixos-install`. Build được tầm 30%, nó dừng:

```
out of memory: Killed process nix
```

Tôi nhìn `free -h`: còn 3.8GB free, 3.2GB buffer. Đủ mà nhỉ?

Không. Không đủ.

NixOS build process rất tốn RAM — nó compile package từ source. `nixos-install` mặc định chạy số jobs bằng số CPU cores. 4 cores = 4 builds cùng lúc. Mỗi build vài trăm MB RAM. Nhân lên. Boom.

Fix:

```bash
sudo nixos-install --flake /mnt/etc/nixos#my-vm --max-jobs 1 --cores 1
```

`--max-jobs 1`: chỉ build 1 package 1 lúc. `--cores 1`: mỗi package chỉ dùng 1 core. Chậm hơn, nhưng không OOM.

Hoặc thêm swap partition ngay trong disko config — 4GB là đủ cho VM 8GB RAM:

```nix
swap = {
  size = "4G";
  type = "8200";
  content = {
    type = "swap";
  };
};
```

Disko tự tạo và bật swap. Không cần `swapon` tay.

Tôi chọn cách thêm swap vào disko — vì nó declarative. Cái gì khai báo được thì không lo quên. Cũng như context manager trong Python. Cũng như `with open()` thay vì `f.close()` tay.

---

## Config cuối cùng — cái nhìn lại

Sau 5 tiếng, repo của tôi có mấy thứ này:

**disk-config.nix** (partition layout):
```
ESP 512M (vfat) → /boot
Swap 4G
Root 100% (ext4) → /
```

**configuration.nix** (hệ thống):
```nix
virtualisation.vmware.guest.enable = true;  # open-vm-tools
boot.loader.systemd-boot.enable = true;     # UEFI only
boot.loader.efi.canTouchEfiVariables = false;  # VMware UEFI quirk
```

3 dòng. Nhưng để ra được 3 dòng đó, tôi đã phải đọc ArchWiki, NixOS Wiki, VMware Community, 3 bài blog, và 1 issue GitHub đã đóng từ 2 năm trước.

**desktop.nix** (Hyprland + SDDM + PipeWire):
```nix
programs.hyprland.enable = true;
services.displayManager.sddm.enable = true;
services.pipewire.enable = true;
```

**common.nix**:
```nix
nix.settings.experimental-features = [ "nix-command" "flakes" ];
```

---

## Cái giá của declarative

NixOS hay ở chỗ: 1 file mô tả cả hệ thống. Không cần snapshot VM. Không cần backup thủ công. `git clone` + `nixos-install` là có máy mới.

Nhưng cái giá là: lần đầu setup rất đau.

Phải hiểu disko để partition. systemd-boot vs GRUB. UEFI vs BIOS. `.vmx` format. `open-vm-tools` vs VMware Tools. What the hell is `canTouchEfiVariables`.

Cái feeling lúc 2h sáng config chạy lần đầu — không gì sánh bằng. Cũng như lần đầu deploy thành công một service tự tay viết. Production xanh. Slack im. Cà phê phin nóng.

Tôi không nói NixOS dễ. Tôi nói: nếu anh em kiên nhẫn, declarative config là thứ đáng để đầu tư.

Nhưng mà đừng như tôi — 3 lần cài mới xong. Nhìn lại, có cái checklist này thì tự nhiên đỡ hơn hẳn:

1. **UEFI** — VM Settings → Options → Advanced → chọn UEFI trước khi boot ISO
2. **3D acceleration** — Bật trong Hardware → Display (nếu dùng Hyprland)
3. **`.vmx`** — Thêm `keyboard.vusb.enable = "TRUE"` — fix lag TTY
4. **Swap** — Thêm vào disko config, 4GB là đủ
5. **Install** — `nixos-install --max-jobs 1 --cores 1` — tránh OOM
6. **`canTouchEfiVariables = false`** — VMware UEFI không cho ghi NVRAM

Còn `systemd-boot` không có GRUB? Không sao. Một lần vào Boot Manager là VMware nhớ cho những lần sau.

---

## Kết luận — sao vẫn chọn NixOS?

Tôi có thể dùng Ubuntu. 15 phút cài xong, có desktop, có mọi thứ.

Nhưng tôi muốn:
- `git push` config → server clone → `nixos-rebuild switch` = production
- 1 file mô tả cả OS
- Rollback nếu update hỏng — `nixos-rebuild switch --rollback`

Và quan trọng nhất: **không sợ hỏng máy**. Hỏng thì rebuild từ config. 30 phút là xong. Cài Ubuntu hỏng thì mất cả buổi.

Cái checkbox firmware UEFI đó bây giờ nằm trong muscle memory của tôi rồi. Cái `canTouchEfiVariables` cũng vậy.

Tôi không nói ai cũng nên dùng NixOS. Nhưng nếu anh em thích cái cảm giác kiểm soát mọi thứ — từ kernel parameter đến font chữ trong terminal — thì thử một lần đi.

Biết đâu cũng mất 5 tiếng như tôi.

Nhưng 5 tiếng đó đáng đấy.

---

À mà nhớ swap nhé. Tôi quên mất 2 lần. Lần thứ 3 mới nhớ.

🦀
