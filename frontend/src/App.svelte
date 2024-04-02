<script lang="ts">
    import Channels from "./components/Channels/Channels.svelte";
    import Chat from "./components/Chat/Chat.svelte";
    import Hubs from "./components/Hubs/Hubs.svelte";
    import Users from "./components/Users/Users.svelte";
</script>

<main>
    <div class="main-view">
        <Hubs />
        <Channels />
        <Chat />
        <Users />
    </div>
</main>

<style>
    .main-view {
        display: flex;
        flex-direction: row;
    }
</style>
