export module Sandcore.Scene.Game.Render.Camera;

export import Sandcore.VectorThree;

export class Camera
{
public:

	Camera(VectorThree<int> worldPosition, VectorThree<float> chunkPosition);

	VectorThree<int> worldPosition;
	VectorThree<float> chunkPosition;
};