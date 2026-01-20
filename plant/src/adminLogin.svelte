<script>
  import { createEventDispatcher } from "svelte";
  const dispatch = createEventDispatcher();

  let username = "";
  let password = "";
  let error = "";

  function login() {
   
    if (username === "admin" && password === "admin123") {
      dispatch("success");
    } else {
      error = "Invalid credentials";
    }
  }
</script>

<main>
  <h2>Admin Login</h2>

  <form on:submit|preventDefault={login}>
    <input placeholder="Username" bind:value={username} required />
    <input type="password" placeholder="Password" bind:value={password} required />
    <button type="submit">Login</button>
  </form>

  {#if error}
    <p style="color:red">{error}</p>
  {/if}
</main>

<style>
main {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 100vh;
  background: #e8f5e8;
}

form {
  display: flex;
  flex-direction: column;
  gap: 10px;
  padding: 20px;
  border: 1px solid #ccc;
  border-radius: 5px;
  background: #fff;
}

input {
  padding: 10px;
  border: 1px solid #ccc;
  border-radius: 5px;
  width: 250px;
}

button {
  padding: 10px;
  border: none;
  border-radius: 5px;
  background: #2f6b2f;
  color: #fff;
  cursor: pointer;
}

button:hover {
  background: #27632a;
}
</style>
