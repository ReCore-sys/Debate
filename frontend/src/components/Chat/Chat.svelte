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
        color: #76ABAE
    }
    p {
        color: #EEEEEE
    }
    .chat {
        width: 70vw;
        background-color: #222831;
        border-radius: 5% 5% 0 0;
        padding: 10px;

    }
</style>
