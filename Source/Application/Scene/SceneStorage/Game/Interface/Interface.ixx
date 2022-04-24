#include <SFML/Graphics.hpp>

class Game;
class Player;

export module Sandcore.Scene.Game.Interface;

import Sandcore.Clock;
import Sandcore.Scene.Game.Interface.InGameMenu;

export class Interface
{
public:

	Interface(sf::RenderWindow& window, sf::Event& event, Game& game);

	void draw();
	void events();
	void input();
	void resize();

private:

	void update();

	sf::Text fps;
	sf::Text playerPosition;
	sf::Text mousePosition;

	InGameMenu inGameMenu;

	Clock fpsClock;

	sf::RenderWindow& window;
	sf::Event& event;

	Game& game;
};