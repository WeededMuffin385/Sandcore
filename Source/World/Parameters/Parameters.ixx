export module Sandcore.World.Parameters;

export import <filesystem>;
export import <string>;

export class Parameters
{
public:

	void load(std::filesystem::path path);
	void save(std::filesystem::path path);

	std::string name;

	float seed;

	float terrainScale; // 75
	float terrainHeight; // 32

	float caveScale;
	float caveSize; // [-1; 1]

	float waterHeight;

	float ironOreScale;
	float ironOreSize;

	float coalOreScale;
	float coalOreSize;

	float goldOreScale;
	float goldOreSize;
};