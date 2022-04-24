#include <SFML/Graphics.hpp>

export module Sandcore.Scene.Menu.MainMenu;

export import Sandcore.Interface.Button.Text;
import Sandcore.Scene;

export class MainMenu : public Scene
{
public:

	MainMenu(sf::RenderWindow& window, sf::Event& event, SceneManager& sceneManager);

	virtual void draw();

	virtual void update();

	virtual void resize();

private:

	virtual void events();

	virtual void input();

	sf::Sprite background;

	TextButton singleplayerMenuButton;
	TextButton multiplayerMenuButton;
	TextButton settingsMenuButton;
	TextButton exitButton;

	sf::Vector2f mouse;
};