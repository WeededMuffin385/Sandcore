#include <SFML/Graphics.hpp>

export module Sandcore.Interface.Button;

export class Button : public sf::Drawable
{
public:

	Button(sf::RenderWindow& window, sf::Event& event);

	virtual void draw(sf::RenderTarget& target, sf::RenderStates states) const;

	void setTexture(sf::Texture& texture);


	void setPosition(float x, float y);
	void setPosition(sf::Vector2f position);

	sf::FloatRect getGlobalBounds() const;

	void aim(sf::Vector2f mouse);

	void setColor(sf::Color normalColor, sf::Color aimedColor);

	bool click(sf::Vector2f mouse);

private:

	sf::Sprite sprite;

	sf::RenderWindow& window;

	sf::Event& event;

	sf::Color normalColor;
	sf::Color aimedColor;
};