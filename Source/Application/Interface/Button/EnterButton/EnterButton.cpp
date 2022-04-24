#include <SFML/Graphics.hpp>

import Sandcore.Interface.Button.Enter;

import Sandcore.Resources;

EnterButton::EnterButton(sf::RenderWindow& window, sf::Event& event) : window(window), event(event), button(window, event)
{
	text.setFont(Resources::font);
	text.setFillColor(sf::Color::Green);
	text.setCharacterSize(Resources::fontSize);

	enteredText.setFont(Resources::font);
	enteredText.setFillColor(sf::Color::Black);
	enteredText.setCharacterSize(Resources::fontSize);
}

void EnterButton::draw(sf::RenderTarget& target, sf::RenderStates states) const
{
	target.draw(button);
	target.draw(enteredText);
	target.draw(text);
}

void EnterButton::enterText()
{
	if (isSelected)
	{
		if (event.type == sf::Event::KeyPressed)
		{
			if (event.key.code == sf::Keyboard::Backspace)
			{
				if (enteredText.getString().getSize() != 0)
				{
					std::string string;
					string = std::string(enteredText.getString());
					string.resize(string.size() - 1);

					enteredText.setString(string);
				}
			}
		}

		if (event.type == sf::Event::TextEntered)
		{
			if (event.text.unicode < 128 && event.text.unicode != 8)
			{
				enteredText.setString(enteredText.getString() + static_cast<char>(event.text.unicode));
			}
		}
	}
}

void EnterButton::select(sf::Vector2f mouse)
{
	if (event.type == sf::Event::MouseButtonPressed)
	{
		isSelected = button.click(mouse);
	}
}

void EnterButton::events(sf::Vector2f mouse)
{
	select(mouse);

	enterText();
}

void EnterButton::input(sf::Vector2f mouse)
{
	button.aim(mouse);
}

const std::string EnterButton::getString()
{
	return enteredText.getString();
}

void EnterButton::setString(std::string string)
{
	text.setString(string);
}

void EnterButton::setTexture(sf::Texture& texture)
{
	button.setTexture(texture);
}

void EnterButton::setPosition(sf::Vector2f position)
{
	button.setPosition(position);

	enteredText.setPosition(position.x, position.y + (button.getGlobalBounds().height - text.getGlobalBounds().height) / 2 - 6);

	text.setPosition(
		position.x - (text.getGlobalBounds().width + button.getGlobalBounds().height / 2),
		position.y + (button.getGlobalBounds().height - text.getGlobalBounds().height) / 2 - 6
	);
}

void EnterButton::setPosition(float x, float y)
{
	setPosition(sf::Vector2f(x, y));
}

sf::FloatRect EnterButton::getGlobalBounds()
{
	return button.getGlobalBounds();
}