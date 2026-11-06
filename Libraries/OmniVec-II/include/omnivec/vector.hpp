#pragma once

#include <span>

namespace omnivec {

void add(
    std::span<const float> a,
    std::span<const float> b,
    std::span<float> output
);

float dot(
    std::span<const float> a,
    std::span<const float> b
);

}
