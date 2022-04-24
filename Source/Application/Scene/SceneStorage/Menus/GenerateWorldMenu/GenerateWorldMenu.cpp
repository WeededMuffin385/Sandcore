#include <SFML/Graphics.hpp>
#include <iostream>

import Sandcore.Scene.Menu.GenerateWorld;

import Sandcore.World.Parameters;
import Sandcore.Resources;
import Sandcore.World;

GenerateWorldMenu::GenerateWorldMenu(sf::RenderWindow& window, sf::Event& event, SceneManager& sceneManager, SingleplayerMenu& singleplayerMenu) :
	Scene(window, event, sceneManager),
	singleplayerMenu(singleplayerMenu),
	generateWorldButton(window, event), view(sf::FloatRect(0.f, 0.f, window.getSize().x, window.getSize().y - generateWorldButton.getGlobalBounds().height - generateWorldButton.getGlobalBounds().height / 8))
{
	background.setTexture(Resources::textureMap[ResourcesIdentification::background]);

	generateWorldButton.setTexture(Resources::textureMap[ResourcesIdentification::button]);
	generateWorldButton.setString("Generate new world");
	generateWorldButton.setPosition(window.getSize().x / 2 - generateWorldButton.getGlobalBounds().width / 2, window.getSize().y - generateWorldButton.getGlobalBounds().height);

	renderTexture.create(window.getSize().x, window.getSize().y - generateWorldButton.getGlobalBounds().height - generateWorldButton.getGlobalBounds().height / 8);
	sprite.setTexture(renderTexture.getTexture());

	addEnterButton("world name", "new world");

	srand(std::chrono::system_clock::now().time_since_epoch().count());
	addEnterButton("seed", std::to_string(rand()));

	addEnterButton("terrain scale", "75");
	addEnterButton("terrain height", "32");

	addEnterButton("water height", "16");

	addEnterButton("cave scale", "35");
	addEnterButton("cave size", "0.7");

	addEnterButton("iron ore scale", "15");
	addEnterButton("iron ore size", "0.75");

	addEnterButton("coal ore scale", "30");
	addEnterButton("coal ore size", "0.6");

	addEnterButton("gold ore scale", "10");
	addEnterButton("gold ore size", "0.9");
}

void GenerateWorldMenu::addEnterButton(std::string title, std::string defaultValue)
{
	int i = enterButtonVector.size();

	enterButtonVector.push_back(EnterButton(window, event));

	enterButtonVector[i].setTexture(Resources::textureMap[ResourcesIdentification::button]);
	enterButtonVector[i].setString(title);
	enterButtonVector[i].enteredText.setString(defaultValue);
	enterButtonVector[i].setPosition(window.getSize().x / 2 - enterButtonVector[i].getGlobalBounds().width / 2, ((enterButtonVector[i].getGlobalBounds().height) + (enterButtonVector[i].getGlobalBounds().height / 8)) * i);
}

void GenerateWorldMenu::update()
{
	renderTextureMouse = renderTexture.mapPixelToCoords(sf::Vector2i(sf::Mouse::getPosition(window)));
	mouse = static_cast<sf::Vector2f>(sf::Mouse::getPosition(window));

	input();

	while (window.pollEvent(event))
	{
		events();
	}
}

void GenerateWorldMenu::draw()
{
	renderTexture.clear(sf::Color(255, 255, 255, 0));
	for (int i = 0; i < enterButtonVector.size(); ++i)
	{
		renderTexture.draw(enterButtonVector[i]);
	}
	renderTexture.setView(view);
	renderTexture.display();

	window.clear(sf::Color::White);
	window.draw(background);
	window.draw(sprite);
	window.draw(generateWorldButton);
	window.display();
}

void GenerateWorldMenu::events()
{
	if (event.type == sf::Event::KeyPressed)
	{
		if (event.key.code == sf::Keyboard::Escape)
		{
			run = false;
		}
	}

	for (size_t i = 0; i < enterButtonVector.size(); ++i)
	{
		if (0 < sf::Mouse::getPosition(window).y && sf::Mouse::getPosition(window).y < renderTexture.getSize().y) // сделано для того, чтобы вне листа серверов кнопки серверов не нажимались
		{
			enterButtonVector[i].events(renderTextureMouse);
		}
	}

	if (generateWorldButton.click(mouse))
	{
		if (enterButtonVector[0].getString() == "")
		{
			std::cout << "You can't create world without name\n";
			return;
		}

		if (std::filesystem::exists(Resources::userData / "worlds" / enterButtonVector[0].getString()))
		{
			std::cout << "World with name \"" << enterButtonVector[0].getString() << "\" already exists\n";
			return;
		}

		{
			Parameters parameters;

			parameters.name = enterButtonVector[0].getString();

			parameters.seed = std::atof(enterButtonVector[1].getString().c_str());

			parameters.terrainScale = std::atof(enterButtonVector[2].getString().c_str());
			parameters.terrainHeight = std::atof(enterButtonVector[3].getString().c_str());

			parameters.waterHeight = std::atof(enterButtonVector[4].getString().c_str());

			parameters.caveScale = std::atof(enterButtonVector[5].getString().c_str());
			parameters.caveSize = std::atof(enterButtonVector[6].getString().c_str());

			parameters.ironOreScale = std::atof(enterButtonVector[7].getString().c_str());
			parameters.ironOreSize = std::atof(enterButtonVector[8].getString().c_str());

			parameters.coalOreScale = std::atof(enterButtonVector[9].getString().c_str());
			parameters.coalOreSize = std::atof(enterButtonVector[10].getString().c_str());

			parameters.goldOreScale = std::atof(enterButtonVector[11].getString().c_str());
			parameters.goldOreSize = std::atof(enterButtonVector[12].getString().c_str());

			World::createDirectory(Resources::userData / "worlds" / enterButtonVector[0].getString(), parameters);
		}

		singleplayerMenu.loadWorldButtons();
		run = false;
	}

	if (event.type == sf::Event::MouseWheelScrolled)
	{
		if (event.mouseWheelScroll.wheel == sf::Mouse::VerticalWheel)
		{
			if (event.mouseWheelScroll.delta == 1)
			{
				view.move(sf::Vector2f(0.f, -16));
			}

			if (event.mouseWheelScroll.delta == -1)
			{
				view.move(sf::Vector2f(0.f, 16));
			}
		}
	}
}

void GenerateWorldMenu::input()
{
	for (int i = 0; i < enterButtonVector.size(); ++i)
	{
		enterButtonVector[i].input(renderTextureMouse);
	}

	for (size_t i = 0; i < enterButtonVector.size(); ++i)
	{
		if (0 < sf::Mouse::getPosition(window).y && sf::Mouse::getPosition(window).y < renderTexture.getSize().y) // сделано для того, чтобы вне листа серверов кнопки серверов не нажимались
		{
			enterButtonVector[i].input(renderTextureMouse);
		}
	}

	generateWorldButton.aim(mouse);
}