fn effect(player) {
    player.hp += player.max_hp / 5;
    if player.hp > player.max_hp {
        player.hp = player.max_hp;
    }
    player
}