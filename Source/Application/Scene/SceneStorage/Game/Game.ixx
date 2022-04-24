export module Sandcore.Scene.Game;

export import <filesystem>;

export import Sandcore.Scene;

import Sandcore.Scene.Game.Controls;
import Sandcore.Scene.Game.Interface;
import Sandcore.Scene.Game.Render;

export class Game : public Scene
{
public:

	Game(sf::RenderWindow& window, sf::Event& event, SceneManager& sceneManager, std::filesystem::path path);

	virtual void update();

	virtual void draw();

	virtual void resize();

protected:

	virtual void input();

	virtual void events();

	Controls controls;
	Interface interface;
	Render render;
	World world;

	Player* player;

	friend class Interface;
	friend class InGameMenu;
};