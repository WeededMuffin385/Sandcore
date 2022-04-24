#include <SFML/Graphics.hpp>

import Sandcore.Scene.Game.Interface.InGameMenu;
import Sandcore.Scene.Game;
import Sandcore.Scene.Manager;
import Sandcore.Scene;
import Sandcore.Resources;

import Sandcore.Scene.Menu.SettingsMenu;

import <memory>;

InGameMenu::InGameMenu(sf::RenderWindow& window, sf::Event& event, Game& game) : window(window), event(event), game(game), continueGame(window, event), exitGame(window, event), settings(window, event)
{
	continueGame.setTexture(Resources::textureMap[ResourcesIdentification::button]);
	continueGame.setString("continue game");
	continueGame.setPosition(window.getSize().x / 2 - continueGame.getGlobalBounds().width / 2, window.getSize().y / 4 - continueGame.getGlobalBounds().height / 2);

	settings.setTexture(Resources::textureMap[ResourcesIdentification::button]);
	settings.setString("settings");
	settings.setPosition(window.getSize().x / 2 - settings.getGlobalBounds().width / 2, window.getSize().y / 4 * 2 - settings.getGlobalBounds().height / 2);

	exitGame.setTexture(Resources::textureMap[ResourcesIdentification::button]);
	exitGame.setString("exit game");
	exitGame.setPosition(window.getSize().x / 2 - exitGame.getGlobalBounds().width / 2, window.getSize().y / 4 * 3 - exitGame.getGlobalBounds().height / 2);

	background.setTexture(Resources::textureMap[ResourcesIdentification::drawableBackground]);
	background.setTextureRect(sf::IntRect(continueGame.getGlobalBounds().left - backgroundBorder, 0, continueGame.getGlobalBounds().width + backgroundBorder * 2, window.getSize().y));
	background.setPosition(continueGame.getGlobalBounds().left - backgroundBorder, 0);
	background.setColor(sf::Color(127, 127, 127, 127));
}

void InGameMenu::resize()
{
	continueGame.setPosition(window.getSize().x / 2 - continueGame.getGlobalBounds().width / 2, window.getSize().y / 4 - continueGame.getGlobalBounds().height / 2);
	settings.setPosition(window.getSize().x / 2 - settings.getGlobalBounds().width / 2, window.getSize().y / 4 * 2 - settings.getGlobalBounds().height / 2);
	exitGame.setPosition(window.getSize().x / 2 - exitGame.getGlobalBounds().width / 2, window.getSize().y / 4 * 3 - exitGame.getGlobalBounds().height / 2);

	background.setTextureRect(sf::IntRect(continueGame.getGlobalBounds().left - backgroundBorder, 0, continueGame.getGlobalBounds().width + backgroundBorder * 2, window.getSize().y));
	background.setPosition(continueGame.getGlobalBounds().left - backgroundBorder, 0);
}


void InGameMenu::draw()
{
	if (run)
	{
		window.draw(background);
		window.draw(continueGame);
		window.draw(settings);
		window.draw(exitGame);
	}
}

void InGameMenu::events()
{
	mouse = static_cast<sf::Vector2f>(sf::Mouse::getPosition(window));

	if (event.type == sf::Event::KeyPressed)
	{
		if (event.key.code == sf::Keyboard::Escape)
		{
			run = !run;
		}
	}

	if (run)
	{
		if (continueGame.click(mouse)) run = false;

		if (settings.click(mouse))
		{
			game.sceneManager.push(std::make_unique<SettingsMenu>(window, event, game.sceneManager));
		}

		if (exitGame.click(mouse))
		{
			game.run = false;
		}
	}
}

void InGameMenu::input()
{
	if (run)
	{
		continueGame.aim(mouse);
		settings.aim(mouse);
		exitGame.aim(mouse);
	}
}