export module Sandcore.World;

export import <filesystem>;
export import <string>;
export import <set>;
export import <unordered_map>;
export import <map>;

export import Sandcore.Clock;
export import Sandcore.VectorThree;
export import Sandcore.VectorThree.Hash;

export import Sandcore.World.Chunk;
export import Sandcore.World.Entity;
export import Sandcore.World.Player;

export import Sandcore.World.Parameters;
export import Sandcore.World.Generator;


export class World
{
public:

	World(std::filesystem::path path);
	~World();

	void loop();

	long long getElapsedTime();

	Chunk& getChunk(int x, int y, int z);
	Chunk& getChunk(VectorThree<int> position);

	Player* getPlayer(std::string username);

	std::set<Entity*>& getEntitySet();


	static void createDirectory(std::filesystem::path path, Parameters parameters);
	static void deleteDirectory(std::filesystem::path path);

private:

	void loadPlayer(std::string identification);
	void savePlayer(std::string identification);

	void loadChunk(VectorThree<int> position);
	void saveChunk(VectorThree<int> position);

	std::filesystem::path path;

	std::unordered_map<VectorThree<int>, Chunk, VectorThreeHash<int>> chunkMap;

	std::map<std::string, Player> playerMap;
	std::set<Entity*> entitySet;

	Parameters parameters;
	Generator generator;

	void readPlayer(std::string identification);
	void generatePlayer(std::string identification);

	void readChunk(VectorThree<int> position);
	void generateChunk(VectorThree<int> position);

	void loadChunkInRadius(VectorThree<int> position, int radius);

	std::filesystem::path playerPath(std::string identification);
	std::filesystem::path chunkPath(VectorThree<int> position);

	Clock clock;

	bool debug = false;

	friend class Engine;
};