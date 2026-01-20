<script>
  import { onMount } from "svelte";

  let orders = [];
  let loading = true;
  let error = "";

  onMount(async () => {
    try {
      const res = await fetch("http://localhost:3000/admin/orders");
      if (!res.ok) throw new Error(await res.text());
      orders = await res.json();
    } catch (e) {
      error = e.message;
    } finally {
      loading = false;
    }
  });
</script>

<div class="container">
  <h2>View Orders</h2>

  {#if loading}
    <p>Loading...</p>

  {:else if error}
    <p class="error">{error}</p>

  {:else if orders.length === 0}
    <p>No orders found</p>

  {:else}
    {#each orders as order}
      <div class="order-card">
        <div class="order-header">
          <span><b>Order </b>{order.order_id}</span>
          <span><b>User:</b> {order.email}</span>
          <span><b>Total:</b> ₹{order.total}</span>
        </div>

        <table>
          <thead>
            <tr>
              <th>Plant</th>
              <th>Price</th>
              <th>Qty</th>
              <th>Subtotal</th>
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
</div>

<style>
    h2{
    color: #2e7d32;
    text-align: center;
    }
.container {
  padding: 25px;
}

h2 {
  margin-bottom: 20px;
}

.order-card {
  background: #fff;
  border-radius: 10px;
  padding: 15px;
  margin-bottom: 25px;
  box-shadow: 0 4px 10px rgba(0,0,0,0.08);
}

.order-header {
  display: flex;
  justify-content: space-between;
  margin-bottom: 12px;
  background: #f4f8f4;
  padding: 10px;
  border-radius: 6px;
  font-size: 14px;
}

table {
  width: 100%;
  border-collapse: collapse;
}

th {
  background: #2f6b2f;
  color: white;
  padding: 10px;
}

td {
  padding: 8px;
  text-align: center;
  border-bottom: 1px solid #ddd;
}

tr:hover {
  background: #f5f5f5;
}

.error {
  color: red;
}
</style>
