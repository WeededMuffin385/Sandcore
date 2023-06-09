class Entity;

export module Sandcore.World.Entity.AI.Pathfinder;

export import Sandcore.VectorThree;
import Sandcore.World.Entity.AI.Pathfinder.Node;

export class Pathfinder
{
public:

	Pathfinder(Entity& entity, VectorThree<int> worldGoalPosition, VectorThree<float> chunkGoalPosition);

	VectorThree<int>	worldGoalPosition = { 0,0,0 }; // ������� ������, ��������� � ������
	VectorThree<float>	chunkGoalPosition = { 0,0,0 }; // ������� ������ ������������ �����.

	VectorThree<int>	worldDeltaPosition; // ������� ������, ��������� � ������
	VectorThree<float>	chunkDeltaPosition; // ������� ������ ������������ �����.

	VectorThree<double>	deltaPosition;


	static double calculateH(int x, int y);

};