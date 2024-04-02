<script lang="ts">
    let hubs = [
        {
            id: 1,
            name: "General",
            description: "General discussion",
            created_at: new Date(),
            updated_at: new Date(),
        },
        {
            id: 2,
            name: "Random",
            description: "Random discussion",
            created_at: new Date(),
            updated_at: new Date(),
        },
    ];
</script>

<main>
    <div class="hubs">
        {#each hubs as hub}
            <div class="hub">
                <h2>{hub.name}</h2>
            </div>
        {/each}
    </div>
</main>

<style>
    .hubs {
        width: 10vw;
    }
</style>
