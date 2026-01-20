<script>
  import { onMount } from "svelte";

  export let cart = [];
  export let isLoggedIn = false;

  let plants = [];
  let loading = true;

  // 🔹 Price filter
  let minPrice = 0;
  let maxPrice = 100000;

  // 🔹 Category filter
  let selectedCategory = "all";
  const categories = ["all", "indoor", "outdoor", "seed", "flowering"];

  onMount(async () => {
    try {
      const res = await fetch("http://localhost:3000/products");
      plants = await res.json();
    } catch (err) {
      console.error("Failed to load products", err);
    } finally {
      loading = false;
    }
  });

  // 🔹 Reactive filteredPlants
  $: filteredPlants = plants.filter(p => {
    const priceMatch = p.price >= minPrice && p.price <= maxPrice;
    const categoryMatch =
      selectedCategory === "all" || (p.category && p.category.toLowerCase() === selectedCategory.toLowerCase());
    return priceMatch && categoryMatch;
  });

  function addToCart(plant) {
    if (!isLoggedIn) {
      alert("Please login to add items 🔐");
      return;
    }

    const item = cart.find(p => p.id === plant.id);

    if (item) {
      item.qty += 1;
      cart = [...cart];
    } else {
      cart = [...cart, { ...plant, qty: 1 }];
    }

    alert(`${plant.name} added to cart 🌱`);
  }
</script>

<h1>Welcome to Plant Store 🌱</h1>

<!-- 🔹 FILTER SECTION -->
<div class="filter">
  <select bind:value={selectedCategory}>
    {#each categories as c}
      <option value={c}>{c.charAt(0).toUpperCase() + c.slice(1)}</option>
    {/each}
  </select>

  <input
    type="number"
    placeholder="Min Price"
    bind:value={minPrice}
  />

  <input
    type="number"
    placeholder="Max Price"
    bind:value={maxPrice}
  />
</div>

{#if loading}
  <p class="loading">Loading products...</p>
{:else if filteredPlants.length === 0}
  <p class="empty">No products found</p>
{:else}
  <div class="grid">
    {#each filteredPlants as plant}
      <div class="card">
        <img
          src={plant.image}
          alt={plant.name}
          on:error={(e) => (e.target.src = "/placeholder.png")}
        />

        <h3>{plant.name}</h3>
        <p>₹{plant.price}</p>
        <small class="cat">{plant.category}</small>

        <button on:click={() => addToCart(plant)}>
          Add to Cart
        </button>
      </div>
    {/each}
  </div>
{/if}

<style>
  h1 {
    text-align: center;
    margin-top: 20px;
    color: #2e7d32;
  }

  /* 🔹 Filter */
  .filter {
    display: flex;
    justify-content: center;
    gap: 12px;
    margin: 15px 0;
    flex-wrap: wrap;
  }

  .filter input,
  .filter select {
    padding: 8px;
    width: 140px;
    border-radius: 6px;
    border: 1px solid #ccc;
  }

  .loading,
  .empty {
    text-align: center;
    margin-top: 40px;
    font-size: 18px;
    color: #666;
  }

  .grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(230px, 1fr));
    gap: 20px;
    padding: 20px;
  }

  .card {
    background: white;
    padding: 15px;
    border-radius: 12px;
    text-align: center;
    box-shadow: 0 4px 10px rgba(0, 0, 0, 0.12);
    transition: transform 0.25s ease;
  }

  .card:hover {
    transform: translateY(-6px);
  }

  .card img {
    width: 100%;
    height: 200px;
    object-fit: contain;
    background: #f5f5f5;
    border-radius: 8px;
    padding: 10px;
  }

  h3 {
    margin: 8px 0 4px;
    font-size: 1.2em;
    color: #333;
  }

  p {
    margin: 5px 0;
    font-weight: bold;
    color: #555;
  }

  .cat {
    display: block;
    margin-bottom: 10px;
    text-transform: capitalize;
    color: #2e7d32;
    font-weight: bold;
  }

  button {
    width: 100%;
    padding: 10px;
    border: none;
    border-radius: 6px;
    background: #4caf50;
    color: white;
    font-weight: bold;
    cursor: pointer;
  }

  button:hover {
    background: #43a047;
  }
  /* Filter container card style */
.filter {
  display: flex;
  justify-content: center;
  gap: 12px;
  margin: 20px auto;
  padding: 15px 20px;
  max-width: 600px;
  background: #f0fdf4; /* soft greenish background */
  border-radius: 12px;
  box-shadow: 0 4px 15px rgba(0, 0, 0, 0.1);
  flex-wrap: wrap;
  align-items: center;
}

/* Inputs and select styling */
.filter input,
.filter select {
  padding: 10px 12px;
  width: 140px;
  border-radius: 8px;
  border: 1px solid #c1e1c1;
  outline: none;
  font-size: 14px;
  transition: 0.2s all;
}

.filter input:focus,
.filter select:focus {
  border-color: #4caf50;
  box-shadow: 0 0 5px rgba(76, 175, 80, 0.4);
}

/* Placeholder styling */
.filter input::placeholder {
  color: #888;
  font-size: 13px;
  font-style: italic;
}

/* Dropdown option capitalization */
.filter select option {
  text-transform: capitalize;
}

</style>
