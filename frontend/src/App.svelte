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
    <div class="main-view">
            <Hubs />
            <Channels />
            <Chat />
            <Users />
    </div>
    {:else}
    <Login bind:logged_in bind:session_token />
    {/if}
</main>

<style>

    .main-view {
        display: flex;
        flex-direction: row;
    }

    main {
      background-image: url('../../../public/mainbkg.png');
      background-size: cover;
      background-position: center;
      font-family: "Lato", sans-serif;
      font-weight: 400;
      font-style: normal;    
      height: 100vh;
      padding: 10px 0;


  }

</style>
