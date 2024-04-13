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
        <button class="signupbutton" on:click={() => {signup_active = !signup_active}}>
          New user? Create an account
          </button>
      </form>
      {#if failed_login}
      <h2>Failed to login</h2>
      {/if}
    {:else}
      <Signup bind:signup_active />
    {/if}

</main>

<style>    

  main {
      display: flex;
      flex-direction: column;
      align-items: center;
      height: 100vh;
      background-image: url('../../../public/loginbkgb.png');
      background-size: cover;
      background-position: center;
      padding: 20px;
      font-family: "Lato", sans-serif;
      font-weight: 400;
      font-style: normal;    
  }

  form {
      display: flex;
      flex-direction: column;
      align-items: center;
      background-color: rgba(46, 67, 68, 0.4);
      box-shadow: inset 0px -2px 5px rgba(0, 0, 0, 0.3);
      padding: 20px;
      align-content: center;
      justify-content: center;
      border-radius: 10px; /* Adjust corner roundness as needed */
      position: absolute;
      top: 50%;
      left: 50%;
      transform: translate(-50%, -50%); /* Center form relative to viewport */
      padding-left: 50px;
      padding-right: 50px;
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
      color: #EEEEEE;
      border-radius: 10px;
      box-shadow: inset 0px -2px 5px rgba(0, 0, 0, 0.3);
      background-color: rgba(46, 67, 68, 0.4);
      
  }

  button {
      margin-top: 2rem;
      align-items: center;
      text-align: center;
      border-radius: 10px;
      box-shadow: inset 0px -2px 5px rgba(0, 0, 0, 0.3);
      background-color: rgba(46, 67, 68, 0.4);
      color: #EEEEEE;

  }

  .signupbutton {
      margin-top: 2rem;
      align-items: center;
      text-align: center;
      color: #EEEEEE;
      background-color: transparent;
      border-color: transparent;
      box-shadow: none;
  }

  .signupbutton:hover {
      margin-top: 2rem;
      align-items: center;
      text-align: center;
      color: #EEEEEE;
      background-color: transparent;
      border-color: transparent;
      box-shadow: none;
      cursor: pointer;
      transition: background-color 0.3s ease;
      text-decoration: underline;
  }


  @import url('https://fonts.googleapis.com/css2?family=Lato:ital,wght@0,100;0,300;0,400;0,700;0,900;1,100;1,300;1,400;1,700;1,900&display=swap');

</style>
