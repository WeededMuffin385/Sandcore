export module Sandcore.Clock;

export import <chrono>;

export 	class Clock
{
public:
	/**
	* @brief function to get elapsed time
	* @returns return elapsed time as microseconds
	*/
	long long getElapsedTime();

	/**
	 * @brief function to restart clock
	 */
	void restart();

private:

	std::chrono::steady_clock::time_point begin = std::chrono::steady_clock::now();
	std::chrono::steady_clock::time_point end = std::chrono::steady_clock::now();
};