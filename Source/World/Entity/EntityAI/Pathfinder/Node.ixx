export module Sandcore.World.Entity.AI.Pathfinder.Node;

export class Node
{
public:

	int x;
	int y;
	int parentX;
	int parentY;
	float gCost;
	float hCost;
	float fCost;

	bool operator<(Node& right)
	{
		return this->fCost < right.fCost;
	}
};