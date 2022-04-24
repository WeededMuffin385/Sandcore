export module Sandcore.World.Chunk;

export import Sandcore.World.Block;
import Sandcore.VectorThree;

export class Chunk
{
public:

	Chunk();

	void loop();

	Block& getBlock(VectorThree<int> position);
	Block& getBlock(int x, int y, int z);

	void setBlock(VectorThree<int> position, Block block);
	void setBlock(int x, int y, int z, Block block);

	enum size : unsigned char
	{
		x = 16,
		y = 16,
		z = 16
	};

private:

	Block blockArray[size::x][size::y][size::z];

	friend class World;
};