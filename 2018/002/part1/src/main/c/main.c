#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>

int
main(int argc, char **argv)
{
	if (argc != 2) {
		fprintf(stderr, "Include exactly one argument\n");
		exit(0);
	}

	char *filepath = argv[1];

	FILE *input = fopen(filepath, "r");

	if (input == NULL) {
		fprintf(stderr, "Failed to open file.\n");
		exit(0);
	}

	int twos = 0;
	int threes = 0;

	char line[27] = {0};
	while (fscanf(input, "%s", line) != EOF) {
		int alphabet_freq[26] = {0};

		for (int i = 0; i < 26; i++) {
			int letter = (int)(line[i] - 'a');
			alphabet_freq[letter]++;
		}

		bool foundTwo = false;
		bool foundThree = false;

		for (int i = 0; i < 26; i++) {
			if (alphabet_freq[i] == 2) {
				foundTwo = true;
			}
			if (alphabet_freq[i] == 3) {
				foundThree = true;
			}
			if (foundTwo && foundThree) {
				break;
			}
		}

		if (foundTwo) {
			twos++;
		}
		if (foundThree) {
			threes++;
		}
	}

	fclose(input);

	printf("%d * %d = %d\n", twos, threes, twos * threes);

	return (0);
}
