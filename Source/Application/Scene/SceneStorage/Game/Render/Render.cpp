#include <SFML/Graphics.hpp>

#include <iostream>

import Sandcore.Scene.Game.Render;
import Sandcore.World.Bounds;
import Sandcore.Resources;

// sf::Uint8 color = (255 / renderDepth) * (renderDepth - camera.chunkPosition.z + z + worldRelativePosition.z * Chunk::size::z);

Render::Render(sf::RenderWindow& window, sf::Event& event, World& world) :
	window(window), event(event), world(world),
	camera({ 0, 0, 0 }, { 0, 0, 0 }),
	texturePackage(Resources::texturePackage),
	view(sf::FloatRect(0.f, 0.f, window.getSize().x, window.getSize().y)),
	background(texturePackage.background)
{
	for (int i = 0; i < renderDepth; ++i)
	{
		renderTextureArray[i].create(window.getSize().x, window.getSize().y);
		spriteArray[i].setTexture(renderTextureArray[i].getTexture());
	}

	view.zoom(2);

	coordinates.setFont(Resources::font);
	coordinates.setFillColor(sf::Color::Green);
}

void Render::resize()
{
	view = sf::View(sf::FloatRect(0.f, 0.f, window.getSize().x, window.getSize().y));

	for (int i = 0; i < renderDepth; ++i)
	{
		renderTextureArray[i].create(window.getSize().x, window.getSize().y);
		spriteArray[i].setTexture(renderTextureArray[i].getTexture(), true);
	}
}

void Render::draw()
{
	view.setCenter
	(
		camera.chunkPosition.x * texturePackage.size.x + Chunk::size::x * texturePackage.size.x * renderDistance.x, 
		camera.chunkPosition.y * texturePackage.size.y + Chunk::size::y * texturePackage.size.y * renderDistance.y
	);

	for (int i = 0; i < renderDepth; ++i)
	{
		renderTextureArray[i].setView(view);
		renderTextureArray[i].clear(sf::Color::Transparent);
	}

	if (Resources::debug)
	{
		coordinates.setString
		(
			"x: " + std::to_string(camera.worldPosition.x) + "\n"
			"y: " + std::to_string(camera.worldPosition.y) + "\n"
			"z: " + std::to_string(camera.worldPosition.z) + "\n"

			"x: " + std::to_string(camera.chunkPosition.x) + "\n"
			"y: " + std::to_string(camera.chunkPosition.y) + "\n"
			"z: " + std::to_string(camera.chunkPosition.z) + "\n"
		);
	}

	isRenderVector = std::vector < std::vector < std::vector<std::vector<bool>>>>(renderDistance.x * 2 + 1, std::vector < std::vector<std::vector<bool>>>(renderDistance.y * 2 + 1, std::vector<std::vector<bool>>(16, std::vector<bool>(16, false))));
	isRenderTransparentVector = std::vector < std::vector < std::vector<std::vector<bool>>>>(renderDistance.x * 2 + 1, std::vector < std::vector<std::vector<bool>>>(renderDistance.y * 2 + 1, std::vector<std::vector<bool>>(16, std::vector<bool>(16, false))));

	for (int x = -renderDistance.x; x <= renderDistance.x; ++x)
	{
		for (int y = -renderDistance.y; y <= renderDistance.y; ++y)
		{
			for (int z = 0; z >= -renderDistance.z; --z)
			{
				drawChunk(VectorThree<int>(x, y, z));
			}
		}
	}

	for (int x = -renderDistance.x; x <= renderDistance.x; ++x)
	{
		for (int y = -renderDistance.y; y <= renderDistance.y; ++y)
		{
			for (int z = -renderDistance.z; z <= 0; ++z)
			{
				drawEntity(VectorThree<int>(x, y, z));
			}
		}
	}

	// window.draw(background);

	for (int i = 0; i < renderDepth; ++i)
	{
		sf::CircleShape a(10, 10);
		renderTextureArray[i].draw(a);
		sf::Uint8 color = (255 / renderDepth) * i;
		renderTextureArray[i].display();
		spriteArray[i].setColor(sf::Color(color, color, color));
		window.draw(spriteArray[i]);
	}

	if (Resources::debug)
	{
		window.draw(coordinates);
	}
}

