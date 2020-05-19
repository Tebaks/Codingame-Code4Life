use std::io;
use std::cell::Cell;

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
    isDone: Cell<bool>,
}

struct Player {
    target: String,
    molecules: [i32; 5],
    remainMolecules: [i32;5],
    expertise: [i32; 5],
    inventory: Vec<Sample>,
}
// Check if sample is blocked or not.
fn isBlocked(sample:Sample,player:&Player,availables:[i32;5]) -> bool{
    for x in 0..5 {
        if (sample.cost[x] - player.molecules[x] - player.expertise[x] -availables[x] >0) {
            return true;
            }
        }
        return false;
}
// find minimum number of needed molecule to block sample.
fn minNeededNumberToBlock(sample: Sample, player:&Player,availables:[i32;5]) -> i32 {
    let mut minNeed = 99;
        for x in 0..5 {
            if (sample.cost[x] - player.molecules[x] -player.expertise[x]) > 0{
                if ((availables[x] - (sample.cost[x] - player.molecules[x] -player.expertise[x]))) < minNeed{
                    minNeed = ((availables[x] - (sample.cost[x] - player.molecules[x] -player.expertise[x])) );
                    }
                }
            }
        return minNeed;
}
// find molecule which is least needed to block sample
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
// find find minimum number of needed molecule to block ALL samples.
fn minNeededNumberToBlockAll (player:&Player,availables:[i32;5]) -> i32{
       let mut minNeed = 99;

       for x in 0..player.inventory.len(){
           let min = minNeededNumberToBlock(player.inventory[x].clone(),player,availables);
           if min < minNeed{
               minNeed = min;
           }
       }
       return minNeed;
}
//find molecule which is minimum number of needed to block sample.
fn minNeededMoleculeToBlockAll(player:&Player,availables:[i32;5]) -> usize{
    let sample = minNeededSampleToBlockAll(player,availables);
    
    return minNeededMoleculeToBlock(sample,player,availables);
}


fn minNeededSampleToBlockAll(player:&Player,availables:[i32;5])->Sample{

let mut sample =  player.inventory[0].clone();
    let mut minNeed = 99;
    for x in 0..player.inventory.len(){
           let min = minNeededNumberToBlock(player.inventory[x].clone(),player,availables);
           if min < minNeed{
               minNeed = min;
               sample = player.inventory[x].clone();
           }
       }

return sample
}
// find total number molecule to block all samples.
fn totalNeededNumberToBlockAll(player:&Player,availables:[i32;5]) -> i32{
    let mut totalNeed = 0;

    for x in 0..player.inventory.len(){
        totalNeed += minNeededNumberToBlock(player.inventory[x].clone(),player,availables);
    }

    return totalNeed;
}
                    
