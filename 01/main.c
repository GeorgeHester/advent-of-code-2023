#include <stdio.h>
#include <stdlib.h>
#include <string.h>

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
                }

                right = line[line_index] - 0x30;
            };
        };

        calibration_value += (left * 10) + right;
    };

    printf("Calibration Value: %d\n", calibration_value);

    fclose(file_pointer);
    if (line != NULL) free(line);

    return 0;
};