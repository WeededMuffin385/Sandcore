import Sandcore.World.Entity.MoveCondition;


void MoveCondition::operator=(MoveCondition& another)
{

	moveState = another.moveState;

	up = another.up;
	down = another.down;

	left = another.left;
	right = another.right;

	jump = another.jump;
	fall = another.fall;
}