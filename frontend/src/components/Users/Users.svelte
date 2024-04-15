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
        background-color: transparent;
        display: flex;
        flex-direction: column;
        position: fixed;
        right: 10px;
        top: 10px;
        bottom: 10px;  
        width: 10%;
        align-items: center;
        background-color: rgba(46, 67, 68, 0.4);
        box-shadow: inset 0px -2px 5px rgba(0, 0, 0, 0.3);
        padding: 20px;
        border-radius: 10px;
        overflow-y: auto;
    }

    .user-button {
    display: flex;
    align-items: center;
    justify-content: center;
    border: none;
    cursor: pointer;
    transition: 0.2s ease;
  }

  .user-button h2 {
  color: #e6e9f2;
  }
</style>

