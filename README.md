# uet-2526i-int3306-3-finals-volunteer-hub

## Deps

- [Rust](https://rust-lang.org/tools/install/)
- [wasm-pack](https://drager.github.io/wasm-pack/) basically
  `cargo install wasm-pack`
- [Deno](https://docs.deno.com/runtime/getting_started/installation/)

## How to install

```cmd
cd ./frontend/sveltekit-minimal && deno install
```

## How to run

```cmd
deno task build-wasm
```

Development

```cmd
deno task dev-sveltekit-minimal
```

Production

```cmd
deno task build-sveltekit-minimal
deno task preview-sveltekit-minimal
```

# Use cases

> https://itest.com.vn/lects/webappdev/mockproj/VolunteerHub.htm

---

### Tình nguyện viên

- **Đăng ký/Đăng nhập**: Tạo tài khoản, đăng nhập bằng email/password.
  > [./backend/core/use-cases/boundaries/sign_in.rs](./backend/core/use-cases/boundaries/sign_in.rs)
  > |
  > [./backend/core/use-cases/interactors/sign_in.rs](./backend/core/use-cases/interactors/sign_in.rs)\
  > [./backend/core/use-cases/boundaries/sign_up.rs](./backend/core/use-cases/boundaries/sign_up.rs)
  > |
  > [./backend/core/use-cases/interactors/sign_up.rs](./backend/core/use-cases/interactors/sign_up.rs)
- **Xem sự kiện**: Xem danh sách sự kiện (tên, ngày, địa điểm, mô tả), lọc theo
  thời gian/danh mục.
  > [./backend/core/use-cases/boundaries/view_published_events.rs](./backend/core/use-cases/boundaries/view_published_events.rs)
  > |
  > [./backend/core/use-cases/interactors/view_published_events.rs](./backend/core/use-cases/interactors/view_published_events.rs)
- **Đăng ký sự kiện**: Đăng ký tham gia sự kiện, nhận thông báo xác nhận.
  > [./backend/core/use-cases/boundaries/subscribe_to_event.rs](./backend/core/use-cases/boundaries/subscribe_to_event.rs)
  > |
  > [./backend/core/use-cases/interactors/subscribe_to_event.rs](./backend/core/use-cases/interactors/subscribe_to_event.rs)
- **Thoái đăng sự kiện**: Hủy đăng ký trước khi sự kiện diễn ra.
  > [./backend/core/use-cases/boundaries/unsubscribe_from_event.rs](./backend/core/use-cases/boundaries/unsubscribe_from_event.rs)
  > |
  > [./backend/core/use-cases/interactors/unsubscribe_from_event.rs](./backend/core/use-cases/interactors/unsubscribe_from_event.rs)
- **Xem lịch sử tham gia**: Xem danh sách sự kiện đã tham gia, trạng thái hoàn
  thành.
  > [./backend/core/use-cases/boundaries/view_event_history.rs](./backend/core/use-cases/boundaries/view_event_history.rs)
  > |
  > [./backend/core/use-cases/interactors/view_event_history.rs](./backend/core/use-cases/interactors/view_event_history.rs)
- **Nhận thông báo**: Nhận thông báo trạng thái đăng ký/hoàn thành (Web Push API).
- **Truy cập kênh trao đổi**: Post bài, comment, like trên kênh sự kiện (tương tự
  wall Facebook), chỉ sau khi sự kiện được duyệt.
  > [./backend/core/use-cases/boundaries/create_post.rs](./backend/core/use-cases/boundaries/create_post.rs)
  > |
  > [./backend/core/use-cases/interactors/create_post.rs](./backend/core/use-cases/interactors/create_post.rs)\
  > [./backend/core/use-cases/boundaries/create_reaction.rs](./backend/core/use-cases/boundaries/create_reaction.rs)
  > |
  > [./backend/core/use-cases/interactors/create_reaction.rs](./backend/core/use-cases/interactors/create_reaction.rs)\
  > [./backend/core/use-cases/boundaries/create_comment.rs](./backend/core/use-cases/boundaries/create_comment.rs)
  > |
  > [./backend/core/use-cases/interactors/create_comment.rs](./backend/core/use-cases/interactors/create_comment.rs)
- **Xem Dashboard**: Xem tổng hợp sự kiện liên quan (mới công bố, có tin bài mới),
  sự kiện thu hút (tăng thành viên/trao đổi/like nhanh).
  > [./backend/core/use-cases/boundaries/view_event_recommendation.rs](./backend/core/use-cases/boundaries/view_event_recommendation.rs)
  > |
  > [./backend/core/use-cases/interactors/view_event_recommendation.rs](./backend/core/use-cases/interactors/view_event_recommendation.rs)

---

### Quản lý sự kiện

- **Đăng ký/Đăng nhập**: Tạo tài khoản, đăng nhập bằng email/password.
  > [./backend/core/use-cases/boundaries/sign_in.rs](./backend/core/use-cases/boundaries/sign_in.rs)
  > |
  > [./backend/core/use-cases/interactors/sign_in.rs](./backend/core/use-cases/interactors/sign_in.rs)\
  > [./backend/core/use-cases/boundaries/sign_up.rs](./backend/core/use-cases/boundaries/sign_up.rs)
  > |
  > [./backend/core/use-cases/interactors/sign_up.rs](./backend/core/use-cases/interactors/sign_up.rs)
- **Quản lý sự kiện**: Tạo, sửa, xóa sự kiện (tên, ngày, địa điểm, mô tả). Validate
  input (Joi/Yup).
  > [./backend/core/use-cases/boundaries/create_event.rs](./backend/core/use-cases/boundaries/create_event.rs)
  > |
  > [./backend/core/use-cases/interactors/create_event.rs](./backend/core/use-cases/interactors/create_event.rs)\
  > [./backend/core/use-cases/boundaries/update_event.rs](./backend/core/use-cases/boundaries/update_event.rs)
  > |
  > [./backend/core/use-cases/interactors/update_event.rs](./backend/core/use-cases/interactors/update_event.rs)\
  > [./backend/core/use-cases/boundaries/remove_event.rs](./backend/core/use-cases/boundaries/remove_event.rs)
  > |
  > [./backend/core/use-cases/interactors/remove_event.rs](./backend/core/use-cases/interactors/remove_event.rs)\
- **Xác nhận đăng ký**: Duyệt/hủy đăng ký của tình nguyện viên.
  > [./backend/core/use-cases/boundaries/accept_event_registration.rs](./backend/core/use-cases/boundaries/accept_event_registration.rs)
  > |
  > [./backend/core/use-cases/interactors/accept_event_registration.rs](./backend/core/use-cases/interactors/accept_event_registration.rs)\
  > [./backend/core/use-cases/boundaries/decline_event_registration.rs](./backend/core/use-cases/boundaries/decline_event_registration.rs)
  > |
  > [./backend/core/use-cases/interactors/decline_event_registration.rs](./backend/core/use-cases/interactors/decline_event_registration.rs)
- **Đánh dấu hoàn thành**: Cập nhật trạng thái hoàn thành cho tình nguyện viên sau
  sự kiện.
  > [./backend/core/use-cases/boundaries/complete_event_registration.rs](./backend/core/use-cases/boundaries/complete_event_registration.rs)
  > |
  > [./backend/core/use-cases/interactors/complete_event_registration.rs](./backend/core/use-cases/interactors/complete_event_registration.rs)
- **Xem báo cáo**: Xem danh sách tình nguyện viên tham gia sự kiện.
  > [./backend/core/use-cases/boundaries/view_event_volunteers.rs](./backend/core/use-cases/boundaries/view_event_volunteers.rs)
  > |
  > [./backend/core/use-cases/interactors/view_event_volunteers.rs](./backend/core/use-cases/interactors/view_event_volunteers.rs)
- **Truy cập kênh trao đổi**: Post bài, comment, like trên kênh sự kiện (tương tự
  wall Facebook), chỉ sau khi sự kiện được duyệt.
  > [./backend/core/use-cases/boundaries/create_post.rs](./backend/core/use-cases/boundaries/create_post.rs)
  > |
  > [./backend/core/use-cases/interactors/create_post.rs](./backend/core/use-cases/interactors/create_post.rs)\
  > [./backend/core/use-cases/boundaries/create_reaction.rs](./backend/core/use-cases/boundaries/create_reaction.rs)
  > |
  > [./backend/core/use-cases/interactors/create_reaction.rs](./backend/core/use-cases/interactors/create_reaction.rs)\
  > [./backend/core/use-cases/boundaries/create_comment.rs](./backend/core/use-cases/boundaries/create_comment.rs)
  > |
  > [./backend/core/use-cases/interactors/create_comment.rs](./backend/core/use-cases/interactors/create_comment.rs)
- **Xem Dashboard**: Xem tổng hợp sự kiện liên quan (mới công bố, có tin bài mới),
  sự kiện thu hút (tăng thành viên/trao đổi/like nhanh).
  > [./backend/core/use-cases/boundaries/view_event_recommendation.rs](./backend/core/use-cases/boundaries/view_event_recommendation.rs)
  > |
  > [./backend/core/use-cases/interactors/view_event_recommendation.rs](./backend/core/use-cases/interactors/view_event_recommendation.rs)

---

### Admin

- **Đăng ký/Đăng nhập**: Tạo tài khoản, đăng nhập bằng email/password.
  > [./backend/core/use-cases/boundaries/sign_in.rs](./backend/core/use-cases/boundaries/sign_in.rs)
  > |
  > [./backend/core/use-cases/interactors/sign_in.rs](./backend/core/use-cases/interactors/sign_in.rs)\
  > [./backend/core/use-cases/boundaries/sign_up.rs](./backend/core/use-cases/boundaries/sign_up.rs)
  > |
  > [./backend/core/use-cases/interactors/sign_up.rs](./backend/core/use-cases/interactors/sign_up.rs)
- **Quản lý sự kiện**: Duyệt/xóa sự kiện do quản lý sự kiện tạo.
  > [./backend/core/use-cases/boundaries/approve_event.rs](./backend/core/use-cases/boundaries/approve_event.rs)
  > |
  > [./backend/core/use-cases/interactors/approve_event.rs](./backend/core/use-cases/interactors/approve_event.rs)\
  > [./backend/core/use-cases/boundaries/reject_event.rs](./backend/core/use-cases/boundaries/reject_event.rs)
  > |
  > [./backend/core/use-cases/interactors/reject_event.rs](./backend/core/use-cases/interactors/reject_event.rs)
- **Quản lý người dùng**: Xem, khóa/mở tài khoản tình nguyện viên/quản lý sự kiện.
  > [./backend/core/use-cases/boundaries/view_non_admin_users.rs](./backend/core/use-cases/boundaries/view_non_admin_users.rs)
  > |
  > [./backend/core/use-cases/interactors/view_non_admin_users.rs](./backend/core/use-cases/interactors/view_non_admin_users.rs)\
  > [./backend/core/use-cases/boundaries/view_non_admin_user.rs](./backend/core/use-cases/boundaries/view_non_admin_user.rs)
  > |
  > [./backend/core/use-cases/interactors/view_non_admin_user.rs](./backend/core/use-cases/interactors/view_non_admin_user.rs)\
  > [./backend/core/use-cases/boundaries/suspend_non_admin_user.rs](./backend/core/use-cases/boundaries/suspend_non_admin_user.rs)
  > |
  > [./backend/core/use-cases/interactors/suspend_non_admin_user.rs](./backend/core/use-cases/interactors/suspend_non_admin_user.rs)\
  > [./backend/core/use-cases/boundaries/reinstate_non_admin_user.rs](./backend/core/use-cases/boundaries/reinstate_non_admin_user.rs)
  > |
  > [./backend/core/use-cases/interactors/reinstate_non_admin_user.rs](./backend/core/use-cases/interactors/reinstate_non_admin_user.rs)
- **Xuất dữ liệu**: Export danh sách sự kiện/tình nguyện viên (CSV/JSON).
  > [./backend/core/use-cases/boundaries/export_events.rs](./backend/core/use-cases/boundaries/export_events.rs)
  > |
  > [./backend/core/use-cases/interactors/export_events.rs](./backend/core/use-cases/interactors/export_events.rs)\
  > [./backend/core/use-cases/boundaries/export_volunteers.rs](./backend/core/use-cases/boundaries/export_volunteers.rs)
  > |
  > [./backend/core/use-cases/interactors/export_volunteers.rs](./backend/core/use-cases/interactors/export_volunteers.rs)
- **Xem Dashboard**: Xem tổng hợp sự kiện liên quan (mới công bố, có tin bài mới),
  sự kiện thu hút (tăng thành viên/trao đổi/like nhanh).
  > [./backend/core/use-cases/boundaries/view_event_recommendation.rs](./backend/core/use-cases/boundaries/view_event_recommendation.rs)
  > |
  > [./backend/core/use-cases/interactors/view_event_recommendation.rs](./backend/core/use-cases/interactors/view_event_recommendation.rs)