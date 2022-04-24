#include <SFML/Graphics.hpp>

class WorldSettingsMenu;

export module Sandcore.Scene.Menu.WorldSettingsMenu.DeleteWorld;

import Sandcore.Interface.Button.Text;

export class DeleteWorld
{
public:

	DeleteWorld(sf::RenderWindow& window, sf::Event& event, WorldSettingsMenu& worldSettingsMenu);

	void draw();

	void events();

	void input();

	bool run = false;

private:

	sf::RenderWindow& window;
	sf::Event& event;

	WorldSettingsMenu& worldSettingsMenu;

	sf::RectangleShape rectangle;
	sf::Text text;

	TextButton yes;
	TextButton no;
};