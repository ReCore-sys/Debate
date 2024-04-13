<script lang="ts">
  import type {User} from "../../types"
  import { invoke } from "@tauri-apps/api/core";
  export let signup_active: boolean;
  $: failed_signup = false;

  function signup(e)
  {
    let formdata = new FormData(e.target)
    
    let data: {[key: string]: string} = {}

    for (let field of formdata) {
      const [key, value] = field
      data[key] = value.toString()
    }

    let new_user: User = {
      username: data["username"],
      password: data["password"],
      email: data["email"],
      uuid: "",
      pfp: "",
      bio: ""
    }
    invoke("signup", {user: new_user}).then((response) => {
      if (response == true) {
        signup_active = true
      } else {
        failed_signup = true;
      }
    }
    )
  }

</script>

<main class="overflow-hidden">
  <!--<h1>Signup</h1>-->
       
      <form on:submit|preventDefault={signup} class="
      flex-col items-center flex h-fit w-fit bg-grey-400 rounded-md 
      bg-clip-padding backdrop-filter backdrop-blur-md bg-opacity-10 
      border border-gray-100 my-12 space-y-4 p-8
    ">
        <label for="username">Username</label>
        <input type="username" id="username" name="username" required>
        <label for="password">Password</label>
        <input type="password" id="password" name="password" required>
        <label for="email">Email</label>
        <input type="email" id="email" name="email" required>
        {#if failed_signup}
          <div class="
          h-10 w-full bg-red-400 rounded-md 
          bg-clip-padding backdrop-filter backdrop-blur-md bg-opacity-10 border border-gray-100
          flex justify-center items-center
          ">
            <h1>Signup Failed</h1>
          </div>
        {/if}
        <button type="submit">Signup</button>
      </form>
</main>

<style>    

main {
    background-image: url('../../../public/signupbkg.png');
    background-size: cover;
    background-position: center;
}

label {
    display: block;
    align-items: center;
    text-align: center;
    width: 100%;
    color: #EEEEEE;
    
}

input {
    display: block;
    align-items: center;
    text-align: center;
    width: 100%;
    color: #EEEEEE;
    border-radius: 10px;
    box-shadow: inset 0px -2px 5px rgba(0, 0, 0, 0.3);
    background-color: rgba(46, 67, 68, 0.4);
    
}

input:-webkit-autofill,
input:-webkit-autofill:hover, 
input:-webkit-autofill:focus, 
input:-webkit-autofill:active  {
  transition: background-color 5000s;
  -webkit-text-fill-color: #fff !important;
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

</style>
