<script lang="ts">
    let channels = [
        {
            id: 1,
            name: "dev-stuff",
            description: "General discussion",
            created_at: new Date(),
            updated_at: new Date(),
        },
        {
            id: 2,
            name: "images",
            description: "Random discussion",
            created_at: new Date(),
            updated_at: new Date(),
        },
        {
            id: 3,
            name: "code",
            description: "General discussion",
            created_at: new Date(),
            updated_at: new Date(),
        },
        {
            id: 4,
            name: "projects",
            description: "Random discussion",
            created_at: new Date(),
            updated_at: new Date(),
        },
    ];
</script>

<main>
    <div class="channels">
        {#each channels as channel}
            <div class="channel-button">
                <h2>{channel.name}</h2>
            </div>
        {/each}
    </div>
</main>

<style>
    .channels {
    width: 10vw;
    display: flex;
    flex-direction: column;
    background-color: transparent;
    text-align: center;
    align-items: center;
    }

    .channel-button {
    display: flex;
    flex-direction: column; /* Arrange vertically */
    align-items: center;  /* Center horizontally */
    justify-content: center; /* Center vertically */
    border: none;
    cursor: pointer;
    transition: background-color 0.2s ease;
  }

  .channel-button h2 {
  color: #e6e9f2;
  }

</style>
