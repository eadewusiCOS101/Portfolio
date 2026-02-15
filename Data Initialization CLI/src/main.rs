use std::io;
use std::fs::File;
use std::io::Write;

fn get_string_input(prompt:&str) -> String{
    println!("{}",prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read name");
    let result:String = input.trim().to_string();
    return result
}
fn get_date(prompt:&str) -> String{
    println!("{}",prompt);

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Invalid date");
    let result:String = input.trim().to_string().to_lowercase();
    return result
}

fn main(){
    println!("Data initialization software is now online.");
    let mut username:String = String::new();
    let mut email:String = String::new();
    let mut day = 0;
    let mut month = 0;
    let mut year = 0;

    let username = get_string_input("Enter your username.");

    let email_domain = vec!["gmail.com","yahoo.com","outlook.com"];
    let email = loop{
        let email_input = get_string_input("\nEnter your Email.");
        let parts_of_email:Vec<&str> = email_input.split('@').collect();

        if parts_of_email.len() != 2{
            println!("Invalid. Missing '@'.");
            continue;
        }

        let domain = parts_of_email[1];

        if email_input.contains('@') && email_domain.contains(&domain){
            break email_input
        }

        else{
            println!("Invalid email. Be sure to put an '@' and check spellings for the domain.");
        }
    };

    let age:i16 = loop{
        let age_input:i16 = get_string_input("\nEnter your age.").parse().expect("Invalid age.");
        if age_input >= 120{
        println!("Retry, possible typo");
        }
        else if age_input < 0{
            println!("Retry, Age connot be negetive")
        }
        else{
            break age_input;
        }
    
    };

    println!("\nWhen were you born?");
    loop{
        day = loop{
            let day_input:u8 = get_date("\nWhat day?").parse().expect("Couldn't get day");
            if day_input > 31{
                println!("Invalid day written");
            } 

            else{
                break day_input;
                }        
            };
        month = loop{
                let month_input = get_date("\nWhat month?");
                match month_input.as_str(){
                    "january" | "jan" => break 1,
                    "february" | "feb" => break 2,
                    "march" | "mar" => break 3,
                    "april" | "apr" => break 4,
                    "may" => break 5,
                    "june" | "jun" => break 6,
                    "july" | "jul" => break 7,
                    "august" | "aug" => break 8,
                    "september" | "sep" => break 9,
                    "october" | "oct" => break 10,
                    "november" | "nov" => break 11,
                    "december" | "dec" => break 12,
                    _ =>{ 
                    println!("Invalid month.");
                    continue;
                    }
                }
            };
        year = loop{
            let year_input:i128 =  get_string_input("\nWhat year?").parse().expect("Couldn't read Year.");
            if year_input > 1935 && year_input <= 2026{
                break year_input;
            }
            else if year_input < 0{
                println!("Invalid year. It cannot be negetive.")
            }
            else{
                println!("Invalid year input. 1935-2025");
            };
        };
        let leap_year_check = year % 4;
        let age_check = 2026 - year;
        
        if day == 31 && matches!(month, 4 |6 | 9 | 11){
            println!("Invalid. April, June, September and November only have 30 days.")
        }
        else if day > 29 && month == 2{
            println!("Invalid. February only has up to 29 days");
        }
        else if day == 29 && month == 2 && leap_year_check != 0{
            println!("Invalid. {} was not a leap year", year);
        }
        else if age_check != age.into(){
            println!("Invalid. The age doesn't match with your inputed birth year");
        }
        else{
            break;
        }
    };
    
    let info = format!("Username: {}
Email: {}, 
Age as of file creation: {}, 
Date of birth: {}/{}/{}",username,email,age,day,month,year);
    println!("{:?}",info);
    println!("File creation and data initialization pending...");

    let mut file = File::create("User Information.txt").expect("File creation failed.");
    file.write_all(info.as_bytes()).expect("Failed to read information into file.");
    println!("Information written successfully. File name: User Information.");

    
}

