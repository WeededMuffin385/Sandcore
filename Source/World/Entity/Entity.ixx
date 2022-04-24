class World;

export module Sandcore.World.Entity;

import Sandcore.World.Entity.Identification;
import Sandcore.VectorThree;

import Sandcore.World.Entity.MoveCondition;
import Sandcore.World.Entity.ActionCondition;
import Sandcore.World.Bounds;

export class Entity
{
public:

	Entity(World& world, VectorThree<int> worldPosition = { 0,0,0 }, VectorThree<float> chunkPosition = { 0,0,0 });

	const VectorThree <int>&	getWorldPosition();
	const VectorThree <float>&	getChunkPosition();

	const VectorThree <float>&	getSize();

	void setMoveCondition(MoveCondition moveCondition);

	virtual void loop();

	EntityIdentification getId();

protected:

	EntityIdentification identification;

	VectorThree<int>	worldPosition; // позиция игрока, считается в чанках
	VectorThree<float>	chunkPosition; // позиция игрока относительно чанка.

	VectorThree<float>	size;

	MoveCondition moveCondition;
	ActionCondition actionCondition;

	void move();
	void fall();
	void action();

	World& world;

	float speed = 8;
	float fallSpeed = 2;

	bool collisions(VectorThree<float> deltaPosition);
	bool collisions(float x, float y, float z);

	bool collision(VectorThree<float> deltaPosition);
	bool collision(float x, float y, float z);

	friend class World;
};