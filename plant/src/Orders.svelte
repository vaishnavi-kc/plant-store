<script>
  import { onMount } from "svelte";
  export let userEmail = "";

  let orders = [];
  let loading = true;
  let error = "";

  onMount(async () => {
    try {
      const res = await fetch(`http://localhost:3000/orders?email=${userEmail}`);
      if (!res.ok) throw new Error(await res.text());
      orders = await res.json();
    } catch (e) {
      error = e.message;
    } finally {
      loading = false;
    }
  });
</script>

<main>
  <h1>My Orders 📦</h1>

  {#if loading}
    <p>Loading orders...</p>
  {:else if error}
    <p style="color:red">{error}</p>
  {:else if orders.length === 0}
    <p>You haven't placed any orders yet.</p>
  {:else}
    {#each orders as order, i}
      <div class="order">
        <h3>Order #{i + 1} - Total: ₹{order.total}</h3>
        <table>
          <thead>
            <tr>
              <th>Plant</th>
              <th>Price</th>
              <th>Qty</th>
              <th>Total</th>
            </tr>
          </thead>
          <tbody>
            {#each order.items as item}
              <tr>
                <td>{item.name}</td>
                <td>₹{item.price}</td>
                <td>{item.qty}</td>
                <td>₹{item.price * item.qty}</td>
              </tr>
            {/each}
          </tbody>
        </table>
      </div>
    {/each}
  {/if}
</main>

<style>
  main { padding: 20px; background: #e8f5e8; min-height: 100vh; }
  h1 { text-align: center; margin-bottom: 20px; }
  .order { background: #fff; padding: 15px; border-radius: 8px; margin-bottom: 20px; }
  table { width: 100%; border-collapse: collapse; }
  th, td { border: 1px solid #ccc; padding: 8px; text-align: center; }
  th { background: #2f6b2f; color: white; }
</style>
