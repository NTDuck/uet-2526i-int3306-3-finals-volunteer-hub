<script lang="ts">
  import type { PageData } from "$types";

  let { data } = $props() satisfies {
    data: PageData,
  };
</script>

<h1>profile!</h1>

<form onsubmit={ (event) => event.preventDefault() }>
  <label for="id">id</label>
  <input type="text" value={data.user.id} readonly disabled>

  <label for="user-role">user-role</label>
  <input type="text" value={data.user.role} readonly disabled>

  <label for="username">username</label>
  <input type="text" value={data.user.username} readonly disabled>

  <label for="email">email</label>
  <input type="text" value={data.user.email} readonly disabled>

  <label for="full-name">full-name</label>
  <input type="text" value={data.user.fullName} readonly disabled>

  <label for="avatar">avatar</label>
  {#if data.user.avatarUrl}
    <img
      src={data.user.avatarUrl}
      alt="avatar"
      style="width: 64px; height: 64px; object-fit: cover;"
    >
  {:else}
    <input type="text" value="" readonly disabled>
  {/if}

  <hr>
  
  {#each data.user.statuses as status, i}
    {#if status.created}
      <label for={"status-" + i + "-kind"}>status</label>
      <input id={"status-" + i + "-kind"} type="text" value="created" readonly disabled>
  
      <label for={"status-" + i + "-created-at"}>created-at</label>
      <input
        id={"status-" + i + "-created-at"}
        type="text"
        value={status.created.created_at}
        readonly disabled
      >
  
    {:else if status.updated}
      <label for={"status-" + i + "-kind"}>status</label>
      <input id={"status-" + i + "-kind"} type="text" value="updated" readonly disabled>
  
      <label for={"status-" + i + "-updated-at"}>updated-at</label>
      <input
        id={"status-" + i + "-updated-at"}
        type="text"
        value={status.updated.updated_at}
        readonly disabled
      >
  
    {:else if status.suspended}
      <label for={"status-" + i + "-kind"}>status</label>
      <input id={"status-" + i + "-kind"} type="text" value="suspended" readonly disabled>
  
      <label for={"status-" + i + "-suspended-at"}>suspended-at</label>
      <input
        id={"status-" + i + "-suspended-at"}
        type="text"
        value={status.suspended.suspended_at}
        readonly disabled
      >
  
      <label for={"status-" + i + "-suspended-by"}>suspended-by-administrator-id</label>
      <input
        id={"status-" + i + "-suspended-by"}
        type="text"
        value={status.suspended.suspended_by_administrator_id}
        readonly disabled
      >
  
    {:else if status.unsuspended}
      <label for={"status-" + i + "-kind"}>status</label>
      <input id={"status-" + i + "-kind"} type="text" value="unsuspended" readonly disabled>
  
      <label for={"status-" + i + "-unsuspended-at"}>unsuspended-at</label>
      <input
        id={"status-" + i + "-unsuspended-at"}
        type="text"
        value={status.unsuspended.unsuspended_at}
        readonly disabled
      >
  
      <label for={"status-" + i + "-unsuspended-by"}>unsuspended-by-administrator-id</label>
      <input
        id={"status-" + i + "-unsuspended-by"}
        type="text"
        value={status.unsuspended.unsuspended_by_administrator_id}
        readonly disabled
      >
    {/if}
  
    <hr>
  {/each}
</form>
