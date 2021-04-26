//1. (preety) printing
#[derive(Debug)]
struct Cart{
    customer: String, 
    products: Vec<Product>
}
#[derive(Debug)]
struct Product {
    name: String,    
    price: f32
}

fn calculate_total(cart: Cart) -> f32 {
    let mut total = 0.0;
    
    //3. (free) lambdas  
    let apply_discount = |c: Cart| -> bool {c.customer == "Boss"};
    
    //2. borrowing 
    for prod in &cart.products {
        total+=prod.price;
    }
    
    if apply_discount(cart) {
        total *= 0.8;
    }
    
    return total;
}

fn main(){
    let mut products = Vec::new();
    products.push(Product{name:"Sponge".to_string(), price: 2.25});
    products.push(Product{name:"Soap".to_string(), price: 7.80});
    //products.push(15); //4. usage type inferring 
    
    let cart = Cart { customer: "Boss".to_string(), products};

    //5. "hygenic" macros https://doc.rust-lang.org/1.30.0/book/first-edition/macros.html#hygiene
    println!("{:?}", cart);
    println!("{:#?}", cart);
    println!("Total {}", calculate_total(cart));
}