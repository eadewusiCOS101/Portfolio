use std::io;

struct Bank_Account{
    name: String,
    account_number: String,
    balance: f64,
}

impl Bank_Account{
    fn withdrawal(&mut self, amount:f64){
        println!("Withdrawing ₦{} from the account", amount);
        self.balance -= amount;
    }
    fn debit(&mut self, amount:f64){
        println!("Debiting ₦{} to the account", amount);
        self.balance += amount;
    }
}

fn get_info(prompt:&str) -> String{
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let mut response:String = input.trim().to_string();
    return response
}

fn get_float_input(prompt:&str) -> f64{
    println!("{}", prompt);
    let mut response:f64 = 0.0; 
    loop{
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let mut result:String = input.trim().to_string();
        if let Ok(num) = result.parse::<f64>(){
            response = num;
            break response;
        }
        else{
            println!("Invalid input");
            println!("{}", prompt);
        }
    }
}

fn contains_number(s:&str) -> bool{
    s.chars().any(|c| c.is_numeric())
}
fn digits_only(s:&str) -> bool{
    s.chars().all(|c| c.is_ascii_digit())
}


fn main(){
    println!("Welcome to the Dominic Bank CLI Application");

    let mut name_insert = String::new();
    let mut account_number_insert = String::new();
    let mut balance_insert = 0.0;
    let mut amount:f64 = 0.0;
    let mut quit:String = String::new();
    let mut effect:&str = "";
    let mut quit0:String = String::new();

    loop{
        name_insert = loop{
            let mut name_input:String = get_info("Enter your account name");
            if contains_number(&name_input){
                println!("Invalid input, your name cannot have numbers.\n");
            }
            else{
                break name_input;
            }
        };

        account_number_insert = loop{
            let account_number_input = get_info("\nEnter your 9-didgit account number");
            if account_number_input.len() != 9{
                println!("Invalid, must be 9 digits long, got {}.\n", account_number_input.len());
                continue;
            }
            if !digits_only(&account_number_input){
                println!("Invalid, must not contain letters.\n");
            }
            else{
                break account_number_input;
            }
        };
        balance_insert = loop{
            let mut balance_input:f64 = get_float_input("\nEnter your account balance."); 
            break balance_input
        };

        let mut account = Bank_Account{
            name: name_insert,
            account_number: account_number_insert,
            balance: balance_insert,
        };

        loop{
            let mut operation = loop{
                let mut operation_input = get_info("\nWhat would you like to do? \n1) Withdraw.\n2) Debit.\n3) Check balance.");
                if operation_input != "1" && operation_input != "2" && operation_input != "3"{
                    println!("Invalid, Select the number code of the operation,1,2,3. For the respective operations.");
                }
                else{
                    break operation_input;
                    }
                };

            if operation == "1"{
                loop{
                    amount = get_float_input("How much do you want to withdraw?");
                    if amount > account.balance{
                        println!("Invalid, Balance is {:.2}.\n", account.balance);
                    }
                    else{
                        account.withdrawal(amount);
                        println!("Withdrawal successful, Balance is now {:.2}.\n", account.balance);
                        break;
                    }
                }
            }
            else if operation == "2"{
                loop{
                    amount = get_float_input("How much do you want to debit?");
                    if amount > 3000000.0 || amount < 500.0{
                        println!("Invalid, Debit floor is ₦500, Debit celing is ₦3,000,000\n");
                    }
                    else{
                        account.debit(amount);
                        println!("Debit successful, ₦{:.2} has now been added to your account, balance is {:.2}.\n", amount, account.balance);
                        break;
                    }
                }
            }
            else{
                println!("Account balance is ₦{:.2}", account.balance);
            }
            let quit0: String = loop{
                let mut quit_input0 = get_info("Do you want to end operations? y or n");
                if quit_input0 != "Y".to_string() && quit_input0 != "N".to_string() && quit_input0 != "y".to_string() && quit_input0 != "n".to_string(){
                    println!("Invalid, only y or n.\n");
                }
                else{
                    break quit_input0;
                }
            };
            let mut effect0:&str = "";
            if quit0 == "Y".to_string() || quit0 == "y".to_string(){
                effect0 = "kill";
            }
            if effect0 == "kill"{
                break;
            }
    }
        let quit: String = loop{
            let mut quit_input = get_info("Do you want to stop the application? y or n");
            if quit_input != "Y".to_string() && quit_input != "N".to_string() && quit_input != "y".to_string() && quit_input != "n".to_string(){
                println!("Invalid, only y or n.\n");
            }
            else{
                break quit_input;
            }
        };
        if quit == "Y".to_string() || quit == "y".to_string(){
            effect = "kill";
        }
        if effect == "kill"{
            println!("Thank you for using our banking application {}", account.name);
            break;
        }
    }
}