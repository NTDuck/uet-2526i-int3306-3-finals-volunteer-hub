<script lang="ts">
  import type { PageData } from "$types";

  let { data, form } = $props() satisfies {
    data: PageData,
    form: {
      errors: string[],
      data: {
        usernameOrEmail: string,
        password: string,
      },
    },
  };
</script>

<h1>log-in!</h1>

<form method="POST">
  <label for="username-or-email">username/email</label>
  <input type="text" name="username-or-email" value={form?.data.usernameOrEmail ?? ""} required>

  <label for="password">password</label>
  <input type="password" name="password" value={form?.data.password ?? ""} required>

  <button type="submit">log-in</button>
</form>

{#each form?.errors as error}
  <p style="color: red;">{error}</p>
{/each}
