<script lang="ts">
    import { invoke } from '@tauri-apps/api/core'
    import Signup from '../Signup/Signup.svelte';
    export let logged_in: boolean;
    export let session_token: string;
    
    $: failed_login = false;
    $: signup_active = false;

    function login() {
      invoke("login", {
        username: document.getElementById("username")?.nodeValue, 
        password: document.getElementById("password")?.nodeValue
      }).
      then((response) => {
        if (response != "") {
            console.log(response);
            logged_in = true;
            session_token = response as string;
        } else {
            failed_login = true;
        }
      });
    }
</script>

<main>
    {#if !signup_active}
      <form>
        <label for="username">Username</label>
        <input type="username" id="username" name="username" required>
        <label for="password">Password</label>
        <input type="password" id="password" name="password" required>
        <button on:click|preventDefault={login}>Login</button>
      </form>
      {#if failed_login}
      <h2>Failed to login</h2>
      {/if}
    {:else}
      <Signup bind:signup_active />
    {/if}
    <button on:click={() => {signup_active = !signup_active}}>
    Sign up
    </button>

</main>

<style>
    
    main {
        display: flex;
        flex-direction: column;
        align-items: center;
        height: 100vh;
        background-image: url('../../../public/bkg.png');
        background-size: cover;
        background-position: center;
        border-radius: 10px; /* Rounded corners for the glass pane */
        padding: 20px;
    
    }

    form {
        display: flex;
        flex-direction: column;
        align-items: center;
        background-color: rgba(46, 67, 68, 0.4); /* Semi-transparent white */
        box-shadow: inset 0px -2px 5px rgba(0, 0, 0, 1.5); /* Adjust as needed */
        padding: 20px;
        align-content: center;
        justify-content: center;
        border-radius: 10px; /* Adjust corner roundness as needed */
        position: absolute;
        top: 50%;
        left: 50%;
        transform: translate(-50%, -50%); /* Center form relative to viewport */
        padding-left: 100px;
        padding-right: 100px;
    }

    label {
        margin-top: 1rem;
        margin-bottom: 10px; /* Spacing between form elements */
        display: block;
        align-items: center;
        text-align: center;
        width: 100%;
        color: #EEEEEE;
    }

    input {
        margin-top: 0.5rem;
        margin-bottom: 10px; /* Spacing between form elements */
        display: block;
        align-items: center;
        text-align: center;
        width: 100%;
        color: #222831;
    }

    button {
        margin-top: 2rem;
        align-items: center;
        text-align: center;

    }

</style>
<!--

    .login-container {
  /* Adjust background image/video URL */
  background-image: url("path/to/your/background.jpg");
  background-size: cover;
  background-position: center;
  backdrop-filter: blur(10px); /* Adjust blur intensity */
  border-radius: 10px; /* Rounded corners for the glass pane */
  padding: 20px;
  display: flex;
  flex-direction: column;
  align-items: center;
}

.login-form {
  background-color: rgba(255, 255, 255, 0.8); /* Semi-transparent white */
  padding: 20px;
}

.login-form label,
.login-form input,
.login-form button {
  margin-bottom: 10px; /* Spacing between form elements */
  display: block;
  width: 100%;
}

.login-form button {
  background-color: #007bff; /* Primary accent color */
  color: #fff; /* White text color */
  border: none;
  padding: 10px 15px;
  border-radius: 5px;
  cursor: pointer;
}

/* Optional: subtle border for the login form */
.login-form {
  border: 1px solid rgba(255, 255, 255, 0.5);
}


-->
