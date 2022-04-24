#include <SFML/Graphics.hpp>

export module Sandcore.Scene.Game.TexturePackage;

export import <map>;
export import <filesystem>;
export import Sandcore.World.Block.Identification;
export import Sandcore.World.Entity.Identification;
export import Sandcore.VectorThree;

export class TexturePackage
{
public:
	TexturePackage(std::filesystem::path path);

	std::map<BlockIdentification, sf::Sprite> blockSpriteMap;
	std::map<EntityIdentification, sf::Sprite> entitySpriteMap;

	// VectorThree<int> size;

	struct{ int x; int y;} size;

	sf::Texture background;

private:

	void load();

	void loadInformation();

	void loadBlock(std::filesystem::path path, BlockIdentification identification);
	void loadBlocks();

	void loadEntity(std::filesystem::path path, EntityIdentification identification);
	void loadEntities();

	std::filesystem::path path;

	std::map<BlockIdentification, sf::Texture> blockTextureMap;
	std::map<EntityIdentification, sf::Texture> entityTextureMap;
};