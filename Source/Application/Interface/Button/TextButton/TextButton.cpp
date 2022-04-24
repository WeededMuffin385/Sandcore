#include <SFML/Graphics.hpp>

import Sandcore.Interface.Button.Text;

import Sandcore.Resources;

TextButton::TextButton(sf::RenderWindow& window, sf::Event& event) : button(window, event)
{
	text.setFont(Resources::font);
	text.setFillColor(sf::Color::Black);
	text.setCharacterSize(Resources::fontSize);
}

void TextButton::draw(sf::RenderTarget& target, sf::RenderStates states) const
{
	target.draw(button);
	target.draw(text);
}

void TextButton::setPosition(sf::Vector2f position)
{
	button.setPosition(position);
	text.setPosition(position);
}

void TextButton::setPosition(float x, float y)
{
	setPosition(sf::Vector2f(x, y));
}

void TextButton::setTexture(sf::Texture& texture)
{
	button.setTexture(texture);
}

void TextButton::setColor(sf::Color normalColor, sf::Color aimedColor)
{
	button.setColor(normalColor, aimedColor);
}

void TextButton::setString(std::string string)
{
	text.setString(string);
}

std::string TextButton::getString()
{
	return text.getString();
}

void TextButton::aim(sf::Vector2f mouse)
{
	button.aim(mouse);
}

bool TextButton::click(sf::Vector2f mouse)
{
	return button.click(mouse);
}

sf::FloatRect TextButton::getGlobalBounds() const
{
	return button.getGlobalBounds();
}