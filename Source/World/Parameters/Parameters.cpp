import Sandcore.World.Parameters;

#include <fstream>

void Parameters::load(std::filesystem::path path)
{
	std::ifstream file(path);

	while (!file.eof())
	{
		std::string identification;
		file >> identification;

		if (identification == "name") std::getline(file, name);

		if (identification == "seed") file >> seed;

		if (identification == "terrainScale") file >> terrainScale;
		if (identification == "terrainHeight") file >> terrainHeight;

		if (identification == "waterHeight") file >> waterHeight;

		if (identification == "caveScale") file >> caveScale;
		if (identification == "caveSize") file >> caveSize;

		if (identification == "ironOreScale") file >> ironOreScale;
		if (identification == "ironOreSize") file >> ironOreSize;

		if (identification == "coalOreScale") file >> coalOreScale;
		if (identification == "coalOreSize") file >> coalOreSize;

		if (identification == "goldOreScale") file >> goldOreScale;
		if (identification == "goldOreSize") file >> goldOreSize;
	}

	file.close();
}

void Parameters::save(std::filesystem::path path)
{
	std::ofstream file(path);
	{
		file << "name " << name << "\n";

		file << "seed " << seed << "\n";

		file << "terrainScale " << terrainScale << "\n";
		file << "terrainHeight " << terrainHeight << "\n";

		file << "waterHeight " << waterHeight << "\n";

		file << "caveScale " << caveScale << "\n";
		file << "caveSize " << caveSize << "\n";

		file << "ironOreScale " << ironOreScale << "\n";
		file << "ironOreSize " << ironOreSize << "\n";

		file << "coalOreScale " << coalOreScale << "\n";
		file << "coalOreSize " << coalOreSize << "\n";

		file << "goldOreScale " << goldOreScale << "\n";
		file << "goldOreSize " << goldOreSize;
	}
}