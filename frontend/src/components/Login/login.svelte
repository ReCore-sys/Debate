<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import Signup from "../Signup/Signup.svelte";
  export let logged_in: boolean;
  export let session_token: string;

  $: failed_login = false;
  $: signup_active = false;

  function login(e) {
    let formdata = new FormData(e.target);

    let data: { [key: string]: string } = {};

    for (let field of formdata) {
      const [key, value] = field;
      data[key] = value.toString();
    }
    invoke("hash", { data: data["password"] }).then((digest) => {
      invoke("login", {
        email: data["email"],
        password: digest,
      }).then((response) => {
        if (response != "") {
          console.log(response);
          logged_in = true;
          session_token = response as string;
        } else {
          failed_login = true;
        }
      });
    });
  }
</script>

<main>
  {#if !signup_active}
    <div
      class="isolate aspect-video w-96 rounded-xl bg-white/5 shadow-lg ring-1 ring-black/5 bg-clip-padding backdrop-filter backdrop-blur-md"
    >
      <form on:submit|preventDefault={login}>
        <label for="email">Email</label>
        <input
          class="bg-white/5 shadow-lg ring-1 ring-black/5 bg-clip-padding backdrop-filter backdrop-blur-md"
          type="email"
          id="email"
          name="email"
          required
        />
        <label for="password">Password</label>
        <input
          class="bg-white/5 shadow-lg ring-1 ring-black/5 bg-clip-padding backdrop-filter backdrop-blur-md"
          type="password"
          id="password"
          name="password"
          required
        />
        <button
          class="loginbutton bg-white/0 shadow-lg ring-1 ring-black/5 bg-clip-padding backdrop-filter backdrop-blur-md"
          type="submit">Login</button
        >
        <button
          class="signupbutton bg-transparent shadow-none"
          on:click={() => {
            signup_active = !signup_active;
          }}
        >
          New user? Create an account
        </button>
      </form>
    </div>
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
    align-content: center;
    justify-content: center;
    height: 100%;
    background-size: cover;
    background-position: center;
    padding: 20px;
  }

  form {
    display: flex;
    flex-direction: column;
    align-items: center;
    padding: 20px;
    align-content: center;
    justify-content: center;
  }

  input,
  label {
    margin-bottom: 10px;
    display: block;
    align-items: center;
    text-align: center;
    width: 100%;
    color: #eeeeee;
    margin-bottom: 15px;
  }

  input:-webkit-autofill,
  input:-webkit-autofill:hover,
  input:-webkit-autofill:focus,
  input:-webkit-autofill:active {
    transition: background-color 5000s;
    -webkit-text-fill-color: #fff !important;
  }

  .loginbutton {
    color: #eeeeee;
    margin-top: 10px;
    margin-bottom: 10px;
  }

  .signupbutton {
    color: #eeeeee;
  }

  .signupbutton:hover {
    color: #eeeeee;
    background-color: transparent;
    border-color: transparent;
    box-shadow: none;
    cursor: pointer;
    transition: background-color 0.3s ease;
    text-decoration: underline;
  }
  /* Old CSS without tailwind
  @import url('https://fonts.googleapis.com/css2?family=Lato:ital,wght@0,100;0,300;0,400;0,700;0,900;1,100;1,300;1,400;1,700;1,900&display=swap');
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
      border-radius: 10px;
      position: absolute;
      top: 50%;
      left: 50%;
      transform: translate(-50%, -50%);
      padding-left: 50px;
      padding-right: 50px;
  }

  label {
      margin-top: 1rem;
      margin-bottom: 10px;
      display: block;
      align-items: center;
      text-align: center;
      width: 100%;
      color: #EEEEEE;
      
  }

  input {
      margin-top: 0.5rem;
      margin-bottom: 10px;
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

input:-webkit-autofill,
input:-webkit-autofill:hover, 
input:-webkit-autofill:focus, 
input:-webkit-autofill:active  {
  transition: background-color 5000s;
  -webkit-text-fill-color: #fff !important;
}

*/
</style>
