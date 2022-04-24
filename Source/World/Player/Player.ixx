class Entity;

export module Sandcore.World.Player;

export class Player
{
public:

	Player(Entity* entity = nullptr);

	bool isWorldPositionChanged();

	Entity* getEntity();

private:

	Entity* entity;

	bool worldPositionChanged;
};