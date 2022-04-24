import Sandcore.World.Entity;

import Sandcore.World.Bounds;

import Sandcore.World;

import Sandcore.World.Block.Identification;

Entity::Entity(World& world, VectorThree<int> worldPosition, VectorThree<float> chunkPosition) : world(world), worldPosition(worldPosition), chunkPosition(chunkPosition), size(0.9f, 0.9f, 0.9f), identification(EntityIdentification::human)
{

}

const VectorThree<int>& Entity::getWorldPosition()
{
	return worldPosition;
}

const VectorThree<float>& Entity::getChunkPosition()
{
	return chunkPosition;
}

const VectorThree <float>& Entity::getSize()
{
	return size;
}

void Entity::setMoveCondition(MoveCondition moveCondition)
{
	this->moveCondition = moveCondition;
}

void Entity::loop()
{
	move();
	fall();
	action();
	bounds(worldPosition, chunkPosition);
}

void Entity::move()
{
	VectorThree<float> deltaPosition;

	float speedMultiplier = 1;

	switch (moveCondition.moveState)
	{
	case MoveState::run:
		speedMultiplier = 2;
		break;

	case MoveState::sneak:
		speedMultiplier = 0.5;
		break;
	}

	{
		if (moveCondition.up) deltaPosition.y -= speed * world.getElapsedTime() / 1000000.f * speedMultiplier;
		if (moveCondition.down) deltaPosition.y += speed * world.getElapsedTime() / 1000000.f * speedMultiplier;

		if (moveCondition.left) deltaPosition.x -= speed * world.getElapsedTime() / 1000000.f * speedMultiplier;
		if (moveCondition.right) deltaPosition.x += speed * world.getElapsedTime() / 1000000.f * speedMultiplier;
	}

	{
		if (moveCondition.jump && !moveCondition.jumpCooldown && collisions(0, 0, -1.2)) deltaPosition.z += 1.2;
		if (moveCondition.fall && !moveCondition.jumpCooldown) deltaPosition.z -= 1.2;
	}

	if (!moveCondition.jump && !moveCondition.fall) moveCondition.jumpCooldown = false; else moveCondition.jumpCooldown = true;

	if (!collisions(deltaPosition.x, 0, 0)) chunkPosition.x += deltaPosition.x;
	if (!collisions(0, deltaPosition.y, 0)) chunkPosition.y += deltaPosition.y;
	if (!collisions(0, 0, deltaPosition.z)) chunkPosition.z += deltaPosition.z;

	if (collisions(0, 0, 0)) chunkPosition.z += 1;
}

void Entity::fall()
{
	fallSpeed = fallSpeed - 10 * world.getElapsedTime() / 1000000.f;

	float deltaFallPosition = world.getElapsedTime() / 1000000.f * fallSpeed;

	if (!collisions(0, 0, deltaFallPosition)) chunkPosition.z += deltaFallPosition; else fallSpeed = 0;
}

void Entity::action()
{
	// inventory.itemVector[actionCondition.slot];
}

bool Entity::collisions(VectorThree<float> deltaPosition) // is there collision with blocks ?
{
	float step = 0.5; // шаг, с которым будет идти проверка на столкновение

	for (float x = -(size.x / 2); x <= (size.x / 2); (x + step) <= (size.x / 2) ? x += step : x = (size.x / 2))
	{
		for (float y = -(size.y / 2); y <= (size.y / 2); (y + step) <= (size.y / 2) ? y += step : y = (size.y / 2))
		{
			for (float z = -(size.z / 2); z <= (size.z / 2); (z + step) <= (size.z / 2) ? z += step : z = (size.z / 2))
			{
				if (collision(deltaPosition.x + x, deltaPosition.y + y, deltaPosition.z + z)) return true;

				if (z == (size.z / 2)) break;
			}

			if (y == (size.y / 2)) break;
		}

		if (x == (size.x / 2)) break;
	}

	return false;
}

bool Entity::collisions(float x, float y, float z)
{
	return collisions(VectorThree<float>(x, y, z));
}

bool Entity::collision(VectorThree<float> deltaPosition)
{
	VectorThree<int> finalWorldPosition(worldPosition);
	VectorThree<float> finalChunkPosition(chunkPosition + deltaPosition);

	bounds(finalWorldPosition, finalChunkPosition);

	switch (world.getChunk(finalWorldPosition).getBlock(finalChunkPosition.x, finalChunkPosition.y, finalChunkPosition.z).getId())
	{
	case BlockIdentification::vacuum:
		return false;
		break;

	case BlockIdentification::water:
		return false;
		break;

	default:
		break;
	}

	return true;
}

bool Entity::collision(float x, float y, float z)
{
	return collision(VectorThree<float>(x, y, z));
}

EntityIdentification Entity::getId()
{
	return identification;
}