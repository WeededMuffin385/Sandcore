#include <SFML/Graphics.hpp>

export module Sandcore.Interface.Button.World;

import <filesystem>;

import Sandcore.Interface.Button.World.ClickIdentification;
import Sandcore.Interface.Button.Text;
import Sandcore.Interface.Button;

export class WorldButton : public sf::Drawable
{
public:

	WorldButton(sf::RenderWindow& window, sf::Event& event, std::string name, std::filesystem::path path);

	void aim(sf::Vector2f mouse);

	WorldButtonClickIdentification click(sf::Vector2f mouse);

	void draw(sf::RenderTarget& target, sf::RenderStates states) const;

	void setTexture(sf::Texture& iconTexture, sf::Texture& settingsTexture, sf::Texture& nameTexture);

	void setPosition(sf::Vector2f position);
	void setPosition(float x, float y);

	const std::filesystem::path& getPath();

	sf::FloatRect getGlobalBounds();

private:

	Button icon;
	Button settings;
	TextButton name;

	std::filesystem::path path;
};