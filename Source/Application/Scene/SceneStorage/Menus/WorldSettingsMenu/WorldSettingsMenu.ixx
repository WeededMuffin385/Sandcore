#include <SFML/Graphics.hpp>

export module Sandcore.Scene.Menu.WorldSettingsMenu;

export import <filesystem>;

export import Sandcore.Scene.Manager;
export import Sandcore.Scene.Menu.WorldSettingsMenu.DeleteWorld;

import Sandcore.Interface.Button.Text;
import Sandcore.Scene.Menu.SingleplayerMenu;

export class WorldSettingsMenu : public Scene
{
public:

	WorldSettingsMenu(sf::RenderWindow& window, sf::Event& event, SceneManager& sceneManager, SingleplayerMenu& singleplayerMenu, std::filesystem::path path);

	virtual void update();

	virtual void draw();

private:

	virtual void events();

	virtual void input();

	std::filesystem::path path;

private:

	sf::Sprite background;

	sf::Vector2f mouse;

	TextButton deleteWorldButton;

	DeleteWorld deleteWorld;

	SingleplayerMenu& singleplayerMenu;

	friend class DeleteWorld;
};