/* old version of pick best sample.
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
}*/
//pick best sample with least molecule
fn pickBestSampleV2(player:&Player,availables:[i32;5],molecules:[i32;5]) -> Option<Sample>{
    let mut best = player.inventory[0].clone();
    let mut max = 99;
    let samples = getMakeableSamples(player.inventory.clone(),availables,player,molecules);
    if samples.len() > 0{
    }
    
    if samples.len() == 0 {
        return None;
    }
        for x in 0..samples.len() {
            if(samples[x].health != -1){
                let letter : Vec<char> = samples[x].gain.chars().collect();
                let mut temp = getSampleTotalNeededMolecule(&samples[x], player,molecules);
                if temp < max && !&samples[x].isDone.get(){
                max = temp;
                best = samples[x].clone();
                }
            }
}
        return Some(best);
}
// check if sample can madeable or not  
fn canMake(player: &Player,availables:[i32;5],sample:Sample,molecules:[i32;5]) -> bool{
      let doneSamples = getDoneSamples(player);
    let mut gainMolecules = [0,0,0,0,0];
    for y in 0..doneSamples.len(){
        if(doneSamples[y].health != -1){
        let letter : Vec<char> = doneSamples[y].gain.chars().collect();
        gainMolecules[letterToNumber(letter[0])] += 1;
        }
    }
            
            if (getPlayerTotalMolecule(&player) + getSampleTotalNeededMolecule(&sample,&player,molecules)) > 10{
            return false;
            }
         for x in 0..5{
           if  (sample.cost[x] - (molecules[x] +availables[x] + player.expertise[x] + gainMolecules[x])) > 0  {
           return false;
           }
           }
           return true;
}
// self-explain
fn getPlayerTotalMolecule (player:&Player) ->i32{
        let mut total = 0;
        for x in 0..5{
            total += player.molecules[x];
            }
        return total;
}
// self-explain
fn getSampleTotalNeededMolecule (sample :&Sample,player:&Player,molecules:[i32;5]) ->i32{
        let mut total = 0;
          let doneSamples = getDoneSamples(player);
    let mut gainMolecules = [0,0,0,0,0];
    for y in 0..doneSamples.len(){
        if(doneSamples[y].health != -1){
        let letter : Vec<char> = doneSamples[y].gain.chars().collect();
        gainMolecules[letterToNumber(letter[0])] += 1;
        }
    }
        for x in 0..5{
            if(sample.cost[x]- player.expertise[x]-molecules[x] -gainMolecules[x] >= 0){
            total += sample.cost[x] - player.expertise[x] - molecules[x] - gainMolecules[x];
            }
        }
        return total;
}
// find sample with most score.
fn pickBestSampleV3(player:&Player,availables :[i32;5],molecules:[i32;5],enemy:&Player) -> Option<Sample>{
    let mut best = player.inventory[0].clone();
    let mut max = -99;
    let samples = getMakeableSamples(player.inventory.clone(),availables,player,molecules);
    if samples.len() > 0{
    }
    
    if samples.len() == 0 {
        return None;
    }
    if enemy.target != "MOLECULES" && player.target == "MOLECULES"{
        let mut max = -99;
        let mut maxSample = samples[0].clone();
        for y in 0..samples.len(){
            let totalNeed = getSampleTotalNeededMolecule(&samples[y],player,molecules);
            if totalNeed > max{
                max =totalNeed;
                maxSample = samples[y].clone();
            }
        }
        return Some(maxSample);
    }
        for x in 0..samples.len() {
            if(samples[x].health != -1){
                let mut temp = calculateSampleScore(&samples[x],player,availables,molecules);
                eprintln!("score = {}", temp);
                if temp > max && !&samples[x].isDone.get(){
                max = temp;
                best = samples[x].clone();
                }
            }
}
    
        return Some(best);
}
// calculate sample's score.
fn calculateSampleScore(sample : &Sample,player: &Player,availables: [i32;5],molecules:[i32;5]) -> i32{

    let needError = calculateNeedError(sample,player);
    let gainAdvantage = calculateGainAdvantage(sample, player);
    let minNeed = calculateMinPickUp(sample,player);

    return 30 - minNeed + gainAdvantage;
}
// calculate minimum need for sample
fn calculateMinPickUp(sample:&Sample,player:&Player) -> i32{
    let mut minNeed = 0;

    for x in 0..5{
        if(sample.cost[x] > player.expertise[x]){
            minNeed += sample.cost[x] - player.expertise[x];
        }
    }

    return minNeed;
}
// find servable sample with only expertise and player.molecule
fn getReadyToServeSample(samples:Vec<Sample>,player:&Player)->Sample{
    let mut sample = samples[0].clone();


    for x in 0..samples.len(){
        let mut canMake = true;
        for y in 0..5{
            if(samples[x].cost[y] - player.molecules[y] - player.expertise[y] > 0 ){
                canMake = false;
            }
        }
        if canMake{
            return samples[x].clone();
        }
    }
    return sample;

}

// calculate advantage coming with sample's expertise gain
fn calculateGainAdvantage(sample:&Sample,player:&Player) ->i32{
     let letter : Vec<char> = sample.gain.chars().collect();
     let gain = letterToNumber(letter[0]);

    let mut advantage = 0;
    for x in 0..player.inventory.len(){
        if(sample.id != player.inventory[x].id){
            if(player.inventory[x].cost[gain] > player.expertise[gain]){
                advantage += 1;
            }
        }
    }
    return advantage;

}
            
        
        
