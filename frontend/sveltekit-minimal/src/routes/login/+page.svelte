<script lang="ts">
  let { data, form } = $props();
</script>

<h1>log-in!</h1>

<form method="POST">
  <label for="username-or-email">username/email</label>
  <input type="text" name="username-or-email" value={form?.description ?? ""} required>

  <label for="password">password</label>
  <input type="password" name="password" value={form?.password ?? ""} required>

  <button type="submit">log-in</button>
</form>

{#if form?.error}
  <p>{form.error}</p>
{/if}