#include <SFML/Graphics.hpp>

export module Sandcore.Interface.Button.Enter;

import Sandcore.Interface.Button;

export class EnterButton : public sf::Drawable
{
public:

	EnterButton(sf::RenderWindow& window, sf::Event& event);

	void draw(sf::RenderTarget& target, sf::RenderStates states) const;

	const std::string getString();

	void setString(std::string string);

	void events(sf::Vector2f mouse);

	void input(sf::Vector2f mouse);

	void setTexture(sf::Texture& texture);

	void setPosition(sf::Vector2f position);
	void setPosition(float x, float y);

	sf::FloatRect getGlobalBounds();

private:

	void enterText();

	void select(sf::Vector2f mouse);

	bool isSelected = false;

	Button button;

	sf::Text enteredText;
	sf::Text text;

	sf::Event& event;
	sf::Window& window;

	friend class GenerateWorldMenu;
};
