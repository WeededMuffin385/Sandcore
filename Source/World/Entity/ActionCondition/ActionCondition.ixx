export module Sandcore.World.Entity.ActionCondition;

import Sandcore.VectorThree;

export class ActionCondition
{
public:
	VectorThree<int>	worldRelativePosition; // ��� ���������
	VectorThree<float>	chunkRelativePosition; // ��� ���������

	int slot; // ��� ���������
};