<script>
  import { onMount } from "svelte";

  let products = [];
  let filteredProducts = [];
  let search = "";

  // PRICE FILTER
  let minPrice = "";
  let maxPrice = "";

  // Form state
  let showForm = false;
  let name = "";
  let price = "";
  let category = "Indoor";
  let image = "";
  let editingId = null;

  async function load() {
    const res = await fetch("http://localhost:3000/products");
    products = await res.json();
    filterProducts();
  }

  function filterProducts() {
    const q = search.toLowerCase();

    filteredProducts = products.filter((p) => {
      const nameMatch = p.name.toLowerCase().includes(q);
      const categoryMatch =
        p.category && p.category.toLowerCase().includes(q);

      const priceValue = Number(p.price);
      const minMatch = minPrice === "" || priceValue >= Number(minPrice);
      const maxMatch = maxPrice === "" || priceValue <= Number(maxPrice);

      return (nameMatch || categoryMatch) && minMatch && maxMatch;
    });
  }

  function resetFilters() {
    search = "";
    minPrice = "";
    maxPrice = "";
    filterProducts();
  }

  async function saveProduct() {
    const payload = { name, price: Number(price), image, category };

    if (editingId) {
      await fetch(`http://localhost:3000/products/${editingId}`, {
        method: "PUT",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(payload),
      });
    } else {
      await fetch("http://localhost:3000/products", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(payload),
      });
    }

    reset();
    load();
    showForm = false;
  }

  function edit(p) {
    editingId = p.id;
    name = p.name;
    price = p.price;
    image = p.image;
    category = p.category || "Indoor";
    showForm = true;
  }

  async function remove(id) {
    if (!confirm("Delete product?")) return;
    await fetch(`http://localhost:3000/products/${id}`, { method: "DELETE" });
    load();
  }

  function reset() {
    editingId = null;
    name = "";
    price = "";
    image = "";
    category = "Indoor";
  }

  onMount(load);
</script>

<h2>Admin – Product Management</h2>

<!-- TOP BAR -->
<div class="top-bar">
  <input
    placeholder="Search by name or category"
    bind:value={search}
    on:input={filterProducts}
  />

  <input
    type="number"
    placeholder="Min ₹"
    bind:value={minPrice}
    on:input={filterProducts}
  />

  <input
    type="number"
    placeholder="Max ₹"
    bind:value={maxPrice}
    on:input={filterProducts}
  />

  <button class="reset-btn" on:click={resetFilters}>Reset</button>

  <button class="add-btn" on:click={() => { reset(); showForm = true; }}>
    Add Product
  </button>
</div>

<!-- MODAL -->
{#if showForm}
  <div class="modal-backdrop" on:click={() => (showForm = false)}></div>
  <div class="modal">
    <h3>{editingId ? "Edit Product" : "Add Product"}</h3>

    <input placeholder="Name" bind:value={name} />
    <input placeholder="Price" type="number" bind:value={price} />

    <select bind:value={category}>
      <option>Indoor</option>
      <option>Outdoor</option>
      <option>Flowering</option>
    </select>

    <input placeholder="Image URL" bind:value={image} />

    <div class="modal-buttons">
      <button on:click={saveProduct}>
        {editingId ? "Update Product" : "Add Product"}
      </button>
      <button class="cancel" on:click={() => (showForm = false)}>Cancel</button>
    </div>
  </div>
{/if}

<hr />

<!-- TABLE -->
<table>
  <tr>
    <th>Image</th>
    <th>Name</th>
    <th>Price</th>
    <th>Category</th>
    <th>Action</th>
  </tr>

  {#each filteredProducts as p}
    <tr>
      <td><img src={p.image} width="60" /></td>
      <td>{p.name}</td>
      <td>₹{p.price}</td>
      <td>{p.category}</td>
      <td>
        <button on:click={() => edit(p)}>Edit</button>
        <button on:click={() => remove(p.id)}>Delete</button>
      </td>
    </tr>
  {/each}
</table>

<style>
  h2 {
    color: #2e7d32;
    text-align: center;
  }

  .top-bar {
    display: flex;
    flex-wrap: wrap;
    gap: 8px;
    margin-bottom: 15px;
  }

  input {
    padding: 8px;
    border-radius: 5px;
    border: 1px solid #ccc;
  }

  button {
    padding: 8px 12px;
    border-radius: 5px;
    border: none;
    background: #4caf50;
    color: white;
    cursor: pointer;
  }

  .add-btn {
    background-color: #1976d2;
  }

  .reset-btn {
    background: #757575;
  }

  table {
    width: 100%;
    border-collapse: collapse;
    margin-top: 20px;
  }

  th, td {
    border: 1px solid #ddd;
    padding: 8px;
    text-align: center;
  }

  th {
    background-color: #f2f2f2;
  }

  .modal-backdrop {
    position: fixed;
    inset: 0;
    background: rgba(0,0,0,0.4);
  }

  .modal {
    position: fixed;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    background: white;
    padding: 20px;
    border-radius: 8px;
    width: 300px;
  }

  .modal-buttons {
    display: flex;
    justify-content: space-between;
    margin-top: 10px;
  }

  .cancel {
    background: #f44336;
  }
</style>
