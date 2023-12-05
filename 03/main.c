#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define WIDTH 140
#define HEIGHT 140

typedef struct Coordinate
{
    char value;
    int checked;
} Coordinate;

typedef struct CoordinateValue
{
    unsigned int value;
    int checked;
} CoordinateValue;

typedef struct CoordinateInformation
{
    unsigned int total;
    unsigned int product;
    unsigned int number_of_part_number;
} CoordinateInformation;

typedef struct TotalInformation
{
    unsigned int part_number_total;
    unsigned int gear_ratio_total;
} TotalInformation;

void
print_input(Coordinate input[HEIGHT][WIDTH])
{
    for (int i = 0; i < HEIGHT; i++)
    {
        for (int j = 0; j < WIDTH; j++)
            printf("%c", input[i][j].value);
        printf("\n");
    };
};

CoordinateValue
find_number(Coordinate (*input)[HEIGHT][WIDTH],
            unsigned int row,
            unsigned int column)
{
    CoordinateValue output;
    output.checked = 0;
    output.value = 0;

    if ((*input)[row][column].checked == 1) output.checked = 1;
    (*input)[row][column].checked = 1;

    if ((*input)[row][column].value >= 0x30 &&
        (*input)[row][column].value <= 0x39)
    {
        int column_start = column - 1;
        int column_end = column + 1;

        while (column_start >= 0)
        {
            (*input)[row][column_start].checked = 1;

            if ((*input)[row][column_start].value >= 0x30 &&
                (*input)[row][column_start].value <= 0x39)
            {
                column_start--;
                continue;
            };

            break;
        };

        while (column_end < WIDTH)
        {
            (*input)[row][column_end].checked = 1;

            if ((*input)[row][column_end].value >= 0x30 &&
                (*input)[row][column_end].value <= 0x39)
            {
                column_end++;
                continue;
            };

            break;
        };

        for (int index = column_start + 1; index < column_end; index++)
        {
            output.value =
              (output.value * 10) + ((*input)[row][index].value - 0x30);
        };
    };

    return output;
};

