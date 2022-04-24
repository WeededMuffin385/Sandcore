#include <SFML/Graphics.hpp>

export module Sandcore.Resources;

export import <filesystem>;
export import <map>;

export import Sandcore.Resources.Identification;

export class Resources
{
public:

	static void load(std::filesystem::path path);

	static std::map<ResourcesIdentification, sf::Texture> textureMap;

	static sf::Font font;
	static int fontSize;
	static sf::Image icon;

	static std::filesystem::path settings;
	static std::filesystem::path userData;
	static std::filesystem::path resources;
	static std::filesystem::path texturePackage;

	static bool debug;

private:

	static void loadTextures(std::filesystem::path path);

};