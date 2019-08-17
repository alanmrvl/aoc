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

	long current_freq = 0;
	long current_iter = 0;
	long frequencies[1000000] = {0}; //TODO: use hashtable that doesn't need pointers for key

	apr_hash_set(freq_visited, frequencies + current_iter, sizeof(long), frequencies + current_iter);

	long next;
	while (1) {
		while (fscanf(file, "%ld", &next) != EOF) {
			current_freq += next;
			frequencies[current_iter] = current_freq;

			printf("%ld Frequency: %ld\n", current_iter, current_freq);

			long *found = apr_hash_get(freq_visited, frequencies + current_iter, sizeof(long));
			
			if (found != NULL) {
				printf("First repeated frequency: %ld\n", current_freq);
				printf("Value: %ld\n", *found);

				apr_pool_destroy(pool);
				fclose(file);

				exit(0);
			}

			apr_hash_set(freq_visited, frequencies + current_iter, sizeof(long), frequencies + current_iter);
			current_iter++;
		}

		printf("EOF\n");
		fseek(file, 0, SEEK_SET);
	}

	return (0);
}
