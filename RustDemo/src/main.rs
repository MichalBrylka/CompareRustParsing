//1. (preety) printing https://doc.rust-lang.org/edition-guide/rust-2018/macros/custom-derive.html
#[derive(Debug)]
struct Cart {
    customer: String,
    products: Vec<Product>,
}
#[derive(Debug)]
struct Product {
    name: String,
    price: f32,
}

fn calculate_total(cart: Cart) -> f32 {
    let mut total = 0.0;

    //3. (free) lambdas
    let apply_discount = |c: Cart| -> bool { c.customer == "Boss" };

    //2. borrowing
    for prod in &cart.products {
        total += prod.price;
    }
    if apply_discount(cart) {
        total *= 0.8;
    }
    return total;
}

fn main() {
    let mut products = Vec::new();
    products.push(Product {
        name: "Sponge".to_string(),
        price: 2.25,
    });
    products.push(Product {
        name: "Soap".to_string(),
        price: 7.80,
    });
    //products.push(15); //4. usage type inferring

    let cart = Cart {
        customer: "Boss".to_string(),
        products,
    };
    //5. "hygenic" macros https://doc.rust-lang.org/1.30.0/book/first-edition/macros.html#hygiene
    println!("{:?}", cart);
    println!("{:#?}", cart);
    println!("Total {}", calculate_total(cart));
}

pub fn avg_greater_than(n: i32) -> f32 {
    let (mut sum, mut count) = (0, 0);
    let predicate = |i: i32| -> bool { i > n };
    for i in 1..100 {
        if predicate(i) {
            sum += i;
            count += 1;
        }
    }
    return (sum as f32) / (count as f32);
}

pub fn sum(k: i32) -> i32 {
    let mut sum = 0;
    let predicate = |ii: i32| -> bool { ii > k };
    for i in 1..1000 {
        if predicate(i) {
            sum += 1;
        }
    }
    return sum;
}
