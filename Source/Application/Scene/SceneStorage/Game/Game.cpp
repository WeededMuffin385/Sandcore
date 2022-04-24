#include <SFML/Graphics.hpp>
#include <fstream>

import Sandcore.Scene.Game;
import Sandcore.World.Bounds;
import Sandcore.Resources;

Game::Game(sf::RenderWindow& window, sf::Event& event, SceneManager& sceneManager, std::filesystem::path path) :
	Scene(window, event, sceneManager),
	render(window, event, world),
	controls(event, player), interface(window, event, *this),
	world(path)
{
	std::ifstream file(Resources::settings);
	std::string username;

	while (!file.eof())
	{
		std::string identification;
		file >> identification;

		if (identification == "username")
		{
			file >> username;
		}
	}

	if (username != "NULL") player = world.getPlayer(username);
}

void Game::resize()
{
	render.resize();
	interface.resize();
}

void Game::update()
{
	world.loop();

	input();

	while (window.pollEvent(event))
	{
		events();
	}

	if (player != nullptr)
	{
		render.camera.chunkPosition.x = player->getEntity()->getChunkPosition().x;
		render.camera.chunkPosition.y = player->getEntity()->getChunkPosition().y;

		render.camera.worldPosition.x = player->getEntity()->getWorldPosition().x;
		render.camera.worldPosition.y = player->getEntity()->getWorldPosition().y;
	}

	bounds(render.camera.worldPosition, render.camera.chunkPosition);
}


void Game::draw()
{
	window.clear(sf::Color::Black);

	render.draw();

	interface.draw();

	window.display();
}

void Game::input()
{
	if (player == nullptr) render.input();

	if (player != nullptr) controls.input();

	interface.input();
}

void Game::events()
{
	render.events();

	controls.events();

	interface.events();
}