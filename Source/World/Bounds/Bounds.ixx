export module Sandcore.World.Bounds;

export import Sandcore.VectorThree;

import Sandcore.World.Chunk;

export inline void bound(int& worldPosition, float& chunkPosition, const int& chunkSize)
{
	while (chunkPosition >= chunkSize)
	{
		worldPosition += 1;
		chunkPosition -= chunkSize;
	}

	while (chunkPosition < 0)
	{
		worldPosition -= 1;
		chunkPosition += chunkSize;
	}
}

export inline void bound(int& worldPosition, int& chunkPosition, const int& chunkSize)
{
	while (chunkPosition >= chunkSize)
	{
		worldPosition += 1;
		chunkPosition -= chunkSize;
	}

	while (chunkPosition < 0)
	{
		worldPosition -= 1;
		chunkPosition += chunkSize;
	}
}

export void bounds(VectorThree<int>& worldPosition, VectorThree<float>& chunkPosition)
{
	bound(worldPosition.x, chunkPosition.x, Chunk::size::x);
	bound(worldPosition.y, chunkPosition.y, Chunk::size::y);
	bound(worldPosition.z, chunkPosition.z, Chunk::size::z);
}
