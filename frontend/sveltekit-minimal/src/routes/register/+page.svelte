<script lang="ts">
  import type { PageData } from "$types";

  let { data, form } = $props() satisfies {
    data: PageData,
    form: {
      errors: string[],
      data: {
        userRole: string,
        username: string,
        email: string,
        password: string,
        fullName: string,
      },
    },
  };
</script>

<h1>register!</h1>

<form method="POST" enctype="multipart/form-data">
  <label for="user-role">user-role</label>
  <select name="user-role" required>
    <option value="volunteer">volunteer</option>
    <option value="event-manager">event-manager</option>
    <option value="administrator">administrator</option>
  </select>

  <label for="username">username</label>
  <input type="text" name="username" value={form?.data.username ?? ""} required>

  <label for="email">email</label>
  <input type="text" name="email" value={form?.data.email ?? ""} required>

  <label for="password">password</label>
  <input type="password" name="password" value={form?.data.password ?? ""} required>

  <label for="full-name">full-name</label>
  <input type="text" name="full-name" value={form?.data.fullName ?? ""} required>

  <label for="avatar">avatar</label>
  <input type="file" name="avatar" accept="image/*">

  <button type="submit">register</button>
</form>

{#each form?.errors as error}
  <p style="color: red;">{error.message}</p>
{/each}
