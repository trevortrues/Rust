
fn main() {
    let order1 = Order{
        boots: Some(3),
        board: Some(0),
        helmet: Some(2),
        order_date: 10
    };
    let order2 = Order{
        boots: Some(2),
        board: Some(1),
        helmet: Some(0),
        order_date: 5
    };
    let order3 = Order{
        boots: Some(0),
        board: Some(1),
        helmet: Some(1),
        order_date: 8
    };
    let order4 = Order{
        boots: Some(100),
        board: Some(100),
        helmet: Some(100),
        order_date: 15
    };

    let mut inventory = Inventory{
        boot_num: 10,
        board_num: 15,
        helmet_num: 5
    };
    
    let mut orders = vec![order1, order2, order3, order4];
    Inventory::process_orders(&mut inventory, &mut orders);
    

}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_check_availability() {
        let order = Order {
            boots: Some(2),
            board: Some(0), 
            helmet: Some(1),
            order_date: 1
        };
        let inventory = Inventory{
            boot_num: 2,
            board_num: 0, 
            helmet_num: 1
         };

         let avail = check_availability(&order, &inventory);
         assert_eq!(true, avail[0]);

    }
   

}

#[derive(Debug)]
struct Order {
    boots: Option<i32>,
    board: Option<i32>,
    helmet: Option<i32>,
    order_date: i32,
}

#[derive(Debug)]
struct Inventory{
    boot_num: i32,
    board_num: i32,
    helmet_num: i32
}


fn sort_orders(orders: &mut Vec<Order>){
    orders.sort_by_key(|order| order.order_date);
}

/// Checks the availability of an order
///
/// # Examples
/// 
/// ```
/// let order = Order {
///     boots: Some(2),
///     board: Some(0), 
///     helmet: Some(1),
///     order_date: 1
/// };
/// let inventory = Inventory{
///     boot_num: 2,
///     board_num: 0, 
///     helmet_num: 1
/// };
/// assert_eq!(true, check_availability(&order, &inventory)[0]);
/// ```


fn check_availability(order: &Order, inventory: &Inventory) -> Vec<bool>{
    let boots_available= match order.boots{
        Some(_) => inventory.boot_num>=order.boots.unwrap_or(0),
        None => true,
    };
    let board_available= match order.board{
        Some(_) => inventory.board_num>=order.board.unwrap_or(0),
        None => true,
    };
    let helmet_available= match order.helmet{
        Some(_) => inventory.helmet_num>=order.helmet.unwrap_or(0),
        None => true,
    };
    vec![boots_available, board_available, helmet_available]

}

impl Inventory{
    fn rent_gear(&mut self, order: &Order){
        let availability = check_availability(order, self);
        if availability[0] == true{
            self.boot_num -= order.boots.unwrap_or(0);
        }
        if availability[1] == true{
            self.board_num -= order.board.unwrap_or(0);
        }
        if availability[2] == true{
            self.helmet_num -= order.helmet.unwrap_or(0);
        }
    }
    fn process_orders(&mut self, orders: &mut Vec<Order>){
        sort_orders(orders);
        for item in orders.iter_mut() {
            println!("Current Order: {:?}", item);
            println!("");
            println!("Current Inventory: {:?}", self);
            println!("");
            Inventory::rent_gear(self, item);
        }
        
        
    }
}


