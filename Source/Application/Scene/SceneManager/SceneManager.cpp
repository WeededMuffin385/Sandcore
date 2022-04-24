import Sandcore.Scene.Manager;

void SceneManager::update()
{
	sceneStack.top()->update();

	if (!top()->run) pop();
}

void SceneManager::draw()
{
	sceneStack.top()->draw();
}

void SceneManager::pop()
{
	sceneStack.pop();
}

void SceneManager::push(std::unique_ptr<Scene> scene)
{
	sceneStack.push(std::move(scene));
}

Scene* SceneManager::top()
{
	return sceneStack.top().get();
}

bool SceneManager::isEmpty()
{
	return sceneStack.empty();
}

void SceneManager::clear()
{
	while (!sceneStack.empty()) sceneStack.pop();
}

void SceneManager::resize()
{
	for (auto &i : sceneStack._Get_container())
	{
		i.get()->resize();
	}
}