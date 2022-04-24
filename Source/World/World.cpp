#include <fstream>
#include <iostream>

import Sandcore.World;

import Sandcore.World.Entity.AI;


World::World(std::filesystem::path path) : generator(parameters), path(path)
{
	parameters.load(path / "parameters.txt"); 

	for (int i = 0; i < 10; ++i)
	{
		EntityAI* newEntity = new EntityAI(*this);
		newEntity->identification = EntityIdentification::elephant;
		entitySet.insert(newEntity);
	}

	// for (int i = 0; i < 10; ++i) entitySet.insert(new EntityAI(*this));
}

World::~World()
{
	for (auto player : playerMap)
	{
		savePlayer(player.first);
	}

	for (auto chunk : chunkMap)
	{
		saveChunk(chunk.first);
	}
}

Chunk& World::getChunk(int x, int y, int z)
{
	return getChunk(VectorThree<int>(x, y, z));
}

Chunk& World::getChunk(VectorThree<int> position)
{
	if (chunkMap.find(position) == chunkMap.end()) loadChunk(position);

	return chunkMap[position];
}

Player* World::getPlayer(std::string username)
{
	if (playerMap.find(username) == playerMap.end()) loadPlayer(username);

	return &playerMap[username];
}

std::set<Entity*>& World::getEntitySet()
{
	return entitySet;
}

long long World::getElapsedTime()
{
	return clock.getElapsedTime();
}

void World::loop()
{
	clock.restart();

	for (auto entity : entitySet)
	{
		entity->loop();
	}

	for (auto& chunk : chunkMap)
	{
		chunk.second.loop();
	}
}

void World::loadPlayer(std::string identification)
{
	if (std::filesystem::exists(playerPath(identification))) readPlayer(identification); else generatePlayer(identification);
}

void World::savePlayer(std::string identification)
{
	if (debug)
	{
		std::cout << "save player [" << identification << "]" << "\n";
	}

	std::ofstream file(playerPath(identification));

	file << "worldPosition ";

	file << playerMap[identification].getEntity()->getWorldPosition().x << " ";
	file << playerMap[identification].getEntity()->getWorldPosition().y << " ";
	file << playerMap[identification].getEntity()->getWorldPosition().z << "\n";

	file << "chunkPosition ";

	file << playerMap[identification].getEntity()->getChunkPosition().x << " ";
	file << playerMap[identification].getEntity()->getChunkPosition().y << " ";
	file << playerMap[identification].getEntity()->getChunkPosition().z << "\n";

	file.close();
}

void World::readPlayer(std::string identification)
{
	if (debug)
	{
		std::cout << "read player [" << identification << "]" << "\n";
	}

	std::ifstream file(playerPath(identification));

	VectorThree<int> worldPosition;
	VectorThree<float> chunkPosition;

	while (!file.eof())
	{
		std::string variable;
		file >> variable;

		if (variable == "worldPosition")
		{
			int x;
			int y;
			int z;

			file >> x >> y >> z;
			worldPosition = VectorThree<int>(x, y, z);
		}

		if (variable == "chunkPosition")
		{
			float x;
			float y;
			float z;

			file >> x >> y >> z;
			chunkPosition = VectorThree<float>(x, y, z);
		}
	}

	Entity* newEntity = new Entity(*this, worldPosition, chunkPosition);

	entitySet.insert(newEntity);

	playerMap[identification] = Player(newEntity);

	file.close();
}

void World::generatePlayer(std::string identification)
{
	if (debug)
	{
		std::cout << "generate player [" << identification << "]" << "\n";
	}

	Entity* newEntity = new Entity(*this);

	entitySet.insert(newEntity);

	playerMap[identification] = Player(newEntity);
}

std::filesystem::path World::playerPath(std::string identification)
{
	return path / "players" / identification;
}

void World::loadChunk(VectorThree<int> position)
{
	if (std::filesystem::exists(chunkPath(position))) readChunk(position); else generateChunk(position);
}

void World::saveChunk(VectorThree<int> position)
{
	if (debug)
	{
		std::cout << "save chunk [" << position.x << "|" << position.y << "|" << position.z << "]" << "\n";
	}

	std::ofstream file(chunkPath(position), std::ios::out | std::ios::binary);

	unsigned char buffer[Chunk::size::x][Chunk::size::y][Chunk::size::z] = {};

	for (int x = 0; x < Chunk::size::x; ++x)
	{
		for (int y = 0; y < Chunk::size::y; ++y)
		{
			for (int z = 0; z < Chunk::size::z; ++z)
			{
				buffer[x][y][z] = static_cast<unsigned char>(chunkMap[position].blockArray[x][y][z].getId());
			}
		}
	}

	file.write((char*)buffer, Chunk::size::x * Chunk::size::y * Chunk::size::z);

	file.close();
}

void World::readChunk(VectorThree<int> position)
{
	if (debug)
	{
		std::cout << "read chunk [" << position.x << "|" << position.y << "|" << position.z << "]" << "\n";
	}

	std::ifstream file(chunkPath(position), std::ios::in | std::ios::binary);

	BlockIdentification buffer[Chunk::size::x][Chunk::size::y][Chunk::size::z] = {};

	file.read((char*)buffer, sizeof(BlockIdentification) * Chunk::size::x * Chunk::size::y * Chunk::size::z);

	for (int x = 0; x < Chunk::size::x; ++x)
	{
		for (int y = 0; y < Chunk::size::y; ++y)
		{
			for (int z = 0; z < Chunk::size::z; ++z)
			{
				chunkMap[position].blockArray[x][y][z] = Block(buffer[x][y][z]);
			}
		}
	}

	file.close();
}

void World::generateChunk(VectorThree<int> position)
{
	if (debug)
	{
		std::cout << "generate chunk [" << position.x << "|" << position.y << "|" << position.z << "]" << "\n";
	}

	chunkMap[position] = generator.generateChunk(position);
}

void World::loadChunkInRadius(VectorThree<int> position, int radius)
{
	for (int x = position.x - radius; x <= position.x + radius; ++x)
	{
		for (int y = position.y - radius; y <= position.y + radius; ++y)
		{
			for (int z = position.z - radius; z <= position.z + radius; ++z)
			{
				if (chunkMap.find(VectorThree<int>(x, y, z)) == chunkMap.end()) loadChunk(VectorThree<int>(x, y, z));
			}
		}
	}
}

std::filesystem::path World::chunkPath(VectorThree<int> position)
{
	return path / "chunks" / std::string(std::to_string(position.x) + " " + std::to_string(position.y) + " " + std::to_string(position.z));
}