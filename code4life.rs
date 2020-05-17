use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

#[derive(Clone)]
struct Sample {
    id: i32,
    carriedBy: i32,
    rank: i32,
    gain: String,
    health: i32,
    cost: [i32; 5],
}

struct Player {
    target: String,
    molecules: [i32; 5],
    expertise: [i32; 5],
    inventory: Vec<Sample>,
}

fn isBlocked(sample:Sample,player:&Player,availables:[i32;5]) -> bool{
    for x in 0..5 {
        if (sample.cost[x] - player.molecules[x] - player.expertise[x] -availables[x] >0) {
            eprintln!("blocklanmış");
            return true;
            }
        }
        eprintln!("blocklanmamış");
        return false;
}
fn minNeededNumberToBlock(sample: Sample, player:&Player,availables:[i32;5]) -> i32 {
    let mut minNeed = 99;
        for x in 0..5 {
            if (sample.cost[x] - player.molecules[x] -player.expertise[x]) > 0{
                if ((availables[x] - (sample.cost[x] - player.molecules[x] -player.expertise[x])) +1) < minNeed{
                    minNeed = ((availables[x] - (sample.cost[x] - player.molecules[x] -player.expertise[x])) +1);
                    }
                }
            }
        eprintln!("en az {}",minNeed);
        return minNeed;
}
fn minNeededMoleculeToBlock(sample: Sample,player:&Player,availables:[i32;5]) -> usize{
    let mut minNeed = 99;
    let mut minNeededMolecule = 0;
        for x in 0..5 {
            if (sample.cost[x] - player.molecules[x] -player.expertise[x]) > 0{
                if ((availables[x] - (sample.cost[x] - player.molecules[x] -player.expertise[x]))) < minNeed && availables[x] > 0{
                    minNeededMolecule = x as i32;
                    }
                }
            }
        return minNeededMolecule as usize;
}
                    

fn pickBestSample(player : &Player,availables: [i32;5]) -> Sample{
    let mut best = player.inventory[0].clone();
    let mut highest = 0;
        for x in 0..player.inventory.len(){
            if(player.inventory[x].health > highest && canMake(&player,availables,player.inventory[x].clone())){
                best = player.inventory[x].clone();
                highest = player.inventory[x].health;
                }
                }
    return best;
}
fn pickBestSampleV2(player:&Player,availables:[i32;5]) -> Sample{
    let mut best = player.inventory[0].clone();
    let mut max = 99;
        for x in 0..player.inventory.len() {
        if canMake(&player,availables,player.inventory[x].clone()){
                let mut temp = getSampleTotalNeededMolecule(&player.inventory[x], player);
                
                if temp < max {
                max = temp;
                best = player.inventory[x].clone();
                }
            }
        }
        return best;
}
fn basicAttack(player:&Player,availables:[i32;5]) -> usize{
        for x in 0..2 {
            if player.molecules[x] == 0 && availables[x] != 0{
                return x;
                }
            }
        return 5;
}
            
fn canMake(player: &Player,availables:[i32;5],sample:Sample) -> bool{
            if getPlayerTotalMolecule(&player) + getSampleTotalNeededMolecule(&sample,&player) > 10{
            return false;
            }
         for x in 0..5{
           if  (sample.cost[x] - player.molecules[x]-availables[x]-player.expertise[x] > 0){
           return false;
           }
           }
           return true;
}

fn getPlayerTotalMolecule (player:&Player) ->i32{
        let mut total = 0;
        for x in 0..5{
            total += player.molecules[x];
            }
        return total;
}
fn getSampleTotalNeededMolecule (sample :&Sample,player:&Player) ->i32{
        let mut total = 0;
        for x in 0..5{
            if(sample.cost[x]- player.expertise[x] >= 0){
            total += sample.cost[x] - player.expertise[x];
            }
        }
        return total;
}
            
        
        
        
fn getNeededMolecule(cost : [i32;5],availables: [i32;5],player : &Player) -> usize{
    let mut lessLeft = 99;
    let mut need = 5;
    for x in 0..5{
        if cost[x]-player.molecules[x]-player.expertise[x] > 0 && availables[x] < lessLeft{
        lessLeft = availables[x];
        need = x;
        }
        }
    return need;
        
        
}


fn checkForDiagnosis(samples: &Vec<Sample>) -> Sample{
    for x in 0..samples.len(){
        if samples[x].health == -1{
            return samples[x].clone();
            }
        }
    return samples[0].clone();
}

fn goDiagnosis(id: i32, player: &Player) {
    if player.target == "DIAGNOSIS" {
        println!("CONNECT {}", id);
    } else {
        println!("GOTO DIAGNOSIS");
    }
}
fn goSamples(rank: i32, player: &Player) {
    if player.target == "SAMPLES" {
        println!("CONNECT {}", rank);
    } else {
        println!("GOTO SAMPLES");
    }
}
fn goMolecules(need: i32, player: &Player) {
    if player.target == "MOLECULES" {
        println!("CONNECT {}", numberToLetter(need));
    } else {
        println!("GOTO MOLECULES");
    }
}
fn goLaboratory(id: i32, player: &Player) {
    if player.target == "LABORATORY" {
        println!("CONNECT {}", id);
    } else {
        println!("GOTO LABORATORY");
    }
}
fn getPlayerTotalExpertise(expertises:[i32;5]) ->i32{
    let mut total = 0;
    for x in 0..5{
        total += expertises[x];
        }
        return total as i32;
}
        

