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
    
    <div class="chat bg-white/5 shadow-lg ring-1 ring-black/5 bg-clip-padding backdrop-filter backdrop-blur-md">
        {#each messages as message}
            <div class="message rounded-xl bg-white/5 shadow-lg ring-1 ring-black/5 bg-clip-padding backdrop-filter backdrop-blur-md">
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
        width: 66%;
        background-color: transparent;
        padding: 10px;
        position:fixed;
        bottom:10px;
        top: 0;
        padding-bottom:10%;
        overflow-y: scroll;
        overflow: hidden;
    }

    .message {
        overflow:   auto;
        bottom:     0; 
        margin-top: 0.5rem;
        margin-bottom: 10px; 
        color: #EEEEEE;
        padding: 10px;
    }
    /*
    .message
    {
        overflow:   auto;
        bottom:     0; 
        margin-top: 0.5rem;
        margin-bottom: 10px; /* Spacing between form elements 
        display: block;
        width: 100%;
        color: #EEEEEE;
        border-radius: 10px;
        box-shadow: inset 0px -2px 5px rgba(0, 0, 0, 0.3);
        background-color: rgba(46, 67, 68, 0.4);
        padding: 10px;
      
    }
*/
    
</style>


<!--
#chatbox
{
    overflow:   none;
    position:   relative;
    width:      100%;
    height:     400px;
}
#chatmessages
{
    overflow:   auto;
    position:   absolute;
    bottom:     0;
    max-height: 400px;
} -->