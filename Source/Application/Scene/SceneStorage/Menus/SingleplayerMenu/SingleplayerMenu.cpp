#include <SFML/Graphics.hpp>
#include <fstream>
#include <stdexcept>

import Sandcore.Scene.Menu.SingleplayerMenu;

import Sandcore.Scene.Menu.GenerateWorld;
import Sandcore.Resources;
import Sandcore.Scene.Menu.WorldSettingsMenu;
import Sandcore.Scene.Game;


SingleplayerMenu::SingleplayerMenu(sf::RenderWindow& window, sf::Event& event, SceneManager& sceneManager) :
	Scene(window, event, sceneManager),
	view(sf::FloatRect(0.f, 0.f, window.getSize().x, window.getSize().y - generateWorldButton.getGlobalBounds().height - generateWorldButton.getGlobalBounds().height / 8)), generateWorldButton(window, event)
{
	background.setTexture(Resources::textureMap[ResourcesIdentification::background]);

	generateWorldButton.setTexture(Resources::textureMap[ResourcesIdentification::button]);
	generateWorldButton.setString("Generate new world");
	generateWorldButton.setPosition(window.getSize().x / 2 - generateWorldButton.getGlobalBounds().width / 2, window.getSize().y - generateWorldButton.getGlobalBounds().height);

	renderTexture.create(window.getSize().x, window.getSize().y - generateWorldButton.getGlobalBounds().height - generateWorldButton.getGlobalBounds().height / 8);
	sprite.setTexture(renderTexture.getTexture());

	worldsPath = Resources::userData / "worlds";

	loadWorldButtons();
}

void SingleplayerMenu::resize()
{
	view = sf::View(sf::FloatRect(0.f, 0.f, window.getSize().x, window.getSize().y - generateWorldButton.getGlobalBounds().height - generateWorldButton.getGlobalBounds().height / 8));

	background.setTexture(Resources::textureMap[ResourcesIdentification::background], true);
	generateWorldButton.setPosition(window.getSize().x / 2 - generateWorldButton.getGlobalBounds().width / 2, window.getSize().y - generateWorldButton.getGlobalBounds().height);
	renderTexture.create(window.getSize().x, window.getSize().y - generateWorldButton.getGlobalBounds().height - generateWorldButton.getGlobalBounds().height / 8);
	sprite.setTexture(renderTexture.getTexture(), true);

	loadWorldButtons();
}

void SingleplayerMenu::update()
{
	renderTextureMouse = renderTexture.mapPixelToCoords(sf::Vector2i(sf::Mouse::getPosition(window)));
	mouse = static_cast<sf::Vector2f>(sf::Mouse::getPosition(window));

	input();

	while (window.pollEvent(event))
	{
		events();
	}

}

void SingleplayerMenu::draw()
{
	renderTexture.clear(sf::Color(255, 255, 255, 0));

	for (size_t i = 0; i < worldButtonVector.size(); ++i)
	{
		renderTexture.draw(worldButtonVector[i]);
	}

	renderTexture.setView(view);
	renderTexture.display();

	window.clear(sf::Color::White);
	window.draw(background);
	window.draw(generateWorldButton);
	window.draw(sprite);

	window.display();
}

void SingleplayerMenu::events()
{
	if (event.type == sf::Event::KeyPressed)
	{
		if (event.key.code == sf::Keyboard::Escape)
		{
			run = false;
		}
	}

	if (generateWorldButton.click(mouse))
	{
		sceneManager.push(std::make_unique<GenerateWorldMenu>(window, event, sceneManager, *this));
	}

	for (size_t i = 0; i < worldButtonVector.size(); ++i)
	{
		if (0 < sf::Mouse::getPosition(window).y && sf::Mouse::getPosition(window).y < renderTexture.getSize().y) // сделано для того, чтобы вне листа серверов кнопки серверов не нажимались
		{
			switch (worldButtonVector[i].click(renderTextureMouse))
			{
			case WorldButtonClickIdentification::name:
				loadWorld(i);
				break;

			case WorldButtonClickIdentification::settings:
				sceneManager.push(std::make_unique<WorldSettingsMenu>(window, event, sceneManager, *this, worldButtonVector[i].getPath()));
				break;
			}
		}
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

void SingleplayerMenu::input()
{
	for (size_t i = 0; i < worldButtonVector.size(); ++i)
	{
		if (0 < sf::Mouse::getPosition(window).y && sf::Mouse::getPosition(window).y < renderTexture.getSize().y) // сделано для того, чтобы вне листа серверов кнопки серверов не нажимались
		{
			worldButtonVector[i].aim(renderTextureMouse);
		}
	}

	generateWorldButton.aim(mouse);
}

void SingleplayerMenu::addWorldButton(std::string name, std::filesystem::path path)
{
	int i = worldButtonVector.size();

	worldButtonVector.push_back(WorldButton(window, event, name, path));

	worldButtonVector[i].setTexture(Resources::textureMap[ResourcesIdentification(4 + rand() % 4)], Resources::textureMap[ResourcesIdentification::settings], Resources::textureMap[ResourcesIdentification::button]);

	worldButtonVector[i].setPosition(window.getSize().x / 2 - worldButtonVector[i].getGlobalBounds().width / 2, ((worldButtonVector[i].getGlobalBounds().height) + (worldButtonVector[i].getGlobalBounds().height / 8)) * i);
}

void SingleplayerMenu::loadWorldButtons()
{
	if (!worldButtonVector.empty()) worldButtonVector.clear();

	for (const auto& worldPath : std::filesystem::directory_iterator(worldsPath))
	{
		std::string name;

		std::ifstream file(worldPath.path() / "parameters.txt");

		while (!file.eof())
		{
			std::string identification;
			file >> identification;

			if (identification == "name")
			{
				std::getline(file, name);
			}
		}

		addWorldButton(name, worldPath);
	}
}

void SingleplayerMenu::loadWorld(int i)
{
	std::filesystem::path path = worldButtonVector[i].getPath();

	if (std::filesystem::exists(path)) sceneManager.push(std::make_unique<Game>(window, event, sceneManager, path));
}