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
    <div class="users rounded-br-x1 isolate bg-white/5 shadow-lg ring-1 ring-black/5 bg-clip-padding backdrop-filter backdrop-blur-md">
        <div class=" bg-white/5 shadow-lg ring-1 ring-black/5 bg-clip-padding backdrop-filter backdrop-blur-md">
            {#each users as user}
                <div class="user-button">
                    <h2>{user.username}</h2>
                </div>
            {/each}
        </div>
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

