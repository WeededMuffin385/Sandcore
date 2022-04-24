#include <SFML/Graphics.hpp>

import Sandcore.Scene.Game.Interface;
import Sandcore.Scene.Game;

import Sandcore.Scene.Game.Render;

import Sandcore.Resources;

Interface::Interface(sf::RenderWindow& window, sf::Event& event, Game& game) : window(window), event(event), game(game), inGameMenu(window, event, game)
{
	fps.setPosition(window.getSize().x - 100.f, 50.f);
	fps.setFont(Resources::font);
	fps.setFillColor(sf::Color::Green);

	playerPosition.setFont(Resources::font);
	playerPosition.setFillColor(sf::Color::Magenta);

	mousePosition.setPosition(fps.getOrigin() + sf::Vector2f(0, 300));
	mousePosition.setFont(Resources::font);
	mousePosition.setFillColor(sf::Color::Red);
}

void Interface::resize()
{
	fps.setPosition(window.getSize().x - 100.f, 50.f);

	inGameMenu.resize();
}

void Interface::draw()
{
	update();

	if (Resources::debug)
	{
		window.draw(fps);
		window.draw(playerPosition);
		window.draw(mousePosition);
	}

	inGameMenu.draw();
}

void Interface::update()
{
	if (Resources::debug)
	{
		fpsClock.restart();
		fps.setString(std::to_string(1000000 / fpsClock.getElapsedTime()));


		mousePosition.setString
		(
			// "x: " + std::to_string(sf::Mouse::getPosition(window).x) + "\n"
			// "y: " + std::to_string(sf::Mouse::getPosition(window).y) + "\n"

			"x: " + std::to_string(game.render.renderTextureArray[0].mapPixelToCoords(sf::Mouse::getPosition(window)).x / 64) + "\n"
			"y: " + std::to_string(game.render.renderTextureArray[0].mapPixelToCoords(sf::Mouse::getPosition(window)).y / 64) + "\n"
		);

		if (game.player != nullptr)
		{
			playerPosition.setString
			(
				"x: " + std::to_string(game.player->getEntity()->getWorldPosition().x) + "\n"
				"y: " + std::to_string(game.player->getEntity()->getWorldPosition().y) + "\n"
				"z: " + std::to_string(game.player->getEntity()->getWorldPosition().z) + "\n"

				"x: " + std::to_string(game.player->getEntity()->getChunkPosition().x) + "\n"
				"y: " + std::to_string(game.player->getEntity()->getChunkPosition().y) + "\n"
				"z: " + std::to_string(game.player->getEntity()->getChunkPosition().z) + "\n"
			);
		}

		playerPosition.setPosition(0.f, window.getSize().y - playerPosition.getGlobalBounds().height);
	}
}

void Interface::events()
{
	inGameMenu.events();
}

void Interface::input()
{
	inGameMenu.input();
}