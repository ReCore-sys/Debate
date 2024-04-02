<script lang="ts">
    let channels = [
        {
            id: 1,
            name: "Chat1",
            description: "General discussion",
            created_at: new Date(),
            updated_at: new Date(),
        },
        {
            id: 2,
            name: "Chat2",
            description: "Random discussion",
            created_at: new Date(),
            updated_at: new Date(),
        },
    ];
</script>

<main>
    <div class="channels">
        {#each channels as channel}
            <div class="hub">
                <h2>{channel.name}</h2>
            </div>
        {/each}
    </div>
</main>

<style>
    .channels {
        width: 10vw;
    }
</style>
