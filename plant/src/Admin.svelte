<script>
  import AddProduct from "./AddProduct.svelte";
  import AdminOrders from "./AdminOrders.svelte";

  export let adminPage;
  export let setAdminPage;
  export let adminLogout;

  let showLogoutConfirm = false;

  const openLogout = () => showLogoutConfirm = true;
  const cancelLogout = () => showLogoutConfirm = false;

  const confirmLogout = () => {
    showLogoutConfirm = false;
    adminLogout();
  };
</script>

<!-- 🔹 NAVBAR -->
<nav class="navbar">
  <div class="left">
    <button on:click={() => setAdminPage("home")}>Home</button>
    <button on:click={() => setAdminPage("add")}>Add Product</button>
    <button on:click={() => setAdminPage("orders")}>View Orders</button>
  </div>

  <button class="logout" on:click={openLogout}>Logout</button>
</nav>

<!-- 🔹 CONTENT -->
{#if adminPage === "home"}
  <h2 class="padding">Welcome Plant Store 🌱</h2>
{:else if adminPage === "add"}
  <AddProduct />
{:else if adminPage === "orders"}
  <AdminOrders />
{/if}

<!-- 🔹 LOGOUT ALERT BOX -->
{#if showLogoutConfirm}
  <div class="overlay">
    <div class="modal">
      <p class="title">Are you sure you want to logout?</p>

      <div class="actions">
        <button class="yes" on:click={confirmLogout}>Yes</button>
        <button class="no" on:click={cancelLogout}>No</button>
      </div>
    </div>
  </div>
{/if}

<style>

.navbar {
  background: #2e7d32;
  padding: 15px 20px;
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.left button {
  margin-right: 10px;
}

button {
  padding: 8px 16px;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  font-weight: 600;
}

.logout {
  background: #2e7d32;
  color: white;
}

.padding {
  text-align: center;
  margin-top: 40px;
  color: #2e7d32;
}


.overlay {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.55);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 999;
}


.modal {
  background: white;
  padding: 30px;
  width: 340px;
  border-radius: 10px;
  text-align: center;
  box-shadow: 0 8px 25px rgba(0,0,0,0.3);
}

.title {
  font-size: 18px;
  margin-bottom: 25px;
  color: #333;
}

.actions {
  display: flex;
  justify-content: center;
  gap: 20px;
}

.yes {
  background: #4caf50;
  color: white;
  width: 80px;
}

.no {
  background: #f44336;
  color: white;
  width: 80px;
}
</style>
