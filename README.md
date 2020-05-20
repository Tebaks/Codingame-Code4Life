# Codingame Code4Life Challenge
# Tebaks - Kenan Abbak
#### Silver #2/506 
#### Total #403

<details>
<summary>Click to expand</summary>

  - [Wood 2 to Wood 1](#wood-2-to-wood-1)
  - [Wood 1 to Bronz](#wood-1-to-bronze)
  - [Bronze to Silver](#bronze-to-silver)
  - [Silver](#silver)
  - [My Final Strategy](#my-final-strategy)
  - [Summary](#summary)


</details>

# Wood 2 to Wood 1


I just check all samples and take sample with most score. Then go molecules to take everything need to complate sample, when sample is complated go laboratory to serve And thats enough for rank up to Wood 1.

```rust
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
In this league now we should diagnose our samples so first I took 3 undiagnosed sample and diagnose all. Boss robot wasn't smart in this league so I just beat him with taking rank 2 samples.

# Bronze to Silver
In bronze I decided to play more aggresively and I create an attack function.

 ## Attack Function
I'm looking enemy robot's first sample and I check if I block it with only taking 1 molecule if I can block just take it. Result of this sometimes I have so many unneccesary molecules but this function increase my win rate by %10-15.

 ## Choosing Sample
 I change my sample pick method.

 ```rust
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

```rust
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

```rust
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
Now I'm taking molecules for completing multiple samples at once. And I change my molecule choosing system. Now I'm giving score to every molecule and take with most score.

```rust
fn calculateMoleculeScore(molecule : usize,enemy: &Player,availables: [i32;5]) -> i32{

    let totalEnemyNeed = calculateEnemyNeed(enemy, molecule);


    return totalEnemyNeed - availables[molecule];
}
```
I'm calculating the score as how many my enemy need and how much left.

## Attack Function
I update my attack function. Now I'm looking for every sample that my enemy has and if I can block it with taking 1,2,3 molecule I take it.
My robot became more aggresive in late game because with expertises making samples become easier so I can spend my molecules bound (10) to block enemy samples.

## Choosing Sample
I update my choosing sample function. Now I'm giving score to every sample and I take with most score.

```rust
fn calculateSampleScore(sample : &Sample,player: &Player,availables: [i32;5],molecules:[i32;5]) -> i32{

    let gainAdvantage = calculateGainAdvantage(sample, player);
    let minNeed = calculateMinPickUp(sample,player);

    return 30 - minNeed + gainAdvantage;
}
```
I give score to sample as how many molecule need to complate and how much effect their expertise gain.
## Set Sample Done
One of the most smartest thing i did I set done samples as done and I pretend like I got their expertise and this way I can save 1 or 2 molecule. At the start of turn I simulate picking best sample and I check if it is done or not if it is done i set is flag true and I update remain molecules by removing dome samples molecules and I prevent we already have their expertise it saves 1-2 molecule per turn this was the most efficient thing I do in this challenge. I repeat this as player inventory size so basicly I simulate program and I set done if samples done in this way I can complete multiple samples when I'm at molecules station. Implementing this actually took 7-8 hours.
## Choosing Rank
I update my choosing rank function taking 3 two's and 3 three's can be problem sometimes so I made it hardcoded. It is not perfect but it is better than old one.
```rs
   let rankOrder = [1,1,1,1,1,1,1,2,2,1,2,2,2,3,2,3,2,3,2,3,3,2,3,3,2,3,2,3,2,3,2,3];
```

# My Final Strategy
- If you are at laboratory and you have done samples give all.
- If you don't have any samples go samples and take 3 if first round take 2 because enemy can block your molecules.
- If you have undiagnosed samples go diagnosis station and diagnose them.
- If you have diagnosed samples and you can make them go molecules and try to make all it once in given order. For example if we have 2 sample
"(AAA) gives expertise c" and "(BBB) give expertise B" my bot will going to take BBBAA and that will be enough to complete both of them because I'm also calculating expertise coming from samples.
- If you are at MOLECULES and enemy at somewhere else try to complete sample with most molecule.
- If you are at MOLECULES and if you can block one of the enemy samples with taking as "blockCount" molecule do it. "blockCount" increasing with player's total expertise, it makes my robot more aggresive in late game.
- If you can't make any of them release one of them to diagnosis station and take new one from samples station.
- If you are at LABORATORY and you complete some of the samples give them and go samples station to take new samples. But if round > 160 and you have 2 samples left go molecules to finish them too. Or round > 180 go molecules to finish them even you have 1 sample.


# Summary
Overall I tried tons of things attack, defence, distraction, wait with molecules but it took 10 hours to go silver and 20-30 hours to trying to be gold but I stuck on Silver #2/500 and #403 in total. Mostly first I do what enemy do and than improve it, it works until bronze. Bronze to silver I became more creative. That was very fun at the beginning but stuck at silver was exhausting. At the end of I did my best.


