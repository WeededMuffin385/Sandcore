import Sandcore.World.Generator;


import Sandcore.SimplexNoise;

Generator::Generator(Parameters& parameters) : parameters(parameters)
{

}

Chunk Generator::generateChunk(VectorThree<int> worldPosition)
{
	this->worldPosition = worldPosition;

	chunk = Chunk();

	for (int x = 0; x < Chunk::size::x; ++x)
	{
		for (int y = 0; y < Chunk::size::y; ++y)
		{
			for (int z = 0; z < Chunk::size::z; ++z)
			{
				chunkPosition = VectorThree<int>(x, y, z);

				globalPosition = VectorThree<int>
					(
						x + worldPosition.x * Chunk::size::x,
						y + worldPosition.y * Chunk::size::y,
						z + worldPosition.z * Chunk::size::z
						);

				if (worldPosition.z >= 0)
				{
					generateTerrain();
					generateWater();
					generateTree();
				}

				if (worldPosition.z < 0)
				{
					generateCave();

					generateOreDeposit(BlockIdentification::vacuum, parameters.caveScale, parameters.caveSize);

					generateOreDeposit(BlockIdentification::ironOre, parameters.ironOreScale, parameters.ironOreSize);
					generateOreDeposit(BlockIdentification::coalOre, parameters.coalOreScale, parameters.coalOreSize);
					generateOreDeposit(BlockIdentification::goldOre, parameters.goldOreScale, parameters.goldOreSize);
				}

				// generateSinkhole(5, 5, 5);
			}
		}
	}

	return chunk;
}

void Generator::generateTerrain()
{
	if (globalPosition.z < static_cast<int>(SimplexNoise::noise((globalPosition.x + parameters.seed) / parameters.terrainScale, (globalPosition.y + parameters.seed) / parameters.terrainScale) * (parameters.terrainHeight / 2) + (parameters.terrainHeight / 2)))
	{
		chunk.setBlock(chunkPosition.x, chunkPosition.y, chunkPosition.z, Block(BlockIdentification::dirt));
	}

	if (globalPosition.z == static_cast<int>(SimplexNoise::noise((globalPosition.x + parameters.seed) / parameters.terrainScale, (globalPosition.y + parameters.seed) / parameters.terrainScale) * (parameters.terrainHeight / 2) + (parameters.terrainHeight / 2)))
	{
		chunk.setBlock(chunkPosition.x, chunkPosition.y, chunkPosition.z, Block(BlockIdentification::grass));
	}
}

void Generator::generateCave()
{
	chunk.setBlock(chunkPosition.x, chunkPosition.y, chunkPosition.z, Block(BlockIdentification::stone));
}

void Generator::generateOreDeposit(BlockIdentification identification, float scale, float size)
{
	if (globalPosition.z < 0 && chunk.getBlock(chunkPosition).getId() == BlockIdentification::stone)

		if (size <= (SimplexNoise::noise((globalPosition.x + parameters.seed) / scale, (globalPosition.y + parameters.seed) / scale, (globalPosition.z + parameters.seed) / scale)))
		{
			chunk.setBlock(chunkPosition.x, chunkPosition.y, chunkPosition.z, Block(identification));
		}
}

void Generator::generateWater()
{
	if (globalPosition.z <= parameters.waterHeight)
	{
		switch (chunk.getBlock(chunkPosition).getId())
		{
		case BlockIdentification::vacuum:
			chunk.setBlock(chunkPosition.x, chunkPosition.y, chunkPosition.z, Block(BlockIdentification::water));
			break;

		case BlockIdentification::grass:
			chunk.setBlock(chunkPosition.x, chunkPosition.y, chunkPosition.z, Block(BlockIdentification::sand));
			break;
		}
	}
}

void Generator::generateTree()
{
	if (chunkPosition.z == 0) return;

	if (rand() % 100 == 0)
	{
		if (chunk.getBlock(chunkPosition + VectorThree<int>(0, 0, -1)).getId() == BlockIdentification::grass) chunk.setBlock(chunkPosition.x, chunkPosition.y, chunkPosition.z, Block(BlockIdentification::wood));
	}

	if (chunk.getBlock(chunkPosition + VectorThree<int>(0, 0, -1)).getId() == BlockIdentification::wood) chunk.setBlock(chunkPosition.x, chunkPosition.y, chunkPosition.z, Block(BlockIdentification::wood));
}

void Generator::generateSinkhole(int a, int b, int c)
{
	if (globalPosition.x * globalPosition.x / a / a + globalPosition.y * globalPosition.y / b / b - globalPosition.z * globalPosition.z / c / c <= 1)
	{
		chunk.setBlock(chunkPosition.x, chunkPosition.y, chunkPosition.z, Block(BlockIdentification::vacuum));
	}
}