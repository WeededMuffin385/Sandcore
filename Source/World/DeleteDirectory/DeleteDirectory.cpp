import Sandcore.World;

import <filesystem>;

void World::deleteDirectory(std::filesystem::path path)
{
	if (std::filesystem::exists(path / "parameters.txt"))
	{
		std::filesystem::remove_all(path);
	}
}