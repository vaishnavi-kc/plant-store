<script>
  import { createEventDispatcher } from "svelte";

  const dispatch = createEventDispatcher();

  let email = "";
  let password = "";
  let error = "";

  async function login() {
    error = "";

    // 🔹 Admin check first (hardcoded)
    if (email === "admin@plantstore.com" && password === "admin123") {
      dispatch("success", { role: "admin", email });
      return;
    }

    // 🔹 Normal user login
    try {
      const res = await fetch("http://localhost:3000/login", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ email, password })
      });

      const data = await res.json();

      if (!res.ok) {
        error = data.message; // ✅ read message from JSON
        return;
      }

      dispatch("success", { role: "user", email: data.user.email });
    } catch (e) {
      error = e.message;
    }
  }
</script>

<main>
  <h2>Login</h2>

  <input placeholder="Email" bind:value={email} />
  <input type="password" placeholder="Password" bind:value={password} />

  <button on:click={login}>Login</button>

  {#if error}
    <p style="color:red">{error}</p>
  {/if}
</main>

<style>
main {
  display:flex;
  flex-direction:column;
  align-items:center;
  justify-content:center;
  height:100vh;
  background:#e8f5e8;
}

input {
  padding:10px;
  border:1px solid #ccc;
  border-radius:5px;
  width:250px;
  margin-bottom:10px;
}

button {
  padding: 6px 12px; 
  width: 120px;      
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
