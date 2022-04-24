#include <SFML/Graphics.hpp>

export module Sandcore.Scene.Menu.SingleplayerMenu;

export import <filesystem>;

export import Sandcore.Scene.Manager;
export import Sandcore.Interface.Button.Text;
export import Sandcore.Interface.Button.World;

export class SingleplayerMenu : public Scene
{
public:

	SingleplayerMenu(sf::RenderWindow& window, sf::Event& event, SceneManager& sceneManager);

	virtual void update();
	virtual void draw();
	virtual void resize();

	void loadWorldButtons();

private:

	virtual void events();

	virtual void input();

	void addWorldButton(std::string name, std::filesystem::path path);


	void loadWorld(int i);

	std::vector<WorldButton> worldButtonVector;

	TextButton generateWorldButton;

	sf::Sprite background;

	sf::RenderTexture renderTexture;
	sf::Sprite sprite;
	sf::View view;

	sf::Vector2f renderTextureMouse;
	sf::Vector2f mouse;

	std::filesystem::path worldsPath;
};