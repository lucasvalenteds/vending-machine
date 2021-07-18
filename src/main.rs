fn main() {
}

struct Coffee {
    price: f32,
    size: usize
}

enum CoffeeType {
    Expresso,
    Cappuccino
}

impl CoffeeType {
    fn new(&self) -> Coffee {
        match *self {
            CoffeeType::Expresso => Coffee { price: 1.50, size: 100 },
            CoffeeType::Cappuccino => Coffee { price: 4.50, size: 200 },
        }
    }
}

enum Command {
    AddMoney(f32),
    Buy(CoffeeType)
}

trait VendingMachine {
    fn new() -> Self;
    fn execute(&self, command: Command);
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_creating_coffee() {
        let expresso: Coffee = CoffeeType::Expresso.new();
        assert_eq!(1.50, expresso.price);
        assert_eq!(100, expresso.size);

        let cappuccino: Coffee = CoffeeType::Cappuccino.new();
        assert_eq!(4.50, cappuccino.price);
        assert_eq!(200, cappuccino.size);
    }
}
