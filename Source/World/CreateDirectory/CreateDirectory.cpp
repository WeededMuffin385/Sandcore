import Sandcore.World;

import <filesystem>;

void World::createDirectory(std::filesystem::path path, Parameters parameters)
{
	std::filesystem::create_directories(path);

	std::filesystem::create_directory(path / "chunks");
	std::filesystem::create_directory(path / "entities");
	std::filesystem::create_directory(path / "players");

	parameters.save(path / "parameters.txt");
}