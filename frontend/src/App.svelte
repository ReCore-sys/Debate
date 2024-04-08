<script lang="ts">
    import Channels from "./components/Channels/Channels.svelte";
    import Chat from "./components/Chat/Chat.svelte";
    import Hubs from "./components/Hubs/Hubs.svelte";
    import Users from "./components/Users/Users.svelte";
    import Login from "./components/Login/Login.svelte";
    $: logged_in = false;
    $: session_token = "";
</script>

<main>
    {#if logged_in}
    <h1>Token: {session_token}</h1>
    <div class="scroll-container">
        <div class="main-view">
            <Hubs />
            <Channels />
            <Chat />
            <Users />
        </div>
    </div>
    {:else}
    <Login bind:logged_in bind:session_token />
    {/if}
</main>

<style>
    /* Styles for the scrolling container */
  .scroll-container {
    height: 100vh; /* 100% viewport height */
    background-color: #1f252a; /* Background color (optional) */
    padding: 10px 0;
    
  }
    .main-view {
        display: flex;
        flex-direction: row;
        background-color: #1f252a
    }
</style>