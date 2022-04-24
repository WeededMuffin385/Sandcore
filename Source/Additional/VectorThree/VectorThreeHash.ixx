export module Sandcore.VectorThree.Hash;

import <functional>;
import Sandcore.VectorThree;

export template<typename type>
class VectorThreeHash
{
public:

	std::size_t operator()(const VectorThree<type>& vectorThree) const
	{
		std::size_t hash1 = std::hash<int>{}(vectorThree.x);
		std::size_t hash2 = std::hash<int>{}(vectorThree.y);
		std::size_t hash3 = std::hash<int>{}(vectorThree.z);

		return (hash1 ^ (hash2 << 1)) ^ hash3;

		// return ((std::hash<int>()(vectorThree.x) ^ (std::hash<int>()(vectorThree.y) << 1)) >> 1) ^ (std::hash<int>()(vectorThree.z) << 1);
	}
};