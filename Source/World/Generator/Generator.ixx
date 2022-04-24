export module Sandcore.World.Generator;

import Sandcore.World.Chunk;
import Sandcore.World.Parameters;

export class Generator
{
public:

	Generator(Parameters& parameters);

	Chunk generateChunk(VectorThree<int> worldPosition);

private:

	Chunk chunk;
	Parameters& parameters;

	VectorThree<int> worldPosition;
	VectorThree<int> chunkPosition;

	VectorThree<int> globalPosition;

	void generateTerrain();
	void generateCave();
	void generateSinkhole(int a, int b, int c);
	void generateWater();
	void generateTree();
	void generateOreDeposit(BlockIdentification identification, float scale, float size);
};