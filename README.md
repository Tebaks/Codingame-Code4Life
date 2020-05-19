# Codingame Code4Life Challange

<details>
<summary>Click to expand</summary>

  - [Basic Functions](#basic-functions)
  - [Wood 2 to Wood 1](#wood-2-to-wood-1)
  - [Wood 1 to Bronz](#wood-1-to-bronze)
  - [Bronze to Silver](#bronze-to-silver)

</details>

# Basic Functions

# Wood 2 to Wood 1
I just store all samples and take sample with most score. And thats enough for rank up to Wood 1.

```rs
fn pickBestSample(player : &Player,availables: [i32;5]) -> Sample{
    let mut best = player.inventory[0].clone();
    let mut highest = 0;
    // Check every sample that I have.
        for x in 0..player.inventory.len(){
            // If I sample is makeable and have highest health.
            if(player.inventory[x].health > highest && canMake(&player,availables,player.inventory[x].clone())){
                // Change my best sample.
                best = player.inventory[x].clone();
                highest = player.inventory[x].health;
                }
                }
    // Return best sample.
    return best;
}
```

# Wood 1 to Bronz   
In this league now we should diogneise our samples so first I took 3 undiagnosed sample and diagnoise all.

# Bronze to Silver
In bronze I decided to play more aggresively and I create a attack function.

 ## Attack Function
I'm looking enemy robot's first sample and I check if I block it with only taking 1 molecule, if I can I take that molecule anyway.

 ## Choosing Sample
 I change my sample pick method.
 ```rs
 fn pickBestSampleV2(player:&Player,availables:[i32;5]) -> Sample{
    let mut best = player.inventory[0].clone();
    let mut max = 99;
        for x in 0..player.inventory.len() {
        if canMake(&player,availables,player.inventory[x].clone()){
            let mut temp = 0;
                for y in 0..5{
                    temp += player.inventory[x].cost[y];
                    }
                if temp < max {
                max = temp;
                best = player.inventory[x].clone();
                }
            }
        }
        return best;
}
```
Now I'm choosing the best sample by number of needed molecule.
And I take with minimum number.

## Taking Molecules
I create a neededMolecule function to choose least remaining molecule.
```rs
fn neededMolecule(cost : [i32;5],availables: [i32;5],player : &Player) -> usize{
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
```

## Choosing Rank
Also I create choosing rank system as my total expertise.
```rs
           if totalExpertise(players[0].expertise)+(players[0].inventory.len() as i32)< 4{
            collectSample(1, &players[0]);
            }else if totalExpertise(players[0].expertise) +(players[0].inventory.len() as i32) < 10 {
            collectSample(2,&players[0]);
            }else{
            collectSample(3,&players[0]);
            };
}
```
I change numbers a lot to fit my strategy. These three big change take me to silver. But mostly because of my attack system, it makes huge difference.

# Silver
In silver I update so many things.

## Taking Molecules
Now I'm taking molecules for completing mulptiple samples at once. And I change my molecule choosing system. Now I'm giving score to every molecule and take with most score.

```rs
fn calculateMoleculeScore(molecule : usize,enemy: &Player,availables: [i32;5]) -> i32{

    let totalEnemyNeed = calculateEnemyNeed(enemy, molecule);


    return totalEnemyNeed - availables[molecule];
}
```
I'm calculating the score as how many my enemy need and how much left.

## Attack Function
I update my attack function. Now I'm looking for every sample that my enemy has and if I can block it with taking 1,2,3 molecule I take it.
My robot became more aggrasive in late game.

## Choosing Sample
I update my choosing sample function. Now I'm giving score to every sample and I take with most score.

```rs
fn calculateSampleScore(sample : &Sample,player: &Player,availables: [i32;5],molecules:[i32;5]) -> i32{

    let gainAdvantage = calculateGainAdvantage(sample, player);
    let minNeed = calculateMinPickUp(sample,player);

    return 30 - minNeed + gainAdvantage;
}
```
I give score to sample as how many molecule need to complate and how much effect their expertise gain.

