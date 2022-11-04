#include <iostream>
#include <fstream>
#include <vector>

std::vector<uint32_t> read_measurements_from_file(const std::string &measurements_file) {
    std::ifstream infile(measurements_file);
    if (!infile.is_open()) {
        throw std::runtime_error("Cannot open the measurements file");
    }

    std::vector<uint32_t> measurements = {};
    for (int number; infile >> number;) {
        measurements.emplace_back(number);
    }
    infile.close();

    return measurements;
}

void calculate_larger_measurements(const std::vector<uint32_t> &measurements) {
    uint32_t count{0};
    uint32_t last_measurement{0};

    for (auto measurement: measurements) {
        if (measurement > last_measurement) {
            count++;
        }
        last_measurement = measurement;
    }

    count--; // Subtract one since the first comparison will be always true.

    std::cout << "There are " << count << " measurements greater than previous measurements" << std::endl;
}

void calculate_larger_measurements_by_windows(const std::vector<uint32_t> &measurements) {
    uint32_t count{0};
    uint32_t last_measurement{0};
    uint32_t m = 0;
    for (size_t i = 0; i < measurements.size(); i++) {
        if (i + 3 > measurements.size()) {
            break;
        }

        // Group the measurements in 3 sized windows
        for (size_t j = i; j < (i + 3); j++) {
            m += measurements[j];
        }

        if (m > last_measurement) {
            count++;
        }
        last_measurement = m;

        m = 0;
    }

    count--; // Subtract one since the first comparison will be always true.

    std::cout << "There are " << count << " measurements sums greater than previous measurements sums" << std::endl;
}

int main() {
//    std::string input_file_name = "../example_input.txt";
    std::string input_file_name = "../input.txt";

    auto measurements = read_measurements_from_file(input_file_name);

    std::cout << "--- Part One ---" << std::endl;
    calculate_larger_measurements(measurements);

    std::cout << "--- Part Two ---" << std::endl;
    calculate_larger_measurements_by_windows(measurements);

    return 0;
}
