# Hold the broadcast design

## General concepts
Player controls ship, that flies through dangerous galaxy regions, which connected with wormholes. Crew of ship tries to spread information that exposing galaxy council. Government forces effort to stop and destroy rebellion ship. There is no hope to survive against all power of mighty galaxy army. But the more people get evidences, the chances to bring down cruel regime.

## General mechanics
- Third person camera follows the ship, that flies through different space locations and fights with enemy forces.
- Ship can travel through wormholes to other regions.
- Enemies periodically attack player's ship in region. Each wave is stronger than previous.
- Enemies count and power increased with each region change.
- Ship can gather parts of defeated enemies.
- Player selects ship type before game starts.
- While travel through a wormhole, player can improve current equipment, or replace it with new.
- Player selects crew members races and their innate abilities before game starts.
- Crew members get perks when they gain enough experience.
- Crew members perks and abilities defined by their role on ship.
- Ship has integrity and power parameters. Integrity decreased with each damage, which wasn't stopped by shields. It restores between regions from collected ship parts. Power supplies ship shields and energetic weapons.

## Before game starts
- Player selects a ship model.
- Player selects a ship start equipment.
- Player selects ship crew and their innate abilities.

## Gameplay in region
- Ship spawns near to wormhole.
- Radar detects close enemies and objects of interest.
- Primary task is to achieve wormhole to next region. All wormholes are protected by enemy squads. Player should destroy them before he can travel.

## Content

### Proof of concept
- Two ships:
    - Lightweight ship with good speed, one additional module cell and good power regeneration rate.
    - Heavy and slow ship with significantly increased integrity and energy capacity and three additional module cells.
- Two weapon categories:
    - Lasers:
        - Powerful but long-to-reload beam.
        - Permanent ray with low damage.
    - Rockets:
        - One rocket with high damage to integrity.
        - Batch of small rockets, that flies to different targets.
- Two core modules:
    - Power accumulator, that allows to instantly get energy boost.
    - Maneuver drives, that allows to dodge enemy fire.
    - Additional solid fuel engines, that allows to get high acceleration, while fuel is present.
- Four additional modules:
    - Anti-projectile autocannon, that has a chance to destroy hostile projectile.
    - Speed booster increases basis speed of the ship.
    - Weapon cooling unit, that reduces laser energy consumption.
    - Reserve rockets parts, that reduces ship parts required to create new rockets.