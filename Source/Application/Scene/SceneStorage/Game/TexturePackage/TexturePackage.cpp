#include <SFML/Graphics.hpp>
#include <fstream>

import Sandcore.Scene.Game.TexturePackage;

import Sandcore.Resources;

TexturePackage::TexturePackage(std::filesystem::path path) : path(path)
{
	load();
}

void TexturePackage::load()
{
	loadInformation();
	loadBlocks();
	loadEntities();

	background.loadFromFile((path / "renderBackground3.png").string());
}

void TexturePackage::loadInformation()
{
	std::ifstream file(path / "info.txt");

	while (!file.eof())
	{
		std::string identification;

		file >> identification;

		if (identification == "resolution")
		{
			file >> size.x;
			size.y = size.x;
		}
	}

	file.close();
}

void TexturePackage::loadBlock(std::filesystem::path path, BlockIdentification identification)
{
	
	blockTextureMap[identification].loadFromFile(path.string());
	blockTextureMap[identification].generateMipmap();
	blockSpriteMap[identification].setTexture(blockTextureMap[identification]);

	blockSpriteMap[identification].setPosition(size.x * static_cast<int>(identification), 0);

	// sf::IntRect(size.x * static_cast<int>(identification), 0, size.x, size.y);
}

void TexturePackage::loadBlocks()
{
	loadBlock(path / "block" / "dirt.png", BlockIdentification::dirt);
	loadBlock(path / "block" / "grass.png", BlockIdentification::grass);
	loadBlock(path / "block" / "stone.png", BlockIdentification::stone);
	loadBlock(path / "block" / "vacuum.png", BlockIdentification::vacuum);

	loadBlock(path / "block" / "water2.png", BlockIdentification::water);
	loadBlock(path / "block" / "sand.png", BlockIdentification::sand);
	loadBlock(path / "block" / "wood.png", BlockIdentification::wood);

	loadBlock(path / "block" / "iron_ore.png", BlockIdentification::ironOre);
	loadBlock(path / "block" / "coal_ore.png", BlockIdentification::coalOre);
	loadBlock(path / "block" / "gold_ore.png", BlockIdentification::goldOre);
}

void TexturePackage::loadEntity(std::filesystem::path path, EntityIdentification identification)
{
	entityTextureMap[identification].loadFromFile(path.string());
	entitySpriteMap[identification].setTexture(entityTextureMap[identification]);
	entitySpriteMap[identification].setOrigin(entityTextureMap[identification].getSize().x / 2, entityTextureMap[identification].getSize().y / 2);
}

void TexturePackage::loadEntities()
{
	loadEntity(path / "entity" / "elephant.png", EntityIdentification::elephant);
	loadEntity(path / "entity" / "player.png", EntityIdentification::human);
}