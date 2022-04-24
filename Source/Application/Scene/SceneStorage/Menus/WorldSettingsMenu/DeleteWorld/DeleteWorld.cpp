#include <SFML/Graphics.hpp>

import Sandcore.Scene.Menu.WorldSettingsMenu.DeleteWorld;

import <filesystem>;

import Sandcore.Resources;
import Sandcore.Scene.Menu.WorldSettingsMenu;
import Sandcore.World;

DeleteWorld::DeleteWorld(sf::RenderWindow& window, sf::Event& event, WorldSettingsMenu& worldSettingsMenu) :
	window(window), event(event), worldSettingsMenu(worldSettingsMenu),
	yes(window, event),
	no(window, event)
{
	rectangle.setSize(sf::Vector2f(600, 300));
	rectangle.setPosition((window.getSize().x - rectangle.getGlobalBounds().width) / 2, (window.getSize().y - rectangle.getGlobalBounds().height) / 2);
	rectangle.setFillColor(sf::Color(127, 127, 127, 127));

	text.setString("Would you like to delete world?");
	text.setFont(Resources::font);
	text.setCharacterSize(Resources::fontSize);
	text.setPosition(rectangle.getGlobalBounds().left, rectangle.getGlobalBounds().top);
	text.setFillColor(sf::Color::Red);

	no.setTexture(Resources::textureMap[ResourcesIdentification::button]);
	yes.setTexture(Resources::textureMap[ResourcesIdentification::button]);

	no.setString("No, don't do it!!!");
	yes.setString("Yes, delete world!");

	no.setPosition((window.getSize().x - no.getGlobalBounds().width) / 2, rectangle.getGlobalBounds().top + rectangle.getGlobalBounds().height / 3);
	yes.setPosition((window.getSize().x - yes.getGlobalBounds().width) / 2, rectangle.getGlobalBounds().top + rectangle.getGlobalBounds().height / 3 * 2);
}

void DeleteWorld::draw()
{
	if (run)
	{
		window.draw(rectangle);

		window.draw(yes);
		window.draw(no);

		window.draw(text);
	}
}

#include <iostream>

void DeleteWorld::events()
{
	if (run)
	{
		if (yes.click(worldSettingsMenu.mouse))
		{
			World::deleteDirectory(worldSettingsMenu.path);

			worldSettingsMenu.singleplayerMenu.loadWorldButtons();

			worldSettingsMenu.run = false;
		}

		if (no.click(worldSettingsMenu.mouse)) run = false;
	}
}

void DeleteWorld::input()
{
	if (run)
	{
		yes.aim(worldSettingsMenu.mouse);
		no.aim(worldSettingsMenu.mouse);
	}
}