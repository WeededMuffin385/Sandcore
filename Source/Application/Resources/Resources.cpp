#include <SFML/Graphics.hpp>
#include <fstream>
#include <iostream>

import Sandcore.Resources;
import <filesystem>;


std::map<ResourcesIdentification, sf::Texture> Resources::textureMap;

sf::Font Resources::font;
int Resources::fontSize;
sf::Image Resources::icon;
bool Resources::debug = true;

std::filesystem::path Resources::settings;
std::filesystem::path Resources::userData;
std::filesystem::path Resources::resources;
std::filesystem::path Resources::texturePackage;

void Resources::load(std::filesystem::path path)
{
	if (std::filesystem::exists(path)) settings = path; else settings = "D:/Sandcore/settings.txt";

	{
		std::ifstream file(settings); // settings directory path

		while (!file.eof())
		{
			std::string identification;

			file >> identification;

			if (identification == "userData") file >> userData;
			if (identification == "resources") file >> resources;
			if (identification == "texturePackage") file >> texturePackage;
		}
	}

	loadTextures(resources / "Textures");

	font.loadFromFile((resources / "retrogaming.ttf").string());

	icon.loadFromFile((resources / "logo.png").string());

	fontSize = 30;

}

void Resources::loadTextures(std::filesystem::path path)
{
	textureMap[ResourcesIdentification::background].loadFromFile((path / "Interface" / "background.png").string());

	textureMap[ResourcesIdentification::drawableBackground].loadFromFile((path / "Interface" / "drawable_background.png").string());
	textureMap[ResourcesIdentification::drawableBackground2].loadFromFile((path / "Interface" / "drawable_background_2.png").string());

	textureMap[ResourcesIdentification::worldIcon1].loadFromFile((path / "Interface" / "world_icon_1.png").string());
	textureMap[ResourcesIdentification::worldIcon2].loadFromFile((path / "Interface" / "world_icon_2.png").string());
	textureMap[ResourcesIdentification::worldIcon3].loadFromFile((path / "Interface" / "world_icon_3.png").string());
	textureMap[ResourcesIdentification::worldIcon4].loadFromFile((path / "Interface" / "world_icon_4.png").string());

	textureMap[ResourcesIdentification::button].loadFromFile((path / "Interface" / "button.png").string());
	textureMap[ResourcesIdentification::settings].loadFromFile((path / "Interface" / "settings.png").string());

	textureMap[ResourcesIdentification::hotbarSlot].loadFromFile((path / "Interface" / "hotbar_slot.png").string());
	textureMap[ResourcesIdentification::hotbarSlotOutline].loadFromFile((path / "Interface" / "hotbar_slot_outline.png").string());
}