import Sandcore.World.Player;

import Sandcore.World.Entity;

Player::Player(Entity* entity) : entity(entity), worldPositionChanged(true)
{

}

bool Player::isWorldPositionChanged()
{
	return worldPositionChanged;
}

Entity* Player::getEntity()
{
	return entity;
}