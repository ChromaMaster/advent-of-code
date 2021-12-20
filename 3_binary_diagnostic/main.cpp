#include <iostream>
#include <fstream>
#include <sstream>
#include <array>
#include <algorithm>
#include <valarray>

class processor {
public:
    explicit processor() : gamma_rate_(0), epsilon_rate_(0), bit_array_({0}) {};

    void process_line(std::string line) {
        uint8_t index = 0;
        for (auto c: line) {
            if (c == '0') {
                bit_array_[index]--;
            } else if (c == '1') {
                bit_array_[index]++;
            }
            index++;
        }
    }

    char get_most_common_bit(const std::vector<std::string> &values, uint8_t bit_to_check) {
        int8_t count = 0;
        for (auto value: values) {
            if (value[bit_to_check] == '0') {
                count--;
            } else {
                count++;
            }
        }
        return (count >= 0) ? '1' : '0';
    }

    void calculate_gamma_and_epsilon_rates() {
        std::stringstream gamma_bit_array;
        std::stringstream epsilon_bit_array;
        std::for_each(bit_array_.begin(), bit_array_.end(), [&gamma_bit_array, &epsilon_bit_array](int &bit) {
            if (bit >= 0) {
                bit = 1;
            } else {
                bit = 0;
            }
            gamma_bit_array << bit;
            epsilon_bit_array << !bit;
        });

        gamma_rate_binary_string_ = gamma_bit_array.str();
        epsilon_rate_binary_string_ = epsilon_bit_array.str();

        gamma_rate_ = bin_to_int(gamma_rate_binary_string_);
        epsilon_rate_ = bin_to_int(epsilon_rate_binary_string_);

        std::cout << "Gamma rate: " << gamma_rate_binary_string_ << " -> " << gamma_rate_ << std::endl;
        std::cout << "Epsilon rate: " << epsilon_rate_binary_string_ << " -> " << epsilon_rate_ << std::endl;
    }

    void calculate_oxygen_and_co2_ratings(const std::vector<std::string> &measurements) {
        oxygen_generator_rating_binary_string_ = filter_by_bit_criteria(measurements, 0, false);
        co2_scrubber_rating_binary_string_ = filter_by_bit_criteria(measurements, 0, true);

        oxygen_generator_rating_ = bin_to_int(oxygen_generator_rating_binary_string_);
        co2_scrubber_rating_ = bin_to_int(co2_scrubber_rating_binary_string_);

        std::cout << "Oxygen generator rating: " << oxygen_generator_rating_binary_string_ << " -> "
                  << oxygen_generator_rating_ << std::endl;
        std::cout << "CO2 scrubber rating: " << co2_scrubber_rating_binary_string_ << " -> " << co2_scrubber_rating_
                  << std::endl;
    }

    std::string
    filter_by_bit_criteria(const std::vector<std::string> &measurements, uint8_t bit_to_check, bool least_common) {
        if (measurements.size() == 1) {
            return measurements[0];
        }

        char value = get_most_common_bit(measurements, bit_to_check);
        if (least_common) {
            value = (value == '0') ? '1' : '0';
        }

        std::vector<std::string> bar;
        std::copy_if(measurements.begin(), measurements.end(), std::back_inserter(bar),
                     [bit_to_check, value](std::string measurement) {
                         return measurement[bit_to_check] == value;
                     });

        return filter_by_bit_criteria(bar, ++bit_to_check, least_common);
    }

    std::string get_summary() {
        std::stringstream ss;
        ss << "Gamma rate is " << gamma_rate_ << " and Epsilon rate is " << epsilon_rate_
           << " so the power consumption is " << gamma_rate_ * epsilon_rate_;

        ss << std::endl;

        ss << "Oxygen generator rating is " << oxygen_generator_rating_ << " and CO2 scrubber rating is "
           << co2_scrubber_rating_
           << " so the life support is " << oxygen_generator_rating_ * co2_scrubber_rating_;
        return ss.str();
    }
private:
    std::string gamma_rate_binary_string_;
    std::string epsilon_rate_binary_string_;
    std::string oxygen_generator_rating_binary_string_;
    std::string co2_scrubber_rating_binary_string_;

    uint32_t gamma_rate_;
    uint32_t epsilon_rate_;
    uint32_t oxygen_generator_rating_;
    uint32_t co2_scrubber_rating_;

    std::array<int, 12> bit_array_;

    uint32_t bin_to_int(std::string bin_data) {
        auto len = bin_data.size() - 1;
        uint32_t value = 0;
        for (auto c: bin_data) {
            if (c == '1') {
                value += static_cast<uint32_t>(std::pow(2, len));
            }
            len--;
        }
        return value;
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
    std::vector<std::string> lines;
    while (std::getline(infile, line)) {
        lines.push_back(line);
    }

    infile.close();

    for (auto line: lines) {
        p.process_line(line);
    }

    std::cout << "--- Part One ---" << std::endl;
    p.calculate_gamma_and_epsilon_rates();

    std::cout << "--- Part Two ---" << std::endl;
    p.calculate_oxygen_and_co2_ratings(lines);

    std::cout << std::endl << p.get_summary() << std::endl;

    return 0;
}
