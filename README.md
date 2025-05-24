# Rusty Store Inventory Management System

## 📦 Overview

Rusty Store is a simple inventory management system for a small retail store, written in Rust. It helps store managers to manage inventory, sales, and purchases efficiently with file persistence and basic authentication.

---

## 🚀 Features

* **Inventory Management**: Add, edit, delete products.
* **Sales Management**: Record sales and track profits.
* **Purchase Management**: Record purchases and track costs.
* **Reporting**: Generate sales and purchase reports.
* **Error Handling**: Graceful handling of invalid inputs and stock issues.
* **Authentication**: Password-based login for store managers.
* **Persistence**: Data saved to JSON files (`data/` folder).
* **Text-Based UI**: Simple terminal interface.

---

## 🛠 Requirements

* Rust (latest stable version) — [Install Rust](https://www.rust-lang.org/tools/install)

---

## 🧪 Running the System

1. **Clone the repository:**

```bash
git clone https://github.com/yourusername/rusty-store.git
cd rusty-store
```

2. **Create the data directory (if it doesn't exist):**

```bash
mkdir data
```

3. **Run the program:**

```bash
cargo run
```

4. **Default Authentication:**

* Username: N/A (not implemented yet)
* Password: `admin`

---

## 📁 Project Structure

```
rusty-store/
├── src/
│   └── main.rs
├── data/
│   ├── inventory.json
│   ├── sales.json
│   └── purchases.json
├── test/
│   ├── integration_test.rs
├── Cargo.toml
└── README.md
```

---

## ✅ Test Cases

Below are some manual test case examples you can try from the terminal:

### 1. Add a product

* Input: Name: `Wine`, Desc: `Gold Wine`, Price: `15.0`, Quantity: `50`
* Expected: Product added to inventory and visible in inventory listing.

### 2. Record a sale

* Input: Product: `Wine`, Quantity: `5`, Sale Price: `20.0`
* Expected: Quantity decreases, sale recorded.

### 3. Record a purchase

* Input: Product: `Wine`, Quantity: `10`, Purchase Price: `10.0`
* Expected: Quantity increases, purchase recorded.

### 4. Edit a product

* Input: Name: `Wine`, New Desc: `Updated Wine`, Price: `18.0`, Quantity: `60`
* Expected: Inventory reflects the updated values.

### 5. Generate report

* Expected: Shows total sales and total purchases with all transactions.

### 6. Error case: insufficient stock

* Try to sell 999 units of a product with 10 in stock
* Expected: Error message shown and transaction blocked.

---

## 🔐 Authentication Notes

* On program start, user must input the password (`admin`) to proceed.
* Unauthorized users cannot add, edit, or delete inventory.
* All other operations are blocked if authentication fails.

---

## 📌 Future Improvements

* Username-based login
* Role-based access control
* Export reports to CSV
* GUI frontend using `egui` or `tui`

---

## 🧾 License

MIT License. Feel free to use and modify.

---

## 👨‍💻 Author

Developed by TTOICRAFT.
