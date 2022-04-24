class Entity;

export module Sandcore.World.Entity.AI.Pathfinder;

export import Sandcore.VectorThree;
import Sandcore.World.Entity.AI.Pathfinder.Node;

export class Pathfinder
{
public:

	Pathfinder(Entity& entity, VectorThree<int> worldGoalPosition, VectorThree<float> chunkGoalPosition);

	VectorThree<int>	worldGoalPosition = { 0,0,0 }; // позиция игрока, считается в чанках
	VectorThree<float>	chunkGoalPosition = { 0,0,0 }; // позиция игрока относительно чанка.

	VectorThree<int>	worldDeltaPosition; // позиция игрока, считается в чанках
	VectorThree<float>	chunkDeltaPosition; // позиция игрока относительно чанка.

	VectorThree<double>	deltaPosition;


	static double calculateH(int x, int y);

};