#include <iostream>
#include <tuple>
#include <fstream>
#include <vector>

struct DoubleList {
    DoubleList* next;
    DoubleList* prev;
    int start;
    int length;
    int id;
};

DoubleList* find_free_block(DoubleList* free_blocks, int size, int limit) {
    while (free_blocks != nullptr && free_blocks->start < limit) {
        if (free_blocks->id == -1 && free_blocks->length >= size) {
            return free_blocks;
        }
        free_blocks = free_blocks->next;
    }
    return nullptr;
}

void insert_into_free(DoubleList* available_free, DoubleList* current_data) {
    available_free->id = current_data->id;
    current_data->id = -1;
    
    if (available_free->length == current_data->length) {
        return;
    }

    DoubleList* tmp = new DoubleList{available_free->next, available_free, available_free->start + current_data->length, available_free->length - current_data->length, -1};
    available_free->length = current_data->length;
    available_free->next = tmp;
}

std::vector<int> create_diskspace(DoubleList* data_blocks) {
    std::vector<int> diskspace;

    while (data_blocks != nullptr) {
        for (int i = 0; i < data_blocks->length; i++) {
            diskspace.push_back(data_blocks->id == -1 ? 0 : data_blocks->id);
        }
        data_blocks = data_blocks->next;
    }
    return diskspace;
}

int main() {   
    std::ifstream file("input/day9.txt");

    std::string bytes;
    file >> bytes;

    std::cout << bytes << std::endl;

    DoubleList* data_blocks = new DoubleList{nullptr, nullptr, 0, 0, -2};
    DoubleList* current = data_blocks;

    int id = 0;
    int disk_length = 0;
    bool free_flag = false;
    for (char byte : bytes) {
        if (free_flag) {
            current->next = new DoubleList{nullptr, current, disk_length, byte - '0', -1};        
        }
        else {
            current->next = new DoubleList{nullptr, current, disk_length, byte - '0', id};
            id++;
        }
        current = current->next;
        disk_length += byte - '0';
        free_flag = !free_flag;
    }
    data_blocks = data_blocks->next;
    DoubleList* free_blocks_ptr = data_blocks;
    free_blocks_ptr->prev = nullptr;

    DoubleList* data_blocks_ptr = current;

    while (free_blocks_ptr != nullptr && data_blocks_ptr != nullptr && free_blocks_ptr->start < data_blocks_ptr->start) {
        DoubleList* available_free = find_free_block(free_blocks_ptr, data_blocks_ptr->length, data_blocks_ptr->start);

        if (available_free != nullptr) {
            insert_into_free(available_free, data_blocks_ptr);
        }

        do {
            data_blocks_ptr = data_blocks_ptr->prev;
        } while (data_blocks_ptr != nullptr && data_blocks_ptr->id == -1);

        while (free_blocks_ptr != nullptr && free_blocks_ptr->id != -1) {
            free_blocks_ptr = free_blocks_ptr->next;
        }
    }

    std::vector<int> diskspace = create_diskspace(data_blocks);
    long long checksum = 0;
    for (long long i = 0; i < diskspace.size(); i++) {
        checksum += i * diskspace[i];
    }
    std::cout << checksum;  
}   