#include <SFML/Graphics.hpp>

export module Sandcore.Interface.Button.Options;

export import <vector>;

import Sandcore.Interface.Button.Text;

export class OptionsButton : public sf::Drawable
{
public:

	OptionsButton(sf::RenderWindow& window, sf::Event& event);

	virtual void draw(sf::RenderTarget& target, sf::RenderStates states) const;

	void setTexture(sf::Texture& texture);

	void setString(std::string string);
	std::string getString();

	void setPosition(sf::Vector2f position);
	void setPosition(float x, float y);

	void addOption(std::string title);

	void events(sf::Vector2f mouse);
	void input(sf::Vector2f mouse);

	sf::FloatRect getGlobalBounds();

private:

	bool isSelected = false;

	int selectedOptionNumber = 0;

	sf::RenderWindow& window;
	sf::Event& event;

	sf::Text text;
	sf::Text selectedOption;
	TextButton button;

	std::vector<TextButton> optionsVector;

	void select(sf::Vector2f mouse);

	sf::RenderTexture renderTexture;
	sf::Sprite sprite;
	sf::View view;

	sf::Vector2f renderTextureMouse;

	void update();
};