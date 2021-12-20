#include <iostream>
#include <array>
#include <fstream>
#include <sstream>

class processor {
    enum class instruction_type {
        forward, down, up, invalid
    };

    struct instruction {
        instruction_type type;
        uint32_t value;
    };

public:
    explicit processor() : horizontal_position_(0), depth_(0), aim_(0) {}

    void process_line(const std::string &line) {
        instruction inst = get_instruction(line);
        switch (inst.type) {
            case instruction_type::forward:
                horizontal_position_ += inst.value;
                depth_ += aim_ * inst.value;
                break;
            case instruction_type::down:
                aim_ += inst.value;
                break;
            case instruction_type::up:
                aim_ -= inst.value;
                break;
            default:
                throw std::runtime_error("Instruction not valid");
        }
    }

    std::string get_summary() {
        std::stringstream ss;
        ss << "The submarine have a horizontal position of " << horizontal_position_ << " and a depth of "
           << depth_ << ". (Multiplying these together produces " << horizontal_position_ * depth_ << ".)";
        return ss.str();
    }

private:
    std::array<std::string, 3> instructions{"forward", "down", "up"};

    uint32_t horizontal_position_;
    uint32_t depth_;
    uint32_t aim_;

    static instruction get_instruction(const std::string &line) {
        std::string type = line.substr(0, line.find(' '));
        uint32_t value = std::stoul(line.substr(line.find(' '), line.size()));
        if (type == "forward") {
            return {instruction_type::forward, value};
        } else if (type == "down") {
            return {instruction_type::down, value};
        } else if (type == "up") {
            return {instruction_type::up, value};
        }
        return {instruction_type::invalid, 0};
    }
};

int main() {
//    std::string input_file_name = "../example_input.txt";
    std::string input_file_name = "../input.txt";

    std::ifstream infile(input_file_name);
    if (!infile.is_open()) {
        throw std::runtime_error("Cannot open the instructions file");
    }

    processor p{};

    std::string line;
    while (std::getline(infile, line)) {
        p.process_line(line);
    }

    infile.close();

    std::cout << p.get_summary() << std::endl;

    return 0;
}
