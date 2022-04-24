#include <SFML/Graphics.hpp>

import Sandcore.Interface.Button.World;

WorldButton::WorldButton(sf::RenderWindow& window, sf::Event& event, std::string name, std::filesystem::path path) : icon(window, event), name(window, event), settings(window, event), path(path)
{
	this->name.setString(name);

	icon.setColor(sf::Color(255, 255, 255, 255), sf::Color(127, 255, 127, 255));
}

void WorldButton::aim(sf::Vector2f mouse)
{
	icon.aim(mouse);
	name.aim(mouse);
	settings.aim(mouse);
}

WorldButtonClickIdentification WorldButton::click(sf::Vector2f mouse)
{

	if (icon.click(mouse)) return WorldButtonClickIdentification::icon;
	if (name.click(mouse)) return WorldButtonClickIdentification::name;
	if (settings.click(mouse)) return WorldButtonClickIdentification::settings;
	return WorldButtonClickIdentification::null;

}

void WorldButton::setPosition(sf::Vector2f position)
{
	icon.setPosition(position.x - icon.getGlobalBounds().width - (name.getGlobalBounds().height / 8), position.y);
	name.setPosition(position);
	settings.setPosition(position.x + name.getGlobalBounds().width + (name.getGlobalBounds().height / 8), position.y);
}

void WorldButton::setPosition(float x, float y)
{
	setPosition(sf::Vector2f(x, y));
}

void WorldButton::setTexture(sf::Texture& iconTexture, sf::Texture& settingsTexture, sf::Texture& nameTexture)
{
	icon.setTexture(iconTexture);
	name.setTexture(nameTexture);
	settings.setTexture(settingsTexture);
}

void WorldButton::draw(sf::RenderTarget& target, sf::RenderStates states) const
{
	target.draw(icon);
	target.draw(name);
	target.draw(settings);
}

const std::filesystem::path& WorldButton::getPath()
{
	return path;
}

sf::FloatRect WorldButton::getGlobalBounds()
{
	return name.getGlobalBounds();
}