// Old version of find needed molecule to complate sample
fn getNeededMolecule(cost : [i32;5],availables: [i32;5],player : &Player,molecules:[i32;5]) -> usize{
    let mut lessLeft = 99;
    let mut need = 5;
    let doneSamples = getDoneSamples(player);
    let mut gainMolecules = [0,0,0,0,0];
    for y in 0..doneSamples.len(){
        let letter : Vec<char> = doneSamples[y].gain.chars().collect();
        gainMolecules[letterToNumber(letter[0])] += 1;
    }
    for x in 0..5{
        if cost[x]-molecules[x]-player.expertise[x]-gainMolecules[x] > 0 && availables[x] < lessLeft && !(availables[x] <= 0){
        lessLeft = availables[x];
        need = x;
        }
        }
    return need;
        
        
}
// find needed molecule with most score to complate sample
fn getNeededMoleculeV2(cost : [i32;5],availables: [i32;5],player : &Player,enemy:&Player,molecules:[i32;5]) -> usize{
    let mut bestScore = -99;
    let mut need = 5;
    let doneSamples = getDoneSamples(player);
    let mut gainMolecules = [0,0,0,0,0];
    for y in 0..doneSamples.len(){
        let letter : Vec<char> = doneSamples[y].gain.chars().collect();
        gainMolecules[letterToNumber(letter[0])] += 1;
    }
    for x in 0..5{
        if cost[x]-molecules[x]-player.expertise[x]-gainMolecules[x] > 0 && !(availables[x] <= 0){
            let moleculeScore = calculateMoleculeScore(x,enemy,availables);
            if(moleculeScore > bestScore){
                bestScore = moleculeScore;
                need = x;
            }
        }
        }
    return need;
        
        
}

//self-explain
fn calculateMoleculeScore(molecule : usize,enemy: &Player,availables: [i32;5]) -> i32{

    let totalEnemyNeed = calculateEnemyNeed(enemy, molecule);


    return totalEnemyNeed - availables[molecule];
}

// calculate how many enemy need this molecule
fn calculateEnemyNeed(enemy:&Player,molecule:usize) -> i32{
    let mut totalEnemyNeed = 0;
    for x in 0..enemy.inventory.len(){
        let enemyNeed =enemy.inventory[x].cost[molecule]-enemy.molecules[molecule]-enemy.expertise[molecule];
        if( enemyNeed > 0){
            totalEnemyNeed += enemyNeed;
        }
    }

    return totalEnemyNeed;
}
// find all makeable samples
fn getMakeableSamples(samples: Vec<Sample>,availables: [i32;5], player: &Player,molecules:[i32;5]) -> Vec<Sample>{
    let mut makeableSamples = Vec::new();
    for x in 0..samples.len(){
        if canMake(player,availables,samples[x].clone(),molecules){
            makeableSamples.push(samples[x].clone());
        }
    }
    return makeableSamples;
}

