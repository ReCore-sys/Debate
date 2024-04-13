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
        <input type="text" placeholder="Type a message..." />
        <button on:click={sendMessage}>Send</button>
    </div>
</main>
<style>
  /*#222831 #31363F #76ABAE #EEEEEE*/
  /*greys getting lighter #3a3d43;
    greys with high contrast: #e0e0e0 and #f2f2f2
    grey with a blueish tint to match nordic theme: #e6e9f2*/

    .container {
     margin: 0px; 
       
    }
    .message-bar {
        background-color: transparent;
        border-radius: 0 0 5% 5%;
        margin-bottom: 10px;
        position:fixed;
        bottom:0;
        width: 70vw;
        padding: 10px;
        align-items: center;        
    }

    .message-bar button {
        background-color: transparent;
        color: #e6e9f2;
        width: 10vw;
        background-color: rgba(46, 67, 68, 0.4);
        box-shadow: inset 0px -2px 5px rgba(0, 0, 0, 0.3);
    }

    .message-bar input {
        background-color: transparent;
        color: #e6e9f2;
        width: 56vw;
        background-color: rgba(46, 67, 68, 0.4);
        box-shadow: inset 0px -2px 5px rgba(0, 0, 0, 0.3);
    }
</style>