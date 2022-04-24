#include <SFML/Graphics.hpp>

import Sandcore.Interface.Button.Options;

import Sandcore.Resources;

OptionsButton::OptionsButton(sf::RenderWindow& window, sf::Event& event) : window(window), event(event), button(window, event),
view(sf::FloatRect(0.f, 0.f, Resources::textureMap[ResourcesIdentification::button].getSize().x, window.getSize().y - 128 - 16))
{
	text.setFont(Resources::font);
	text.setFillColor(sf::Color::Green);
	text.setCharacterSize(Resources::fontSize);

	renderTexture.create(Resources::textureMap[ResourcesIdentification::button].getSize().x, window.getSize().y - 128 - 16);
	sprite.setTexture(renderTexture.getTexture(), true);
}

void OptionsButton::addOption(std::string title)
{
	int i = optionsVector.size();

	optionsVector.push_back(TextButton(window, event));
	optionsVector[i].setTexture(Resources::textureMap[ResourcesIdentification::button]);
	optionsVector[i].setString(title);
	optionsVector[i].setColor(sf::Color(255, 255, 127, 192), sf::Color(127, 255, 127, 192)); // normalColor(255, 255, 255, 192), aimedColor(127, 255, 127, 192)
	optionsVector[i].setPosition(0, (button.getGlobalBounds().height + (button.getGlobalBounds().height / 8)) * i);
}

void OptionsButton::setTexture(sf::Texture& texture)
{
	button.setTexture(texture);
}

void OptionsButton::setPosition(sf::Vector2f position)
{
	button.setPosition(position);

	text.setPosition(
		position.x - (text.getGlobalBounds().width + button.getGlobalBounds().height / 2),
		position.y + (button.getGlobalBounds().height - text.getGlobalBounds().height) / 2 - 6
	);

	sprite.setPosition(button.getGlobalBounds().left, button.getGlobalBounds().top + 64 + 8);
}

void OptionsButton::setPosition(float x, float y)
{
	setPosition(sf::Vector2f(x, y));
}

void OptionsButton::draw(sf::RenderTarget& target, sf::RenderStates states) const
{
	target.draw(button);
	target.draw(sprite);
}

void OptionsButton::update()
{
	renderTexture.clear(sf::Color::Transparent);
	renderTexture.setView(view);
	if (isSelected) for (int i = 0; i < optionsVector.size(); ++i) renderTexture.draw(optionsVector[i]);
	renderTexture.display();
}

void OptionsButton::select(sf::Vector2f mouse)
{
	if (event.type == sf::Event::MouseButtonPressed)
	{
		isSelected = button.click(mouse);
	}
}

void OptionsButton::events(sf::Vector2f mouse)
{
	if (isSelected)
	{
		for (int i = 0; i < optionsVector.size(); ++i) if (optionsVector[i].click(renderTextureMouse)) button.setString(optionsVector[i].getString());

		if (event.type == sf::Event::MouseWheelScrolled)
		{
			if (event.mouseWheelScroll.wheel == sf::Mouse::VerticalWheel)
			{
				if (event.mouseWheelScroll.delta == 1 && view.getCenter().y > view.getSize().y / 2)
				{
					view.move(sf::Vector2f(0.f, -16));
				}

				if (event.mouseWheelScroll.delta == -1 && view.getCenter().y < -view.getSize().y / 2 + optionsVector.size() * (button.getGlobalBounds().height + button.getGlobalBounds().height / 8))
				{
					view.move(sf::Vector2f(0.f, 16));
				}
			}
		}
	}

	select(mouse);
}

void OptionsButton::input(sf::Vector2f mouse)
{
	renderTextureMouse = renderTexture.mapPixelToCoords(sf::Mouse::getPosition(window) - static_cast<sf::Vector2i>(sprite.getPosition()));

	button.aim(mouse);

	for (int i = 0; i < optionsVector.size(); ++i) optionsVector[i].aim(renderTextureMouse);

	update();
}

void OptionsButton::setString(std::string string)
{
	button.setString(string);
}

std::string OptionsButton::getString()
{
	return button.getString();
}

sf::FloatRect OptionsButton::getGlobalBounds()
{
	return button.getGlobalBounds();
}