#include "omnivec/vector.hpp"

#include <array>
#include <cassert>
#include <stdexcept>

int main() {
    std::array<float, 3> a{1, 2, 3};
    std::array<float, 3> b{4, 5, 6};
    std::array<float, 3> output{};

    omnivec::add(a, b, output);

    assert(output[0] == 5);
    assert(output[1] == 7);
    assert(output[2] == 9);

    assert(omnivec::dot(a, b) == 32);

    return 0;
}
