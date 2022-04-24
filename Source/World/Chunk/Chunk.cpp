import Sandcore.World.Chunk;

Chunk::Chunk()
{

}

void Chunk::loop()
{

}

Block& Chunk::getBlock(VectorThree<int> position)
{
	return getBlock(position.x, position.y, position.z);
}

Block& Chunk::getBlock(int x, int y, int z)
{
	return blockArray[x][y][z];
}

void Chunk::setBlock(VectorThree<int> position, Block block)
{
	setBlock(position.x, position.y, position.z, block);
}

void Chunk::setBlock(int x, int y, int z, Block block)
{
	blockArray[x][y][z] = block;
}