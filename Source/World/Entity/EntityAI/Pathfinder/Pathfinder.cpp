#include <cmath>

import Sandcore.World.Entity.AI.Pathfinder;

import Sandcore.World.Chunk;
import Sandcore.World.Entity;

Pathfinder::Pathfinder(Entity& entity, VectorThree<int> worldGoalPosition, VectorThree<float> chunkGoalPosition) : worldGoalPosition(worldGoalPosition), chunkGoalPosition(chunkGoalPosition)
{
	worldDeltaPosition = worldGoalPosition - entity.getWorldPosition();
	chunkDeltaPosition = chunkGoalPosition - entity.getChunkPosition();

	deltaPosition = VectorThree<double>(worldDeltaPosition.x * Chunk::size::x + chunkDeltaPosition.x, worldDeltaPosition.y * Chunk::size::y + chunkDeltaPosition.y, worldDeltaPosition.z * Chunk::size::z + chunkDeltaPosition.z);
}

double Pathfinder::calculateH(int x, int y)
{
	double h = (std::sqrt((x - dGoal))
}