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
        firstName: string,
        lastName: string,
      },
    },
  };
</script>

<h1>register!</h1>

<form method="POST">
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

  <label for="first-name">first-name</label>
  <input type="text" name="first-name" value={form?.data.firstName ?? ""} required>

  <label for="last-name">last-name</label>
  <input type="text" name="last-name" value={form?.data.lastName ?? ""} required>

  <button type="submit">register</button>
</form>

{#each form?.errors as error}
  <p style="color: red;">{error}</p>
{/each}
