<script lang="ts">
  let { children } = $props();
</script>

<nav>
  <a href="/login">log-in</a>
  <a href="/register">register</a>
  <a href="/dashboard">dashboard</a>
</nav>

{@render children?.()}
