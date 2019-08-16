#include <stdio.h>

int
main(int argc, char **argv)
{
	if (argc != 2) {
		fprintf(stderr, "Specify exactly one file.\n");
		exit(1);
	}
	
	char *filename = argv[1];

	FILE *file = fopen(filename, "r");

	if (file == NULL) {
		fprintf(stderr, "Could not open file.\n");
		exit(1);
	}

	long total = 0;

	long next;
	while (fscanf(file, "%d", &next) != EOF) {
		total += next;	
	}

	printf("Total: %d\n", total);

	fclose(file);

	return (0);
}
