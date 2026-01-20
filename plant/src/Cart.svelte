<script>
  export let cart = [];
  export let userEmail = ""; 

  $: total = cart.reduce((sum, item) => sum + item.price * item.qty, 0);

  function increaseQty(index) {
    cart[index].qty += 1;
    cart = [...cart];
  }

  function decreaseQty(index) {
    if (cart[index].qty > 1) {
      cart[index].qty -= 1;
    } else {
      cart.splice(index, 1);
    }
    cart = [...cart];
  }

  async function buyNow() {
    if (cart.length === 0) {
      alert("Your cart is empty 🛒");
      return;
    }

    try {
      const res = await fetch("http://localhost:3000/orders", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ email: userEmail, items: cart, total })
      });

      if (!res.ok) throw new Error(await res.text());

      alert(`Order placed successfully 🎉\n\nTotal Amount: ₹${total}\n\nThank you for shopping 🌱`);
      cart = [];
    } catch (e) {
      alert("Failed to place order: " + e.message);
    }
  }
</script>



<main>
  <h1>Your Cart 🛒</h1>

  {#if cart.length === 0}
    <p class="empty">Your cart is empty</p>
  {:else}
    <table class="cart-table">
      <thead>
        <tr>
          <th>Image</th>
          <th>Plant</th>
          <th>Price</th>
          <th>Qty</th>
          <th>Total</th>
          <th>Action</th>
        </tr>
      </thead>

      <tbody>
        {#each cart as item, index}
          <tr>
            <td>
              <img src={item.image} alt={item.name} />
            </td>

            <td>{item.name}</td>
            <td>₹{item.price}</td>

            <td class="qty">
              <button on:click={() => decreaseQty(index)}>-</button>
              <span>{item.qty}</span>
              <button on:click={() => increaseQty(index)}>+</button>
            </td>

            <td>₹{item.price * item.qty}</td>

            <td>
              <button class="remove" on:click={() => decreaseQty(index)}>
                Remove
              </button>
            </td>
          </tr>
        {/each}

        <tr class="grand-total">
          <td colspan="4"><strong>Grand Total</strong></td>
          <td colspan="2"><strong>₹{total}</strong></td>
        </tr>
      </tbody>
    </table>

    <div class="buy">
      <button class="buy-btn" on:click={buyNow}>
        Buy Now 💳
      </button>
    </div>
  {/if}
</main>

<style>
  main {
    padding: 20px;
    background: #e8f5e8;
    min-height: 100vh;
  }

  h1 {
    text-align: center;
    margin-bottom: 20px;
  }

  .empty {
    text-align: center;
    font-size: 18px;
  }

  .cart-table {
    width: 100%;
    border-collapse: collapse;
    background: white;
  }

  th {
    background: #2f6b2f;
    color: white;
  }

  th, td {
    border: 1px solid #ccc;
    padding: 10px;
    text-align: center;
  }

  img {
    width: 70px;
    height: 70px;
    border-radius: 6px;
    object-fit: cover;
  }

  .qty {
    display: flex;
    justify-content: center;
    align-items: center;
    gap: 10px;
  }

  .qty button {
    width: 30px;
    height: 30px;
  }

  button {
    background: #2f6b2f;
    color: white;
    border: none;
    border-radius: 5px;
    cursor: pointer;
    padding: 6px 12px;
  }

  .remove {
    background: #c0392b;
  }

  .grand-total {
    background: #f4f4f4;
    font-size: 18px;
  }

  .buy {
    text-align: right;
    margin-top: 20px;
  }

  .buy-btn {
    background: #ffb703;
    color: #333;
    font-size: 16px;
    padding: 10px 20px;
    border-radius: 8px;
  }

  .buy-btn:hover {
    background: #e6a800;
  }
</style>
