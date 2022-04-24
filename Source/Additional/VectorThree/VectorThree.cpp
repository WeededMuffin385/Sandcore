import Sandcore.VectorThree;

template class VectorThree<int>;
template class VectorThree<float>;

template<typename type>
VectorThree<type>::VectorThree(type x, type y, type z) : x(x), y(y), z(z)
{

}

template<typename type>
bool VectorThree<type>::operator<(const VectorThree<type>& right) const
{
	if (this->x < right.x) return true; else if (this->x > right.x) return false;
	if (this->y < right.y) return true; else if (this->y > right.y) return false;
	if (this->z < right.z) return true; else if (this->z > right.z) return false;
	return false;
}

template<typename type>
VectorThree<type> VectorThree<type>::operator+(const VectorThree<type>& right)
{
	VectorThree<type> buffer = *this;

	buffer += right;

	return buffer;
}

template<typename type>
VectorThree<type> VectorThree<type>::operator-(const VectorThree<type>& right)
{
	VectorThree<type> buffer = *this;

	buffer -= right;

	return buffer;
}

template<typename type>
void VectorThree<type>::operator+=(const VectorThree<type>& right)
{
	this->x += right.x;
	this->y += right.y;
	this->z += right.z;
}

template<typename type>
void VectorThree<type>::operator-=(const VectorThree<type>& right)
{
	this->x -= right.x;
	this->y -= right.y;
	this->z -= right.z;
}

template<typename type>
bool VectorThree<type>::operator==(const VectorThree<type>& right) const
{
	return x == right.x
		&& y == right.y
		&& z == right.z;
}