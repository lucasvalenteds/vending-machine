fn main() {}

#[derive(Copy, Clone, Debug)]
struct Coffee {
    price: f32,
    size: usize,
}

#[derive(Copy, Clone, Debug)]
enum CoffeeType {
    Expresso,
    Cappuccino,
}

impl CoffeeType {
    fn new(&self) -> Coffee {
        match *self {
            CoffeeType::Expresso => Coffee {
                price: 1.50,
                size: 100,
            },
            CoffeeType::Cappuccino => Coffee {
                price: 4.50,
                size: 200,
            },
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Command {
    AddMoney(f32),
    Buy(CoffeeType),
}

trait VendingMachine {
    fn new() -> Self;
    fn execute(&mut self, command: Command) -> Self;
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum OperationResult {
    Success,
    InsufficientBalance(f32),
}

#[derive(Clone, Copy)]
struct CoffeeMachine {
    balance: f32,
    result: Option<OperationResult>,
}

impl VendingMachine for CoffeeMachine {
    fn new() -> Self {
        CoffeeMachine {
            balance: 0.0,
            result: None,
        }
    }

    fn execute(&mut self, command: Command) -> Self {
        match command {
            Command::AddMoney(ref amount) => CoffeeMachine {
                balance: self.balance + amount,
                result: Some(OperationResult::Success),
            },
            Command::Buy(ref coffee_type) => {
                let coffee: Coffee = coffee_type.new();
                let change = self.balance - coffee.price;

                if change.is_sign_positive() {
                    CoffeeMachine {
                        balance: change,
                        result: Some(OperationResult::Success),
                    }
                } else {
                    CoffeeMachine {
                        balance: self.balance,
                        result: Some(OperationResult::InsufficientBalance(change)),
                    }
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
    fn test_initializing_the_vending_machine() {
        let machine = CoffeeMachine::new();

        assert_eq!(0.0, machine.balance);
        assert_eq!(None, machine.result);
    }

    #[test]
    fn test_adding_money() {
        let machine = CoffeeMachine::new()
            .execute(Command::AddMoney(0.25))
            .execute(Command::AddMoney(1.00))
            .execute(Command::AddMoney(0.25))
            .execute(Command::AddMoney(2.05));

        assert_eq!(3.55, machine.balance);
        assert_eq!(Some(OperationResult::Success), machine.result)
    }

    #[test]
    fn test_buying_coffee() {
        let machine = CoffeeMachine::new()
            .execute(Command::AddMoney(3.00))
            .execute(Command::Buy(CoffeeType::Expresso));

        assert_eq!(1.50, machine.balance);
        assert_eq!(Some(OperationResult::Success), machine.result)
    }

    #[test]
    fn test_not_buying_coffee_insufficient_balance() {
        let machine = CoffeeMachine::new()
            .execute(Command::AddMoney(3.25))
            .execute(Command::Buy(CoffeeType::Cappuccino));

        assert_eq!(3.25, machine.balance);
        assert_eq!(
            Some(OperationResult::InsufficientBalance(-1.25)),
            machine.result
        )
    }
}
