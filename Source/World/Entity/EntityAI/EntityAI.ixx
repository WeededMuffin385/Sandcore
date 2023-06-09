export module Sandcore.World.Entity.AI;

export import Sandcore.World.Entity;

export class EntityAI : public Entity
{
public:

	EntityAI(World& world, VectorThree<int> worldPosition = { 0,0,0 }, VectorThree<float> chunkPosition = { 0,0,0 });

	virtual void loop();

protected:

	VectorThree<int>	worldPreviousPosition; // ������� ������, ��������� � ������
	VectorThree<float>	chunkPreviousPosition; // ������� ������ ������������ �����.


	MoveCondition generateMoveCondition();

	void checkTarget();
	
	VectorThree<int>	worldTargetPosition; // ������� ������, ��������� � ������
	VectorThree<float>	chunkTargetPosition; // ������� ������ ������������ �����.

	int targetAttemptsNumber = 0;

	bool haveTarget = false;
};