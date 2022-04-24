#include <cstdlib>
#include <iostream>

import Sandcore.World.Entity.AI;
import Sandcore.World.Entity;
import Sandcore.World.Bounds;
import Sandcore.World.Entity.MoveCondition;

EntityAI::EntityAI(World& world, VectorThree<int> worldPosition, VectorThree<float> chunkPosition) : Entity(world, worldPosition, chunkPosition)
{

}

void EntityAI::loop()
{
	checkTarget();

	setMoveCondition(generateMoveCondition());

	moveCondition.jumpCooldown = false;
	moveCondition.jump = true;

	worldPreviousPosition = worldPosition;
	chunkPreviousPosition = chunkPosition;

	move();
	fall();
	action();
	bounds(worldPosition, chunkPosition);

	if ((worldPosition.x == worldPreviousPosition.x) && (worldPosition.y == worldPreviousPosition.y) && (int(chunkPosition.x) == int(chunkPreviousPosition.x)) && (int(chunkPosition.y) == int(chunkPreviousPosition.y))) ++targetAttemptsNumber; else targetAttemptsNumber = 0;
}

void EntityAI::checkTarget()
{
	if ((worldPosition.x == worldTargetPosition.x) && (worldPosition.y == worldTargetPosition.y) && (int(chunkPosition.x) == int(chunkTargetPosition.x)) && (int(chunkPosition.y) == int(chunkTargetPosition.y))) haveTarget = false;

	if (!haveTarget || (targetAttemptsNumber > 1000))
	{
		targetAttemptsNumber = 0;

		chunkTargetPosition = VectorThree<float>(std::rand() % 8 - 4, std::rand() % 8 - 4);

		bounds(worldTargetPosition, chunkTargetPosition);

		haveTarget = true;
	}
}

MoveCondition EntityAI::generateMoveCondition()
{
	MoveCondition moveCondition_;
	moveCondition_.moveState = MoveState::sneak;

	if (worldPosition.x > worldTargetPosition.x) moveCondition_.left = true; else if (worldPosition.x < worldTargetPosition.x) moveCondition_.right = true;

	if (worldPosition.x == worldTargetPosition.x)
	{
		if (chunkPosition.x > chunkTargetPosition.x) moveCondition_.left = true; 
		else 
		if (chunkPosition.x < chunkTargetPosition.x) moveCondition_.right = true;
	}
	
	
	if (worldPosition.y > worldTargetPosition.y) moveCondition_.up = true; else if(worldPosition.y < worldTargetPosition.y) moveCondition_.down = true;

	if (worldPosition.y == worldTargetPosition.y)
	{
		if (chunkPosition.y > chunkTargetPosition.y) moveCondition_.up = true;
		else
		if (chunkPosition.y < chunkTargetPosition.y) moveCondition_.down = true;
	}

	return moveCondition_;
}