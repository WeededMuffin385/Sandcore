#include <SFML/Graphics.hpp>

export module Sandcore.Scene.Manager;

export import <memory>;
export import <stack>;

export import Sandcore.Scene;

export class SceneManager
{
public:

	void update();
	void draw();

	bool isEmpty();
	void clear();

	void resize();

public:

	void push(std::unique_ptr<Scene> scene);

	Scene* top();

private:

	void pop();

	std::stack<std::unique_ptr<Scene>> sceneStack;
};