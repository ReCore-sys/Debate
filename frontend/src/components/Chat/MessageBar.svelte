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
    <!--<div class="overflow-y-scroll ..."></div>-->
    <div class="message-bar">
        <input class="bg-white/5 shadow-lg ring-1 ring-black/5 bg-clip-padding backdrop-filter backdrop-blur-md"type="text" placeholder="Type a message..." />
        <button class="bg-white/5 shadow-lg ring-1 ring-black/5 bg-clip-padding backdrop-filter backdrop-blur-md"on:click={sendMessage}>Send</button>
    </div>
</main>
<style>
  /*#222831 #31363F #76ABAE #EEEEEE*/
  /*greys getting lighter #3a3d43;
    greys with high contrast: #e0e0e0 and #f2f2f2
    grey with a blueish tint to match nordic theme: #e6e9f2*/

    .message-bar {
        background-color: transparent;
        border-radius: 0 0 5% 5%;
        margin-bottom: 10px;
        position:fixed;
        bottom:0;
        width: 70%;
        padding: 10px;
        align-items: center;        
    }

    .message-bar button {
        background-color: transparent;
        color: #e6e9f2;
        width: 10%;
    }

    .message-bar input {
        background-color: transparent;
        color: #e6e9f2;
        width: 56%;
    }
</style>
