#include <SFML/Graphics.hpp>
#include <sstream>
#include <fstream>

import Sandcore.Scene.Menu.SettingsMenu;

import Sandcore.Resources;

import Sandcore.Scene.Manager;
import Sandcore.Application;


SettingsMenu::SettingsMenu(sf::RenderWindow& window, sf::Event& event, SceneManager& sceneManager) :
	Scene(window, event, sceneManager),
	apply(window, event),
	resolution(window, event),
	windowMode(window, event)
{
	background.setTexture(Resources::textureMap[ResourcesIdentification::background]);

	apply.setTexture(Resources::textureMap[ResourcesIdentification::button]);
	apply.setPosition(window.getSize().x / 2 - apply.getGlobalBounds().width / 2, window.getSize().y - apply.getGlobalBounds().height); // если кнопки будут разными, то будет ошибка т.к. везде юзается singleplayerMenuButton
	apply.setString("Apply settings");

	resolution.setTexture(Resources::textureMap[ResourcesIdentification::button]);
	resolution.setPosition(0, 0);
	resolution.setString("Resolution");

	windowMode.setTexture(Resources::textureMap[ResourcesIdentification::button]);
	windowMode.setPosition(resolution.getGlobalBounds().left + resolution.getGlobalBounds().width + 8, 0);
	windowMode.setString("Window mode");

	load();
}

void SettingsMenu::resize()
{
	apply.setPosition(window.getSize().x / 2 - apply.getGlobalBounds().width / 2, window.getSize().y - apply.getGlobalBounds().height);
	resolution.setPosition(0, 0);
	windowMode.setPosition(resolution.getGlobalBounds().left + resolution.getGlobalBounds().width + 8, 0);
}

void SettingsMenu::update()
{
	mouse = static_cast<sf::Vector2f>(sf::Mouse::getPosition(window));

	input();

	while (window.pollEvent(event))
	{
		events();
	}
}

void SettingsMenu::draw()
{
	window.clear(sf::Color::White);
	window.draw(background);
	window.draw(apply);
	window.draw(resolution);
	window.draw(windowMode);
	window.display();
}

void SettingsMenu::events()
{
	if (event.type == sf::Event::KeyPressed)
	{
		if (event.key.code == sf::Keyboard::Escape)
		{
			run = false;
		}
	}

	resolution.events(mouse);
	windowMode.events(mouse);

	if (apply.click(mouse))
	{
		save();
		Application::windowInit(window);
		sceneManager.resize();
	}
}

void SettingsMenu::input()
{
	apply.aim(mouse);

	resolution.input(mouse);

	windowMode.input(mouse);
}

void SettingsMenu::load()
{
	loadWindowSettings();
	loadWindowOptions();
}

void SettingsMenu::loadWindowSettings()
{
	std::ifstream file(Resources::userData / "settings" / "window.txt");

	int x = 500;
	int y = 500;

	std::string style = "windowed";

	while (!file.eof())
	{
		std::string identification;

		file >> identification;


		if (identification == "x") file >> x;

		if (identification == "y") file >> y;

		if (identification == "mode") file >> style;
	}

	if (style == "windowed") windowMode.setString("windowed");

	if (style == "fullscreen") windowMode.setString("fullscreen");

	resolution.setString("Resolution " + std::to_string(x) + " " + std::to_string(y));
}

void SettingsMenu::loadWindowOptions()
{
	std::vector<sf::VideoMode> videoModeVector = sf::VideoMode::getFullscreenModes();

	for (int i = 0; i < videoModeVector.size(); ++i)
	{
		resolution.addOption("Resolution " + std::to_string(videoModeVector[i].width) + " " + std::to_string(videoModeVector[i].height));
	}

	windowMode.addOption("fullscreen");
	windowMode.addOption("windowed");
}

void SettingsMenu::save()
{
	saveWindowSettings();
}

void SettingsMenu::saveWindowSettings()
{
	std::istringstream stringStream(resolution.getString());

	int x;
	int y;

	std::string none;

	stringStream >> none;

	stringStream >> x;
	stringStream >> y;

	std::ofstream file(Resources::userData / "settings" / "window.txt");

	file <<
		"x " << x << "\n" <<
		"y " << y << "\n" <<
		(windowMode.getString() == "fullscreen" ? "mode fullscreen" : "mode windowed");

	file.close();
}