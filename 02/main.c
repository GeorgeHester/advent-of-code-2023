#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MAXIMUM_RED_CUBES 12
#define MAXIMUM_GREEN_CUBES 13
#define MAXIMUM_BLUE_CUBES 14

typedef struct Game
{
    unsigned int id;
    int valid;
} Game;

enum CubeColor
{
    Red,
    Green,
    Blue,
    None
};

enum CubeColor
parse_color(char* input, unsigned int* length)
{
    char reset = input[5];
    input[5] = '\0';
    if (strcmp(input, "green") == 0)
    {
        input[5] = reset;
        *length = 5;
        return Green;
    };

    input[5] = reset;
    reset = input[4];
    input[4] = '\0';
    if (strcmp(input, "blue") == 0)
    {
        input[4] = reset;
        *length = 4;
        return Blue;
    };

    input[4] = reset;
    reset = input[3];
    input[3] = '\0';
    if (strcmp(input, "red") == 0)
    {
        input[3] = reset;
        *length = 3;
        return Red;
    };

    input[3] = reset;
    return None;
};

unsigned int
parse_number(char* input, unsigned int* length)
{
    char* input_start = input;
    unsigned int output = 0;

    while (input[0] >= 0x30 && input[0] <= 0x39)
    {
        input++;
        (*length)++;
    };
    input[0] = '\0';
    (*length)++;

    for (size_t index = 0; index < strlen(input_start); index++)
    {
        output = output * 10 + (input_start[index] - 0x30);
    };

    return output;
};

void
move_input(char** input, unsigned int* length)
{
    (*input) += *length;
    *length = 0;
};

Game
parse_game(char* input)
{
    Game game;
    unsigned int length = 5;

    move_input(&input, &length);
    unsigned int game_id = parse_number(input, &length);
    move_input(&input, &length);

    game.id = game_id;
    game.valid = 1;

    unsigned int red_cubes = 0;
    unsigned int green_cubes = 0;
    unsigned int blue_cubes = 0;

    while (input[0] != '\n' && input[0] != '\0')
    {
        while (input[0] == ' ')
            input++;

        unsigned int number_of_cubes = parse_number(input, &length);
        move_input(&input, &length);

        while (input[0] == ' ')
            input++;

        enum CubeColor color_of_cubes = parse_color(input, &length);
        move_input(&input, &length);

        switch (color_of_cubes)
        {
            case Red:
                red_cubes += number_of_cubes;
                break;
            case Green:
                green_cubes += number_of_cubes;
                break;
            case Blue:
                blue_cubes += number_of_cubes;
                break;
            default:
                break;
        };

        if (input[0] == ',') input++;
        if (input[0] == ';' || input[0] == '\n' || input[0] == '\0')
        {
            if (red_cubes > MAXIMUM_RED_CUBES)
            {
                game.valid = 0;
                break;
            };
            if (green_cubes > MAXIMUM_GREEN_CUBES)
            {
                game.valid = 0;
                break;
            };
            if (blue_cubes > MAXIMUM_BLUE_CUBES)
            {
                game.valid = 0;
                break;
            };

            red_cubes = 0;
            green_cubes = 0;
            blue_cubes = 0;
        };

        while (input[0] == ' ')
            input++;
    };

    return game;
};

int
main(void)
{
    FILE* file_pointer;
    file_pointer = fopen("input.txt", "r");

    char* line = NULL;
    size_t line_length = 0;

    unsigned int valid_total = 0;

    while (getline(&line, &line_length, file_pointer) != -1)
    {
        Game game = parse_game(line);

        if (game.valid == 1)
        {
            valid_total += game.id;
        };
    };

    printf("Total: %u\n", valid_total);

    return 0;
};