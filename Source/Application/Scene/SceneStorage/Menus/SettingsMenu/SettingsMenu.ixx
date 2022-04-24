#include <SFML/Graphics.hpp>

export module Sandcore.Scene.Menu.SettingsMenu;

import Sandcore.Scene;
import Sandcore.Interface.Button.Options;
import Sandcore.Interface.Button.Text;

export class SettingsMenu : public Scene
{
public:

	SettingsMenu(sf::RenderWindow& window, sf::Event& event, SceneManager& sceneManager);

	virtual void update();

	virtual void draw();

	virtual void resize();

private:

	virtual void events();

	virtual void input();

	sf::Sprite background;

	TextButton apply;

	OptionsButton resolution;

	OptionsButton windowMode;

	void load();

	void loadWindowSettings();
	void loadWindowOptions();

	void save();
	void saveWindowSettings();
	void applySettings();

	sf::Vector2f mouse;
};