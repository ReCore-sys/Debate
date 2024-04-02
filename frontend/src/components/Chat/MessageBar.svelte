<script lang="ts">
    import type { Message } from "../../types";

    export let messages: Message[];

    function sendMessage() {
        messages = [
            ...messages,
            {
                id: messages.length + 1,
                content: "New message",
                from: "user1",
            },
        ];
    }
</script>

<main>
    <div class="message-bar">
        <input type="text" placeholder="Type a message..." />
        <button on:click={sendMessage}>Send</button>
    </div>
</main>
