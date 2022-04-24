export module Sandcore.VectorThree;


export template <typename type>
class VectorThree
{
public:

	VectorThree(type x = 0, type y = 0, type z = 0);

	bool operator<(const VectorThree<type>& right) const;

	VectorThree<type> operator+(const VectorThree<type>& right);
	VectorThree<type> operator-(const VectorThree<type>& right);

	void operator+=(const VectorThree<type>& right);
	void operator-=(const VectorThree<type>& right);

	bool operator==(const VectorThree<type>& right) const;

public:

	type x;
	type y;
	type z;
};