#include <SFML/Graphics.hpp>
#include <fstream>

import <chrono>;

import Sandcore.Application;

import Sandcore.Scene.Menu.MainMenu;

import Sandcore.Resources;

Application::Application()
{
	load();

	sceneManager.push(std::make_unique<MainMenu>(window, event, sceneManager));
}

#include <iostream>

void Application::loop()
{
	while (!sceneManager.isEmpty())
	{
		sceneManager.draw();

		sceneManager.update();
		/**
			—начала отрисовка, а потом обновление потому, что во врем€ update провер€етс€ run сцены и еЄ удаление из стека, а так как
			при выходе из последней сцены отрисовывать нечего, то по€вл€етс€ ошибка, можно провер€ть наличие сцен в середине, но это лишн€€
			нагрузка.
			**/
	}
}

void Application::load()
{
	Resources::load("settings.txt");

	windowInit(window);
}

void Application::windowInit(sf::RenderWindow& window)
{
	std::ifstream file(Resources::userData / "settings" / "window.txt");

	int x;
	int y;

	std::string mode;

	while (!file.eof())
	{
		std::string identification;
		file >> identification;

		if (identification == "x") file >> x;
		if (identification == "y") file >> y;

		if (identification == "mode") file >> mode;
	}

	std::string name;

	file.close();
	file.open(Resources::userData / "settings" / "window_names.txt");

	int n =
	std::count(std::istreambuf_iterator<char>(file),
		std::istreambuf_iterator<char>(), '\n') + 1;

	file.clear();
	file.seekg(0);

	srand(std::chrono::duration_cast<std::chrono::milliseconds>(std::chrono::system_clock::now().time_since_epoch()).count());
	n = rand() % n;

	for (int i = 1; i < n; ++i) std::getline(file, name);

	window.create(sf::VideoMode(x, y), "Sandcore SFML " + name, sf::Style::Titlebar | sf::Style::Close | (mode == "fullscreen" ? sf::Style::Fullscreen : 0));

	window.setIcon(Resources::icon.getSize().x, Resources::icon.getSize().y, Resources::icon.getPixelsPtr());
}