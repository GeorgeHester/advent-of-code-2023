#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int
word(char* input)
{
    char check[6] = "\0";
    strncpy(check, input, 6);

    check[5] = '\0';

    if (strcmp(check, "eight") == 0) return 8;
    if (strcmp(check, "seven") == 0) return 7;
    if (strcmp(check, "three") == 0) return 3;

    check[4] = '\0';

    if (strcmp(check, "nine") == 0) return 9;
    if (strcmp(check, "five") == 0) return 5;
    if (strcmp(check, "four") == 0) return 4;

    check[3] = '\0';

    if (strcmp(check, "six") == 0) return 6;
    if (strcmp(check, "two") == 0) return 2;
    if (strcmp(check, "one") == 0) return 1;

    return 0;
};

int
main(void)
{
    FILE* file_pointer;
    file_pointer = fopen("input", "r");

    if (file_pointer == NULL)
    {
        printf("Error: Failed to open input file");
        return 1;
    };

    char* line = NULL;
    size_t line_length = 0;

    int calibration_value = 0;

    while (getline(&line, &line_length, file_pointer) != -1)
    {
        char left = 0;
        char right = 0;

        for (size_t line_index = 0; line_index < strlen(line); line_index++)
        {
            if (line[line_index] >= 0x31 && line[line_index] <= 0x39)
            {
                if (left == 0)
                {
                    left = line[line_index] - 0x30;
                    right = line[line_index] - 0x30;
                    continue;
                }

                right = line[line_index] - 0x30;
                continue;
            };

            int word_check = word(line + line_index);

            if (word_check != 0)
            {
                if (left == 0)
                {
                    left = word_check;
                    right = word_check;
                    continue;
                }

                right = word_check;
                continue;
            };
        };

        calibration_value += (left * 10) + right;
    };

    printf("Calibration Value: %d\n", calibration_value);

    fclose(file_pointer);
    if (line != NULL) free(line);

    return 0;
};