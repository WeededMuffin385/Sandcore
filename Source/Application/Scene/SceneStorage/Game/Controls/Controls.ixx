#include <SFML/Graphics.hpp>

class Player;

export module Sandcore.Scene.Game.Controls;

export import Sandcore.World.Entity.MoveCondition;

export class Controls
{
public:
	Controls(sf::Event& event, Player*& player);

	void input();
	void events();

private:

	MoveCondition generateMoveCondition();

	Player*& player;
	sf::Event& event;
};