void Render::drawChunk(VectorThree<int> worldRelativePosition)
{
	Chunk& chunk = world.getChunk(camera.worldPosition + worldRelativePosition);

	VectorThree<int> relativeIndient = generateRelativeIndient(worldRelativePosition);

	for (int x = 0; x < Chunk::size::x; ++x)
	{
		for (int y = 0; y < Chunk::size::y; ++y)
		{
			for (int z = Chunk::size::z - 1; z != -1; --z)
			{
				if (!isRenderVector[worldRelativePosition.x + renderDistance.x][worldRelativePosition.y + renderDistance.y][x][y])
				{
					if (camera.chunkPosition.z >= z || worldRelativePosition.z < 0)
					{
						BlockIdentification& id = chunk.getBlock(x, y, z).getId();

						if (id == BlockIdentification::water) continue;
						if (id == BlockIdentification::vacuum) continue;
						if (renderDepth - int(camera.chunkPosition.z) + z + worldRelativePosition.z * Chunk::size::z - 1 >= renderDepth) continue;
						if (renderDepth - int(camera.chunkPosition.z) + z + worldRelativePosition.z * Chunk::size::z - 1 < 0) continue;

						texturePackage.blockSpriteMap[id].setPosition(texturePackage.size.x * x + relativeIndient.x, texturePackage.size.y * y + relativeIndient.y);

						renderTextureArray[renderDepth - int(camera.chunkPosition.z) + z + worldRelativePosition.z * Chunk::size::z - 1].draw(texturePackage.blockSpriteMap[id]);
						
						isRenderVector[worldRelativePosition.x + renderDistance.x][worldRelativePosition.y + renderDistance.y][x][y] = true;
					}
				}
			}
		}
	}

	for (int x = 0; x < Chunk::size::x; ++x)
	{
		for (int y = 0; y < Chunk::size::y; ++y)
		{
			for (int z = Chunk::size::z - 1; z != -1; --z)
			{
				if (!isRenderTransparentVector[worldRelativePosition.x + renderDistance.x][worldRelativePosition.y + renderDistance.y][x][y])
				{
					if (camera.chunkPosition.z >= z || worldRelativePosition.z < 0)
					{
						BlockIdentification& id = chunk.getBlock(x, y, z).getId();

						if (id == BlockIdentification::vacuum) continue;
						if (renderDepth - int(camera.chunkPosition.z) + z + worldRelativePosition.z * Chunk::size::z - 1 >= renderDepth) continue;
						if (renderDepth - int(camera.chunkPosition.z) + z + worldRelativePosition.z * Chunk::size::z - 1 < 0) continue;


						if (id == BlockIdentification::water)
						{

							texturePackage.blockSpriteMap[id].setPosition(texturePackage.size.x * x + relativeIndient.x, texturePackage.size.y * y + relativeIndient.y);

							renderTextureArray[renderDepth - int(camera.chunkPosition.z) + z + worldRelativePosition.z * Chunk::size::z - 1].draw(texturePackage.blockSpriteMap[id]);
						}

						isRenderTransparentVector[worldRelativePosition.x + renderDistance.x][worldRelativePosition.y + renderDistance.y][x][y] = true;
					}
				}
			}
		}
	}
}

void Render::drawChunk(int x, int y, int z)
{
	drawChunk(VectorThree<int>(x, y, z));
}

