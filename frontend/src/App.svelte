<script lang="ts">
    import Channels from "./components/Channels/Channels.svelte";
    import Chat from "./components/Chat/Chat.svelte";
    import Hubs from "./components/Hubs/Hubs.svelte";
    import Users from "./components/Users/Users.svelte";
</script>

<main>
    <div class="scroll-container">
        <div class="main-view">
            <Hubs />
            <Channels />
            <Chat />
            <Users />
        </div>
    </div>
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

