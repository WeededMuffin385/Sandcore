#include <SFML/Graphics.hpp>

import Sandcore.Scene.Menu.WorldSettingsMenu;

import Sandcore.Resources;

WorldSettingsMenu::WorldSettingsMenu(sf::RenderWindow& window, sf::Event& event, SceneManager& sceneManager, SingleplayerMenu& singleplayerMenu, std::filesystem::path path) :
	Scene(window, event, sceneManager), 
	singleplayerMenu(singleplayerMenu),
	path(path),
	deleteWorldButton(window, event),
	deleteWorld(window, event, *this)
{
	background.setTexture(Resources::textureMap[ResourcesIdentification::background]);

	deleteWorldButton.setString("delete world");
	deleteWorldButton.setTexture(Resources::textureMap[ResourcesIdentification::button]);
	deleteWorldButton.setPosition((window.getSize().x - deleteWorldButton.getGlobalBounds().width) / 2, window.getSize().y - deleteWorldButton.getGlobalBounds().height);
}

void WorldSettingsMenu::update()
{
	mouse = static_cast<sf::Vector2f>(sf::Mouse::getPosition(window));

	input();

	while (window.pollEvent(event))
	{
		events();
	}
}

void WorldSettingsMenu::draw()
{
	window.clear(sf::Color::White);

	window.draw(background);

	window.draw(deleteWorldButton);

	deleteWorld.draw();

	window.display();
}


void WorldSettingsMenu::events()
{
	if (event.type == sf::Event::KeyPressed)
	{
		if (event.key.code == sf::Keyboard::Escape)
		{
			run = false;
		}
	}

	if (deleteWorldButton.click(mouse)) deleteWorld.run = true;

	deleteWorld.events();
}

void WorldSettingsMenu::input()
{
	deleteWorldButton.aim(mouse);

	deleteWorld.input();
}