#include <SFML/Graphics.hpp>

import Sandcore.Scene.Game.Controls;

import Sandcore.World.Player;
import Sandcore.World.Entity;

Controls::Controls(sf::Event& event, Player*& player) : event(event), player(player)
{

}

void Controls::input()
{
	player->getEntity()->setMoveCondition(generateMoveCondition());
}

void Controls::events()
{

}

MoveCondition Controls::generateMoveCondition()
{
	MoveCondition moveCondition;

	moveCondition.jump = sf::Keyboard::isKeyPressed(sf::Keyboard::Space);

	moveCondition.fall = sf::Keyboard::isKeyPressed(sf::Keyboard::LAlt);

	if (sf::Keyboard::isKeyPressed(sf::Keyboard::C)) moveCondition.moveState = MoveState::sneak;

	if (sf::Keyboard::isKeyPressed(sf::Keyboard::LShift)) moveCondition.moveState = MoveState::run;

	moveCondition.up = sf::Keyboard::isKeyPressed(sf::Keyboard::W);

	moveCondition.down = sf::Keyboard::isKeyPressed(sf::Keyboard::S);

	moveCondition.left = sf::Keyboard::isKeyPressed(sf::Keyboard::A);

	moveCondition.right = sf::Keyboard::isKeyPressed(sf::Keyboard::D);

	return moveCondition;
}