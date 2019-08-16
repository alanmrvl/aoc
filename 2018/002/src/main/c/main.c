#include <stdio.h>
#include <stdlib.h>
#include <apr.h>
#include <apr_hash.h>

int
main(int argc, char **argv)
{
	apr_initialize();

	atexit(apr_terminate);

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
	while (fscanf(file, "%ld", &next) != EOF) {
		total += next;	
	}

	printf("Total: %ld\n", total);

	fclose(file);

	return (0);
}
