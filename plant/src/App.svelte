<script>
  import { onMount } from "svelte";

  // User pages
  import Home from "./Home.svelte";
  import Cart from "./Cart.svelte";
  import Login from "./Login.svelte";
  import Register from "./Register.svelte";
  import Orders from "./Orders.svelte";

  // Admin pages
  import Admin from "./Admin.svelte";

  let currentPage = "home"; // home, login, register, cart, orders, adminDashboard
  let isLoggedIn = false;
  let isAdminLoggedIn = false;
  let cart = [];
  let userEmail = "";
  let showLogout = false;
  let adminPage = "home";
  // Persist login
  onMount(() => {
    const savedEmail = localStorage.getItem("userEmail");
    if (savedEmail) {
      userEmail = savedEmail;
      isLoggedIn = true;
    }
  });

  // User logout
  function logoutConfirm() {
    isLoggedIn = false;
    cart = [];
    userEmail = "";
    localStorage.removeItem("userEmail");
    currentPage = "home";
    showLogout = false;
    alert("Logged out successfully");
  }

  // Admin logout
  function adminLogout() {
    isAdminLoggedIn = false;
    currentPage = "login"; // redirect to login page
  }

  // Handle navigation for user pages
  function goTo(page) {
    currentPage = page;
  }
</script>

<!-- MAIN NAVBAR -->
<nav class="navbar">
  <div>{isAdminLoggedIn ? "🌱 Admin Panel" : "🌱 Plant Store"}</div>

  <div class="links">
    {#if !isAdminLoggedIn}
      <!-- USER NAV LINKS -->
      <button on:click={() => goTo("home")}>Home</button>

      {#if isLoggedIn}
        <button on:click={() => goTo("cart")}>Cart ({cart.length})</button>
        <button on:click={() => goTo("orders")}>My Orders</button>
        <button class="logout" on:click={() => showLogout = true}>Logout</button>
      {:else}
        <button on:click={() => goTo("login")}>Login</button>
        <button on:click={() => goTo("register")}>Register</button>
      {/if}
    {/if}
  </div>
</nav>

<!-- USER PAGES -->
{#if !isAdminLoggedIn}
  {#if currentPage === "home"}
    <Home {isLoggedIn} bind:cart />
  {:else if currentPage === "cart"}
    <Cart bind:cart {userEmail} />
  {:else if currentPage === "orders"}
    <Orders {userEmail} />
  {:else if currentPage === "login"}
    <Login
      on:success={(event) => {
        const { role, email } = event.detail;

        if (role === "admin") {
          isAdminLoggedIn = true;
          currentPage = "adminDashboard";
        } else if (role === "user") {
          isLoggedIn = true;
          userEmail = email;
          localStorage.setItem("userEmail", userEmail);
          currentPage = "home";
        } else {
          alert("Login failed");
        }
      }}
    />
  {:else if currentPage === "register"}
    <Register on:done={() => goTo("login")} />
  {/if}
{/if}

{#if isAdminLoggedIn && currentPage === "adminDashboard"}
  <Admin
    {adminPage}
    setAdminPage={(page) => adminPage = page}
    {adminLogout}
  />
{/if}

<!-- LOGOUT CONFIRMATION -->
{#if showLogout}
  <div class="overlay">
    <div class="alert-box">
      <p>Are you sure you want to logout?</p>
      <div class="actions">
        <button class="yes" on:click={logoutConfirm}>Yes</button>
        <button class="no" on:click={() => showLogout = false}>No</button>
      </div>
    </div>
  </div>
{/if}

<style>
  .navbar { background: #2f6b2f; color: white; padding: 15px; display: flex; justify-content: space-between; align-items: center; }
  .links button { margin-left: 10px; padding: 8px 12px; border-radius: 5px; border: none; cursor: pointer; }
  .logout { background: #4caf50; color: white; }
  .overlay { position: fixed; inset: 0; background: rgba(0,0,0,0.5); display: flex; justify-content: center; align-items: center; }
  .alert-box { background: white; padding: 20px; border-radius: 8px; text-align:center; }
  .alert-box .yes { background: #4caf50; color:white; padding:8px 16px; border:none; border-radius:5px; cursor:pointer; font-weight:bold; margin-right:10px; }
  .alert-box .no { background: #f44336; color:white; padding:8px 16px; border:none; border-radius:5px; cursor:pointer; font-weight:bold; }
</style>
