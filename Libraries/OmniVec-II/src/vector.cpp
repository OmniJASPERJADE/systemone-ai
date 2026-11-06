#include "omnivec/vector.hpp"

#include <stdexcept>

namespace omnivec {

void add(
    std::span<const float> a,
    std::span<const float> b,
    std::span<float> output
) {
    if (a.size() != b.size() || a.size() != output.size()) {
        throw std::invalid_argument("Vector dimensions differ");
    }

    for (std::size_t i = 0; i < a.size(); ++i) {
        output[i] = a[i] + b[i];
    }
}

float dot(
    std::span<const float> a,
    std::span<const float> b
) {
    if (a.size() != b.size()) {
        throw std::invalid_argument("Vector dimensions differ");
    }

    float result = 0.0f;

    for (std::size_t i = 0; i < a.size(); ++i) {
        result += a[i] * b[i];
    }

    return result;
}

}
