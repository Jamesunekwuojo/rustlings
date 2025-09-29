// This is a quiz for the following sections:
// - Variables
// - Functions
// - If
//
// Mary is buying apples. The price of an apple is calculated as follows:
// - An apple costs 2 rustbucks.
// - However, if Mary buys more than 40 apples, the price of each apple in the
// entire order is reduced to only 1 rustbuck!

// TODO: Write a function that calculates the price of an order of apples given
// the quantity bought.
// fn calculate_price_of_apples(???) -> ??? { ??? }

// Origial cost of apple is let mut price = 2 rustbucks
// price of each apple remans 2 for each when order <= 40
// total_price = price * order
//price = 1 when order > 40
// print total price

fn calculate_price_of_apples(total_order_no:i64) -> i64 {
    

    // (if total_order_no > 40 {
    //     1
    // } else {
    //     2
    // }) * total_order_no

    let price = if total_order_no > 40 {1} else {2};

    let total_price = price * total_order_no;
    total_price

    // Lesson learnt
    //You must return a value from functions, and statements need semicolons unless they are the final expression you want to return.


  



}


fn main() {
    // You can optionally experiment here.
}

// Don't change the tests!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_test() {
        assert_eq!(calculate_price_of_apples(35), 70);
        assert_eq!(calculate_price_of_apples(40), 80);
        assert_eq!(calculate_price_of_apples(41), 41);
        assert_eq!(calculate_price_of_apples(65), 65);
    }
}
