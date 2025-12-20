# uet-2526i-int3306-3-finals-volunteer-hub

## Deps

- [Rust](https://rust-lang.org/tools/install/)<br> [windows] use
  `stable-x86_64-pc-windows-msvc` toolchain.
- [MSVC Build tools](https://visualstudio.microsoft.com/downloads/)<br>
  [windows] use the following individual components: Windows Universal C
  Runtime, Windows 11 SDK (10.0.26100.7175), Windows Performance Toolkit, MSVC
  v143 - VS 2022 C++ ARM build tools (v14.44-17.14), MSVC v143 - VS 2022 C++
  x64/x86 build tools (v14.44-17.14)
- [wasm-pack](https://drager.github.io/wasm-pack/)<br> run
  `cargo install wasm-pack`
- [Deno](https://docs.deno.com/runtime/getting_started/installation/)

## How to install

```cmd
$ cd ./frontend/sveltekit-minimal && deno install
```

## How to run

```cmd
$ deno task build-wasm
```

### Development

```cmd
$ deno task dev-sveltekit-minimal
```

### Production

```cmd
$ deno task build-sveltekit-minimal
$ deno task preview-sveltekit-minimal
```

## How to please Github

Assume we're merging PR branch `indev` into `master`. The following snippet
achieves the same thing as if we visit each conflict and select
`Keep Current Changes`.

```cmd
# https://trunk.io/blog/git-commit-messages-are-useless
$ git config --global alias.nccommit "commit -a --allow-empty-message -m ''"

$ git pull origin master
$ git checkout indev
$ git merge master
$ git checkout --ours .
$ git add .
$ git nccommit
$ git push -u origin indev
```

## Use cases

> https://itest.com.vn/lects/webappdev/mockproj/VolunteerHub.htm

---

### Tình nguyện viên

- **Đăng ký/Đăng nhập**: Tạo tài khoản, đăng nhập bằng email/password.
  > [./backend/core/use-cases/src/boundaries/sign_in.rs](./backend/core/use-cases/src/boundaries/sign_in.rs)
  > |
  > [./backend/core/use-cases/src/interactors/sign_in.rs](./backend/core/use-cases/src/interactors/sign_in.rs)\
  > [./backend/core/use-cases/src/boundaries/sign_up.rs](./backend/core/use-cases/src/boundaries/sign_up.rs)
  > |
  > [./backend/core/use-cases/src/interactors/sign_up.rs](./backend/core/use-cases/src/interactors/sign_up.rs)
- **Xem sự kiện**: Xem danh sách sự kiện (tên, ngày, địa điểm, mô tả), lọc theo
  thời gian/danh mục.
  > [./backend/core/use-cases/src/boundaries/view_published_events.rs](./backend/core/use-cases/src/boundaries/view_published_events.rs)
  > |
  > [./backend/core/use-cases/src/interactors/view_published_events.rs](./backend/core/use-cases/src/interactors/view_published_events.rs)
- **Đăng ký sự kiện**: Đăng ký tham gia sự kiện, nhận thông báo xác nhận.
  > [./backend/core/use-cases/src/boundaries/subscribe_to_event.rs](./backend/core/use-cases/src/boundaries/subscribe_to_event.rs)
  > |
  > [./backend/core/use-cases/src/interactors/subscribe_to_event.rs](./backend/core/use-cases/src/interactors/subscribe_to_event.rs)
- **Thoái đăng sự kiện**: Hủy đăng ký trước khi sự kiện diễn ra.
  > [./backend/core/use-cases/src/boundaries/unsubscribe_from_event.rs](./backend/core/use-cases/src/boundaries/unsubscribe_from_event.rs)
  > |
  > [./backend/core/use-cases/src/interactors/unsubscribe_from_event.rs](./backend/core/use-cases/src/interactors/unsubscribe_from_event.rs)
- **Xem lịch sử tham gia**: Xem danh sách sự kiện đã tham gia, trạng thái hoàn
  thành.
  > [./backend/core/use-cases/src/boundaries/view_event_history.rs](./backend/core/use-cases/src/boundaries/view_event_history.rs)
  > |
  > [./backend/core/use-cases/src/interactors/view_event_history.rs](./backend/core/use-cases/src/interactors/view_event_history.rs)
- **Nhận thông báo**: Nhận thông báo trạng thái đăng ký/hoàn thành (Web Push
  API).
- **Truy cập kênh trao đổi**: Post bài, comment, like trên kênh sự kiện (tương
  tự wall Facebook), chỉ sau khi sự kiện được duyệt.
  > [./backend/core/use-cases/src/boundaries/view_event_channel.rs](./backend/core/use-cases/src/boundaries/view_event_channel.rs)
  > |
  > [./backend/core/use-cases/src/interactors/view_event_channel.rs](./backend/core/use-cases/src/interactors/view_event_channel.rs)\
  > [./backend/core/use-cases/src/boundaries/view_post.rs](./backend/core/use-cases/src/boundaries/view_post.rs)
  > |
  > [./backend/core/use-cases/src/interactors/view_post.rs](./backend/core/use-cases/src/interactors/view_post.rs)\
  > [./backend/core/use-cases/src/boundaries/create_post.rs](./backend/core/use-cases/src/boundaries/create_post.rs)
  > |
  > [./backend/core/use-cases/src/interactors/create_post.rs](./backend/core/use-cases/src/interactors/create_post.rs)\
  > [./backend/core/use-cases/src/boundaries/create_reaction.rs](./backend/core/use-cases/src/boundaries/create_reaction.rs)
  > |
  > [./backend/core/use-cases/src/interactors/create_reaction.rs](./backend/core/use-cases/src/interactors/create_reaction.rs)\
  > [./backend/core/use-cases/src/boundaries/create_comment.rs](./backend/core/use-cases/src/boundaries/create_comment.rs)
  > |
  > [./backend/core/use-cases/src/interactors/create_comment.rs](./backend/core/use-cases/src/interactors/create_comment.rs)\
  > [./backend/core/use-cases/src/boundaries/update_post.rs](./backend/core/use-cases/src/boundaries/update_post.rs)
  > |
  > [./backend/core/use-cases/src/interactors/update_post.rs](./backend/core/use-cases/src/interactors/update_post.rs)\
  > [./backend/core/use-cases/src/boundaries/update_comment.rs](./backend/core/use-cases/src/boundaries/update_comment.rs)
  > |
  > [./backend/core/use-cases/src/interactors/update_comment.rs](./backend/core/use-cases/src/interactors/update_comment.rs)\
  > [./backend/core/use-cases/src/boundaries/remove_post.rs](./backend/core/use-cases/src/boundaries/remove_post.rs)
  > |
  > [./backend/core/use-cases/src/interactors/remove_post.rs](./backend/core/use-cases/src/interactors/remove_post.rs)\
  > [./backend/core/use-cases/src/boundaries/remove_reaction.rs](./backend/core/use-cases/src/boundaries/remove_reaction.rs)
  > |
  > [./backend/core/use-cases/src/interactors/remove_reaction.rs](./backend/core/use-cases/src/interactors/remove_reaction.rs)\
  > [./backend/core/use-cases/src/boundaries/remove_comment.rs](./backend/core/use-cases/src/boundaries/remove_comment.rs)
  > |
  > [./backend/core/use-cases/src/interactors/remove_comment.rs](./backend/core/use-cases/src/interactors/remove_comment.rs)
- **Xem Dashboard**: Xem tổng hợp sự kiện liên quan (mới công bố, có tin bài
  mới), sự kiện thu hút (tăng thành viên/trao đổi/like nhanh).
  > [./backend/core/use-cases/src/boundaries/view_event_recommendation.rs](./backend/core/use-cases/src/boundaries/view_event_recommendation.rs)
  > |
  > [./backend/core/use-cases/src/interactors/view_event_recommendation.rs](./backend/core/use-cases/src/interactors/view_event_recommendation.rs)

---

### Quản lý sự kiện

- **Đăng ký/Đăng nhập**: Tạo tài khoản, đăng nhập bằng email/password.
  > [./backend/core/use-cases/src/boundaries/sign_in.rs](./backend/core/use-cases/src/boundaries/sign_in.rs)
  > |
  > [./backend/core/use-cases/src/interactors/sign_in.rs](./backend/core/use-cases/src/interactors/sign_in.rs)\
  > [./backend/core/use-cases/src/boundaries/sign_up.rs](./backend/core/use-cases/src/boundaries/sign_up.rs)
  > |
  > [./backend/core/use-cases/src/interactors/sign_up.rs](./backend/core/use-cases/src/interactors/sign_up.rs)
- **Quản lý sự kiện**: Tạo, sửa, xóa sự kiện (tên, ngày, địa điểm, mô tả).
  Validate input (Joi/Yup).
  > [./backend/core/use-cases/src/boundaries/create_event.rs](./backend/core/use-cases/src/boundaries/create_event.rs)
  > |
  > [./backend/core/use-cases/src/interactors/create_event.rs](./backend/core/use-cases/src/interactors/create_event.rs)\
  > [./backend/core/use-cases/src/boundaries/update_event.rs](./backend/core/use-cases/src/boundaries/update_event.rs)
  > |
  > [./backend/core/use-cases/src/interactors/update_event.rs](./backend/core/use-cases/src/interactors/update_event.rs)\
  > [./backend/core/use-cases/src/boundaries/remove_event.rs](./backend/core/use-cases/src/boundaries/remove_event.rs)
  > |
  > [./backend/core/use-cases/src/interactors/remove_event.rs](./backend/core/use-cases/src/interactors/remove_event.rs)\
- **Xác nhận đăng ký**: Duyệt/hủy đăng ký của tình nguyện viên.
  > [./backend/core/use-cases/src/boundaries/moderate_event_registration.rs](./backend/core/use-cases/src/boundaries/moderate_event_registration.rs)
  > |
  > [./backend/core/use-cases/src/interactors/moderate_event_registration.rs](./backend/core/use-cases/src/interactors/moderate_event_registration.rs)
- **Đánh dấu hoàn thành**: Cập nhật trạng thái hoàn thành cho tình nguyện viên
  sau sự kiện.
  > [./backend/core/use-cases/src/boundaries/moderate_event_registration.rs](./backend/core/use-cases/src/boundaries/moderate_event_registration.rs)
  > |
  > [./backend/core/use-cases/src/interactors/moderate_event_registration.rs](./backend/core/use-cases/src/interactors/moderate_event_registration.rs)
- **Xem báo cáo**: Xem danh sách tình nguyện viên tham gia sự kiện.
  > [./backend/core/use-cases/src/boundaries/view_event_volunteers.rs](./backend/core/use-cases/src/boundaries/view_event_volunteers.rs)
  > |
  > [./backend/core/use-cases/src/interactors/view_event_volunteers.rs](./backend/core/use-cases/src/interactors/view_event_volunteers.rs)
- **Truy cập kênh trao đổi**: Post bài, comment, like trên kênh sự kiện (tương
  tự wall Facebook), chỉ sau khi sự kiện được duyệt.
  > [./backend/core/use-cases/src/boundaries/view_event_channel.rs](./backend/core/use-cases/src/boundaries/view_event_channel.rs)
  > |
  > [./backend/core/use-cases/src/interactors/view_event_channel.rs](./backend/core/use-cases/src/interactors/view_event_channel.rs)\
  > [./backend/core/use-cases/src/boundaries/view_post.rs](./backend/core/use-cases/src/boundaries/view_post.rs)
  > |
  > [./backend/core/use-cases/src/interactors/view_post.rs](./backend/core/use-cases/src/interactors/view_post.rs)\
  > [./backend/core/use-cases/src/boundaries/create_post.rs](./backend/core/use-cases/src/boundaries/create_post.rs)
  > |
  > [./backend/core/use-cases/src/interactors/create_post.rs](./backend/core/use-cases/src/interactors/create_post.rs)\
  > [./backend/core/use-cases/src/boundaries/create_reaction.rs](./backend/core/use-cases/src/boundaries/create_reaction.rs)
  > |
  > [./backend/core/use-cases/src/interactors/create_reaction.rs](./backend/core/use-cases/src/interactors/create_reaction.rs)\
  > [./backend/core/use-cases/src/boundaries/create_comment.rs](./backend/core/use-cases/src/boundaries/create_comment.rs)
  > |
  > [./backend/core/use-cases/src/interactors/create_comment.rs](./backend/core/use-cases/src/interactors/create_comment.rs)\
  > [./backend/core/use-cases/src/boundaries/update_post.rs](./backend/core/use-cases/src/boundaries/update_post.rs)
  > |
  > [./backend/core/use-cases/src/interactors/update_post.rs](./backend/core/use-cases/src/interactors/update_post.rs)\
  > [./backend/core/use-cases/src/boundaries/update_comment.rs](./backend/core/use-cases/src/boundaries/update_comment.rs)
  > |
  > [./backend/core/use-cases/src/interactors/update_comment.rs](./backend/core/use-cases/src/interactors/update_comment.rs)\
  > [./backend/core/use-cases/src/boundaries/remove_post.rs](./backend/core/use-cases/src/boundaries/remove_post.rs)
  > |
  > [./backend/core/use-cases/src/interactors/remove_post.rs](./backend/core/use-cases/src/interactors/remove_post.rs)\
  > [./backend/core/use-cases/src/boundaries/remove_reaction.rs](./backend/core/use-cases/src/boundaries/remove_reaction.rs)
  > |
  > [./backend/core/use-cases/src/interactors/remove_reaction.rs](./backend/core/use-cases/src/interactors/remove_reaction.rs)\
  > [./backend/core/use-cases/src/boundaries/remove_comment.rs](./backend/core/use-cases/src/boundaries/remove_comment.rs)
  > |
  > [./backend/core/use-cases/src/interactors/remove_comment.rs](./backend/core/use-cases/src/interactors/remove_comment.rs)
- **Xem Dashboard**: Xem tổng hợp sự kiện liên quan (mới công bố, có tin bài
  mới), sự kiện thu hút (tăng thành viên/trao đổi/like nhanh).
  > [./backend/core/use-cases/src/boundaries/view_event_recommendation.rs](./backend/core/use-cases/src/boundaries/view_event_recommendation.rs)
  > |
  > [./backend/core/use-cases/src/interactors/view_event_recommendation.rs](./backend/core/use-cases/src/interactors/view_event_recommendation.rs)

---

### Admin

- **Đăng ký/Đăng nhập**: Tạo tài khoản, đăng nhập bằng email/password.
  > [./backend/core/use-cases/src/boundaries/sign_in.rs](./backend/core/use-cases/src/boundaries/sign_in.rs)
  > |
  > [./backend/core/use-cases/src/interactors/sign_in.rs](./backend/core/use-cases/src/interactors/sign_in.rs)\
  > [./backend/core/use-cases/src/boundaries/sign_up.rs](./backend/core/use-cases/src/boundaries/sign_up.rs)
  > |
  > [./backend/core/use-cases/src/interactors/sign_up.rs](./backend/core/use-cases/src/interactors/sign_up.rs)
- **Quản lý sự kiện**: Duyệt/xóa sự kiện do quản lý sự kiện tạo.
  > [./backend/core/use-cases/src/boundaries/moderate_event.rs](./backend/core/use-cases/src/boundaries/moderate_event.rs)
  > |
  > [./backend/core/use-cases/src/interactors/moderate_event.rs](./backend/core/use-cases/src/interactors/moderate_event.rs)
- **Quản lý người dùng**: Xem, khóa/mở tài khoản tình nguyện viên/quản lý sự
  kiện.
  > [./backend/core/use-cases/src/boundaries/view_users.rs](./backend/core/use-cases/src/boundaries/view_users.rs)
  > |
  > [./backend/core/use-cases/src/interactors/view_users.rs](./backend/core/use-cases/src/interactors/view_users.rs)\
  > [./backend/core/use-cases/src/boundaries/moderate_user.rs](./backend/core/use-cases/src/boundaries/moderate_user.rs)
  > |
  > [./backend/core/use-cases/src/interactors/moderate_user.rs](./backend/core/use-cases/src/interactors/moderate_user.rs)
- **Xuất dữ liệu**: Export danh sách sự kiện/tình nguyện viên (CSV/JSON).
  > [./backend/core/use-cases/src/boundaries/export_events.rs](./backend/core/use-cases/src/boundaries/export_events.rs)
  > |
  > [./backend/core/use-cases/src/interactors/export_events.rs](./backend/core/use-cases/src/interactors/export_events.rs)\
  > [./backend/core/use-cases/src/boundaries/export_volunteers.rs](./backend/core/use-cases/src/boundaries/export_volunteers.rs)
  > |
  > [./backend/core/use-cases/src/interactors/export_volunteers.rs](./backend/core/use-cases/src/interactors/export_volunteers.rs)
- **Xem Dashboard**: Xem tổng hợp sự kiện liên quan (mới công bố, có tin bài
  mới), sự kiện thu hút (tăng thành viên/trao đổi/like nhanh).
  > [./backend/core/use-cases/src/boundaries/view_event_recommendation.rs](./backend/core/use-cases/src/boundaries/view_event_recommendation.rs)
  > |
  > [./backend/core/use-cases/src/interactors/view_event_recommendation.rs](./backend/core/use-cases/src/interactors/view_event_recommendation.rs)

## TODO

- Implement the following use cases: `view_self_profile`, `update_self_profile`,
  `view_user`, `view_event`, `view_published_event`
- Increase parallelism via e.g. `buffer_unordered(...)` for streams, `rayon`
- Replace `.filter_map(|transposable| async move { transposable.transpose() })`
  with `.transpose()` via an `axiom` trait
- Implement push notification
- Improve performance by collecting into `::smallvec::SmallVec` instead of
  `::std::vec::Vec` in hot loops
- Improve performance by querying specific column(s)
