<script lang="ts">
    import MessageBar from "./MessageBar.svelte";

    $: messages = [
        {
            id: 1,
            content: "Hello",
            from: "user1",
        },
        {
            id: 2,
            content: "Hi",
            from: "user2",
        },
    ];
</script>

<main>
    <div class="chat">
        {#each messages as message}
            <div class="message">
                <h2>{message.from}</h2>
                <p>{message.content}</p>
            </div>
        {/each}
    </div>
    <MessageBar bind:messages />
</main>

<style>
    h2 {
        font-weight: bold;
    }
    .chat {
        width: 70vw;
    }
</style>
