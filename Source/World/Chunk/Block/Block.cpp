import Sandcore.World.Block;

Block::Block(BlockIdentification identification) : identification(identification)
{

}

BlockIdentification& Block::getId()
{
	return identification;
}