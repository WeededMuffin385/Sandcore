#include <SFML/Graphics.hpp>

export module Sandcore.Application;

import Sandcore.Scene.Manager;

export class Application
{
public:

	Application();

	void loop();

private:

	void load();

	static void windowInit(sf::RenderWindow&window);

	SceneManager sceneManager;

	sf::RenderWindow window;

	sf::Event event;

	friend class SettingsMenu;
};