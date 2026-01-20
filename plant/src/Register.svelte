<script>
  import { createEventDispatcher } from "svelte";
  const dispatch = createEventDispatcher();

  let name = "";
  let email = "";
  let password = "";
  let error = "";

  async function register() {
    try {
      const res = await fetch("http://localhost:3000/register", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ name, email, password })
      });

      if (res.ok) {
        alert("Registered successfully!");
        dispatch("done"); 
      } else {
        const err = await res.text();
        error = err || "Registration failed";
      }
    } catch (e) {
      error = e.message;
    }
  }
</script>

<main>
  <h2>Register</h2>

  <form on:submit|preventDefault={register}>
    <input placeholder="Name" bind:value={name} required />
    <input placeholder="Email" bind:value={email} required />
    <input type="password" placeholder="Password" bind:value={password} required />
    <button type="submit">Register</button>
  </form>

  {#if error}
    <p style="color:red; margin-top:10px">{error}</p>
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
  background: #4CAF50;
  color: #fff;
  cursor: pointer;
}

button:hover {
  background: #45a049;
}
</style>