// check if sample need diagnosis
fn checkForDiagnosis(samples: &Vec<Sample>) -> Sample{
    for x in 0..samples.len(){
        if samples[x].health == -1{
            return samples[x].clone();
            }
        }
    return samples[0].clone();
}
// go or connect diagnosis
fn goDiagnosis(id: i32, player: &Player) {
    if player.target == "DIAGNOSIS" {
        println!("CONNECT {}", id);
    } else {
        println!("GOTO DIAGNOSIS");
    }
}
// go or connect samples
fn goSamples(rank: i32, player: &Player) {
    if player.target == "SAMPLES" {
        println!("CONNECT {}", rank);
    } else {
        println!("GOTO SAMPLES");
    }
}
// go or connect molecules
fn goMolecules(need: i32, player: &Player) {
    if player.target == "MOLECULES" {
        println!("CONNECT {}", numberToLetter(need));
    } else {
        println!("GOTO MOLECULES");
    }
}
// go or connect laboratory
fn goLaboratory(id: i32, player: &Player) {
    if player.target == "LABORATORY" {
        println!("CONNECT {}", id);
    } else {
        println!("GOTO LABORATORY");
    }
}
//get players total expertise
fn getPlayerTotalExpertise(expertises:[i32;5]) ->i32{
    let mut total = 0;
    for x in 0..5{
        total += expertises[x];
        }
        return total as i32;
}
// get remain molecules with removing done samples
fn getRemainMolecules(player:&Player)->[i32;5] { 
    let mut molecules = player.molecules;
    for x in 0..player.inventory.len(){
        if (player.inventory[x].isDone.get()){
            for y in 0..5{
                if player.inventory[x].cost[y] != 0{
                if player.expertise[y] >= player.inventory[x].cost[y] {
                    molecules[y] = molecules[y];
                }else if ((molecules[y] + player.expertise[y] - player.inventory[x].cost[y]) >= 0){
                molecules[y] = molecules[y] + player.expertise[y] - player.inventory[x].cost[y];
                }else {
                molecules[y]  = 0;
                }
                }
                
            }
        }
    }
    return molecules;
}
// self-explain
fn setSamplesDone(player:&Player,availables:[i32;5],molecules:[i32;5],enemy:&Player){
    if(player.inventory.len() >0){
    let mut sample = player.inventory[0].clone();
    let mut isNone =false;
    for x in 0..player.inventory.len(){
        let molecules = getRemainMolecules(player);

        let bestSample = pickBestSampleV3(player,availables,molecules,enemy);

        match bestSample{
                    Some(x) =>{
                        sample = x;
                        isNone =false;
                    },
                    None => isNone = true,
                }
        if(!isNone){
        let mut isDone = checkSampleDone(player,sample.cost,molecules);
        if(isDone){
            for y in 0..player.inventory.len(){
                if sample.id == player.inventory[y].id{
                    player.inventory[y].isDone.set(true);
                }
            }
        }else{
            if player.inventory.len() == 2{
                for c in 0..player.inventory.len(){
                    if(player.inventory[c].id != sample.id){
                        sample = player.inventory[c].clone();
                    }
                }
            }
            isDone = checkSampleDone(player,sample.cost,molecules);
        if(isDone){
            for y in 0..player.inventory.len(){
                if sample.id == player.inventory[y].id{
                    player.inventory[y].isDone.set(true);
                }
            }
        }

        }
        }
    }
    }
}
// check if sample is done or not
fn checkSampleDone(player:&Player,cost:[i32;5],molecules:[i32;5]) -> bool{
    let doneSamples = getDoneSamples(player);
    let mut gainMolecules = [0,0,0,0,0];
    for y in 0..doneSamples.len(){
        if(doneSamples[y].health != -1){
        let letter : Vec<char> = doneSamples[y].gain.chars().collect();
        gainMolecules[letterToNumber(letter[0])] += 1;
        }
    }
    for x in 0..5{
        if cost[x] - molecules[x]- gainMolecules[x] - player.expertise[x] > 0 {
            return false;
        }
    }
    return true;

}