CoordinateInformation
find_around(Coordinate (*input)[HEIGHT][WIDTH],
            unsigned int row,
            unsigned int column)
{
    CoordinateInformation output;
    output.total = 0;
    output.product = 0;
    output.number_of_part_number = 0;

    int check_up = 1;
    int check_right_up = 1;
    int check_right = 1;
    int check_right_down = 1;
    int check_down = 1;
    int check_left_down = 1;
    int check_left = 1;
    int check_left_up = 1;

    if (column == 0)
    {
        check_left_up = 0;
        check_left = 0;
        check_left_down = 0;
    };

    if (row == 0)
    {
        check_left_up = 0;
        check_up = 0;
        check_right_up = 0;
    };

    if (column + 1 == WIDTH)
    {
        check_right_up = 0;
        check_right = 0;
        check_right_down = 0;
    };

    if (row + 1 == HEIGHT)
    {
        check_left_down = 0;
        check_down = 0;
        check_right_down = 0;
    };

    if (check_up == 1)
    {
        CoordinateValue coordinate = find_number(input, row - 1, column);
        if (coordinate.checked == 0) output.total += coordinate.value;
        if (coordinate.value > 0 && coordinate.checked == 0)
        {
            output.number_of_part_number++;
            if (output.product != 0) output.product *= coordinate.value;
            if (output.product == 0) output.product = coordinate.value;
        };
    };

    if (check_right_up == 1)
    {
        CoordinateValue coordinate = find_number(input, row - 1, column + 1);
        if (coordinate.checked == 0) output.total += coordinate.value;
        if (coordinate.value > 0 && coordinate.checked == 0)
        {
            output.number_of_part_number++;
            if (output.product != 0) output.product *= coordinate.value;
            if (output.product == 0) output.product = coordinate.value;
        };
    };

    if (check_right == 1)
    {
        CoordinateValue coordinate = find_number(input, row, column + 1);
        if (coordinate.checked == 0) output.total += coordinate.value;
        if (coordinate.value > 0 && coordinate.checked == 0)
        {
            output.number_of_part_number++;
            if (output.product != 0) output.product *= coordinate.value;
            if (output.product == 0) output.product = coordinate.value;
        };
    };

    if (check_right_down == 1)
    {
        CoordinateValue coordinate = find_number(input, row + 1, column + 1);
        if (coordinate.checked == 0) output.total += coordinate.value;
        if (coordinate.value > 0 && coordinate.checked == 0)
        {
            output.number_of_part_number++;
            if (output.product != 0) output.product *= coordinate.value;
            if (output.product == 0) output.product = coordinate.value;
        };
    };

    if (check_down == 1)
    {
        CoordinateValue coordinate = find_number(input, row + 1, column);
        if (coordinate.checked == 0) output.total += coordinate.value;
        if (coordinate.value > 0 && coordinate.checked == 0)
        {
            output.number_of_part_number++;
            if (output.product != 0) output.product *= coordinate.value;
            if (output.product == 0) output.product = coordinate.value;
        };
    };

    if (check_left_down == 1)
    {
        CoordinateValue coordinate = find_number(input, row + 1, column - 1);
        if (coordinate.checked == 0) output.total += coordinate.value;
        if (coordinate.value > 0 && coordinate.checked == 0)
        {
            output.number_of_part_number++;
            if (output.product != 0) output.product *= coordinate.value;
            if (output.product == 0) output.product = coordinate.value;
        };
    };

    if (check_left == 1)
    {
        CoordinateValue coordinate = find_number(input, row, column - 1);
        if (coordinate.checked == 0) output.total += coordinate.value;
        if (coordinate.value > 0 && coordinate.checked == 0)
        {
            output.number_of_part_number++;
            if (output.product != 0) output.product *= coordinate.value;
            if (output.product == 0) output.product = coordinate.value;
        };
    };

    if (check_left_up == 1)
    {
        CoordinateValue coordinate = find_number(input, row - 1, column - 1);
        if (coordinate.checked == 0) output.total += coordinate.value;
        if (coordinate.value > 0 && coordinate.checked == 0)
        {
            output.number_of_part_number++;
            if (output.product != 0) output.product *= coordinate.value;
            if (output.product == 0) output.product = coordinate.value;
        };
    };

    return output;
};

TotalInformation
parse_input(Coordinate (*input)[HEIGHT][WIDTH])
{
    TotalInformation output;
    output.part_number_total = 0;
    output.gear_ratio_total = 0;

    for (unsigned int row = 0; row < HEIGHT; row++)
    {
        for (unsigned int column = 0; column < HEIGHT; column++)
        {
            if ((*input)[row][column].value == '.') continue;
            if ((*input)[row][column].value >= 0x30 &&
                (*input)[row][column].value <= 0x39)
                continue;

            CoordinateInformation information = find_around(input, row, column);
            output.part_number_total += information.total;
            if (information.number_of_part_number == 2)
                output.gear_ratio_total += information.product;
        };
    };

    return output;
};

int
main(void)
{
    FILE* file_pointer;
    file_pointer = fopen("input.txt", "r");

    if (file_pointer == NULL)
    {
        printf("Error: Failed to open input file");
        return 1;
    };

    Coordinate input[HEIGHT][WIDTH];

    for (unsigned int row = 0; row < HEIGHT; row++)
    {
        char* line = NULL;
        size_t line_length = 0;

        if (getline(&line, &line_length, file_pointer) == -1)
        {
            printf("Error: Failed to read input file line");
            return 1;
        };

        for (unsigned int column = 0; column < HEIGHT; column++)
        {
            input[row][column].value = line[column];
            input[row][column].checked = 0;
        };
    };

    TotalInformation information = parse_input(&input);
    printf("Part Total: %u\n", information.part_number_total);
    printf("Gear Total: %u\n", information.gear_ratio_total);

    return 0;
};