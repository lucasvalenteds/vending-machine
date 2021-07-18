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
    fn execute(&mut self, command: Command) -> Self;
}

#[derive(Clone, Copy)]
struct CoffeeMachine {
    balance: f32
}

impl VendingMachine for CoffeeMachine {
    fn new() -> Self {
        CoffeeMachine { balance: 0.0 }
    }

    fn execute(&mut self, command: Command) -> Self {
        match command {
            Command::AddMoney(ref amount) => {
                CoffeeMachine {
                    balance: self.balance + amount
                }
            },
            Command::Buy(ref coffe_type) => {
                let coffee: Coffee = coffe_type.new();
                CoffeeMachine {
                    balance: self.balance - coffee.price
                }
            }
        }
    }
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

    #[test]
    fn test_buying_coffee() {
        let machine = CoffeeMachine::new()
            .execute(Command::AddMoney(3.00))
            .execute(Command::Buy(CoffeeType::Expresso));

        assert_eq!(1.50, machine.balance)
    }
}