// find all done samples
fn getDoneSamples(player:&Player)-> Vec<Sample>{
    let mut doneSamples = Vec::new();
    for x in 0..player.inventory.len(){
        if(player.inventory[x].isDone.get()){
            doneSamples.push(player.inventory[x].clone());
        }
    }
    
    return doneSamples;
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
fn letterToNumber(letter: char) -> usize {
    match letter {
         'A' => 0,
         'B' => 1,
         'C' => 2,
         'D' => 3,
         'E' => 4,
          _  => 5,
    }
}

/**
 * Bring data on patient samples from the diagnosis machine to the laboratory with enough molecules to produce medicine!
 **/
fn main() {
    let mut fulled = false; 
    let mut firstRound = true;
    let mut raund = 0;
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
            // create players and add them into list
            let player = Player {
                target: target,
                molecules: [storage_a, storage_b, storage_c, storage_d, storage_e],
                remainMolecules : [storage_a, storage_b, storage_c, storage_d, storage_e],
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
        // store every available molecule
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
            // create samples and add them in to list
            let sample = Sample {
                id: sample_id,
                carriedBy: carried_by,
                rank: rank,
                gain: expertise_gain,
                health: health,
                cost: cost,
                isDone: Cell::new(false),
            };
            samples.push(sample.clone());
            if carried_by > -1 {
                players[carried_by as usize].inventory.push(sample);
            }
        }
        raund += 1;

        let rankOrder = [1,1,1,1,1,1,1,2,2,1,2,2,2,3,2,3,2,3,2,3,3,2,3,3,2,3,2,3,2,3,2,3];
    
        let me = &players[0];
        let enemy = &players[1];
        let mut totalSample = 3;
        if firstRound {
            totalSample = 2;
            }
        if me.inventory.len() == (totalSample){
            fulled = true;
            firstRound = false;
            }else if me.inventory.len() == 0{
            fulled = false;
            }
        // Set samples done
        setSamplesDone(&me,availables,me.molecules,&enemy);
        // get remain molecules using done samples
        let remainMolecules = getRemainMolecules(&me);
        let doneSamples = getDoneSamples(&me);
        let totalNeed = totalNeededNumberToBlockAll(enemy,availables);
        // if I'm at laboratory and if I have done samples give it all
        if(doneSamples.len() > 0 && &me.target == "LABORATORY"){
            let serveSample = getReadyToServeSample(doneSamples, &me);
            goLaboratory(serveSample.id, &me);
        }else{
            // if my hand doesn't full get samples using my rank system
            if !fulled {
                
                let rankCount = getPlayerTotalExpertise(me.expertise)+(me.inventory.len() as i32);
                goSamples(rankOrder[rankCount as usize],&me);
                
            /* This is my old system
            if getPlayerTotalExpertise(me.expertise)+(me.inventory.len() as i32)< 12{
            goSamples(1,&me);
            }else if getPlayerTotalExpertise(me.expertise) +(me.inventory.len() as i32) < 18 {
            goSamples(2,&me);
            }else{
            goSamples(3,&me);
            }; 
            */
            
            
        } else {
            // check for diagnosis
            let mut sample = checkForDiagnosis(&me.inventory);
            // if sample health is -1 it meain we should diagnose it
            if  sample.health != -1{
                let mut isNone = false;
                // pick best sample
                let result = pickBestSampleV3(&me,availables,remainMolecules,&enemy);
                match result{
                    Some(x) => sample = x,
                    None => isNone = true,
                }
                // if best sample is null and there is no done sample we should take new samples or go diagnoise to release our samples
                if isNone && doneSamples.len() == 0 {
                    if &me.inventory.len() < &(3 as usize){
                        fulled = false;
                        goSamples(2,&me);
                    }else{
                        goDiagnosis(me.inventory[0].id,&me);
                    }               
                }else{
                // find needed molecule
                let mut need = getNeededMoleculeV2(sample.cost,availables,&me,&enemy,remainMolecules);
                
                if need != 5{
                    let mut blockNumber = 2;
                    let playerTotalExpertise = getPlayerTotalExpertise(me.expertise);
                    if  playerTotalExpertise > 16 {
                        blockNumber = 2;
                    }else if playerTotalExpertise > 13{
                        blockNumber = 2;
                    }else if playerTotalExpertise > 9 {
                        blockNumber = 2;
                    }
                    // if minimum number needed to block is less than blockNumber go block enemy.
                     if ((enemy.inventory.len() != 0 && minNeededNumberToBlockAll(&enemy,availables) < blockNumber) && !(isBlocked(minNeededSampleToBlockAll(&enemy,availables),&enemy,availables)) && getPlayerTotalMolecule(&me)<10){
                        eprintln!("Trying to block");
                        need = minNeededMoleculeToBlockAll(&enemy,availables);
                        goMolecules(need as i32, &me);
                        }else{
                        // if sample is done go laboratory
                        if(sample.isDone.get()){
                            goLaboratory(doneSamples[0].id, &me)
                        }else{
                            // else check if we can make or not
                            if !canMake(&me,availables,sample.clone(),remainMolecules) || getPlayerTotalMolecule(&me) >=10{
                            // if we can't make and we have done samples go laboratory to take point.
                        if doneSamples.len() > 0 {
                            goLaboratory(doneSamples[0].id, &me)
                        }else{
                            // else it means we should give our samples
                            goDiagnosis(sample.id, &me)
                        }
                        }else{
                            // if we are at laboratory
                            if &me.target == "LABORATORY"{
                                // in late game we should finish our all samples
                                if(raund > 140 && &me.inventory.len() > &(1 as usize) || raund > 160){
                                    goMolecules(need as i32, &me)
                                }else{
                                // but in early game we can go samples to take new ones
                                    fulled = false;
                                    goSamples(2,&me);
                                }
                           

                            }else{
                        // go molecules to finish our best sample
                        goMolecules(need as i32, &me);   
                            }              
                    };  
                        }
                }
                                    
                } else { 
                    if doneSamples.len() > 0 {
                            goLaboratory(doneSamples[0].id, &me)
                        }else{
                            if &me.target == "LABORATORY"{
                                if(raund > 140 && &me.inventory.len() > &(1 as usize) || raund > 160){
                                    goMolecules(need as i32, &me)
                                }else{
                                    fulled = false;
                                    goSamples(2,&me);  
                                }

                            }else{
                            goMolecules(need as i32, &me);   
                            } 
                        }
                     
                }
                }
            } else {
                goDiagnosis(sample.id, &me);
            }
        }
        }
    
        
        

        // Write an action using println!("message...");
        // To debug: eprintln!("Debug message...");
    }

}
