<script lang="ts">
  let { children } = $props();
</script>

{@render children?.()}

<form method="POST" action="/logout">
  <button>log-out</button>
</form>
