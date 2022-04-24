#include <SFML/Graphics.hpp>

class SceneManager;

export module Sandcore.Scene;

export class Scene
{
public:

	Scene(sf::RenderWindow& window, sf::Event& event, SceneManager& sceneManager);

	virtual ~Scene() {};

	virtual void update() = 0;

	virtual void draw() = 0;

	virtual void resize() {};

	bool run = true;

protected:

	virtual void events() = 0;

	virtual void input() = 0;

	sf::RenderWindow& window;
	sf::Event& event;
	SceneManager& sceneManager;
};
