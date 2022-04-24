import Sandcore.Clock;

long long Clock::getElapsedTime()
{
	return std::chrono::duration_cast<std::chrono::microseconds>(end - begin).count();
}

void Clock::restart()
{
	begin = end;
	end = std::chrono::steady_clock::now();
}