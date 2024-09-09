// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
struct GroceryItem {
    quantity: i32,
    id_number: i32,
}

// * Create a function to display the quantity, with the struct as a parameter

fn display_quantity(groceryitem: &GroceryItem) {
    println!("Quantity: {}", groceryitem.quantity);
}

// * Create a function to display the id number, with the struct as a parameter

fn display_id_number(groceryitem: &GroceryItem) {
    println!("ID Number: {}", groceryitem.id_number);
}

fn main() {
    let item = GroceryItem {
        quantity: 110,
        id_number: 119,
    };
    display_quantity(&item);
    display_id_number(&item);
}
