<script lang="ts">
  import type { User } from "../../types";
  import { invoke } from "@tauri-apps/api/core";
  export let signup_active: boolean;
  $: failed_signup = false;

  function signup(e) {
    let formdata = new FormData(e.target);

    let data: { [key: string]: string } = {};

    for (let field of formdata) {
      const [key, value] = field;
      data[key] = value.toString();
    }
    invoke("hash", { data: data["password"] }).then((digest) => {
      let new_user: User = {
        username: data["username"],
        password: digest as string,
        email: data["email"],
        uuid: "",
        pfp: "",
        bio: "",
      };
      invoke("signup", { user: new_user }).then((response) => {
        if (response == true) {
          signup_active = true;
        } else {
          failed_signup = true;
        }
      });
    });
  }
</script>

<main class="overflow-hidden">
  <div>
    <!--<h1>Signup</h1>-->
    <div
      class="isolate aspect-video w-96 rounded-xl bg-white/5 shadow-lg ring-1 ring-black/5 bg-clip-padding backdrop-filter backdrop-blur-md"
    >
      <form on:submit|preventDefault={signup}>
        <label for="username">Username</label>
        <input
          class="bg-white/5 shadow-lg ring-1 ring-black/5 bg-clip-padding backdrop-filter backdrop-blur-md"
          type="username"
          id="username"
          name="username"
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
        <label for="email">Email</label>
        <input
          class="bg-white/5 shadow-lg ring-1 ring-black/5 bg-clip-padding backdrop-filter backdrop-blur-md"
          type="email"
          id="email"
          name="email"
          required
        />
        {#if failed_signup}
          <div
            class="
          h-10 w-full bg-red-400 rounded-md
          bg-clip-padding backdrop-filter backdrop-blur-md bg-opacity-10 border border-gray-100
          flex justify-center items-center
          "
          >
            <h1>Signup Failed</h1>
          </div>
        {/if}
        <button
          class="button bg-white/0 shadow-lg ring-1 ring-black/5 bg-clip-padding backdrop-filter backdrop-blur-md"
          type="submit">Signup</button
        >
      </form>
    </div>
  </div>
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

  .button {
    color: #eeeeee;
    margin-top: 10px;
    margin-bottom: 10px;
  }
</style>
