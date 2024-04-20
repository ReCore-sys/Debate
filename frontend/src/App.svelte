<script lang="ts">
  import Channels from "./components/Channels/Channels.svelte";
  import Chat from "./components/Chat/Chat.svelte";
  import Hubs from "./components/Hubs/Hubs.svelte";
  import Users from "./components/Users/Users.svelte";
  import Login from "./components/Login/Login.svelte";
  import { invoke } from "@tauri-apps/api/core";
  $: logged_in = false;
  $: session_token = "";
  $: login_ready = false;
  $: current_message = "Checking connection to server";
  $: error_reason = 0;
  async function prelogin() {
    let res = await invoke("status");
    login_ready = true;
    if (res == null) {
      error_reason = 1;
      return false;
    }
    // @ts-ignore
    if (res.db == false) {
      error_reason = 2;
      return false;
    }
    return true
  }
</script>

<main>
  {#await prelogin()}
    <div class="loading-area">
      <h1>{current_message}</h1>
    </div>
  {:then res}
    {#if res}
      {#if logged_in}
        <h1>Token: {session_token}</h1>
        <div class="main-view">
          <Hubs />
          <Channels />
          <Chat />
          <Users />
        </div>
      {:else}
        <Login bind:logged_in bind:session_token />
      {/if}
    {:else if error_reason == 1}
      <h1>Connection to server failed</h1>
    {:else if error_reason == 2}
      <h1>Server could not connect to database</h1>
    {/if}
  {/await}
</main>

<style>
  .main-view {
    display: flex;
    flex-direction: row;
  }

  main {
    background-image: url("/bg4k-blue.png");
    background-size: cover;
    background-position: center;
    font-family: "Lato", sans-serif;
    font-weight: 400;
    font-style: normal;
    height: 100vh;
    padding: 10px 0;
  }
</style>
