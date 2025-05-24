use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, BufRead, Write};

#[derive(Clone, Serialize, Deserialize)]
struct Product {
    name: String,
    description: String,
    price: f64,     
    quantity: u32,
    total_cost: f64, 
    total_purchased_qty: u32,
}

impl Product {
    fn average_cost(&self) -> f64 {
        if self.total_purchased_qty == 0 {
            0.0
        } else {
            self.total_cost / self.total_purchased_qty as f64
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Sale {
    product_name: String,
    quantity: u32,
    sale_price: f64,
    profit: f64,
}

#[derive(Debug, Serialize, Deserialize)]
struct Purchase {
    product_name: String,
    quantity: u32,
    purchase_price: f64,
}

#[derive(Serialize, Deserialize)]
struct Store {
    inventory: HashMap<String, Product>,
    sales: Vec<Sale>,
    purchases: Vec<Purchase>,
    authenticated: bool,
    password: String,
}

impl Store {
    fn new() -> Self {
        Store {
            inventory: HashMap::new(),
            sales: Vec::new(),
            purchases: Vec::new(),
            authenticated: false,
            password: "admin".to_string(),
        }
    }

    fn load_from_files() -> io::Result<Self> {
        let inventory = match fs::read_to_string("data/inventory.json") {
            Ok(data) => serde_json::from_str(&data).unwrap_or_default(),
            Err(_) => HashMap::new(),
        };
        let sales = match fs::read_to_string("data/sales.json") {
            Ok(data) => serde_json::from_str(&data).unwrap_or_default(),
            Err(_) => Vec::new(),
        };
        let purchases = match fs::read_to_string("data/purchases.json") {
            Ok(data) => serde_json::from_str(&data).unwrap_or_default(),
            Err(_) => Vec::new(),
        };

        Ok(Store {
            inventory,
            sales,
            purchases,
            authenticated: false,
            password: "admin".to_string(),
        })
    }

    fn save_to_files(&self) -> io::Result<()> {
        fs::write("data/inventory.json", serde_json::to_string_pretty(&self.inventory).unwrap())?;
        fs::write("data/sales.json", serde_json::to_string_pretty(&self.sales).unwrap())?;
        fs::write("data/purchases.json", serde_json::to_string_pretty(&self.purchases).unwrap())?;
        Ok(())
    }

    fn authenticate(&mut self) -> bool {
        println!("Enter password:");
        let input = read_trimmed_line();
        if input == self.password {
            self.authenticated = true;
            println!("Authentication successful!");
            true
        } else {
            println!("Authentication failed.");
            false
        }
    }

    fn require_authentication(&self) -> bool {
        if !self.authenticated {
            println!("Access denied. Please authenticate first.");
            false
        } else {
            true
        }
    }

    fn add_product(&mut self) {
        if !self.require_authentication() {
            return;
        }
        println!("Adding new product.");
        let name = prompt_nonempty("Product name: ");
        if self.inventory.contains_key(&name) {
            println!("Product already exists. Use edit instead.");
            return;
        }
        let description = prompt_nonempty("Description: ");
        let price = prompt_positive_f64("Selling price: ");
        let quantity = prompt_u32("Quantity: ");

        let product = Product {
            name: name.clone(),
            description,
            price,
            quantity,
            total_cost: 0.0,
            total_purchased_qty: 0,
        };
        self.inventory.insert(name, product);
        println!("Product added.");
    }

    fn edit_product(&mut self) {
        if !self.require_authentication() {
            return;
        }
        let name = prompt_nonempty("Enter product name to edit: ");
        if let Some(product) = self.inventory.get_mut(&name) {
            println!("Editing product: {}", name);
            let description = prompt_nonempty("New description: ");
            let price = prompt_positive_f64("New selling price: ");
            let quantity = prompt_u32("New quantity: ");

            product.description = description;
            product.price = price;
            product.quantity = quantity;
            println!("Product updated.");
        } else {
            println!("Product not found.");
        }
    }

    fn delete_product(&mut self) {
        if !self.require_authentication() {
            return;
        }
        let name = prompt_nonempty("Enter product name to delete: ");
        if self.inventory.remove(&name).is_some() {
            println!("Product '{}' deleted.", name);
        } else {
            println!("Product not found.");
        }
    }

    fn list_inventory(&self) {
        println!("\nInventory:");
        if self.inventory.is_empty() {
            println!("No products found.");
            return;
        }
        for product in self.inventory.values() {
            println!(
                "{}: {}, Price: ${:.2}, Qty: {}, Avg Cost: ${:.2}",
                product.name,
                product.description,
                product.price,
                product.quantity,
                product.average_cost()
            );
        }
    }

    fn record_sale(&mut self) {
        if !self.require_authentication() {
            return;
        }
        let product_name = prompt_nonempty("Product sold: ");
        let quantity = prompt_u32("Quantity sold: ");
        let sale_price = prompt_positive_f64("Sale price per unit: ");

        match self.inventory.get_mut(&product_name) {
            Some(product) if product.quantity >= quantity => {
                product.quantity -= quantity;
                let profit = (sale_price - product.average_cost()) * quantity as f64;
                self.sales.push(Sale {
                    product_name: product_name.clone(),
                    quantity,
                    sale_price,
                    profit,
                });
                println!("Sale recorded. Profit: ${:.2}", profit);
            }
            Some(_) => {
                println!("Not enough stock.");
            }
            None => {
                println!("Product not found.");
            }
        }
    }

    fn record_purchase(&mut self) {
        if !self.require_authentication() {
            return;
        }
        let product_name = prompt_nonempty("Product purchased: ");
        let quantity = prompt_u32("Quantity purchased: ");
        let purchase_price = prompt_positive_f64("Purchase price per unit: ");

        if let Some(product) = self.inventory.get_mut(&product_name) {
            product.quantity += quantity;
            product.total_cost += purchase_price * quantity as f64;
            product.total_purchased_qty += quantity;
            if purchase_price > product.price {
                product.price = purchase_price * 1.1; // 10% markup
            }
        } else {
            let product = Product {
                name: product_name.clone(),
                description: "No description".to_string(),
                price: purchase_price * 1.1,
                quantity,
                total_cost: purchase_price * quantity as f64,
                total_purchased_qty: quantity,
            };
            self.inventory.insert(product_name.clone(), product);
        }
        self.purchases.push(Purchase {
            product_name,
            quantity,
            purchase_price,
        });
        println!("Purchase recorded.");
    }

    fn generate_reports(&self) {
        println!("\n----- Sales Report -----");
        if self.sales.is_empty() {
            println!("No sales recorded.");
        } else {
            let mut total_sales = 0.0;
            let mut total_profit = 0.0;
            for sale in &self.sales {
                println!(
                    "Sold {} units of {} at ${:.2} each. Profit: ${:.2}",
                    sale.quantity, sale.product_name, sale.sale_price, sale.profit
                );
                total_sales += sale.sale_price * sale.quantity as f64;
                total_profit += sale.profit;
            }
            println!("Total Sales: ${:.2}", total_sales);
            println!("Total Profit: ${:.2}", total_profit);
        }

        println!("\n----- Purchase Report -----");
        if self.purchases.is_empty() {
            println!("No purchases recorded.");
        } else {
            let mut total_cost = 0.0;
            for purchase in &self.purchases {
                println!(
                    "Purchased {} units of {} at ${:.2} each",
                    purchase.quantity, purchase.product_name, purchase.purchase_price
                );
                total_cost += purchase.purchase_price * purchase.quantity as f64;
            }
            println!("Total Purchase Cost: ${:.2}", total_cost);
        }

        println!("\n----- Inventory Report -----");
        self.list_inventory();
    }
}

fn main() {
    let mut store = Store::load_from_files().unwrap_or_else(|_| Store::new());

    if !store.authenticate() {
        return;
    }

    loop {
        println!("\n--- Store Management Menu ---");
        println!("1. Add Product");
        println!("2. Edit Product");
        println!("3. Delete Product");
        println!("4. List Inventory");
        println!("5. Record Sale");
        println!("6. Record Purchase");
        println!("7. Generate Reports");
        println!("8. Logout");
        println!("9. Exit");
        print!("Choose an option: ");
        io::stdout().flush().unwrap();

        let choice = read_trimmed_line();

        match choice.as_str() {
            "1" => store.add_product(),
            "2" => store.edit_product(),
            "3" => store.delete_product(),
            "4" => store.list_inventory(),
            "5" => store.record_sale(),
            "6" => store.record_purchase(),
            "7" => store.generate_reports(),
            "8" => {
                store.authenticated = false;
                println!("Logged out.");
                if !store.authenticate() {
                    break;
                }
            }
            "9" => {
                println!("Exiting...");
                break;
            }
            _ => println!("Invalid option."),
        }
    
        if let Err(e) = store.save_to_files() {
            println!("Warning: Failed to save data: {}", e);
        }
    }
}

/// Helper: Reads a trimmed line from stdin
fn read_trimmed_line() -> String {
let stdin = io::stdin();
let mut input = String::new();
stdin.lock().read_line(&mut input).expect("Failed to read input");
input.trim().to_string()
}

/// Helper: Prompt repeatedly until a non-empty string is entered
fn prompt_nonempty(prompt: &str) -> String {
loop {
print!("{}", prompt);
io::stdout().flush().unwrap();
let input = read_trimmed_line();
if !input.is_empty() {
return input;
}
println!("Input cannot be empty. Please try again.");
}
}

/// Helper: Prompt for a positive f64 number
fn prompt_positive_f64(prompt: &str) -> f64 {
loop {
print!("{}", prompt);
io::stdout().flush().unwrap();
let input = read_trimmed_line();
match input.parse::<f64>() {
Ok(val) if val > 0.0 => return val,
_ => println!("Please enter a valid positive number."),
}
}
}

/// Helper: Prompt for a u32 number
fn prompt_u32(prompt: &str) -> u32 {
loop {
print!("{}", prompt);
io::stdout().flush().unwrap();
let input = read_trimmed_line();
match input.parse::<u32>() {
Ok(val) => return val,
_ => println!("Please enter a valid positive integer."),
}
}
}    
