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

	apr_pool_t *pool;
	
	if (apr_pool_create_core(&pool)) {
		fprintf(stderr, "Failed to allocate pool.\n");
		exit(1);
	}

	apr_hash_t *freq_visited = apr_hash_make(pool);

	if (freq_visited == NULL) {
		fprintf(stderr, "Failed to allocate hashtable\n");
		exit(1);
	}

	long total = 0;

	long next;
	while (fscanf(file, "%ld", &next) != EOF) {
		total += next;	
	}

	printf("Total: %ld\n", total);

	apr_pool_destroy(pool);

	fclose(file);

	return (0);
}
