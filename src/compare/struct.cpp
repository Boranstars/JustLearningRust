#include <string_view>
#include <string>
#include <fmt/core.h>

using namespace std; // I'm tired of writing std::
struct User {
    string name;
    int age;
    string email;
};

auto create_user(string_view name, int age, string email) -> User {
    User user;
    user.name = name;
    user.age = age;
    user.email = email;
    return user;
}

int main(int argc, const char** argv) {
    string name = "Alice";
    int age = 30;
    string email = "example@example.com";
    User user = create_user(name, age, email);
    fmt::print("Name: {}, Age: {}, Email: {}\n", user.name, user.age, user.email);
    return 0;
}