<script lang="ts">
    let users = [
        {
            id: 1,
            username: "User1",
        },
        {
            id: 2,
            username: "User2",
        },
    ];
</script>

<main>
    <div class="users">
        {#each users as user}
            <div class="user">
                <h2>{user.username}</h2>
            </div>
        {/each}
    </div>
</main>

<style>
    .users {
        width: 10vw;
    }
</style>
