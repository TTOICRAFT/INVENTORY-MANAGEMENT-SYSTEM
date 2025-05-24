
#[cfg(test)]
mod tests {
    use super::*;

    fn setup_store() -> Store {
        let mut store = Store::new();
        store.authenticated = true; // bypass authentication for tests
        store
    }

    #[test]
    fn test_add_product() {
        let mut store = setup_store();
        let product = Product {
            name: "TestProduct".to_string(),
            description: "Test Description".to_string(),
            price: 10.0,
            quantity: 5,
        };
        store.add_product(product.clone());
        assert!(store.inventory.contains_key("TestProduct"));
        assert_eq!(store.inventory["TestProduct"].quantity, 5);
    }

    #[test]
    fn test_edit_product() {
        let mut store = setup_store();
        let product = Product {
            name: "EditProduct".to_string(),
            description: "Initial Description".to_string(),
            price: 20.0,
            quantity: 10,
        };
        store.add_product(product);
        store.edit_product("EditProduct", "New Description".to_string(), 25.0, 15);
        let updated = &store.inventory["EditProduct"];
        assert_eq!(updated.description, "New Description");
        assert_eq!(updated.price, 25.0);
        assert_eq!(updated.quantity, 15);
    }

    #[test]
    fn test_delete_product() {
        let mut store = setup_store();
        let product = Product {
            name: "DeleteProduct".to_string(),
            description: "To be deleted".to_string(),
            price: 5.0,
            quantity: 3,
        };
        store.add_product(product);
        store.delete_product("DeleteProduct");
        assert!(!store.inventory.contains_key("DeleteProduct"));
    }

    #[test]
    fn test_record_purchase() {
        let mut store = setup_store();
        store.record_purchase("NewItem".to_string(), 10, 2.5);
        assert!(store.inventory.contains_key("NewItem"));
        assert_eq!(store.inventory["NewItem"].quantity, 10);
        assert_eq!(store.purchases.len(), 1);
    }

    #[test]
    fn test_record_sale_success() {
        let mut store = setup_store();
        store.add_product(Product {
            name: "SaleItem".to_string(),
            description: "Sell me".to_string(),
            price: 8.0,
            quantity: 20,
        });
        store.record_sale("SaleItem".to_string(), 5, 10.0);
        assert_eq!(store.inventory["SaleItem"].quantity, 15);
        assert_eq!(store.sales.len(), 1);
    }

    #[test]
    fn test_record_sale_insufficient_stock() {
        let mut store = setup_store();
        store.add_product(Product {
            name: "LimitedStock".to_string(),
            description: "Rare item".to_string(),
            price: 15.0,
            quantity: 2,
        });
        store.record_sale("LimitedStock".to_string(), 5, 20.0);
        assert_eq!(store.inventory["LimitedStock"].quantity, 2); // should not decrease
        assert!(store.sales.is_empty()); // sale not recorded
    }
}
