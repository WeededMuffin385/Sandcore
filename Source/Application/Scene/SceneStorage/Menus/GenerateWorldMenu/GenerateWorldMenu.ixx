#include <SFML/Graphics.hpp>

export module Sandcore.Scene.Menu.GenerateWorld;

import <filesystem>;
import <string>;

import Sandcore.Interface.Button.Enter;
import Sandcore.Interface.Button.Text;
import Sandcore.Scene;
import Sandcore.Scene.Menu.SingleplayerMenu;

export class GenerateWorldMenu : public Scene
{
public:

	GenerateWorldMenu(sf::RenderWindow& window, sf::Event& event, SceneManager& sceneManager, SingleplayerMenu& singleplayerMenu);

	virtual void update();

	virtual void draw();

private:

	virtual void events();

	virtual void input();

	void addEnterButton(std::string title, std::string defaultValue);

	sf::Sprite background;

	std::vector<EnterButton> enterButtonVector;

	TextButton generateWorldButton;

	sf::RenderTexture renderTexture;
	sf::Sprite sprite;
	sf::View view;

	sf::Vector2f renderTextureMouse;
	sf::Vector2f mouse;

	SingleplayerMenu& singleplayerMenu;
};