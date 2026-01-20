use actix_cors::Cors;
use actix_web::{get, post, put, delete, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use sqlx::{Row, SqlitePool};
use std::env;

/* ================= USER ================= */

#[derive(Deserialize)]
struct RegisterData {
    name: String,
    email: String,
    password: String,
}

#[derive(Deserialize)]
struct LoginData {
    email: String,
    password: String,
}

#[derive(Serialize)]
struct UserResponse {
    id: i64,
    name: String,
    email: String,
}

/* ================= PRODUCT ================= */

#[derive(Serialize, Deserialize)]
struct Product {
    id: i64,
    name: String,
    price: i32,
    image: String,
    category: String,
}

#[derive(Deserialize)]
struct ProductRequest {
    name: String,
    price: i32,
    image: String,
    category: String,
}

/* ================= ORDER ================= */

#[derive(Deserialize)]
struct OrderRequest {
    email: String,
    items: Vec<OrderItem>,
    total: i32,
}

#[derive(Serialize, Deserialize, Clone)]
struct OrderItem {
    name: String,
    price: i32,
    qty: i32,
}

/* ================= AUTH ================= */

#[post("/register")]
async fn register(
    pool: web::Data<SqlitePool>,
    data: web::Json<RegisterData>,
) -> impl Responder {
    match sqlx::query(
        "INSERT INTO users (name, email, password) VALUES (?, ?, ?)",
    )
    .bind(&data.name)
    .bind(&data.email)
    .bind(&data.password)
    .execute(pool.get_ref())
    .await
    {
        Ok(_) => HttpResponse::Ok().body("Registered successfully"),
        Err(e) => HttpResponse::BadRequest().body(e.to_string()),
    }
}

#[post("/login")]
async fn login(
    pool: web::Data<SqlitePool>,
    data: web::Json<LoginData>,
) -> impl Responder {
    let result = sqlx::query(
        "SELECT id, name, email FROM users WHERE email=? AND password=?",
    )
    .bind(&data.email)
    .bind(&data.password)
    .fetch_optional(pool.get_ref())
    .await;

    match result {
        Ok(Some(row)) => {
            let user = UserResponse {
                id: row.get("id"),
                name: row.get("name"),
                email: row.get("email"),
            };

            HttpResponse::Ok().json(serde_json::json!({
                "message": "Login success",
                "user": user
            }))
        }
        Ok(None) => HttpResponse::Unauthorized().body("Invalid email or password"),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

/* ================= PRODUCTS ================= */

#[get("/products")]
async fn get_products(pool: web::Data<SqlitePool>) -> impl Responder {
    let rows = sqlx::query(
        "SELECT id, name, price, image, category FROM products",
    )
    .fetch_all(pool.get_ref())
    .await
    .unwrap();

    let products: Vec<Product> = rows
        .into_iter()
        .map(|r| Product {
            id: r.get("id"),
            name: r.get("name"),
            price: r.get("price"),
            image: r.get("image"),
            category: r.get("category"),
        })
        .collect();

    HttpResponse::Ok().json(products)
}

#[post("/products")]
async fn add_product(
    pool: web::Data<SqlitePool>,
    data: web::Json<ProductRequest>,
) -> impl Responder {
    sqlx::query(
        "INSERT INTO products (name, price, image, category)
         VALUES (?, ?, ?, ?)",
    )
    .bind(&data.name)
    .bind(data.price)
    .bind(&data.image)
    .bind(&data.category)
    .execute(pool.get_ref())
    .await
    .unwrap();

    HttpResponse::Ok().body("Product added")
}

#[put("/products/{id}")]
async fn update_product(
    pool: web::Data<SqlitePool>,
    path: web::Path<i64>,
    data: web::Json<ProductRequest>,
) -> impl Responder {
    sqlx::query(
        "UPDATE products
         SET name=?, price=?, image=?, category=?
         WHERE id=?",
    )
    .bind(&data.name)
    .bind(data.price)
    .bind(&data.image)
    .bind(&data.category)
    .bind(*path)
    .execute(pool.get_ref())
    .await
    .unwrap();

    HttpResponse::Ok().body("Product updated")
}

#[delete("/products/{id}")]
async fn delete_product(
    pool: web::Data<SqlitePool>,
    path: web::Path<i64>,
) -> impl Responder {
    sqlx::query("DELETE FROM products WHERE id=?")
        .bind(*path)
        .execute(pool.get_ref())
        .await
        .unwrap();

    HttpResponse::Ok().body("Product deleted")
}

/* ================= ORDERS ================= */

#[post("/orders")]
async fn create_order(
    pool: web::Data<SqlitePool>,
    data: web::Json<OrderRequest>,
) -> impl Responder {
    let res = sqlx::query(
        "INSERT INTO orders (user_email, total) VALUES (?, ?)",
    )
    .bind(&data.email)
    .bind(data.total)
    .execute(pool.get_ref())
    .await
    .unwrap();

    let order_id = res.last_insert_rowid();

    for item in &data.items {
        sqlx::query(
            "INSERT INTO order_items (order_id, name, price, qty)
             VALUES (?, ?, ?, ?)",
        )
        .bind(order_id)
        .bind(&item.name)
        .bind(item.price)
        .bind(item.qty)
        .execute(pool.get_ref())
        .await
        .unwrap();
    }

    HttpResponse::Ok().body("Order placed")
}

#[get("/orders")]
async fn get_orders(
    pool: web::Data<SqlitePool>,
    query: web::Query<std::collections::HashMap<String, String>>,
) -> impl Responder {
    let email = query.get("email").unwrap();

    let orders = sqlx::query("SELECT id, total FROM orders WHERE user_email=?")
        .bind(email)
        .fetch_all(pool.get_ref())
        .await
        .unwrap();

    let mut result = Vec::new();

    for o in orders {
        let order_id: i64 = o.get("id");

        let items = sqlx::query(
            "SELECT name, price, qty FROM order_items WHERE order_id=?",
        )
        .bind(order_id)
        .fetch_all(pool.get_ref())
        .await
        .unwrap()
        .into_iter()
        .map(|i| OrderItem {
            name: i.get("name"),
            price: i.get("price"),
            qty: i.get("qty"),
        })
        .collect::<Vec<_>>();

        result.push(serde_json::json!({
            "order_id": order_id,
            "items": items,
            "total": o.get::<i32,_>("total")
        }));
    }

    HttpResponse::Ok().json(result)
}

/* ================= SERVER ================= */

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL")
        .unwrap_or("sqlite:plant.db".to_string());

    let pool = SqlitePool::connect(&db_url).await.unwrap();

    println!("🚀 Server running on http://localhost:3000");

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .app_data(web::Data::new(pool.clone()))
            .service(register)
            .service(login)
            .service(get_products)
            .service(add_product)
            .service(update_product)
            .service(delete_product)
            .service(create_order)
            .service(get_orders)
    })
    .bind(("0.0.0.0", 3000))?
    .run()
    .await
}
