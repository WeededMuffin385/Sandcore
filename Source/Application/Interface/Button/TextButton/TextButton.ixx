#include <SFML/Graphics.hpp>

export module Sandcore.Interface.Button.Text;

import Sandcore.Interface.Button;

export class TextButton : public sf::Drawable
{
public:

	TextButton(sf::RenderWindow& window, sf::Event& event);

	virtual void draw(sf::RenderTarget& target, sf::RenderStates states) const;

	void setPosition(sf::Vector2f position);
	void setPosition(float x, float y);

	void setTexture(sf::Texture& texture);
	void setColor(sf::Color normalColor, sf::Color aimedColor);

	std::string getString();
	void setString(std::string string);

	void aim(sf::Vector2f mouse);
	bool click(sf::Vector2f mouse);


	sf::FloatRect getGlobalBounds() const;

private:

	Button button;

	sf::Text text;
};