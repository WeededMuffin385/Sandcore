#include <SFML/Graphics.hpp>

export module Sandcore.Scene.Game.Render;

export import Sandcore.VectorThree;
export import Sandcore.Clock;
export import Sandcore.World;
export import Sandcore.Scene.Game.Render.Camera;
export import Sandcore.Scene.Game.TexturePackage;

export import <vector>;

export class Render
{
public:

	Render(sf::RenderWindow& window, sf::Event& event, World& world);

	void draw();
	void input();
	void events();

	void resize();

private:

	void drawChunk(VectorThree<int> worldRelativePosition);
	void drawChunk(int x, int y, int z);

	void drawEntity(VectorThree<int> worldRelativePosition);
	void drawEntity(int x, int y, int z);

	VectorThree<int> generateRelativeIndient(VectorThree<int> worldRelativePosition);

	sf::RenderWindow& window;
	sf::Event& event;
	World& world;

	TexturePackage texturePackage;
	sf::View view;

	int renderDepth = 16;

	std::vector <sf::RenderTexture> renderTextureArray = std::vector <sf::RenderTexture>(renderDepth);
	std::vector <sf::Sprite> spriteArray = std::vector <sf::Sprite>(renderDepth);

	VectorThree<int> renderDistance = { 3, 3, 1 };

	std::vector < std::vector < std::vector < std::vector<bool>>>> isRenderVector;
	std::vector < std::vector < std::vector < std::vector<bool>>>> isRenderTransparentVector;

	float viewSpeed;

	const float fastViewSpeed = 100;
	const float normalViewSpeed = 30;

	sf::Text coordinates;

	Camera camera;

	Clock clock;

	sf::Sprite background;

	friend class Game;
	friend class MultiplayerGame;
	friend class RenderChunk;
	friend class Interface;
};