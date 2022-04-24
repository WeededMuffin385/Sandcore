export module Sandcore.World.Entity.ActionCondition;

import Sandcore.VectorThree;

export class ActionCondition
{
public:
	VectorThree<int>	worldRelativePosition; // где применить
	VectorThree<float>	chunkRelativePosition; // где применить

	int slot; // что применить
};