void Render::drawEntity(VectorThree<int> worldRelativePosition)
{
	VectorThree<int> relativeIndient = generateRelativeIndient(worldRelativePosition);

	for (auto entity : world.getEntitySet())
	{
		if (entity->getWorldPosition() == (worldRelativePosition + camera.worldPosition))
		{
			if (static_cast<int>(entity->getChunkPosition().z) <= static_cast<int>(camera.chunkPosition.z) || worldRelativePosition.z < 0)
			{
				const EntityIdentification id = entity->getId();
				int z = renderDepth - int(camera.chunkPosition.z) + int(entity->getChunkPosition().z) + worldRelativePosition.z * Chunk::size::z - 1;
				if (z >= renderDepth) continue;
				if (z < 0) continue;

				texturePackage.entitySpriteMap[id].setPosition(texturePackage.size.x * entity->getChunkPosition().x + relativeIndient.x, texturePackage.size.y * entity->getChunkPosition().y + relativeIndient.y);
				
				renderTextureArray[z].draw(texturePackage.entitySpriteMap[id]);

				if (Resources::debug)
				{
					static sf::RectangleShape hitbox;

					hitbox.setPosition(texturePackage.size.x * entity->getChunkPosition().x + relativeIndient.x, texturePackage.size.y * entity->getChunkPosition().y + relativeIndient.y);

					hitbox.setFillColor(sf::Color::Transparent);
					hitbox.setOutlineColor(sf::Color::Red);
					hitbox.setOutlineThickness(-8);
					VectorThree<float> size = entity->getSize();
					hitbox.setSize(sf::Vector2f(size.x * texturePackage.size.x, size.y * texturePackage.size.y));
					hitbox.setOrigin(hitbox.getGlobalBounds().width / 2, hitbox.getGlobalBounds().height / 2);

					renderTextureArray[z].draw(hitbox);
				}
			}
		}
	}
}

void Render::drawEntity(int x, int y, int z)
{
	drawEntity(VectorThree<int>(x, y, z));
}

void Render::input()
{
	clock.restart();

	if (sf::Keyboard::isKeyPressed(sf::Keyboard::LShift)) viewSpeed = fastViewSpeed; else viewSpeed = normalViewSpeed;

	if (sf::Keyboard::isKeyPressed(sf::Keyboard::W)) camera.chunkPosition.y -= viewSpeed * clock.getElapsedTime() / 1000000.f;
	if (sf::Keyboard::isKeyPressed(sf::Keyboard::S)) camera.chunkPosition.y += viewSpeed * clock.getElapsedTime() / 1000000.f;

	if (sf::Keyboard::isKeyPressed(sf::Keyboard::A)) camera.chunkPosition.x -= viewSpeed * clock.getElapsedTime() / 1000000.f;
	if (sf::Keyboard::isKeyPressed(sf::Keyboard::D)) camera.chunkPosition.x += viewSpeed * clock.getElapsedTime() / 1000000.f;
}

void Render::events()
{
	if (!sf::Keyboard::isKeyPressed(sf::Keyboard::LControl))
	{
		if (event.type == sf::Event::MouseWheelScrolled)
		{
			if (event.mouseWheelScroll.wheel == sf::Mouse::VerticalWheel)
			{
				if (event.mouseWheelScroll.delta == 1)
				{
					camera.chunkPosition.z += 1;
				}

				if (event.mouseWheelScroll.delta == -1)
				{
					camera.chunkPosition.z -= 1;
				}
			}
		}
	}

	if (sf::Keyboard::isKeyPressed(sf::Keyboard::LControl))
	{
		if (event.type == sf::Event::MouseWheelScrolled)
		{
			if (event.mouseWheelScroll.wheel == sf::Mouse::VerticalWheel)
			{
				if (event.mouseWheelScroll.delta == 1)
				{
					view.zoom(95.f / 100.f);
				}

				if (event.mouseWheelScroll.delta == -1)
				{
					view.zoom(100.f / 95.f);
				}
			}
		}
	}
}

VectorThree<int> Render::generateRelativeIndient(VectorThree<int> worldRelativePosition)
{
	return VectorThree<int>
	(
		worldRelativePosition.x * Chunk::size::x * texturePackage.size.x + Chunk::size::x * texturePackage.size.x * renderDistance.x,
		worldRelativePosition.y * Chunk::size::y * texturePackage.size.y + Chunk::size::y * texturePackage.size.y * renderDistance.y
	);
}