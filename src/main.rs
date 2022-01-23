use rand::Rng; // 0.8.0

#[derive(Debug)]
pub struct LotteryBox {
    pub participants: Vec<u32>,
    pub pool_size: f32,
    pub base_fee: f32
}
pub struct Manager {
    pub address: u32
}

#[derive(Debug)]
pub struct WinnerList {
    pub winners: Vec<u32>
}

pub fn process_instruction(program_id:bool, account: u32, action: &str, manager: &mut Manager, lottery_box: &mut LotteryBox, winners: &mut WinnerList){
    match action{
        // if init
        "new" => {
            if !program_id{
                println!("#### Unauthorised #####");
                return
            }
            println!("Init of program");
            *manager = Manager { address: account };
            println!("Manager is {}.", manager.address);

            *lottery_box = LotteryBox { participants: Vec::new(), pool_size: 0.0, base_fee: 0.01};
            println!("Lottery box state is {:?}.", lottery_box);
        },
        // if participate
        "take-part" => {
            println!("Participate in program");

            lottery_box.participants.push(account);
            lottery_box.pool_size += lottery_box.base_fee;
            println!("Lottery box state is {:?}.", lottery_box);
        },
        // if result time
        "result" => {
            if account != manager.address{
                println!("#### Unauthorised #####");
                return
            }
            println!("Result of program");
            let winner_index = rand::thread_rng().gen_range(0..lottery_box.participants.len());
            let winner = lottery_box.participants[winner_index];
            println!("Winner is {}.", winner);

            winners.winners.push(winner);
            println!("######################################");
            println!("Updated Winners List is {:?}.", winners);
            println!("######################################");

            lottery_box.participants = Vec::new();
            lottery_box.pool_size = 0.0;
            lottery_box.base_fee = 0.0;
            println!("Resetting Lottery Complete!");
        },
        _ => println!("Invalid Statement!")
    }
    
}

fn main() {
    let mut manager: Manager = Manager { address: 0 };
    let mut lottery_box: LotteryBox = LotteryBox { participants: Vec::new(), pool_size: 0.0, base_fee: 0.0};
    let mut winners: WinnerList = WinnerList { winners: Vec::new()};
    for _i in 0..5{
        process_instruction(true, 101, "new", &mut manager, &mut lottery_box, &mut winners); //init
        process_instruction(false, 102, "take-part", &mut manager, &mut lottery_box, &mut winners); //participate
        process_instruction(false, 103, "take-part", &mut manager, &mut lottery_box, &mut winners); //participate
        process_instruction(false, 104, "take-part", &mut manager, &mut lottery_box, &mut winners); //participate
        process_instruction(false, 105, "take-part", &mut manager, &mut lottery_box, &mut winners); //participate
        process_instruction(false, 106, "take-part", &mut manager, &mut lottery_box, &mut winners); //participate
        process_instruction(false, 107, "take-part", &mut manager, &mut lottery_box, &mut winners); //participate
        process_instruction(false, 108, "take-part", &mut manager, &mut lottery_box, &mut winners); //participate
        process_instruction(false, 109, "take-part", &mut manager, &mut lottery_box, &mut winners); //participate
        process_instruction(true, 101, "result", &mut manager, &mut lottery_box, &mut winners); //winner
    
    }
    
    

}
