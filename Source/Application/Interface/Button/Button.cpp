#include <SFML/Graphics.hpp>

import Sandcore.Interface.Button;

Button::Button(sf::RenderWindow& window, sf::Event& event) : window(window), event(event), normalColor(255, 255, 255, 192), aimedColor(127, 255, 127, 192)
{
	sprite.setColor(normalColor);
}

void Button::setTexture(sf::Texture& texture)
{
	sprite.setTexture(texture, true);
}

void Button::setPosition(float x, float y)
{
	setPosition(sf::Vector2f(x, y));
}

void Button::setPosition(sf::Vector2f position)
{
	sprite.setPosition(position);
}

sf::FloatRect Button::getGlobalBounds() const
{
	return sprite.getGlobalBounds();
}

void Button::aim(sf::Vector2f mouse)
{
	sprite.setColor(normalColor);

	if (sprite.getGlobalBounds().contains(mouse)) sprite.setColor(aimedColor);
}

bool Button::click(sf::Vector2f mouse)
{
	if (event.type == sf::Event::MouseButtonPressed)
	{
		if (sprite.getGlobalBounds().contains(mouse))
		{
			return true;
		}
	}
	return false;
}

void Button::draw(sf::RenderTarget& target, sf::RenderStates states) const
{
	target.draw(sprite);
}

void Button::setColor(sf::Color normalColor, sf::Color aimedColor)
{
	sprite.setColor(normalColor);
	this->normalColor = normalColor;
	this->aimedColor = aimedColor;
}