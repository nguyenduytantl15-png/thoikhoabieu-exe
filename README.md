# Đóng gói "Xếp thời khóa biểu" thành file .exe (Windows)

Thư mục này chứa phần mềm (`dist/index.html`) và cấu hình để GitHub tự build ra file `.exe`.
Không cần cài gì trên máy của bạn.

## Build lần đầu (khoảng 10–15 phút)

1. Vào https://github.com, đăng nhập, bấm **New** để tạo kho mới, ví dụ `xep-tkb-exe`.
   Chọn **Private** nếu không muốn người khác xem mã nguồn.
2. Trong kho mới, bấm **uploading an existing file**.
   Giải nén file zip, kéo **toàn bộ nội dung bên trong thư mục `xep-tkb-exe`** vào trang
   (gồm `dist`, `src-tauri`, `.github`, `package.json`, `README.md`), rồi bấm **Commit changes**.
   - Lưu ý: thư mục `.github` bị Windows ẩn đi. Nếu kéo thả không thấy nó, bật
     "Hiện mục ẩn" (Hidden items) trong File Explorer, hoặc tạo file thủ công trên GitHub:
     **Add file → Create new file**, gõ tên `.github/workflows/build-windows.yml` rồi dán nội dung file đó vào.
3. Bấm tab **Actions**: thấy dòng "Build Windows EXE" đang chạy (vòng tròn vàng).
   Chờ đến khi có dấu tích xanh.
4. Bấm vào lần chạy đó, kéo xuống mục **Artifacts**, tải **XepThoiKhoaBieu-Windows** (file zip). Bên trong có:
   - `XepThoiKhoaBieu_1.0.0_x64-setup.exe`: **bản cài đặt**, tạo biểu tượng ngoài Desktop và trong menu Start.
   - `XepThoiKhoaBieu-portable.exe`: **bản chạy luôn**, không cần cài, chép vào USB mang đi được
     (cần Windows 10/11 có sẵn WebView2, hầu hết máy đều có).

## Cập nhật phiên bản mới
Thay file `dist/index.html` bằng file `xep-thoi-khoa-bieu.html` mới (đổi tên thành `index.html`).
Muốn đổi số phiên bản thì sửa `"version"` trong `src-tauri/tauri.conf.json`.
GitHub tự build lại; tải file mới ở tab Actions như trên.

## Lưu ý khi dùng bản .exe
- Dữ liệu của bản .exe **lưu riêng**, không dùng chung với dữ liệu đang có trong Chrome/Edge.
  Để chuyển sang: trong Chrome bấm **Xuất file** (.json), mở bản .exe, bấm **Mở file** và chọn file đó.
- Mọi nút tải (Excel, .json, sổ dạy thay...) sẽ hiện hộp **Lưu thành...** để chọn nơi lưu.
- Báo cáo BGH và các trang in mở thẳng hộp thoại in của Windows.
- Lần đầu chạy, Windows có thể báo "Windows protected your PC" vì file chưa có chữ ký số:
  bấm **More info → Run anyway**.
