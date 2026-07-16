#[derive(Debug, Clone)]
struct Electronics {
    name: String,
    brand: String,
    price: f64
}

#[derive(Debug, Clone)]
struct Clothing {
    name: String,
    size: String,
    price: f64
}

#[derive(Debug, Clone)]
struct Grocery {
    name: String,
    expiry_days: u32,
    price: f64
}

#[derive(Debug, Clone)]
struct Product<T> {
    id: u32,
    data: T,
    stock: u32
}

impl<T> Product<T> {
    fn new(id: u32, data: T, stock: u32) -> Self {
        Self {
            id,
            data,
            stock
        }
    }

    fn get_id(&self) -> &u32 {
        &self.id
    }
    
    fn get_data(&self) -> &T {
        &self.data
    }

    fn get_stock(&self) -> &u32 {
        &self.stock
    }

    fn update_stock(&mut self, value: u32) {
        self.stock += value;
    }
}

#[derive(Debug, Clone)]
struct Store<T> {
    name: String,
    products: Vec<Product<T>>
}

impl<T> Store<T> {
    fn new(name: String) -> Self {
        Self {
            name,
            products: Vec::new()
        }
    }

    fn add_product(&mut self, product: Product<T>) {
        self.products.push(product);
    }

    fn remove_product(&mut self, id: u32) {
        self.products.retain(|product| product.id != id);
    }

    fn get_products_count(&self) -> usize {
        self.products.len()
    }

    fn is_empty(&self) -> bool {
        self.products.len() <= 0
    }

    fn get_first_product(&self) -> Option<&Product<T>> {
        self.products.get(0)
    }

    fn get_last_product(&self) -> Option<&Product<T>> {
        self.products.get(self.get_products_count() - 1)
    }

    fn find_product_by_id(&self, id: u32) -> Option<&Product<T>> {
        for product in &self.products {
            if product.id == id {
                return Some(product);
            }
        }
        return None;
    }
}

fn main() {
    let electronics1: Electronics = Electronics {
        name: String::from("iphone_air"),
        brand: String::from("Apple"),
        price: 9999999999999.9999999999999
    };

    let product1: Product<Electronics> = Product::new(1, electronics1, 20);

    let electronics2: Electronics = Electronics {
        name: String::from("iphone_pro_max"),
        brand: String::from("Apple"),
        price: 99999999999999.99999999999999
    };

    let product2: Product<Electronics> = Product::new(2, electronics2, 50);

    let mut electronic_store1: Store<Electronics> = Store::new(String::from("Apple"));
    electronic_store1.add_product(product1);
    electronic_store1.add_product(product2);
    println!("the electronic_store1 before removing any product is {:#?}", electronic_store1);
    println!("total number of products the electronic_store1 is {}", electronic_store1.get_products_count());
    println!("is electronic_store1 empty: {}", electronic_store1.is_empty());
}