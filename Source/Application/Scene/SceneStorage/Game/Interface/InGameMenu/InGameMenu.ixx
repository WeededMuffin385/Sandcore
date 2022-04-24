#include <SFML/Graphics.hpp>

class Game;


export module Sandcore.Scene.Game.Interface.InGameMenu;

export import Sandcore.Interface.Button.Text;

export class InGameMenu
{
public:

	InGameMenu(sf::RenderWindow& window, sf::Event& event, Game& game);

	void draw();
	void events();
	void input();
	void resize();

	TextButton continueGame;
	TextButton settings;
	TextButton exitGame;

private:

	int backgroundBorder = 64;

	sf::Vector2f mouse;

	bool run = false;

	sf::RenderWindow& window;
	sf::Event& event;

	Game& game;

	sf::Sprite background;
};