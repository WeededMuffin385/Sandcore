export module Sandcore.World.Entity.MoveCondition;

import Sandcore.VectorThree;

export enum class MoveState
{
	sneak,
	move,
	run,
};

export class MoveCondition
{
public:
	void operator=(MoveCondition& another);

	MoveState moveState = MoveState::move;

	bool up = false;
	bool down = false;

	bool left = false;
	bool right = false;

	bool jump = false;
	bool fall = false;

	bool jumpCooldown = false;
};