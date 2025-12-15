#include <fmt/core.h>
#include <fmt/color.h>
#include <optional>


std::optional<int> add_one(const std::optional<int>& opt) {
    if (opt) {
        return *opt + 1; // 解引用获取值并加一
    }
    return std::nullopt; // 如果没有值，返回空的optional
}

int main(int argc, const char** argv) {
    // cpp中也有类似rust Option的东西，叫std::optional
    std::optional<int> maybe_value{}; // 默认是空的
    std::optional<int> null_value = std::nullopt; // 也可以这样表示空
    auto another_value = std::optional<int>(42); // 也可以直接初始化一个值
    auto maked_value = std::make_optional(100); // 也可以用make_optional来创建

    // 使用has_value()检查是否有值
    if (maybe_value.has_value()) {
        fmt::print("Value is: {}\n", maybe_value.value());
    } else {
        fmt::print("No value present.\n");
    }

    // 直接赋值
    maybe_value = 55;

    // 使用value_or提供默认值
    int null_default = null_value.value_or(-1);
    fmt::print("maybe_value: {}, null_value with default: {}\n", maybe_value.value(), null_default);

    // 使用解引用操作符获取值
    int deref_value = *maybe_value;
    fmt::print("Dereferenced maybe_value: {}\n", deref_value);
    // 也可以使用value()方法
    int value_method = maybe_value.value();

    // 对空的optional调用value()会抛出异常，所以建议直接使用value_or或者先检查has_value()
    // 这感觉有点像unwrap
    try {
        int should_throw = null_value.value();
    } catch (const std::bad_optional_access& e) {
        fmt::print(fmt::fg(fmt::color::red), "Caught exception when accessing null_value: {}\n", e.what());
    } 

    fmt::print("add_one(maybe_value): {}\n", add_one(maybe_value).value_or(-1));
    fmt::print("add_one(null_value): {}\n", add_one(null_value).value_or(-1));

    maked_value.emplace(200); // 直接就地构造一个新值
    fmt::print("maked_value after emplace: {}\n", *maked_value);
    maked_value.reset(); // 清空值
    fmt::print("maked_value after reset has value? {}\n", maked_value.has_value());

    maked_value.swap(another_value); // 交换值
    fmt::print("maked_value after swap: {}, another_value after swap: {}\n", 
        maked_value.value_or(-1), another_value.value_or(-1));

    return 0;
}