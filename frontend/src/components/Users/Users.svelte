<script lang="ts">
    let users = [
        {
            id: 1,
            username: "Kai",
        },
        {
            id: 2,
            username: "Skye",
        },
    ];
</script>

<main>
    <div class="users">
        {#each users as user}
            <div class="user-button">
                <h2>{user.username}</h2>
            </div>
        {/each}
    </div>
</main>

<style>
    .users {
        width: 10vw;
        background-color: transparent;
        display: flex;
        flex-direction: column;

    }

    .user-button {
    display: flex;
    align-items: center;
    justify-content: center;
    border: none;
    cursor: pointer;
    transition: background-color 0.2s ease;
  }

  .user-button h2 {
  color: #e6e9f2;
  }
</style>
