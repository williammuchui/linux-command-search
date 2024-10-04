#include <stdio.h>
#include <string.h>

#define DEFAULT_LINUX_PATH "linux"
#define MAX_LINE_LENGTH 256

void handle_default_case(void);
void read_from_linux_file(void);
void handle_commands(int argc, char **argv);
void search_commands(const char *query);
int strcasestr(const char *haystack, const char *needle);

int main(int argc, char **argv) {
  if (argc == 1) {
    handle_default_case();
  } else if (argc == 2) {
    handle_commands(argc, argv);
  } else if (argc == 3 && strcmp(argv[1], "-s") == 0) {
    search_commands(argv[2]);
  } else {
    fprintf(stderr, "Usage: %s [-l | search <query>]\n", argv[0]);
    return 1;
  }
  return 0;
}

void handle_default_case() { read_from_linux_file(); }

void read_from_linux_file(void) {
  FILE *linux_file = fopen(DEFAULT_LINUX_PATH, "r");
  if (!linux_file) {
    perror("Error opening linux file");
    return;
  }

  char line[MAX_LINE_LENGTH];
  while (fgets(line, sizeof(line), linux_file)) {
    /*format the output*/
    char *command = strtok(line, " ");
    char *description = strtok(NULL, "\n");

    printf("%s\t%s\n", command, description);
  }

  if (ferror(linux_file)) {
    perror("Error reading linux file contents");
  }

  fclose(linux_file);
}

void handle_commands(int argc, char **argv) {
  if (strcmp(argv[1], "-l") == 0) {
    handle_default_case();
  } else {
    fprintf(stderr, "Error: Unknown argument\n");
  }
}

void search_commands(const char *query) {
  FILE *linux_file = fopen(DEFAULT_LINUX_PATH, "r");
  if (!linux_file) {
    perror("Error opening linux file");
    return;
  }

  char line[MAX_LINE_LENGTH];
  int found = 0;
  while (fgets(line, sizeof(line), linux_file)) {
    if (strcasestr(line, query) != 0) {
      char *command = strtok(line, " ");
      char *description = strtok(NULL, "\n");
      printf("%s\t%s\n", command, description);
      found = 1;
    }
  }

  if (!found) {
    printf("No commands found matching: %s\n", query);
  }

  fclose(linux_file);
}

// Case-insensitive substring search implementation
int strcasestr(const char *haystack, const char *needle) {
  size_t needle_len = strlen(needle);
  size_t haystack_len = strlen(haystack);

  for (size_t i = 0; i <= haystack_len - needle_len; i++) {
    if (strncasecmp(haystack + i, needle, needle_len) == 0) {
      return 1;
    }
  }
  return 0;
}