fn numberToLetter(number: i32) -> char {
    match number {
        0 => 'A',
        1 => 'B',
        2 => 'C',
        3 => 'D',
        4 => 'E',
        _ => 'F',
    }
}

/**
 * Bring data on patient samples from the diagnosis machine to the laboratory with enough molecules to produce medicine!
 **/
fn main() {
    let mut fulled = false; 
    let mut firstRound = true;
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let project_count = parse_input!(input_line, i32);
    for _ in 0..project_count as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let a = parse_input!(inputs[0], i32);
        let b = parse_input!(inputs[1], i32);
        let c = parse_input!(inputs[2], i32);
        let d = parse_input!(inputs[3], i32);
        let e = parse_input!(inputs[4], i32);
    }

    // game loop
    loop {
        let mut samples = Vec::new();
        let mut my_molecules: [i32; 5];
        let mut players: Vec<Player> = Vec::new();
        for i in 0..2 as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(" ").collect::<Vec<_>>();
            let target = inputs[0].trim().to_string();
            let eta = parse_input!(inputs[1], i32);
            let score = parse_input!(inputs[2], i32);
            let storage_a = parse_input!(inputs[3], i32);
            let storage_b = parse_input!(inputs[4], i32);
            let storage_c = parse_input!(inputs[5], i32);
            let storage_d = parse_input!(inputs[6], i32);
            let storage_e = parse_input!(inputs[7], i32);
            let expertise_a = parse_input!(inputs[8], i32);
            let expertise_b = parse_input!(inputs[9], i32);
            let expertise_c = parse_input!(inputs[10], i32);
            let expertise_d = parse_input!(inputs[11], i32);
            let expertise_e = parse_input!(inputs[12], i32);
            let player = Player {
                target: target,
                molecules: [storage_a, storage_b, storage_c, storage_d, storage_e],
                expertise: [expertise_a,expertise_b,expertise_c,expertise_d,expertise_e],
                inventory: Vec::new(),
            };
            players.push(player);
        }
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let available_a = parse_input!(inputs[0], i32);
        let available_b = parse_input!(inputs[1], i32);
        let available_c = parse_input!(inputs[2], i32);
        let available_d = parse_input!(inputs[3], i32);
        let available_e = parse_input!(inputs[4], i32);
        let availables : [i32;5] = [available_a,available_b,available_c,available_d,available_e];
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let sample_count = parse_input!(input_line, i32);
        for _ in 0..sample_count as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(" ").collect::<Vec<_>>();
            let sample_id = parse_input!(inputs[0], i32);
            let carried_by = parse_input!(inputs[1], i32);
            let rank = parse_input!(inputs[2], i32);
            let expertise_gain = inputs[3].trim().to_string();
            let health = parse_input!(inputs[4], i32);
            let cost_a = parse_input!(inputs[5], i32);
            let cost_b = parse_input!(inputs[6], i32);
            let cost_c = parse_input!(inputs[7], i32);
            let cost_d = parse_input!(inputs[8], i32);
            let cost_e = parse_input!(inputs[9], i32);
            let cost: [i32; 5] = [cost_a, cost_b, cost_c, cost_d, cost_e];
            let sample = Sample {
                id: sample_id,
                carriedBy: carried_by,
                rank: rank,
                gain: expertise_gain,
                health: health,
                cost: cost,
            };
            samples.push(sample.clone());
            if carried_by > -1 {
                players[carried_by as usize].inventory.push(sample);
            }
        }
        let me = &players[0];
        let enemy = &players[1];
        
        
        if me.inventory.len() == 0{
            fulled = false;
            }
        let mut total = 3;
        if firstRound {
            total = 2;
            }
        if me.inventory.len() < total && !fulled {
            if getPlayerTotalExpertise(me.expertise)+(me.inventory.len() as i32)< 4{
            goSamples(1,&me);
            }else if getPlayerTotalExpertise(me.expertise) +(me.inventory.len() as i32) < 10 {
            goSamples(2,&me);
            }else{
            goSamples(3,&me);
            };
            if me.inventory.len() == (total -1){
            fulled = true;
            firstRound = false;
                }
            
        } else {
            let mut sample = checkForDiagnosis(&me.inventory);
            if  sample.health != -1{
                sample = pickBestSampleV2(&me,availables);
        
                let mut need = getNeededMolecule(sample.cost,availables,&me);
                if need != 5{
                     if ((enemy.inventory.len() != 0 && minNeededNumberToBlock(me.inventory[0].clone(),&enemy,availables) < 2) && !(isBlocked(enemy.inventory[0].clone(),&enemy,availables)) && getPlayerTotalMolecule(&me)<10){
                        eprintln!("Trying to block");
                        need = minNeededMoleculeToBlock(enemy.inventory[0].clone(),&enemy,availables);
                        goMolecules(need as i32, &me);
                        }else{
                        if !canMake(&me,availables,sample.clone()) || getPlayerTotalMolecule(&me) >=10{
                        goDiagnosis(sample.id,&me);
                        }else{
                    goMolecules(need as i32, &me);
                    
                    };
                    }
                    
                } else {
                    goLaboratory(sample.id, &me);
                }
            } else {
                goDiagnosis(sample.id, &me);
            }
        }

        // Write an action using println!("message...");
        // To debug: eprintln!("Debug message...");
    }

}
