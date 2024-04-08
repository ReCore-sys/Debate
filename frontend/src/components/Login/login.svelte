<script lang="ts">
    import { invoke } from '@tauri-apps/api/core'
    export let logged_in: bool;
    export let session_token: string;
    $: failed_login = false;
    function login(e) {
    e.preventDefault();
    invoke("login", {username: document.getElementById("username").value, password: document.getElementById("password").value}).
    then((response) => {
        if (response != "") {
            console.log(response);
            logged_in = true;
            session_token = response;
        } else {
            failed_login = true;
        }
    });
    }
</script>

<main>
    <h1>Login</h1>
    <h2>Logged in: {logged_in}</h2>
    <form>
        <label for="username">Username</label>
        <input type="username" id="username" name="username" required>
        <label for="password">Password</label>
        <input type="password" id="password" name="password" required>
        <button on:click={login}>Login</button>
    </form>
    {#if failed_login}
    <h2>Failed to login</h2>
    {/if}
</main>

<style>
    main {
        display: flex;
        flex-direction: column;
        align-items: center;
    }

    form {
        display: flex;
        flex-direction: column;
        align-items: center;
    }

    label {
        margin-top: 1rem;
    }

    input {
        margin-top: 0.5rem;
    }

    button {
        margin-top: 1rem;
    }
</style>