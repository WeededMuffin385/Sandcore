#include <SFML/Graphics.hpp>

import Sandcore.Scene.Menu.MainMenu;

import Sandcore.Scene.Menu.SingleplayerMenu;
import Sandcore.Scene.Menu.SettingsMenu;
import Sandcore.Resources;

MainMenu::MainMenu(sf::RenderWindow& window, sf::Event& event, SceneManager& sceneManager) :
	Scene(window, event, sceneManager),
	singleplayerMenuButton(window, event),
	multiplayerMenuButton(window, event),
	settingsMenuButton(window, event),
	exitButton(window, event)
{
	background.setTexture(Resources::textureMap[ResourcesIdentification::background]);

	singleplayerMenuButton.setTexture(Resources::textureMap[ResourcesIdentification::button]);
	multiplayerMenuButton.setTexture(Resources::textureMap[ResourcesIdentification::button]);
	settingsMenuButton.setTexture(Resources::textureMap[ResourcesIdentification::button]);
	exitButton.setTexture(Resources::textureMap[ResourcesIdentification::button]);

	singleplayerMenuButton.setPosition(sf::Vector2f(window.getSize().x / 2 - singleplayerMenuButton.getGlobalBounds().width / 2, window.getSize().y / 5 - singleplayerMenuButton.getGlobalBounds().height / 2)); // если кнопки будут разными, то будет ошибка т.к. везде юзается singleplayerMenuButton
	multiplayerMenuButton.setPosition(sf::Vector2f(window.getSize().x / 2 - singleplayerMenuButton.getGlobalBounds().width / 2, window.getSize().y / 5 * 2 - singleplayerMenuButton.getGlobalBounds().height / 2));
	settingsMenuButton.setPosition(sf::Vector2f(window.getSize().x / 2 - singleplayerMenuButton.getGlobalBounds().width / 2, window.getSize().y / 5 * 3 - singleplayerMenuButton.getGlobalBounds().height / 2));
	exitButton.setPosition(sf::Vector2f(window.getSize().x / 2 - singleplayerMenuButton.getGlobalBounds().width / 2, window.getSize().y / 5 * 4 - singleplayerMenuButton.getGlobalBounds().height / 2));

	singleplayerMenuButton.setString("Singleplayer");
	multiplayerMenuButton.setString("Multiplayer");
	settingsMenuButton.setString("Settings");
	exitButton.setString("Exit");
}

void MainMenu::draw()
{
	window.clear(sf::Color::White);
	window.draw(background);
	window.draw(singleplayerMenuButton);
	window.draw(multiplayerMenuButton);
	window.draw(settingsMenuButton);
	window.draw(exitButton);
	window.display();
}

void MainMenu::update()
{
	mouse = static_cast<sf::Vector2f>(sf::Mouse::getPosition(window));

	input();

	while (window.pollEvent(event))
	{
		events();
	}
}

void MainMenu::events()
{
	if (singleplayerMenuButton.click(mouse))
	{
		sceneManager.push(std::make_unique<SingleplayerMenu>(window, event, sceneManager));
	}

	if (multiplayerMenuButton.click(mouse))
	{
		/*
		multiplayer_menu multiplayer_menu;
		multiplayer_menu.loop();
		*/
	}

	if (settingsMenuButton.click(mouse))
	{
		sceneManager.push(std::make_unique<SettingsMenu>(window, event, sceneManager));
	}

	if (exitButton.click(mouse))
	{
		run = false;
	}
}

void MainMenu::resize()
{
	singleplayerMenuButton.setPosition(sf::Vector2f(window.getSize().x / 2 - singleplayerMenuButton.getGlobalBounds().width / 2, window.getSize().y / 5 - singleplayerMenuButton.getGlobalBounds().height / 2)); // если кнопки будут разными, то будет ошибка т.к. везде юзается singleplayerMenuButton
	multiplayerMenuButton.setPosition(sf::Vector2f(window.getSize().x / 2 - singleplayerMenuButton.getGlobalBounds().width / 2, window.getSize().y / 5 * 2 - singleplayerMenuButton.getGlobalBounds().height / 2));
	settingsMenuButton.setPosition(sf::Vector2f(window.getSize().x / 2 - singleplayerMenuButton.getGlobalBounds().width / 2, window.getSize().y / 5 * 3 - singleplayerMenuButton.getGlobalBounds().height / 2));
	exitButton.setPosition(sf::Vector2f(window.getSize().x / 2 - singleplayerMenuButton.getGlobalBounds().width / 2, window.getSize().y / 5 * 4 - singleplayerMenuButton.getGlobalBounds().height / 2));
}

void MainMenu::input()
{
	singleplayerMenuButton.aim(mouse);
	multiplayerMenuButton.aim(mouse);
	settingsMenuButton.aim(mouse);
	exitButton.aim(mouse);
}