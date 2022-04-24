export module Sandcore.World.Entity.AI;

export import Sandcore.World.Entity;

export class EntityAI : public Entity
{
public:

	EntityAI(World& world, VectorThree<int> worldPosition = { 0,0,0 }, VectorThree<float> chunkPosition = { 0,0,0 });

	virtual void loop();

protected:

	VectorThree<int>	worldPreviousPosition; // позиция игрока, считается в чанках
	VectorThree<float>	chunkPreviousPosition; // позиция игрока относительно чанка.


	MoveCondition generateMoveCondition();

	void checkTarget();
	
	VectorThree<int>	worldTargetPosition; // позиция игрока, считается в чанках
	VectorThree<float>	chunkTargetPosition; // позиция игрока относительно чанка.

	int targetAttemptsNumber = 0;

	bool haveTarget = false;
};