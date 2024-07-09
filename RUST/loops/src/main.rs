use std::thread;
use std::time::Duration;

fn main() {
    let mut count: u32 = 1;

    println!();
    thread::sleep(Duration::from_millis(500));
    println!("...Noises of people fucking...");
    thread::sleep(Duration::from_millis(500));
    println!();
    thread::sleep(Duration::from_millis(500));

    let you = loop {
        println!("Hello, world!");
        thread::sleep(Duration::from_millis(500));
        if count == 3 {
            println!("The world has been saluted exactly {count} times. Kudos, you're born now.");
            thread::sleep(Duration::from_millis(500));
            break "sucker"; // nice way to return something lmao, ; optional
        }
        count += 1;
    };
    println!("So now Hello to you, {you}.");
    thread::sleep(Duration::from_millis(500));

    // I find confusing the example on doc so I do a better one

    let mut fucked_over = 2;
    'counting_loop: loop {
        println!("Welcome in the Hell's Loop sucker");
        thread::sleep(Duration::from_millis(500));

        let mut remaining = 10;

        'life: loop {
            println!("You have {remaining} lives remaining.");
            thread::sleep(Duration::from_millis(500));

            if remaining == 0 {
                let mut jesus_help = 3;
                println!("Jesus had pity of you and gave you {jesus_help} extra lives.");
                thread::sleep(Duration::from_millis(700));
                println!("Btw don't fuck it up bruta.");
                thread::sleep(Duration::from_millis(500));
                'Jesus: loop {
                    if jesus_help == 3 {
                        println!("Fuck! You lose one life all in on Solana.");
                    } else if jesus_help == 2 {
                        println!("Fuck! You went long ADA at its shitty ATH.");
                    } else if jesus_help == 1 {
                        println!("Fuck! You mortgaged you house for BTC before a 95% small dip.");
                    }
                    thread::sleep(Duration::from_millis(700));
                    jesus_help -= 1;
                    if jesus_help == 0 {
                        println!("Not even Jesus could save a crypto bro.");
                        thread::sleep(Duration::from_millis(500));
                        break 'Jesus;
                    }
                }
            } else if remaining == -1 {
                println!("YOU GOT A FUCKING EXTRA LIFE BRO");
                thread::sleep(Duration::from_millis(500));
                println!("DON'T FUCK IT UP");
                thread::sleep(Duration::from_millis(500));
                println!("omg you went long pepe and get a 5000% gain");
                thread::sleep(Duration::from_millis(1000));
                println!("You ARE A BILLIONARE!");
                thread::sleep(Duration::from_millis(2000));
                println!("...of fuck...");
                thread::sleep(Duration::from_millis(500));
                println!("...of fuck...");
                thread::sleep(Duration::from_millis(500));
                println!("You forgot your seed phrase and pin");
                thread::sleep(Duration::from_millis(500));
                println!(".....");
                thread::sleep(Duration::from_millis(500));
                println!("You run on the street in desperation");
                thread::sleep(Duration::from_millis(500));
                println!("A Lamborghini smashes you on the ground.");
                thread::sleep(Duration::from_millis(1000));
                println!("...you die...");
                thread::sleep(Duration::from_millis(500));
                println!("...poor like you deserve..");
                thread::sleep(Duration::from_millis(500));
                break 'life;
            }
            remaining -= 1;
            println!("You lose all your money in crypto");
            thread::sleep(Duration::from_millis(500));
            println!("Back to the mine");
            thread::sleep(Duration::from_millis(500));
        }
        if fucked_over == 0 {
            println!("Congratulations, you're a true crypto bro");
            thread::sleep(Duration::from_millis(500));
            println!("hodl.");
            thread::sleep(Duration::from_millis(500));
            break 'counting_loop;
        }
        fucked_over -= 1;
        println!("You're a true crypto bro.");
        thread::sleep(Duration::from_millis(500));
        println!("If you are born again you will receive the same fate.");
        thread::sleep(Duration::from_millis(500));
        println!();
        thread::sleep(Duration::from_millis(500));
        println!("...Noises of people fucking...");
        thread::sleep(Duration::from_millis(500));
        println!();
        thread::sleep(Duration::from_millis(500));
    }
    println!("Coded in RUST");
}
